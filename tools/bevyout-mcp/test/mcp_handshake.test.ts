import { expect, test } from "bun:test";
import { resolve } from "node:path";

type RpcMessage = Record<string, unknown>;

test("FastMCP stdio handshake exposes the bevyout tools", async () => {
  const packageRoot = resolve(import.meta.dir, "..");
  const process = Bun.spawn(["bun", "run", "src/server.ts"], {
    cwd: packageRoot,
    stdin: "pipe",
    stdout: "pipe",
    stderr: "ignore",
  });
  const reader = process.stdout.getReader();
  const decoder = new TextDecoder();
  let pending = "";

  const nextMessage = async (): Promise<RpcMessage> => {
    const deadline = new Promise<never>((_, reject) =>
      setTimeout(() => reject(new Error("timed out waiting for MCP response")), 5_000),
    );
    const read = async (): Promise<RpcMessage> => {
      while (true) {
        const newline = pending.indexOf("\n");
        if (newline >= 0) {
          const line = pending.slice(0, newline);
          pending = pending.slice(newline + 1);
          if (line.trim()) return JSON.parse(line) as RpcMessage;
          continue;
        }
        const chunk = await reader.read();
        if (chunk.done) throw new Error("MCP server closed stdout");
        pending += decoder.decode(chunk.value, { stream: true });
      }
    };
    return Promise.race([read(), deadline]);
  };

  const send = (message: RpcMessage) => {
    process.stdin.write(`${JSON.stringify(message)}\n`);
    process.stdin.flush();
  };

  try {
    send({
      jsonrpc: "2.0",
      id: 1,
      method: "initialize",
      params: {
        protocolVersion: "2025-06-18",
        capabilities: {},
        clientInfo: { name: "bevyout-test", version: "0.1.0" },
      },
    });
    const initialized = await nextMessage();
    expect(initialized.id).toBe(1);

    send({ jsonrpc: "2.0", method: "notifications/initialized", params: {} });
    send({ jsonrpc: "2.0", id: 2, method: "tools/list", params: {} });
    const listing = await nextMessage();
    const tools = ((listing.result as { tools: Array<{ name: string }> }).tools ?? []).map(
      (tool) => tool.name,
    );
    expect(tools).toContain("viewer_status");
    expect(tools).toContain("world_query");
    expect(tools).toContain("performance_snapshot");
    expect(tools).toContain("performance_probe");
    expect(tools).toContain("schedule_snapshot");
    expect(tools).toContain("console_exec");
    expect(tools).toContain("console_help");
    expect(tools).toContain("brp_call");
    expect(tools).toContain("viewport_capture");
  } finally {
    process.kill("SIGKILL");
    await process.exited;
  }
});
