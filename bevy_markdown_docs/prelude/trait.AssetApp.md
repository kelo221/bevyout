[bevy](../index.html)::[prelude](index.html)

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

Adds asset-related builder methods to [`App`](struct.App.html "struct bevy::prelude::App").

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#556)

#### fn [register\_asset\_loader](#tymethod.register_asset_loader)<L>(&mut self, loader: L) -> &mut Self

where L: [AssetLoader](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader"),

Registers the given `loader` in the [`App`](struct.App.html "struct bevy::prelude::App")’s [`AssetServer`](struct.AssetServer.html "struct bevy::prelude::AssetServer").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#558)

#### fn [register\_asset\_processor](#tymethod.register_asset_processor)<P>(&mut self, processor: P) -> &mut Self

where P: [Process](../asset/processor/trait.Process.html "trait bevy::asset::processor::Process"),

Registers the given `processor` in the [`App`](struct.App.html "struct bevy::prelude::App")’s [`AssetProcessor`](../asset/processor/struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#563-567)

#### fn [register\_asset\_source](#tymethod.register_asset_source)( &mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetSourceId](../asset/io/enum.AssetSourceId.html "enum bevy::asset::io::AssetSourceId")<'static>>, source: [AssetSourceBuilder](../asset/io/struct.AssetSourceBuilder.html "struct bevy::asset::io::AssetSourceBuilder"), ) -> &mut Self

Registers the given [`AssetSourceBuilder`](../asset/io/struct.AssetSourceBuilder.html "struct bevy::asset::io::AssetSourceBuilder") with the given `id`.

Note that asset sources must be registered before adding [`AssetPlugin`](struct.AssetPlugin.html "struct bevy::prelude::AssetPlugin") to your application, since registered asset sources are built at that point and not after.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#569)

#### fn [set\_default\_asset\_processor](#tymethod.set_default_asset_processor)<P>(&mut self, extension: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> &mut Self

where P: [Process](../asset/processor/trait.Process.html "trait bevy::asset::processor::Process"),

Sets the default asset processor for the given `extension`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#571)

#### fn [init\_asset\_loader](#tymethod.init_asset_loader)<L>(&mut self) -> &mut Self

where L: [AssetLoader](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") + [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Initializes the given loader in the [`App`](struct.App.html "struct bevy::prelude::App")’s [`AssetServer`](struct.AssetServer.html "struct bevy::prelude::AssetServer").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#579)

#### fn [init\_asset](#tymethod.init_asset)<A>(&mut self) -> &mut Self

where A: [Asset](trait.Asset.html "trait bevy::prelude::Asset"),

Initializes the given [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") in the [`App`](struct.App.html "struct bevy::prelude::App") by:

*   Registering the [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") in the [`AssetServer`](struct.AssetServer.html "struct bevy::prelude::AssetServer")
*   Initializing the [`AssetEvent`](enum.AssetEvent.html "enum bevy::prelude::AssetEvent") resource for the [`Asset`](trait.Asset.html "trait bevy::prelude::Asset")
*   Adding other relevant systems and resources for the [`Asset`](trait.Asset.html "trait bevy::prelude::Asset")
*   Ignoring schedule ambiguities in [`Assets`](struct.Assets.html "struct bevy::prelude::Assets") resource. Any time a system takes mutable access to this resource this causes a conflict, but they rarely actually modify the same underlying asset.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#584-586)

#### fn [register\_asset\_reflect](#tymethod.register_asset_reflect)<A>(&mut self) -> &mut Self

where A: [Asset](trait.Asset.html "trait bevy::prelude::Asset") + [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

Registers the asset type `T` using `[App::register]`, and adds [`ReflectAsset`](../asset/struct.ReflectAsset.html "struct bevy::asset::ReflectAsset") type data to `T` and [`ReflectHandle`](../asset/struct.ReflectHandle.html "struct bevy::asset::ReflectHandle") type data to [`Handle<T>`](enum.Handle.html "enum bevy::prelude::Handle") in the type registry.

This enables reflection code to access assets. For detailed information, see the docs on [`ReflectAsset`](../asset/struct.ReflectAsset.html "struct bevy::asset::ReflectAsset") and [`ReflectHandle`](../asset/struct.ReflectHandle.html "struct bevy::asset::ReflectHandle").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#589)

#### fn [preregister\_asset\_loader](#tymethod.preregister_asset_loader)<L>(&mut self, extensions: &\[&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\]) -> &mut Self

where L: [AssetLoader](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader"),

Preregisters a loader for the given extensions, that will block asset loads until a real loader is registered.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#592)

### impl [AssetApp](trait.AssetApp.html "trait bevy::prelude::AssetApp") for [App](struct.App.html "struct bevy::prelude::App")