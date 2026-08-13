export const meta = {
  name: "feature_slice",
  description: "Scout, approve, implement, review, and accept one Bevyout feature slice",
  phases: [
    { title: "Architecture" },
    { title: "Approval" },
    { title: "Contract" },
    { title: "Implementation" },
    { title: "Focused gates" },
    { title: "Independent review" },
    { title: "Runtime acceptance" },
  ],
};

const request = String(args?.request ?? args?.feature ?? "the requested feature");

phase("Architecture");
const architecture = await agent(
  `Read-only architecture scout for this Bevy 0.19 feature: ${request}. Start with CodeGraph. Trace schedule/resource/data boundaries, affected tests, and the smallest vertical slice. Return exact files and risks.`,
  { label: "feature architecture", tier: "scout", tools: ["read", "bash"] },
);

phase("Approval");
await checkpoint(`Approve this architecture before implementation?\n\n${architecture}`);

phase("Contract");
const contract = await agent(
  `Define executable behavior and regression tests for this feature. Preserve Bevyout architecture and synthetic-data rules.\n\nREQUEST:\n${request}\n\nARCHITECTURE:\n${architecture}`,
  { label: "feature contract", tier: "worker", tools: ["read", "bash"] },
);

phase("Implementation");
const implementation = await agent(
  `Implement the approved feature and tests in a dedicated local git worktree and pi-workflow/* branch. Never modify, merge into, or push the caller branch. Run focused checks and return branch/commit/files.\n\nREQUEST:\n${request}\n\nARCHITECTURE:\n${architecture}\n\nCONTRACT:\n${contract}`,
  { label: "worktree implementation", tier: "worker", tools: ["read", "bash", "edit", "write"] },
);

phase("Focused gates");
const gates = await agent(
  `Read-only. Locate the implementation branch/worktree from this handoff and run the narrow Fast check plus affected tests. Do not edit.\n\n${implementation}`,
  { label: "focused gates", tier: "worker", tools: ["read", "bash"] },
);

phase("Independent review");
const review = await agent(
  `Review the implementation branch independently for Bevy ECS soundness, architecture, performance, prepared revisions, data safety, and missing tests. Do not edit.\n\n${implementation}\n\nGATES:\n${gates}`,
  { label: "feature review", tier: "reviewer", tools: ["read", "bash"] },
);

phase("Runtime acceptance");
const acceptance = await agent(
  `Read-only runtime acceptance. Use tools/bevyout-mcp/src/cli.ts with bounded output when a viewer is available. Otherwise identify the exact unavailable acceptance without generating data.\n\nREQUEST:\n${request}\n\nIMPLEMENTATION:\n${implementation}\n\nREVIEW:\n${review}`,
  { label: "runtime acceptance", tier: "reviewer", tools: ["read", "bash"] },
);

return { request, architecture, contract, implementation, gates, review, acceptance };
