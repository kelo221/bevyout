import { mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { accessMode, assertBrpAccess, assertConsoleAccess } from "./access";
import { BrpClient, type Json } from "./brp/client";
import { captureViewport } from "./capture";

const DEFAULT_URL = "http://127.0.0.1:15702";
const DEFAULT_TIMEOUT_MS = 15_000;
const DEFAULT_MAX_BYTES = 32_768;

export type CliArgs = {
	command: string;
	options: Map<string, string | true>;
};

export function parseArgs(argv: string[]): CliArgs {
	const command = argv[0] ?? "help";
	const options = new Map<string, string | true>();
	for (let index = 1; index < argv.length; index += 1) {
		const token = argv[index];
		if (!token?.startsWith("--")) throw new Error(`unexpected argument '${token}'`);
		const key = token.slice(2);
		const next = argv[index + 1];
		if (next !== undefined && !next.startsWith("--")) {
			options.set(key, next);
			index += 1;
		} else {
			options.set(key, true);
		}
	}
	return { command, options };
}

function stringOption(args: CliArgs, key: string): string | undefined {
	const value = args.options.get(key);
	return typeof value === "string" ? value : undefined;
}

function numberOption(args: CliArgs, key: string, fallback?: number): number | undefined {
	const raw = stringOption(args, key);
	if (raw === undefined) return fallback;
	const value = Number(raw);
	if (!Number.isFinite(value)) throw new Error(`--${key} must be numeric`);
	return value;
}

function boolOption(args: CliArgs, key: string): boolean {
	return args.options.get(key) === true;
}

export function boundedJson(value: unknown, maxBytes = DEFAULT_MAX_BYTES): string {
	if (!Number.isInteger(maxBytes) || maxBytes < 512 || maxBytes > 262_144)
		throw new Error("--max-bytes must be between 512 and 262144");
	const complete = JSON.stringify(value, null, 2);
	const bytes = Buffer.byteLength(complete);
	if (bytes <= maxBytes) return complete;
	let previewLength = Math.max(1, maxBytes - 200);
	let bounded = "";
	do {
		bounded = JSON.stringify(
			{ truncated: true, original_bytes: bytes, preview: complete.slice(0, previewLength) },
			null,
			2,
		);
		previewLength = Math.max(0, previewLength - 128);
	} while (Buffer.byteLength(bounded) > maxBytes && previewLength > 0);
	return bounded;
}

function parseJsonOption(args: CliArgs, key: string): unknown {
	const raw = stringOption(args, key);
	if (raw === undefined) return undefined;
	try {
		return JSON.parse(raw);
	} catch (error) {
		throw new Error(`--${key} must be valid JSON: ${error instanceof Error ? error.message : String(error)}`);
	}
}

async function performanceProbe(client: BrpClient, args: CliArgs): Promise<Json> {
	const warmupMs = numberOption(args, "warmup-ms", 1_000) ?? 1_000;
	const durationMs = numberOption(args, "duration-ms", 5_000) ?? 5_000;
	if (warmupMs < 0 || warmupMs > 30_000 || durationMs < 100 || durationMs > 60_000)
		throw new Error("performance bounds: warmup 0..30000 ms, duration 100..60000 ms");
	if (warmupMs > 0) await Bun.sleep(warmupMs);
	const baseline = (await client.call(
		"bevyout.performance_snapshot",
		{ latest_limit: 1, include_samples: false },
		{ timeoutMs: DEFAULT_TIMEOUT_MS },
	)) as { latest_sample?: number | null };
	await Bun.sleep(durationMs);
	const result = await client.call(
		"bevyout.performance_snapshot",
		{
			...(baseline.latest_sample == null ? {} : { after_sample: baseline.latest_sample }),
			latest_limit: numberOption(args, "limit", 600),
			budget_ms: numberOption(args, "budget-ms", 16.667),
			include_samples: boolOption(args, "include-samples"),
		},
		{ timeoutMs: DEFAULT_TIMEOUT_MS },
	);
	return { warmup_ms: warmupMs, requested_duration_ms: durationMs, start_after_sample: baseline.latest_sample ?? null, result };
}

export async function execute(args: CliArgs): Promise<unknown> {
	const url = stringOption(args, "url") ?? process.env.BEVYOUT_BRP_URL ?? DEFAULT_URL;
	const client = new BrpClient(url);
	switch (args.command) {
		case "help":
			return {
				commands: ["status", "capabilities", "scene", "console", "console-help", "schedule", "performance", "capture", "brp"],
				access_mode: accessMode,
				common: "--url URL --max-bytes N",
			};
		case "status":
			return { access_mode: accessMode, session: await client.call("bevyout.session", undefined, { timeoutMs: DEFAULT_TIMEOUT_MS }) };
		case "capabilities":
			return client.call("bevyout.capabilities", undefined, { timeoutMs: DEFAULT_TIMEOUT_MS });
		case "scene":
			return client.call("bevyout.scene_snapshot", {
				offset: numberOption(args, "offset", 0),
				limit: numberOption(args, "limit", 100),
				...(stringOption(args, "role") ? { role: stringOption(args, "role") } : {}),
				...(stringOption(args, "name") ? { name_contains: stringOption(args, "name") } : {}),
				include_other: boolOption(args, "include-other"),
				include_total: !boolOption(args, "no-total"),
			}, { timeoutMs: DEFAULT_TIMEOUT_MS });
		case "console": {
			const line = stringOption(args, "line");
			if (!line) throw new Error("console requires --line");
			assertConsoleAccess(line);
			return client.call("bevyout.console.exec", { line, session: stringOption(args, "session") ?? `cli-${process.pid}` }, { timeoutMs: DEFAULT_TIMEOUT_MS });
		}
		case "console-help":
			return client.call("bevyout.console.help", undefined, { timeoutMs: DEFAULT_TIMEOUT_MS });
		case "schedule":
			return client.call("bevyout.schedule_snapshot", {
				...(stringOption(args, "contains") ? { schedule_contains: stringOption(args, "contains") } : {}),
				include_systems: boolOption(args, "include-systems"),
				conflict_limit: numberOption(args, "conflict-limit", 100),
			}, { timeoutMs: DEFAULT_TIMEOUT_MS });
		case "performance":
			if (args.options.has("duration-ms")) return performanceProbe(client, args);
			return client.call("bevyout.performance_snapshot", {
				latest_limit: numberOption(args, "limit", 600),
				budget_ms: numberOption(args, "budget-ms", 16.667),
				include_samples: boolOption(args, "include-samples"),
			}, { timeoutMs: DEFAULT_TIMEOUT_MS });
		case "capture": {
			const output = resolve(stringOption(args, "out") ?? `.bevyout/captures/cli-${Date.now()}.png`);
			const capture = await captureViewport(client, undefined, numberOption(args, "timeout-ms", 10_000));
			if (capture.empty) throw new Error("viewport capture was empty; keep the viewer visible and unoccluded");
			await mkdir(dirname(output), { recursive: true });
			await writeFile(output, capture.buffer);
			return { path: output, bytes: capture.buffer.length };
		}
		case "brp": {
			const method = stringOption(args, "method");
			if (!method) throw new Error("brp requires --method");
			assertBrpAccess(method);
			return client.call(method, parseJsonOption(args, "params"), { timeoutMs: numberOption(args, "timeout-ms", DEFAULT_TIMEOUT_MS) });
		}
		default:
			throw new Error(`unknown command '${args.command}'`);
	}
}

if (import.meta.main) {
	try {
		const args = parseArgs(Bun.argv.slice(2));
		const result = await execute(args);
		const maxBytes = numberOption(args, "max-bytes", DEFAULT_MAX_BYTES) ?? DEFAULT_MAX_BYTES;
		process.stdout.write(`${boundedJson(result, maxBytes)}\n`);
	} catch (error) {
		process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
		process.exitCode = 1;
	}
}
