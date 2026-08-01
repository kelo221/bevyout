export type Json =
	| null
	| boolean
	| number
	| string
	| Json[]
	| { [key: string]: Json };
export type BrpError = { code?: number; message?: string; data?: Json };
export type BrpResponse = { result?: Json; error?: BrpError };

export type CallOptions = {
	signal?: AbortSignal;
	timeoutMs?: number;
};

export type BrpWatchStatus = "completed" | "timed_out" | "cancelled";
export type BrpWatchResult = {
	status: BrpWatchStatus;
	events: Json[];
	elapsed_ms: number;
};

export class BrpRequestError extends Error {
	constructor(
		message: string,
		readonly kind: "timeout" | "cancelled" | "http" | "rpc" | "protocol",
	) {
		super(message);
		this.name = "BrpRequestError";
	}
}

type RequestSignal = {
	signal: AbortSignal;
	timedOut: () => boolean;
	dispose: () => void;
};

function requestSignal(options: CallOptions): RequestSignal {
	const timeoutController = new AbortController();
	let timedOut = false;
	let timeout: ReturnType<typeof setTimeout> | undefined;
	if (options.timeoutMs !== undefined) {
		if (!Number.isFinite(options.timeoutMs) || options.timeoutMs <= 0) {
			throw new BrpRequestError(
				"BRP timeout must be greater than zero",
				"protocol",
			);
		}
		timeout = setTimeout(() => {
			timedOut = true;
			timeoutController.abort();
		}, options.timeoutMs);
	}

	let signal: AbortSignal;
	let removeForwarder: (() => void) | undefined;
	if (options.signal && typeof AbortSignal.any === "function") {
		signal = AbortSignal.any([options.signal, timeoutController.signal]);
	} else if (options.signal) {
		const combined = new AbortController();
		const forward = () => combined.abort(options.signal?.reason);
		if (options.signal.aborted) forward();
		else options.signal.addEventListener("abort", forward, { once: true });
		timeoutController.signal.addEventListener("abort", () => combined.abort(), {
			once: true,
		});
		removeForwarder = () =>
			options.signal?.removeEventListener("abort", forward);
		signal = combined.signal;
	} else {
		signal = timeoutController.signal;
	}

	return {
		signal,
		timedOut: () => timedOut,
		dispose: () => {
			if (timeout) clearTimeout(timeout);
			removeForwarder?.();
		},
	};
}

function abortError(signal: RequestSignal): BrpRequestError {
	return signal.timedOut()
		? new BrpRequestError("BRP request timed out", "timeout")
		: new BrpRequestError("BRP request cancelled", "cancelled");
}

function rpcError(error: BrpError): BrpRequestError {
	const suffix =
		error.data === undefined ? "" : ` (${JSON.stringify(error.data)})`;
	return new BrpRequestError(
		`BRP ${error.code ?? "error"}: ${error.message ?? "request failed"}${suffix}`,
		"rpc",
	);
}

function parseSseEvent(lines: string[]): BrpResponse | undefined {
	const data = lines
		.flatMap((line) =>
			line.startsWith("data:") ? [line.slice(5).replace(/^ /, "")] : [],
		)
		.join("\n");
	if (!data) return undefined;
	try {
		return JSON.parse(data) as BrpResponse;
	} catch (error) {
		throw new BrpRequestError(
			`BRP watch returned invalid SSE JSON: ${error instanceof Error ? error.message : String(error)}`,
			"protocol",
		);
	}
}

export class BrpClient {
	private nextId = 1;

	constructor(readonly baseUrl: string) {}

	async call(
		method: string,
		params?: unknown,
		options: CallOptions = {},
	): Promise<Json> {
		const request = requestSignal(options);
		try {
			let response: Response;
			try {
				response = await fetch(`${this.baseUrl.replace(/\/$/, "")}/`, {
					method: "POST",
					headers: { "content-type": "application/json" },
					body: JSON.stringify({
						jsonrpc: "2.0",
						id: this.nextId++,
						method,
						...(params === undefined ? {} : { params }),
					}),
					signal: request.signal,
				});
			} catch (error) {
				if (request.signal.aborted) throw abortError(request);
				throw new BrpRequestError(
					`BRP connection failed: ${error instanceof Error ? error.message : String(error)}`,
					"http",
				);
			}
			if (!response.ok) {
				throw new BrpRequestError(
					`BRP HTTP ${response.status}: ${await response.text()}`,
					"http",
				);
			}
			let payload: BrpResponse;
			try {
				payload = (await response.json()) as BrpResponse;
			} catch (error) {
				throw new BrpRequestError(
					`BRP returned invalid JSON: ${error instanceof Error ? error.message : String(error)}`,
					"protocol",
				);
			}
			if (payload.error) throw rpcError(payload.error);
			return payload.result ?? null;
		} finally {
			request.dispose();
		}
	}

	async watch(
		method: string,
		params: unknown,
		options: CallOptions & { maxEvents: number },
	): Promise<BrpWatchResult> {
		const started = performance.now();
		const request = requestSignal(options);
		const events: Json[] = [];
		let reader: ReadableStreamDefaultReader<Uint8Array> | undefined;
		let buffer = "";
		let eventLines: string[] = [];
		try {
			let response: Response;
			try {
				response = await fetch(`${this.baseUrl.replace(/\/$/, "")}/`, {
					method: "POST",
					headers: {
						"content-type": "application/json",
						accept: "text/event-stream",
					},
					body: JSON.stringify({
						jsonrpc: "2.0",
						id: this.nextId++,
						method,
						params,
					}),
					signal: request.signal,
				});
			} catch (error) {
				if (request.signal.aborted) {
					return {
						status: request.timedOut() ? "timed_out" : "cancelled",
						events,
						elapsed_ms: Math.round(performance.now() - started),
					};
				}
				throw new BrpRequestError(
					`BRP watch connection failed: ${error instanceof Error ? error.message : String(error)}`,
					"http",
				);
			}
			if (!response.ok || !response.body) {
				throw new BrpRequestError(
					`BRP watch HTTP ${response.status}: ${response.body ? await response.text() : "empty response body"}`,
					"http",
				);
			}

			reader = response.body.getReader();
			const decoder = new TextDecoder();
			const dispatch = () => {
				const payload = parseSseEvent(eventLines);
				eventLines = [];
				if (!payload) return;
				if (payload.error) throw rpcError(payload.error);
				const result = payload.result;
				if (Array.isArray(result)) events.push(...result);
				else if (result !== undefined && result !== null) events.push(result);
			};

			while (events.length < options.maxEvents) {
				const chunk = await reader.read();
				if (chunk.done) {
					buffer += decoder.decode();
					if (buffer) {
						for (const line of buffer.split(/\r?\n/)) {
							if (line === "") {
								dispatch();
							} else if (!line.startsWith(":") && line.startsWith("data:")) {
								eventLines.push(line);
							}
						}
						buffer = "";
					}
					break;
				}
				buffer += decoder.decode(chunk.value, { stream: true });
				const lines = buffer.split(/\r?\n/);
				buffer = lines.pop() ?? "";
				for (const line of lines) {
					if (line === "") {
						dispatch();
						if (events.length >= options.maxEvents) break;
					} else if (
						!line.startsWith(":") &&
						(line.startsWith("data:") || line.startsWith("event:"))
					) {
						if (line.startsWith("data:")) eventLines.push(line);
					}
				}
			}
			if (eventLines.length > 0 && events.length < options.maxEvents)
				dispatch();
			return {
				status: "completed",
				events: events.slice(0, options.maxEvents),
				elapsed_ms: Math.round(performance.now() - started),
			};
		} catch (error) {
			if (request.signal.aborted) {
				return {
					status: request.timedOut() ? "timed_out" : "cancelled",
					events: events.slice(0, options.maxEvents),
					elapsed_ms: Math.round(performance.now() - started),
				};
			}
			throw error;
		} finally {
			await reader?.cancel().catch(() => undefined);
			request.dispose();
		}
	}
}
