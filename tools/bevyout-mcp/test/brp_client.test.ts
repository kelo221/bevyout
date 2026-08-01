import { expect, test } from "bun:test";
import { BrpClient, BrpRequestError } from "../src/brp/client";

const originalFetch = globalThis.fetch;

test("BrpClient parses fragmented multiline SSE events", async () => {
	globalThis.fetch = (async () => {
		const chunks = ["da", 'ta: {"result": {\n', 'data: "answer": 42}}\n', "\n"];
		const stream = new ReadableStream<Uint8Array>({
			start(controller) {
				for (const chunk of chunks)
					controller.enqueue(new TextEncoder().encode(chunk));
				controller.close();
			},
		});
		return new Response(stream, { status: 200 });
	}) as unknown as typeof fetch;

	try {
		const result = await new BrpClient("http://127.0.0.1:15702").watch(
			"world.get_components+watch",
			null,
			{
				timeoutMs: 1_000,
				maxEvents: 1,
			},
		);
		expect(result.status).toBe("completed");
		expect(result.events).toEqual([{ answer: 42 }]);
	} finally {
		globalThis.fetch = originalFetch;
	}
});

test("BrpClient reports call deadlines as typed timeout errors", async () => {
	globalThis.fetch = (async (
		_input: RequestInfo,
		init: RequestInit | undefined,
	) =>
		await new Promise<Response>((_resolve, reject) => {
			init?.signal?.addEventListener(
				"abort",
				() => reject(new DOMException("aborted", "AbortError")),
				{ once: true },
			);
		})) as typeof fetch;

	try {
		const promise = new BrpClient("http://127.0.0.1:15702").call(
			"bevyout.session",
			undefined,
			{ timeoutMs: 10 },
		);
		await expect(promise).rejects.toBeInstanceOf(BrpRequestError);
		await expect(promise).rejects.toMatchObject({ kind: "timeout" });
	} finally {
		globalThis.fetch = originalFetch;
	}
});

test("BrpClient distinguishes a watch timeout from an empty completed stream", async () => {
	globalThis.fetch = (async (
		_input: RequestInfo,
		init: RequestInit | undefined,
	) => {
		const stream = new ReadableStream<Uint8Array>({
			start(controller) {
				init?.signal?.addEventListener(
					"abort",
					() => controller.error(new DOMException("aborted", "AbortError")),
					{
						once: true,
					},
				);
			},
		});
		return new Response(stream, { status: 200 });
	}) as unknown as typeof fetch;

	try {
		const result = await new BrpClient("http://127.0.0.1:15702").watch(
			"world.get_components+watch",
			null,
			{
				timeoutMs: 10,
				maxEvents: 1,
			},
		);
		expect(result).toMatchObject({ status: "timed_out", events: [] });
		expect(result.elapsed_ms).toBeGreaterThanOrEqual(0);
	} finally {
		globalThis.fetch = originalFetch;
	}
});
