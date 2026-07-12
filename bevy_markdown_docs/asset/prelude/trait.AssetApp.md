[bevy](../../index.html)::[asset](../index.html)::[prelude](index.html)

# Trait AssetApp 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#554)

```rust
pub trait AssetApp {
    // Required methods
    fn register_asset_loader<L>(&mut self, loader: L) -> &mut Self
       where L: AssetLoader;
    fn register_asset_processor<P>(&mut self, processor: P) -> &mut Self
       where P: Process;
    fn register_asset_source(
        &mut self,
        id: impl Into<AssetSourceId<'static>>,
        source: AssetSourceBuilder,
    ) -> &mut Self;
    fn set_default_asset_processor<P>(&mut self, extension: &str) -> &mut Self
       where P: Process;
    fn init_asset_loader<L>(&mut self) -> &mut Self
       where L: AssetLoader + FromWorld;
    fn init_asset<A>(&mut self) -> &mut Self
       where A: Asset;
    fn register_asset_reflect<A>(&mut self) -> &mut Self
       where A: Asset + Reflect + FromReflect + GetTypeRegistration;
    fn preregister_asset_loader<L>(&mut self, extensions: &[&str]) -> &mut Self
       where L: AssetLoader;
}
```

Adds asset-related builder methods to [`App`](../../prelude/struct.App.html "struct bevy::prelude::App").

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#556)

#### fn [register\_asset\_loader](#tymethod.register_asset_loader)<L>(&mut self, loader: L) -> &mut Self

where L: [AssetLoader](../trait.AssetLoader.html "trait bevy::asset::AssetLoader"),

Registers the given `loader` in the [`App`](../../prelude/struct.App.html "struct bevy::prelude::App")’s [`AssetServer`](../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#558)

#### fn [register\_asset\_processor](#tymethod.register_asset_processor)<P>(&mut self, processor: P) -> &mut Self

where P: [Process](../processor/trait.Process.html "trait bevy::asset::processor::Process"),

Registers the given `processor` in the [`App`](../../prelude/struct.App.html "struct bevy::prelude::App")’s [`AssetProcessor`](../processor/struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#563-567)

#### fn [register\_asset\_source](#tymethod.register_asset_source)( &mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetSourceId](../io/enum.AssetSourceId.html "enum bevy::asset::io::AssetSourceId")<'static>>, source: [AssetSourceBuilder](../io/struct.AssetSourceBuilder.html "struct bevy::asset::io::AssetSourceBuilder"), ) -> &mut Self

Registers the given [`AssetSourceBuilder`](../io/struct.AssetSourceBuilder.html "struct bevy::asset::io::AssetSourceBuilder") with the given `id`.

Note that asset sources must be registered before adding [`AssetPlugin`](../../prelude/struct.AssetPlugin.html "struct bevy::prelude::AssetPlugin") to your application, since registered asset sources are built at that point and not after.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#569)

#### fn [set\_default\_asset\_processor](#tymethod.set_default_asset_processor)<P>(&mut self, extension: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> &mut Self

where P: [Process](../processor/trait.Process.html "trait bevy::asset::processor::Process"),

Sets the default asset processor for the given `extension`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#571)

#### fn [init\_asset\_loader](#tymethod.init_asset_loader)<L>(&mut self) -> &mut Self

where L: [AssetLoader](../trait.AssetLoader.html "trait bevy::asset::AssetLoader") + [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Initializes the given loader in the [`App`](../../prelude/struct.App.html "struct bevy::prelude::App")’s [`AssetServer`](../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#579)

#### fn [init\_asset](#tymethod.init_asset)<A>(&mut self) -> &mut Self

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Initializes the given [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") in the [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") by:

*   Registering the [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") in the [`AssetServer`](../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")
*   Initializing the [`AssetEvent`](../../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent") resource for the [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset")
*   Adding other relevant systems and resources for the [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset")
*   Ignoring schedule ambiguities in [`Assets`](../../prelude/struct.Assets.html "struct bevy::prelude::Assets") resource. Any time a system takes mutable access to this resource this causes a conflict, but they rarely actually modify the same underlying asset.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#584-586)

#### fn [register\_asset\_reflect](#tymethod.register_asset_reflect)<A>(&mut self) -> &mut Self

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

Registers the asset type `T` using `[App::register]`, and adds [`ReflectAsset`](../struct.ReflectAsset.html "struct bevy::asset::ReflectAsset") type data to `T` and [`ReflectHandle`](../struct.ReflectHandle.html "struct bevy::asset::ReflectHandle") type data to [`Handle<T>`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") in the type registry.

This enables reflection code to access assets. For detailed information, see the docs on [`ReflectAsset`](../struct.ReflectAsset.html "struct bevy::asset::ReflectAsset") and [`ReflectHandle`](../struct.ReflectHandle.html "struct bevy::asset::ReflectHandle").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#589)

#### fn [preregister\_asset\_loader](#tymethod.preregister_asset_loader)<L>(&mut self, extensions: &\[&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\]) -> &mut Self

where L: [AssetLoader](../trait.AssetLoader.html "trait bevy::asset::AssetLoader"),

Preregisters a loader for the given extensions, that will block asset loads until a real loader is registered.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#592)

### impl [AssetApp](../../prelude/trait.AssetApp.html "trait bevy::prelude::AssetApp") for [App](../../prelude/struct.App.html "struct bevy::prelude::App")