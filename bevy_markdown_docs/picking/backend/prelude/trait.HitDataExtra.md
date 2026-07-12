[bevy](../../../index.html)::[picking](../../index.html)::[backend](../index.html)::[prelude](index.html)

# Trait HitDataExtra 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#78)

```rust
pub trait HitDataExtra:
    Any
    + Send
    + Sync
    + Debug { }
```

Extra data attached to a [`HitData`](../struct.HitData.html "struct bevy::picking::backend::HitData") by a picking backend.

Use this for backend-specific data like triangle indices, UVs, or material information. Any `Send + Sync + fmt::Debug + 'static` type implements this trait automatically. `Clone` is not required: extra data is stored in an [`Arc`](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc"), so [`HitData`](../struct.HitData.html "struct bevy::picking::backend::HitData") can still implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"). `Clone` requires knowing the size of the type, which is not possible with dynamically dispatched types, so it cannot be used for `dyn HitDataExtra`.

```rust
#[derive(Debug)]
struct MyHitInfo { triangle_index: u32 }
```

Read it back with [`HitData::extra_as`](../struct.HitData.html#method.extra_as "method bevy::picking::backend::HitData::extra_as"):

```rust
fn read_extra(hit: &HitData) {
    if let Some(info) = hit.extra_as::<MyHitInfo>() {
        println!("Hit triangle {}", info.triangle_index);
    }
}
```

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,