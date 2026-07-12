[bevy](../index.html)::[audio](index.html)

# Type Alias Sample 

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/common.rs.html#43)

```rust
pub type Sample = f32;
```

Represents value of a single sample. Silence corresponds to the value `0.0`. The expected amplitude range is -1.0…1.0. Values below and above this range are clipped in conversion to other sample types. Use conversion traits from [dasp\_sample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/index.html "mod dasp_sample") crate or [crate::conversions::SampleTypeConverter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/conversions/sample/struct.SampleTypeConverter.html "struct rodio::conversions::sample::SampleTypeConverter") to convert between sample types if necessary.