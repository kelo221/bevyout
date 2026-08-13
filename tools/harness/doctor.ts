#!/usr/bin/env bun

import { $ } from "bun";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "..", "..");
process.chdir(root);
const failures: string[] = [];

async function probe(label: string, command: string[]): Promise<void> {
	const result = await $`${command}`.quiet().nothrow();
	const output = new TextDecoder().decode(result.stdout).trim().split(/\r?\n/)[0] ?? "";
	if (result.exitCode === 0) console.log(`PASS ${label} ${output}`.trim());
	else {
		failures.push(`${label}: exit ${result.exitCode}`);
		console.log(`FAIL ${label}`);
	}
}

await probe("Pi", ["pi", "--version"]);
await probe("Rust", ["rustc", "--version"]);
await probe("rust-analyzer", ["rust-analyzer", "--version"]);
await probe("CodeGraph", ["codegraph", "--version"]);
await probe("ast-grep", ["ast-grep", "--version"]);
await probe("Bun", ["bun", "--version"]);
await probe("cargo-nextest", ["cargo", "nextest", "--version"]);
await probe("cargo-deny", ["cargo", "deny", "--version"]);
await probe("cargo-audit", ["cargo", "audit", "--version"]);
await probe("cargo-machete", ["cargo", "machete", "--version"]);
await probe("cargo-llvm-cov", ["cargo", "llvm-cov", "--version"]);
await probe("sccache", ["sccache", "--version"]);
await probe("Harness config/model validation", ["bun", "run", "tools/harness/validate_harness.ts"]);
await probe("AST rule fixtures", ["bun", "run", "tools/harness/ast_guard.ts", "--self-test"]);
await probe("Revision guard fixtures", ["bun", "run", "tools/harness/revision_guard.ts", "--self-test"]);
await probe("MCP handshake", ["bun", "test", "--cwd", "tools/bevyout-mcp", "test/mcp_handshake.test.ts"]);
await probe("Pi full launcher", ["bun", "run", "tools/harness/launcher_smoke.ts"]);
await probe("Pi lean launcher", ["bun", "run", "tools/harness/launcher_smoke.ts", "--lean"]);
await probe("Pi workflow doctor", ["bun", "run", "tools/harness/launcher_smoke.ts", "--workflow-doctor"]);

const hooksResult = await $`git config --local --get core.hooksPath`.quiet().nothrow();
const hooks = new TextDecoder().decode(hooksResult.stdout).trim();
if (hooks === ".githooks") console.log("PASS git hooks .githooks");
else failures.push(`git hooks: expected .githooks, got '${hooks}'`);
if (Bun.which("sccache")) console.log("PASS conditional sccache launcher support");

if (failures.length > 0) {
	console.error("Doctor failed:");
	for (const failure of failures) console.error(`- ${failure}`);
	process.exit(1);
}
console.log("Doctor passed: routes, workflows, tools, MCP, hooks, and launchers ready.");
