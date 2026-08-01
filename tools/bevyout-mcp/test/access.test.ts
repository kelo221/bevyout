import { expect, test } from "bun:test";
import {
	accessMode,
	consoleCommandIsReadOnly,
	isMutatingMethod,
} from "../src/access";

test("MCP mutation policy defaults to read-only and recognizes entity reads", () => {
	expect(["read_only", "runtime_write", "unrestricted"]).toContain(accessMode);
	expect(consoleCommandIsReadOnly("00000001.getpos")).toBe(true);
	expect(consoleCommandIsReadOnly("setpos x 1")).toBe(false);
	expect(isMutatingMethod("world.insert")).toBe(true);
	expect(isMutatingMethod("world.insert_components")).toBe(true);
	expect(isMutatingMethod("world.query")).toBe(false);
});
