#!/usr/bin/env bun

import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { basename, join, resolve } from "node:path";

const root = resolve(import.meta.dir, "..", "..");
function run(command: string[], check = false) {
	const result = Bun.spawnSync(command, { cwd: root, stdout: "pipe", stderr: "pipe" });
	if (check && result.exitCode !== 0) throw new Error(result.stderr.toString().trim());
	return result;
}

const rust = run(["git", "diff", "--cached", "--name-only", "--diff-filter=ACMR"]).stdout.toString()
	.split(/\r?\n/).filter((name) => name.endsWith(".rs"));
if (rust.length === 0) {
	console.log("Staged rustfmt passed (no staged Rust files)");
	process.exit(0);
}
const directory = await mkdtemp(join(tmpdir(), "bevyout-rustfmt-"));
const failures: string[] = [];
try {
	for (const name of rust) {
		const blob = run(["git", "show", `:${name}`]);
		if (blob.exitCode !== 0) {
			failures.push(`${name}: ${blob.stderr.toString().trim()}`);
			continue;
		}
		const target = resolve(directory, basename(name));
		await writeFile(target, blob.stdout);
		if (run(["rustfmt", "--edition", "2024", "--check", target]).exitCode !== 0) failures.push(`${name}: staged blob is not rustfmt-clean`);
	}
} finally {
	await rm(directory, { recursive: true, force: true });
}
if (failures.length > 0) {
	console.error("Staged rustfmt failed:");
	for (const failure of failures) console.error(`- ${failure}`);
	process.exit(1);
}
console.log(`Staged rustfmt passed (${rust.length} files)`);
