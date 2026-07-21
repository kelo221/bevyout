import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";
import { homedir } from "node:os";
import { dirname, join, resolve } from "node:path";

export const SERVER_NAME = "bevyout";
export const SERVER_RELATIVE_PATH = "tools/bevyout-mcp/src/server.ts";

export type InstallTarget = "codex" | "claude-desktop" | "claude-code" | "opencode";

export type InstallOptions = {
  targets: InstallTarget[];
  repoRoot?: string;
  codexPath?: string;
  claudeDesktopPath?: string;
  claudeCodePath?: string;
  opencodePath?: string;
  dryRun?: boolean;
  now?: Date;
};

export type InstallResult = {
  target: InstallTarget;
  path: string;
  changed: boolean;
  backupPath?: string;
};

type JsonRecord = Record<string, unknown>;

const ALL_TARGETS: InstallTarget[] = ["codex", "claude-desktop", "claude-code", "opencode"];

export function codexConfigPath(): string {
  const root = process.env.CODEX_HOME?.trim() || join(homedir(), ".codex");
  return join(root, "config.toml");
}

export function claudeDesktopConfigPath(): string {
  if (process.platform === "win32") {
    const root = process.env.APPDATA?.trim() || join(homedir(), "AppData", "Roaming");
    return join(root, "Claude", "claude_desktop_config.json");
  }
  if (process.platform === "darwin") {
    return join(homedir(), "Library", "Application Support", "Claude", "claude_desktop_config.json");
  }
  const root = process.env.XDG_CONFIG_HOME?.trim() || join(homedir(), ".config");
  return join(root, "Claude", "claude_desktop_config.json");
}

export function claudeCodeConfigPath(repoRoot: string): string {
  return join(repoRoot, ".mcp.json");
}

export function opencodeConfigPath(): string {
  const root = process.env.XDG_CONFIG_HOME?.trim() || join(homedir(), ".config");
  return join(root, "opencode", "opencode.jsonc");
}

function isObject(value: unknown): value is JsonRecord {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function asObject(value: unknown, description: string): JsonRecord {
  if (!isObject(value)) {
    throw new Error(`${description} must be a JSON object`);
  }
  return value;
}

function parseJsonConfig(text: string, path: string): JsonRecord {
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch (error) {
    throw new Error(`Invalid JSON in ${path}: ${error instanceof Error ? error.message : String(error)}`);
  }
  return asObject(parsed, path);
}

function mergeJsonConfig(
  existing: string | undefined,
  path: string,
  server: JsonRecord,
): string {
  const root = existing === undefined ? {} : parseJsonConfig(existing, path);
  const servers = root.mcpServers === undefined
    ? {}
    : asObject(root.mcpServers, `${path}.mcpServers`);
  const previous = servers[SERVER_NAME] === undefined
    ? {}
    : asObject(servers[SERVER_NAME], `${path}.mcpServers.${SERVER_NAME}`);

  root.mcpServers = {
    ...servers,
    [SERVER_NAME]: { ...previous, ...server },
  };
  return `${JSON.stringify(root, null, 2)}\n`;
}

function mergeOpencodeConfig(
  existing: string | undefined,
  path: string,
  server: JsonRecord,
): string {
  const root = existing === undefined ? {} : parseJsonConfig(existing, path);
  const mcp = root.mcp === undefined
    ? {}
    : asObject(root.mcp, `${path}.mcp`);
  const previous = mcp[SERVER_NAME] === undefined
    ? {}
    : asObject(mcp[SERVER_NAME], `${path}.mcp.${SERVER_NAME}`);

  root.mcp = {
    ...mcp,
    [SERVER_NAME]: { ...previous, ...server },
  };
  return `${JSON.stringify(root, null, 2)}\n`;
}

function validateTomlConfig(text: string, path: string): void {
  try {
    Bun.TOML.parse(text);
  } catch (error) {
    throw new Error(`Invalid TOML in ${path}: ${error instanceof Error ? error.message : String(error)}`);
  }
}

function managedCodexLines(serverScript: string): string[] {
  return [
    `[mcp_servers.${SERVER_NAME}]`,
    `command = "bun run"`,
    `args = [${JSON.stringify(serverScript)}]`,
    "startup_timeout_sec = 120",
  ];
}

function mergeCodexConfig(existing: string | undefined, path: string, serverScript: string): string {
  if (existing !== undefined) {
    validateTomlConfig(existing, path);
  }
  const newline = existing?.includes("\r\n") ? "\r\n" : "\n";
  const managed = managedCodexLines(serverScript);
  if (existing === undefined || existing.length === 0) {
    return `${managed.join(newline)}${newline}`;
  }

  const lines = existing.split(/\r?\n/);
  const header = `[mcp_servers.${SERVER_NAME}]`;
  const headerIndex = lines.findIndex((line) => line.trim() === header);
  if (headerIndex < 0) {
    const withoutTrailingNewlines = existing.replace(/(?:\r?\n)+$/u, "");
    return `${withoutTrailingNewlines}${newline}${newline}${managed.join(newline)}${newline}`;
  }

  let endIndex = lines.length;
  for (let index = headerIndex + 1; index < lines.length; index += 1) {
    if (/^\s*\[[^\]]+\]\s*$/u.test(lines[index])) {
      endIndex = index;
      break;
    }
  }

  const preserved = lines
    .slice(headerIndex + 1, endIndex)
    .filter((line) => !/^\s*(?:command|args|startup_timeout_sec)\s*=/u.test(line));
  const updated = [
    ...lines.slice(0, headerIndex),
    ...managed,
    ...preserved,
    ...lines.slice(endIndex),
  ];
  return updated.join(newline);
}

async function optionalRead(path: string): Promise<string | undefined> {
  try {
    return await readFile(path, "utf8");
  } catch (error) {
    if (isNodeError(error) && error.code === "ENOENT") {
      return undefined;
    }
    throw error;
  }
}

function isNodeError(error: unknown): error is NodeJS.ErrnoException {
  return error instanceof Error && "code" in error;
}

function timestamp(now: Date): string {
  return now.toISOString().replace(/[-:]/gu, "").replace(/\.\d{3}/u, "");
}

async function pathExists(path: string): Promise<boolean> {
  try {
    await readFile(path);
    return true;
  } catch (error) {
    if (isNodeError(error) && error.code === "ENOENT") {
      return false;
    }
    throw error;
  }
}

async function uniqueBackupPath(path: string, now: Date): Promise<string> {
  const base = `${path}.bevyout-${timestamp(now)}.bak`;
  if (!(await pathExists(base))) {
    return base;
  }
  for (let index = 1; ; index += 1) {
    const candidate = `${base}.${index}`;
    if (!(await pathExists(candidate))) {
      return candidate;
    }
  }
}

async function applyConfig(
  target: InstallTarget,
  path: string,
  desired: string,
  options: InstallOptions,
): Promise<InstallResult> {
  const existing = await optionalRead(path);
  if (existing === desired) {
    return { target, path, changed: false };
  }
  if (options.dryRun) {
    return { target, path, changed: true };
  }

  await mkdir(dirname(path), { recursive: true });
  let backupPath: string | undefined;
  if (existing !== undefined) {
    backupPath = await uniqueBackupPath(path, options.now ?? new Date());
    await copyFile(path, backupPath);
  }
  await writeFile(path, desired, "utf8");
  return { target, path, changed: true, backupPath };
}

export async function installTargets(options: InstallOptions): Promise<InstallResult[]> {
  const repoRoot = resolve(options.repoRoot ?? resolve(import.meta.dir, "..", "..", ".."));
  const serverScript = resolve(repoRoot, SERVER_RELATIVE_PATH);
  const uniqueTargets = [...new Set(options.targets)];
  const results: InstallResult[] = [];

  for (const target of uniqueTargets) {
    if (target === "codex") {
      const path = options.codexPath ?? codexConfigPath();
      const existing = await optionalRead(path);
      const desired = mergeCodexConfig(existing, path, serverScript);
      results.push(await applyConfig(target, path, desired, options));
      continue;
    }

    if (target === "claude-desktop") {
      const path = options.claudeDesktopPath ?? claudeDesktopConfigPath();
      const existing = await optionalRead(path);
      const desired = mergeJsonConfig(existing, path, {
        command: "bun",
        args: ["run", serverScript],
      });
      results.push(await applyConfig(target, path, desired, options));
      continue;
    }

    if (target === "claude-code") {
      const path = options.claudeCodePath ?? claudeCodeConfigPath(repoRoot);
      const existing = await optionalRead(path);
      const desired = mergeJsonConfig(existing, path, {
        command: "bun",
        args: ["run", SERVER_RELATIVE_PATH],
      });
      results.push(await applyConfig(target, path, desired, options));
      continue;
    }

    if (target === "opencode") {
      const path = options.opencodePath ?? opencodeConfigPath();
      const existing = await optionalRead(path);
      const desired = mergeOpencodeConfig(existing, path, {
        type: "local",
        command: ["bun", "run", serverScript],
        enabled: true,
      });
      results.push(await applyConfig(target, path, desired, options));
      continue;
    }

    throw new Error(`Unsupported install target: ${target}`);
  }

  return results;
}

type CliOptions = {
  targets: InstallTarget[];
  dryRun: boolean;
  help: boolean;
};

export function parseInstallArgs(argv: string[]): CliOptions {
  const targets: InstallTarget[] = [];
  let dryRun = false;
  let help = false;
  for (const argument of argv) {
    if (argument === "--all") {
      targets.push(...ALL_TARGETS);
    } else if (argument === "--codex") {
      targets.push("codex");
    } else if (argument === "--claude-desktop") {
      targets.push("claude-desktop");
    } else if (argument === "--claude-code") {
      targets.push("claude-code");
    } else if (argument === "--opencode") {
      targets.push("opencode");
    } else if (argument === "--dry-run") {
      dryRun = true;
    } else if (argument === "--help" || argument === "-h") {
      help = true;
    } else {
      throw new Error(`Unknown option: ${argument}`);
    }
  }

  return {
    targets: targets.length > 0 ? [...new Set(targets)] : ALL_TARGETS,
    dryRun,
    help,
  };
}

function printHelp(): void {
  console.log(`bevyout MCP installer\n\nUsage:\n  bun run tools/bevyout-mcp/src/install.ts --all\n\nTargets:\n  --codex            Update CODEX_HOME/config.toml\n  --claude-desktop   Update Claude Desktop's global config\n  --claude-code      Update this repository's .mcp.json\n  --opencode         Update OpenCode's global config\n  --all              Update all four targets\n  --dry-run          Show changes without writing files`);
}

function printResults(results: InstallResult[], dryRun: boolean): void {
  for (const result of results) {
    const status = result.changed ? (dryRun ? "would update" : "updated") : "already configured";
    const backup = result.backupPath === undefined ? "" : `; backup=${result.backupPath}`;
    console.log(`${result.target}: ${status} ${result.path}${backup}`);
  }
}

export async function main(argv = process.argv.slice(2)): Promise<void> {
  const options = parseInstallArgs(argv);
  if (options.help) {
    printHelp();
    return;
  }
  const results = await installTargets({ targets: options.targets, dryRun: options.dryRun });
  printResults(results, options.dryRun);
}

if (import.meta.main) {
  await main().catch((error: unknown) => {
    console.error(`bevyout installer: ${error instanceof Error ? error.message : String(error)}`);
    process.exitCode = 1;
  });
}
