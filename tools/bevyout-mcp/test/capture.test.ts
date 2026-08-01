import { expect, test } from "bun:test";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { captureViewport } from "../src/capture";
import type { BrpClient } from "../src/brp/client";

const png = Uint8Array.from([137, 80, 78, 71, 13, 10, 26, 10, 0]);

test("captureViewport validates the expected PNG path and cleans up", async () => {
	let path = "";
	const client = {
		call: async (_method: string, params: { token: string }) => {
			path = join(tmpdir(), "bevyout-agent", `${params.token}.png`);
			await Bun.write(path, png);
			return { path };
		},
	} as unknown as BrpClient;

	const result = await captureViewport(client);
	expect(result.empty).toBe(false);
	expect(result.buffer).toEqual(Buffer.from(png));
	expect(await Bun.file(path).exists()).toBe(false);
});

test("captureViewport rejects a bridge path outside its token directory", async () => {
	const client = {
		call: async () => ({ path: join(tmpdir(), "unexpected.png") }),
	} as unknown as BrpClient;

	await expect(captureViewport(client)).rejects.toThrow(
		"unexpected screenshot path",
	);
});
