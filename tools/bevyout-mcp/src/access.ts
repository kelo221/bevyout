export type McpAccessMode = "read_only" | "runtime_write" | "unrestricted";

const readMethods = new Set([
	"world.query",
	"world.get_components",
	"world.get_resources",
	"world.list_components",
	"world.list_resources",
	"world.registry.schema",
	"bevyout.session",
	"bevyout.capabilities",
	"bevyout.scene_snapshot",
	"bevyout.performance_snapshot",
	"bevyout.schedule_snapshot",
	"bevyout.capture_viewport",
	"bevyout.console.help",
	"bevyout.ragdoll_lab_probe",
	"bevyout.animation_zoo_probe",
]);

const runtimeWriteMethods = new Set([
	"world.insert",
	"world.remove",
	"world.spawn",
	"world.despawn",
	"world.mutate",
	"world.trigger",
	"world.write_message",
	"world.reparent",
	"world.spawn_entity",
	"world.insert_components",
	"world.remove_components",
	"world.despawn_entity",
	"world.reparent_entities",
	"world.mutate_components",
	"world.insert_resources",
	"world.remove_resources",
	"world.mutate_resources",
	"world.trigger_event",
	"bevyout.console.exec",
	"bevyout.animation_zoo_control",
]);

function configuredMode(): McpAccessMode {
	const value = process.env.BEVYOUT_MCP_ACCESS ?? "read_only";
	if (
		value === "read_only" ||
		value === "runtime_write" ||
		value === "unrestricted"
	)
		return value;
	throw new Error(
		`BEVYOUT_MCP_ACCESS must be read_only, runtime_write, or unrestricted (got '${value}')`,
	);
}

export const accessMode = configuredMode();

export function isMutatingMethod(method: string): boolean {
	return (
		runtimeWriteMethods.has(method) ||
		/(?:^|\.)(?:insert|remove|spawn|despawn|mutate|trigger|write|reparent|control)(?:_|$)/.test(
			method,
		)
	);
}

export function assertBrpAccess(method: string): void {
	if (accessMode === "unrestricted") return;
	if (readMethods.has(method)) return;
	if (accessMode === "runtime_write" && runtimeWriteMethods.has(method)) return;
	throw new Error(
		`BRP method '${method}' is not allowed in ${accessMode} mode; set BEVYOUT_MCP_ACCESS=runtime_write or unrestricted`,
	);
}

const readOnlyConsoleCommands = new Set([
	"help",
	"prid",
	"dump",
	"getpos",
	"getrender",
	"renderreport",
	"cam status",
	"tna status",
]);

export function consoleCommandIsReadOnly(line: string): boolean {
	const command = line.trim().toLocaleLowerCase().replace(/\s+/g, " ");
	const commandName = command.split(" ", 1)[0]?.split(".").at(-1) ?? command;
	return (
		readOnlyConsoleCommands.has(command) ||
		readOnlyConsoleCommands.has(commandName)
	);
}

export function assertConsoleAccess(line: string): void {
	if (accessMode !== "read_only" || consoleCommandIsReadOnly(line)) return;
	throw new Error(
		`console command '${line.trim().split(/\s+/, 1)[0] ?? line}' is mutating or not classified as read-only; set BEVYOUT_MCP_ACCESS=runtime_write`,
	);
}
