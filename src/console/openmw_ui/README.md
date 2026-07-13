# OpenMW-derived console UX helpers

This isolated module adapts the user-facing behavior of the OpenMW 0.52
console into pure Rust helpers: bounded persistent history, unfinished-draft
restoration, case-insensitive longest-common-prefix completion, repeated-Tab
candidate listing, and selected-reference titles supplied by bevyout's own
console frontend.

It does not compile or port MyGUI, Lua console modes, OpenMW's compiler,
interpreter, opcode registry, or runtime object model. The command parser and
executor are project-native Rust and use Gamebryo-style syntax directly.

See `NOTICE.md` for the source map, hashes, and license attribution.
