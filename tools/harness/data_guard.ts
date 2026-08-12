#!/usr/bin/env bun

import { extname, resolve } from "node:path";

const root = resolve(import.meta.dir, "..", "..");
const proprietary = new Set([".esm", ".esp", ".bsa", ".ba2", ".nif", ".dds", ".xwm", ".fuz", ".lip"]);
const riskRoots = [".bevyout/", "data/", "fallout 3/", "fallout3/"];
const syntheticRoots = ["tests/fixtures/synthetic/", "crates/bevyout-core/tests/fixtures/synthetic/"];

function git(args: string[]): string {
	const result = Bun.spawnSync(["git", ...args], { cwd: root, stdout: "pipe", stderr: "pipe" });
	if (result.exitCode !== 0) throw new Error(result.stderr.toString().trim());
	return result.stdout.toString();
}

const args = process.argv.slice(2);
const staged = args.includes("--staged");
const baseIndex = args.indexOf("--base");
const base = baseIndex >= 0 ? args[baseIndex + 1] : undefined;
if (staged === Boolean(base)) throw new Error("choose exactly one of --staged or --base");
const names = git(staged
	? ["diff", "--cached", "--name-only", "--diff-filter=ACMR"]
	: ["diff", "--name-only", "--diff-filter=ACMR", `${base}...HEAD`])
	.split(/\r?\n/).filter(Boolean);

const findings: string[] = [];
for (const raw of names) {
	const path = raw.replaceAll("\\", "/");
	const lower = path.toLowerCase();
	if (riskRoots.some((prefix) => lower.startsWith(prefix))) findings.push(`${path}: derived/game-data root is never commit-safe`);
	else if (proprietary.has(extname(lower)) && !syntheticRoots.some((prefix) => lower.startsWith(prefix))) {
		findings.push(`${path}: proprietary game-data format requires an explicitly synthetic fixture path`);
	} else if (["fallout3.esm", "fallout - textures", "fallout - meshes"].some((name) => lower.includes(name))) {
		findings.push(`${path}: known Bethesda source-data name`);
	}
}
if (findings.length > 0) {
	console.error("Bethesda data guard failed:");
	for (const finding of findings) console.error(`- ${finding}`);
	process.exit(1);
}
console.log("Bethesda data guard passed");
