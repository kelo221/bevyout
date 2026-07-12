[bevy](../index.html)::[asset](index.html)

# Trait LoadFromPath 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#398)

```rust
pub trait LoadFromPath {
    // Required method
    fn load_from_path_erased(
        &mut self,
        type_id: TypeId,
        path: AssetPath<'static>,
    ) -> UntypedHandle;
}
```

A trait for loading an asset.

There are several ways to load an asset. This trait allows deserializing in many contexts depending on how assets can be loaded. Note all these loads are deferred, and must have a concrete type.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#402-403)

#### fn [load\_from\_path\_erased](#tymethod.load_from_path_erased)( &mut self, type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), path: [AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'static>, ) -> [UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

Initiates the load for the given expected type ID, and the path.

See [`LoadBuilder::load_erased`](struct.LoadBuilder.html#method.load_erased "method bevy::asset::LoadBuilder::load_erased") for more.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#426)

### impl [LoadFromPath](trait.LoadFromPath.html "trait bevy::asset::LoadFromPath") for &[AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#416)

### impl [LoadFromPath](trait.LoadFromPath.html "trait bevy::asset::LoadFromPath") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#406)

### impl [LoadFromPath](trait.LoadFromPath.html "trait bevy::asset::LoadFromPath") for [LoadContext](struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>