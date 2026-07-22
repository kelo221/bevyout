import { expect, test } from "bun:test";
import { resolve } from "node:path";

type RpcMessage = Record<string, unknown>;
type JsonSchema = {
  type?: string;
  required?: string[];
  properties?: Record<string, { type?: string; enum?: string[]; maximum?: number; minimum?: number }>;
};

test("cinema_record is registered with the filmstrip parameter contract", async () => {
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
    await nextMessage();

    send({ jsonrpc: "2.0", method: "notifications/initialized", params: {} });
    send({ jsonrpc: "2.0", id: 2, method: "tools/list", params: {} });
    const listing = await nextMessage();
    const tools = (listing.result as { tools: Array<{ name: string; inputSchema: JsonSchema }> }).tools;
    const tool = tools.find((candidate) => candidate.name === "cinema_record");
    expect(tool).toBeDefined();

    const schema = tool!.inputSchema;
    expect(schema.required).toContain("subject");
    expect(schema.properties?.subject?.type).toBe("string");
    expect(schema.properties?.frames?.maximum).toBe(8);
    expect(schema.properties?.intervalMs).toBeDefined();
    expect(schema.properties?.mode?.enum).toEqual(["follow", "orbit"]);
    expect(schema.properties?.dist).toBeDefined();
    expect(schema.properties?.height).toBeDefined();
    expect(schema.properties?.radius).toBeDefined();
  } finally {
    process.kill("SIGKILL");
    await process.exited;
  }
});
