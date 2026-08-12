#!/usr/bin/env bun

import { $ } from "bun";
import { chmod } from "node:fs/promises";
import { resolve } from "node:path";

type Toolchain = {
	components: string[];
	cargo_tools: Record<string, string>;
};

const root = resolve(import.meta.dir, "..", "..");
process.chdir(root);
const manifest = (await Bun.file(resolve(root, ".pi", "toolchain.json")).json()) as Toolchain;

async function gate(label: string, action: () => Promise<unknown>): Promise<void> {
	console.log(`==> ${label}`);
	await action();
}

await gate("Rust toolchain components", async () => {
	await $`rustup component add ${manifest.components}`;
});

const installed = await $`cargo install --list`.text();
for (const [tool, version] of Object.entries(manifest.cargo_tools)) {
	const exact = new RegExp(`^${tool.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")} v${version.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}:$`, "m");
	if (exact.test(installed)) {
		console.log(`==> ${tool} ${version} already installed`);
		continue;
	}
	await gate(`Install ${tool} ${version}`, async () => {
		await $`cargo install --locked --version ${version} ${tool}`;
	});
}

await gate("Project git hooks", async () => {
	if (process.platform !== "win32") {
		await chmod(resolve(root, ".githooks", "pre-commit"), 0o755);
		await chmod(resolve(root, ".githooks", "pre-push"), 0o755);
	}
	await $`git config --local core.hooksPath .githooks`;
});

if (Bun.which("sccache")) {
	const stats = await $`sccache --show-stats`.quiet().nothrow();
	if (stats.exitCode !== 0) await $`sccache --start-server`.quiet();
}

console.log("Bootstrap complete. Re-running is safe.");
