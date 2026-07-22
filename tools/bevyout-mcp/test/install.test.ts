import { mkdir, mkdtemp, readdir, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { afterEach, describe, expect, test } from "bun:test";

import {
  installTargets,
  type InstallOptions,
  parseInstallArgs,
  SERVER_NAME,
  SERVER_RELATIVE_PATH,
} from "../src/install";

const temporaryRoots: string[] = [];

async function makePaths() {
  const root = await mkdtemp(join(tmpdir(), "bevyout-install-"));
  temporaryRoots.push(root);
  return {
    root,
    codex: join(root, "codex", "config.toml"),
    desktop: join(root, "claude", "claude_desktop_config.json"),
    code: join(root, ".mcp.json"),
    opencode: join(root, "opencode", "opencode.jsonc"),
  };
}

async function writeFixture(path: string, content: string): Promise<void> {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, content);
}

afterEach(async () => {
  await Promise.all(temporaryRoots.splice(0).map((root) => rm(root, { recursive: true, force: true })));
});

describe("bevyout agent installer", () => {
  test("creates all target configurations with the expected server shapes", async () => {
    const paths = await makePaths();
    const results = await installTargets({
      targets: ["codex", "claude-desktop", "claude-code", "opencode"],
      repoRoot: paths.root,
      codexPath: paths.codex,
      claudeDesktopPath: paths.desktop,
      claudeCodePath: paths.code,
      opencodePath: paths.opencode,
      now: new Date("2026-07-13T19:00:00.000Z"),
    });

    expect(results.every((result) => result.changed)).toBe(true);

    const codex = Bun.TOML.parse(await readFile(paths.codex, "utf8")) as {
      mcp_servers: Record<string, Record<string, unknown>>;
    };
    expect(codex.mcp_servers[SERVER_NAME].command).toBe("bun run");
    expect(codex.mcp_servers[SERVER_NAME].startup_timeout_sec).toBe(120);

    const desktop = JSON.parse(await readFile(paths.desktop, "utf8")) as {
      mcpServers: Record<string, Record<string, unknown>>;
    };
    expect(desktop.mcpServers[SERVER_NAME]).toEqual({
      command: "bun",
      args: ["run", join(paths.root, SERVER_RELATIVE_PATH)],
    });

    const code = JSON.parse(await readFile(paths.code, "utf8")) as {
      mcpServers: Record<string, Record<string, unknown>>;
    };
    expect(code.mcpServers[SERVER_NAME]).toEqual({
      command: "bun",
      args: ["run", SERVER_RELATIVE_PATH],
    });

    const opencode = JSON.parse(await readFile(paths.opencode, "utf8")) as {
      mcp: Record<string, Record<string, unknown>>;
    };
    expect(opencode.mcp[SERVER_NAME]).toEqual({
      type: "local",
      command: ["bun", "run", join(paths.root, SERVER_RELATIVE_PATH)],
      enabled: true,
    });
  });

  test("preserves unrelated entries and managed server environment fields", async () => {
    const paths = await makePaths();
    await writeFixture(
      paths.codex,
      [
        "[mcp_servers.other]",
        'command = "other"',
        "",
        "[mcp_servers.bevyout]",
        'command = "old"',
        "args = []",
        "startup_timeout_sec = 1",
        "",
      ].join("\n"),
    );
    await writeFixture(
      paths.desktop,
      JSON.stringify({
        preferences: { keep: true },
        mcpServers: {
          other: { command: "other" },
          [SERVER_NAME]: { env: { KEEP: "yes" }, command: "old", args: [] },
        },
      }),
    );
    await writeFixture(
      paths.opencode,
      JSON.stringify({
        $schema: "https://opencode.ai/config.json",
        mcp: {
          other: { type: "local", command: ["other"] },
        },
      }),
    );

    const results = await installTargets({
      targets: ["codex", "claude-desktop", "opencode"],
      repoRoot: paths.root,
      codexPath: paths.codex,
      claudeDesktopPath: paths.desktop,
      opencodePath: paths.opencode,
      now: new Date("2026-07-13T19:01:00.000Z"),
    });

    expect(results.every((result) => result.backupPath !== undefined)).toBe(true);
    for (const result of results) {
      expect(await readFile(result.backupPath!, "utf8")).toContain("other");
    }

    const codex = Bun.TOML.parse(await readFile(paths.codex, "utf8")) as {
      mcp_servers: Record<string, Record<string, unknown>>;
    };
    expect(codex.mcp_servers.other.command).toBe("other");
    expect(codex.mcp_servers[SERVER_NAME].command).toBe("bun run");

    const desktop = JSON.parse(await readFile(paths.desktop, "utf8")) as {
      preferences: { keep: boolean };
      mcpServers: Record<string, Record<string, unknown>>;
    };
    expect(desktop.preferences.keep).toBe(true);
    expect(desktop.mcpServers.other.command).toBe("other");
    expect(desktop.mcpServers[SERVER_NAME].env).toEqual({ KEEP: "yes" });

    const opencode = JSON.parse(await readFile(paths.opencode, "utf8")) as {
      $schema: string;
      mcp: Record<string, Record<string, unknown>>;
    };
    expect(opencode.$schema).toBe("https://opencode.ai/config.json");
    expect(opencode.mcp.other).toEqual({ type: "local", command: ["other"] });
    expect(opencode.mcp[SERVER_NAME]).toEqual({
      type: "local",
      command: ["bun", "run", join(paths.root, SERVER_RELATIVE_PATH)],
      enabled: true,
    });
  });

  test("is idempotent and does not create a second backup", async () => {
    const paths = await makePaths();
    await writeFixture(paths.code, JSON.stringify({ mcpServers: { existing: { command: "keep" } } }));
    const options: InstallOptions = {
      targets: ["claude-code"],
      repoRoot: paths.root,
      claudeCodePath: paths.code,
      now: new Date("2026-07-13T19:02:00.000Z"),
    };

    const first = await installTargets(options);
    const firstBackups = (await readdir(dirname(paths.code))).filter((name) => name.includes(".bevyout-"));
    const second = await installTargets(options);
    const secondBackups = (await readdir(dirname(paths.code))).filter((name) => name.includes(".bevyout-"));

    expect(first[0].changed).toBe(true);
    expect(second[0].changed).toBe(false);
    expect(secondBackups).toEqual(firstBackups);
  });

  test("rejects malformed configuration without changing it", async () => {
    const paths = await makePaths();
    const malformedJson = "{\"mcpServers\":";
    const malformedToml = "[mcp_servers.bevyout\ncommand = \"broken\"";
    await writeFixture(paths.desktop, malformedJson);
    await writeFixture(paths.codex, malformedToml);

    await expect(
      installTargets({
        targets: ["claude-desktop"],
        repoRoot: paths.root,
        claudeDesktopPath: paths.desktop,
      }),
    ).rejects.toThrow("Invalid JSON");
    await expect(
      installTargets({
        targets: ["codex"],
        repoRoot: paths.root,
        codexPath: paths.codex,
      }),
    ).rejects.toThrow("Invalid TOML");
    expect(await readFile(paths.desktop, "utf8")).toBe(malformedJson);
    expect(await readFile(paths.codex, "utf8")).toBe(malformedToml);
  });

  test("dry-run reports changes without creating files", async () => {
    const paths = await makePaths();
    const results = await installTargets({
      targets: ["codex", "claude-desktop", "claude-code", "opencode"],
      repoRoot: paths.root,
      codexPath: paths.codex,
      claudeDesktopPath: paths.desktop,
      claudeCodePath: paths.code,
      opencodePath: paths.opencode,
      dryRun: true,
    });

    expect(results.every((result) => result.changed && result.backupPath === undefined)).toBe(true);
    await expect(readFile(paths.codex)).rejects.toThrow();
    await expect(readFile(paths.desktop)).rejects.toThrow();
    await expect(readFile(paths.code)).rejects.toThrow();
    await expect(readFile(paths.opencode)).rejects.toThrow();
  });

  test("parses target flags and defaults to all targets", () => {
    expect(parseInstallArgs([])).toEqual({
      targets: ["codex", "claude-desktop", "claude-code", "opencode"],
      dryRun: false,
      help: false,
    });
    expect(parseInstallArgs(["--claude-code", "--dry-run"])).toEqual({
      targets: ["claude-code"],
      dryRun: true,
      help: false,
    });
    expect(parseInstallArgs(["--opencode"])).toEqual({
      targets: ["opencode"],
      dryRun: false,
      help: false,
    });
  });
});
