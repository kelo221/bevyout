#!/usr/bin/env bun

import { resolve } from "node:path";

type Revision = { path: string; constant: string };
type Schema = { path: string; type_pattern: string; revisions: Revision[] };
type Mapping = { schemas: Schema[] };
type Snapshot = Record<string, string>;

const root = resolve(import.meta.dir, "..", "..");
const mapPath = resolve(import.meta.dir, "prepared-revisions.json");
const typeStart = /\bpub\s+(?:struct|enum)\s+(Prepared[A-Za-z0-9_]+)\b/g;

function git(args: string[], check = true): string {
	const result = Bun.spawnSync(["git", ...args], { cwd: root, stdout: "pipe", stderr: "pipe" });
	if (check && result.exitCode !== 0) throw new Error(result.stderr.toString().trim() || `git ${args.join(" ")} failed`);
	return result.stdout.toString();
}

function extractTypes(text: string): Map<string, string> {
	const found = new Map<string, string>();
	for (const match of text.matchAll(typeStart)) {
		const start = match.index;
		let brace = text.indexOf("{", start + match[0].length);
		if (brace < 0) continue;
		let depth = 0;
		let end = brace;
		for (; end < text.length; end += 1) {
			if (text[end] === "{") depth += 1;
			else if (text[end] === "}") {
				depth -= 1;
				if (depth === 0) { end += 1; break; }
			}
		}
		found.set(match[1], text.slice(start, end).replace(/\s+/g, " ").trim());
	}
	return found;
}

function escapeRegExp(value: string): string {
	return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function constantValue(text: string, name: string): string | undefined {
	return new RegExp(`\\b${escapeRegExp(name)}\\b[^=]*=\\s*"([^"]+)"`).exec(text)?.[1];
}

function evaluate(before: Snapshot, after: Snapshot, changed: Set<string>, mapping: Mapping): string[] {
	const findings: string[] = [];
	const schemaPaths = new Set(mapping.schemas.map((entry) => entry.path));
	for (const path of [...changed].filter((value) => schemaPaths.has(value)).sort()) {
		const oldTypes = extractTypes(before[path] ?? "");
		const newTypes = extractTypes(after[path] ?? "");
		const names = new Set([...oldTypes.keys(), ...newTypes.keys()]);
		for (const typeName of [...names].filter((name) => oldTypes.get(name) !== newTypes.get(name)).sort()) {
			const requirements = mapping.schemas
				.filter((entry) => entry.path === path && new RegExp(entry.type_pattern).test(typeName))
				.flatMap((entry) => entry.revisions);
			if (requirements.length === 0) {
				findings.push(`${path}: ${typeName} has no revision-map entry`);
				continue;
			}
			const seen = new Set<string>();
			for (const revision of requirements) {
				const key = `${revision.path}\0${revision.constant}`;
				if (seen.has(key)) continue;
				seen.add(key);
				const oldValue = constantValue(before[revision.path] ?? "", revision.constant);
				const newValue = constantValue(after[revision.path] ?? "", revision.constant);
				if (newValue === undefined || (oldTypes.has(typeName) && oldValue === undefined)) {
					findings.push(`${path}: ${typeName} maps to missing ${revision.path}:${revision.constant}`);
				} else if (oldTypes.has(typeName) && oldValue === newValue) {
					findings.push(`${path}: ${typeName} changed without bumping ${revision.path}:${revision.constant} (${newValue})`);
				}
			}
		}
	}
	return findings;
}

function snapshot(ref: string, paths: Set<string>, staged: boolean): Snapshot {
	return Object.fromEntries([...paths].map((path) => [path, git(["show", staged ? `:${path}` : `${ref}:${path}`], false)]));
}

function selfTest(): void {
	const mapping: Mapping = { schemas: [
		{ path: "schema.rs", type_pattern: "^PreparedA$", revisions: [{ path: "rev.rs", constant: "A_REVISION" }] },
		{ path: "schema.rs", type_pattern: "^PreparedB$", revisions: [{ path: "rev.rs", constant: "B_REVISION" }] },
	] };
	const base = { "schema.rs": "pub struct PreparedA { pub x: u32 }\npub struct PreparedB { pub y: u32 }", "rev.rs": 'const A_REVISION: &str = "a1"; const B_REVISION: &str = "b1";' };
	const fieldOnly = { ...base, "schema.rs": "pub struct PreparedA { pub x: u64 }\npub struct PreparedB { pub y: u32 }" };
	if (evaluate(base, fieldOnly, new Set(["schema.rs"]), mapping).length !== 1) throw new Error("field-only fixture failed");
	const revisionOnly = { ...base, "rev.rs": 'const A_REVISION: &str = "a2"; const B_REVISION: &str = "b1";' };
	if (evaluate(base, revisionOnly, new Set(["rev.rs"]), mapping).length !== 0) throw new Error("revision-only fixture failed");
	const unrelated = { ...base, "other.rs": "fn changed() {}" };
	if (evaluate(base, unrelated, new Set(["other.rs"]), mapping).length !== 0) throw new Error("unrelated fixture failed");
	const multi = { ...fieldOnly, "schema.rs": "pub struct PreparedA { pub x: u64 }\npub struct PreparedB { pub y: u64 }", "rev.rs": revisionOnly["rev.rs"] };
	const findings = evaluate(base, multi, new Set(["schema.rs", "rev.rs"]), mapping);
	if (findings.length !== 1 || !findings[0].includes("B_REVISION")) throw new Error("multi-schema fixture failed");
	const addedMapping: Mapping = { schemas: [
		{ path: "schema.rs", type_pattern: "^PreparedC$", revisions: [{ path: "rev.rs", constant: "C_REVISION" }] },
	] };
	const withAddedType = {
		"schema.rs": `${base["schema.rs"]}\npub struct PreparedC { pub z: u32 }`,
		"rev.rs": `${base["rev.rs"]} const C_REVISION: &str = "c1";`,
	};
	if (evaluate(base, withAddedType, new Set(["schema.rs", "rev.rs"]), addedMapping).length !== 0) {
		throw new Error("new type with new revision fixture failed");
	}
	console.log("revision guard self-test passed");
}

const args = process.argv.slice(2);
if (args.includes("--self-test")) {
	selfTest();
	process.exit(0);
}
const staged = args.includes("--staged");
const baseIndex = args.indexOf("--base");
const base = baseIndex >= 0 ? args[baseIndex + 1] : undefined;
if (staged === Boolean(base)) throw new Error("choose exactly one of --staged or --base");

const mapping = await Bun.file(mapPath).json() as Mapping;
const revisionPaths = new Set(mapping.schemas.flatMap((entry) => entry.revisions.map((revision) => revision.path)));
const schemaPaths = new Set(mapping.schemas.map((entry) => entry.path));
const relevant = new Set([...schemaPaths, ...revisionPaths]);
let changed: Set<string>;
let before: Snapshot;
let after: Snapshot;
if (staged) {
	changed = new Set(git(["diff", "--cached", "--name-only", "--diff-filter=ACMR"]).split(/\r?\n/).filter(Boolean));
	before = snapshot("HEAD", relevant, false);
	after = snapshot("", relevant, true);
} else {
	changed = new Set(git(["diff", "--name-only", "--diff-filter=ACMR", `${base}...HEAD`]).split(/\r?\n/).filter(Boolean));
	before = snapshot(base!, relevant, false);
	after = snapshot("HEAD", relevant, false);
}
const findings = evaluate(before, after, changed, mapping);
if (findings.length > 0) {
	console.error("Prepared revision guard failed:");
	for (const finding of findings) console.error(`- ${finding}`);
	process.exit(1);
}
console.log("Prepared revision guard passed");
