#!/usr/bin/env bun

import { mkdtemp, mkdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { basename, dirname, join, resolve } from "node:path";

type Finding = {
	ruleId: string;
	severity: string;
	file: string;
	message: string;
	range: { start: { line: number } };
};

const root = resolve(import.meta.dir, "..", "..");
const config = resolve(root, "sgconfig.yml");
const expected = new Set([
	"bevy-system-blocking",
	"bevy-system-frame-allocation",
	"viewer-stdio-macro",
	"runtime-asset-load",
	"bevy-system-panic-path",
	"bevy-schedule-chain-review",
	"bevy-overlapping-mut-query-review",
]);
const excluded = new Set([
	".bevyout", "target", "tests", "benches", "examples", "BevyCheatSheet", "BevyDocs",
	"bevy_markdown_docs", "node_modules", "third_party", "vendor", "NifConverter", "prepare", "preparation",
	"bake", "catalog",
]);

function run(command: string[], cwd = root, check = true): { stdout: string; stderr: string; exitCode: number } {
	const result = Bun.spawnSync(command, { cwd, stdout: "pipe", stderr: "pipe" });
	const stdout = result.stdout.toString();
	const stderr = result.stderr.toString();
	if (check && result.exitCode !== 0) throw new Error(stderr.trim() || `${command.join(" ")} failed`);
	return { stdout, stderr, exitCode: result.exitCode };
}

function shouldScan(path: string): boolean {
	const normalized = path.replaceAll("\\", "/");
	const parts = normalized.split("/");
	const name = parts.at(-1)?.toLowerCase() ?? "";
	if (!normalized.endsWith(".rs") || parts.some((part) => excluded.has(part))) return false;
	if (name.startsWith("test_") || name.endsWith("_test.rs")) return false;
	if (
		normalized === "src/cli.rs" ||
		normalized === "src/main.rs" ||
		normalized.includes("/cli/") ||
		normalized === "src/vsa/catalog.rs" ||
		normalized.includes("src/vsa/bake/") ||
		normalized.includes("src/vsa/cache_stats/")
	) return false;
	return normalized.startsWith("src/") || normalized.startsWith("crates/");
}

function scanFiles(files: string[]): Finding[] {
	const findings: Finding[] = [];
	for (const path of files) {
		const result = run(["ast-grep", "scan", "--config", config, path, "--json=compact"], root, false);
		if (result.stdout.trim()) {
			try { findings.push(...JSON.parse(result.stdout)); }
			catch (error) { throw new Error(`invalid ast-grep JSON for ${path}: ${error}\n${result.stdout}\n${result.stderr}`); }
		}
		if (![0, 1].includes(result.exitCode)) throw new Error(result.stderr.trim() || `ast-grep failed for ${path}`);
	}
	return findings;
}

function sameSet(left: Set<string>, right: Set<string>): boolean {
	return left.size === right.size && [...left].every((value) => right.has(value));
}

function selfTest(): void {
	const fixture = resolve(root, "tools", "ast-grep", "fixtures");
	const ids = new Set(scanFiles([resolve(fixture, "positive.rs")]).map((finding) => finding.ruleId));
	if (!sameSet(ids, expected)) {
		const missing = [...expected].filter((id) => !ids.has(id));
		const extra = [...ids].filter((id) => !expected.has(id));
		throw new Error(`positive fixture mismatch: missing=${missing} extra=${extra}`);
	}
	for (const name of ["negative.rs", "startup.rs", "preparation.rs"]) {
		const findings = scanFiles([resolve(fixture, name)]);
		if (findings.length > 0) throw new Error(`${name} unexpectedly matched: ${findings.map((finding) => finding.ruleId)}`);
	}
	if (shouldScan("tools/ast-grep/fixtures/test_code.rs")) throw new Error("test-code fixture was not excluded");
	console.log("AST-grep fixture suite passed");
}

const args = process.argv.slice(2);
const modeFlags = ["--staged", "--all", "--base", "--self-test"].filter((flag) => args.includes(flag));
if (modeFlags.length !== 1) throw new Error("choose exactly one of --staged, --all, --base, or --self-test");
if (args.includes("--self-test")) {
	selfTest();
	process.exit(0);
}
const blockingOnly = args.includes("--blocking-only");
const maxIndex = args.indexOf("--max");
const maximum = maxIndex >= 0 ? Number(args[maxIndex + 1]) : 80;
const baseIndex = args.indexOf("--base");
const base = baseIndex >= 0 ? args[baseIndex + 1] : undefined;

let temporary: string | undefined;
let files: string[] = [];
try {
	if (args.includes("--staged")) {
		temporary = await mkdtemp(join(tmpdir(), "bevyout-ast-"));
		const names = run(["git", "diff", "--cached", "--name-only", "--diff-filter=ACMR"]).stdout.split(/\r?\n/).filter(Boolean);
		for (const name of names.filter(shouldScan)) {
			const blob = run(["git", "show", `:${name}`]);
			const target = resolve(temporary, name);
			await mkdir(dirname(target), { recursive: true });
			await writeFile(target, blob.stdout, "utf8");
			files.push(target);
		}
	} else if (base) {
		files = run(["git", "diff", "--name-only", "--diff-filter=ACMR", `${base}...HEAD`]).stdout
			.split(/\r?\n/).filter(shouldScan).map((name) => resolve(root, name));
	} else {
		files = run(["git", "ls-files", "*.rs"]).stdout.split(/\r?\n/).filter(shouldScan).map((name) => resolve(root, name));
	}

	const selected = scanFiles(files).filter((finding) => !blockingOnly || finding.severity === "error");
	for (const finding of selected.slice(0, maximum)) {
		console.log(`${finding.severity} ${finding.ruleId} ${finding.file}:${finding.range.start.line + 1}: ${finding.message}`);
	}
	if (selected.length > maximum) console.log(`... ${selected.length - maximum} more findings (raise --max)`);
	if (selected.some((finding) => finding.severity === "error")) process.exitCode = 1;
	else console.log(`AST guard passed (${selected.length} advisory findings)`);
} finally {
	if (temporary) await rm(temporary, { recursive: true, force: true });
}
