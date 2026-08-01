import { resolve } from "node:path";
import { FastMCP, imageContent, UserError } from "fastmcp";
import { z } from "zod";
import {
	assertBrpAccess,
	assertConsoleAccess,
	accessMode,
	isMutatingMethod,
} from "./access";
import { BrpClient, BrpRequestError, type Json } from "./brp/client";
import { captureViewport } from "./capture";

const repoRoot = resolve(import.meta.dir, "..", "..", "..");
const defaultPort = 15_702;
const defaultBrpUrl =
	process.env.BEVYOUT_BRP_URL ?? `http://127.0.0.1:${defaultPort}`;
const consoleSessionId = `mcp-${process.pid}-${crypto.randomUUID()}`;
const DEFAULT_CALL_TIMEOUT_MS = 15_000;
const VIEWER_LAUNCH_TIMEOUT_MS = 120_000;
const auditRecords: AuditRecord[] = [];

type AuditRecord = {
	timestamp: string;
	tool: string;
	method: string;
	session_id?: string;
	duration_ms: number;
	status: "success" | "error";
};

type ToolContext = {
	reportProgress?: (progress: {
		progress: number;
		total?: number;
	}) => Promise<void>;
	sessionId?: string;
	requestId?: string;
	signal?: AbortSignal;
	log?: { info?: (message: string) => void };
};

const outputSchema = z.record(z.string(), z.unknown());

function structured(result: Json, summary = JSON.stringify(result, null, 2)) {
	const structuredContent =
		result !== null && typeof result === "object" && !Array.isArray(result)
			? (result as Record<string, unknown>)
			: { value: result };
	return {
		content: [{ type: "text" as const, text: summary }],
		structuredContent,
	};
}

function toolError(error: unknown, fallback = "tool execution failed"): never {
	const message = error instanceof Error ? error.message : String(error);
	const status = error instanceof BrpRequestError ? error.kind : "error";
	throw new UserError(message || fallback, { status, error: message });
}

function recordAudit(
	tool: string,
	method: string,
	started: number,
	status: AuditRecord["status"],
	sessionId?: string,
): void {
	auditRecords.push({
		timestamp: new Date().toISOString(),
		tool,
		method,
		session_id: sessionId,
		duration_ms: Math.round(performance.now() - started),
		status,
	});
	if (auditRecords.length > 500) auditRecords.shift();
}

async function progress(
	context: ToolContext | undefined,
	value: number,
	total: number,
	label: string,
): Promise<void> {
	context?.log?.info?.(label);
	await context?.reportProgress?.({ progress: value, total });
}

function signalFrom(context: ToolContext | undefined): AbortSignal | undefined {
	return context?.signal;
}

async function wait(ms: number, signal?: AbortSignal): Promise<void> {
	if (signal?.aborted)
		throw new BrpRequestError("operation cancelled", "cancelled");
	await new Promise<void>((resolvePromise, reject) => {
		const timer = setTimeout(resolvePromise, ms);
		signal?.addEventListener(
			"abort",
			() => {
				clearTimeout(timer);
				reject(new BrpRequestError("operation cancelled", "cancelled"));
			},
			{ once: true },
		);
	});
}

async function readStream(
	stream: ReadableStream<Uint8Array>,
	logs: string[],
): Promise<void> {
	const reader = stream.getReader();
	const decoder = new TextDecoder();
	let buffer = "";
	while (true) {
		const chunk = await reader.read();
		if (chunk.done) {
			buffer += decoder.decode();
			if (buffer) {
				logs.push(buffer);
				if (logs.length > 500) logs.shift();
			}
			break;
		}
		buffer += decoder.decode(chunk.value, { stream: true });
		const lines = buffer.split(/\r?\n/);
		buffer = lines.pop() ?? "";
		for (const line of lines) {
			if (!line) continue;
			logs.push(line);
			if (logs.length > 500) logs.shift();
		}
	}
}

type ViewerState = {
	client: BrpClient;
	process?: Bun.Subprocess;
	logs: string[];
	owned: boolean;
};

let viewer: ViewerState | undefined;
let lastViewerFailure: string | undefined;

function clientFor(port = defaultPort): BrpClient {
	if (port === defaultPort && process.env.BEVYOUT_BRP_URL)
		return new BrpClient(defaultBrpUrl);
	return new BrpClient(`http://127.0.0.1:${port}`);
}

async function probe(
	client: BrpClient,
	timeoutMs = DEFAULT_CALL_TIMEOUT_MS,
	signal?: AbortSignal,
): Promise<Json> {
	return client.call("bevyout.session", undefined, { timeoutMs, signal });
}

async function waitForSession(
	client: BrpClient,
	child?: Bun.Subprocess,
	logs: string[] = [],
	timeoutMs = VIEWER_LAUNCH_TIMEOUT_MS,
	signal?: AbortSignal,
): Promise<Json> {
	const deadline = Date.now() + timeoutMs;
	let lastError = "viewer did not respond";
	while (Date.now() < deadline) {
		if (child?.exitCode !== null && child?.exitCode !== undefined) {
			await wait(50, signal);
			const output = logs.slice(-80).join("\n");
			throw new Error(
				`viewer process exited with code ${child.exitCode}${output ? `\nLast viewer output:\n${output}` : ""}`,
			);
		}
		try {
			return await probe(
				client,
				Math.min(DEFAULT_CALL_TIMEOUT_MS, Math.max(100, deadline - Date.now())),
				signal,
			);
		} catch (error) {
			lastError = error instanceof Error ? error.message : String(error);
			await wait(500, signal);
		}
	}
	const output = logs.slice(-80).join("\n");
	throw new BrpRequestError(
		`${lastError}; timed out waiting for Bevy Remote Protocol${output ? `\nLast viewer output:\n${output}` : ""}`,
		"timeout",
	);
}

async function status(): Promise<Json> {
	if (viewer) {
		try {
			const session = await probe(viewer.client, 3_000);
			return {
				connected: true,
				owned: viewer.owned,
				brp_url: viewer.client.baseUrl,
				session,
			};
		} catch (error) {
			if (viewer.process && viewer.process.exitCode !== null) {
				const output = viewer.logs.slice(-80).join("\n");
				lastViewerFailure = `viewer process exited with code ${viewer.process.exitCode}${output ? `\nLast viewer output:\n${output}` : ""}`;
				viewer = undefined;
			} else throw error;
		}
	}

	const client = clientFor();
	try {
		const session = await probe(client, 3_000);
		return { connected: true, owned: false, brp_url: client.baseUrl, session };
	} catch {
		return {
			connected: false,
			owned: false,
			brp_url: client.baseUrl,
			...(lastViewerFailure ? { error: lastViewerFailure } : {}),
		};
	}
}

async function stopOwnedViewer(sessionId?: string): Promise<Json> {
	if (!viewer)
		return { stopped: false, reason: "no viewer owned by this MCP process" };
	if (!viewer.owned || !viewer.process) {
		return {
			stopped: false,
			reason: "the connected viewer was started externally",
		};
	}

	const started = performance.now();
	const child = viewer.process;
	const pid = child.pid;
	if (process.platform === "win32") {
		const killer = Bun.spawn(["taskkill", "/PID", String(pid), "/T", "/F"], {
			stdout: "ignore",
			stderr: "ignore",
		});
		await killer.exited;
	} else {
		try {
			process.kill(-pid, "SIGTERM");
		} catch {
			// The process may have exited between the status check and the signal.
		}
		await Promise.race([child.exited, wait(5_000)]);
		if (child.exitCode === null || child.exitCode === undefined) {
			try {
				process.kill(-pid, "SIGKILL");
			} catch {
				// Already gone.
			}
		}
	}
	await child.exited;
	viewer = undefined;
	recordAudit("viewer_stop", "process_stop", started, "success", sessionId);
	return { stopped: true, pid };
}

const server = new FastMCP({
	name: "bevyout",
	version: "0.1.0",
	instructions:
		"Use .agents/skills/bevyout-mcp/SKILL.md for the live-scene workflow. Connect through loopback Bevy Remote Protocol; console and entity edits are runtime-only and may be overwritten by game systems.",
});

server.addTool({
	name: "viewer_status",
	description:
		"Report whether a bevyout viewer is connected and return its session metadata.",
	outputSchema,
	annotations: { readOnlyHint: true },
	execute: async () => structured(await status()),
});

server.addTool({
	name: "bridge_capabilities",
	description:
		"Return bridge API, build, runtime, and method capability metadata for the connected target.",
	outputSchema,
	annotations: { readOnlyHint: true },
	execute: async () => {
		const client = viewer?.client ?? clientFor();
		return structured(
			await client.call("bevyout.capabilities", undefined, {
				timeoutMs: DEFAULT_CALL_TIMEOUT_MS,
			}),
		);
	},
});

server.addTool({
	name: "viewer_launch",
	description:
		"Attach to an existing bevyout viewer or launch one from a cached selector or manifest.",
	parameters: z
		.object({
			selector: z
				.string()
				.optional()
				.describe("GECK EditorID or hexadecimal cell FormID"),
			manifest: z.string().optional().describe("Prepared scene manifest path"),
			disablePhysics: z.boolean().optional(),
			port: z.number().int().min(1024).max(65535).optional(),
		})
		.refine((value) => Boolean(value.selector) !== Boolean(value.manifest), {
			message: "Provide exactly one of selector or manifest",
		}),
	outputSchema,
	timeoutMs: VIEWER_LAUNCH_TIMEOUT_MS + 5_000,
	execute: async ({ selector, manifest, disablePhysics, port }, context) => {
		const started = performance.now();
		const effectivePort = port ?? defaultPort;
		const effectiveDisablePhysics = disablePhysics ?? false;
		const client = clientFor(effectivePort);
		if (viewer && viewer.client.baseUrl !== client.baseUrl) {
			throw new UserError(
				"another viewer is already tracked by this MCP process",
			);
		}
		await progress(context, 0, 4, "viewer_launch: compiling/attaching");
		try {
			const session = await probe(client, 3_000, signalFrom(context));
			viewer = { client, logs: [], owned: false };
			lastViewerFailure = undefined;
			const result = {
				attached: true,
				owned: false,
				session,
				access_mode: accessMode,
			};
			recordAudit(
				"viewer_launch",
				"attach",
				started,
				"success",
				context.sessionId,
			);
			await progress(context, 4, 4, "viewer_launch: scene_ready");
			return structured(result);
		} catch (error) {
			if (
				error instanceof BrpRequestError &&
				(error.kind === "timeout" || error.kind === "cancelled")
			) {
				toolError(error);
			}
			// No existing endpoint: launch below.
		}
		if (viewer)
			throw new UserError(
				"another viewer is already tracked by this MCP process",
			);

		const args = ["run-dev", "--", selector ? "render" : "view"];
		if (selector) args.push(selector);
		else {
			if (!manifest)
				throw new UserError("manifest is required when selector is absent");
			args.push("--manifest", resolve(repoRoot, manifest));
		}
		if (effectiveDisablePhysics) args.push("--disable-physics");
		args.push("--agent-bridge", "--agent-port", String(effectivePort));

		const logs: string[] = [];
		lastViewerFailure = undefined;
		// Bun's detached option calls setsid() on POSIX (including macOS) and
		// creates an independent process group on Windows. Do not shell out to
		// the optional `setsid` utility: macOS does not ship it by default.
		const child = Bun.spawn(["cargo", ...args], {
			cwd: repoRoot,
			stdin: "ignore",
			stdout: "pipe",
			stderr: "pipe",
			detached: true,
		});
		await progress(context, 1, 4, "viewer_launch: process_started");
		void readStream(child.stdout, logs);
		void readStream(child.stderr, logs);
		viewer = { client, process: child, logs, owned: true };
		void child.exited.then((exitCode) => {
			if (viewer?.process?.pid === child.pid) viewer = undefined;
			if (exitCode !== 0) {
				const output = logs.slice(-80).join("\n");
				lastViewerFailure = `viewer process exited with code ${exitCode}${output ? `\nLast viewer output:\n${output}` : ""}`;
			}
		});

		try {
			await progress(context, 2, 4, "viewer_launch: waiting_for_bridge");
			const session = await waitForSession(
				client,
				child,
				logs,
				VIEWER_LAUNCH_TIMEOUT_MS,
				signalFrom(context),
			);
			const result = {
				attached: true,
				owned: true,
				pid: child.pid,
				session,
				access_mode: accessMode,
			};
			recordAudit(
				"viewer_launch",
				"process_start",
				started,
				"success",
				context.sessionId,
			);
			await progress(context, 4, 4, "viewer_launch: scene_ready");
			return structured(result);
		} catch (error) {
			recordAudit(
				"viewer_launch",
				"process_start",
				started,
				"error",
				context.sessionId,
			);
			await stopOwnedViewer(context.sessionId);
			toolError(error);
		}
	},
});

server.addTool({
	name: "viewer_stop",
	description: "Stop only a viewer process launched by this MCP server.",
	outputSchema,
	annotations: { destructiveHint: true },
	execute: async (_args, context) =>
		structured(await stopOwnedViewer(context.sessionId)),
});

server.addTool({
	name: "viewer_logs",
	description:
		"Return the captured tail of logs from a viewer launched by this MCP server.",
	parameters: z.object({ tail: z.number().int().min(1).max(500).optional() }),
	execute: async ({ tail }) => {
		if (!viewer?.owned)
			return (
				lastViewerFailure ??
				"No logs are available for an externally started viewer."
			);
		return viewer.logs.slice(-(tail ?? 100)).join("\n");
	},
});

server.addTool({
	name: "mcp_audit",
	description: "Return recent MCP mutation and process-control audit records.",
	outputSchema,
	annotations: { readOnlyHint: true },
	execute: async () =>
		structured({ records: auditRecords.slice(-100), access_mode: accessMode }),
});

server.addTool({
	name: "scene_snapshot",
	description:
		"Return compact bevyout-aware placement, camera, player, and light entity data.",
	parameters: z.object({
		offset: z.number().int().min(0).optional(),
		limit: z.number().int().min(1).max(1000).optional(),
		role: z
			.enum(["placement", "camera", "player", "light", "entity"])
			.optional(),
		nameContains: z.string().optional(),
		includeOther: z.boolean().optional(),
		includeTotal: z
			.boolean()
			.optional()
			.describe(
				"When false, the bridge returns a bounded page without scanning the full match set",
			),
	}),
	outputSchema,
	annotations: { readOnlyHint: true },
	execute: async ({
		offset,
		limit,
		role,
		nameContains,
		includeOther,
		includeTotal,
	}) => {
		const effectiveOffset = offset ?? 0;
		const effectiveLimit = limit ?? 100;
		const effectiveIncludeOther = includeOther ?? false;
		const client = viewer?.client ?? clientFor();
		const result = await client.call(
			"bevyout.scene_snapshot",
			{
				offset: effectiveOffset,
				limit: effectiveLimit,
				...(role ? { role } : {}),
				...(nameContains ? { name_contains: nameContains } : {}),
				include_other: effectiveIncludeOther,
				include_total: includeTotal ?? true,
			},
			{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS },
		);
		return structured(result);
	},
});

server.addTool({
	name: "performance_snapshot",
	description:
		"Summarize a bounded recent frame window and return current Bevy diagnostics and world counts.",
	parameters: z.object({
		afterSample: z.number().int().min(0).optional(),
		latestLimit: z.number().int().min(1).max(600).optional(),
		budgetMs: z.number().positive().optional(),
		includeSamples: z.boolean().optional(),
	}),
	outputSchema,
	annotations: { readOnlyHint: true },
	execute: async ({ afterSample, latestLimit, budgetMs, includeSamples }) => {
		const client = viewer?.client ?? clientFor();
		const result = await client.call(
			"bevyout.performance_snapshot",
			{
				...(afterSample === undefined ? {} : { after_sample: afterSample }),
				latest_limit: latestLimit ?? 600,
				budget_ms: budgetMs ?? 16.667,
				include_samples: includeSamples ?? false,
			},
			{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS },
		);
		return structured(result);
	},
});

server.addTool({
	name: "performance_probe",
	description:
		"Measure an exact timed frame window in the connected viewer, excluding an optional warmup period.",
	parameters: z.object({
		durationMs: z.number().int().min(100).max(60_000).optional(),
		warmupMs: z.number().int().min(0).max(30_000).optional(),
		latestLimit: z.number().int().min(1).max(600).optional(),
		budgetMs: z.number().positive().optional(),
		includeSamples: z.boolean().optional(),
	}),
	outputSchema,
	timeoutMs: 95_000,
	annotations: { readOnlyHint: true },
	execute: async (
		{ durationMs, warmupMs, latestLimit, budgetMs, includeSamples },
		context,
	) => {
		const client = viewer?.client ?? clientFor();
		const effectiveWarmupMs = warmupMs ?? 1_000;
		const effectiveDurationMs = durationMs ?? 5_000;
		const signal = signalFrom(context);
		await progress(context, 0, 3, "performance_probe: warmup");
		if (effectiveWarmupMs > 0) await wait(effectiveWarmupMs, signal);
		const baseline = (await client.call(
			"bevyout.performance_snapshot",
			{
				latest_limit: 1,
				budget_ms: budgetMs ?? 16.667,
				include_samples: false,
			},
			{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS, signal },
		)) as { latest_sample?: number | null };
		await progress(context, 1, 3, "performance_probe: sampling");
		await wait(effectiveDurationMs, signal);
		const result = await client.call(
			"bevyout.performance_snapshot",
			{
				...(baseline.latest_sample === undefined ||
				baseline.latest_sample === null
					? {}
					: { after_sample: baseline.latest_sample }),
				latest_limit: latestLimit ?? 600,
				budget_ms: budgetMs ?? 16.667,
				include_samples: includeSamples ?? false,
			},
			{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS, signal },
		);
		await progress(context, 3, 3, "performance_probe: collecting_results");
		return structured({
			warmup_ms: effectiveWarmupMs,
			requested_duration_ms: effectiveDurationMs,
			start_after_sample: baseline.latest_sample ?? null,
			result,
		});
	},
});

server.addTool({
	name: "schedule_snapshot",
	description:
		"Inspect assembled Bevy schedules, system execution traits, and data-access conflict pairs that constrain parallelism.",
	parameters: z.object({
		scheduleContains: z.string().optional(),
		includeSystems: z.boolean().optional(),
		conflictLimit: z.number().int().min(0).max(1000).optional(),
	}),
	outputSchema,
	annotations: { readOnlyHint: true },
	execute: async ({ scheduleContains, includeSystems, conflictLimit }) => {
		const client = viewer?.client ?? clientFor();
		const result = await client.call(
			"bevyout.schedule_snapshot",
			{
				...(scheduleContains ? { schedule_contains: scheduleContains } : {}),
				include_systems: includeSystems ?? false,
				conflict_limit: conflictLimit ?? 100,
			},
			{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS },
		);
		return structured(result);
	},
});

server.addTool({
	name: "console_exec",
	description:
		"Execute one Gamebryo-style command through bevyout's structured console core.",
	parameters: z.object({
		line: z.string().min(1).max(16_384),
		session: z.string().min(1).max(128).optional(),
	}),
	outputSchema,
	annotations: { destructiveHint: true },
	execute: async ({ line, session }, context) => {
		assertConsoleAccess(line);
		const started = performance.now();
		const client = viewer?.client ?? clientFor();
		try {
			const result = await client.call(
				"bevyout.console.exec",
				{
					line,
					session: session ?? consoleSessionId,
				},
				{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS, signal: signalFrom(context) },
			);
			recordAudit(
				"console_exec",
				"bevyout.console.exec",
				started,
				"success",
				context.sessionId,
			);
			return structured(result);
		} catch (error) {
			recordAudit(
				"console_exec",
				"bevyout.console.exec",
				started,
				"error",
				context.sessionId,
			);
			toolError(error);
		}
	},
});

server.addTool({
	name: "console_help",
	description:
		"Return structured metadata for every registered bevyout console command.",
	outputSchema,
	annotations: { readOnlyHint: true },
	execute: async () => {
		const client = viewer?.client ?? clientFor();
		return structured(
			await client.call("bevyout.console.help", undefined, {
				timeoutMs: DEFAULT_CALL_TIMEOUT_MS,
			}),
		);
	},
});

server.addTool({
	name: "world_query",
	description:
		"Query reflected ECS components with optional BRP filters. Pagination is client-side because standard world.query returns the full response; use scene_snapshot for genuinely bounded bridge pagination.",
	parameters: z.object({
		components: z.array(z.string()).optional(),
		optional: z.array(z.string()).optional(),
		has: z.array(z.string()).optional(),
		with: z.array(z.string()).optional(),
		without: z.array(z.string()).optional(),
		strict: z.boolean().optional(),
		offset: z.number().int().min(0).optional(),
		limit: z.number().int().min(1).max(1000).optional(),
	}),
	execute: async ({
		components,
		optional,
		has,
		with: withTypes,
		without,
		strict,
		offset,
		limit,
	}) => {
		const effectiveComponents = components ?? [];
		const effectiveOptional = optional ?? [];
		const effectiveHas = has ?? [];
		const effectiveWith = withTypes ?? [];
		const effectiveWithout = without ?? [];
		const effectiveStrict = strict ?? false;
		const effectiveOffset = offset ?? 0;
		const effectiveLimit = limit ?? 100;
		const client = viewer?.client ?? clientFor();
		const result = await client.call(
			"world.query",
			{
				data: {
					components: effectiveComponents,
					option: effectiveOptional,
					has: effectiveHas,
				},
				filter: { with: effectiveWith, without: effectiveWithout },
				strict: effectiveStrict,
			},
			{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS },
		);
		if (!Array.isArray(result)) return structured(result);
		return structured({
			rows: result.slice(effectiveOffset, effectiveOffset + effectiveLimit),
			total: result.length,
			offset: effectiveOffset,
			limit: effectiveLimit,
			bounded_on_bridge: false,
		});
	},
});

server.addTool({
	name: "brp_call",
	description:
		"Call any instantaneous Bevy Remote Protocol method, including ECS reads and writes.",
	parameters: z.object({
		method: z.string().min(1),
		params: z.any().optional(),
	}),
	timeoutMs: DEFAULT_CALL_TIMEOUT_MS + 1_000,
	execute: async ({ method, params }, context) => {
		if (method.endsWith("+watch"))
			throw new UserError("Use brp_watch for streaming BRP methods");
		assertBrpAccess(method);
		const started = performance.now();
		const client = viewer?.client ?? clientFor();
		try {
			const result = await client.call(method, params, {
				timeoutMs: DEFAULT_CALL_TIMEOUT_MS,
				signal: signalFrom(context),
			});
			if (isMutatingMethod(method))
				recordAudit("brp_call", method, started, "success", context.sessionId);
			return structured(result);
		} catch (error) {
			if (isMutatingMethod(method))
				recordAudit("brp_call", method, started, "error", context.sessionId);
			toolError(error);
		}
	},
});

server.addTool({
	name: "brp_watch",
	description:
		"Collect a bounded batch from a Bevy Remote Protocol watch method.",
	parameters: z.object({
		method: z.string().min(1),
		params: z.any().optional(),
		timeoutMs: z.number().int().min(100).max(60_000).optional(),
		maxEvents: z.number().int().min(1).max(100).optional(),
	}),
	outputSchema,
	timeoutMs: 65_000,
	annotations: { readOnlyHint: true },
	execute: async ({ method, params, timeoutMs, maxEvents }, context) => {
		if (!method.endsWith("+watch"))
			throw new UserError("brp_watch requires a method ending in +watch");
		assertBrpAccess(method.slice(0, -6));
		const client = viewer?.client ?? clientFor();
		const result = await client.watch(method, params ?? null, {
			timeoutMs: timeoutMs ?? 10_000,
			maxEvents: maxEvents ?? 20,
			signal: signalFrom(context),
		});
		return structured(result);
	},
});

server.addTool({
	name: "viewport_capture",
	description:
		"Capture the visible primary Bevy window and return it as MCP image content.",
	annotations: { readOnlyHint: true },
	execute: async (_args, context) => {
		const client = viewer?.client ?? clientFor();
		const { buffer, empty } = await captureViewport(
			client,
			signalFrom(context),
		);
		if (empty) {
			return {
				content: [
					{
						type: "text",
						text: "Viewport captured an empty (0-byte) frame; the game window is likely not foreground/visible (common on macOS when occluded).",
					},
				],
			};
		}
		return {
			content: [
				{
					type: "text",
					text: "Viewport captured; the game window must remain visible to the renderer.",
				},
				await imageContent({ buffer }),
			],
		};
	},
});

/** Builds the `cam` console line that engages the cinema camera on a subject. */
function cinemaEngageLine(
	mode: "follow" | "orbit",
	subject: string,
	dist?: number,
	height?: number,
	radius?: number,
): string {
	if (mode === "orbit") {
		return `cam orbit ${subject} ${radius ?? 3}`;
	}
	if (dist === undefined && height === undefined)
		return `cam follow ${subject}`;
	return `cam follow ${subject} ${dist ?? 4} ${height ?? 2}`;
}

server.addTool({
	name: "cinema_record",
	description:
		"Engage the cinema debug camera (issue #209) on a subject FormID and capture a short filmstrip of viewport frames in one call, so an agent can assess motion instead of reasoning from a single still.",
	parameters: z.object({
		subject: z
			.string()
			.min(1)
			.describe("FormID selector to track, e.g. 0005cf10 or 0x0005cf10"),
		frames: z
			.number()
			.int()
			.min(1)
			.max(8)
			.optional()
			.describe("Number of frames to capture (default 4, max 8)"),
		intervalMs: z
			.number()
			.int()
			.min(50)
			.max(10_000)
			.optional()
			.describe("Spacing between frames in ms (default 500)"),
		mode: z
			.enum(["follow", "orbit"])
			.optional()
			.describe("Cinema camera mode (default follow)"),
		dist: z.number().optional().describe("Follow distance (follow mode only)"),
		height: z.number().optional().describe("Follow height (follow mode only)"),
		radius: z.number().optional().describe("Orbit radius (orbit mode only)"),
	}),
	annotations: { destructiveHint: true },
	timeoutMs: 120_000,
	execute: async (
		{ subject, frames, intervalMs, mode, dist, height, radius },
		context,
	) => {
		assertConsoleAccess(`cam follow ${subject}`);
		const effectiveFrames = frames ?? 4;
		const effectiveIntervalMs = intervalMs ?? 500;
		const effectiveMode = mode ?? "follow";
		const client = viewer?.client ?? clientFor();
		const signal = signalFrom(context);

		const engageLine = cinemaEngageLine(
			effectiveMode,
			subject,
			dist,
			height,
			radius,
		);
		const engageResult = await client.call(
			"bevyout.console.exec",
			{
				line: engageLine,
				session: consoleSessionId,
			},
			{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS, signal },
		);
		await progress(
			context,
			0,
			effectiveFrames + 1,
			"cinema_record: camera_engaged",
		);

		const content: Array<
			{ type: "text"; text: string } | Awaited<ReturnType<typeof imageContent>>
		> = [
			{
				type: "text",
				text: `Engaged '${engageLine}' -> ${JSON.stringify(engageResult)}`,
			},
		];

		for (let index = 0; index < effectiveFrames; index += 1) {
			await progress(
				context,
				index + 1,
				effectiveFrames + 1,
				`cinema_record: frame_${index + 1}_of_${effectiveFrames}`,
			);
			const { buffer, empty } = await captureViewport(client, signal);

			let camStatus: Json = null;
			let tnaStatus: Json = null;
			try {
				camStatus = await client.call(
					"bevyout.console.exec",
					{ line: "cam status", session: consoleSessionId },
					{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS, signal },
				);
			} catch {
				// Best-effort: status is a bonus alongside the frame, not required.
			}
			try {
				tnaStatus = await client.call(
					"bevyout.console.exec",
					{ line: "tna status", session: consoleSessionId },
					{ timeoutMs: DEFAULT_CALL_TIMEOUT_MS, signal },
				);
			} catch {
				// tna may have no bound agent; that is not a reason to fail the capture.
			}

			const emptyNote = empty
				? " [EMPTY/black frame: game window not foreground/visible]"
				: "";
			content.push({
				type: "text",
				text: `frame ${index + 1}/${effectiveFrames} t+${index * effectiveIntervalMs}ms${emptyNote} cam_status=${JSON.stringify(
					camStatus,
				)} tna_status=${JSON.stringify(tnaStatus)}`,
			});
			if (!empty) content.push(await imageContent({ buffer }));

			if (index < effectiveFrames - 1) await wait(effectiveIntervalMs, signal);
		}

		return { content };
	},
});

let shuttingDown = false;
async function shutdown(): Promise<void> {
	if (shuttingDown) return;
	shuttingDown = true;
	await stopOwnedViewer();
	await server.stop();
	process.exit(0);
}

process.once("SIGINT", () => void shutdown());
process.once("SIGTERM", () => void shutdown());

await server.start({ transportType: "stdio" });
