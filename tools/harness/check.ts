#!/usr/bin/env bun

import { $ } from "bun";
import { mkdir } from "node:fs/promises";
import { resolve } from "node:path";

type Mode = "Staged" | "Fast" | "Full" | "Security" | "Coverage";

const root = resolve(import.meta.dir, "..", "..");
process.chdir(root);

let mode: Mode = "Fast";
let testFilter: string | undefined;
const args = process.argv.slice(2);
for (let index = 0; index < args.length; index += 1) {
	if (args[index] === "--mode" || args[index] === "-Mode") {
		const requested = args[++index];
		const match = ["Staged", "Fast", "Full", "Security", "Coverage"].find(
			(candidate) => candidate.toLowerCase() === requested?.toLowerCase(),
		);
		if (!match) throw new Error(`unknown check mode: ${requested}`);
		mode = match as Mode;
	} else if (args[index] === "--test-filter" || args[index] === "-TestFilter") {
		testFilter = args[++index];
	} else {
		throw new Error(`unknown argument: ${args[index]}`);
	}
}

async function gate(label: string, action: () => Promise<unknown>): Promise<void> {
	console.log(`==> ${label}`);
	await action();
}

async function fast(): Promise<void> {
	await gate("Format", async () => $`cargo fmt --all -- --check`);
	await gate("Architecture tests", async () => $`cargo nextest run --test architecture`);
	const changed = (await $`git diff --name-only HEAD`.text()).trim().split(/\r?\n/).filter(Boolean);
	if (changed.length > 0 && changed.every((path) => path.startsWith("crates/bevyout-core/"))) {
		await gate("Affected crate check (bevyout-core)", async () => $`cargo check -p bevyout-core`);
	} else if (changed.length > 0 && changed.every((path) => path.startsWith("crates/bevyout-xatlas/"))) {
		await gate("Affected crate check (bevyout-xatlas)", async () => $`cargo check -p bevyout-xatlas`);
	} else {
		await gate("Application check (dynamic linking)", async () => $`cargo check-dev`);
	}
	if (testFilter) await gate(`Focused nextest: ${testFilter}`, async () => $`cargo nextest run -E ${testFilter}`);
	else await gate("Focused contract tests", async () => $`cargo nextest run --test cli_contract --test command_smoke`);
}

switch (mode) {
	case "Staged":
		await gate("Exact staged rustfmt", async () => $`bun run tools/harness/staged_rustfmt.ts`);
		await gate("Blocking staged AST rules", async () => $`bun run tools/harness/ast_guard.ts --staged --blocking-only`);
		await gate("Prepared revision guard", async () => $`bun run tools/harness/revision_guard.ts --staged`);
		await gate("Bethesda data guard", async () => $`bun run tools/harness/data_guard.ts --staged`);
		break;
	case "Fast":
		await fast();
		break;
	case "Full": {
		const nextestFilter = "not binary(features)";
		await gate("Format", async () => $`cargo fmt --all -- --check`);
		await gate("Clippy", async () => $`cargo clippy --workspace --all-targets -- -D warnings`);
		await gate("Workspace nextest", async () => $`cargo nextest run --workspace -E ${nextestFilter}`);
		await gate("Doctests", async () => $`cargo test --workspace --doc`);
		await gate("Architecture tests", async () => $`cargo nextest run --test architecture`);
		await gate("Cucumber features", async () => $`cargo test --test features`);
		await gate("AST rule fixtures", async () => $`bun run tools/harness/ast_guard.ts --self-test`);
		await gate("Prepared revision guard fixtures", async () => $`bun run tools/harness/revision_guard.ts --self-test`);
		await gate("Performance scanner fixtures", async () => $`bun run .agents/skills/bevy-performance-audit/scripts/scan_bevy_code.ts --self-test`);
		await gate("MCP typecheck", async () => $`bun run --cwd tools/bevyout-mcp typecheck`);
		await gate("MCP adapter tests", async () => $`bun test --cwd tools/bevyout-mcp`);
		break;
	}
	case "Security":
		await gate("Dependency policy", async () => $`cargo deny check --hide-inclusion-graph`);
		// Bevy 0.19's Linux-only Wayland generator pins this build-time parser.
		await gate("RustSec audit", async () => $`cargo audit --ignore RUSTSEC-2026-0194 --ignore RUSTSEC-2026-0195`);
		await gate("Unused dependencies", async () => $`cargo machete --skip-target-dir`);
		break;
	case "Coverage": {
		const coverage = resolve(root, ".bevyout", "coverage");
		const output = resolve(coverage, "lcov.info");
		await mkdir(coverage, { recursive: true });
		await gate("LLVM coverage", async () => $`cargo llvm-cov --workspace --lcov --output-path ${output}`);
		console.log(`Coverage: ${output}`);
		break;
	}
}
