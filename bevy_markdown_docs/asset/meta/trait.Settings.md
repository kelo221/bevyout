[bevy](../../index.html)::[asset](../index.html)::[meta](index.html)

# Trait Settings 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#188)

```rust
pub trait Settings:
    Downcast
    + Send
    + Sync
    + 'static { }
```

Settings used by the asset system, such as by [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader"), [`Process`](../processor/trait.Process.html "trait bevy::asset::processor::Process"), and [`AssetSaver`](../saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver")

## Implementations

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#192)

### impl dyn [Settings](trait.Settings.html "trait bevy::asset::meta::Settings")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#192)

#### pub fn [is](#method.is)<\_\_T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where \_\_T: [Settings](trait.Settings.html "trait bevy::asset::meta::Settings"),

Returns true if the trait object wraps an object of type `__T`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#192)

#### pub fn [downcast](#method.downcast)<\_\_T>( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Settings](trait.Settings.html "trait bevy::asset::meta::Settings")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<\_\_T>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Settings](trait.Settings.html "trait bevy::asset::meta::Settings")\>>

where \_\_T: [Settings](trait.Settings.html "trait bevy::asset::meta::Settings"),

Returns a boxed object from a boxed trait object if the underlying object is of type `__T`. Returns the original boxed trait if it isn’t.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#192)

#### pub fn [downcast\_rc](#method.downcast_rc)<\_\_T>( self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Settings](trait.Settings.html "trait bevy::asset::meta::Settings")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<\_\_T>, [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Settings](trait.Settings.html "trait bevy::asset::meta::Settings")\>>

where \_\_T: [Settings](trait.Settings.html "trait bevy::asset::meta::Settings"),

Returns an `Rc`\-ed object from an `Rc`\-ed trait object if the underlying object is of type `__T`. Returns the original `Rc`\-ed trait if it isn’t.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#192)

#### pub fn [downcast\_ref](#method.downcast_ref)<\_\_T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&\_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [Settings](trait.Settings.html "trait bevy::asset::meta::Settings"),

Returns a reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#192)

#### pub fn [downcast\_mut](#method.downcast_mut)<\_\_T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut \_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [Settings](trait.Settings.html "trait bevy::asset::meta::Settings"),

Returns a mutable reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),