[bevy](../../index.html)::[ecs](../index.html)::[intern](index.html)

# Trait Internable 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#97)

```rust
pub trait Internable: Hash + Eq {
    // Required methods
    fn leak(&self) -> &'static Self;
    fn ref_eq(&self, other: &Self) -> bool;
    fn ref_hash<H>(&self, state: &mut H)
       where H: Hasher;
}
```

A trait for internable values.

This is used by [`Interner<T>`](struct.Interner.html "struct bevy::ecs::intern::Interner") to create static references for values that are interned.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#99)

#### fn [leak](#tymethod.leak)(&self) -> &'static Self

Creates a static reference to `self`, possibly leaking memory.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#102)

#### fn [ref\_eq](#tymethod.ref_eq)(&self, other: &Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the two references point to the same value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#105)

#### fn [ref\_hash](#tymethod.ref_hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds the reference to the hasher.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#108)

### impl [Internable](trait.Internable.html "trait bevy::ecs::intern::Internable") for [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#109)

#### fn [leak](#tymethod.leak)(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#114)

#### fn [ref\_eq](#tymethod.ref_eq)(&self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#118)

#### fn [ref\_hash](#tymethod.ref_hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

## Implementors

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#39-46)

### impl [Internable](trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [AppLabel](../../app/trait.AppLabel.html "trait bevy::app::AppLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#19-26)

### impl [Internable](trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [DrawFunctionLabel](../../material/labels/trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#22-61)

### impl [Internable](trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [ScheduleLabel](../schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#5-12)

### impl [Internable](trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [ShaderLabel](../../material/labels/trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#63-176)

### impl [Internable](trait.Internable.html "trait bevy::ecs::intern::Internable") for dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")