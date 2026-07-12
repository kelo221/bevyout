[bevy](../../index.html)::[asset](../index.html)::[meta](index.html)

# Trait AssetMetaDyn 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#128)

```rust
pub trait AssetMetaDyn:
    Downcast
    + Send
    + Sync {
    // Required methods
    fn loader_settings(&self) -> Option<&(dyn Settings + 'static)>;
    fn loader_settings_mut(&mut self) -> Option<&mut (dyn Settings + 'static)>;
    fn process_settings(&self) -> Option<&(dyn Settings + 'static)>;
    fn serialize(&self) -> Vec<u8> ⓘ;
    fn processed_info(&self) -> &Option<ProcessedInfo>;
    fn processed_info_mut(&mut self) -> &mut Option<ProcessedInfo>;
}
```

A dynamic type-erased counterpart to [`AssetMeta`](struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta") that enables passing around and interacting with [`AssetMeta`](struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta") without knowing its type.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#130)

#### fn [loader\_settings](#tymethod.loader_settings)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Settings](trait.Settings.html "trait bevy::asset::meta::Settings") + 'static)>

Returns a reference to the [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader") settings, if they exist.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#132)

#### fn [loader\_settings\_mut](#tymethod.loader_settings_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Settings](trait.Settings.html "trait bevy::asset::meta::Settings") + 'static)>

Returns a mutable reference to the [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader") settings, if they exist.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#134)

#### fn [process\_settings](#tymethod.process_settings)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Settings](trait.Settings.html "trait bevy::asset::meta::Settings") + 'static)>

Returns a reference to the [`Process`](../processor/trait.Process.html "trait bevy::asset::processor::Process") settings, if they exist.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#136)

#### fn [serialize](#tymethod.serialize)(&self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> [ⓘ](#)

Serializes the internal [`AssetMeta`](struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#138)

#### fn [processed\_info](#tymethod.processed_info)(&self) -> &[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ProcessedInfo](struct.ProcessedInfo.html "struct bevy::asset::meta::ProcessedInfo")\>

Returns a reference to the [`ProcessedInfo`](struct.ProcessedInfo.html "struct bevy::asset::meta::ProcessedInfo") if it exists.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#140)

#### fn [processed\_info\_mut](#tymethod.processed_info_mut)(&mut self) -> &mut [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ProcessedInfo](struct.ProcessedInfo.html "struct bevy::asset::meta::ProcessedInfo")\>

Returns a mutable reference to the [`ProcessedInfo`](struct.ProcessedInfo.html "struct bevy::asset::meta::ProcessedInfo") if it exists.

## Implementations

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#183)

### impl dyn [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#183)

#### pub fn [is](#method.is)<\_\_T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where \_\_T: [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn"),

Returns true if the trait object wraps an object of type `__T`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#183)

#### pub fn [downcast](#method.downcast)<\_\_T>( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<\_\_T>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")\>>

where \_\_T: [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn"),

Returns a boxed object from a boxed trait object if the underlying object is of type `__T`. Returns the original boxed trait if it isn’t.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#183)

#### pub fn [downcast\_rc](#method.downcast_rc)<\_\_T>( self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<\_\_T>, [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")\>>

where \_\_T: [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn"),

Returns an `Rc`\-ed object from an `Rc`\-ed trait object if the underlying object is of type `__T`. Returns the original `Rc`\-ed trait if it isn’t.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#183)

#### pub fn [downcast\_ref](#method.downcast_ref)<\_\_T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&\_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn"),

Returns a reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#183)

#### pub fn [downcast\_mut](#method.downcast_mut)<\_\_T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut \_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn"),

Returns a mutable reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#143)

### impl<L, P> [AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn") for [AssetMeta](struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta")<L, P>

where L: [AssetLoader](../trait.AssetLoader.html "trait bevy::asset::AssetLoader"), P: [Process](../processor/trait.Process.html "trait bevy::asset::processor::Process"),

{"Vec<u8>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../prelude/struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../prelude/struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div>"}