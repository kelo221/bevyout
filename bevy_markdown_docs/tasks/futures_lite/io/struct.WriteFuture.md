[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Struct WriteFuture 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2907)

```rust
pub struct WriteFuture<'a, W>where
    W: Unpin + ?Sized,{ /* private fields */ }
```

Available on **crate feature `std`** only.

Future for the [`AsyncWriteExt::write()`](../../../asset/trait.AsyncWriteExt.html#method.write "method bevy::asset::AsyncWriteExt::write") method.

## Trait Implementations

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2905)

### impl<'a, W> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'a, W>

where W: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2905)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2914)

### impl<W> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'\_, W>

where W: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2915)

#### type [Output](../trait.Future.html#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

The type of value produced on completion.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2917)

#### fn [poll](../trait.Future.html#tymethod.poll)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'\_, W>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<<[WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'\_, W> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>

Attempts to resolve the future to a final value, registering the current task for wakeup if the value is not yet available. [Read more](../trait.Future.html#tymethod.poll)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2912)

### impl<W> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'\_, W>

where W: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

## Auto Trait Implementations

### impl<'a, W> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'a, W>

### impl<'a, W> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'a, W>

where W: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<'a, W> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'a, W>

where W: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<'a, W> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'a, W>

where W: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<'a, W> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'a, W>

where W: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<'a, W> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'a, W>

where W: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

## Blanket Implementations

[Source](https://docs.rs/warnings/0.2.1/x86_64-unknown-linux-gnu/src/warnings/warnings.rs.html#165)

### impl<F> [AllowFutureExt](https://docs.rs/warnings/0.2.1/x86_64-unknown-linux-gnu/warnings/warnings/trait.AllowFutureExt.html "trait warnings::warnings::AllowFutureExt") for F

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/warnings/0.2.1/x86_64-unknown-linux-gnu/src/warnings/warnings.rs.html#157-159)

#### fn [allow](https://docs.rs/warnings/0.2.1/x86_64-unknown-linux-gnu/warnings/warnings/trait.AllowFutureExt.html#method.allow)<W>(self) -> [AllowFuture](https://docs.rs/warnings/0.2.1/x86_64-unknown-linux-gnu/warnings/warnings/struct.AllowFuture.html "struct warnings::warnings::AllowFuture")<Self> [ⓘ](#)

where W: [Warning](https://docs.rs/warnings/0.2.1/x86_64-unknown-linux-gnu/warnings/warnings/trait.Warning.html "trait warnings::warnings::Warning") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Allow a lint while a future is running

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

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#58)

### impl<T> [ConditionalSendFuture](../../trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture") for T

where T: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + [ConditionalSend](../../trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend"),

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

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#830)

### impl<F> [FutureExt](../trait.FutureExt.html "trait bevy::tasks::futures_lite::FutureExt") for F

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#688-690)

#### fn [poll](../trait.FutureExt.html#method.poll)(&mut self, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

A convenience for calling [`Future::poll()`](../trait.Future.html#tymethod.poll "method bevy::tasks::futures_lite::Future::poll") on `!`[`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") types.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#713-716)

#### fn [or](../trait.FutureExt.html#method.or)<F>(self, other: F) -> [Or](../future/struct.Or.html "struct bevy::tasks::futures_lite::future::Or")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>,

Returns the result of `self` or `other` future, preferring `self` if both are ready. [Read more](../trait.FutureExt.html#method.or)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#746-749)

#### fn [race](../trait.FutureExt.html#method.race)<F>(self, other: F) -> [Race](../future/struct.Race.html "struct bevy::tasks::futures_lite::future::Race")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>,

Available on **crate features `race` and `std`** only.

Returns the result of `self` or `other` future, with no preference if both are ready. [Read more](../trait.FutureExt.html#method.race)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#774-776)

#### fn [catch\_unwind](../trait.FutureExt.html#method.catch_unwind)(self) -> [CatchUnwind](../future/struct.CatchUnwind.html "struct bevy::tasks::futures_lite::future::CatchUnwind")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

Available on **crate feature `std`** only.

Catches panics while polling the future. [Read more](../trait.FutureExt.html#method.catch_unwind)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#798-800)

#### fn [boxed](../trait.FutureExt.html#method.boxed)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a,

Available on **crate feature `alloc`** only.

Boxes the future and changes its type to `dyn Future + Send + 'a`. [Read more](../trait.FutureExt.html#method.boxed)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#822-824)

#### fn [boxed\_local](../trait.FutureExt.html#method.boxed_local)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\> + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + 'a,

Available on **crate feature `alloc`** only.

Boxes the future and changes its type to `dyn Future + 'a`. [Read more](../trait.FutureExt.html#method.boxed_local)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#117)

### impl<T> [FutureExt](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html "trait futures_util::future::future::FutureExt") for T

where T: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#144-147)

#### fn [map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.map)<U, F>(self, f: F) -> [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Self, F> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")) -> U, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Map this future’s output to a different type, returning a new future of the resulting type. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.map)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#157-160)

#### fn [map\_into](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.map_into)<U>(self) -> [MapInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.MapInto.html "struct futures_util::future::future::MapInto")<Self, U> [ⓘ](#)

where Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Map this future’s output to a different type, returning a new future of the resulting type. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.map_into)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#189-193)

#### fn [then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.then)<Fut, F>(self, f: F) -> [Then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Then.html "struct futures_util::future::future::Then")<Self, Fut, F> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Chain on a computation for when a future finished, passing the result of the future to the provided closure `f`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.then)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#220-223)

#### fn [left\_future](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.left_future)<B>(self) -> [Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<Self, B> [ⓘ](#)

where B: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wrap this future in an `Either` future, making it the left-hand variant of that `Either`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.left_future)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#250-253)

#### fn [right\_future](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.right_future)<A>(self) -> [Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<A, Self> [ⓘ](#)

where A: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wrap this future in an `Either` future, making it the right-hand variant of that `Either`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.right_future)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#276-278)

#### fn [into\_stream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.into_stream)(self) -> [IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.IntoStream.html "struct futures_util::future::future::IntoStream")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Convert this future into a single element stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.into_stream)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#305-308)

#### fn [flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.flatten)(self) -> [Flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Flatten.html "struct futures_util::future::future::Flatten")<Self> [ⓘ](#)

where Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"): [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Flatten the execution of this future when the output of this future is itself another future. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.flatten)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#339-342)

#### fn [flatten\_stream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.flatten_stream)(self) -> [FlattenStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.FlattenStream.html "struct futures_util::future::future::FlattenStream")<Self>

where Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"): [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Flatten the execution of this future when the successful result of this future is a stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.flatten_stream)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#363-365)

#### fn [fuse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.fuse)(self) -> [Fuse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/fuse/struct.Fuse.html "struct futures_util::future::future::fuse::Fuse")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fuse a future such that `poll` will never again be called once it has completed. This method can be used to turn any `Future` into a `FusedFuture`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.fuse)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#389-392)

#### fn [inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.inspect)<F>(self, f: F) -> [Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Inspect.html "struct futures_util::future::future::Inspect")<Self, F> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Do something with the output of a future before passing it on. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.inspect)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#429-431)

#### fn [catch\_unwind](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.catch_unwind)(self) -> [CatchUnwind](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/catch_unwind/struct.CatchUnwind.html "struct futures_util::future::future::catch_unwind::CatchUnwind")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

Available on **crate feature `std`** only.

Catches unwinding panics while polling the future. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.catch_unwind)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#480-483)

#### fn [shared](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.shared)(self) -> [Shared](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/shared/struct.Shared.html "struct futures_util::future::future::shared::Shared")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **crate feature `std`, or crate features `alloc` and `spin`** only.

Create a cloneable handle to this future where all handles will resolve to the same result. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.shared)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#512-514)

#### fn [boxed](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.boxed)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a,

Available on **crate feature `alloc`** only.

Wrap the future in a Box, pinning it. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.boxed)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#526-528)

#### fn [boxed\_local](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.boxed_local)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\> + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + 'a,

Available on **crate feature `alloc`** only.

Wrap the future in a Box, pinning it. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.boxed_local)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#535-537)

#### fn [unit\_error](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.unit_error)(self) -> [UnitError](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.UnitError.html "struct futures_util::future::future::UnitError")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Turns a [`Future<Output = T>`](../trait.Future.html "trait bevy::tasks::futures_lite::Future") into a [`TryFuture<Ok = T, Error = ()`\>](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture").

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#544-546)

#### fn [never\_error](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.never_error)(self) -> [NeverError](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.NeverError.html "struct futures_util::future::future::NeverError")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Turns a [`Future<Output = T>`](../trait.Future.html "trait bevy::tasks::futures_lite::Future") into a [`TryFuture<Ok = T, Error = Never`\>](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture").

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#552-554)

#### fn [poll\_unpin](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.poll_unpin)(&mut self, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

A convenience for calling `Future::poll` on `Unpin` future types.

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#590-592)

#### fn [now\_or\_never](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.now_or_never)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Evaluates and consumes the future, returning the resulting output if the future is ready after the first call to `Future::poll`. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.now_or_never)

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

[Source](https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#138)

### impl<F> [IntoFuture](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html "trait core::future::into_future::IntoFuture") for F

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#139)

#### type [Output](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.Output) = <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

The output that the future will produce on completion.

[Source](https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#140)

#### type [IntoFuture](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture) = F

Which kind of future are we turning this into?

[Source](https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#142)

#### fn [into\_future](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future)(self) -> <F as [IntoFuture](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html "trait core::future::into_future::IntoFuture")\>::[IntoFuture](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#associatedtype.IntoFuture "type core::future::into_future::IntoFuture::IntoFuture")

Creates a future from a value. [Read more](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future)

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

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/future.rs.html#83-85)

### impl<F, T, E> [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture") for F

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/future.rs.html#87)

#### type [Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok) = T

The type of successful values yielded by this future

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/future.rs.html#88)

#### type [Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error) = E

The type of failures yielded by this future

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/future.rs.html#91)

#### fn [try\_poll](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#tymethod.try_poll)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<<F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>

Poll this `TryFuture` as if it were a `Future`. [Read more](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#tymethod.try_poll)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#134)

### impl<Fut> [TryFutureExt](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html "trait futures_util::future::try_future::TryFutureExt") for Fut

where Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#217-220)

#### fn [map\_ok](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.map_ok)<T, F>(self, f: F) -> [MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapOk.html "struct futures_util::future::try_future::MapOk")<Self, F> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")) -> T, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Maps this future’s success value to a different value. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.map_ok)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#257-261)

#### fn [map\_ok\_or\_else](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.map_ok_or_else)<T, E, F>(self, e: E, f: F) -> [MapOkOrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapOkOrElse.html "struct futures_util::future::try_future::MapOkOrElse")<Self, F, E> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")) -> T, E: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")) -> T, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Maps this future’s success value to a different value, and permits for error handling resulting in the same type. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.map_ok_or_else)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#308-311)

#### fn [map\_err](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.map_err)<E, F>(self, f: F) -> [MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapErr.html "struct futures_util::future::try_future::MapErr")<Self, F> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")) -> E, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Maps this future’s error value to a different value. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.map_err)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#341-344)

#### fn [err\_into](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.err_into)<E>(self) -> [ErrInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.ErrInto.html "struct futures_util::future::try_future::ErrInto")<Self, E> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error"): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<E>,

Maps this future’s [`Error`](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "associated type futures_core::future::TryFuture::Error") to a new error type using the [`Into`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") trait. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.err_into)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#351-354)

#### fn [ok\_into](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.ok_into)<U>(self) -> [OkInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.OkInto.html "struct futures_util::future::try_future::OkInto")<Self, U> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

Maps this future’s [`Ok`](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "associated type futures_core::future::TryFuture::Ok") to a new type using the [`Into`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") trait.

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#395-399)

#### fn [and\_then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.and_then)<Fut, F>(self, f: F) -> [AndThen](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.AndThen.html "struct futures_util::future::try_future::AndThen")<Self, Fut, F> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Executes another future after this one resolves successfully. The success value is passed to a closure to create this subsequent future. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.and_then)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#440-444)

#### fn [or\_else](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.or_else)<Fut, F>(self, f: F) -> [OrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.OrElse.html "struct futures_util::future::try_future::OrElse")<Self, Fut, F> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Executes another future if this one resolves to an error. The error value is passed to a closure to create this subsequent future. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.or_else)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#467-470)

#### fn [inspect\_ok](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.inspect_ok)<F>(self, f: F) -> [InspectOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.InspectOk.html "struct futures_util::future::try_future::InspectOk")<Self, F> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Do something with the success value of a future before passing it on. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.inspect_ok)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#493-496)

#### fn [inspect\_err](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.inspect_err)<F>(self, f: F) -> [InspectErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.InspectErr.html "struct futures_util::future::try_future::InspectErr")<Self, F> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Do something with the error value of a future before passing it on. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.inspect_err)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#505-508)

#### fn [try\_flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.try_flatten)(self) -> [TryFlatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.TryFlatten.html "struct futures_util::future::try_future::TryFlatten")<Self, Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")\> [ⓘ](#)

where Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"): [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Flatten the execution of this future when the successful result of this future is another future. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.try_flatten)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#538-541)

#### fn [try\_flatten\_stream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.try_flatten_stream)(self) -> [TryFlattenStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.TryFlattenStream.html "struct futures_util::future::try_future::TryFlattenStream")<Self>

where Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"): [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")<Error = Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Flatten the execution of this future when the successful result of this future is a stream. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.try_flatten_stream)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#570-573)

#### fn [unwrap\_or\_else](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.unwrap_or_else)<F>(self, f: F) -> [UnwrapOrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.UnwrapOrElse.html "struct futures_util::future::try_future::UnwrapOrElse")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")) -> Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"),

Unwraps this future’s output, producing a future with this future’s [`Ok`](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "associated type futures_core::future::TryFuture::Ok") type as its [`Output`](../trait.Future.html#associatedtype.Output "associated type bevy::tasks::futures_lite::Future::Output") type. [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.unwrap_or_else)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#610-612)

#### fn [into\_future](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.into_future)(self) -> [IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wraps a [`TryFuture`](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture") into a type that implements [`Future`](../trait.Future.html "trait bevy::tasks::futures_lite::Future"). [Read more](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.into_future)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#619-621)

#### fn [try\_poll\_unpin](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.try_poll_unpin)( &mut self, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), Self::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>>

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

A convenience method for calling [`TryFuture::try_poll`](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#tymethod.try_poll "method futures_core::future::TryFuture::try_poll") on [`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") future types.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

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

{"AllowFuture<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/warnings/0.2.1/x86\_64-unknown-linux-gnu/warnings/warnings/struct.AllowFuture.html\\" title=\\"struct warnings::warnings::AllowFuture\\">AllowFuture</a>&lt;F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/warnings/0.2.1/x86\_64-unknown-linux-gnu/warnings/warnings/struct.AllowFuture.html\\" title=\\"struct warnings::warnings::AllowFuture\\">AllowFuture</a>&lt;F&gt;<div class=\\"where\\">where\\n F: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;F as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","AndThen<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.AndThen.html\\" title=\\"struct futures\_util::future::try\_future::AndThen\\">AndThen</a>&lt;Fut1, Fut2, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut1, Fut2, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.AndThen.html\\" title=\\"struct futures\_util::future::try\_future::AndThen\\">AndThen</a>&lt;Fut1, Fut2, F&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.TryFlatten.html\\" title=\\"struct futures\_util::future::try\_future::TryFlatten\\">TryFlatten</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapOk.html\\" title=\\"struct futures\_util::future::try\_future::MapOk\\">MapOk</a>&lt;Fut1, F&gt;, Fut2&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.TryFlatten.html\\" title=\\"struct futures\_util::future::try\_future::TryFlatten\\">TryFlatten</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapOk.html\\" title=\\"struct futures\_util::future::try\_future::MapOk\\">MapOk</a>&lt;Fut1, F&gt;, Fut2&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","CatchUnwind<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/catch\_unwind/struct.CatchUnwind.html\\" title=\\"struct futures\_util::future::future::catch\_unwind::CatchUnwind\\">CatchUnwind</a>&lt;Fut&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/catch\_unwind/struct.CatchUnwind.html\\" title=\\"struct futures\_util::future::future::catch\_unwind::CatchUnwind\\">CatchUnwind</a>&lt;Fut&gt;<div class=\\"where\\">where\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/panic/unwind\_safe/trait.UnwindSafe.html\\" title=\\"trait core::panic::unwind\_safe::UnwindSafe\\">UnwindSafe</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;&lt;Fut as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>, <a class=\\"struct\\" href=\\"../../../prelude/struct.Box.html\\" title=\\"struct bevy::prelude::Box\\">Box</a>&lt;dyn <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\\" title=\\"trait core::any::Any\\">Any</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\\" title=\\"trait core::marker::Send\\">Send</a>&gt;&gt;;</div>","Either<A, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/either/enum.Either.html\\" title=\\"enum futures\_util::future::either::Either\\">Either</a>&lt;A, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A, B&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/either/enum.Either.html\\" title=\\"enum futures\_util::future::either::Either\\">Either</a>&lt;A, B&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n B: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;A as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;A as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Either<Self, B>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/either/enum.Either.html\\" title=\\"enum futures\_util::future::either::Either\\">Either</a>&lt;A, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A, B&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/either/enum.Either.html\\" title=\\"enum futures\_util::future::either::Either\\">Either</a>&lt;A, B&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n B: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;A as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;A as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","ErrInto<Self, E>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.ErrInto.html\\" title=\\"struct futures\_util::future::try\_future::ErrInto\\">ErrInto</a>&lt;Fut, E&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, E&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.ErrInto.html\\" title=\\"struct futures\_util::future::try\_future::ErrInto\\">ErrInto</a>&lt;Fut, E&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapErr.html\\" title=\\"struct futures\_util::future::try\_future::MapErr\\">MapErr</a>&lt;Fut, IntoFn&lt;E&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapErr.html\\" title=\\"struct futures\_util::future::try\_future::MapErr\\">MapErr</a>&lt;Fut, IntoFn&lt;E&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Flatten<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Flatten.html\\" title=\\"struct futures\_util::future::future::Flatten\\">Flatten</a>&lt;F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Flatten.html\\" title=\\"struct futures\_util::future::future::Flatten\\">Flatten</a>&lt;F&gt;<div class=\\"where\\">where\\n Flatten&lt;F, &lt;F as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n F: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;Flatten&lt;F, &lt;F as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Fuse<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/fuse/struct.Fuse.html\\" title=\\"struct futures\_util::future::future::fuse::Fuse\\">Fuse</a>&lt;Fut&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/fuse/struct.Fuse.html\\" title=\\"struct futures\_util::future::future::fuse::Fuse\\">Fuse</a>&lt;Fut&gt;<div class=\\"where\\">where\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;Fut as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Inspect<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Inspect.html\\" title=\\"struct futures\_util::future::future::Inspect\\">Inspect</a>&lt;Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Inspect.html\\" title=\\"struct futures\_util::future::future::Inspect\\">Inspect</a>&lt;Fut, F&gt;<div class=\\"where\\">where\\n Map&lt;Fut, InspectFn&lt;F&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;Map&lt;Fut, InspectFn&lt;F&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","InspectErr<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.InspectErr.html\\" title=\\"struct futures\_util::future::try\_future::InspectErr\\">InspectErr</a>&lt;Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.InspectErr.html\\" title=\\"struct futures\_util::future::try\_future::InspectErr\\">InspectErr</a>&lt;Fut, F&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Inspect.html\\" title=\\"struct futures\_util::future::future::Inspect\\">Inspect</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, InspectErrFn&lt;F&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Inspect.html\\" title=\\"struct futures\_util::future::future::Inspect\\">Inspect</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, InspectErrFn&lt;F&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","InspectOk<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.InspectOk.html\\" title=\\"struct futures\_util::future::try\_future::InspectOk\\">InspectOk</a>&lt;Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.InspectOk.html\\" title=\\"struct futures\_util::future::try\_future::InspectOk\\">InspectOk</a>&lt;Fut, F&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Inspect.html\\" title=\\"struct futures\_util::future::future::Inspect\\">Inspect</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, InspectOkFn&lt;F&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Inspect.html\\" title=\\"struct futures\_util::future::future::Inspect\\">Inspect</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, InspectOkFn&lt;F&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","IntoFuture<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;<div class=\\"where\\">where\\n Fut: <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/future/trait.TryFuture.html\\" title=\\"trait futures\_core::future::TryFuture\\">TryFuture</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;&lt;Fut as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/future/trait.TryFuture.html\\" title=\\"trait futures\_core::future::TryFuture\\">TryFuture</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/future/trait.TryFuture.html#associatedtype.Ok\\" title=\\"type futures\_core::future::TryFuture::Ok\\">Ok</a>, &lt;Fut as <a class=\\"trait\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/future/trait.TryFuture.html\\" title=\\"trait futures\_core::future::TryFuture\\">TryFuture</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/futures-core/0.3.32/x86\_64-unknown-linux-gnu/futures\_core/future/trait.TryFuture.html#associatedtype.Error\\" title=\\"type futures\_core::future::TryFuture::Error\\">Error</a>&gt;;</div>","Map<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut, F&gt;<div class=\\"where\\">where\\n Map&lt;Fut, F&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;Map&lt;Fut, F&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","MapErr<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapErr.html\\" title=\\"struct futures\_util::future::try\_future::MapErr\\">MapErr</a>&lt;Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapErr.html\\" title=\\"struct futures\_util::future::try\_future::MapErr\\">MapErr</a>&lt;Fut, F&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, MapErrFn&lt;F&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, MapErrFn&lt;F&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","MapInto<Self, U>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.MapInto.html\\" title=\\"struct futures\_util::future::future::MapInto\\">MapInto</a>&lt;Fut, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.MapInto.html\\" title=\\"struct futures\_util::future::future::MapInto\\">MapInto</a>&lt;Fut, T&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut, IntoFn&lt;T&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut, IntoFn&lt;T&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","MapOk<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapOk.html\\" title=\\"struct futures\_util::future::try\_future::MapOk\\">MapOk</a>&lt;Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapOk.html\\" title=\\"struct futures\_util::future::try\_future::MapOk\\">MapOk</a>&lt;Fut, F&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, MapOkFn&lt;F&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, MapOkFn&lt;F&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","MapOkOrElse<Self, F, E>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapOkOrElse.html\\" title=\\"struct futures\_util::future::try\_future::MapOkOrElse\\">MapOkOrElse</a>&lt;Fut, F, G&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, F, G&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapOkOrElse.html\\" title=\\"struct futures\_util::future::try\_future::MapOkOrElse\\">MapOkOrElse</a>&lt;Fut, F, G&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, ChainFn&lt;MapOkFn&lt;F&gt;, ChainFn&lt;MapErrFn&lt;G&gt;, MergeResultFn&gt;&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, ChainFn&lt;MapOkFn&lt;F&gt;, ChainFn&lt;MapErrFn&lt;G&gt;, MergeResultFn&gt;&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","NeverError<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.NeverError.html\\" title=\\"struct futures\_util::future::future::NeverError\\">NeverError</a>&lt;Fut&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.NeverError.html\\" title=\\"struct futures\_util::future::future::NeverError\\">NeverError</a>&lt;Fut&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut, OkFn&lt;<a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html\\" title=\\"enum core::convert::Infallible\\">Infallible</a>&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut, OkFn&lt;<a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html\\" title=\\"enum core::convert::Infallible\\">Infallible</a>&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","OkInto<Self, U>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.OkInto.html\\" title=\\"struct futures\_util::future::try\_future::OkInto\\">OkInto</a>&lt;Fut, E&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, E&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.OkInto.html\\" title=\\"struct futures\_util::future::try\_future::OkInto\\">OkInto</a>&lt;Fut, E&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapOk.html\\" title=\\"struct futures\_util::future::try\_future::MapOk\\">MapOk</a>&lt;Fut, IntoFn&lt;E&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapOk.html\\" title=\\"struct futures\_util::future::try\_future::MapOk\\">MapOk</a>&lt;Fut, IntoFn&lt;E&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Or<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../future/struct.Or.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Or\\">Or</a>&lt;F1, F2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F1, F2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../future/struct.Or.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Or\\">Or</a>&lt;F1, F2&gt;<div class=\\"where\\">where\\n F1: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,\\n F2: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>","OrElse<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.OrElse.html\\" title=\\"struct futures\_util::future::try\_future::OrElse\\">OrElse</a>&lt;Fut1, Fut2, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut1, Fut2, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.OrElse.html\\" title=\\"struct futures\_util::future::try\_future::OrElse\\">OrElse</a>&lt;Fut1, Fut2, F&gt;<div class=\\"where\\">where\\n TryFlattenErr&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapErr.html\\" title=\\"struct futures\_util::future::try\_future::MapErr\\">MapErr</a>&lt;Fut1, F&gt;, Fut2&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;TryFlattenErr&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.MapErr.html\\" title=\\"struct futures\_util::future::try\_future::MapErr\\">MapErr</a>&lt;Fut1, F&gt;, Fut2&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Race<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../future/struct.Race.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Race\\">Race</a>&lt;F1, F2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F1, F2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../future/struct.Race.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Race\\">Race</a>&lt;F1, F2&gt;<div class=\\"where\\">where\\n F1: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,\\n F2: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>","Shared<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/shared/struct.Shared.html\\" title=\\"struct futures\_util::future::future::shared::Shared\\">Shared</a>&lt;Fut&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/shared/struct.Shared.html\\" title=\\"struct futures\_util::future::future::shared::Shared\\">Shared</a>&lt;Fut&gt;<div class=\\"where\\">where\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n &lt;Fut as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;Fut as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Then<Self, Fut, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Then.html\\" title=\\"struct futures\_util::future::future::Then\\">Then</a>&lt;Fut1, Fut2, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut1, Fut2, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Then.html\\" title=\\"struct futures\_util::future::future::Then\\">Then</a>&lt;Fut1, Fut2, F&gt;<div class=\\"where\\">where\\n Flatten&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut1, F&gt;, Fut2&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;Flatten&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut1, F&gt;, Fut2&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","TryFlatten<Self, Self::Ok>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.TryFlatten.html\\" title=\\"struct futures\_util::future::try\_future::TryFlatten\\">TryFlatten</a>&lt;Fut1, Fut2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut1, Fut2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.TryFlatten.html\\" title=\\"struct futures\_util::future::try\_future::TryFlatten\\">TryFlatten</a>&lt;Fut1, Fut2&gt;<div class=\\"where\\">where\\n TryFlatten&lt;Fut1, Fut2&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;TryFlatten&lt;Fut1, Fut2&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","UnitError<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.UnitError.html\\" title=\\"struct futures\_util::future::future::UnitError\\">UnitError</a>&lt;Fut&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.UnitError.html\\" title=\\"struct futures\_util::future::future::UnitError\\">UnitError</a>&lt;Fut&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut, OkFn&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;Fut, OkFn&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","UnwrapOrElse<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.UnwrapOrElse.html\\" title=\\"struct futures\_util::future::try\_future::UnwrapOrElse\\">UnwrapOrElse</a>&lt;Fut, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/struct.UnwrapOrElse.html\\" title=\\"struct futures\_util::future::try\_future::UnwrapOrElse\\">UnwrapOrElse</a>&lt;Fut, F&gt;<div class=\\"where\\">where\\n <a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, UnwrapOrElseFn&lt;F&gt;&gt;: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/future/struct.Map.html\\" title=\\"struct futures\_util::future::future::Map\\">Map</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/futures-util/0.3.32/x86\_64-unknown-linux-gnu/futures\_util/future/try\_future/into\_future/struct.IntoFuture.html\\" title=\\"struct futures\_util::future::try\_future::into\_future::IntoFuture\\">IntoFuture</a>&lt;Fut&gt;, UnwrapOrElseFn&lt;F&gt;&gt; as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}