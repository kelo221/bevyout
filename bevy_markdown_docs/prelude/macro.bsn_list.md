[bevy](../index.html)::[prelude](index.html)

# Macro bsn\_list 

[Source](https://docs.rs/bevy_scene_macros/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene_macros/lib.rs.html#67)

```rust
bsn_list!() { /* proc-macro */ }
```

Creates a `SceneList` using BSN (Bevy Scene Notation) syntax.

This is useful when you want multiple root entities in your scene that do not share a common parent, or if you want to create multiple scenes at once.

Like in \[`bsn!`\], commas separate entities, while whitespace separates components on the same entity.

All root entries in a [`bsn_list!`](macro.bsn_list.html "macro bevy::prelude::bsn_list") share a single name scope, so sibling root entities can cross-reference each other by `#Name`. This is not possible with separate \[`bsn!`\] calls, and is a key motivation for using [`bsn_list!`](macro.bsn_list.html "macro bevy::prelude::bsn_list").

See \[`bsn!`\] for an example of the syntax. See the `bevy_scene` crate docs for a high-level overview of the key concepts.#\[doc(hidden)\]