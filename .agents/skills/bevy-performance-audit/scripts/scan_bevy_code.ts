#!/usr/bin/env bun

import { isAbsolute, relative, resolve } from "node:path";

type Hit = {
	category: string;
	concern: string;
	confidence: string;
	runtime_context: string;
	path: string;
	line: number;
	text: string;
};

const skipDirs = new Set([
	".bevyout", ".git", ".claude", "BevyCheatSheet", "BevyDocs", "bevy_markdown_docs",
	"node_modules", "target", "third_party", "tools", "vendor", "NifConverter",
]);
const preparationPrefixes = [
	"src/vsa/prepare/", "src/vsa/bake/", "src/vsa/nif_convert/", "src/vsa/exterior/conversion.rs",
	"src/commands/prepare", "src/bin/prepare",
];
const rules: Array<[string, RegExp, string, string]> = [
	["exclusive-world-access", /(?:&mut\s+World|ExclusiveSystemParam)/, "parallelism", "medium"],
	["schedule-order", /\.(?:chain|chain_ignore_deferred)\s*\(\s*\)|\.(?:before|after)\s*\(/, "parallelism", "low"],
	["deferred-barrier", /\bapply_deferred\b/, "parallelism", "medium"],
	["broad-mutable-resource", /\bResMut\s*</, "parallelism", "low"],
	["non-send", /\bNonSend(?:Mut)?\s*</, "parallelism", "medium"],
	["task-pool", /\b(?:ComputeTaskPool|AsyncComputeTaskPool|IoTaskPool)\b/, "parallelism", "low"],
	["parallel-iterator", /\b(?:par_iter|par_iter_mut|into_par_iter)\s*\(/, "parallelism", "low"],
	["sync-wait", /\b(?:block_on|thread::sleep|park|park_timeout)\s*\(|\.recv(?:_timeout)?\s*\(/, "blocking", "high"],
	["shared-lock", /\b(?:Mutex|RwLock)\s*<|\.lock\s*\(/, "blocking", "medium"],
	["filesystem-io", /\b(?:std::fs|fs::(?:read|write|read_to_string|File))\b/, "io", "medium"],
	["asset-load", /\b(?:AssetServer|asset_server)\b.*\.load(?:_with_settings)?\s*\(/, "assets", "medium"],
	["entity-churn", /\.(?:spawn|spawn_batch|despawn|despawn_related)\s*\(/, "ecs", "low"],
	["full-query-scan", /\.(?:iter|iter_mut)\s*\(\s*&?(?:world|query)|\bfor\s+.+\s+in\s+&(?:mut\s+)?(?:query[A-Za-z0-9_]*|[A-Za-z_][A-Za-z0-9_]*query[A-Za-z0-9_]*)\b/, "ecs", "low"],
	["collection-allocation", /\b(?:Vec|HashMap|HashSet|BTreeMap)(?:\s*::<[^;=()]*>)?::(?:new|with_capacity)\s*\(|\.collect\s*::<\s*(?:Vec|HashMap|HashSet|BTreeMap)/, "allocation", "medium"],
	["render-specialization", /\b(?:SpecializedMeshPipeline|specialize|RenderAsset|RenderCommand)\b/, "render", "low"],
];

function isPreparationPath(path: string): boolean {
	return preparationPrefixes.some((prefix) => path.startsWith(prefix));
}

function runtimeContext(path: string): string {
	if (path.startsWith("src/vsa/render/") || path.includes("renderer") || path.includes("render_")) return "render-runtime";
	if (path.startsWith("src/viewer/") || path.startsWith("src/vsa/")) return "viewer-runtime";
	if (path.startsWith("crates/bevyout-core/")) return "core-policy";
	return "application-runtime";
}

function maskCommentsAndLiterals(text: string): string {
	const output = [...text];
	let index = 0;
	let blockDepth = 0;
	let state: "code" | "line-comment" | "block-comment" | "string" | "raw-string" = "code";
	let rawHashes = 0;
	const mask = (position: number) => { if (output[position] !== "\n") output[position] = " "; };

	while (index < text.length) {
		if (state === "line-comment") {
			if (text[index] === "\n") state = "code"; else mask(index);
			index += 1;
			continue;
		}
		if (state === "block-comment") {
			if (text.startsWith("/*", index)) { mask(index); mask(index + 1); blockDepth += 1; index += 2; }
			else if (text.startsWith("*/", index)) { mask(index); mask(index + 1); blockDepth -= 1; index += 2; if (blockDepth === 0) state = "code"; }
			else { mask(index); index += 1; }
			continue;
		}
		if (state === "string") {
			mask(index);
			if (text[index] === "\\" && index + 1 < text.length) { mask(index + 1); index += 2; }
			else if (text[index] === '"') { state = "code"; index += 1; }
			else index += 1;
			continue;
		}
		if (state === "raw-string") {
			const terminator = `"${"#".repeat(rawHashes)}`;
			if (text.startsWith(terminator, index)) {
				for (let offset = 0; offset < terminator.length; offset += 1) mask(index + offset);
				index += terminator.length;
				state = "code";
			} else { mask(index); index += 1; }
			continue;
		}
		if (text.startsWith("//", index)) { mask(index); mask(index + 1); index += 2; state = "line-comment"; continue; }
		if (text.startsWith("/*", index)) { mask(index); mask(index + 1); index += 2; blockDepth = 1; state = "block-comment"; continue; }
		const raw = /^(?:br|cr|r)(#{0,255})"/.exec(text.slice(index));
		if (raw) {
			rawHashes = raw[1].length;
			for (let offset = 0; offset < raw[0].length; offset += 1) mask(index + offset);
			index += raw[0].length;
			state = "raw-string";
			continue;
		}
		if (text[index] === '"' || (["b", "c"].includes(text[index]) && text[index + 1] === '"')) {
			if (text[index] !== '"') { mask(index); index += 1; }
			mask(index); index += 1; state = "string"; continue;
		}
		index += 1;
	}
	return output.join("");
}

function scanText(path: string, text: string): Hit[] {
	const hits: Hit[] = [];
	const sourceLines = text.split(/\r?\n/);
	const codeLines = maskCommentsAndLiterals(text).split(/\r?\n/);
	if (sourceLines.length !== codeLines.length) throw new Error("lexical masking changed the source line count");
	for (let index = 0; index < sourceLines.length; index += 1) {
		if (!codeLines[index].trim()) continue;
		for (const [category, pattern, concern, confidence] of rules) {
			if (pattern.test(codeLines[index])) hits.push({
				category, concern, confidence, runtime_context: runtimeContext(path), path,
				line: index + 1, text: sourceLines[index].trim().slice(0, 240),
			});
		}
	}
	return hits;
}

async function scan(root: string, includeTests: boolean, includePreparation: boolean): Promise<Hit[]> {
	const paths = [...await Array.fromAsync(new Bun.Glob("**/*.rs").scan({ cwd: root, absolute: true, dot: true }))].sort();
	const hits: Hit[] = [];
	for (const path of paths) {
		const normalized = relative(root, path).replaceAll("\\", "/");
		const parts = normalized.split("/");
		const name = parts.at(-1) ?? "";
		if (parts.some((part) => skipDirs.has(part))) continue;
		if (!includeTests && (parts.includes("tests") || name === "tests.rs" || name.endsWith("_tests.rs") || name.startsWith("test_"))) continue;
		if (!includePreparation && isPreparationPath(normalized)) continue;
		try { hits.push(...scanText(normalized, await Bun.file(path).text())); } catch (error) {
			if (!(error instanceof TypeError)) throw error;
		}
	}
	return hits;
}

function counts(hits: Hit[], key: keyof Hit): Record<string, number> {
	const values: Record<string, number> = {};
	for (const hit of hits) values[String(hit[key])] = (values[String(hit[key])] ?? 0) + 1;
	return Object.fromEntries(Object.entries(values).sort(([left], [right]) => left.localeCompare(right)));
}

function bounded(hits: Hit[], maximum: number): Hit[] {
	const used: Record<string, number> = {};
	return hits.filter((hit) => {
		used[hit.category] = (used[hit.category] ?? 0) + 1;
		return used[hit.category] <= maximum;
	});
}

function selfTest(): void {
	const sample = `
fn exclusive(world: &mut World) { std::thread::sleep(duration); }
fn regular(mut state: ResMut<State>) { values.par_iter().for_each(work); }
app.add_systems(Update, (a, b).chain());
fn scan(query: Query<&Transform>) { for transform in &query { consume(transform); } }
// fn commented(world: &mut World) { block_on(work); }
const EXAMPLE: &str = "mut fake: ResMut<State>";
/* nested /* .chain() */ AssetServer.load("fake") */
`;
	const hits = scanText("src/sample.rs", sample);
	const categories = new Set(hits.map((hit) => hit.category));
	for (const expected of ["exclusive-world-access", "sync-wait", "broad-mutable-resource", "parallel-iterator", "schedule-order", "full-query-scan"]) {
		if (!categories.has(expected)) throw new Error(`missing category: ${expected}`);
	}
	if (categories.has("asset-load")) throw new Error("comment or string produced asset-load hit");
	if (scanText("sample.rs", sample).filter((hit) => hit.category === "broad-mutable-resource").length !== 1) {
		throw new Error("comments or strings produced a broad-mutable-resource hit");
	}
	console.log("self-test passed");
}

const args = process.argv.slice(2);
if (args.includes("--self-test")) { selfTest(); process.exit(0); }
const json = args.includes("--json");
const includeTests = args.includes("--include-tests");
const includePreparation = args.includes("--include-preparation");
const maxIndex = args.indexOf("--max-per-category");
const maximum = maxIndex >= 0 ? Number(args[maxIndex + 1]) : 20;
if (!Number.isInteger(maximum) || maximum < 1) throw new Error("--max-per-category must be at least 1");
const positional = args.filter((argument, index) => !argument.startsWith("--") && args[index - 1] !== "--max-per-category");
const root = resolve(positional[0] ?? ".");
const hits = await scan(root, includeTests, includePreparation);
const selected = bounded(hits, maximum);
if (json) {
	console.log(JSON.stringify({
		root, candidate_count: hits.length, reported_candidate_count: selected.length,
		truncated: selected.length < hits.length, category_counts: counts(hits, "category"),
		confidence_counts: counts(hits, "confidence"), runtime_context_counts: counts(hits, "runtime_context"),
		candidates: selected,
	}, null, 2));
} else {
	console.log(`# Bevy static candidate inventory\n\nRoot: \`${root}\``);
	console.log("\nThese are review leads, not confirmed bottlenecks.");
	for (const category of [...new Set(hits.map((hit) => hit.category))].sort()) {
		const categoryHits = hits.filter((hit) => hit.category === category);
		const confidence = Object.entries(counts(categoryHits, "confidence")).map(([key, value]) => `${key}=${value}`).join(", ");
		const contexts = Object.entries(counts(categoryHits, "runtime_context")).map(([key, value]) => `${key}=${value}`).join(", ");
		console.log(`\n## ${category} (${categoryHits[0].concern}, ${categoryHits.length} hits; confidence: ${confidence}; context: ${contexts})`);
		for (const hit of categoryHits.slice(0, maximum)) console.log(`- \`${hit.path}:${hit.line}\` — \`${hit.text}\``);
		if (categoryHits.length > maximum) console.log(`- … ${categoryHits.length - maximum} more (raise --max-per-category)`);
	}
}
