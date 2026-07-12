[bevy](../index.html)::[audio](index.html)

# Type Alias ChannelCount 

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/common.rs.html#8)

```rust
pub type ChannelCount = NonZero<u16>;
```

Number of channels in a stream. Can never be Zero

## Aliased Type

```rust
pub struct ChannelCount(/* private fields */);
```