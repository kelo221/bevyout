# Contributing

A 2-4 person team, vibe-coding. This is kept deliberately short.

## Before pushing

Run these four, all green:

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run-dev -- render SuperDuperMart
```

The `run-dev` command is a real smoke test, not decoration — pick whatever cell
you're actually touching; `SuperDuperMart` is just a known-good default. It
catches the class of bug the automated checks can't (Blender/ImageMagick/KTX
tool detection, manifest/viewer drift) on your actual machine.

CI (`.github/workflows/ci.yml`) runs the first three checks on Linux, macOS,
and Windows on every push and PR. It must be green before merging — it exists
so drift and OS-specific bugs (e.g. hardcoded Windows paths in a test) get
caught before they land on someone else's machine.

## Two people, no formal review process (yet)

Direct pushes to `master` are still fine for routine changes. But get a second
pair of eyes — a quick look from the other person, doesn't need to be a formal
PR review — before landing anything that's expensive to get wrong:

- The prepared scene manifest schema (`src/vsa/manifest/`).
- The Fallout↔Bevy coordinate scale (`src/vsa/paths.rs`, ~70 units/metre).
  Changing it invalidates every cached GLB and bake.
- Bumping `NIF_CONVERTER_REVISION` (invalidates the NIF→GLB cache for
  everyone; make sure it's intentional and the cache-bust is warranted).

Revisit this whole section once the team grows past four people.

## Architecture

See `AGENTS.md` for the Vertical Slice Architecture rules (what owns what,
where new features go). Read it before adding a new module or command.

## Documentation

Keep the documentation surfaces separated by purpose:

- `README.md` owns installation and the first-run command path.
- The [project wiki](https://github.com/kelo221/bevyout/wiki) owns durable
  explanations, workflows, compatibility summaries, and troubleshooting.
- `AGENTS.md` owns agent and repository workflow constraints.
- `docs/plans/` owns milestone and wave records; live status remains in GitHub
  issues and milestones.

The repository's checked-in `codegraph.json` maps Markdown files to CodeGraph's
file-level tracker. Re-index documentation with:

```powershell
codegraph index .
codegraph files --path . --pattern '*.md' --format flat
```

This is file-level visibility only. Use `rg` or another normal file-search tool
for Markdown headings and prose.

## Executable spec / feature files

Plain-language `Given/When/Then` feature files (via the `cucumber` crate) live
in `features/`, run under `cargo test`. They're the pragmatic middle ground
between "idea" and "implementation" — mine the README's `prepare`/`bake`
prose for candidate features when adding one. Keep them hermetic: no real
Blender/ImageMagick/KTX or game data, same as the rest of the test suite.
