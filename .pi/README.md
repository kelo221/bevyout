# bevyout Pi Harness

This is the Bevyout full/lean Pi harness. The launcher itself selects no
provider or foreground model; saved agent routes are project-owned and bounded.

## Launch

From any directory:

```sh
bun run C:/Users/V/Projects/Rust/bevyout/tools/pi-bevyout.ts
```

All Pi arguments are preserved:

```sh
bun run tools/pi-bevyout.ts --provider openai --model <model-id>
```

The launcher changes to the repository root, disables automatic root context
loading, approves project-local Pi settings for that run, and explicitly loads
`.pi/PI.md`. It does not write user-global settings. Detailed project knowledge
remains on demand in `AGENTS.md`, repository skills, local Bevy documentation,
and the project wiki.

## Validate

No model request or running viewer is required:

```sh
bun run tools/pi-bevyout.ts --offline --version
bun run tools/harness/doctor.ts
git diff --check
```

Root package aliases are also available, for example `bun run pi:lean`,
`bun run harness:fast`, and `bun run harness:full`.
