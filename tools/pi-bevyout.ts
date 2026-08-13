#!/usr/bin/env bun

import { resolve } from "node:path";

const root = resolve(import.meta.dir, "..");
const passthrough: string[] = [];
let lean = false;
let runtimeWrite = false;

for (const argument of process.argv.slice(2)) {
	if (argument === "--lean" || argument === "-Lean") lean = true;
	else if (argument === "--runtime-write" || argument === "-RuntimeWrite") runtimeWrite = true;
	else passthrough.push(argument);
}

if (lean && runtimeWrite) {
	console.error("--runtime-write requires full mode because --lean disables MCP.");
	process.exit(2);
}

const environment = { ...process.env };
environment.BEVYOUT_MCP_ACCESS = runtimeWrite ? "runtime_write" : "read_only";
if (Bun.which("sccache")) environment.RUSTC_WRAPPER = "sccache";

const pi = Bun.which("pi") ?? Bun.which("pi.cmd");
if (!pi) {
	console.error("Pi is not installed or is not on PATH.");
	process.exit(127);
}

const arguments_ = [
	"--no-context-files",
	"--no-skills",
	"--skill",
	resolve(root, ".agents", "skills"),
	"--approve",
	"--append-system-prompt",
	resolve(root, ".pi", "PI.md"),
	"--mcp-config",
	resolve(root, ".pi", lean ? "mcp-lean.json" : "../.mcp.json"),
];

if (lean) arguments_.push("--no-tests", "--no-lens-context", "--no-opengrep");
else arguments_.push("--lens-guard", "--lens-turn-summary");
arguments_.push(...passthrough);

const child = Bun.spawn([pi, ...arguments_], {
	cwd: root,
	env: environment,
	stdin: "inherit",
	stdout: "inherit",
	stderr: "inherit",
});

process.exit(await child.exited);
