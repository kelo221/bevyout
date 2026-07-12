[bevy](../index.html)::[prelude](index.html)

# Trait ValNum 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#514)

```rust
pub trait ValNum {
    // Required method
    fn val_num_f32(self) -> f32;
}
```

All the types that should be able to be used in the [`Val`](enum.Val.html "enum bevy::prelude::Val") enum should implement this trait.

Instead of just implementing `Into<Val>` a custom trait is added. This is done in order to prevent having to define a default unit, which could lead to confusion especially for newcomers.

## Required Methods

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#517)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Called by the [`Val`](enum.Val.html "enum bevy::prelude::Val") helper functions to convert the implementing type to an `f32` that can be used by [`Val`](enum.Val.html "enum bevy::prelude::Val").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

### impl [ValNum](trait.ValNum.html "trait bevy::prelude::ValNum") for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#532)

#### fn [val\_num\_f32](#tymethod.val_num_f32)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

## Implementors