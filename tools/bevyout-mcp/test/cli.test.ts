import { describe, expect, test } from "bun:test";
import { boundedJson, parseArgs } from "../src/cli";

describe("bounded MCP CLI", () => {
	test("parses bounded command options", () => {
		const parsed = parseArgs(["scene", "--limit", "25", "--no-total"]);
		expect(parsed.command).toBe("scene");
		expect(parsed.options.get("limit")).toBe("25");
		expect(parsed.options.get("no-total")).toBe(true);
	});

	test("keeps oversized output under the requested byte bound", () => {
		const output = boundedJson({ rows: ["x".repeat(10_000)] }, 1_024);
		expect(Buffer.byteLength(output)).toBeLessThanOrEqual(1_024);
		expect(JSON.parse(output).truncated).toBe(true);
	});

	test("rejects positional spillover", () => {
		expect(() => parseArgs(["status", "surprise"])).toThrow("unexpected argument");
	});
});
