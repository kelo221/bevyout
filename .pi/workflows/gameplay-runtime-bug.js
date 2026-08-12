export const meta = {
  name: "gameplay_runtime_bug",
  description: "Reproduce, trace, regress, fix, and live-verify a Bevyout gameplay bug",
  phases: [
    { title: "Reproduce" },
    { title: "Trace" },
    { title: "Regression" },
    { title: "Fix" },
    { title: "Verify" },
  ],
};

const bug = String(args?.bug ?? args?.request ?? "the reported gameplay bug");

phase("Reproduce");
const reproduction = await agent(
  `Reproduce this Bevyout bug without editing. Prefer the bounded MCP CLI or an existing .bscript. Capture status, scene/console evidence, exact steps, and whether live reproduction is unavailable. BUG: ${bug}`,
  { label: "bug reproduction", tier: "worker", tools: ["read", "bash"] },
);

phase("Trace");
const trace = await agent(
  `Trace the reproduced bug with CodeGraph and Rust navigation. Identify the first wrong transition, schedule ordering, queries/resources, and minimal ownership point.\n\nBUG:\n${bug}\n\nEVIDENCE:\n${reproduction}`,
  { label: "bug trace", tier: "scout", tools: ["read", "bash"] },
);

phase("Regression");
const regression = await agent(
  `Define and add the smallest failing regression test for this bug. Do not implement the production fix yet. Return the failing command/evidence.\n\n${trace}`,
  { label: "regression test", tier: "worker", tools: ["read", "bash", "edit", "write"] },
);

phase("Fix");
const fix = await agent(
  `Implement the narrow production fix that makes the regression pass. Preserve public/runtime behavior outside the bug. Run focused Fast checks and inspect the diff.\n\nTRACE:\n${trace}\n\nREGRESSION:\n${regression}`,
  { label: "bug fix", tier: "worker", tools: ["read", "bash", "edit", "write"] },
);

phase("Verify");
const verification = await agent(
  `Independently verify the regression and the original live reproduction with bounded MCP CLI output. Do not edit. Separate static and runtime proof.\n\nBUG:\n${bug}\n\nREPRO:\n${reproduction}\n\nFIX:\n${fix}`,
  { label: "bug verification", tier: "reviewer", tools: ["read", "bash"] },
);

return { bug, reproduction, trace, regression, fix, verification };
