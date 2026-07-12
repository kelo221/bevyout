[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[level\_filters](index.html)

# Constant STATIC\_MAX\_LEVEL 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/level_filters.rs.html#66)

```rust
pub const STATIC_MAX_LEVEL: LevelFilter;
```

The statically configured maximum trace level.

See the [module-level documentation](index.html#compile-time-filters "mod bevy::log::tracing::level_filters") for information on how to configure this.

This value is checked by the `event!` and `span!` macros. Code that manually constructs events or spans via the `Event::record` function or `Span` constructors should compare the level against this value to determine if those spans or events are enabled.