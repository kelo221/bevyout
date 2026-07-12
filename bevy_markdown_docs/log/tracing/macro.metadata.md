[bevy](../../index.html)::[log](../index.html)::[tracing](index.html)

# Macro metadata 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/lib.rs.html#230)

```rust
macro_rules! metadata {
    (
        name: $name:expr,
        target: $target:expr,
        level: $level:expr,
        fields: $fields:expr,
        callsite: $callsite:expr,
        kind: $kind:expr
    ) => { ... };
    (
        name: $name:expr,
        target: $target:expr,
        level: $level:expr,
        fields: $fields:expr,
        callsite: $callsite:expr,
        kind: $kind:expr,
    ) => { ... };
}
```

Statically constructs new span [metadata](struct.Metadata.html "struct bevy::log::tracing::Metadata").

/// For example:

```rust
use tracing_core::metadata;
use tracing_core::metadata::{Kind, Level, Metadata};
static FOO_CALLSITE: MyCallsite = MyCallsite {
    // ...
};

static FOO_METADATA: Metadata = metadata!{
    name: "foo",
    target: module_path!(),
    level: Level::DEBUG,
    fields: &["bar", "baz"],
    callsite: &FOO_CALLSITE,
    kind: Kind::SPAN,
};
```