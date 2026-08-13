#!/usr/bin/env bun

import { YAML } from "bun";
import { existsSync, readFileSync } from "node:fs";
import { basename, relative, resolve } from "node:path";

const root = resolve(import.meta.dir, "..", "..");
const expectedAgents = new Set([
	"bevy-scout", "bevy-core-worker", "bevy-runtime-worker", "bevy-pipeline-worker",
	"bevy-runtime-verifier", "bevy-reviewer", "bevy-wave-integrator",
]);
const expectedWorkflows = new Set(["feature-slice", "gameplay-runtime-bug", "renderer-performance", "prepared-schema-pipeline"]);
const failures: string[] = [];

function text(path: string): string {
	return readFileSync(path, "utf8");
}

function sameSet(left: Set<string>, right: Set<string>): boolean {
	return left.size === right.size && [...left].every((value) => right.has(value));
}

function frontmatter(path: string): Record<string, any> {
	const match = /^---\s*\n(.*?)\n---\s*\n/s.exec(text(path));
	if (!match) throw new Error("missing YAML frontmatter");
	const value = YAML.parse(match[1]);
	if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("frontmatter is not a mapping");
	return value as Record<string, any>;
}

async function glob(pattern: string, cwd = root): Promise<string[]> {
	return [...await Array.fromAsync(new Bun.Glob(pattern).scan({ cwd, absolute: true }))].sort();
}

function availableModels(): Set<string> {
	const pi = Bun.which("pi") ?? Bun.which("pi.cmd");
	if (!pi) throw new Error("Pi is not on PATH");
	const result = Bun.spawnSync([pi, "--offline", "--list-models"], { cwd: root, stdout: "pipe", stderr: "pipe" });
	if (result.exitCode !== 0) throw new Error(result.stderr.toString().trim() || "pi --list-models failed");
	const models = new Set<string>();
	for (const line of result.stdout.toString().split(/\r?\n/).slice(1)) {
		const columns = line.trim().split(/\s+/);
		if (columns.length >= 2 && columns[0] !== "provider") models.add(`${columns[0]}/${columns[1]}`);
	}
	return models;
}

const jsonFiles = [resolve(root, "package.json"), resolve(root, ".mcp.json"), resolve(root, ".pi-lens.json"),
	...await glob("*.json", resolve(root, ".pi")), ...await glob("*.json", import.meta.dir)];
for (const path of jsonFiles) {
	try { JSON.parse(text(path)); }
	catch (error) { failures.push(`invalid JSON ${relative(root, path)}: ${error}`); }
}

const yamlFiles = [resolve(root, "sgconfig.yml"), resolve(root, ".github", "workflows", "ci.yml"),
	...await glob("*.yml", resolve(root, "tools", "ast-grep", "rules"))];
for (const path of yamlFiles) {
	try { YAML.parse(text(path)); }
	catch (error) { failures.push(`invalid YAML ${relative(root, path)}: ${error}`); }
}

const agentPaths = await glob("*.md", resolve(root, ".pi", "agents"));
const agents = new Map(agentPaths.map((path) => [basename(path, ".md"), path]));
if (!sameSet(new Set(agents.keys()), expectedAgents)) failures.push(`agent inventory mismatch: ${[...agents.keys()].sort()}`);
const configuredModels = new Set<string>();
for (const [name, path] of agents) {
	try {
		const data = frontmatter(path);
		configuredModels.add(String(data.model ?? ""));
		if (data.inherit_context !== false) failures.push(`${name}: inherit_context must be false`);
		if (data.output_transcript !== false) failures.push(`${name}: output_transcript must be false`);
		if (!Number.isInteger(data.max_turns) || data.max_turns <= 0) failures.push(`${name}: bounded max_turns required`);
		if (data.thinking !== "high") failures.push(`${name}: thinking must be high`);
		if (["bevy-core-worker", "bevy-runtime-worker", "bevy-pipeline-worker", "bevy-wave-integrator"].includes(name) && data.isolation !== "worktree") {
			failures.push(`${name}: worktree isolation required`);
		}
	} catch (error) { failures.push(`invalid frontmatter ${relative(root, path)}: ${error}`); }
}

const workflowConfig = JSON.parse(text(resolve(root, ".pi", "workflow.json")));
for (const [key, expected] of Object.entries({ version: 1, profile: "custom", concurrency: 4, maxAgents: 12, approvalMode: "always" })) {
	if (workflowConfig[key] !== expected) failures.push(`workflow.json ${key} must be ${JSON.stringify(expected)}`);
}
for (const route of Object.values(workflowConfig.routes ?? {}) as Array<Record<string, any>>) {
	configuredModels.add(String(route.model ?? ""));
	if (route.thinking !== "high") failures.push("workflow routes must use high thinking");
}
try {
	const models = availableModels();
	const missing = [...configuredModels].filter((model) => !models.has(model));
	if (missing.length > 0) failures.push(`models unavailable: ${missing.sort()}`);
} catch (error) { failures.push(`model registry check failed: ${error}`); }

const workflowPaths = await glob("*.js", resolve(root, ".pi", "workflows"));
const workflows = new Map(workflowPaths.map((path) => [basename(path, ".js"), path]));
if (!sameSet(new Set(workflows.keys()), expectedWorkflows)) failures.push(`workflow inventory mismatch: ${[...workflows.keys()].sort()}`);
for (const [name, path] of workflows) {
	const source = text(path);
	if (!source.startsWith("export const meta = {")) failures.push(`${name}: missing literal metadata export`);
	if (!source.includes("phase(") || !source.includes("return {")) failures.push(`${name}: incomplete orchestration`);
}

const launcher = text(resolve(root, "tools", "pi-bevyout.ts"));
for (const token of ["--no-skills", "--skill", "--no-tests", "--lens-guard", "--no-lens-context", "--no-opengrep", "BEVYOUT_MCP_ACCESS", "sccache"]) {
	if (!launcher.includes(token)) failures.push(`launcher missing ${token}`);
}
if (text(resolve(root, ".pi", "PI.md")).includes("Unity")) failures.push("Pi context references Unity");
if ((await glob("*.ps1", import.meta.dir)).length > 0 || existsSync(resolve(root, "tools", "pi-bevyout.ps1"))) {
	failures.push("PowerShell harness entrypoints are forbidden; use Bun");
}
if ((await glob("*.py", import.meta.dir)).length > 0 || existsSync(resolve(root, ".agents", "skills", "bevy-performance-audit", "scripts", "scan_bevy_code.py"))) {
	failures.push("Python harness entrypoints are forbidden; use Bun");
}

const subagents = JSON.parse(text(resolve(root, ".pi", "subagents.json")));
for (const [key, expected] of Object.entries({ maxConcurrent: 4, maxSubagentDepth: 2, outputTranscript: false, toolDescriptionMode: "compact" })) {
	if (subagents[key] !== expected) failures.push(`subagents.json ${key} must be ${JSON.stringify(expected)}`);
}

const revisions = JSON.parse(text(resolve(import.meta.dir, "prepared-revisions.json")));
const mappedPaths = new Set<string>((revisions.schemas ?? []).map((entry: { path: string }) => entry.path));
const preparedPaths = new Set<string>();
for (const base of [resolve(root, "crates", "bevyout-core", "src"), resolve(root, "src", "vsa")]) {
	for (const path of await glob("**/*.rs", base)) {
		if (/pub\s+(?:struct|enum)\s+Prepared/.test(text(path))) preparedPaths.add(relative(root, path).replaceAll("\\", "/"));
	}
}
const unmapped = [...preparedPaths].filter((path) => !mappedPaths.has(path));
if (unmapped.length > 0) failures.push(`prepared schema files missing revision map: ${unmapped.sort()}`);

if (failures.length > 0) {
	console.error("Harness validation failed:");
	for (const failure of failures) console.error(`- ${failure}`);
	process.exit(1);
}
console.log(`Harness validation passed: ${agents.size} agents, ${workflows.size} workflows, ${configuredModels.size} model routes`);
