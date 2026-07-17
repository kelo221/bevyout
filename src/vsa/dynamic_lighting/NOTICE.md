# DynamicLighting provenance

The frozen original Unity package is kept at
`src/vsa/dynamic_lighting/upstream/`. It is intentionally ignored and is not
compiled by Cargo. The reference and every Rust runtime, Bevy bridge, renderer,
shader, exporter, and test for the port therefore live below one isolated
directory.

- Upstream repository: <https://github.com/Henry00IS/DynamicLighting>
- Upstream baseline commit: `dd7c195cba2599a20bf1b662fa0f69366e0f74b5`
- Upstream package version: `1.330.0`
- Upstream minimum Unity version: `2018.3` (from `package.json`)
- Golden-reference Unity editor: `6000.3.17f1`
- bevyout `DynamicLightsPort` baseline: `8c90fea02ddb325794caa0a37d3763dc5b5cf061`
- Upstream license: MIT, Copyright (c) 2020 Henry de Jongh / Alpaca IT

The reference harness compiles the unmodified upstream `LightEffects/*.cs`
files with a small test-only partial class, then asks Unity to emit JSON. Rust
and WGSL are translations derived from those source expressions; copied source
references are recorded in `PORT_MATRIX.md` and comments beside translated
functions. The upstream package itself is not distributed by this crate.
