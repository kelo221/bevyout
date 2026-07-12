[bevy](../index.html)

# Crate asset 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#1-3328)

In the context of game development, an “asset” is a piece of content that is loaded from disk and displayed in the game. Typically, these are authored by artists and designers (in contrast to code), are relatively large in size, and include everything from textures and models to sounds and music to levels and scripts.

This presents two main challenges:

*   Assets take up a lot of memory; simply storing a copy for each instance of an asset in the game would be prohibitively expensive.
*   Loading assets from disk is slow, and can cause long load times and delays.

These problems play into each other, for if assets are expensive to store in memory, then larger game worlds will need to load them from disk as needed, ideally without a loading screen.

As is common in Rust, non-blocking asset loading is done using `async`, with background tasks used to load assets while the game is running. Bevy coordinates these tasks using the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") resource, storing each loaded asset in a strongly-typed [`Assets<T>`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") collection (also a resource). [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle")s serve as an id-based reference to entries in the [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") collection, allowing them to be cheaply shared between systems, and providing a way to initialize objects (generally entities) before the required assets are loaded. In short: [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle")s are not the assets themselves, they just tell how to look them up!

### Loading assets

The [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") is the main entry point for loading assets. Typically, you’ll use the [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load") method to load an asset from disk, which returns a [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle"). Note that this method does not attempt to reload the asset if it has already been loaded: as long as at least one handle has not been dropped, calling [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load") on the same path will return the same handle. The handle that’s returned can be used to instantiate various [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")s that require asset data to function, which will then be spawned into the world as part of an entity.

To avoid assets “popping” into existence, you may want to check that all of the required assets are loaded before transitioning to a new scene. This can be done by checking the [`LoadState`](enum.LoadState.html "enum bevy::asset::LoadState") of the asset handle using [`AssetServer::is_loaded_with_dependencies`](../prelude/struct.AssetServer.html#method.is_loaded_with_dependencies "method bevy::prelude::AssetServer::is_loaded_with_dependencies"), which will be `true` when the asset is ready to use.

Keep track of what you’re waiting on by using a [`HashSet`](../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") of asset handles or similar data structure, which iterate over and poll in your update loop, and transition to the new scene once all assets are loaded. Bevy’s built-in states system can be very helpful for this!

## Modifying entities that use assets

If we later want to change the asset data a given component uses (such as changing an entity’s material), we have three options:

1.  Change the handle stored on the responsible component to the handle of a different asset
2.  Despawn the entity and spawn a new one with the new asset data.
3.  Use the [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") collection to directly modify the current handle’s asset data

The first option is the most common: just query for the component that holds the handle, and mutate it, pointing to the new asset. Check how the handle was passed in to the entity when it was spawned: if a mesh-related component required a handle to a mesh asset, you’ll need to find that component via a query and change the handle to the new mesh asset. This is so commonly done that you should think about strategies for how to store and swap handles in your game.

The second option is the simplest, but can be slow if done frequently, and can lead to frustrating bugs as references to the old entity (such as what is targeting it) and other data on the entity are lost. Generally, this isn’t a great strategy.

The third option has different semantics: rather than modifying the asset data for a single entity, it modifies the asset data for _all_ entities using this handle. While this might be what you want, it generally isn’t!

## Hot reloading assets

Bevy supports asset hot reloading, allowing you to change assets on disk and see the changes reflected in your game without restarting. When enabled, any changes to the underlying asset file will be detected by the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer"), which will then reload the asset, mutating the asset data in the [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") collection and thus updating all entities that use the asset. While it has limited uses in published games, it is very useful when developing, as it allows you to iterate quickly.

To enable asset hot reloading on desktop platforms, enable `bevy`’s `file_watcher` cargo feature. To toggle it at runtime, you can use the `watch_for_changes_override` field in the [`AssetPlugin`](../prelude/struct.AssetPlugin.html "struct bevy::prelude::AssetPlugin") to enable or disable hot reloading.

## Procedural asset creation

Not all assets are loaded from disk: some are generated at runtime, such as procedural materials, sounds or even levels. After creating an item of a type that implements [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), you can add it to the [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") collection using [`Assets::add`](../prelude/struct.Assets.html#method.add "method bevy::prelude::Assets::add"). Once in the asset collection, this data can be operated on like any other asset.

Note that, unlike assets loaded from a file path, no general mechanism currently exists to deduplicate procedural assets: calling [`Assets::add`](../prelude/struct.Assets.html#method.add "method bevy::prelude::Assets::add") for every entity that needs the asset will create a new copy of the asset for each entity, quickly consuming memory.

### Handles and reference counting

[`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") (or their untyped counterpart [`UntypedHandle`](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")) are used to reference assets in the [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") collection, and are the primary way to interact with assets in Bevy. As a user, you’ll be working with handles a lot!

The most important thing to know about handles is that they are reference counted: when you clone a handle, you’re incrementing a reference count. When the object holding the handle is dropped (generally because an entity was despawned), the reference count is decremented. When the reference count hits zero, the asset it references is removed from the [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") collection.

This reference counting is a simple, largely automatic way to avoid holding onto memory for game objects that are no longer in use. However, it can lead to surprising behavior if you’re not careful!

There are two categories of problems to watch out for:

*   never dropping a handle, causing the asset to never be removed from memory
*   dropping a handle too early, causing the asset to be removed from memory while it’s still in use

The first problem is less critical for beginners, as for tiny games, you can often get away with simply storing all of the assets in memory at once, and loading them all at the start of the game. As your game grows, you’ll need to be more careful about when you load and unload assets, segmenting them by level or area, and loading them on-demand. This problem generally arises when handles are stored in a persistent “collection” or “manifest” of possible objects (generally in a resource), which is convenient for easy access and zero-latency spawning, but can result in high but stable memory usage.

The second problem is more concerning, and looks like your models or textures suddenly disappearing from the game. Debugging reveals that the _entities_ are still there, but nothing is rendering! This is because the assets were removed from memory while they were still in use. You were probably too aggressive with the use of weak handles (which don’t increment the reference count of the asset): think through the lifecycle of your assets carefully! As soon as an asset is loaded, you must ensure that at least one strong handle is held to it until all matching entities are out of sight of the player.

## Asset dependencies

Some assets depend on other assets to be loaded before they can be loaded themselves. For example, a 3D model might require both textures and meshes to be loaded, or a 2D level might require a tileset to be loaded.

The assets that are required to load another asset are called “dependencies”. An asset is only considered fully loaded when it and all of its dependencies are loaded. Asset dependencies can be declared when implementing the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") trait by implementing the [`VisitAssetDependencies`](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") trait, and the `#[dependency]` attribute can be used to automatically derive this implementation.

## Custom asset types

While Bevy comes with implementations for a large number of common game-oriented asset types (often behind off-by-default feature flags!), implementing a custom asset type can be useful when dealing with unusual, game-specific, or proprietary formats.

Defining a new asset type is as simple as implementing the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") trait. This requires [`TypePath`](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for metadata about the asset type, and [`VisitAssetDependencies`](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") to track asset dependencies. In simple cases, you can derive [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") and [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and be done with it: the required supertraits will be implemented for you.

With a new asset type in place, we now need to figure out how to load it. While [`AssetReader`](io/trait.AssetReader.html "trait bevy::asset::io::AssetReader") describes strategies to read asset bytes from various sources, [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") is the trait that actually turns those into your desired in-memory format. Generally, (only) [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") needs to be implemented for custom assets, as the [`AssetReader`](io/trait.AssetReader.html "trait bevy::asset::io::AssetReader") implementations are provided by Bevy.

However, [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") shouldn’t be implemented for your asset type directly: instead, this is implemented for a “loader” type that can store settings and any additional data required to load your asset, while your asset type is used as the [`AssetLoader::Asset`](trait.AssetLoader.html#associatedtype.Asset "associated type bevy::asset::AssetLoader::Asset") associated type. As the trait documentation explains, this allows various [`AssetLoader::Settings`](trait.AssetLoader.html#associatedtype.Settings "associated type bevy::asset::AssetLoader::Settings") to be used to configure the loader.

After the loader is implemented, it needs to be registered with the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") using [`App::register_asset_loader`](../prelude/trait.AssetApp.html#tymethod.register_asset_loader "method bevy::prelude::AssetApp::register_asset_loader"). Once your asset type is loaded, you can use it in your game like any other asset type!

## Modules

[io](io/index.html "mod bevy::asset::io")

[meta](meta/index.html "mod bevy::asset::meta")

[prelude](prelude/index.html "mod bevy::asset::prelude")

The asset prelude.

[processor](processor/index.html "mod bevy::asset::processor")

Asset processing in Bevy is a framework for automatically transforming artist-authored assets into the format that best suits the needs of your particular game.

[saver](saver/index.html "mod bevy::asset::saver")

[transformer](transformer/index.html "mod bevy::asset::transformer")

[uuid](uuid/index.html "mod bevy::asset::uuid")

Generate and parse universally unique identifiers (UUIDs).

## Macros

[embedded\_asset](macro.embedded_asset.html "macro bevy::asset::embedded_asset")

Creates a new `embedded` asset by embedding the bytes of the given path into the current binary and registering those bytes with the `embedded` [`AssetSource`](io/struct.AssetSource.html "struct bevy::asset::io::AssetSource").

[embedded\_path](macro.embedded_path.html "macro bevy::asset::embedded_path")

Returns the [`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path") for a given `embedded` asset. This is used internally by [`embedded_asset`](macro.embedded_asset.html "macro bevy::asset::embedded_asset") and can be used to get a [`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path") that matches the [`AssetPath`](struct.AssetPath.html "struct bevy::asset::AssetPath") used by that asset.

[load\_embedded\_asset](macro.load_embedded_asset.html "macro bevy::asset::load_embedded_asset")

Load an [embedded asset](macro.embedded_asset.html "macro bevy::asset::embedded_asset").

[load\_internal\_asset](macro.load_internal_asset.html "macro bevy::asset::load_internal_asset")

Loads an “internal” asset by embedding the string stored in the given `path_str` and associates it with the given handle.

[load\_internal\_binary\_asset](macro.load_internal_binary_asset.html "macro bevy::asset::load_internal_binary_asset")

Loads an “internal” binary asset by embedding the bytes stored in the given `path_str` and associates it with the given handle.

[uuid\_handle](macro.uuid_handle.html "macro bevy::asset::uuid_handle")

Creates a [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") from a string literal containing a UUID.

[weak\_handle](macro.weak_handle.html "macro bevy::asset::weak_handle")Deprecated

## Structs

[AddAsyncError](struct.AddAsyncError.html "struct bevy::asset::AddAsyncError")

An error that occurs while resolving an asset added by `add_async`.

[ArcMutexValue](struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")

Stores an [`Arc<Mutex<AssetOrHandle<T>>>`](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc").

[AssetEventSystems](struct.AssetEventSystems.html "struct bevy::asset::AssetEventSystems")

A system set where events accumulated in [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") are applied to the [`AssetEvent`](../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent") [`Messages`](../prelude/struct.Messages.html "struct bevy::prelude::Messages") resource.

[AssetHandleProvider](struct.AssetHandleProvider.html "struct bevy::asset::AssetHandleProvider")

Provides [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") and [`UntypedHandle`](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle") _for a specific asset type_. This should _only_ be used for one specific asset type.

[AssetIndex](struct.AssetIndex.html "struct bevy::asset::AssetIndex")

A generational runtime-only identifier for a specific [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") stored in [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets"). This is optimized for efficient runtime usage and is not suitable for identifying assets across app runs.

[AssetLoadFailedEvent](struct.AssetLoadFailedEvent.html "struct bevy::asset::AssetLoadFailedEvent")

A [`Message`](../prelude/trait.Message.html "trait bevy::prelude::Message") emitted when a specific [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") fails to load.

[AssetLoaderError](struct.AssetLoaderError.html "struct bevy::asset::AssetLoaderError")

An error that can occur during asset loading.

[AssetMut](struct.AssetMut.html "struct bevy::asset::AssetMut")

Unique mutable borrow of an asset.

[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")

Represents a path to an asset in a “virtual filesystem”.

[AssetPlugin](struct.AssetPlugin.html "struct bevy::asset::AssetPlugin")

Provides “asset” loading and processing functionality. An [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") is a “runtime value” that is loaded from an [`AssetSource`](io/struct.AssetSource.html "struct bevy::asset::io::AssetSource"), which can be something like a filesystem, a network, etc.

[AssetServer](struct.AssetServer.html "struct bevy::asset::AssetServer")

Loads and tracks the state of [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") values from a configured [`AssetReader`](io/trait.AssetReader.html "trait bevy::asset::io::AssetReader"). This can be used to kick off new asset loads and retrieve their current load states.

[AssetTrackingSystems](struct.AssetTrackingSystems.html "struct bevy::asset::AssetTrackingSystems")

A system set that holds all “track asset” operations.

[Assets](struct.Assets.html "struct bevy::asset::Assets")

Stores [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") values identified by their [`AssetId`](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId").

[AssetsMutIterator](struct.AssetsMutIterator.html "struct bevy::asset::AssetsMutIterator")

A mutable iterator over [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets").

[ErasedLoadedAsset](struct.ErasedLoadedAsset.html "struct bevy::asset::ErasedLoadedAsset")

A “type erased / boxed” counterpart to [`LoadedAsset`](struct.LoadedAsset.html "struct bevy::asset::LoadedAsset"). This is used in places where the loaded type is not statically known.

[HandleDeserializeProcessor](struct.HandleDeserializeProcessor.html "struct bevy::asset::HandleDeserializeProcessor")

A [`ReflectDeserializerProcessor`](../reflect/serde/trait.ReflectDeserializerProcessor.html "trait bevy::reflect::serde::ReflectDeserializerProcessor") that manually deserializes [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") and [`UntypedHandle`](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle"), and passes through for all other types.

[HandleSerializeProcessor](struct.HandleSerializeProcessor.html "struct bevy::asset::HandleSerializeProcessor")

A [`ReflectSerializerProcessor`](../reflect/serde/trait.ReflectSerializerProcessor.html "trait bevy::reflect::serde::ReflectSerializerProcessor") that manually serializes [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") and [`UntypedHandle`](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle"), and passes through for all other types.

[LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")

A builder for initiating a more complex load than the one provided by [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load").

[LoadContext](struct.LoadContext.html "struct bevy::asset::LoadContext")

A context that provides access to assets in [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader")s, tracks dependencies, and collects asset load state.

[LoadedAsset](struct.LoadedAsset.html "struct bevy::asset::LoadedAsset")

The successful result of an [`AssetLoader::load`](trait.AssetLoader.html#tymethod.load "method bevy::asset::AssetLoader::load") call. This contains the loaded “root” asset and any other “labeled” assets produced by the loader. It also holds the input [`AssetMeta`](meta/struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta") (if it exists) and tracks dependencies:

[LoadedFolder](struct.LoadedFolder.html "struct bevy::asset::LoadedFolder")

A “loaded folder” containing handles for all assets stored in a given [`AssetPath`](struct.AssetPath.html "struct bevy::asset::AssetPath").

[LoadedUntypedAsset](struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset")

A “loaded asset” containing the untyped handle for an asset stored in a given [`AssetPath`](struct.AssetPath.html "struct bevy::asset::AssetPath").

[MissingAssetLoaderForExtensionError](struct.MissingAssetLoaderForExtensionError.html "struct bevy::asset::MissingAssetLoaderForExtensionError")

An error that occurs when an [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") is not registered for a given extension.

[MissingAssetLoaderForTypeIdError](struct.MissingAssetLoaderForTypeIdError.html "struct bevy::asset::MissingAssetLoaderForTypeIdError")

An error that occurs when an [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") is not registered for a given [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId").

[MissingAssetLoaderForTypeNameError](struct.MissingAssetLoaderForTypeNameError.html "struct bevy::asset::MissingAssetLoaderForTypeNameError")

An error that occurs when an [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") is not registered for a given [`core::any::type_name`](https://doc.rust-lang.org/nightly/core/any/fn.type_name.html "fn core::any::type_name").

[NestedLoadBuilder](struct.NestedLoadBuilder.html "struct bevy::asset::NestedLoadBuilder")

A builder for loading nested assets inside a [`LoadContext`](struct.LoadContext.html "struct bevy::asset::LoadContext").

[ReflectAsset](struct.ReflectAsset.html "struct bevy::asset::ReflectAsset")

Type data for the [`TypeRegistry`](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry") used to operate on reflected [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset")s.

[ReflectHandle](struct.ReflectHandle.html "struct bevy::asset::ReflectHandle")

Reflect type data struct relating a [`Handle<T>`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") back to the `T` asset type.

[RenderAssetUsages](struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages")

Defines where the asset will be used.

[StrongHandle](struct.StrongHandle.html "struct bevy::asset::StrongHandle")

The internal “strong” [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") handle storage for [`Handle::Strong`](../prelude/enum.Handle.html#variant.Strong "variant bevy::prelude::Handle::Strong") and [`UntypedHandle::Strong`](../prelude/enum.UntypedHandle.html#variant.Strong "variant bevy::prelude::UntypedHandle::Strong"). When this is dropped, the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") will be freed. It also stores some asset metadata for easy access from handles.

[TypedHandleReference](struct.TypedHandleReference.html "struct bevy::asset::TypedHandleReference")

The “stable” data of a handle whose asset type information is stored internally.

[UntypedAssetLoadFailedEvent](struct.UntypedAssetLoadFailedEvent.html "struct bevy::asset::UntypedAssetLoadFailedEvent")

An untyped version of [`AssetLoadFailedEvent`](struct.AssetLoadFailedEvent.html "struct bevy::asset::AssetLoadFailedEvent").

## Enums

[AssetEvent](enum.AssetEvent.html "enum bevy::asset::AssetEvent")

[`Message`](../prelude/trait.Message.html "trait bevy::prelude::Message")s that occur for a specific loaded [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), such as “value changed” events and “dependency” events.

[AssetId](enum.AssetId.html "enum bevy::asset::AssetId")

A unique runtime-only identifier for an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset"). This is cheap to [`Copy`](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy")/[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") and is not directly tied to the lifetime of the Asset. This means it _can_ point to an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") that no longer exists.

[AssetLoadError](enum.AssetLoadError.html "enum bevy::asset::AssetLoadError")

An error that occurs during an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") load.

[AssetMetaCheck](enum.AssetMetaCheck.html "enum bevy::asset::AssetMetaCheck")

Configures how / if meta files will be checked. If an asset’s meta file is not checked, the default meta for the asset will be used.

[AssetMode](enum.AssetMode.html "enum bevy::asset::AssetMode")

Controls whether or not assets are pre-processed before being loaded.

[AssetServerMode](enum.AssetServerMode.html "enum bevy::asset::AssetServerMode")

The “asset mode” the server is currently in.

[DependencyLoadState](enum.DependencyLoadState.html "enum bevy::asset::DependencyLoadState")

The load state of an asset’s dependencies.

[DeserializeMetaError](enum.DeserializeMetaError.html "enum bevy::asset::DeserializeMetaError")

An error that occurs while deserializing [`AssetMeta`](meta/struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta").

[EphemeralHandleBehavior](enum.EphemeralHandleBehavior.html "enum bevy::asset::EphemeralHandleBehavior")

Specifies the action that will be taken when attempting to serialize an ephemeral handle.

[Handle](enum.Handle.html "enum bevy::asset::Handle")

A handle to a specific [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") of type `A`. Handles act as abstract “references” to assets, whose data are stored in the [`Assets<A>`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") resource, avoiding the need to store multiple copies of the same data.

[HandleReference](enum.HandleReference.html "enum bevy::asset::HandleReference")

The “stable” data of a handle that can be serialized and deserialized.

[HandleTemplate](enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")

A [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that produces a [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle").

[InvalidGenerationError](enum.InvalidGenerationError.html "enum bevy::asset::InvalidGenerationError")

An error returned when an [`AssetIndex`](struct.AssetIndex.html "struct bevy::asset::AssetIndex") has an invalid generation.

[LoadDirectError](enum.LoadDirectError.html "enum bevy::asset::LoadDirectError")

An error that occurs when attempting an async load using [`NestedLoadBuilder`](struct.NestedLoadBuilder.html "struct bevy::asset::NestedLoadBuilder").

[LoadState](enum.LoadState.html "enum bevy::asset::LoadState")

The load state of an asset.

[ParseAssetPathError](enum.ParseAssetPathError.html "enum bevy::asset::ParseAssetPathError")

An error that occurs when parsing a string type to create an [`AssetPath`](struct.AssetPath.html "struct bevy::asset::AssetPath") fails, such as during [`AssetPath::parse`](struct.AssetPath.html#method.parse "associated function bevy::asset::AssetPath::parse").

[ReadAssetBytesError](enum.ReadAssetBytesError.html "enum bevy::asset::ReadAssetBytesError")

An error produced when calling [`LoadContext::read_asset_bytes`](struct.LoadContext.html#method.read_asset_bytes "method bevy::asset::LoadContext::read_asset_bytes")

[RecursiveDependencyLoadState](enum.RecursiveDependencyLoadState.html "enum bevy::asset::RecursiveDependencyLoadState")

The recursive load state of an asset’s dependencies.

[UnapprovedPathMode](enum.UnapprovedPathMode.html "enum bevy::asset::UnapprovedPathMode")

Determines how to react to attempts to load assets not inside the approved folders.

[UntypedAssetConversionError](enum.UntypedAssetConversionError.html "enum bevy::asset::UntypedAssetConversionError")

Errors preventing the conversion of to/from an [`UntypedHandle`](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle") and a [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle").

[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")

An “untyped” / “generic-less” [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") identifier that behaves much like [`AssetId`](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId"), but stores the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") type information at runtime instead of compile-time. This increases the size of the type, but it enables storing asset ids across asset types together and enables comparisons between them.

[UntypedAssetIdConversionError](enum.UntypedAssetIdConversionError.html "enum bevy::asset::UntypedAssetIdConversionError")

Errors preventing the conversion of to/from an [`UntypedAssetId`](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId") and an [`AssetId`](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId").

[UntypedHandle](enum.UntypedHandle.html "enum bevy::asset::UntypedHandle")

An untyped variant of [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle"), which internally stores the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") type information at runtime as a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") instead of encoding it in the compile-time type. This allows handles across [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") types to be stored together and compared.

[WaitForAssetError](enum.WaitForAssetError.html "enum bevy::asset::WaitForAssetError")

An error when attempting to wait asynchronously for an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") to load.

[WriteDefaultMetaError](enum.WriteDefaultMetaError.html "enum bevy::asset::WriteDefaultMetaError")

## Traits

[AsAssetId](trait.AsAssetId.html "trait bevy::asset::AsAssetId")

A trait for components that can be used as asset identifiers, e.g. handle wrappers.

[Asset](trait.Asset.html "trait bevy::asset::Asset")

Declares that this type is an asset, which can be loaded and managed by the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") and stored in [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") collections.

[AssetApp](trait.AssetApp.html "trait bevy::asset::AssetApp")

Adds asset-related builder methods to [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

[AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader")

Loads an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") from a given byte [`Reader`](io/trait.Reader.html "trait bevy::asset::io::Reader"). This can accept [`AssetLoader::Settings`](trait.AssetLoader.html#associatedtype.Settings "associated type bevy::asset::AssetLoader::Settings"), which configure how the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") should be loaded.

[AsyncReadExt](trait.AsyncReadExt.html "trait bevy::asset::AsyncReadExt")

Extension trait for [`AsyncRead`](../tasks/futures_lite/trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead").

[AsyncSeekExt](trait.AsyncSeekExt.html "trait bevy::asset::AsyncSeekExt")

Extension trait for [`AsyncSeek`](../tasks/futures_lite/trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek").

[AsyncWriteExt](trait.AsyncWriteExt.html "trait bevy::asset::AsyncWriteExt")

Extension trait for [`AsyncWrite`](../tasks/futures_lite/trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite").

[DirectAssetAccessExt](trait.DirectAssetAccessExt.html "trait bevy::asset::DirectAssetAccessExt")

An extension trait for methods for working with assets directly from a [`World`](../prelude/struct.World.html "struct bevy::prelude::World").

[ErasedAssetLoader](trait.ErasedAssetLoader.html "trait bevy::asset::ErasedAssetLoader")

Provides type-erased access to an [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

[LoadFromPath](trait.LoadFromPath.html "trait bevy::asset::LoadFromPath")

A trait for loading an asset.

[VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies")

This trait defines how to visit the dependencies of an asset. For example, a 3D model might require both textures and meshes to be loaded.

## Functions

[asset\_value](fn.asset_value.html "fn bevy::asset::asset_value")

This will create a new [`HandleTemplate`](enum.HandleTemplate.html "enum bevy::asset::HandleTemplate") for the given `asset` value. This makes it possible to define assets “inline” in templates / scenes that produce a [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle").

[handle\_internal\_asset\_events](fn.handle_internal_asset_events.html "fn bevy::asset::handle_internal_asset_events")

A system that manages internal [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") events, such as finalizing asset loads.

[publish\_asset\_server\_diagnostics](fn.publish_asset_server_diagnostics.html "fn bevy::asset::publish_asset_server_diagnostics")

A system publishing asset server statistics to [`bevy_diagnostic`](../diagnostic/index.html "mod bevy::diagnostic").

## Derive Macros

[Asset](derive.Asset.html "derive bevy::asset::Asset")

Implement the `Asset` trait.

[VisitAssetDependencies](derive.VisitAssetDependencies.html "derive bevy::asset::VisitAssetDependencies")

Implement the `VisitAssetDependencies` trait.