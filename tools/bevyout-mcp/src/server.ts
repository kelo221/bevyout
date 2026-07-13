import { rm } from "node:fs/promises";
import { resolve } from "node:path";
import { FastMCP, imageContent } from "fastmcp";
import { z } from "zod";

const repoRoot = resolve(import.meta.dir, "..", "..", "..");
const defaultPort = 15_702;
const defaultBrpUrl = process.env.BEVYOUT_BRP_URL ?? `http://127.0.0.1:${defaultPort}`;

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type BrpError = { code?: number; message?: string; data?: Json };
type BrpResponse = { result?: Json; error?: BrpError };

class BrpClient {
  private nextId = 1;

  constructor(readonly baseUrl: string) {}

  async call(method: string, params?: unknown): Promise<Json> {
    const response = await fetch(`${this.baseUrl.replace(/\/$/, "")}/`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        jsonrpc: "2.0",
        id: this.nextId++,
        method,
        ...(params === undefined ? {} : { params }),
      }),
    });
    if (!response.ok) {
      throw new Error(`BRP HTTP ${response.status}: ${await response.text()}`);
    }
    const payload = (await response.json()) as BrpResponse;
    if (payload.error) {
      const suffix = payload.error.data === undefined ? "" : ` (${JSON.stringify(payload.error.data)})`;
      throw new Error(`BRP ${payload.error.code ?? "error"}: ${payload.error.message ?? "request failed"}${suffix}`);
    }
    return payload.result ?? null;
  }

  async watch(method: string, params: unknown, timeoutMs: number, maxEvents: number): Promise<Json[]> {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), timeoutMs);
    try {
      const response = await fetch(`${this.baseUrl.replace(/\/$/, "")}/`, {
        method: "POST",
        headers: { "content-type": "application/json", accept: "text/event-stream" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: this.nextId++,
          method,
          params,
        }),
        signal: controller.signal,
      });
      if (!response.ok || !response.body) {
        throw new Error(`BRP watch HTTP ${response.status}: ${await response.text()}`);
      }

      const reader = response.body.getReader();
      const decoder = new TextDecoder();
      let buffer = "";
      const events: Json[] = [];
      while (events.length < maxEvents) {
        const chunk = await reader.read();
        if (chunk.done) break;
        buffer += decoder.decode(chunk.value, { stream: true });
        const lines = buffer.split(/\r?\n/);
        buffer = lines.pop() ?? "";
        for (const line of lines) {
          if (!line.startsWith("data: ")) continue;
          const payload = JSON.parse(line.slice(6)) as BrpResponse;
          if (payload.error) {
            throw new Error(`BRP ${payload.error.code ?? "error"}: ${payload.error.message ?? "watch failed"}`);
          }
          const result = payload.result;
          if (Array.isArray(result)) events.push(...result);
          else if (result !== undefined && result !== null) events.push(result);
          if (events.length >= maxEvents) break;
        }
      }
      await reader.cancel();
      return events.slice(0, maxEvents);
    } catch (error) {
      if (controller.signal.aborted) {
        return [];
      }
      throw error;
    } finally {
      clearTimeout(timeout);
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

function clientFor(port = defaultPort): BrpClient {
  if (port === defaultPort && process.env.BEVYOUT_BRP_URL) return new BrpClient(defaultBrpUrl);
  return new BrpClient(`http://127.0.0.1:${port}`);
}

async function probe(client: BrpClient): Promise<Json> {
  return client.call("bevyout.session");
}

async function readStream(stream: ReadableStream<Uint8Array>, logs: string[]): Promise<void> {
  const reader = stream.getReader();
  const decoder = new TextDecoder();
  while (true) {
    const chunk = await reader.read();
    if (chunk.done) break;
    const text = decoder.decode(chunk.value, { stream: true });
    for (const line of text.split(/\r?\n/)) {
      if (!line) continue;
      logs.push(line);
      if (logs.length > 500) logs.shift();
    }
  }
}

function wait(ms: number): Promise<void> {
  return new Promise((resolvePromise) => setTimeout(resolvePromise, ms));
}

async function waitForSession(client: BrpClient, timeoutMs = 120_000): Promise<Json> {
  const deadline = Date.now() + timeoutMs;
  let lastError = "viewer did not respond";
  while (Date.now() < deadline) {
    try {
      return await probe(client);
    } catch (error) {
      lastError = error instanceof Error ? error.message : String(error);
      await wait(500);
    }
  }
  throw new Error(`${lastError}; timed out waiting for Bevy Remote Protocol`);
}

async function status(): Promise<Json> {
  if (viewer) {
    try {
      const session = await probe(viewer.client);
      return { connected: true, owned: viewer.owned, brp_url: viewer.client.baseUrl, session };
    } catch (error) {
      if (viewer.process && viewer.process.exitCode !== null) viewer = undefined;
      else throw error;
    }
  }

  const client = clientFor();
  try {
    const session = await probe(client);
    return { connected: true, owned: false, brp_url: client.baseUrl, session };
  } catch {
    return { connected: false, owned: false, brp_url: client.baseUrl };
  }
}

async function stopOwnedViewer(): Promise<Json> {
  if (!viewer) return { stopped: false, reason: "no viewer owned by this MCP process" };
  if (!viewer.owned || !viewer.process) {
    return { stopped: false, reason: "the connected viewer was started externally" };
  }

  const pid = viewer.process.pid;
  if (process.platform === "win32") {
    const killer = Bun.spawn(["taskkill", "/PID", String(pid), "/T", "/F"], {
      stdout: "ignore",
      stderr: "ignore",
    });
    await killer.exited;
  } else {
    viewer.process.kill();
  }
  viewer = undefined;
  return { stopped: true, pid };
}

const server = new FastMCP({
  name: "bevyout",
  version: "0.1.0",
  instructions:
    "Connect to a running bevyout viewer through Bevy Remote Protocol. Entity edits are runtime-only and may be overwritten by game systems.",
});

server.addTool({
  name: "viewer_status",
  description: "Report whether a bevyout viewer is connected and return its session metadata.",
  execute: async () => JSON.stringify(await status(), null, 2),
});

server.addTool({
  name: "viewer_launch",
  description: "Attach to an existing bevyout viewer or launch one from a cached selector or manifest.",
  parameters: z
    .object({
      selector: z.string().optional().describe("GECK EditorID or hexadecimal cell FormID"),
      manifest: z.string().optional().describe("Prepared scene manifest path"),
      disablePhysics: z.boolean().optional(),
      port: z.number().int().min(1024).max(65535).optional(),
    })
    .refine((value) => Boolean(value.selector) !== Boolean(value.manifest), {
      message: "Provide exactly one of selector or manifest",
    }),
  execute: async ({ selector, manifest, disablePhysics, port }) => {
    const effectivePort = port ?? defaultPort;
    const effectiveDisablePhysics = disablePhysics ?? false;
    const client = clientFor(effectivePort);
    try {
      const session = await probe(client);
      viewer = { client, logs: [], owned: false };
      return JSON.stringify({ attached: true, owned: false, session }, null, 2);
    } catch {
      // No existing endpoint: launch below.
    }
    if (viewer) throw new Error("another viewer is already tracked by this MCP process");

    const args = ["run-dev", "--", selector ? "render" : "view"];
    if (selector) args.push(selector);
    else args.push("--manifest", resolve(repoRoot, manifest!));
    if (effectiveDisablePhysics) args.push("--disable-physics");
    args.push("--agent-bridge", "--agent-port", String(effectivePort));

    const logs: string[] = [];
    const child = Bun.spawn(["cargo", ...args], {
      cwd: repoRoot,
      stdin: "ignore",
      stdout: "pipe",
      stderr: "pipe",
    });
    void readStream(child.stdout, logs);
    void readStream(child.stderr, logs);
    viewer = { client, process: child, logs, owned: true };
    void child.exited.then(() => {
      if (viewer?.process?.pid === child.pid) viewer = undefined;
    });

    try {
      const session = await waitForSession(client);
      return JSON.stringify({ attached: true, owned: true, pid: child.pid, session }, null, 2);
    } catch (error) {
      await stopOwnedViewer();
      throw error;
    }
  },
});

server.addTool({
  name: "viewer_stop",
  description: "Stop only a viewer process launched by this MCP server.",
  execute: async () => JSON.stringify(await stopOwnedViewer(), null, 2),
});

server.addTool({
  name: "viewer_logs",
  description: "Return the captured tail of logs from a viewer launched by this MCP server.",
  parameters: z.object({ tail: z.number().int().min(1).max(500).optional() }),
  execute: async ({ tail }) => {
    if (!viewer?.owned) return "No logs are available for an externally started viewer.";
    return viewer.logs.slice(-(tail ?? 100)).join("\n");
  },
});

server.addTool({
  name: "scene_snapshot",
  description: "Return compact bevyout-aware placement, camera, player, and light entity data.",
  parameters: z.object({
    offset: z.number().int().min(0).optional(),
    limit: z.number().int().min(1).max(1000).optional(),
    role: z.enum(["placement", "camera", "player", "light", "entity"]).optional(),
    nameContains: z.string().optional(),
    includeOther: z.boolean().optional(),
  }),
  execute: async ({ offset, limit, role, nameContains, includeOther }) => {
    const effectiveOffset = offset ?? 0;
    const effectiveLimit = limit ?? 100;
    const effectiveIncludeOther = includeOther ?? false;
    const client = viewer?.client ?? clientFor();
    const result = await client.call("bevyout.scene_snapshot", {
      offset: effectiveOffset,
      limit: effectiveLimit,
      ...(role ? { role } : {}),
      ...(nameContains ? { name_contains: nameContains } : {}),
      include_other: effectiveIncludeOther,
    });
    return JSON.stringify(result, null, 2);
  },
});

server.addTool({
  name: "world_query",
  description: "Query reflected ECS components with optional BRP filters and compact client-side pagination.",
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
  execute: async ({ components, optional, has, with: withTypes, without, strict, offset, limit }) => {
    const effectiveComponents = components ?? [];
    const effectiveOptional = optional ?? [];
    const effectiveHas = has ?? [];
    const effectiveWith = withTypes ?? [];
    const effectiveWithout = without ?? [];
    const effectiveStrict = strict ?? false;
    const effectiveOffset = offset ?? 0;
    const effectiveLimit = limit ?? 100;
    const client = viewer?.client ?? clientFor();
    const result = await client.call("world.query", {
      data: { components: effectiveComponents, option: effectiveOptional, has: effectiveHas },
      filter: { with: effectiveWith, without: effectiveWithout },
      strict: effectiveStrict,
    });
    if (!Array.isArray(result)) return JSON.stringify(result, null, 2);
    return JSON.stringify(
      {
        rows: result.slice(effectiveOffset, effectiveOffset + effectiveLimit),
        total: result.length,
        offset: effectiveOffset,
        limit: effectiveLimit,
      },
      null,
      2,
    );
  },
});

server.addTool({
  name: "brp_call",
  description: "Call any instantaneous Bevy Remote Protocol method, including ECS reads and writes.",
  parameters: z.object({ method: z.string().min(1), params: z.any().optional() }),
  execute: async ({ method, params }) => {
    if (method.endsWith("+watch")) throw new Error("Use brp_watch for streaming BRP methods");
    const client = viewer?.client ?? clientFor();
    return JSON.stringify(await client.call(method, params), null, 2);
  },
});

server.addTool({
  name: "brp_watch",
  description: "Collect a bounded batch from a Bevy Remote Protocol watch method.",
  parameters: z.object({
    method: z.string().min(1),
    params: z.any().optional(),
    timeoutMs: z.number().int().min(100).max(60_000).optional(),
    maxEvents: z.number().int().min(1).max(100).optional(),
  }),
  execute: async ({ method, params, timeoutMs, maxEvents }) => {
    if (!method.endsWith("+watch")) throw new Error("brp_watch requires a method ending in +watch");
    const client = viewer?.client ?? clientFor();
    return JSON.stringify(
      await client.watch(method, params ?? null, timeoutMs ?? 10_000, maxEvents ?? 20),
      null,
      2,
    );
  },
});

server.addTool({
  name: "viewport_capture",
  description: "Capture the visible primary Bevy window and return it as MCP image content.",
  execute: async () => {
    const client = viewer?.client ?? clientFor();
    const token = crypto.randomUUID();
    const request = (await client.call("bevyout.capture_viewport", { token })) as { path?: string };
    if (!request.path) throw new Error("Bevy did not return a screenshot path");
    const deadline = Date.now() + 10_000;
    while (Date.now() < deadline && !(await Bun.file(request.path).exists())) await wait(100);
    if (!(await Bun.file(request.path).exists())) throw new Error("Timed out waiting for viewport PNG");
    const buffer = Buffer.from(await Bun.file(request.path).arrayBuffer());
    await rm(request.path, { force: true });
    return {
      content: [
        { type: "text", text: "Viewport captured; the game window must remain visible to the renderer." },
        await imageContent({ buffer }),
      ],
    };
  },
});

process.once("SIGINT", () => void stopOwnedViewer());
process.once("SIGTERM", () => void stopOwnedViewer());

await server.start({ transportType: "stdio" });
