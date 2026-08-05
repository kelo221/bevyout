# CLI progress manual acceptance

This verifies that prepare and bake keep deterministic summaries on stdout and
send opt-in progress rendering to stderr. It assumes `.bevyout/config.toml`
is configured and that `SuperDuperMart` is present in the configured Fallout 3
content set. Use another known prepared cell selector if the local data set
does not contain that cell.

1. Run a terminal prepare:

   ```powershell
   cargo run-dev -- prepare SuperDuperMart --progress tty
   ```

   Expected: the terminal shows one updating progress line. It may include
   `cell`, preparation phases, asset cache counts, and native job counts. The
   final `prepared ...` and `asset cache: ...` summaries remain normal stdout
   lines.

2. Run a terminal CPU bake:

   ```powershell
   cargo run-dev -- bake SuperDuperMart --progress tty --bake-backend cpu
   ```

   Expected: progress identifies `CPU`, scene composition, primitive/tile
   transport, cache hits/misses, denoise/dilation, atlas encoding, irradiance
   probes, and manifest publication. The existing `Rust bake: ...`, surface
   lightmap, cache, and batching summaries remain unchanged on stdout.

3. Verify redirected plain output:

   ```powershell
   cargo run-dev -- prepare SuperDuperMart --progress plain 1>prepare.stdout 2>prepare.stderr
   cargo run-dev -- bake SuperDuperMart --progress plain --bake-backend cpu 1>bake.stdout 2>bake.stderr
   ```

   Expected: `prepare.stderr` and `bake.stderr` contain newline-delimited
   progress with no carriage returns or ANSI escape sequences. `prepare.stdout`
   and `bake.stdout` retain the deterministic summaries.

4. Verify the compatibility mode:

   ```powershell
   cargo run-dev -- prepare SuperDuperMart --progress off 1>off.stdout 2>off.stderr
   cargo run-dev -- bake SuperDuperMart --progress off --bake-backend cpu 1>off-bake.stdout 2>off-bake.stderr
   ```

   Expected: no progress lines are added to stderr. Compare the stdout files
   with the corresponding successful runs; the existing summaries are still
   present and retain their wording.
