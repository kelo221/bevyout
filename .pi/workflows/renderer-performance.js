export const meta = {
  name: "renderer_performance",
  description: "Measure and tune one Bevyout renderer or schedule bottleneck",
  phases: [
    { title: "Baseline" },
    { title: "Probe" },
    { title: "Fix" },
    { title: "Compare" },
  ],
};

const target = String(args?.target ?? args?.request ?? "the reported renderer/performance issue");

phase("Baseline");
const baseline = await agent(
  `Capture a reproducible baseline for ${target}. Use existing diagnostics plus bounded MCP status, schedule, performance, and capture commands. Record command, scene/state, warmup, sample count, machine caveats, and artifacts. Do not edit.`,
  { label: "performance baseline", tier: "worker", tools: ["read", "bash"] },
);

phase("Probe");
const probe = await agent(
  `Trace the dominant measured cost through renderer schedules, extraction/prepare/queue, systems, and local bevy_pbr ownership. Recommend one variable to change. Do not infer from candidate counts alone.\n\nTARGET:\n${target}\n\nBASELINE:\n${baseline}`,
  { label: "performance probe", tier: "scout", tools: ["read", "bash"] },
);

phase("Fix");
const fix = await agent(
  `Implement only the measured one-variable fix. Preserve visuals and prepared/runtime boundaries. Add focused regression/architecture coverage and run Fast checks.\n\n${probe}`,
  { label: "performance fix", tier: "worker", tools: ["read", "bash", "edit", "write"] },
);

phase("Compare");
const comparison = await agent(
  `Repeat the identical baseline protocol after the fix. Reject incomparable or noisy runs. Report before/after raw values, delta, visual/capture evidence, and acceptance. Do not edit.\n\nBASELINE:\n${baseline}\n\nFIX:\n${fix}`,
  { label: "performance comparison", tier: "reviewer", tools: ["read", "bash"] },
);

return { target, baseline, probe, fix, comparison };
