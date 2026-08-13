#!/usr/bin/env bun

import { resolve } from "node:path";

const root = resolve(import.meta.dir, "..", "..");
const lean = process.argv.includes("--lean") || process.argv.includes("-Lean");
const workflowDoctor = process.argv.includes("--workflow-doctor") || process.argv.includes("-WorkflowDoctor");
if (lean && workflowDoctor) throw new Error("workflow doctor requires full mode");

const launcher = resolve(root, "tools", "pi-bevyout.ts");
const args = [process.execPath, "run", launcher];
if (lean) args.push("--lean");
args.push("--offline", "--mode", "rpc", "--no-session");

const process_ = Bun.spawn(args, {
	cwd: root,
	stdin: "pipe",
	stdout: "pipe",
	stderr: "pipe",
});
const stderr = new Response(process_.stderr).text();
const reader = process_.stdout.getReader();
const decoder = new TextDecoder();
let buffered = "";
let mcpStatus = "";

async function nextLine(): Promise<string> {
	while (true) {
		const newline = buffered.indexOf("\n");
		if (newline >= 0) {
			const line = buffered.slice(0, newline).replace(/\r$/, "");
			buffered = buffered.slice(newline + 1);
			return line;
		}
		const chunk = await reader.read();
		if (chunk.done) {
			if (buffered.length > 0) {
				const line = buffered;
				buffered = "";
				return line;
			}
			throw new Error(`Pi RPC closed early: ${(await stderr).trim()}`);
		}
		buffered += decoder.decode(chunk.value, { stream: true });
	}
}

async function nextEvent(): Promise<any> {
	const line = await Promise.race([
		nextLine(),
		Bun.sleep(15_000).then(() => {
			throw new Error("Pi RPC output timed out");
		}),
	]);
	const event = JSON.parse(line);
	if (event.statusKey === "mcp") mcpStatus = String(event.statusText ?? "");
	return event;
}

async function request(id: string, payload: Record<string, unknown>): Promise<any> {
	process_.stdin.write(`${JSON.stringify({ id, ...payload })}\n`);
	process_.stdin.flush();
	for (let index = 0; index < 200; index += 1) {
		const event = await nextEvent();
		if (event.id === id && event.type === "response") {
			if (!event.success) throw new Error(`Pi RPC ${id} failed: ${event.error ?? "unknown error"}`);
			return event;
		}
	}
	throw new Error(`Pi RPC did not return ${id}`);
}

try {
	await request("smoke", { type: "get_state" });
	const commandResponse = await request("commands", { type: "get_commands" });
	const names = new Set<string>((commandResponse.data.commands as Array<{ name: string }>).map((command) => command.name));
	for (const required of ["workflow", "agents", "skill:bevy-performance-audit", "skill:bevyout-mcp", "skill:bevyout-scene-pipeline"]) {
		if (!names.has(required)) throw new Error(`Pi command inventory is missing ${required}`);
	}
	for (const unrelated of ["skill:unity-official-mcp", "skill:substance-painter-mcp-guide", "skill:quest-qa"]) {
		if (names.has(unrelated)) throw new Error(`unrelated global skill leaked into project harness: ${unrelated}`);
	}
	if (lean && !/MCP:\s+0 servers enabled/.test(mcpStatus)) throw new Error(`Lean mode did not disable MCP: ${mcpStatus}`);
	if (!lean && !/MCP:\s+1 server enabled/.test(mcpStatus)) throw new Error(`Full mode did not enable Bevyout MCP: ${mcpStatus}`);

	console.log(`Pi ${lean ? "lean" : "full"} offline RPC startup passed (project-only skills; MCP status verified)`);
	if (workflowDoctor) {
		process_.stdin.write(`${JSON.stringify({ id: "workflow-doctor", type: "prompt", message: "/workflow doctor" })}\n`);
		process_.stdin.flush();
		let doctorText = "";
		for (let index = 0; index < 200; index += 1) {
			const event = await nextEvent();
			if (event.message?.customType === "workflow") doctorText = String(event.message.content ?? "");
			if (event.id === "workflow-doctor" && event.type === "response") {
				if (!event.success) throw new Error("Workflow doctor RPC response failed");
				break;
			}
		}
		if (!/^Ready\./m.test(doctorText)) throw new Error(`Workflow doctor not ready: ${doctorText}`);
		console.log("Pi /workflow doctor passed");
	}
} finally {
	process_.stdin.end();
	process_.kill();
	await process_.exited;
}
