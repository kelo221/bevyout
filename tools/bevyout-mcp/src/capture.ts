import { mkdir, readFile, realpath, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { type BrpClient, BrpRequestError } from "./brp/client";

const pngSignature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);

export async function captureViewport(
	client: BrpClient,
	signal?: AbortSignal,
	timeoutMs = 10_000,
): Promise<{ buffer: Buffer; empty: boolean }> {
	const token = crypto.randomUUID();
	const directory = join(tmpdir(), "bevyout-agent");
	await mkdir(directory, { recursive: true });
	const expectedPath = join(directory, `${token}.png`);
	try {
		const request = (await client.call(
			"bevyout.capture_viewport",
			{ token },
			{ timeoutMs, signal },
		)) as {
			path?: string;
		};
		if (!request.path) throw new Error("Bevy did not return a screenshot path");
		if (resolve(request.path) !== resolve(expectedPath)) {
			throw new Error("Bevy returned an unexpected screenshot path");
		}
		const deadline = Date.now() + timeoutMs;
		while (Date.now() < deadline && !(await Bun.file(expectedPath).exists())) {
			if (signal?.aborted)
				throw new BrpRequestError("operation cancelled", "cancelled");
			await new Promise((resolvePromise) => setTimeout(resolvePromise, 100));
		}
		if (!(await Bun.file(expectedPath).exists())) {
			throw new BrpRequestError(
				"Timed out waiting for viewport PNG",
				"timeout",
			);
		}
		const canonicalExpected = join(await realpath(directory), `${token}.png`);
		const canonicalActual = await realpath(expectedPath);
		if (resolve(canonicalActual) !== resolve(canonicalExpected)) {
			throw new Error("Screenshot path failed canonical path validation");
		}
		const buffer = Buffer.from(await readFile(canonicalActual));
		if (
			buffer.length > 0 &&
			!buffer.subarray(0, pngSignature.length).equals(pngSignature)
		) {
			throw new Error("Viewport capture did not produce a PNG");
		}
		return { buffer, empty: buffer.length === 0 };
	} finally {
		await rm(expectedPath, { force: true });
	}
}
