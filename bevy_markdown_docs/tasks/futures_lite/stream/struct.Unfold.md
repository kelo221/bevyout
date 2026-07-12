[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Struct Unfold 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#466-476)

```rust
pub struct Unfold<T, F, Fut> { /* private fields */ }
```

Stream for the [`unfold()`](fn.unfold.html "fn bevy::tasks::futures_lite::stream::unfold") function.

## Trait Implementations

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#468)

### impl<T, F, Fut> [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), F: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), Fut: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#468)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#478-481)

### impl<T, F, Fut> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where T: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"), Fut: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#483)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#491-494)

### impl<T, F, Fut, Item> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(Item, T)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#496)

#### type [Item](../trait.Stream.html#associatedtype.Item) = Item

Values yielded by the stream.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#498)

#### fn [poll\_next](../trait.Stream.html#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. [Read more](../trait.Stream.html#tymethod.poll_next)

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#105)

#### fn [size\_hint](../trait.Stream.html#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

Returns the bounds on the remaining length of the stream. [Read more](../trait.Stream.html#method.size_hint)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#466-476)

### impl<'\_\_pin, T, F, Fut> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where <PinnedFieldsOfHelperStruct<\_\_Origin<'\_\_pin, T, F, Fut>> as PinnedFieldsOfHelperTrait>::Actual: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

## Auto Trait Implementations

### impl<T, F, Fut> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where F: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"), T: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"), Fut: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"),

### impl<T, F, Fut> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where F: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"), T: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"), Fut: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<T, F, Fut> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where F: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"), T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"), Fut: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

### impl<T, F, Fut> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where F: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), Fut: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

### impl<T, F, Fut> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where F: [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"), T: [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"), Fut: [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"),

### impl<T, F, Fut> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where F: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"), T: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"), Fut: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2068)

### impl<S> [StreamExt](../trait.StreamExt.html "trait bevy::tasks::futures_lite::StreamExt") for S

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#836-838)

#### fn [poll\_next](../trait.StreamExt.html#method.poll_next)(&mut self, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

A convenience for calling [`Stream::poll_next()`](../trait.Stream.html#tymethod.poll_next "method bevy::tasks::futures_lite::Stream::poll_next") on `!`[`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") types.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#862-864)

#### fn [next](../trait.StreamExt.html#method.next)(&mut self) -> [NextFuture](struct.NextFuture.html "struct bevy::tasks::futures_lite::stream::NextFuture")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Retrieves the next item in the stream. [Read more](../trait.StreamExt.html#method.next)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#890-892)

#### fn [try\_next](../trait.StreamExt.html#method.try_next)<T, E>(&mut self) -> [TryNextFuture](struct.TryNextFuture.html "struct bevy::tasks::futures_lite::stream::TryNextFuture")<'\_, Self> [ⓘ](#)

where Self: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Retrieves the next item in the stream. [Read more](../trait.StreamExt.html#method.try_next)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#912-914)

#### fn [count](../trait.StreamExt.html#method.count)(self) -> [CountFuture](struct.CountFuture.html "struct bevy::tasks::futures_lite::stream::CountFuture")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Counts the number of items in the stream. [Read more](../trait.StreamExt.html#method.count)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#939-942)

#### fn [map](../trait.StreamExt.html#method.map)<T, F>(self, f: F) -> [Map](struct.Map.html "struct bevy::tasks::futures_lite::stream::Map")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> T,

Maps items of the stream to new values using a closure. [Read more](../trait.StreamExt.html#method.map)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#965-969)

#### fn [flat\_map](../trait.StreamExt.html#method.flat_map)<U, F>(self, f: F) -> [FlatMap](struct.FlatMap.html "struct bevy::tasks::futures_lite::stream::FlatMap")<Self, U, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> U,

Maps items to streams and then concatenates them. [Read more](../trait.StreamExt.html#method.flat_map)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#993-996)

#### fn [flatten](../trait.StreamExt.html#method.flatten)(self) -> [Flatten](struct.Flatten.html "struct bevy::tasks::futures_lite::stream::Flatten")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

Concatenates inner streams. [Read more](../trait.StreamExt.html#method.flatten)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1023-1027)

#### fn [then](../trait.StreamExt.html#method.then)<F, Fut>(self, f: F) -> [Then](struct.Then.html "struct bevy::tasks::futures_lite::stream::Then")<Self, F, Fut>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

Maps items of the stream to new values using an async closure. [Read more](../trait.StreamExt.html#method.then)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1052-1055)

#### fn [filter](../trait.StreamExt.html#method.filter)<P>(self, predicate: P) -> [Filter](struct.Filter.html "struct bevy::tasks::futures_lite::stream::Filter")<Self, P>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Keeps items of the stream for which `predicate` returns `true`. [Read more](../trait.StreamExt.html#method.filter)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1080-1083)

#### fn [filter\_map](../trait.StreamExt.html#method.filter_map)<T, F>(self, f: F) -> [FilterMap](struct.FilterMap.html "struct bevy::tasks::futures_lite::stream::FilterMap")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>,

Filters and maps items of the stream using a closure. [Read more](../trait.StreamExt.html#method.filter_map)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1103-1105)

#### fn [take](../trait.StreamExt.html#method.take)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Take](struct.Take.html "struct bevy::tasks::futures_lite::stream::Take")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Takes only the first `n` items of the stream. [Read more](../trait.StreamExt.html#method.take)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1126-1129)

#### fn [take\_while](../trait.StreamExt.html#method.take_while)<P>(self, predicate: P) -> [TakeWhile](struct.TakeWhile.html "struct bevy::tasks::futures_lite::stream::TakeWhile")<Self, P>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Takes items while `predicate` returns `true`. [Read more](../trait.StreamExt.html#method.take_while)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1161-1164)

#### fn [map\_while](../trait.StreamExt.html#method.map_while)<B, P>(self, predicate: P) -> [MapWhile](struct.MapWhile.html "struct bevy::tasks::futures_lite::stream::MapWhile")<Self, P>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Maps items while `predicate` returns [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some"). [Read more](../trait.StreamExt.html#method.map_while)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1187-1189)

#### fn [skip](../trait.StreamExt.html#method.skip)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Skip](struct.Skip.html "struct bevy::tasks::futures_lite::stream::Skip")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Skips the first `n` items of the stream. [Read more](../trait.StreamExt.html#method.skip)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1210-1213)

#### fn [skip\_while](../trait.StreamExt.html#method.skip_while)<P>(self, predicate: P) -> [SkipWhile](struct.SkipWhile.html "struct bevy::tasks::futures_lite::stream::SkipWhile")<Self, P>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Skips items while `predicate` returns `true`. [Read more](../trait.StreamExt.html#method.skip_while)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1242-1244)

#### fn [step\_by](../trait.StreamExt.html#method.step_by)(self, step: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [StepBy](struct.StepBy.html "struct bevy::tasks::futures_lite::stream::StepBy")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Yields every `step`th item. [Read more](../trait.StreamExt.html#method.step_by)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1273-1276)

#### fn [chain](../trait.StreamExt.html#method.chain)<U>(self, other: U) -> [Chain](struct.Chain.html "struct bevy::tasks::futures_lite::stream::Chain")<Self, U>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

Appends another stream to the end of this one. [Read more](../trait.StreamExt.html#method.chain)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1300-1303)

#### fn [cloned](../trait.StreamExt.html#method.cloned)<'a, T>(self) -> [Cloned](struct.Cloned.html "struct bevy::tasks::futures_lite::stream::Cloned")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + 'a,

Clones all items. [Read more](../trait.StreamExt.html#method.cloned)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1324-1327)

#### fn [copied](../trait.StreamExt.html#method.copied)<'a, T>(self) -> [Copied](struct.Copied.html "struct bevy::tasks::futures_lite::stream::Copied")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + 'a,

Copies all items. [Read more](../trait.StreamExt.html#method.copied)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1346-1349)

#### fn [collect](../trait.StreamExt.html#method.collect)<C>(self) -> [CollectFuture](struct.CollectFuture.html "struct bevy::tasks::futures_lite::stream::CollectFuture")<Self, C> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

Collects all items in the stream into a collection. [Read more](../trait.StreamExt.html#method.collect)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1372-1375)

#### fn [try\_collect](../trait.StreamExt.html#method.try_collect)<T, E, C>(self) -> [TryCollectFuture](struct.TryCollectFuture.html "struct bevy::tasks::futures_lite::stream::TryCollectFuture")<Self, C> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<T>,

Collects all items in the fallible stream into a collection. [Read more](../trait.StreamExt.html#method.try_collect)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1399-1403)

#### fn [partition](../trait.StreamExt.html#method.partition)<B, P>(self, predicate: P) -> [PartitionFuture](struct.PartitionFuture.html "struct bevy::tasks::futures_lite::stream::PartitionFuture")<Self, P, B> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>, P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Partitions items into those for which `predicate` is `true` and those for which it is `false`, and then collects them into two collections. [Read more](../trait.StreamExt.html#method.partition)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1429-1432)

#### fn [fold](../trait.StreamExt.html#method.fold)<T, F>(self, init: T, f: F) -> [FoldFuture](struct.FoldFuture.html "struct bevy::tasks::futures_lite::stream::FoldFuture")<Self, F, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T, Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> T,

Accumulates a computation over the stream. [Read more](../trait.StreamExt.html#method.fold)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1467-1470)

#### fn [try\_fold](../trait.StreamExt.html#method.try_fold)<T, E, F, B>( &mut self, init: B, f: F, ) -> [TryFoldFuture](struct.TryFoldFuture.html "struct bevy::tasks::futures_lite::stream::TryFoldFuture")<'\_, Self, F, B> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, T) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<B, E>,

Accumulates a fallible computation over the stream. [Read more](../trait.StreamExt.html#method.try_fold)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1502-1505)

#### fn [scan](../trait.StreamExt.html#method.scan)<St, B, F>(self, initial\_state: St, f: F) -> [Scan](struct.Scan.html "struct bevy::tasks::futures_lite::stream::Scan")<Self, St, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut St](https://doc.rust-lang.org/nightly/std/primitive.reference.html), Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Maps items of the stream to new values using a state value and a closure. [Read more](../trait.StreamExt.html#method.scan)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1528-1530)

#### fn [fuse](../trait.StreamExt.html#method.fuse)(self) -> [Fuse](struct.Fuse.html "struct bevy::tasks::futures_lite::stream::Fuse")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fuses the stream so that it stops yielding items after the first [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). [Read more](../trait.StreamExt.html#method.fuse)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1554-1556)

#### fn [cycle](../trait.StreamExt.html#method.cycle)(self) -> [Cycle](struct.Cycle.html "struct bevy::tasks::futures_lite::stream::Cycle")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Repeats the stream from beginning to end, forever. [Read more](../trait.StreamExt.html#method.cycle)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1581-1583)

#### fn [enumerate](../trait.StreamExt.html#method.enumerate)(self) -> [Enumerate](struct.Enumerate.html "struct bevy::tasks::futures_lite::stream::Enumerate")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Enumerates items, mapping them to `(index, item)`. [Read more](../trait.StreamExt.html#method.enumerate)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1606-1609)

#### fn [inspect](../trait.StreamExt.html#method.inspect)<F>(self, f: F) -> [Inspect](struct.Inspect.html "struct bevy::tasks::futures_lite::stream::Inspect")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")),

Calls a closure on each item and passes it on. [Read more](../trait.StreamExt.html#method.inspect)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1631-1633)

#### fn [nth](../trait.StreamExt.html#method.nth)(&mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [NthFuture](struct.NthFuture.html "struct bevy::tasks::futures_lite::stream::NthFuture")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Gets the `n`th item of the stream. [Read more](../trait.StreamExt.html#method.nth)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1653-1655)

#### fn [last](../trait.StreamExt.html#method.last)(self) -> [LastFuture](struct.LastFuture.html "struct bevy::tasks::futures_lite::stream::LastFuture")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns the last item in the stream. [Read more](../trait.StreamExt.html#method.last)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1677-1680)

#### fn [find](../trait.StreamExt.html#method.find)<P>(&mut self, predicate: P) -> [FindFuture](struct.FindFuture.html "struct bevy::tasks::futures_lite::stream::FindFuture")<'\_, Self, P> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Finds the first item of the stream for which `predicate` returns `true`. [Read more](../trait.StreamExt.html#method.find)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1702-1705)

#### fn [find\_map](../trait.StreamExt.html#method.find_map)<F, B>(&mut self, f: F) -> [FindMapFuture](struct.FindMapFuture.html "struct bevy::tasks::futures_lite::stream::FindMapFuture")<'\_, Self, F> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Applies a closure to items in the stream and returns the first [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some") result. [Read more](../trait.StreamExt.html#method.find_map)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1725-1728)

#### fn [position](../trait.StreamExt.html#method.position)<P>(&mut self, predicate: P) -> [PositionFuture](struct.PositionFuture.html "struct bevy::tasks::futures_lite::stream::PositionFuture")<'\_, Self, P> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Finds the index of the first item of the stream for which `predicate` returns `true`. [Read more](../trait.StreamExt.html#method.position)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1757-1760)

#### fn [all](../trait.StreamExt.html#method.all)<P>(&mut self, predicate: P) -> [AllFuture](struct.AllFuture.html "struct bevy::tasks::futures_lite::stream::AllFuture")<'\_, Self, P> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Tests if `predicate` returns `true` for all items in the stream. [Read more](../trait.StreamExt.html#method.all)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1788-1791)

#### fn [any](../trait.StreamExt.html#method.any)<P>(&mut self, predicate: P) -> [AnyFuture](struct.AnyFuture.html "struct bevy::tasks::futures_lite::stream::AnyFuture")<'\_, Self, P> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Tests if `predicate` returns `true` for any item in the stream. [Read more](../trait.StreamExt.html#method.any)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1811-1814)

#### fn [for\_each](../trait.StreamExt.html#method.for_each)<F>(self, f: F) -> [ForEachFuture](struct.ForEachFuture.html "struct bevy::tasks::futures_lite::stream::ForEachFuture")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")),

Calls a closure on each item of the stream. [Read more](../trait.StreamExt.html#method.for_each)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1845-1848)

#### fn [try\_for\_each](../trait.StreamExt.html#method.try_for_each)<F, E>(&mut self, f: F) -> [TryForEachFuture](struct.TryForEachFuture.html "struct bevy::tasks::futures_lite::stream::TryForEachFuture")<'\_, Self, F> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), E>,

Calls a fallible closure on each item of the stream, stopping on first error. [Read more](../trait.StreamExt.html#method.try_for_each)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1873-1876)

#### fn [zip](../trait.StreamExt.html#method.zip)<U>(self, other: U) -> [Zip](struct.Zip.html "struct bevy::tasks::futures_lite::stream::Zip")<Self, U>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

Zips up two streams into a single stream of pairs. [Read more](../trait.StreamExt.html#method.zip)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1900-1904)

#### fn [unzip](../trait.StreamExt.html#method.unzip)<A, B, FromA, FromB>(self) -> [UnzipFuture](struct.UnzipFuture.html "struct bevy::tasks::futures_lite::stream::UnzipFuture")<Self, FromA, FromB> [ⓘ](#)

where FromA: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<A>, FromB: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<B>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

Collects a stream of pairs into a pair of collections. [Read more](../trait.StreamExt.html#method.unzip)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1928-1931)

#### fn [or](../trait.StreamExt.html#method.or)<S>(self, other: S) -> [Or](struct.Or.html "struct bevy::tasks::futures_lite::stream::Or")<Self, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

Merges with `other` stream, preferring items from `self` whenever both streams are ready. [Read more](../trait.StreamExt.html#method.or)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1956-1959)

#### fn [race](../trait.StreamExt.html#method.race)<S>(self, other: S) -> [Race](struct.Race.html "struct bevy::tasks::futures_lite::stream::Race")<Self, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

Available on **crate features `race` and `std`** only.

Merges with `other` stream, with no preference for either stream when both are ready. [Read more](../trait.StreamExt.html#method.race)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2015)

#### fn [drain](../trait.StreamExt.html#method.drain)(&mut self) -> [Drain](struct.Drain.html "struct bevy::tasks::futures_lite::stream::Drain")<'\_, Self>

Yields all immediately available values from a stream. [Read more](../trait.StreamExt.html#method.drain)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2036-2038)

#### fn [boxed](../trait.StreamExt.html#method.boxed)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a,

Available on **crate feature `alloc`** only.

Boxes the stream and changes its type to `dyn Stream + Send + 'a`. [Read more](../trait.StreamExt.html#method.boxed)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2060-2062)

#### fn [boxed\_local](../trait.StreamExt.html#method.boxed_local)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\> + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + 'a,

Available on **crate feature `alloc`** only.

Boxes the stream and changes its type to `dyn Stream + 'a`. [Read more](../trait.StreamExt.html#method.boxed_local)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#245)

### impl<T> [StreamExt](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html "trait futures_util::stream::stream::StreamExt") for T

where T: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#273-275)

#### fn [next](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.next)(&mut self) -> [Next](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/next/struct.Next.html "struct futures_util::stream::stream::next::Next")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Creates a future that resolves to the next item in the stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.next)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#308-310)

#### fn [into\_future](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.into_future)(self) -> [StreamFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/into_future/struct.StreamFuture.html "struct futures_util::stream::stream::into_future::StreamFuture")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Converts this stream into a future of `(next_item, tail_of_stream)`. If the stream terminates, then the next item is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.into_future)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#341-344)

#### fn [map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.map)<T, F>(self, f: F) -> [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<Self, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> T, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Maps this stream’s items to a different type, returning a new stream of the resulting type. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.map)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#387-389)

#### fn [enumerate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.enumerate)(self) -> [Enumerate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/enumerate/struct.Enumerate.html "struct futures_util::stream::stream::enumerate::Enumerate")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates a stream which gives the current iteration count as well as the next value. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.enumerate)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#420-424)

#### fn [filter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.filter)<Fut, F>(self, f: F) -> [Filter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/filter/struct.Filter.html "struct futures_util::stream::stream::filter::Filter")<Self, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Filters the values produced by this stream according to the provided asynchronous predicate. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.filter)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#454-458)

#### fn [filter\_map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.filter_map)<Fut, T, F>(self, f: F) -> [FilterMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/filter_map/struct.FilterMap.html "struct futures_util::stream::stream::filter_map::FilterMap")<Self, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Filters the values produced by this stream while simultaneously mapping them to a different type according to the provided asynchronous closure. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.filter_map)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#488-492)

#### fn [then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.then)<Fut, F>(self, f: F) -> [Then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/then/struct.Then.html "struct futures_util::stream::stream::then::Then")<Self, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Computes from this stream’s items new items of a different type using an asynchronous closure. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.then)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#522-524)

#### fn [collect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.collect)<C>(self) -> [Collect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/collect/struct.Collect.html "struct futures_util::stream::stream::collect::Collect")<Self, C> [ⓘ](#)

where C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Transforms a stream into a collection, returning a future representing the result of that computation. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.collect)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#559-563)

#### fn [unzip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.unzip)<A, B, FromA, FromB>(self) -> [Unzip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/unzip/struct.Unzip.html "struct futures_util::stream::stream::unzip::Unzip")<Self, FromA, FromB> [ⓘ](#)

where FromA: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<A>, FromB: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<B>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

Converts a stream of pairs into a future, which resolves to pair of containers. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.unzip)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#600-603)

#### fn [concat](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.concat)(self) -> [Concat](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/concat/struct.Concat.html "struct futures_util::stream::stream::concat::Concat")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\> + [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Concatenate all items of a stream into a single extendable destination, returning a future representing the end result. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.concat)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#633-635)

#### fn [count](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.count)(self) -> [Count](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/count/struct.Count.html "struct futures_util::stream::stream::count::Count")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Drives the stream to completion, counting the number of items. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.count)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#663-665)

#### fn [cycle](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.cycle)(self) -> [Cycle](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/cycle/struct.Cycle.html "struct futures_util::stream::stream::cycle::Cycle")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Repeats a stream endlessly. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.cycle)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#690-694)

#### fn [fold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.fold)<T, Fut, F>(self, init: T, f: F) -> [Fold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/fold/struct.Fold.html "struct futures_util::stream::stream::fold::Fold")<Self, Fut, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T, Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Execute an accumulating asynchronous computation over a stream, collecting all the values into one final result. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.fold)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#712-716)

#### fn [any](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.any)<Fut, F>(self, f: F) -> [Any](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/any/struct.Any.html "struct futures_util::stream::stream::any::Any")<Self, Fut, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Execute predicate over asynchronous stream, and return `true` if any element in stream satisfied a predicate. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.any)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#734-738)

#### fn [all](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.all)<Fut, F>(self, f: F) -> [All](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/all/struct.All.html "struct futures_util::stream::stream::all::All")<Self, Fut, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Execute predicate over asynchronous stream, and return `true` if all element in stream satisfied a predicate. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.all)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#774-777)

#### fn [flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.flatten)(self) -> [Flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Flatten.html "struct futures_util::stream::stream::Flatten")<Self>

where Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Flattens a stream of streams into just one continuous stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.flatten)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#825-828)

#### fn [flatten\_unordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.flatten_unordered)( self, limit: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>, ) -> FlattenUnorderedWithFlowController<Self, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

where Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

Flattens a stream of streams into just one continuous stream. Polls inner streams produced by the base stream concurrently. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.flatten_unordered)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#858-862)

#### fn [flat\_map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.flat_map)<U, F>(self, f: F) -> [FlatMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.FlatMap.html "struct futures_util::stream::stream::FlatMap")<Self, U, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> U, U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Maps a stream like [`StreamExt::map`](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.map "method futures_util::stream::stream::StreamExt::map") but flattens nested `Stream`s. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.flat_map)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#907-915)

#### fn [flat\_map\_unordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.flat_map_unordered)<U, F>( self, limit: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>, f: F, ) -> [FlatMapUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.FlatMapUnordered.html "struct futures_util::stream::stream::FlatMapUnordered")<Self, U, F>

where U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> U, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

Maps a stream like [`StreamExt::map`](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.map "method futures_util::stream::stream::StreamExt::map") but flattens nested `Stream`s and polls them concurrently, yielding items in any order, as they made available. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.flat_map_unordered)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#944-948)

#### fn [scan](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.scan)<S, B, Fut, F>(self, initial\_state: S, f: F) -> [Scan](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/scan/struct.Scan.html "struct futures_util::stream::stream::scan::Scan")<Self, S, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html), Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Combinator similar to [`StreamExt::fold`](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.fold "method futures_util::stream::stream::StreamExt::fold") that holds internal state and produces a new stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.scan)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#975-979)

#### fn [skip\_while](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.skip_while)<Fut, F>(self, f: F) -> [SkipWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/skip_while/struct.SkipWhile.html "struct futures_util::stream::stream::skip_while::SkipWhile")<Self, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Skip elements on this stream while the provided asynchronous predicate resolves to `true`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.skip_while)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1005-1009)

#### fn [take\_while](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.take_while)<Fut, F>(self, f: F) -> [TakeWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/take_while/struct.TakeWhile.html "struct futures_util::stream::stream::take_while::TakeWhile")<Self, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Take elements from this stream while the provided asynchronous predicate resolves to `true`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.take_while)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1050-1053)

#### fn [take\_until](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.take_until)<Fut>(self, fut: Fut) -> [TakeUntil](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/take_until/struct.TakeUntil.html "struct futures_util::stream::stream::take_until::TakeUntil")<Self, Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Take elements from this stream until the provided future resolves. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.take_until)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1091-1095)

#### fn [for\_each](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.for_each)<Fut, F>(self, f: F) -> [ForEach](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/for_each/struct.ForEach.html "struct futures_util::stream::stream::for_each::ForEach")<Self, Fut, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Runs this stream to completion, executing the provided asynchronous closure for each element on the stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.for_each)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1147-1155)

#### fn [for\_each\_concurrent](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.for_each_concurrent)<Fut, F>( self, limit: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>, f: F, ) -> [ForEachConcurrent](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/for_each_concurrent/struct.ForEachConcurrent.html "struct futures_util::stream::stream::for_each_concurrent::ForEachConcurrent")<Self, Fut, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

Runs this stream to completion, executing the provided asynchronous closure for each element on the stream concurrently as elements become available. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.for_each_concurrent)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1176-1178)

#### fn [take](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.take)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Take](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/take/struct.Take.html "struct futures_util::stream::stream::take::Take")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates a new stream of at most `n` items of the underlying stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.take)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1199-1201)

#### fn [skip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.skip)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Skip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/skip/struct.Skip.html "struct futures_util::stream::stream::skip::Skip")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates a new stream which skips `n` items of the underlying stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.skip)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1245-1247)

#### fn [fuse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.fuse)(self) -> [Fuse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/fuse/struct.Fuse.html "struct futures_util::stream::stream::fuse::Fuse")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fuse a stream such that [`poll_next`](../trait.Stream.html#tymethod.poll_next "method bevy::tasks::futures_lite::Stream::poll_next") will never again be called once it has finished. This method can be used to turn any `Stream` into a `FusedStream`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.fuse)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1278)

#### fn [by\_ref](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.by_ref)(&mut self) -> &mut Self

Borrows a stream, rather than consuming it. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.by_ref)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1323-1325)

#### fn [catch\_unwind](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.catch_unwind)(self) -> [CatchUnwind](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/catch_unwind/struct.CatchUnwind.html "struct futures_util::stream::stream::catch_unwind::CatchUnwind")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

Available on **crate feature `std`** only.

Catches unwinding panics while polling the stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.catch_unwind)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1335-1337)

#### fn [boxed](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.boxed)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a,

Available on **crate feature `alloc`** only.

Wrap the stream in a Box, pinning it. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.boxed)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1349-1351)

#### fn [boxed\_local](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.boxed_local)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\> + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + 'a,

Available on **crate feature `alloc`** only.

Wrap the stream in a Box, pinning it. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.boxed_local)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1370-1373)

#### fn [buffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.buffered)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Buffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/buffered/struct.Buffered.html "struct futures_util::stream::stream::buffered::Buffered")<Self>

where Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

An adaptor for creating a buffered list of pending futures. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.buffered)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1415-1418)

#### fn [buffer\_unordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.buffer_unordered)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [BufferUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/buffer_unordered/struct.BufferUnordered.html "struct futures_util::stream::stream::buffer_unordered::BufferUnordered")<Self>

where Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

An adaptor for creating a buffered list of pending futures (unordered). [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.buffer_unordered)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1445-1448)

#### fn [zip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.zip)<St>(self, other: St) -> [Zip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/zip/struct.Zip.html "struct futures_util::stream::stream::zip::Zip")<Self, St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

An adapter for zipping two streams together. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.zip)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1476-1479)

#### fn [chain](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.chain)<St>(self, other: St) -> [Chain](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/chain/struct.Chain.html "struct futures_util::stream::stream::chain::Chain")<Self, St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Adapter for chaining two streams. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.chain)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1487-1489)

#### fn [peekable](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.peekable)(self) -> [Peekable](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/peek/struct.Peekable.html "struct futures_util::stream::stream::peek::Peekable")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates a new stream which exposes a `peek` method. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.peekable)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1513-1515)

#### fn [chunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.chunks)(self, capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Chunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/chunks/struct.Chunks.html "struct futures_util::stream::stream::chunks::Chunks")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

An adaptor for chunking up items of the stream inside a vector. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.chunks)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1538-1540)

#### fn [ready\_chunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.ready_chunks)(self, capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [ReadyChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/ready_chunks/struct.ReadyChunks.html "struct futures_util::stream::stream::ready_chunks::ReadyChunks")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

An adaptor for chunking up ready items of the stream inside a vector. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.ready_chunks)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1598-1601)

#### fn [inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.inspect)<F>(self, f: F) -> [Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Inspect.html "struct futures_util::stream::stream::Inspect")<Self, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Do something with each item of this stream, afterwards passing it on. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.inspect)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1611-1614)

#### fn [left\_stream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.left_stream)<B>(self) -> [Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<Self, B> [ⓘ](#)

where B: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wrap this stream in an `Either` stream, making it the left-hand variant of that `Either`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.left_stream)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1624-1627)

#### fn [right\_stream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.right_stream)<B>(self) -> [Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<B, Self> [ⓘ](#)

where B: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wrap this stream in an `Either` stream, making it the right-hand variant of that `Either`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.right_stream)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1634-1636)

#### fn [poll\_next\_unpin](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.poll_next_unpin)(&mut self, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

A convenience method for calling [`Stream::poll_next`](../trait.Stream.html#tymethod.poll_next "method bevy::tasks::futures_lite::Stream::poll_next") on [`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") stream types.

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#1691-1693)

#### fn [select\_next\_some](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.select_next_some)(&mut self) -> [SelectNextSome](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/select_next_some/struct.SelectNextSome.html "struct futures_util::stream::stream::select_next_some::SelectNextSome")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [FusedStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.FusedStream.html "trait futures_core::stream::FusedStream"),

Returns a [`Future`](../trait.Future.html "trait bevy::tasks::futures_lite::Future") that resolves when the next item in this stream is ready. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/trait.StreamExt.html#method.select_next_some)

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../../../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../../../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../../prelude/trait.ToOwned.html#method.clone_into)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#195-197)

### impl<S, T, E> [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream") for S

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#199)

#### type [Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok) = T

The type of successful values yielded by this future

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#200)

#### type [Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error) = E

The type of failures yielded by this future

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#202-205)

#### fn [try\_poll\_next](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#tymethod.try_poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <S as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>>>

Poll this `TryStream` as if it were a `Stream`. [Read more](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#tymethod.try_poll_next)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#177)

### impl<S> [TryStreamExt](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html "trait futures_util::stream::try_stream::TryStreamExt") for S

where S: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#198-201)

#### fn [err\_into](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.err_into)<E>(self) -> [ErrInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.ErrInto.html "struct futures_util::stream::try_stream::ErrInto")<Self, E>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error"): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<E>,

Wraps the current stream in a new stream which converts the error type into the one provided. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.err_into)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#223-226)

#### fn [map\_ok](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.map_ok)<T, F>(self, f: F) -> [MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapOk.html "struct futures_util::stream::try_stream::MapOk")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> T,

Wraps the current stream in a new stream which maps the success value using the provided closure. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.map_ok)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#248-251)

#### fn [map\_err](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.map_err)<E, F>(self, f: F) -> [MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapErr.html "struct futures_util::stream::try_stream::MapErr")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")) -> E,

Wraps the current stream in a new stream which maps the error value using the provided closure. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.map_err)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#294-298)

#### fn [and\_then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.and_then)<Fut, F>(self, f: F) -> [AndThen](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/and_then/struct.AndThen.html "struct futures_util::stream::try_stream::and_then::AndThen")<Self, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Chain on a computation for when a value is ready, passing the successful results to the provided closure `f`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.and_then)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#320-324)

#### fn [or\_else](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.or_else)<Fut, F>(self, f: F) -> [OrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/or_else/struct.OrElse.html "struct futures_util::stream::try_stream::or_else::OrElse")<Self, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Chain on a computation for when an error happens, passing the erroneous result to the provided closure `f`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.or_else)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#335-338)

#### fn [inspect\_ok](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.inspect_ok)<F>(self, f: F) -> [InspectOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.InspectOk.html "struct futures_util::stream::try_stream::InspectOk")<Self, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Do something with the success value of this stream, afterwards passing it on. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.inspect_ok)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#348-351)

#### fn [inspect\_err](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.inspect_err)<F>(self, f: F) -> [InspectErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.InspectErr.html "struct futures_util::stream::try_stream::InspectErr")<Self, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Do something with the error value of this stream, afterwards passing it on. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.inspect_err)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#377-379)

#### fn [into\_stream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.into_stream)(self) -> [IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wraps a [`TryStream`](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream") into a type that implements [`Stream`](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.into_stream)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#404-406)

#### fn [try\_next](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_next)(&mut self) -> [TryNext](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_next/struct.TryNext.html "struct futures_util::stream::try_stream::try_next::TryNext")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Creates a future that attempts to resolve the next item in the stream. If an error is encountered before the next item, the error is returned instead. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_next)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#444-448)

#### fn [try\_for\_each](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_for_each)<Fut, F>(self, f: F) -> [TryForEach](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_for_each/struct.TryForEach.html "struct futures_util::stream::try_stream::try_for_each::TryForEach")<Self, Fut, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempts to run this stream to completion, executing the provided asynchronous closure for each element on the stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_for_each)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#474-478)

#### fn [try\_skip\_while](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_skip_while)<Fut, F>(self, f: F) -> [TrySkipWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_skip_while/struct.TrySkipWhile.html "struct futures_util::stream::try_stream::try_skip_while::TrySkipWhile")<Self, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Skip elements on this stream while the provided asynchronous predicate resolves to `true`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_skip_while)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#504-508)

#### fn [try\_take\_while](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_take_while)<Fut, F>(self, f: F) -> [TryTakeWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_take_while/struct.TryTakeWhile.html "struct futures_util::stream::try_stream::try_take_while::TryTakeWhile")<Self, Fut, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Take elements on this stream while the provided asynchronous predicate resolves to `true`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_take_while)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#556-564)

#### fn [try\_for\_each\_concurrent](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_for_each_concurrent)<Fut, F>( self, limit: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>, f: F, ) -> [TryForEachConcurrent](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_for_each_concurrent/struct.TryForEachConcurrent.html "struct futures_util::stream::try_stream::try_for_each_concurrent::TryForEachConcurrent")<Self, Fut, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

Attempts to run this stream to completion, executing the provided asynchronous closure for each element on the stream concurrently as elements become available, exiting as soon as an error occurs. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_for_each_concurrent)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#603-605)

#### fn [try\_collect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_collect)<C>(self) -> [TryCollect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_collect/struct.TryCollect.html "struct futures_util::stream::try_stream::try_collect::TryCollect")<Self, C> [ⓘ](#)

where C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempt to transform a stream into a collection, returning a future representing the result of that computation. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_collect)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#648-650)

#### fn [try\_chunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_chunks)(self, capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [TryChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_chunks/struct.TryChunks.html "struct futures_util::stream::try_stream::try_chunks::TryChunks")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

An adaptor for chunking up successful items of the stream inside a vector. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_chunks)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#697-699)

#### fn [try\_ready\_chunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_ready_chunks)(self, capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [TryReadyChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_ready_chunks/struct.TryReadyChunks.html "struct futures_util::stream::try_stream::try_ready_chunks::TryReadyChunks")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

An adaptor for chunking up successful, ready items of the stream inside a vector. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_ready_chunks)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#736-740)

#### fn [try\_filter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_filter)<Fut, F>(self, f: F) -> [TryFilter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_filter/struct.TryFilter.html "struct futures_util::stream::try_stream::try_filter::TryFilter")<Self, Fut, F>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempt to filter the values produced by this stream according to the provided asynchronous closure. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_filter)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#780-784)

#### fn [try\_filter\_map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_filter_map)<Fut, F, T>(self, f: F) -> [TryFilterMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_filter_map/struct.TryFilterMap.html "struct futures_util::stream::try_stream::try_filter_map::TryFilterMap")<Self, Fut, F>

where Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>, Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempt to filter the values produced by this stream while simultaneously mapping them to a different type according to the provided asynchronous closure. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_filter_map)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#835-839)

#### fn [try\_flatten\_unordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_flatten_unordered)( self, limit: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>, ) -> [TryFlattenUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_flatten_unordered/struct.TryFlattenUnordered.html "struct futures_util::stream::try_stream::try_flatten_unordered::TryFlattenUnordered")<Self>

where Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), <Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

Flattens a stream of streams into just one continuous stream. Produced streams will be polled concurrently and any errors will be passed through without looking at them. If the underlying base stream returns an error, it will be **immediately** propagated. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_flatten_unordered)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#888-892)

#### fn [try\_flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_flatten)(self) -> [TryFlatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_flatten/struct.TryFlatten.html "struct futures_util::stream::try_stream::try_flatten::TryFlatten")<Self>

where Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), <Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Flattens a stream of streams into just one continuous stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_flatten)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#927-931)

#### fn [try\_fold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_fold)<T, Fut, F>(self, init: T, f: F) -> [TryFold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_fold/struct.TryFold.html "struct futures_util::stream::try_stream::try_fold::TryFold")<Self, Fut, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T, Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = T, Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempt to execute an accumulating asynchronous computation over a stream, collecting all the values into one final result. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_fold)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#970-973)

#### fn [try\_concat](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_concat)(self) -> [TryConcat](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_concat/struct.TryConcat.html "struct futures_util::stream::try_stream::try_concat::TryConcat")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\> + [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Attempt to concatenate all items of a stream into a single extendable destination, returning a future representing the end result. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_concat)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#1037-1040)

#### fn [try\_buffer\_unordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_buffer_unordered)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [TryBufferUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_buffer_unordered/struct.TryBufferUnordered.html "struct futures_util::stream::try_stream::try_buffer_unordered::TryBufferUnordered")<Self>

where Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

Attempt to execute several futures from a stream concurrently (unordered). [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_buffer_unordered)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#1113-1116)

#### fn [try\_buffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_buffered)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [TryBuffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_buffered/struct.TryBuffered.html "struct futures_util::stream::try_stream::try_buffered::TryBuffered")<Self>

where Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

Attempt to execute several futures from a stream concurrently. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_buffered)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#1127-1132)

#### fn [try\_poll\_next\_unpin](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_poll_next_unpin)( &mut self, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>>>

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

A convenience method for calling [`TryStream::try_poll_next`](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#tymethod.try_poll_next "method futures_core::stream::TryStream::try_poll_next") on [`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") stream types.

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#1217-1221)

#### fn [try\_all](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_all)<Fut, F>(self, f: F) -> [TryAll](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_all/struct.TryAll.html "struct futures_util::stream::try_stream::try_all::TryAll")<Self, Fut, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

Attempt to execute a predicate over an asynchronous stream and evaluate if all items satisfy the predicate. Exits early if an `Err` is encountered or if an `Ok` item is found that does not satisfy the predicate. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_all)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#1246-1250)

#### fn [try\_any](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_any)<Fut, F>(self, f: F) -> [TryAny](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_any/struct.TryAny.html "struct futures_util::stream::try_stream::try_any::TryAny")<Self, Fut, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

Attempt to execute a predicate over an asynchronous stream and evaluate if any items satisfy the predicate. Exits early if an `Err` is encountered or if an `Ok` item is found that satisfies the predicate. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/trait.TryStreamExt.html#method.try_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../../../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"All<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/all/struct.All.html\\" title=\\"struct futures\_util::stream::stream::all::All\\">All</a>&lt;St, Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/all/struct.All.html\\" title=\\"struct futures\_util::stream::stream::all::All\\">All</a>&lt;St, Fut, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>;</div>","AllFuture<'\_, Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.AllFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::AllFuture\\">AllFuture</a>&lt;'\_, S, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.AllFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::AllFuture\\">AllFuture</a>&lt;'\_, S, P&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>;</div>","Any<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/any/struct.Any.html\\" title=\\"struct futures\_util::stream::stream::any::Any\\">Any</a>&lt;St, Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/any/struct.Any.html\\" title=\\"struct futures\_util::stream::stream::any::Any\\">Any</a>&lt;St, Fut, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>;</div>","AnyFuture<'\_, Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.AnyFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::AnyFuture\\">AnyFuture</a>&lt;'\_, S, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.AnyFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::AnyFuture\\">AnyFuture</a>&lt;'\_, S, P&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>;</div>","Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Collect<Self, C>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/collect/struct.Collect.html\\" title=\\"struct futures\_util::stream::stream::collect::Collect\\">Collect</a>&lt;St, C&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, C&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/collect/struct.Collect.html\\" title=\\"struct futures\_util::stream::stream::collect::Collect\\">Collect</a>&lt;St, C&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n C: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;&lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = C;</div>","CollectFuture<Self, C>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.CollectFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::CollectFuture\\">CollectFuture</a>&lt;S, C&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, C&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.CollectFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::CollectFuture\\">CollectFuture</a>&lt;S, C&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n C: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = C;</div>","Concat<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/concat/struct.Concat.html\\" title=\\"struct futures\_util::stream::stream::concat::Concat\\">Concat</a>&lt;St&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/concat/struct.Concat.html\\" title=\\"struct futures\_util::stream::stream::concat::Concat\\">Concat</a>&lt;St&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n &lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;&lt;&lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a> as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\\" title=\\"trait core::iter::traits::collect::IntoIterator\\">IntoIterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::collect::IntoIterator::Item\\">Item</a>&gt; + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\\" title=\\"trait core::iter::traits::collect::IntoIterator\\">IntoIterator</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>;</div>","Count<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/count/struct.Count.html\\" title=\\"struct futures\_util::stream::stream::count::Count\\">Count</a>&lt;St&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/count/struct.Count.html\\" title=\\"struct futures\_util::stream::stream::count::Count\\">Count</a>&lt;St&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>;</div>","CountFuture<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.CountFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::CountFuture\\">CountFuture</a>&lt;S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.CountFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::CountFuture\\">CountFuture</a>&lt;S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>;</div>","Either<B, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/either/enum.Either.html\\" title=\\"enum futures\_util::future::either::Either\\">Either</a>&lt;A, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A, B&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/either/enum.Either.html\\" title=\\"enum futures\_util::future::either::Either\\">Either</a>&lt;A, B&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n B: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;A as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;A as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Either<Self, B>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/either/enum.Either.html\\" title=\\"enum futures\_util::future::either::Either\\">Either</a>&lt;A, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A, B&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/either/enum.Either.html\\" title=\\"enum futures\_util::future::either::Either\\">Either</a>&lt;A, B&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n B: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;A as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;A as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FindFuture<'\_, Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.FindFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FindFuture\\">FindFuture</a>&lt;'\_, S, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.FindFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FindFuture\\">FindFuture</a>&lt;'\_, S, P&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;;</div>","FindMapFuture<'\_, Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.FindMapFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FindMapFuture\\">FindMapFuture</a>&lt;'\_, S, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, B, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.FindMapFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FindMapFuture\\">FindMapFuture</a>&lt;'\_, S, F&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;B&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;B&gt;;</div>","Fold<Self, Fut, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/fold/struct.Fold.html\\" title=\\"struct futures\_util::stream::stream::fold::Fold\\">Fold</a>&lt;St, Fut, T, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, T, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/fold/struct.Fold.html\\" title=\\"struct futures\_util::stream::stream::fold::Fold\\">Fold</a>&lt;St, Fut, T, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(T, &lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>","FoldFuture<Self, F, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.FoldFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FoldFuture\\">FoldFuture</a>&lt;S, F, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, F, T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.FoldFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FoldFuture\\">FoldFuture</a>&lt;S, F, T&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(T, &lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; T,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>","ForEach<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/for\_each/struct.ForEach.html\\" title=\\"struct futures\_util::stream::stream::for\_each::ForEach\\">ForEach</a>&lt;St, Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/for\_each/struct.ForEach.html\\" title=\\"struct futures\_util::stream::stream::for\_each::ForEach\\">ForEach</a>&lt;St, Fut, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>;</div>","ForEachConcurrent<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/for\_each\_concurrent/struct.ForEachConcurrent.html\\" title=\\"struct futures\_util::stream::stream::for\_each\_concurrent::ForEachConcurrent\\">ForEachConcurrent</a>&lt;St, Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/for\_each\_concurrent/struct.ForEachConcurrent.html\\" title=\\"struct futures\_util::stream::stream::for\_each\_concurrent::ForEachConcurrent\\">ForEachConcurrent</a>&lt;St, Fut, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>;</div>","ForEachFuture<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ForEachFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::ForEachFuture\\">ForEachFuture</a>&lt;S, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.ForEachFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::ForEachFuture\\">ForEachFuture</a>&lt;S, F&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>),</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","LastFuture<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.LastFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::LastFuture\\">LastFuture</a>&lt;S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.LastFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::LastFuture\\">LastFuture</a>&lt;S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;;</div>","Next<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/next/struct.Next.html\\" title=\\"struct futures\_util::stream::stream::next::Next\\">Next</a>&lt;'\_, St&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/next/struct.Next.html\\" title=\\"struct futures\_util::stream::stream::next::Next\\">Next</a>&lt;'\_, St&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;;</div>","NextFuture<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.NextFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::NextFuture\\">NextFuture</a>&lt;'\_, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.NextFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::NextFuture\\">NextFuture</a>&lt;'\_, S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;;</div>","NthFuture<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.NthFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::NthFuture\\">NthFuture</a>&lt;'\_, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.NthFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::NthFuture\\">NthFuture</a>&lt;'\_, S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;;</div>","PartitionFuture<Self, P, B>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.PartitionFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::PartitionFuture\\">PartitionFuture</a>&lt;S, P, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P, B&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.PartitionFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::PartitionFuture\\">PartitionFuture</a>&lt;S, P, B&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,\\n B: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(B, B)</a>;</div>","PositionFuture<'\_, Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.PositionFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::PositionFuture\\">PositionFuture</a>&lt;'\_, S, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.PositionFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::PositionFuture\\">PositionFuture</a>&lt;'\_, S, P&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt;;</div>","SelectNextSome<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/select\_next\_some/struct.SelectNextSome.html\\" title=\\"struct futures\_util::stream::stream::select\_next\_some::SelectNextSome\\">SelectNextSome</a>&lt;'\_, St&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/select\_next\_some/struct.SelectNextSome.html\\" title=\\"struct futures\_util::stream::stream::select\_next\_some::SelectNextSome\\">SelectNextSome</a>&lt;'\_, St&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.FusedStream.html\\" title=\\"trait futures\_core::stream::FusedStream\\">FusedStream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>;</div>","StreamFuture<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/into\_future/struct.StreamFuture.html\\" title=\\"struct futures\_util::stream::stream::into\_future::StreamFuture\\">StreamFuture</a>&lt;St&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/into\_future/struct.StreamFuture.html\\" title=\\"struct futures\_util::stream::stream::into\_future::StreamFuture\\">StreamFuture</a>&lt;St&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = (<a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;St as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;, St);</div>","TryAll<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_all/struct.TryAll.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_all::TryAll\\">TryAll</a>&lt;St, Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_all/struct.TryAll.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_all::TryAll\\">TryAll</a>&lt;St, Fut, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;;</div>","TryAny<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_any/struct.TryAny.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_any::TryAny\\">TryAny</a>&lt;St, Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_any/struct.TryAny.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_any::TryAny\\">TryAny</a>&lt;St, Fut, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;;</div>","TryCollect<Self, C>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_collect/struct.TryCollect.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_collect::TryCollect\\">TryCollect</a>&lt;St, C&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, C&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_collect/struct.TryCollect.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_collect::TryCollect\\">TryCollect</a>&lt;St, C&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>,\\n C: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;&lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;C, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;;</div>","TryCollectFuture<Self, C>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TryCollectFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryCollectFuture\\">TryCollectFuture</a>&lt;S, C&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, E, S, C&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.TryCollectFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryCollectFuture\\">TryCollectFuture</a>&lt;S, C&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;&gt;,\\n C: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;C, E&gt;;</div>","TryConcat<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_concat/struct.TryConcat.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_concat::TryConcat\\">TryConcat</a>&lt;St&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_concat/struct.TryConcat.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_concat::TryConcat\\">TryConcat</a>&lt;St&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>,\\n &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;&lt;&lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a> as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\\" title=\\"trait core::iter::traits::collect::IntoIterator\\">IntoIterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::collect::IntoIterator::Item\\">Item</a>&gt; + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\\" title=\\"trait core::iter::traits::collect::IntoIterator\\">IntoIterator</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;&lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a>, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;;</div>","TryFold<Self, Fut, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_fold/struct.TryFold.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_fold::TryFold\\">TryFold</a>&lt;St, Fut, T, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, T, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_fold/struct.TryFold.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_fold::TryFold\\">TryFold</a>&lt;St, Fut, T, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(T, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/future/trait.TryFuture.html\\" title=\\"trait futures\_core::future::TryFuture\\">TryFuture</a>&lt;Ok = T, Error = &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;;</div>","TryFoldFuture<'\_, Self, F, B>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TryFoldFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryFoldFuture\\">TryFoldFuture</a>&lt;'\_, S, F, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, E, S, F, B&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.TryFoldFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryFoldFuture\\">TryFoldFuture</a>&lt;'\_, S, F, B&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;&gt; + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(B, T) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;B, E&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;B, E&gt;;</div>","TryForEach<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_for\_each/struct.TryForEach.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_for\_each::TryForEach\\">TryForEach</a>&lt;St, Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_for\_each/struct.TryForEach.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_for\_each::TryForEach\\">TryForEach</a>&lt;St, Fut, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/future/trait.TryFuture.html\\" title=\\"trait futures\_core::future::TryFuture\\">TryFuture</a>&lt;Ok = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, Error = &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;;</div>","TryForEachConcurrent<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_for\_each\_concurrent/struct.TryForEachConcurrent.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_for\_each\_concurrent::TryForEachConcurrent\\">TryForEachConcurrent</a>&lt;St, Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_for\_each\_concurrent/struct.TryForEachConcurrent.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_for\_each\_concurrent::TryForEachConcurrent\\">TryForEachConcurrent</a>&lt;St, Fut, F&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a>) -&gt; Fut,\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;;</div>","TryForEachFuture<'\_, Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TryForEachFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryForEachFuture\\">TryForEachFuture</a>&lt;'\_, S, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, F, E&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.TryForEachFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryForEachFuture\\">TryForEachFuture</a>&lt;'\_, S, F&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, E&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, E&gt;;</div>","TryNext<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_next/struct.TryNext.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_next::TryNext\\">TryNext</a>&lt;'\_, St&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/try\_stream/try\_next/struct.TryNext.html\\" title=\\"struct futures\_util::stream::try\_stream::try\_next::TryNext\\">TryNext</a>&lt;'\_, St&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Ok\\" title=\\"type futures\_core::stream::TryStream::Ok\\">Ok</a>&gt;, &lt;St as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html\\" title=\\"trait futures\_core::stream::TryStream\\">TryStream</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/stream/trait.TryStream.html#associatedtype.Error\\" title=\\"type futures\_core::stream::TryStream::Error\\">Error</a>&gt;;</div>","TryNextFuture<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TryNextFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryNextFuture\\">TryNextFuture</a>&lt;'\_, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, E, S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.TryNextFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryNextFuture\\">TryNextFuture</a>&lt;'\_, S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;&gt; + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;T&gt;, E&gt;;</div>","Unzip<Self, FromA, FromB>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/unzip/struct.Unzip.html\\" title=\\"struct futures\_util::stream::stream::unzip::Unzip\\">Unzip</a>&lt;St, FromA, FromB&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;St, A, B, FromA, FromB&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/stream/stream/unzip/struct.Unzip.html\\" title=\\"struct futures\_util::stream::stream::unzip::Unzip\\">Unzip</a>&lt;St, FromA, FromB&gt;<div class=\\"where\\">where\\n St: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&lt;Item = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(A, B)</a>&gt;,\\n FromA: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;A&gt;,\\n FromB: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;B&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(FromA, FromB)</a>;</div>","UnzipFuture<Self, FromA, FromB>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.UnzipFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::UnzipFuture\\">UnzipFuture</a>&lt;S, FromA, FromB&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, A, B, FromA, FromB&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.UnzipFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::UnzipFuture\\">UnzipFuture</a>&lt;S, FromA, FromB&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&lt;Item = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(A, B)</a>&gt;,\\n FromA: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;A&gt;,\\n FromB: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;B&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(FromA, FromB)</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}