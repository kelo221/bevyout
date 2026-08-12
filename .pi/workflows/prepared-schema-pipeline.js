export const meta = {
  name: "prepared_schema_pipeline",
  description: "Change a prepared schema with revision, fingerprint, and data-safety proof",
  phases: [
    { title: "Contract trace" },
    { title: "Revision guard" },
    { title: "Synthetic tests" },
    { title: "Prepare verification" },
    { title: "Real-data acceptance" },
  ],
};

const change = String(args?.change ?? args?.request ?? "the requested prepared-schema change");

phase("Contract trace");
const trace = await agent(
  `Trace this serialized contract from source records through Prepared* types, fingerprints, caches, revisions, and viewer consumers. Use CodeGraph and the explicit revision map. Do not edit. CHANGE: ${change}`,
  { label: "schema trace", tier: "scout", tools: ["read", "bash"] },
);

phase("Revision guard");
const guard = await agent(
  `Define the exact mapped *_REVISION bump and old-cache rejection behavior required for this change. Identify multi-schema effects and guard tests.\n\n${trace}`,
  { label: "revision design", tier: "reviewer", tools: ["read", "bash"] },
);

phase("Synthetic tests");
const implementation = await agent(
  `Implement the schema change, mapped revision bump, synthetic fixtures, serialization round-trip, fingerprint, and stale-cache tests. Never use or commit Bethesda-derived data. Run Staged and Fast checks.\n\nCHANGE:\n${change}\n\nTRACE:\n${trace}\n\nGUARD:\n${guard}`,
  { label: "schema implementation", tier: "worker", tools: ["read", "bash", "edit", "write"] },
);

phase("Prepare verification");
const prepare = await agent(
  `Independently verify revision guard behavior, deterministic fingerprinting, synthetic prepare output, and viewer rejection of stale artifacts. Do not edit.\n\n${implementation}`,
  { label: "prepare verification", tier: "reviewer", tools: ["read", "bash"] },
);

phase("Real-data acceptance");
const acceptance = await agent(
  `If compatible ignored prepared data already exists, run a bounded viewer smoke via the MCP CLI. Otherwise report unavailable. Never create or commit derived assets.\n\n${prepare}`,
  { label: "real data acceptance", tier: "reviewer", tools: ["read", "bash"] },
);

return { change, trace, guard, implementation, prepare, acceptance };
