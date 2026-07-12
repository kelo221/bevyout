[bevy](../../../index.html)::[reflect](../../index.html)::[func](../index.html)::[args](index.html)

# Trait FromArg 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/from_arg.rs.html#20)

```rust
pub trait FromArg {
    type This<'a>;

    // Required method
    fn from_arg(arg: Arg<'_>) -> Result<Self::This<'_>, ArgError>;
}
```

Available on **crate feature `functions`** only.

A trait for types that can be created from an [`Arg`](struct.Arg.html "struct bevy::reflect::func::args::Arg").

This trait exists so that types can be automatically converted into an [`Arg`](struct.Arg.html "struct bevy::reflect::func::args::Arg") so they can be put into an [`ArgList`](../struct.ArgList.html "struct bevy::reflect::func::ArgList") and passed to a [`DynamicFunction`](../struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") or [`DynamicFunctionMut`](../struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

This trait is used instead of a blanket [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") implementation due to coherence issues: we can’t implement `From<T>` for both `T` and `&T`/`&mut T`.

This trait is automatically implemented for non-reference types when using the `Reflect` [derive macro](../../../prelude/derive.Reflect.html "derive bevy::prelude::Reflect"). Blanket impls cover `&T` and `&mut T`.

## Required Associated Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/from_arg.rs.html#27)

#### type [This](#associatedtype.This)<'a>

The type to convert into.

This should almost always be the same as `Self`, but with the lifetime `'a`.

The reason we use a separate associated type is to allow for the lifetime to be tied to the argument, rather than the type itself.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/from_arg.rs.html#32)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument.

The argument must be of the expected type and ownership.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### type [This](#associatedtype.This)<'from\_arg> = [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#320)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#320)

#### type [This](#associatedtype.This)<'from\_arg> = [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#320)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#142)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#142)

#### type [This](#associatedtype.This)<'from\_arg> = [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#142)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### type [This](#associatedtype.This)<'from\_arg> = [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration") as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### type [This](#associatedtype.This)<'from\_arg> = [NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex") as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### type [This](#associatedtype.This)<'from\_arg> = [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OsString](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### type [This](#associatedtype.This)<'from\_arg> = [OsString](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[OsString](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString") as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### type [This](#associatedtype.This)<'from\_arg> = [PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf") as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### type [This](#associatedtype.This)<'from\_arg> = [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull") as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### type [This](#associatedtype.This)<'from\_arg> = [SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr") as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SocketAddr](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### type [This](#associatedtype.This)<'from\_arg> = [SocketAddr](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[SocketAddr](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr") as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### type [This](#associatedtype.This)<'from\_arg> = [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### type [This](#associatedtype.This)<'from\_arg> = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### type [This](#associatedtype.This)<'from\_arg> = [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### type [This](#associatedtype.This)<'from\_arg> = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### type [This](#associatedtype.This)<'from\_arg> = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### type [This](#associatedtype.This)<'from\_arg> = [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### type [This](#associatedtype.This)<'from\_arg> = [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### type [This](#associatedtype.This)<'from\_arg> = [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### type [This](#associatedtype.This)<'from\_arg> = [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### type [This](#associatedtype.This)<'from\_arg> = [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### type [This](#associatedtype.This)<'from\_arg> = [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### type [This](#associatedtype.This)<'from\_arg> = [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### type [This](#associatedtype.This)<'from\_arg> = [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### type [This](#associatedtype.This)<'from\_arg> = [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### type [This](#associatedtype.This)<'from\_arg> = [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### type [This](#associatedtype.This)<'from\_arg> = [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### type [This](#associatedtype.This)<'from\_arg> = [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#17-23)

### impl<K, V, S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, V, S>

where K: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#17-23)

#### type [This](#associatedtype.This)<'from\_arg> = [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, V, S>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#17-23)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, V, S> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#231-236)

### impl<K, V> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V>

where K: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), V: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#231-236)

#### type [This](#associatedtype.This)<'from\_arg> = [BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#231-236)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

### impl<N, E, Ix> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix>

where N: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), E: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), Ix: [IndexType](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/trait.IndexType.html "trait petgraph::graph_impl::IndexType") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### type [This](#associatedtype.This)<'from\_arg> = [Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P3: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P4: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P5: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P6: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P7: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P8: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P9: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P10: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P11: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P3: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P4: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P5: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P6: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P7: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P8: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P9: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P10: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P3: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P4: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P5: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P6: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P7: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P8: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P9: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P3: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P4: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P5: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P6: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P7: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P8: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2, P3, P4, P5, P6, P7, P8)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2, P3, P4, P5, P6, P7, P8)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2, P3, P4, P5, P6, P7> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2, P3, P4, P5, P6, P7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P3: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P4: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P5: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P6: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P7: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2, P3, P4, P5, P6, P7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2, P3, P4, P5, P6, P7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2, P3, P4, P5, P6> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2, P3, P4, P5, P6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P3: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P4: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P5: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P6: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2, P3, P4, P5, P6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2, P3, P4, P5, P6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2, P3, P4, P5> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2, P3, P4, P5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P3: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P4: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P5: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2, P3, P4, P5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2, P3, P4, P5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2, P3, P4> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2, P3, P4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P3: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P4: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2, P3, P4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2, P3, P4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2, P3> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2, P3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P3: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2, P3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2, P3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1, P2> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1, P2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P2: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1, P2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1, P2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P0, P1> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P0, P1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), P1: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P0, P1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P0, P1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### type [This](#associatedtype.This)<'from\_arg> = [(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#803-809)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

### impl<T, E> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>

where [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, E: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### type [This](#associatedtype.This)<'from\_arg> = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#652)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#652)

#### type [This](#associatedtype.This)<'from\_arg> = [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#652)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/from_arg.rs.html#36)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [&'static T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/from_arg.rs.html#37)

#### type [This](#associatedtype.This)<'a> = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/from_arg.rs.html#38)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[&'static T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/from_arg.rs.html#44)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [&'static mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/from_arg.rs.html#45)

#### type [This](#associatedtype.This)<'a> = [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/from_arg.rs.html#46)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[&'static mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### type [This](#associatedtype.This)<'from\_arg> = [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### type [This](#associatedtype.This)<'from\_arg> = [BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### type [This](#associatedtype.This)<'from\_arg> = [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#320)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where T: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#320)

#### type [This](#associatedtype.This)<'from\_arg> = [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#320)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### type [This](#associatedtype.This)<'from\_arg> = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### type [This](#associatedtype.This)<'from\_arg> = [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### type [This](#associatedtype.This)<'from\_arg> = [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### type [This](#associatedtype.This)<'from\_arg> = [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### type [This](#associatedtype.This)<'from\_arg> = [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [from\_arg](#tymethod.from_arg)(arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### type [This](#associatedtype.This)<'from\_arg> = [Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#239)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<T>

where T: [Array](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html "trait smallvec::Array") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), <T as [Array](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html "trait smallvec::Array")\>::[Item](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html#associatedtype.Item "type smallvec::Array::Item"): [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#239)

#### type [This](#associatedtype.This)<'from\_arg> = [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#239)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#20)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where T: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#20)

#### type [This](#associatedtype.This)<'from\_arg> = [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#20)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### type [This](#associatedtype.This)<'from\_arg> = [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#12-17)

### impl<V, S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<V, S>

where V: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#12-17)

#### type [This](#associatedtype.This)<'from\_arg> = [HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<V, S>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#12-17)

#### fn [from\_arg](#tymethod.from_arg)( arg: [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<V, S> as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

## Implementors

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#62)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Aabb](../../../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#62)

#### type [This](#associatedtype.This)<'from\_arg> = [Aabb](../../../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#42)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Aabb2d](../../../math/bounding/struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#42)

#### type [This](#associatedtype.This)<'from\_arg> = [Aabb2d](../../../math/bounding/struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#48)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Aabb3d](../../../math/bounding/struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#48)

#### type [This](#associatedtype.This)<'from\_arg> = [Aabb3d](../../../math/bounding/struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#112)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AabbCast2d](../../../math/bounding/struct.AabbCast2d.html "struct bevy::math::bounding::AabbCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#112)

#### type [This](#associatedtype.This)<'from\_arg> = [AabbCast2d](../../../math/bounding/struct.AabbCast2d.html "struct bevy::math::bounding::AabbCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#109)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AabbCast3d](../../../math/bounding/struct.AabbCast3d.html "struct bevy::math::bounding::AabbCast3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#109)

#### type [This](#associatedtype.This)<'from\_arg> = [AabbCast3d](../../../math/bounding/struct.AabbCast3d.html "struct bevy::math::bounding::AabbCast3d")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#43)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AabbGizmoConfigGroup](../../../prelude/struct.AabbGizmoConfigGroup.html "struct bevy::prelude::AabbGizmoConfigGroup")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#43)

#### type [This](#associatedtype.This)<'from\_arg> = [AabbGizmoConfigGroup](../../../prelude/struct.AabbGizmoConfigGroup.html "struct bevy::prelude::AabbGizmoConfigGroup")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#110)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AccessibilityRequested](../../../a11y/struct.AccessibilityRequested.html "struct bevy::a11y::AccessibilityRequested")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#110)

#### type [This](#associatedtype.This)<'from\_arg> = [AccessibilityRequested](../../../a11y/struct.AccessibilityRequested.html "struct bevy::a11y::AccessibilityRequested")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#251)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AccessibilitySystems](../../../a11y/enum.AccessibilitySystems.html "enum bevy::a11y::AccessibilitySystems")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#251)

#### type [This](#associatedtype.This)<'from\_arg> = [AccessibilitySystems](../../../a11y/enum.AccessibilitySystems.html "enum bevy::a11y::AccessibilitySystems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/accessibility.rs.html#185)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AccessibleLabel](../../../prelude/struct.AccessibleLabel.html "struct bevy::prelude::AccessibleLabel")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/accessibility.rs.html#185)

#### type [This](#associatedtype.This)<'from\_arg> = [AccessibleLabel](../../../prelude/struct.AccessibleLabel.html "struct bevy::prelude::AccessibleLabel")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#210)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AccumulatedMouseMotion](../../../input/mouse/struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#210)

#### type [This](#associatedtype.This)<'from\_arg> = [AccumulatedMouseMotion](../../../input/mouse/struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#231)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AccumulatedMouseScroll](../../../input/mouse/struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#231)

#### type [This](#associatedtype.This)<'from\_arg> = [AccumulatedMouseScroll](../../../input/mouse/struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#208)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AcquireFocus](../../../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#208)

#### type [This](#associatedtype.This)<'from\_arg> = [AcquireFocus](../../../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#80)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Activate](../../../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#80)

#### type [This](#associatedtype.This)<'from\_arg> = [Activate](../../../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#33)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ActivateOnPress](../../../ui_widgets/struct.ActivateOnPress.html "struct bevy::ui_widgets::ActivateOnPress")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#33)

#### type [This](#associatedtype.This)<'from\_arg> = [ActivateOnPress](../../../ui_widgets/struct.ActivateOnPress.html "struct bevy::ui_widgets::ActivateOnPress")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ActiveAnimation](../../../animation/struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### type [This](#associatedtype.This)<'from\_arg> = [ActiveAnimation](../../../animation/struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#49)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ActiveDescendant](../../../ui_widgets/struct.ActiveDescendant.html "struct bevy::ui_widgets::ActiveDescendant")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#49)

#### type [This](#associatedtype.This)<'from\_arg> = [ActiveDescendant](../../../ui_widgets/struct.ActiveDescendant.html "struct bevy::ui_widgets::ActiveDescendant")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#333)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Add](../../../prelude/struct.Add.html "struct bevy::prelude::Add")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#333)

#### type [This](#associatedtype.This)<'from\_arg> = [Add](../../../prelude/struct.Add.html "struct bevy::prelude::Add")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Affine2](../../../math/struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### type [This](#associatedtype.This)<'from\_arg> = [Affine2](../../../math/struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#416-423)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Affine3](../../../math/struct.Affine3.html "struct bevy::math::Affine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#416-423)

#### type [This](#associatedtype.This)<'from\_arg> = [Affine3](../../../math/struct.Affine3.html "struct bevy::math::Affine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#424-431)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Affine3A](../../../math/struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#424-431)

#### type [This](#associatedtype.This)<'from\_arg> = [Affine3A](../../../math/struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1055)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AlignContent](../../../prelude/enum.AlignContent.html "enum bevy::prelude::AlignContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1055)

#### type [This](#associatedtype.This)<'from\_arg> = [AlignContent](../../../prelude/enum.AlignContent.html "enum bevy::prelude::AlignContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#895)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AlignItems](../../../prelude/enum.AlignItems.html "enum bevy::prelude::AlignItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#895)

#### type [This](#associatedtype.This)<'from\_arg> = [AlignItems](../../../prelude/enum.AlignItems.html "enum bevy::prelude::AlignItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#975)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AlignSelf](../../../prelude/enum.AlignSelf.html "enum bevy::prelude::AlignSelf")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#975)

#### type [This](#associatedtype.This)<'from\_arg> = [AlignSelf](../../../prelude/enum.AlignSelf.html "enum bevy::prelude::AlignSelf")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/alpha.rs.html#7)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AlphaMode](../../../prelude/enum.AlphaMode.html "enum bevy::prelude::AlphaMode")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/alpha.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [AlphaMode](../../../prelude/enum.AlphaMode.html "enum bevy::prelude::AlphaMode")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#245)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AlphaMode2d](../../../sprite_render/enum.AlphaMode2d.html "enum bevy::sprite_render::AlphaMode2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#245)

#### type [This](#associatedtype.This)<'from\_arg> = [AlphaMode2d](../../../sprite_render/enum.AlphaMode2d.html "enum bevy::sprite_render::AlphaMode2d")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#9)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AmbientLight](../../../prelude/struct.AmbientLight.html "struct bevy::prelude::AmbientLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#9)

#### type [This](#associatedtype.This)<'from\_arg> = [AmbientLight](../../../prelude/struct.AmbientLight.html "struct bevy::prelude::AmbientLight")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#254)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Anchor](../../../sprite/struct.Anchor.html "struct bevy::sprite::Anchor")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#254)

#### type [This](#associatedtype.This)<'from\_arg> = [Anchor](../../../sprite/struct.Anchor.html "struct bevy::sprite::Anchor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#113)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AngularColorStop](../../../prelude/struct.AngularColorStop.html "struct bevy::prelude::AngularColorStop")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#113)

#### type [This](#associatedtype.This)<'from\_arg> = [AngularColorStop](../../../prelude/struct.AngularColorStop.html "struct bevy::prelude::AngularColorStop")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#213)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimatedBy](../../../animation/struct.AnimatedBy.html "struct bevy::animation::AnimatedBy")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#213)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimatedBy](../../../animation/struct.AnimatedBy.html "struct bevy::animation::AnimatedBy")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#103)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimationClip](../../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#103)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimationClip](../../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#112)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimationGraph](../../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#112)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimationGraph](../../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#135)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimationGraphHandle](../../../prelude/struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#135)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimationGraphHandle](../../../prelude/struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#169)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimationGraphNode](../../../prelude/struct.AnimationGraphNode.html "struct bevy::prelude::AnimationGraphNode")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#169)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimationGraphNode](../../../prelude/struct.AnimationGraphNode.html "struct bevy::prelude::AnimationGraphNode")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#211)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimationNodeType](../../../prelude/enum.AnimationNodeType.html "enum bevy::prelude::AnimationNodeType")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#211)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimationNodeType](../../../prelude/enum.AnimationNodeType.html "enum bevy::prelude::AnimationNodeType")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#730)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimationPlayer](../../../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#730)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimationPlayer](../../../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#184)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimationTargetId](../../../animation/struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#184)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimationTargetId](../../../animation/struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#54)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimationTransition](../../../prelude/struct.AnimationTransition.html "struct bevy::prelude::AnimationTransition")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#54)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimationTransition](../../../prelude/struct.AnimationTransition.html "struct bevy::prelude::AnimationTransition")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#31)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimationTransitions](../../../prelude/struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#31)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimationTransitions](../../../prelude/struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#955)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Annulus](../../../prelude/struct.Annulus.html "struct bevy::prelude::Annulus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#955)

#### type [This](#associatedtype.This)<'from\_arg> = [Annulus](../../../prelude/struct.Annulus.html "struct bevy::prelude::Annulus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#745)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnnulusMeshBuilder](../../../mesh/struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#745)

#### type [This](#associatedtype.This)<'from\_arg> = [AnnulusMeshBuilder](../../../mesh/struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1565)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AppExit](../../../prelude/enum.AppExit.html "enum bevy::prelude::AppExit")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1565)

#### type [This](#associatedtype.This)<'from\_arg> = [AppExit](../../../prelude/enum.AppExit.html "enum bevy::prelude::AppExit")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#453)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AppLifecycle](../../../window/enum.AppLifecycle.html "enum bevy::window::AppLifecycle")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#453)

#### type [This](#associatedtype.This)<'from\_arg> = [AppLifecycle](../../../window/enum.AppLifecycle.html "enum bevy::window::AppLifecycle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#117)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Arc2d](../../../prelude/struct.Arc2d.html "struct bevy::prelude::Arc2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#117)

#### type [This](#associatedtype.This)<'from\_arg> = [Arc2d](../../../prelude/struct.Arc2d.html "struct bevy::prelude::Arc2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/aspect_ratio.rs.html#14)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AspectRatio](../../../math/struct.AspectRatio.html "struct bevy::math::AspectRatio")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/aspect_ratio.rs.html#14)

#### type [This](#associatedtype.This)<'from\_arg> = [AspectRatio](../../../math/struct.AspectRatio.html "struct bevy::math::AspectRatio")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AssetIndex](../../../asset/struct.AssetIndex.html "struct bevy::asset::AssetIndex")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [AssetIndex](../../../asset/struct.AssetIndex.html "struct bevy::asset::AssetIndex")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#414)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AtmosphereMode](../../../pbr/enum.AtmosphereMode.html "enum bevy::pbr::AtmosphereMode")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#414)

#### type [This](#associatedtype.This)<'from\_arg> = [AtmosphereMode](../../../pbr/enum.AtmosphereMode.html "enum bevy::pbr::AtmosphereMode")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#286)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AtmosphereSettings](../../../pbr/struct.AtmosphereSettings.html "struct bevy::pbr::AtmosphereSettings")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#286)

#### type [This](#associatedtype.This)<'from\_arg> = [AtmosphereSettings](../../../pbr/struct.AtmosphereSettings.html "struct bevy::pbr::AtmosphereSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/auto_directional_navigation.rs.html#105)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AutoDirectionalNavigation](../../../ui/auto_directional_navigation/struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/auto_directional_navigation.rs.html#105)

#### type [This](#associatedtype.This)<'from\_arg> = [AutoDirectionalNavigation](../../../ui/auto_directional_navigation/struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AutoExposure](../../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

#### type [This](#associatedtype.This)<'from\_arg> = [AutoExposure](../../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/compensation_curve.rs.html#20)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AutoExposureCompensationCurve](../../../post_process/auto_exposure/struct.AutoExposureCompensationCurve.html "struct bevy::post_process::auto_exposure::AutoExposureCompensationCurve")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/compensation_curve.rs.html#20)

#### type [This](#associatedtype.This)<'from\_arg> = [AutoExposureCompensationCurve](../../../post_process/auto_exposure/struct.AutoExposureCompensationCurve.html "struct bevy::post_process::auto_exposure::AutoExposureCompensationCurve")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/autofocus.rs.html#20)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AutoFocus](../../../input_focus/struct.AutoFocus.html "struct bevy::input_focus::AutoFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/autofocus.rs.html#20)

#### type [This](#associatedtype.This)<'from\_arg> = [AutoFocus](../../../input_focus/struct.AutoFocus.html "struct bevy::input_focus::AutoFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#90)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AutoNavigationConfig](../../../input_focus/directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#90)

#### type [This](#associatedtype.This)<'from\_arg> = [AutoNavigationConfig](../../../input_focus/directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#984)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AxisSettings](../../../input/gamepad/struct.AxisSettings.html "struct bevy::input::gamepad::AxisSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#984)

#### type [This](#associatedtype.This)<'from\_arg> = [AxisSettings](../../../input/gamepad/struct.AxisSettings.html "struct bevy::input::gamepad::AxisSettings")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#287-294)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BVec2](../../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#287-294)

#### type [This](#associatedtype.This)<'from\_arg> = [BVec2](../../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#295-303)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BVec3](../../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#295-303)

#### type [This](#associatedtype.This)<'from\_arg> = [BVec3](../../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#304-313)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BVec4](../../../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#304-313)

#### type [This](#associatedtype.This)<'from\_arg> = [BVec4](../../../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#502-508)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BVec3A](../../../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#502-508)

#### type [This](#associatedtype.This)<'from\_arg> = [BVec3A](../../../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#509-515)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BVec4A](../../../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#509-515)

#### type [This](#associatedtype.This)<'from\_arg> = [BVec4A](../../../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#95)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Backfaces](../../../picking/mesh_picking/ray_cast/enum.Backfaces.html "enum bevy::picking::mesh_picking::ray_cast::Backfaces")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#95)

#### type [This](#associatedtype.This)<'from\_arg> = [Backfaces](../../../picking/mesh_picking/ray_cast/enum.Backfaces.html "enum bevy::picking::mesh_picking::ray_cast::Backfaces")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2222)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BackgroundColor](../../../prelude/struct.BackgroundColor.html "struct bevy::prelude::BackgroundColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2222)

#### type [This](#associatedtype.This)<'from\_arg> = [BackgroundColor](../../../prelude/struct.BackgroundColor.html "struct bevy::prelude::BackgroundColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#526)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BackgroundGradient](../../../prelude/struct.BackgroundGradient.html "struct bevy::prelude::BackgroundGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#526)

#### type [This](#associatedtype.This)<'from\_arg> = [BackgroundGradient](../../../prelude/struct.BackgroundGradient.html "struct bevy::prelude::BackgroundGradient")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/wgpu_types.rs.html#11-18)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BlendState](../../../render/render_resource/struct.BlendState.html "struct bevy::render::render_resource::BlendState")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/wgpu_types.rs.html#11-18)

#### type [This](#associatedtype.This)<'from\_arg> = [BlendState](../../../render/render_resource/struct.BlendState.html "struct bevy::render::render_resource::BlendState")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#30)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Bloom](../../../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#30)

#### type [This](#associatedtype.This)<'from\_arg> = [Bloom](../../../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#216)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BloomCompositeMode](../../../post_process/bloom/enum.BloomCompositeMode.html "enum bevy::post_process::bloom::BloomCompositeMode")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#216)

#### type [This](#associatedtype.This)<'from\_arg> = [BloomCompositeMode](../../../post_process/bloom/enum.BloomCompositeMode.html "enum bevy::post_process::bloom::BloomCompositeMode")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#199)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BloomPrefilter](../../../post_process/bloom/struct.BloomPrefilter.html "struct bevy::post_process::bloom::BloomPrefilter")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#199)

#### type [This](#associatedtype.This)<'from\_arg> = [BloomPrefilter](../../../post_process/bloom/struct.BloomPrefilter.html "struct bevy::post_process::bloom::BloomPrefilter")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2249)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BorderColor](../../../prelude/struct.BorderColor.html "struct bevy::prelude::BorderColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2249)

#### type [This](#associatedtype.This)<'from\_arg> = [BorderColor](../../../prelude/struct.BorderColor.html "struct bevy::prelude::BorderColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#542)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BorderGradient](../../../prelude/struct.BorderGradient.html "struct bevy::prelude::BorderGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#542)

#### type [This](#associatedtype.This)<'from\_arg> = [BorderGradient](../../../prelude/struct.BorderGradient.html "struct bevy::prelude::BorderGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BorderRadius](../../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

#### type [This](#associatedtype.This)<'from\_arg> = [BorderRadius](../../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/border_rect.rs.html#8)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BorderRect](../../../prelude/struct.BorderRect.html "struct bevy::prelude::BorderRect")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/border_rect.rs.html#8)

#### type [This](#associatedtype.This)<'from\_arg> = [BorderRect](../../../prelude/struct.BorderRect.html "struct bevy::prelude::BorderRect")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#478)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BoundingCircle](../../../math/bounding/struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#478)

#### type [This](#associatedtype.This)<'from\_arg> = [BoundingCircle](../../../math/bounding/struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#150)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BoundingCircleCast](../../../math/bounding/struct.BoundingCircleCast.html "struct bevy::math::bounding::BoundingCircleCast")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#150)

#### type [This](#associatedtype.This)<'from\_arg> = [BoundingCircleCast](../../../math/bounding/struct.BoundingCircleCast.html "struct bevy::math::bounding::BoundingCircleCast")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#504)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BoundingSphere](../../../math/bounding/struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#504)

#### type [This](#associatedtype.This)<'from\_arg> = [BoundingSphere](../../../math/bounding/struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#154)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BoundingSphereCast](../../../math/bounding/struct.BoundingSphereCast.html "struct bevy::math::bounding::BoundingSphereCast")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#154)

#### type [This](#associatedtype.This)<'from\_arg> = [BoundingSphereCast](../../../math/bounding/struct.BoundingSphereCast.html "struct bevy::math::bounding::BoundingSphereCast")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2831)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BoxShadow](../../../prelude/struct.BoxShadow.html "struct bevy::prelude::BoxShadow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2831)

#### type [This](#associatedtype.This)<'from\_arg> = [BoxShadow](../../../prelude/struct.BoxShadow.html "struct bevy::prelude::BoxShadow")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#186)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BoxShadowSamples](../../../prelude/struct.BoxShadowSamples.html "struct bevy::prelude::BoxShadowSamples")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#186)

#### type [This](#associatedtype.This)<'from\_arg> = [BoxShadowSamples](../../../prelude/struct.BoxShadowSamples.html "struct bevy::prelude::BoxShadowSamples")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1181)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [BoxSizing](../../../prelude/enum.BoxSizing.html "enum bevy::prelude::BoxSizing")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1181)

#### type [This](#associatedtype.This)<'from\_arg> = [BoxSizing](../../../prelude/enum.BoxSizing.html "enum bevy::prelude::BoxSizing")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/button.rs.html#6)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for bevy::prelude::[Button](../../../prelude/struct.Button.html "struct bevy::prelude::Button")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/button.rs.html#6)

#### type [This](#associatedtype.This)<'from\_arg> = [Button](../../../prelude/struct.Button.html "struct bevy::prelude::Button")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#27)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for bevy::ui\_widgets::[Button](../../../ui_widgets/struct.Button.html "struct bevy::ui_widgets::Button")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#27)

#### type [This](#associatedtype.This)<'from\_arg> = [Button](../../../ui_widgets/struct.Button.html "struct bevy::ui_widgets::Button")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1412)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ButtonAxisSettings](../../../input/gamepad/struct.ButtonAxisSettings.html "struct bevy::input::gamepad::ButtonAxisSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1412)

#### type [This](#associatedtype.This)<'from\_arg> = [ButtonAxisSettings](../../../input/gamepad/struct.ButtonAxisSettings.html "struct bevy::input::gamepad::ButtonAxisSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#820)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ButtonSettings](../../../input/gamepad/struct.ButtonSettings.html "struct bevy::input::gamepad::ButtonSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#820)

#### type [This](#associatedtype.This)<'from\_arg> = [ButtonSettings](../../../input/gamepad/struct.ButtonSettings.html "struct bevy::input::gamepad::ButtonSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#172)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ButtonState](../../../input/enum.ButtonState.html "enum bevy::input::ButtonState")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#172)

#### type [This](#associatedtype.This)<'from\_arg> = [ButtonState](../../../input/enum.ButtonState.html "enum bevy::input::ButtonState")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#34)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ButtonVariant](../../../feathers/controls/enum.ButtonVariant.html "enum bevy::feathers::controls::ButtonVariant")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#34)

#### type [This](#associatedtype.This)<'from\_arg> = [ButtonVariant](../../../feathers/controls/enum.ButtonVariant.html "enum bevy::feathers::controls::ButtonVariant")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2407)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CalculatedClip](../../../prelude/struct.CalculatedClip.html "struct bevy::prelude::CalculatedClip")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2407)

#### type [This](#associatedtype.This)<'from\_arg> = [CalculatedClip](../../../prelude/struct.CalculatedClip.html "struct bevy::prelude::CalculatedClip")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#374)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Camera](../../../prelude/struct.Camera.html "struct bevy::prelude::Camera")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#374)

#### type [This](#associatedtype.This)<'from\_arg> = [Camera](../../../prelude/struct.Camera.html "struct bevy::prelude::Camera")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#9)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Camera2d](../../../prelude/struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#9)

#### type [This](#associatedtype.This)<'from\_arg> = [Camera2d](../../../prelude/struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Camera3d](../../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [Camera3d](../../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#58)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Camera3dDepthLoadOp](../../../camera/enum.Camera3dDepthLoadOp.html "enum bevy::camera::Camera3dDepthLoadOp")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#58)

#### type [This](#associatedtype.This)<'from\_arg> = [Camera3dDepthLoadOp](../../../camera/enum.Camera3dDepthLoadOp.html "enum bevy::camera::Camera3dDepthLoadOp")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#41)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Camera3dDepthTextureUsage](../../../camera/struct.Camera3dDepthTextureUsage.html "struct bevy::camera::Camera3dDepthTextureUsage")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#41)

#### type [This](#associatedtype.This)<'from\_arg> = [Camera3dDepthTextureUsage](../../../camera/struct.Camera3dDepthTextureUsage.html "struct bevy::camera::Camera3dDepthTextureUsage")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#1044)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CameraMainTextureUsages](../../../camera/struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#1044)

#### type [This](#associatedtype.This)<'from\_arg> = [CameraMainTextureUsages](../../../camera/struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#860)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CameraOutputMode](../../../camera/enum.CameraOutputMode.html "enum bevy::camera::CameraOutputMode")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#860)

#### type [This](#associatedtype.This)<'from\_arg> = [CameraOutputMode](../../../camera/enum.CameraOutputMode.html "enum bevy::camera::CameraOutputMode")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#176)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CameraRenderGraph](../../../render/camera/struct.CameraRenderGraph.html "struct bevy::render::camera::CameraRenderGraph")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#176)

#### type [This](#associatedtype.This)<'from\_arg> = [CameraRenderGraph](../../../render/camera/struct.CameraRenderGraph.html "struct bevy::render::camera::CameraRenderGraph")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#178)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Cancel](../../../prelude/struct.Cancel.html "struct bevy::prelude::Cancel")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#178)

#### type [This](#associatedtype.This)<'from\_arg> = [Cancel](../../../prelude/struct.Cancel.html "struct bevy::prelude::Cancel")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2183)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Capsule2d](../../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2183)

#### type [This](#associatedtype.This)<'from\_arg> = [Capsule2d](../../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1121)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Capsule2dMeshBuilder](../../../mesh/struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1121)

#### type [This](#associatedtype.This)<'from\_arg> = [Capsule2dMeshBuilder](../../../mesh/struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#856)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Capsule3d](../../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#856)

#### type [This](#associatedtype.This)<'from\_arg> = [Capsule3d](../../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Capsule3dMeshBuilder](../../../mesh/struct.Capsule3dMeshBuilder.html "struct bevy::mesh::Capsule3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [Capsule3dMeshBuilder](../../../mesh/struct.Capsule3dMeshBuilder.html "struct bevy::mesh::Capsule3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#7)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CapsuleUvProfile](../../../mesh/enum.CapsuleUvProfile.html "enum bevy::mesh::CapsuleUvProfile")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [CapsuleUvProfile](../../../mesh/enum.CapsuleUvProfile.html "enum bevy::mesh::CapsuleUvProfile")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#179)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Cascade](../../../light/cascade/struct.Cascade.html "struct bevy::light::cascade::Cascade")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#179)

#### type [This](#associatedtype.This)<'from\_arg> = [Cascade](../../../light/cascade/struct.Cascade.html "struct bevy::light::cascade::Cascade")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#24)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CascadeShadowConfig](../../../light/struct.CascadeShadowConfig.html "struct bevy::light::CascadeShadowConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#24)

#### type [This](#associatedtype.This)<'from\_arg> = [CascadeShadowConfig](../../../light/struct.CascadeShadowConfig.html "struct bevy::light::CascadeShadowConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#167)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Cascades](../../../light/struct.Cascades.html "struct bevy::light::Cascades")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#167)

#### type [This](#associatedtype.This)<'from\_arg> = [Cascades](../../../light/struct.Cascades.html "struct bevy::light::Cascades")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#443)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CascadesFrusta](../../../camera/primitives/struct.CascadesFrusta.html "struct bevy::camera::primitives::CascadesFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#443)

#### type [This](#associatedtype.This)<'from\_arg> = [CascadesFrusta](../../../camera/primitives/struct.CascadesFrusta.html "struct bevy::camera::primitives::CascadesFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#460)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CascadesVisibleEntities](../../../camera/visibility/struct.CascadesVisibleEntities.html "struct bevy::camera::visibility::CascadesVisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#460)

#### type [This](#associatedtype.This)<'from\_arg> = [CascadesVisibleEntities](../../../camera/visibility/struct.CascadesVisibleEntities.html "struct bevy::camera::visibility::CascadesVisibleEntities")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#49)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Checkable](../../../ui/struct.Checkable.html "struct bevy::ui::Checkable")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#49)

#### type [This](#associatedtype.This)<'from\_arg> = [Checkable](../../../ui/struct.Checkable.html "struct bevy::ui::Checkable")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#36)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Checkbox](../../../ui_widgets/struct.Checkbox.html "struct bevy::ui_widgets::Checkbox")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#36)

#### type [This](#associatedtype.This)<'from\_arg> = [Checkbox](../../../ui_widgets/struct.Checkbox.html "struct bevy::ui_widgets::Checkbox")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#54)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Checked](../../../ui/struct.Checked.html "struct bevy::ui::Checked")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#54)

#### type [This](#associatedtype.This)<'from\_arg> = [Checked](../../../ui/struct.Checked.html "struct bevy::ui::Checked")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#95)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ChildOf](../../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#95)

#### type [This](#associatedtype.This)<'from\_arg> = [ChildOf](../../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#149)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Children](../../../prelude/struct.Children.html "struct bevy::prelude::Children")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#149)

#### type [This](#associatedtype.This)<'from\_arg> = [Children](../../../prelude/struct.Children.html "struct bevy::prelude::Children")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#43)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ChromaticAberration](../../../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#43)

#### type [This](#associatedtype.This)<'from\_arg> = [ChromaticAberration](../../../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#29)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Circle](../../../prelude/struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#29)

#### type [This](#associatedtype.This)<'from\_arg> = [Circle](../../../prelude/struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CircleMeshBuilder](../../../mesh/struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [CircleMeshBuilder](../../../mesh/struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#106)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CircularMeshUvMode](../../../mesh/enum.CircularMeshUvMode.html "enum bevy::mesh::CircularMeshUvMode")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#106)

#### type [This](#associatedtype.This)<'from\_arg> = [CircularMeshUvMode](../../../mesh/enum.CircularMeshUvMode.html "enum bevy::mesh::CircularMeshUvMode")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#285)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CircularSector](../../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#285)

#### type [This](#associatedtype.This)<'from\_arg> = [CircularSector](../../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#128)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CircularSectorMeshBuilder](../../../mesh/struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#128)

#### type [This](#associatedtype.This)<'from\_arg> = [CircularSectorMeshBuilder](../../../mesh/struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#437)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CircularSegment](../../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#437)

#### type [This](#associatedtype.This)<'from\_arg> = [CircularSegment](../../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#266)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CircularSegmentMeshBuilder](../../../mesh/struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#266)

#### type [This](#associatedtype.This)<'from\_arg> = [CircularSegmentMeshBuilder](../../../mesh/struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#53)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ClearColor](../../../prelude/struct.ClearColor.html "struct bevy::prelude::ClearColor")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#53)

#### type [This](#associatedtype.This)<'from\_arg> = [ClearColor](../../../prelude/struct.ClearColor.html "struct bevy::prelude::ClearColor")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#11)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ClearColorConfig](../../../prelude/enum.ClearColorConfig.html "enum bevy::prelude::ClearColorConfig")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#11)

#### type [This](#associatedtype.This)<'from\_arg> = [ClearColorConfig](../../../prelude/enum.ClearColorConfig.html "enum bevy::prelude::ClearColorConfig")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#309)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Click](../../../prelude/struct.Click.html "struct bevy::prelude::Click")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#309)

#### type [This](#associatedtype.This)<'from\_arg> = [Click](../../../prelude/struct.Click.html "struct bevy::prelude::Click")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#105)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ClusterConfig](../../../light/cluster/enum.ClusterConfig.html "enum bevy::light::cluster::ClusterConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#105)

#### type [This](#associatedtype.This)<'from\_arg> = [ClusterConfig](../../../light/cluster/enum.ClusterConfig.html "enum bevy::light::cluster::ClusterConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#82)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ClusterFarZMode](../../../light/cluster/enum.ClusterFarZMode.html "enum bevy::light::cluster::ClusterFarZMode")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#82)

#### type [This](#associatedtype.This)<'from\_arg> = [ClusterFarZMode](../../../light/cluster/enum.ClusterFarZMode.html "enum bevy::light::cluster::ClusterFarZMode")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#95)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ClusterZConfig](../../../light/cluster/struct.ClusterZConfig.html "struct bevy::light::cluster::ClusterZConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#95)

#### type [This](#associatedtype.This)<'from\_arg> = [ClusterZConfig](../../../light/cluster/struct.ClusterZConfig.html "struct bevy::light::cluster::ClusterZConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#229)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ClusteredDecal](../../../light/struct.ClusteredDecal.html "struct bevy::light::ClusteredDecal")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#229)

#### type [This](#associatedtype.This)<'from\_arg> = [ClusteredDecal](../../../light/struct.ClusteredDecal.html "struct bevy::light::ClusteredDecal")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Color](../../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### type [This](#associatedtype.This)<'from\_arg> = [Color](../../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#47)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorChannel](../../../feathers/controls/enum.ColorChannel.html "enum bevy::feathers::controls::ColorChannel")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#47)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorChannel](../../../feathers/controls/enum.ColorChannel.html "enum bevy::feathers::controls::ColorChannel")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#399)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorGrading](../../../render/view/struct.ColorGrading.html "struct bevy::render::view::ColorGrading")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#399)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorGrading](../../../render/view/struct.ColorGrading.html "struct bevy::render::view::ColorGrading")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#428)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorGradingGlobal](../../../render/view/struct.ColorGradingGlobal.html "struct bevy::render::view::ColorGradingGlobal")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#428)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorGradingGlobal](../../../render/view/struct.ColorGradingGlobal.html "struct bevy::render::view::ColorGradingGlobal")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#494)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorGradingSection](../../../render/view/struct.ColorGradingSection.html "struct bevy::render::view::ColorGradingSection")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#494)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorGradingSection](../../../render/view/struct.ColorGradingSection.html "struct bevy::render::view::ColorGradingSection")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorMaterial](../../../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorMaterial](../../../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#68)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorPlaneValue](../../../feathers/controls/struct.ColorPlaneValue.html "struct bevy::feathers::controls::ColorPlaneValue")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#68)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorPlaneValue](../../../feathers/controls/struct.ColorPlaneValue.html "struct bevy::feathers::controls::ColorPlaneValue")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#187)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorSlider](../../../feathers/controls/struct.ColorSlider.html "struct bevy::feathers::controls::ColorSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#187)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorSlider](../../../feathers/controls/struct.ColorSlider.html "struct bevy::feathers::controls::ColorSlider")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#10)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorStop](../../../prelude/struct.ColorStop.html "struct bevy::prelude::ColorStop")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#10)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorStop](../../../prelude/struct.ColorStop.html "struct bevy::prelude::ColorStop")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#40)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorSwatchFg](../../../feathers/controls/struct.ColorSwatchFg.html "struct bevy::feathers::controls::ColorSwatchFg")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#40)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorSwatchFg](../../../feathers/controls/struct.ColorSwatchFg.html "struct bevy::feathers::controls::ColorSwatchFg")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#33)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorSwatchValue](../../../feathers/controls/struct.ColorSwatchValue.html "struct bevy::feathers::controls::ColorSwatchValue")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#33)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorSwatchValue](../../../feathers/controls/struct.ColorSwatchValue.html "struct bevy::feathers::controls::ColorSwatchValue")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#132)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CompassOctant](../../../math/enum.CompassOctant.html "enum bevy::math::CompassOctant")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#132)

#### type [This](#associatedtype.This)<'from\_arg> = [CompassOctant](../../../math/enum.CompassOctant.html "enum bevy::math::CompassOctant")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#25)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CompassQuadrant](../../../math/enum.CompassQuadrant.html "enum bevy::math::CompassQuadrant")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#25)

#### type [This](#associatedtype.This)<'from\_arg> = [CompassQuadrant](../../../math/enum.CompassQuadrant.html "enum bevy::math::CompassQuadrant")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/info.rs.html#178)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ComponentId](../../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/info.rs.html#178)

#### type [This](#associatedtype.This)<'from\_arg> = [ComponentId](../../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#136)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ComponentTicks](../../../ecs/change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#136)

#### type [This](#associatedtype.This)<'from\_arg> = [ComponentTicks](../../../ecs/change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1295)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CompositeAlphaMode](../../../window/enum.CompositeAlphaMode.html "enum bevy::window::CompositeAlphaMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1295)

#### type [This](#associatedtype.This)<'from\_arg> = [CompositeAlphaMode](../../../window/enum.CompositeAlphaMode.html "enum bevy::window::CompositeAlphaMode")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#92)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CompositingSpace](../../../prelude/enum.CompositingSpace.html "enum bevy::prelude::CompositingSpace")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#92)

#### type [This](#associatedtype.This)<'from\_arg> = [CompositingSpace](../../../prelude/enum.CompositingSpace.html "enum bevy::prelude::CompositingSpace")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#217)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ComputedCameraValues](../../../camera/struct.ComputedCameraValues.html "struct bevy::camera::ComputedCameraValues")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#217)

#### type [This](#associatedtype.This)<'from\_arg> = [ComputedCameraValues](../../../camera/struct.ComputedCameraValues.html "struct bevy::camera::ComputedCameraValues")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#26)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ComputedNode](../../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#26)

#### type [This](#associatedtype.This)<'from\_arg> = [ComputedNode](../../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#17)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ComputedStackIndex](../../../ui/struct.ComputedStackIndex.html "struct bevy::ui::ComputedStackIndex")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#17)

#### type [This](#associatedtype.This)<'from\_arg> = [ComputedStackIndex](../../../ui/struct.ComputedStackIndex.html "struct bevy::ui::ComputedStackIndex")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#37)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ComputedTextBlock](../../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#37)

#### type [This](#associatedtype.This)<'from\_arg> = [ComputedTextBlock](../../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3036)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ComputedUiRenderTargetInfo](../../../prelude/struct.ComputedUiRenderTargetInfo.html "struct bevy::prelude::ComputedUiRenderTargetInfo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3036)

#### type [This](#associatedtype.This)<'from\_arg> = [ComputedUiRenderTargetInfo](../../../prelude/struct.ComputedUiRenderTargetInfo.html "struct bevy::prelude::ComputedUiRenderTargetInfo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3014)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ComputedUiTargetCamera](../../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3014)

#### type [This](#associatedtype.This)<'from\_arg> = [ComputedUiTargetCamera](../../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#927)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Cone](../../../prelude/struct.Cone.html "struct bevy::prelude::Cone")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#927)

#### type [This](#associatedtype.This)<'from\_arg> = [Cone](../../../prelude/struct.Cone.html "struct bevy::prelude::Cone")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#7)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ConeAnchor](../../../mesh/enum.ConeAnchor.html "enum bevy::mesh::ConeAnchor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [ConeAnchor](../../../mesh/enum.ConeAnchor.html "enum bevy::mesh::ConeAnchor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#20)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ConeMeshBuilder](../../../mesh/struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#20)

#### type [This](#associatedtype.This)<'from\_arg> = [ConeMeshBuilder](../../../mesh/struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#410)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ConicGradient](../../../prelude/struct.ConicGradient.html "struct bevy::prelude::ConicGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#410)

#### type [This](#associatedtype.This)<'from\_arg> = [ConicGradient](../../../prelude/struct.ConicGradient.html "struct bevy::prelude::ConicGradient")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1010)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ConicalFrustum](../../../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1010)

#### type [This](#associatedtype.This)<'from\_arg> = [ConicalFrustum](../../../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/conical_frustum.rs.html#7)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ConicalFrustumMeshBuilder](../../../mesh/struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::ConicalFrustumMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/conical_frustum.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [ConicalFrustumMeshBuilder](../../../mesh/struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::ConicalFrustumMeshBuilder")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#34)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ContactShadows](../../../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#34)

#### type [This](#associatedtype.This)<'from\_arg> = [ContactShadows](../../../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/measurement.rs.html#139)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ContentSize](../../../ui/struct.ContentSize.html "struct bevy::ui::ContentSize")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/measurement.rs.html#139)

#### type [This](#associatedtype.This)<'from\_arg> = [ContentSize](../../../ui/struct.ContentSize.html "struct bevy::ui::ContentSize")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#37)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ContrastAdaptiveSharpening](../../../anti_alias/contrast_adaptive_sharpening/struct.ContrastAdaptiveSharpening.html "struct bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#37)

#### type [This](#associatedtype.This)<'from\_arg> = [ContrastAdaptiveSharpening](../../../anti_alias/contrast_adaptive_sharpening/struct.ContrastAdaptiveSharpening.html "struct bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#27)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ControlOrientation](../../../ui_widgets/enum.ControlOrientation.html "enum bevy::ui_widgets::ControlOrientation")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#27)

#### type [This](#associatedtype.This)<'from\_arg> = [ControlOrientation](../../../ui_widgets/enum.ControlOrientation.html "enum bevy::ui_widgets::ControlOrientation")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1950)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ConvexPolygon](../../../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1950)

#### type [This](#associatedtype.This)<'from\_arg> = [ConvexPolygon](../../../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#413)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ConvexPolygonMeshBuilder](../../../mesh/struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#413)

#### type [This](#associatedtype.This)<'from\_arg> = [ConvexPolygonMeshBuilder](../../../mesh/struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#392)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubemapFrusta](../../../camera/primitives/struct.CubemapFrusta.html "struct bevy::camera::primitives::CubemapFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#392)

#### type [This](#associatedtype.This)<'from\_arg> = [CubemapFrusta](../../../camera/primitives/struct.CubemapFrusta.html "struct bevy::camera::primitives::CubemapFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#408)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubemapLayout](../../../camera/primitives/enum.CubemapLayout.html "enum bevy::camera::primitives::CubemapLayout")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#408)

#### type [This](#associatedtype.This)<'from\_arg> = [CubemapLayout](../../../camera/primitives/enum.CubemapLayout.html "enum bevy::camera::primitives::CubemapLayout")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#435)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubemapVisibleEntities](../../../camera/visibility/struct.CubemapVisibleEntities.html "struct bevy::camera::visibility::CubemapVisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#435)

#### type [This](#associatedtype.This)<'from\_arg> = [CubemapVisibleEntities](../../../camera/visibility/struct.CubemapVisibleEntities.html "struct bevy::camera::visibility::CubemapVisibleEntities")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#113)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubicRotationCurve](../../../animation/gltf_curves/struct.CubicRotationCurve.html "struct bevy::animation::gltf_curves::CubicRotationCurve")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#113)

#### type [This](#associatedtype.This)<'from\_arg> = [CubicRotationCurve](../../../animation/gltf_curves/struct.CubicRotationCurve.html "struct bevy::animation::gltf_curves::CubicRotationCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#684)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Cuboid](../../../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#684)

#### type [This](#associatedtype.This)<'from\_arg> = [Cuboid](../../../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cuboid.rs.html#7)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CuboidMeshBuilder](../../../mesh/struct.CuboidMeshBuilder.html "struct bevy::mesh::CuboidMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cuboid.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [CuboidMeshBuilder](../../../mesh/struct.CuboidMeshBuilder.html "struct bevy::mesh::CuboidMeshBuilder")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#209)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CursorEntered](../../../prelude/struct.CursorEntered.html "struct bevy::prelude::CursorEntered")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#209)

#### type [This](#associatedtype.This)<'from\_arg> = [CursorEntered](../../../prelude/struct.CursorEntered.html "struct bevy::prelude::CursorEntered")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1076)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CursorGrabMode](../../../window/enum.CursorGrabMode.html "enum bevy::window::CursorGrabMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1076)

#### type [This](#associatedtype.This)<'from\_arg> = [CursorGrabMode](../../../window/enum.CursorGrabMode.html "enum bevy::window::CursorGrabMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/mod.rs.html#24)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CursorIcon](../../../window/enum.CursorIcon.html "enum bevy::window::CursorIcon")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/mod.rs.html#24)

#### type [This](#associatedtype.This)<'from\_arg> = [CursorIcon](../../../window/enum.CursorIcon.html "enum bevy::window::CursorIcon")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#226)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CursorLeft](../../../prelude/struct.CursorLeft.html "struct bevy::prelude::CursorLeft")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#226)

#### type [This](#associatedtype.This)<'from\_arg> = [CursorLeft](../../../prelude/struct.CursorLeft.html "struct bevy::prelude::CursorLeft")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#184)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CursorMoved](../../../prelude/struct.CursorMoved.html "struct bevy::prelude::CursorMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#184)

#### type [This](#associatedtype.This)<'from\_arg> = [CursorMoved](../../../prelude/struct.CursorMoved.html "struct bevy::prelude::CursorMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#744)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CursorOptions](../../../window/struct.CursorOptions.html "struct bevy::window::CursorOptions")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#744)

#### type [This](#associatedtype.This)<'from\_arg> = [CursorOptions](../../../window/struct.CursorOptions.html "struct bevy::window::CursorOptions")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#71)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CustomCursor](../../../window/enum.CustomCursor.html "enum bevy::window::CustomCursor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#71)

#### type [This](#associatedtype.This)<'from\_arg> = [CustomCursor](../../../window/enum.CustomCursor.html "enum bevy::window::CustomCursor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#15)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CustomCursorImage](../../../window/struct.CustomCursorImage.html "struct bevy::window::CustomCursorImage")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#15)

#### type [This](#associatedtype.This)<'from\_arg> = [CustomCursorImage](../../../window/struct.CustomCursorImage.html "struct bevy::window::CustomCursorImage")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#55)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CustomCursorUrl](../../../window/struct.CustomCursorUrl.html "struct bevy::window::CustomCursorUrl")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#55)

#### type [This](#associatedtype.This)<'from\_arg> = [CustomCursorUrl](../../../window/struct.CustomCursorUrl.html "struct bevy::window::CustomCursorUrl")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#109)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CustomProjection](../../../camera/struct.CustomProjection.html "struct bevy::camera::CustomProjection")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#109)

#### type [This](#associatedtype.This)<'from\_arg> = [CustomProjection](../../../camera/struct.CustomProjection.html "struct bevy::camera::CustomProjection")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#777)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Cylinder](../../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#777)

#### type [This](#associatedtype.This)<'from\_arg> = [Cylinder](../../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#7)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CylinderAnchor](../../../mesh/enum.CylinderAnchor.html "enum bevy::mesh::CylinderAnchor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [CylinderAnchor](../../../mesh/enum.CylinderAnchor.html "enum bevy::mesh::CylinderAnchor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#20)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CylinderMeshBuilder](../../../mesh/struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#20)

#### type [This](#associatedtype.This)<'from\_arg> = [CylinderMeshBuilder](../../../mesh/struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#433-440)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DAffine2](../../../math/struct.DAffine2.html "struct bevy::math::DAffine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#433-440)

#### type [This](#associatedtype.This)<'from\_arg> = [DAffine2](../../../math/struct.DAffine2.html "struct bevy::math::DAffine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#441-448)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DAffine3](../../../math/struct.DAffine3.html "struct bevy::math::DAffine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#441-448)

#### type [This](#associatedtype.This)<'from\_arg> = [DAffine3](../../../math/struct.DAffine3.html "struct bevy::math::DAffine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#380-387)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DMat2](../../../math/struct.DMat2.html "struct bevy::math::DMat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#380-387)

#### type [This](#associatedtype.This)<'from\_arg> = [DMat2](../../../math/struct.DMat2.html "struct bevy::math::DMat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DMat3](../../../math/struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### type [This](#associatedtype.This)<'from\_arg> = [DMat3](../../../math/struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#397-406)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DMat4](../../../math/struct.DMat4.html "struct bevy::math::DMat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#397-406)

#### type [This](#associatedtype.This)<'from\_arg> = [DMat4](../../../math/struct.DMat4.html "struct bevy::math::DMat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#460-469)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DQuat](../../../math/struct.DQuat.html "struct bevy::math::DQuat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#460-469)

#### type [This](#associatedtype.This)<'from\_arg> = [DQuat](../../../math/struct.DQuat.html "struct bevy::math::DQuat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#315-322)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DVec2](../../../math/struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#315-322)

#### type [This](#associatedtype.This)<'from\_arg> = [DVec2](../../../math/struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DVec3](../../../math/struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### type [This](#associatedtype.This)<'from\_arg> = [DVec3](../../../math/struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DVec4](../../../math/struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### type [This](#associatedtype.This)<'from\_arg> = [DVec4](../../../math/struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DebandDither](../../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

#### type [This](#associatedtype.This)<'from\_arg> = [DebandDither](../../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#23)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DefaultCursor](../../../feathers/cursor/struct.DefaultCursor.html "struct bevy::feathers::cursor::DefaultCursor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#23)

#### type [This](#associatedtype.This)<'from\_arg> = [DefaultCursor](../../../feathers/cursor/struct.DefaultCursor.html "struct bevy::feathers::cursor::DefaultCursor")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#84)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DefaultGizmoConfigGroup](../../../prelude/struct.DefaultGizmoConfigGroup.html "struct bevy::prelude::DefaultGizmoConfigGroup")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#84)

#### type [This](#associatedtype.This)<'from\_arg> = [DefaultGizmoConfigGroup](../../../prelude/struct.DefaultGizmoConfigGroup.html "struct bevy::prelude::DefaultGizmoConfigGroup")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1358)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DefaultOpaqueRendererMethod](../../../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1358)

#### type [This](#associatedtype.This)<'from\_arg> = [DefaultOpaqueRendererMethod](../../../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#172)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DefaultQueryFilters](../../../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#172)

#### type [This](#associatedtype.This)<'from\_arg> = [DefaultQueryFilters](../../../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#232)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DefaultSpatialScale](../../../audio/struct.DefaultSpatialScale.html "struct bevy::audio::DefaultSpatialScale")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#232)

#### type [This](#associatedtype.This)<'from\_arg> = [DefaultSpatialScale](../../../audio/struct.DefaultSpatialScale.html "struct bevy::audio::DefaultSpatialScale")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#82)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DeferredPrepass](../../../core_pipeline/prepass/struct.DeferredPrepass.html "struct bevy::core_pipeline::prepass::DeferredPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#82)

#### type [This](#associatedtype.This)<'from\_arg> = [DeferredPrepass](../../../core_pipeline/prepass/struct.DeferredPrepass.html "struct bevy::core_pipeline::prepass::DeferredPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#93)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DeferredPrepassDoubleBuffer](../../../core_pipeline/prepass/struct.DeferredPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DeferredPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#93)

#### type [This](#associatedtype.This)<'from\_arg> = [DeferredPrepassDoubleBuffer](../../../core_pipeline/prepass/struct.DeferredPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DeferredPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#133)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DelayedCommandQueue](../../../time/struct.DelayedCommandQueue.html "struct bevy::time::DelayedCommandQueue")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#133)

#### type [This](#associatedtype.This)<'from\_arg> = [DelayedCommandQueue](../../../time/struct.DelayedCommandQueue.html "struct bevy::time::DelayedCommandQueue")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#66)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DenoiseCas](../../../anti_alias/contrast_adaptive_sharpening/struct.DenoiseCas.html "struct bevy::anti_alias::contrast_adaptive_sharpening::DenoiseCas")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#66)

#### type [This](#associatedtype.This)<'from\_arg> = [DenoiseCas](../../../anti_alias/contrast_adaptive_sharpening/struct.DenoiseCas.html "struct bevy::anti_alias::contrast_adaptive_sharpening::DenoiseCas")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#75)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DepthOfField](../../../post_process/dof/struct.DepthOfField.html "struct bevy::post_process::dof::DepthOfField")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#75)

#### type [This](#associatedtype.This)<'from\_arg> = [DepthOfField](../../../post_process/dof/struct.DepthOfField.html "struct bevy::post_process::dof::DepthOfField")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#119)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DepthOfFieldMode](../../../post_process/dof/enum.DepthOfFieldMode.html "enum bevy::post_process::dof::DepthOfFieldMode")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#119)

#### type [This](#associatedtype.This)<'from\_arg> = [DepthOfFieldMode](../../../post_process/dof/enum.DepthOfFieldMode.html "enum bevy::post_process::dof::DepthOfFieldMode")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#62)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DepthPrepass](../../../core_pipeline/prepass/struct.DepthPrepass.html "struct bevy::core_pipeline::prepass::DepthPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#62)

#### type [This](#associatedtype.This)<'from\_arg> = [DepthPrepass](../../../core_pipeline/prepass/struct.DepthPrepass.html "struct bevy::core_pipeline::prepass::DepthPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#87)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DepthPrepassDoubleBuffer](../../../core_pipeline/prepass/struct.DepthPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DepthPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#87)

#### type [This](#associatedtype.This)<'from\_arg> = [DepthPrepassDoubleBuffer](../../../core_pipeline/prepass/struct.DepthPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DepthPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#388)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Despawn](../../../prelude/struct.Despawn.html "struct bevy::prelude::Despawn")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#388)

#### type [This](#associatedtype.This)<'from\_arg> = [Despawn](../../../prelude/struct.Despawn.html "struct bevy::prelude::Despawn")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#88)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Dir2](../../../prelude/struct.Dir2.html "struct bevy::prelude::Dir2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#88)

#### type [This](#associatedtype.This)<'from\_arg> = [Dir2](../../../prelude/struct.Dir2.html "struct bevy::prelude::Dir2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#399)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Dir3](../../../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#399)

#### type [This](#associatedtype.This)<'from\_arg> = [Dir3](../../../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1053)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Dir4](../../../math/struct.Dir4.html "struct bevy::math::Dir4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1053)

#### type [This](#associatedtype.This)<'from\_arg> = [Dir4](../../../math/struct.Dir4.html "struct bevy::math::Dir4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#803)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Dir3A](../../../prelude/struct.Dir3A.html "struct bevy::prelude::Dir3A")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#803)

#### type [This](#associatedtype.This)<'from\_arg> = [Dir3A](../../../prelude/struct.Dir3A.html "struct bevy::prelude::Dir3A")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#61)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DirectionalLight](../../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#61)

#### type [This](#associatedtype.This)<'from\_arg> = [DirectionalLight](../../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#191)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DirectionalLightShadowMap](../../../light/struct.DirectionalLightShadowMap.html "struct bevy::light::DirectionalLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#191)

#### type [This](#associatedtype.This)<'from\_arg> = [DirectionalLightShadowMap](../../../light/struct.DirectionalLightShadowMap.html "struct bevy::light::DirectionalLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#173)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DirectionalLightTexture](../../../light/struct.DirectionalLightTexture.html "struct bevy::light::DirectionalLightTexture")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#173)

#### type [This](#associatedtype.This)<'from\_arg> = [DirectionalLightTexture](../../../light/struct.DirectionalLightTexture.html "struct bevy::light::DirectionalLightTexture")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#251)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DirectionalNavigationMap](../../../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#251)

#### type [This](#associatedtype.This)<'from\_arg> = [DirectionalNavigationMap](../../../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#354)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DirectlyHovered](../../../picking/hover/struct.DirectlyHovered.html "struct bevy::picking::hover::DirectlyHovered")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#354)

#### type [This](#associatedtype.This)<'from\_arg> = [DirectlyHovered](../../../picking/hover/struct.DirectlyHovered.html "struct bevy::picking::hover::DirectlyHovered")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#131)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Disabled](../../../ecs/entity_disabling/struct.Disabled.html "struct bevy::ecs::entity_disabling::Disabled")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#131)

#### type [This](#associatedtype.This)<'from\_arg> = [Disabled](../../../ecs/entity_disabling/struct.Disabled.html "struct bevy::ecs::entity_disabling::Disabled")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#361)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Discard](../../../prelude/struct.Discard.html "struct bevy::prelude::Discard")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#361)

#### type [This](#associatedtype.This)<'from\_arg> = [Discard](../../../prelude/struct.Discard.html "struct bevy::prelude::Discard")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1147)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Display](../../../prelude/enum.Display.html "enum bevy::prelude::Display")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1147)

#### type [This](#associatedtype.This)<'from\_arg> = [Display](../../../prelude/enum.Display.html "enum bevy::prelude::Display")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DistanceFog](../../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

#### type [This](#associatedtype.This)<'from\_arg> = [DistanceFog](../../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#66)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DoubleTapGesture](../../../input/gestures/struct.DoubleTapGesture.html "struct bevy::input::gestures::DoubleTapGesture")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#66)

#### type [This](#associatedtype.This)<'from\_arg> = [DoubleTapGesture](../../../input/gestures/struct.DoubleTapGesture.html "struct bevy::input::gestures::DoubleTapGesture")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#348)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Drag](../../../prelude/struct.Drag.html "struct bevy::prelude::Drag")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#348)

#### type [This](#associatedtype.This)<'from\_arg> = [Drag](../../../prelude/struct.Drag.html "struct bevy::prelude::Drag")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#421)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DragDrop](../../../prelude/struct.DragDrop.html "struct bevy::prelude::DragDrop")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#421)

#### type [This](#associatedtype.This)<'from\_arg> = [DragDrop](../../../prelude/struct.DragDrop.html "struct bevy::prelude::DragDrop")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#370)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DragEnd](../../../prelude/struct.DragEnd.html "struct bevy::prelude::DragEnd")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#370)

#### type [This](#associatedtype.This)<'from\_arg> = [DragEnd](../../../prelude/struct.DragEnd.html "struct bevy::prelude::DragEnd")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#385)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DragEnter](../../../prelude/struct.DragEnter.html "struct bevy::prelude::DragEnter")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#385)

#### type [This](#associatedtype.This)<'from\_arg> = [DragEnter](../../../prelude/struct.DragEnter.html "struct bevy::prelude::DragEnter")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#433)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DragEntry](../../../prelude/struct.DragEntry.html "struct bevy::prelude::DragEntry")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#433)

#### type [This](#associatedtype.This)<'from\_arg> = [DragEntry](../../../prelude/struct.DragEntry.html "struct bevy::prelude::DragEntry")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#409)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DragLeave](../../../prelude/struct.DragLeave.html "struct bevy::prelude::DragLeave")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#409)

#### type [This](#associatedtype.This)<'from\_arg> = [DragLeave](../../../prelude/struct.DragLeave.html "struct bevy::prelude::DragLeave")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#397)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DragOver](../../../prelude/struct.DragOver.html "struct bevy::prelude::DragOver")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#397)

#### type [This](#associatedtype.This)<'from\_arg> = [DragOver](../../../prelude/struct.DragOver.html "struct bevy::prelude::DragOver")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#338)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DragStart](../../../prelude/struct.DragStart.html "struct bevy::prelude::DragStart")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#338)

#### type [This](#associatedtype.This)<'from\_arg> = [DragStart](../../../prelude/struct.DragStart.html "struct bevy::prelude::DragStart")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#329)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DynamicSkinnedMeshBounds](../../../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#329)

#### type [This](#associatedtype.This)<'from\_arg> = [DynamicSkinnedMeshBounds](../../../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#36)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DynamicWorldRoot](../../../prelude/struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#36)

#### type [This](#associatedtype.This)<'from\_arg> = [DynamicWorldRoot](../../../prelude/struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#431)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EaseFunction](../../../prelude/enum.EaseFunction.html "enum bevy::prelude::EaseFunction")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#431)

#### type [This](#associatedtype.This)<'from\_arg> = [EaseFunction](../../../prelude/enum.EaseFunction.html "enum bevy::prelude::EaseFunction")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#804)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Ellipse](../../../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#804)

#### type [This](#associatedtype.This)<'from\_arg> = [Ellipse](../../../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#556)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EllipseMeshBuilder](../../../mesh/struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#556)

#### type [This](#associatedtype.This)<'from\_arg> = [EllipseMeshBuilder](../../../mesh/struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1432)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EnabledButtons](../../../window/struct.EnabledButtons.html "struct bevy::window::EnabledButtons")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1432)

#### type [This](#associatedtype.This)<'from\_arg> = [EnabledButtons](../../../window/struct.EnabledButtons.html "struct bevy::window::EnabledButtons")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#223)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Enter](../../../prelude/struct.Enter.html "struct bevy::prelude::Enter")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#223)

#### type [This](#associatedtype.This)<'from\_arg> = [Enter](../../../prelude/struct.Enter.html "struct bevy::prelude::Enter")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#414)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#414)

#### type [This](#associatedtype.This)<'from\_arg> = [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#32)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityCursor](../../../feathers/cursor/enum.EntityCursor.html "enum bevy::feathers::cursor::EntityCursor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#32)

#### type [This](#associatedtype.This)<'from\_arg> = [EntityCursor](../../../feathers/cursor/enum.EntityCursor.html "enum bevy::feathers::cursor::EntityCursor")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#248)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityGeneration](../../../ecs/entity/struct.EntityGeneration.html "struct bevy::ecs::entity::EntityGeneration")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#248)

#### type [This](#associatedtype.This)<'from\_arg> = [EntityGeneration](../../../ecs/entity/struct.EntityGeneration.html "struct bevy::ecs::entity::EntityGeneration")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash.rs.html#8)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityHash](../../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash.rs.html#8)

#### type [This](#associatedtype.This)<'from\_arg> = [EntityHash](../../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityHashSet](../../../ecs/entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [EntityHashSet](../../../ecs/entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#147)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityIndex](../../../ecs/entity/struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#147)

#### type [This](#associatedtype.This)<'from\_arg> = [EntityIndex](../../../ecs/entity/struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#29)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityIndexSet](../../../ecs/entity/struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#29)

#### type [This](#associatedtype.This)<'from\_arg> = [EntityIndexSet](../../../ecs/entity/struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#105)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EnvironmentMapLight](../../../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#105)

#### type [This](#associatedtype.This)<'from\_arg> = [EnvironmentMapLight](../../../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#90)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ErasedGizmoConfigGroup](../../../gizmos/config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#90)

#### type [This](#associatedtype.This)<'from\_arg> = [ErasedGizmoConfigGroup](../../../gizmos/config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#471-500)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EulerRot](../../../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#471-500)

#### type [This](#associatedtype.This)<'from\_arg> = [EulerRot](../../../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#229)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Exposure](../../../camera/struct.Exposure.html "struct bevy::camera::Exposure")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#229)

#### type [This](#associatedtype.This)<'from\_arg> = [Exposure](../../../camera/struct.Exposure.html "struct bevy::camera::Exposure")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#59)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersButton](../../../feathers/controls/struct.FeathersButton.html "struct bevy::feathers::controls::FeathersButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#59)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersButton](../../../feathers/controls/struct.FeathersButton.html "struct bevy::feathers::controls::FeathersButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#48)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersCheckbox](../../../feathers/controls/struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#48)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersCheckbox](../../../feathers/controls/struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersColorPlane](../../../feathers/controls/enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersColorPlane](../../../feathers/controls/enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#162)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersColorSlider](../../../feathers/controls/struct.FeathersColorSlider.html "struct bevy::feathers::controls::FeathersColorSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#162)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersColorSlider](../../../feathers/controls/struct.FeathersColorSlider.html "struct bevy::feathers::controls::FeathersColorSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#27)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersColorSwatch](../../../feathers/controls/struct.FeathersColorSwatch.html "struct bevy::feathers::controls::FeathersColorSwatch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#27)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersColorSwatch](../../../feathers/controls/struct.FeathersColorSwatch.html "struct bevy::feathers::controls::FeathersColorSwatch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/disclosure_toggle.rs.html#33)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersDisclosureToggle](../../../feathers/controls/struct.FeathersDisclosureToggle.html "struct bevy::feathers::controls::FeathersDisclosureToggle")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/disclosure_toggle.rs.html#33)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersDisclosureToggle](../../../feathers/controls/struct.FeathersDisclosureToggle.html "struct bevy::feathers::controls::FeathersDisclosureToggle")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#106)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersListRow](../../../feathers/controls/struct.FeathersListRow.html "struct bevy::feathers::controls::FeathersListRow")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#106)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersListRow](../../../feathers/controls/struct.FeathersListRow.html "struct bevy::feathers::controls::FeathersListRow")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#36)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersListView](../../../feathers/controls/struct.FeathersListView.html "struct bevy::feathers::controls::FeathersListView")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#36)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersListView](../../../feathers/controls/struct.FeathersListView.html "struct bevy::feathers::controls::FeathersListView")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#48)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersMenu](../../../feathers/controls/struct.FeathersMenu.html "struct bevy::feathers::controls::FeathersMenu")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#48)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersMenu](../../../feathers/controls/struct.FeathersMenu.html "struct bevy::feathers::controls::FeathersMenu")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#141)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersMenuButton](../../../feathers/controls/struct.FeathersMenuButton.html "struct bevy::feathers::controls::FeathersMenuButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#141)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersMenuButton](../../../feathers/controls/struct.FeathersMenuButton.html "struct bevy::feathers::controls::FeathersMenuButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#439)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersMenuDivider](../../../feathers/controls/struct.FeathersMenuDivider.html "struct bevy::feathers::controls::FeathersMenuDivider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#439)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersMenuDivider](../../../feathers/controls/struct.FeathersMenuDivider.html "struct bevy::feathers::controls::FeathersMenuDivider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#250)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersMenuItem](../../../feathers/controls/struct.FeathersMenuItem.html "struct bevy::feathers::controls::FeathersMenuItem")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#250)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersMenuItem](../../../feathers/controls/struct.FeathersMenuItem.html "struct bevy::feathers::controls::FeathersMenuItem")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#195)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersMenuPopup](../../../feathers/controls/struct.FeathersMenuPopup.html "struct bevy::feathers::controls::FeathersMenuPopup")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#195)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersMenuPopup](../../../feathers/controls/struct.FeathersMenuPopup.html "struct bevy::feathers::controls::FeathersMenuPopup")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#55)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersNumberInput](../../../feathers/controls/struct.FeathersNumberInput.html "struct bevy::feathers::controls::FeathersNumberInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#55)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersNumberInput](../../../feathers/controls/struct.FeathersNumberInput.html "struct bevy::feathers::controls::FeathersNumberInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/radio.rs.html#47)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersRadio](../../../feathers/controls/struct.FeathersRadio.html "struct bevy::feathers::controls::FeathersRadio")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/radio.rs.html#47)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersRadio](../../../feathers/controls/struct.FeathersRadio.html "struct bevy::feathers::controls::FeathersRadio")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/scrollbar.rs.html#22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersScrollbar](../../../feathers/controls/struct.FeathersScrollbar.html "struct bevy::feathers::controls::FeathersScrollbar")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/scrollbar.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersScrollbar](../../../feathers/controls/struct.FeathersScrollbar.html "struct bevy::feathers::controls::FeathersScrollbar")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/slider.rs.html#50)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersSlider](../../../feathers/controls/struct.FeathersSlider.html "struct bevy::feathers::controls::FeathersSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/slider.rs.html#50)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersSlider](../../../feathers/controls/struct.FeathersSlider.html "struct bevy::feathers::controls::FeathersSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#85)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersTextInput](../../../feathers/controls/struct.FeathersTextInput.html "struct bevy::feathers::controls::FeathersTextInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#85)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersTextInput](../../../feathers/controls/struct.FeathersTextInput.html "struct bevy::feathers::controls::FeathersTextInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#38)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersTextInputContainer](../../../feathers/controls/struct.FeathersTextInputContainer.html "struct bevy::feathers::controls::FeathersTextInputContainer")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#38)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersTextInputContainer](../../../feathers/controls/struct.FeathersTextInputContainer.html "struct bevy::feathers::controls::FeathersTextInputContainer")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/toggle_switch.rs.html#42)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersToggleSwitch](../../../feathers/controls/struct.FeathersToggleSwitch.html "struct bevy::feathers::controls::FeathersToggleSwitch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/toggle_switch.rs.html#42)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersToggleSwitch](../../../feathers/controls/struct.FeathersToggleSwitch.html "struct bevy::feathers::controls::FeathersToggleSwitch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#126)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FeathersToolButton](../../../feathers/controls/struct.FeathersToolButton.html "struct bevy::feathers::controls::FeathersToolButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#126)

#### type [This](#associatedtype.This)<'from\_arg> = [FeathersToolButton](../../../feathers/controls/struct.FeathersToolButton.html "struct bevy::feathers::controls::FeathersToolButton")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#376)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FileDragAndDrop](../../../prelude/enum.FileDragAndDrop.html "enum bevy::prelude::FileDragAndDrop")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#376)

#### type [This](#associatedtype.This)<'from\_arg> = [FileDragAndDrop](../../../prelude/enum.FileDragAndDrop.html "enum bevy::prelude::FileDragAndDrop")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#68)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Fixed](../../../prelude/struct.Fixed.html "struct bevy::prelude::Fixed")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#68)

#### type [This](#associatedtype.This)<'from\_arg> = [Fixed](../../../prelude/struct.Fixed.html "struct bevy::prelude::Fixed")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1206)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FlexDirection](../../../prelude/enum.FlexDirection.html "enum bevy::prelude::FlexDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1206)

#### type [This](#associatedtype.This)<'from\_arg> = [FlexDirection](../../../prelude/enum.FlexDirection.html "enum bevy::prelude::FlexDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1478)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FlexWrap](../../../prelude/enum.FlexWrap.html "enum bevy::prelude::FlexWrap")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1478)

#### type [This](#associatedtype.This)<'from\_arg> = [FlexWrap](../../../prelude/enum.FlexWrap.html "enum bevy::prelude::FlexWrap")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/float_ord.rs.html#22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FloatOrd](../../../math/struct.FloatOrd.html "struct bevy::math::FloatOrd")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/float_ord.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [FloatOrd](../../../math/struct.FloatOrd.html "struct bevy::math::FloatOrd")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#15)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FocusCause](../../../input_focus/enum.FocusCause.html "enum bevy::input_focus::FocusCause")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#15)

#### type [This](#associatedtype.This)<'from\_arg> = [FocusCause](../../../input_focus/enum.FocusCause.html "enum bevy::input_focus::FocusCause")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#33)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FocusGained](../../../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#33)

#### type [This](#associatedtype.This)<'from\_arg> = [FocusGained](../../../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#23)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FocusIndicator](../../../feathers/focus/struct.FocusIndicator.html "struct bevy::feathers::focus::FocusIndicator")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#23)

#### type [This](#associatedtype.This)<'from\_arg> = [FocusIndicator](../../../feathers/focus/struct.FocusIndicator.html "struct bevy::feathers::focus::FocusIndicator")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#50)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FocusLost](../../../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#50)

#### type [This](#associatedtype.This)<'from\_arg> = [FocusLost](../../../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#101)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FocusPolicy](../../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#101)

#### type [This](#associatedtype.This)<'from\_arg> = [FocusPolicy](../../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#30)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FocusWithinIndicator](../../../feathers/focus/struct.FocusWithinIndicator.html "struct bevy::feathers::focus::FocusWithinIndicator")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#30)

#### type [This](#associatedtype.This)<'from\_arg> = [FocusWithinIndicator](../../../feathers/focus/struct.FocusWithinIndicator.html "struct bevy::feathers::focus::FocusWithinIndicator")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#469)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FocusableArea](../../../input_focus/directional_navigation/struct.FocusableArea.html "struct bevy::input_focus::directional_navigation::FocusableArea")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#469)

#### type [This](#associatedtype.This)<'from\_arg> = [FocusableArea](../../../input_focus/directional_navigation/struct.FocusableArea.html "struct bevy::input_focus::directional_navigation::FocusableArea")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#100)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FogFalloff](../../../prelude/enum.FogFalloff.html "enum bevy::prelude::FogFalloff")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#100)

#### type [This](#associatedtype.This)<'from\_arg> = [FogFalloff](../../../prelude/enum.FogFalloff.html "enum bevy::prelude::FogFalloff")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#75)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FogVolume](../../../light/struct.FogVolume.html "struct bevy::light::FogVolume")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#75)

#### type [This](#associatedtype.This)<'from\_arg> = [FogVolume](../../../light/struct.FogVolume.html "struct bevy::light::FogVolume")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#728)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontFeatureTag](../../../text/struct.FontFeatureTag.html "struct bevy::text::FontFeatureTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#728)

#### type [This](#associatedtype.This)<'from\_arg> = [FontFeatureTag](../../../text/struct.FontFeatureTag.html "struct bevy::text::FontFeatureTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#839)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontFeatures](../../../text/struct.FontFeatures.html "struct bevy::text::FontFeatures")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#839)

#### type [This](#associatedtype.This)<'from\_arg> = [FontFeatures](../../../text/struct.FontFeatures.html "struct bevy::text::FontFeatures")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1199)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontHinting](../../../prelude/enum.FontHinting.html "enum bevy::prelude::FontHinting")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1199)

#### type [This](#associatedtype.This)<'from\_arg> = [FontHinting](../../../prelude/enum.FontHinting.html "enum bevy::prelude::FontHinting")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#486)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontSize](../../../prelude/enum.FontSize.html "enum bevy::prelude::FontSize")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#486)

#### type [This](#associatedtype.This)<'from\_arg> = [FontSize](../../../prelude/enum.FontSize.html "enum bevy::prelude::FontSize")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1179)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontSmoothing](../../../prelude/enum.FontSmoothing.html "enum bevy::prelude::FontSmoothing")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1179)

#### type [This](#associatedtype.This)<'from\_arg> = [FontSmoothing](../../../prelude/enum.FontSmoothing.html "enum bevy::prelude::FontSmoothing")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#267)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontSource](../../../prelude/enum.FontSource.html "enum bevy::prelude::FontSource")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#267)

#### type [This](#associatedtype.This)<'from\_arg> = [FontSource](../../../prelude/enum.FontSource.html "enum bevy::prelude::FontSource")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#704)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontStyle](../../../prelude/enum.FontStyle.html "enum bevy::prelude::FontStyle")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#704)

#### type [This](#associatedtype.This)<'from\_arg> = [FontStyle](../../../prelude/enum.FontStyle.html "enum bevy::prelude::FontStyle")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#913)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontVariationTag](../../../text/struct.FontVariationTag.html "struct bevy::text::FontVariationTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#913)

#### type [This](#associatedtype.This)<'from\_arg> = [FontVariationTag](../../../text/struct.FontVariationTag.html "struct bevy::text::FontVariationTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#960)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontVariations](../../../text/struct.FontVariations.html "struct bevy::text::FontVariations")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#960)

#### type [This](#associatedtype.This)<'from\_arg> = [FontVariations](../../../text/struct.FontVariations.html "struct bevy::text::FontVariations")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#596)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontWeight](../../../prelude/struct.FontWeight.html "struct bevy::prelude::FontWeight")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#596)

#### type [This](#associatedtype.This)<'from\_arg> = [FontWeight](../../../prelude/struct.FontWeight.html "struct bevy::prelude::FontWeight")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#659)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FontWidth](../../../prelude/struct.FontWidth.html "struct bevy::prelude::FontWidth")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#659)

#### type [This](#associatedtype.This)<'from\_arg> = [FontWidth](../../../prelude/struct.FontWidth.html "struct bevy::prelude::FontWidth")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#73)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ForceTouch](../../../input/touch/enum.ForceTouch.html "enum bevy::input::touch::ForceTouch")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#73)

#### type [This](#associatedtype.This)<'from\_arg> = [ForceTouch](../../../input/touch/enum.ForceTouch.html "enum bevy::input::touch::ForceTouch")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#62)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ForwardDecal](../../../pbr/decal/struct.ForwardDecal.html "struct bevy::pbr::decal::ForwardDecal")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#62)

#### type [This](#associatedtype.This)<'from\_arg> = [ForwardDecal](../../../pbr/decal/struct.ForwardDecal.html "struct bevy::pbr::decal::ForwardDecal")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#108)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FpsOverlayConfig](../../../dev_tools/fps_overlay/struct.FpsOverlayConfig.html "struct bevy::dev_tools::fps_overlay::FpsOverlayConfig")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#108)

#### type [This](#associatedtype.This)<'from\_arg> = [FpsOverlayConfig](../../../dev_tools/fps_overlay/struct.FpsOverlayConfig.html "struct bevy::dev_tools::fps_overlay::FpsOverlayConfig")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#139)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FrameTimeGraphConfig](../../../dev_tools/fps_overlay/struct.FrameTimeGraphConfig.html "struct bevy::dev_tools::fps_overlay::FrameTimeGraphConfig")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#139)

#### type [This](#associatedtype.This)<'from\_arg> = [FrameTimeGraphConfig](../../../dev_tools/fps_overlay/struct.FrameTimeGraphConfig.html "struct bevy::dev_tools::fps_overlay::FrameTimeGraphConfig")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#247)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Frustum](../../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#247)

#### type [This](#associatedtype.This)<'from\_arg> = [Frustum](../../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#78)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FrustumGizmoConfigGroup](../../../prelude/struct.FrustumGizmoConfigGroup.html "struct bevy::prelude::FrustumGizmoConfigGroup")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#78)

#### type [This](#associatedtype.This)<'from\_arg> = [FrustumGizmoConfigGroup](../../../prelude/struct.FrustumGizmoConfigGroup.html "struct bevy::prelude::FrustumGizmoConfigGroup")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Fxaa](../../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

#### type [This](#associatedtype.This)<'from\_arg> = [Fxaa](../../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#371)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Gamepad](../../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#371)

#### type [This](#associatedtype.This)<'from\_arg> = [Gamepad](../../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#664)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadAxis](../../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#664)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadAxis](../../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#258)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadAxisChangedEvent](../../../input/gamepad/struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#258)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadAxisChangedEvent](../../../input/gamepad/struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#572)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadButton](../../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#572)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadButton](../../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#222)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadButtonChangedEvent](../../../input/gamepad/struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#222)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadButtonChangedEvent](../../../input/gamepad/struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#190)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadButtonStateChangedEvent](../../../input/gamepad/struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#190)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadButtonStateChangedEvent](../../../input/gamepad/struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1554)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadConnection](../../../input/gamepad/enum.GamepadConnection.html "enum bevy::input::gamepad::GamepadConnection")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1554)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadConnection](../../../input/gamepad/enum.GamepadConnection.html "enum bevy::input::gamepad::GamepadConnection")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#151)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadConnectionEvent](../../../input/gamepad/struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#151)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadConnectionEvent](../../../input/gamepad/struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#38)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadEvent](../../../input/gamepad/enum.GamepadEvent.html "enum bevy::input::gamepad::GamepadEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#38)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadEvent](../../../input/gamepad/enum.GamepadEvent.html "enum bevy::input::gamepad::GamepadEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#710)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadInput](../../../input/gamepad/enum.GamepadInput.html "enum bevy::input::gamepad::GamepadInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#710)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadInput](../../../input/gamepad/enum.GamepadInput.html "enum bevy::input::gamepad::GamepadInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1688)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadRumbleIntensity](../../../input/gamepad/struct.GamepadRumbleIntensity.html "struct bevy::input::gamepad::GamepadRumbleIntensity")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1688)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadRumbleIntensity](../../../input/gamepad/struct.GamepadRumbleIntensity.html "struct bevy::input::gamepad::GamepadRumbleIntensity")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1778)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadRumbleRequest](../../../input/gamepad/enum.GamepadRumbleRequest.html "enum bevy::input::gamepad::GamepadRumbleRequest")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1778)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadRumbleRequest](../../../input/gamepad/enum.GamepadRumbleRequest.html "enum bevy::input::gamepad::GamepadRumbleRequest")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#736)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GamepadSettings](../../../prelude/struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#736)

#### type [This](#associatedtype.This)<'from\_arg> = [GamepadSettings](../../../prelude/struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#261)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GeneratedEnvironmentMapLight](../../../prelude/struct.GeneratedEnvironmentMapLight.html "struct bevy::prelude::GeneratedEnvironmentMapLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#261)

#### type [This](#associatedtype.This)<'from\_arg> = [GeneratedEnvironmentMapLight](../../../prelude/struct.GeneratedEnvironmentMapLight.html "struct bevy::prelude::GeneratedEnvironmentMapLight")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GhostNode](../../../ui/experimental/struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [GhostNode](../../../ui/experimental/struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#64)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Gizmo](../../../prelude/struct.Gizmo.html "struct bevy::prelude::Gizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#64)

#### type [This](#associatedtype.This)<'from\_arg> = [Gizmo](../../../prelude/struct.Gizmo.html "struct bevy::prelude::Gizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#206)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GizmoConfig](../../../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#206)

#### type [This](#associatedtype.This)<'from\_arg> = [GizmoConfig](../../../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#97)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GizmoConfigStore](../../../prelude/struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#97)

#### type [This](#associatedtype.This)<'from\_arg> = [GizmoConfigStore](../../../prelude/struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#246)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GizmoLineConfig](../../../prelude/struct.GizmoLineConfig.html "struct bevy::prelude::GizmoLineConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#246)

#### type [This](#associatedtype.This)<'from\_arg> = [GizmoLineConfig](../../../prelude/struct.GizmoLineConfig.html "struct bevy::prelude::GizmoLineConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#19)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GizmoLineJoint](../../../prelude/enum.GizmoLineJoint.html "enum bevy::prelude::GizmoLineJoint")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#19)

#### type [This](#associatedtype.This)<'from\_arg> = [GizmoLineJoint](../../../prelude/enum.GizmoLineJoint.html "enum bevy::prelude::GizmoLineJoint")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#37)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GizmoLineStyle](../../../prelude/enum.GizmoLineStyle.html "enum bevy::prelude::GizmoLineStyle")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#37)

#### type [This](#associatedtype.This)<'from\_arg> = [GizmoLineStyle](../../../prelude/enum.GizmoLineStyle.html "enum bevy::prelude::GizmoLineStyle")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#60)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GlobalAmbientLight](../../../prelude/struct.GlobalAmbientLight.html "struct bevy::prelude::GlobalAmbientLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#60)

#### type [This](#associatedtype.This)<'from\_arg> = [GlobalAmbientLight](../../../prelude/struct.GlobalAmbientLight.html "struct bevy::prelude::GlobalAmbientLight")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GlobalRenderDebugOverlay](../../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

#### type [This](#associatedtype.This)<'from\_arg> = [GlobalRenderDebugOverlay](../../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#53)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GlobalTransform](../../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#53)

#### type [This](#associatedtype.This)<'from\_arg> = [GlobalTransform](../../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#107)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GlobalUiDebugOptions](../../../prelude/struct.GlobalUiDebugOptions.html "struct bevy::prelude::GlobalUiDebugOptions")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#107)

#### type [This](#associatedtype.This)<'from\_arg> = [GlobalUiDebugOptions](../../../prelude/struct.GlobalUiDebugOptions.html "struct bevy::prelude::GlobalUiDebugOptions")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#8)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GlobalVolume](../../../prelude/struct.GlobalVolume.html "struct bevy::prelude::GlobalVolume")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#8)

#### type [This](#associatedtype.This)<'from\_arg> = [GlobalVolume](../../../prelude/struct.GlobalVolume.html "struct bevy::prelude::GlobalVolume")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2448)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GlobalZIndex](../../../prelude/struct.GlobalZIndex.html "struct bevy::prelude::GlobalZIndex")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2448)

#### type [This](#associatedtype.This)<'from\_arg> = [GlobalZIndex](../../../prelude/struct.GlobalZIndex.html "struct bevy::prelude::GlobalZIndex")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GlobalsUniform](../../../render/globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

#### type [This](#associatedtype.This)<'from\_arg> = [GlobalsUniform](../../../render/globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#266)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GltfExtras](../../../prelude/struct.GltfExtras.html "struct bevy::prelude::GltfExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#266)

#### type [This](#associatedtype.This)<'from\_arg> = [GltfExtras](../../../prelude/struct.GltfExtras.html "struct bevy::prelude::GltfExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#334)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GltfMaterialExtras](../../../gltf/struct.GltfMaterialExtras.html "struct bevy::gltf::GltfMaterialExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#334)

#### type [This](#associatedtype.This)<'from\_arg> = [GltfMaterialExtras](../../../gltf/struct.GltfMaterialExtras.html "struct bevy::gltf::GltfMaterialExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#344)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GltfMaterialName](../../../gltf/struct.GltfMaterialName.html "struct bevy::gltf::GltfMaterialName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#344)

#### type [This](#associatedtype.This)<'from\_arg> = [GltfMaterialName](../../../gltf/struct.GltfMaterialName.html "struct bevy::gltf::GltfMaterialName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#309)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GltfMeshExtras](../../../gltf/struct.GltfMeshExtras.html "struct bevy::gltf::GltfMeshExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#309)

#### type [This](#associatedtype.This)<'from\_arg> = [GltfMeshExtras](../../../gltf/struct.GltfMeshExtras.html "struct bevy::gltf::GltfMeshExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#319)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GltfMeshName](../../../gltf/struct.GltfMeshName.html "struct bevy::gltf::GltfMeshName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#319)

#### type [This](#associatedtype.This)<'from\_arg> = [GltfMeshName](../../../gltf/struct.GltfMeshName.html "struct bevy::gltf::GltfMeshName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#284)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GltfSceneExtras](../../../gltf/struct.GltfSceneExtras.html "struct bevy::gltf::GltfSceneExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#284)

#### type [This](#associatedtype.This)<'from\_arg> = [GltfSceneExtras](../../../gltf/struct.GltfSceneExtras.html "struct bevy::gltf::GltfSceneExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#294)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GltfSceneName](../../../gltf/struct.GltfSceneName.html "struct bevy::gltf::GltfSceneName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#294)

#### type [This](#associatedtype.This)<'from\_arg> = [GltfSceneName](../../../gltf/struct.GltfSceneName.html "struct bevy::gltf::GltfSceneName")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#32)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GlyphAtlasInfo](../../../text/struct.GlyphAtlasInfo.html "struct bevy::text::GlyphAtlasInfo")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#32)

#### type [This](#associatedtype.This)<'from\_arg> = [GlyphAtlasInfo](../../../text/struct.GlyphAtlasInfo.html "struct bevy::text::GlyphAtlasInfo")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#51)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GlyphAtlasLocation](../../../text/struct.GlyphAtlasLocation.html "struct bevy::text::GlyphAtlasLocation")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#51)

#### type [This](#associatedtype.This)<'from\_arg> = [GlyphAtlasLocation](../../../text/struct.GlyphAtlasLocation.html "struct bevy::text::GlyphAtlasLocation")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#357)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GpuAtmosphereSettings](../../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#357)

#### type [This](#associatedtype.This)<'from\_arg> = [GpuAtmosphereSettings](../../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#457)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Gradient](../../../prelude/enum.Gradient.html "enum bevy::prelude::Gradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#457)

#### type [This](#associatedtype.This)<'from\_arg> = [Gradient](../../../prelude/enum.Gradient.html "enum bevy::prelude::Gradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1512)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GridAutoFlow](../../../prelude/enum.GridAutoFlow.html "enum bevy::prelude::GridAutoFlow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1512)

#### type [This](#associatedtype.This)<'from\_arg> = [GridAutoFlow](../../../prelude/enum.GridAutoFlow.html "enum bevy::prelude::GridAutoFlow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2020)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GridPlacement](../../../prelude/struct.GridPlacement.html "struct bevy::prelude::GridPlacement")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2020)

#### type [This](#associatedtype.This)<'from\_arg> = [GridPlacement](../../../prelude/struct.GridPlacement.html "struct bevy::prelude::GridPlacement")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GridTrack](../../../prelude/struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

#### type [This](#associatedtype.This)<'from\_arg> = [GridTrack](../../../prelude/struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1768)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GridTrackRepetition](../../../prelude/enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1768)

#### type [This](#associatedtype.This)<'from\_arg> = [GridTrackRepetition](../../../prelude/enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/half_space.rs.html#36)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [HalfSpace](../../../prelude/struct.HalfSpace.html "struct bevy::prelude::HalfSpace")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/half_space.rs.html#36)

#### type [This](#associatedtype.This)<'from\_arg> = [HalfSpace](../../../prelude/struct.HalfSpace.html "struct bevy::prelude::HalfSpace")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#60)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [HashedStr](../../../ecs/name/struct.HashedStr.html "struct bevy::ecs::name::HashedStr")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#60)

#### type [This](#associatedtype.This)<'from\_arg> = [HashedStr](../../../ecs/name/struct.HashedStr.html "struct bevy::ecs::name::HashedStr")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#87)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Hdr](../../../camera/struct.Hdr.html "struct bevy::camera::Hdr")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#87)

#### type [This](#associatedtype.This)<'from\_arg> = [Hdr](../../../camera/struct.Hdr.html "struct bevy::camera::Hdr")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#133)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [HitData](../../../picking/backend/struct.HitData.html "struct bevy::picking::backend::HitData")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#133)

#### type [This](#associatedtype.This)<'from\_arg> = [HitData](../../../picking/backend/struct.HitData.html "struct bevy::picking::backend::HitData")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#336)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Hovered](../../../picking/hover/struct.Hovered.html "struct bevy::picking::hover::Hovered")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#336)

#### type [This](#associatedtype.This)<'from\_arg> = [Hovered](../../../picking/hover/struct.Hovered.html "struct bevy::picking::hover::Hovered")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#18)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Hsla](../../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#18)

#### type [This](#associatedtype.This)<'from\_arg> = [Hsla](../../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#18)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Hsva](../../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#18)

#### type [This](#associatedtype.This)<'from\_arg> = [Hsva](../../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Hwba](../../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [Hwba](../../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#48-55)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [I8Vec2](../../../math/struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#48-55)

#### type [This](#associatedtype.This)<'from\_arg> = [I8Vec2](../../../math/struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#57-65)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [I8Vec3](../../../math/struct.I8Vec3.html "struct bevy::math::I8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#57-65)

#### type [This](#associatedtype.This)<'from\_arg> = [I8Vec3](../../../math/struct.I8Vec3.html "struct bevy::math::I8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#67-76)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [I8Vec4](../../../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#67-76)

#### type [This](#associatedtype.This)<'from\_arg> = [I8Vec4](../../../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#78-85)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [I16Vec2](../../../math/struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#78-85)

#### type [This](#associatedtype.This)<'from\_arg> = [I16Vec2](../../../math/struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#87-95)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [I16Vec3](../../../math/struct.I16Vec3.html "struct bevy::math::I16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#87-95)

#### type [This](#associatedtype.This)<'from\_arg> = [I16Vec3](../../../math/struct.I16Vec3.html "struct bevy::math::I16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#97-106)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [I16Vec4](../../../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#97-106)

#### type [This](#associatedtype.This)<'from\_arg> = [I16Vec4](../../../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#108-115)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [I64Vec2](../../../math/struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#108-115)

#### type [This](#associatedtype.This)<'from\_arg> = [I64Vec2](../../../math/struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#117-125)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [I64Vec3](../../../math/struct.I64Vec3.html "struct bevy::math::I64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#117-125)

#### type [This](#associatedtype.This)<'from\_arg> = [I64Vec3](../../../math/struct.I64Vec3.html "struct bevy::math::I64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#127-136)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [I64Vec4](../../../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#127-136)

#### type [This](#associatedtype.This)<'from\_arg> = [I64Vec4](../../../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/irect.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [IRect](../../../prelude/struct.IRect.html "struct bevy::prelude::IRect")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/irect.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [IRect](../../../prelude/struct.IRect.html "struct bevy::prelude::IRect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#20-27)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [IVec2](../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#20-27)

#### type [This](#associatedtype.This)<'from\_arg> = [IVec2](../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#28-36)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [IVec3](../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#28-36)

#### type [This](#associatedtype.This)<'from\_arg> = [IVec3](../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [IVec4](../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### type [This](#associatedtype.This)<'from\_arg> = [IVec4](../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#436)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [IgnoreScroll](../../../prelude/struct.IgnoreScroll.html "struct bevy::prelude::IgnoreScroll")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#436)

#### type [This](#associatedtype.This)<'from\_arg> = [IgnoreScroll](../../../prelude/struct.IgnoreScroll.html "struct bevy::prelude::IgnoreScroll")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Image](../../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### type [This](#associatedtype.This)<'from\_arg> = [Image](../../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#723)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ImageAddressMode](../../../image/enum.ImageAddressMode.html "enum bevy::image::ImageAddressMode")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#723)

#### type [This](#associatedtype.This)<'from\_arg> = [ImageAddressMode](../../../image/enum.ImageAddressMode.html "enum bevy::image::ImageAddressMode")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#776)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ImageCompareFunction](../../../image/enum.ImageCompareFunction.html "enum bevy::image::ImageCompareFunction")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#776)

#### type [This](#associatedtype.This)<'from\_arg> = [ImageCompareFunction](../../../image/enum.ImageCompareFunction.html "enum bevy::image::ImageCompareFunction")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#757)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ImageFilterMode](../../../image/enum.ImageFilterMode.html "enum bevy::image::ImageFilterMode")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#757)

#### type [This](#associatedtype.This)<'from\_arg> = [ImageFilterMode](../../../image/enum.ImageFilterMode.html "enum bevy::image::ImageFilterMode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#15)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ImageNode](../../../prelude/struct.ImageNode.html "struct bevy::prelude::ImageNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#15)

#### type [This](#associatedtype.This)<'from\_arg> = [ImageNode](../../../prelude/struct.ImageNode.html "struct bevy::prelude::ImageNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#192)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ImageNodeSize](../../../ui/widget/struct.ImageNodeSize.html "struct bevy::ui::widget::ImageNodeSize")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#192)

#### type [This](#associatedtype.This)<'from\_arg> = [ImageNodeSize](../../../ui/widget/struct.ImageNodeSize.html "struct bevy::ui::widget::ImageNodeSize")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#983)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ImageRenderTarget](../../../camera/struct.ImageRenderTarget.html "struct bevy::camera::ImageRenderTarget")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#983)

#### type [This](#associatedtype.This)<'from\_arg> = [ImageRenderTarget](../../../camera/struct.ImageRenderTarget.html "struct bevy::camera::ImageRenderTarget")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#673)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ImageSampler](../../../image/enum.ImageSampler.html "enum bevy::image::ImageSampler")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#673)

#### type [This](#associatedtype.This)<'from\_arg> = [ImageSampler](../../../image/enum.ImageSampler.html "enum bevy::image::ImageSampler")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#804)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ImageSamplerBorderColor](../../../image/enum.ImageSamplerBorderColor.html "enum bevy::image::ImageSamplerBorderColor")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#804)

#### type [This](#associatedtype.This)<'from\_arg> = [ImageSamplerBorderColor](../../../image/enum.ImageSamplerBorderColor.html "enum bevy::image::ImageSamplerBorderColor")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#830)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ImageSamplerDescriptor](../../../image/struct.ImageSamplerDescriptor.html "struct bevy::image::ImageSamplerDescriptor")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#830)

#### type [This](#associatedtype.This)<'from\_arg> = [ImageSamplerDescriptor](../../../image/struct.ImageSamplerDescriptor.html "struct bevy::image::ImageSamplerDescriptor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#247)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Ime](../../../prelude/enum.Ime.html "enum bevy::prelude::Ime")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#247)

#### type [This](#associatedtype.This)<'from\_arg> = [Ime](../../../prelude/enum.Ime.html "enum bevy::prelude::Ime")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/index.rs.html#83)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Indices](../../../mesh/enum.Indices.html "enum bevy::mesh::Indices")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/index.rs.html#83)

#### type [This](#associatedtype.This)<'from\_arg> = [Indices](../../../mesh/enum.Indices.html "enum bevy::mesh::Indices")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#89)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InfiniteGrid](../../../dev_tools/infinite_grid/struct.InfiniteGrid.html "struct bevy::dev_tools::infinite_grid::InfiniteGrid")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#89)

#### type [This](#associatedtype.This)<'from\_arg> = [InfiniteGrid](../../../dev_tools/infinite_grid/struct.InfiniteGrid.html "struct bevy::dev_tools::infinite_grid::InfiniteGrid")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#105)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InfiniteGridSettings](../../../dev_tools/infinite_grid/struct.InfiniteGridSettings.html "struct bevy::dev_tools::infinite_grid::InfiniteGridSettings")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#105)

#### type [This](#associatedtype.This)<'from\_arg> = [InfiniteGridSettings](../../../dev_tools/infinite_grid/struct.InfiniteGridSettings.html "struct bevy::dev_tools::infinite_grid::InfiniteGridSettings")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#180)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InfinitePlane3d](../../../prelude/struct.InfinitePlane3d.html "struct bevy::prelude::InfinitePlane3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#180)

#### type [This](#associatedtype.This)<'from\_arg> = [InfinitePlane3d](../../../prelude/struct.InfinitePlane3d.html "struct bevy::prelude::InfinitePlane3d")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/font_styles.rs.html#19)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InheritableFont](../../../feathers/font_styles/struct.InheritableFont.html "struct bevy::feathers::font_styles::InheritableFont")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/font_styles.rs.html#19)

#### type [This](#associatedtype.This)<'from\_arg> = [InheritableFont](../../../feathers/font_styles/struct.InheritableFont.html "struct bevy::feathers::font_styles::InheritableFont")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#106)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InheritableThemeTextColor](../../../feathers/theme/struct.InheritableThemeTextColor.html "struct bevy::feathers::theme::InheritableThemeTextColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#106)

#### type [This](#associatedtype.This)<'from\_arg> = [InheritableThemeTextColor](../../../feathers/theme/struct.InheritableThemeTextColor.html "struct bevy::feathers::theme::InheritableThemeTextColor")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#162)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InheritedVisibility](../../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#162)

#### type [This](#associatedtype.This)<'from\_arg> = [InheritedVisibility](../../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#875)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InlineDirection](../../../prelude/enum.InlineDirection.html "enum bevy::prelude::InlineDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#875)

#### type [This](#associatedtype.This)<'from\_arg> = [InlineDirection](../../../prelude/enum.InlineDirection.html "enum bevy::prelude::InlineDirection")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#100)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InputFocus](../../../input_focus/struct.InputFocus.html "struct bevy::input_focus::InputFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#100)

#### type [This](#associatedtype.This)<'from\_arg> = [InputFocus](../../../input_focus/struct.InputFocus.html "struct bevy::input_focus::InputFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#173)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InputFocusVisible](../../../input_focus/struct.InputFocusVisible.html "struct bevy::input_focus::InputFocusVisible")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#173)

#### type [This](#associatedtype.This)<'from\_arg> = [InputFocusVisible](../../../input_focus/struct.InputFocusVisible.html "struct bevy::input_focus::InputFocusVisible")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#346)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Insert](../../../prelude/struct.Insert.html "struct bevy::prelude::Insert")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#346)

#### type [This](#associatedtype.This)<'from\_arg> = [Insert](../../../prelude/struct.Insert.html "struct bevy::prelude::Insert")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#50)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InstanceId](../../../world_serialization/struct.InstanceId.html "struct bevy::world_serialization::InstanceId")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#50)

#### type [This](#associatedtype.This)<'from\_arg> = [InstanceId](../../../world_serialization/struct.InstanceId.html "struct bevy::world_serialization::InstanceId")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/time.rs.html#3-5)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Instant](../../../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/time.rs.html#3-5)

#### type [This](#associatedtype.This)<'from\_arg> = [Instant](../../../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#44)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Interaction](../../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#44)

#### type [This](#associatedtype.This)<'from\_arg> = [Interaction](../../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InteractionDisabled](../../../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [InteractionDisabled](../../../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1098)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InternalWindowState](../../../window/struct.InternalWindowState.html "struct bevy::window::InternalWindowState")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1098)

#### type [This](#associatedtype.This)<'from\_arg> = [InternalWindowState](../../../window/struct.InternalWindowState.html "struct bevy::window::InternalWindowState")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#634)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InterpolationColorSpace](../../../prelude/enum.InterpolationColorSpace.html "enum bevy::prelude::InterpolationColorSpace")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#634)

#### type [This](#associatedtype.This)<'from\_arg> = [InterpolationColorSpace](../../../prelude/enum.InterpolationColorSpace.html "enum bevy::prelude::InterpolationColorSpace")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/interval.rs.html#23)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Interval](../../../prelude/struct.Interval.html "struct bevy::prelude::Interval")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/interval.rs.html#23)

#### type [This](#associatedtype.This)<'from\_arg> = [Interval](../../../prelude/struct.Interval.html "struct bevy::prelude::Interval")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#329)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [IrradianceVolume](../../../light/struct.IrradianceVolume.html "struct bevy::light::IrradianceVolume")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#329)

#### type [This](#associatedtype.This)<'from\_arg> = [IrradianceVolume](../../../light/struct.IrradianceVolume.html "struct bevy::light::IrradianceVolume")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2978)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [IsDefaultUiCamera](../../../prelude/struct.IsDefaultUiCamera.html "struct bevy::prelude::IsDefaultUiCamera")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2978)

#### type [This](#associatedtype.This)<'from\_arg> = [IsDefaultUiCamera](../../../prelude/struct.IsDefaultUiCamera.html "struct bevy::prelude::IsDefaultUiCamera")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/resource.rs.html#121)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [IsResource](../../../ecs/resource/struct.IsResource.html "struct bevy::ecs::resource::IsResource")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/resource.rs.html#121)

#### type [This](#associatedtype.This)<'from\_arg> = [IsResource](../../../ecs/resource/struct.IsResource.html "struct bevy::ecs::resource::IsResource")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#90)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Isometry2d](../../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#90)

#### type [This](#associatedtype.This)<'from\_arg> = [Isometry2d](../../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#368)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Isometry3d](../../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#368)

#### type [This](#associatedtype.This)<'from\_arg> = [Isometry3d](../../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#53)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [JointAabb](../../../mesh/skinning/struct.JointAabb.html "struct bevy::mesh::skinning::JointAabb")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#53)

#### type [This](#associatedtype.This)<'from\_arg> = [JointAabb](../../../mesh/skinning/struct.JointAabb.html "struct bevy::mesh::skinning::JointAabb")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#336)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [JointIndex](../../../mesh/skinning/struct.JointIndex.html "struct bevy::mesh::skinning::JointIndex")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#336)

#### type [This](#associatedtype.This)<'from\_arg> = [JointIndex](../../../mesh/skinning/struct.JointIndex.html "struct bevy::mesh::skinning::JointIndex")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#346)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [JumpAt](../../../prelude/enum.JumpAt.html "enum bevy::prelude::JumpAt")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#346)

#### type [This](#associatedtype.This)<'from\_arg> = [JumpAt](../../../prelude/enum.JumpAt.html "enum bevy::prelude::JumpAt")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#230)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Justify](../../../prelude/enum.Justify.html "enum bevy::prelude::Justify")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#230)

#### type [This](#associatedtype.This)<'from\_arg> = [Justify](../../../prelude/enum.Justify.html "enum bevy::prelude::Justify")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1102)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [JustifyContent](../../../prelude/enum.JustifyContent.html "enum bevy::prelude::JustifyContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1102)

#### type [This](#associatedtype.This)<'from\_arg> = [JustifyContent](../../../prelude/enum.JustifyContent.html "enum bevy::prelude::JustifyContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#938)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [JustifyItems](../../../prelude/enum.JustifyItems.html "enum bevy::prelude::JustifyItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#938)

#### type [This](#associatedtype.This)<'from\_arg> = [JustifyItems](../../../prelude/enum.JustifyItems.html "enum bevy::prelude::JustifyItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1018)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [JustifySelf](../../../prelude/enum.JustifySelf.html "enum bevy::prelude::JustifySelf")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1018)

#### type [This](#associatedtype.This)<'from\_arg> = [JustifySelf](../../../prelude/enum.JustifySelf.html "enum bevy::prelude::JustifySelf")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#804)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Key](../../../input/keyboard/enum.Key.html "enum bevy::input::keyboard::Key")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#804)

#### type [This](#associatedtype.This)<'from\_arg> = [Key](../../../input/keyboard/enum.Key.html "enum bevy::input::keyboard::Key")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#262)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [KeyCode](../../../prelude/enum.KeyCode.html "enum bevy::prelude::KeyCode")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#262)

#### type [This](#associatedtype.This)<'from\_arg> = [KeyCode](../../../prelude/enum.KeyCode.html "enum bevy::prelude::KeyCode")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#152)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [KeyboardFocusLost](../../../input/keyboard/struct.KeyboardFocusLost.html "struct bevy::input::keyboard::KeyboardFocusLost")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#152)

#### type [This](#associatedtype.This)<'from\_arg> = [KeyboardFocusLost](../../../input/keyboard/struct.KeyboardFocusLost.html "struct bevy::input::keyboard::KeyboardFocusLost")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#103)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [KeyboardInput](../../../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#103)

#### type [This](#associatedtype.This)<'from\_arg> = [KeyboardInput](../../../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#17)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Laba](../../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#17)

#### type [This](#associatedtype.This)<'from\_arg> = [Laba](../../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/label.rs.html#5)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Label](../../../prelude/struct.Label.html "struct bevy::prelude::Label")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/label.rs.html#5)

#### type [This](#associatedtype.This)<'from\_arg> = [Label](../../../prelude/struct.Label.html "struct bevy::prelude::Label")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2903)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LayoutConfig](../../../prelude/struct.LayoutConfig.html "struct bevy::prelude::LayoutConfig")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2903)

#### type [This](#associatedtype.This)<'from\_arg> = [LayoutConfig](../../../prelude/struct.LayoutConfig.html "struct bevy::prelude::LayoutConfig")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#17)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Lcha](../../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#17)

#### type [This](#associatedtype.This)<'from\_arg> = [Lcha](../../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#273)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Leave](../../../prelude/struct.Leave.html "struct bevy::prelude::Leave")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#273)

#### type [This](#associatedtype.This)<'from\_arg> = [Leave](../../../prelude/struct.Leave.html "struct bevy::prelude::Leave")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LensDistortion](../../../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [LensDistortion](../../../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1039)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LetterSpacing](../../../text/enum.LetterSpacing.html "enum bevy::text::LetterSpacing")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1039)

#### type [This](#associatedtype.This)<'from\_arg> = [LetterSpacing](../../../text/enum.LetterSpacing.html "enum bevy::text::LetterSpacing")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#151)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LightGizmoColor](../../../prelude/enum.LightGizmoColor.html "enum bevy::prelude::LightGizmoColor")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#151)

#### type [This](#associatedtype.This)<'from\_arg> = [LightGizmoColor](../../../prelude/enum.LightGizmoColor.html "enum bevy::prelude::LightGizmoColor")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#166)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LightGizmoConfigGroup](../../../prelude/struct.LightGizmoConfigGroup.html "struct bevy::prelude::LightGizmoConfigGroup")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#166)

#### type [This](#associatedtype.This)<'from\_arg> = [LightGizmoConfigGroup](../../../prelude/struct.LightGizmoConfigGroup.html "struct bevy::prelude::LightGizmoConfigGroup")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#71)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LightProbe](../../../prelude/struct.LightProbe.html "struct bevy::prelude::LightProbe")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#71)

#### type [This](#associatedtype.This)<'from\_arg> = [LightProbe](../../../prelude/struct.LightProbe.html "struct bevy::prelude::LightProbe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#87)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Lightmap](../../../pbr/struct.Lightmap.html "struct bevy::pbr::Lightmap")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#87)

#### type [This](#associatedtype.This)<'from\_arg> = [Lightmap](../../../pbr/struct.Lightmap.html "struct bevy::pbr::Lightmap")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1234)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Line2d](../../../prelude/struct.Line2d.html "struct bevy::prelude::Line2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1234)

#### type [This](#associatedtype.This)<'from\_arg> = [Line2d](../../../prelude/struct.Line2d.html "struct bevy::prelude::Line2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#357)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Line3d](../../../prelude/struct.Line3d.html "struct bevy::prelude::Line3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#357)

#### type [This](#associatedtype.This)<'from\_arg> = [Line3d](../../../prelude/struct.Line3d.html "struct bevy::prelude::Line3d")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1112)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LineBreak](../../../prelude/enum.LineBreak.html "enum bevy::prelude::LineBreak")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1112)

#### type [This](#associatedtype.This)<'from\_arg> = [LineBreak](../../../prelude/enum.LineBreak.html "enum bevy::prelude::LineBreak")

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#622)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LineGizmoEntities](../../../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#622)

#### type [This](#associatedtype.This)<'from\_arg> = [LineGizmoEntities](../../../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1011)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LineHeight](../../../text/enum.LineHeight.html "enum bevy::text::LineHeight")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1011)

#### type [This](#associatedtype.This)<'from\_arg> = [LineHeight](../../../text/enum.LineHeight.html "enum bevy::text::LineHeight")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#227)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LinearGradient](../../../prelude/struct.LinearGradient.html "struct bevy::prelude::LinearGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#227)

#### type [This](#associatedtype.This)<'from\_arg> = [LinearGradient](../../../prelude/struct.LinearGradient.html "struct bevy::prelude::LinearGradient")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#18)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LinearRgba](../../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#18)

#### type [This](#associatedtype.This)<'from\_arg> = [LinearRgba](../../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#39)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ListItem](../../../ui_widgets/struct.ListItem.html "struct bevy::ui_widgets::ListItem")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#39)

#### type [This](#associatedtype.This)<'from\_arg> = [ListItem](../../../ui_widgets/struct.ListItem.html "struct bevy::ui_widgets::ListItem")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#210)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Location](../../../picking/pointer/struct.Location.html "struct bevy::picking::pointer::Location")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#210)

#### type [This](#associatedtype.This)<'from\_arg> = [Location](../../../picking/pointer/struct.Location.html "struct bevy::picking::pointer::Location")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#158)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MainEntity](../../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#158)

#### type [This](#associatedtype.This)<'from\_arg> = [MainEntity](../../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#142)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MainPassResolutionOverride](../../../camera/struct.MainPassResolutionOverride.html "struct bevy::camera::MainPassResolutionOverride")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#142)

#### type [This](#associatedtype.This)<'from\_arg> = [MainPassResolutionOverride](../../../camera/struct.MainPassResolutionOverride.html "struct bevy::camera::MainPassResolutionOverride")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#156)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ManageAccessibilityUpdates](../../../a11y/struct.ManageAccessibilityUpdates.html "struct bevy::a11y::ManageAccessibilityUpdates")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#156)

#### type [This](#associatedtype.This)<'from\_arg> = [ManageAccessibilityUpdates](../../../a11y/struct.ManageAccessibilityUpdates.html "struct bevy::a11y::ManageAccessibilityUpdates")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#976)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ManualTextureViewHandle](../../../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#976)

#### type [This](#associatedtype.This)<'from\_arg> = [ManualTextureViewHandle](../../../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#343-350)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mat2](../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#343-350)

#### type [This](#associatedtype.This)<'from\_arg> = [Mat2](../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#351-359)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mat3](../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#351-359)

#### type [This](#associatedtype.This)<'from\_arg> = [Mat3](../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mat4](../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### type [This](#associatedtype.This)<'from\_arg> = [Mat4](../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#360-368)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mat3A](../../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#360-368)

#### type [This](#associatedtype.This)<'from\_arg> = [Mat3A](../../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#276)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MaterialBindGroupIndex](../../../pbr/struct.MaterialBindGroupIndex.html "struct bevy::pbr::MaterialBindGroupIndex")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#276)

#### type [This](#associatedtype.This)<'from\_arg> = [MaterialBindGroupIndex](../../../pbr/struct.MaterialBindGroupIndex.html "struct bevy::pbr::MaterialBindGroupIndex")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#294)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MaterialBindGroupSlot](../../../pbr/struct.MaterialBindGroupSlot.html "struct bevy::pbr::MaterialBindGroupSlot")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#294)

#### type [This](#associatedtype.This)<'from\_arg> = [MaterialBindGroupSlot](../../../pbr/struct.MaterialBindGroupSlot.html "struct bevy::pbr::MaterialBindGroupSlot")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#259)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MaterialBindingId](../../../pbr/struct.MaterialBindingId.html "struct bevy::pbr::MaterialBindingId")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#259)

#### type [This](#associatedtype.This)<'from\_arg> = [MaterialBindingId](../../../pbr/struct.MaterialBindingId.html "struct bevy::pbr::MaterialBindingId")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1569)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MaxTrackSizingFunction](../../../prelude/enum.MaxTrackSizingFunction.html "enum bevy::prelude::MaxTrackSizingFunction")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1569)

#### type [This](#associatedtype.This)<'from\_arg> = [MaxTrackSizingFunction](../../../prelude/enum.MaxTrackSizingFunction.html "enum bevy::prelude::MaxTrackSizingFunction")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#61)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MenuAction](../../../ui_widgets/enum.MenuAction.html "enum bevy::ui_widgets::MenuAction")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#61)

#### type [This](#associatedtype.This)<'from\_arg> = [MenuAction](../../../ui_widgets/enum.MenuAction.html "enum bevy::ui_widgets::MenuAction")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#414)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MenuButton](../../../ui_widgets/struct.MenuButton.html "struct bevy::ui_widgets::MenuButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#414)

#### type [This](#associatedtype.This)<'from\_arg> = [MenuButton](../../../ui_widgets/struct.MenuButton.html "struct bevy::ui_widgets::MenuButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#79)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MenuEvent](../../../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#79)

#### type [This](#associatedtype.This)<'from\_arg> = [MenuEvent](../../../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#139)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MenuFocusState](../../../ui_widgets/enum.MenuFocusState.html "enum bevy::ui_widgets::MenuFocusState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#139)

#### type [This](#associatedtype.This)<'from\_arg> = [MenuFocusState](../../../ui_widgets/enum.MenuFocusState.html "enum bevy::ui_widgets::MenuFocusState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#133)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MenuItem](../../../ui_widgets/struct.MenuItem.html "struct bevy::ui_widgets::MenuItem")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#133)

#### type [This](#associatedtype.This)<'from\_arg> = [MenuItem](../../../ui_widgets/struct.MenuItem.html "struct bevy::ui_widgets::MenuItem")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#91)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MenuLayout](../../../ui_widgets/enum.MenuLayout.html "enum bevy::ui_widgets::MenuLayout")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#91)

#### type [This](#associatedtype.This)<'from\_arg> = [MenuLayout](../../../ui_widgets/enum.MenuLayout.html "enum bevy::ui_widgets::MenuLayout")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#123)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MenuPopup](../../../ui_widgets/struct.MenuPopup.html "struct bevy::ui_widgets::MenuPopup")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#123)

#### type [This](#associatedtype.This)<'from\_arg> = [MenuPopup](../../../ui_widgets/struct.MenuPopup.html "struct bevy::ui_widgets::MenuPopup")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#225)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mesh](../../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#225)

#### type [This](#associatedtype.This)<'from\_arg> = [Mesh](../../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#41)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mesh2d](../../../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#41)

#### type [This](#associatedtype.This)<'from\_arg> = [Mesh2d](../../../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#445)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mesh2dWireframe](../../../sprite_render/struct.Mesh2dWireframe.html "struct bevy::sprite_render::Mesh2dWireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#445)

#### type [This](#associatedtype.This)<'from\_arg> = [Mesh2dWireframe](../../../sprite_render/struct.Mesh2dWireframe.html "struct bevy::sprite_render::Mesh2dWireframe")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#98)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mesh3d](../../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#98)

#### type [This](#associatedtype.This)<'from\_arg> = [Mesh3d](../../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#935)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mesh3dWireframe](../../../pbr/wireframe/struct.Mesh3dWireframe.html "struct bevy::pbr::wireframe::Mesh3dWireframe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#935)

#### type [This](#associatedtype.This)<'from\_arg> = [Mesh3dWireframe](../../../pbr/wireframe/struct.Mesh3dWireframe.html "struct bevy::pbr::wireframe::Mesh3dWireframe")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#118)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MeshMorphWeights](../../../mesh/morph/enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#118)

#### type [This](#associatedtype.This)<'from\_arg> = [MeshMorphWeights](../../../mesh/morph/enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#33)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MeshPickingCamera](../../../prelude/struct.MeshPickingCamera.html "struct bevy::prelude::MeshPickingCamera")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#33)

#### type [This](#associatedtype.This)<'from\_arg> = [MeshPickingCamera](../../../prelude/struct.MeshPickingCamera.html "struct bevy::prelude::MeshPickingCamera")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#38)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MeshPickingSettings](../../../prelude/struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#38)

#### type [This](#associatedtype.This)<'from\_arg> = [MeshPickingSettings](../../../prelude/struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#154)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MeshTag](../../../mesh/struct.MeshTag.html "struct bevy::mesh::MeshTag")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#154)

#### type [This](#associatedtype.This)<'from\_arg> = [MeshTag](../../../mesh/struct.MeshTag.html "struct bevy::mesh::MeshTag")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#230)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MeshletMesh3d](../../../pbr/experimental/meshlet/struct.MeshletMesh3d.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#230)

#### type [This](#associatedtype.This)<'from\_arg> = [MeshletMesh3d](../../../pbr/experimental/meshlet/struct.MeshletMesh3d.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3d")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1540)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MinTrackSizingFunction](../../../prelude/enum.MinTrackSizingFunction.html "enum bevy::prelude::MinTrackSizingFunction")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1540)

#### type [This](#associatedtype.This)<'from\_arg> = [MinTrackSizingFunction](../../../prelude/enum.MinTrackSizingFunction.html "enum bevy::prelude::MinTrackSizingFunction")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#805)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MipBias](../../../render/camera/struct.MipBias.html "struct bevy::render::camera::MipBias")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#805)

#### type [This](#associatedtype.This)<'from\_arg> = [MipBias](../../../render/camera/struct.MipBias.html "struct bevy::render::camera::MipBias")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#24)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Monitor](../../../window/struct.Monitor.html "struct bevy::window::Monitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#24)

#### type [This](#associatedtype.This)<'from\_arg> = [Monitor](../../../window/struct.Monitor.html "struct bevy::window::Monitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1147)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MonitorSelection](../../../prelude/enum.MonitorSelection.html "enum bevy::prelude::MonitorSelection")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1147)

#### type [This](#associatedtype.This)<'from\_arg> = [MonitorSelection](../../../prelude/enum.MonitorSelection.html "enum bevy::prelude::MonitorSelection")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#133)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MorphAttributes](../../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#133)

#### type [This](#associatedtype.This)<'from\_arg> = [MorphAttributes](../../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#79)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MorphWeights](../../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#79)

#### type [This](#associatedtype.This)<'from\_arg> = [MorphWeights](../../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#73)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MotionBlur](../../../post_process/motion_blur/struct.MotionBlur.html "struct bevy::post_process::motion_blur::MotionBlur")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#73)

#### type [This](#associatedtype.This)<'from\_arg> = [MotionBlur](../../../post_process/motion_blur/struct.MotionBlur.html "struct bevy::post_process::motion_blur::MotionBlur")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#76)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MotionVectorPrepass](../../../core_pipeline/prepass/struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#76)

#### type [This](#associatedtype.This)<'from\_arg> = [MotionVectorPrepass](../../../core_pipeline/prepass/struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#64)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MouseButton](../../../prelude/enum.MouseButton.html "enum bevy::prelude::MouseButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#64)

#### type [This](#associatedtype.This)<'from\_arg> = [MouseButton](../../../prelude/enum.MouseButton.html "enum bevy::prelude::MouseButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#34)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MouseButtonInput](../../../input/mouse/struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#34)

#### type [This](#associatedtype.This)<'from\_arg> = [MouseButtonInput](../../../input/mouse/struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#99)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MouseMotion](../../../input/mouse/struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#99)

#### type [This](#associatedtype.This)<'from\_arg> = [MouseMotion](../../../input/mouse/struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#121)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MouseScrollUnit](../../../input/mouse/enum.MouseScrollUnit.html "enum bevy::input::mouse::MouseScrollUnit")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#121)

#### type [This](#associatedtype.This)<'from\_arg> = [MouseScrollUnit](../../../input/mouse/enum.MouseScrollUnit.html "enum bevy::input::mouse::MouseScrollUnit")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#160)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MouseWheel](../../../input/mouse/struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#160)

#### type [This](#associatedtype.This)<'from\_arg> = [MouseWheel](../../../input/mouse/struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#323)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Move](../../../prelude/struct.Move.html "struct bevy::prelude::Move")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#323)

#### type [This](#associatedtype.This)<'from\_arg> = [Move](../../../prelude/struct.Move.html "struct bevy::prelude::Move")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#231)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Msaa](../../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#231)

#### type [This](#associatedtype.This)<'from\_arg> = [Msaa](../../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#29)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MsaaWriteback](../../../prelude/enum.MsaaWriteback.html "enum bevy::prelude::MsaaWriteback")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#29)

#### type [This](#associatedtype.This)<'from\_arg> = [MsaaWriteback](../../../prelude/enum.MsaaWriteback.html "enum bevy::prelude::MsaaWriteback")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Name](../../../prelude/struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### type [This](#associatedtype.This)<'from\_arg> = [Name](../../../prelude/struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#758)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NativeKey](../../../input/keyboard/enum.NativeKey.html "enum bevy::input::keyboard::NativeKey")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#758)

#### type [This](#associatedtype.This)<'from\_arg> = [NativeKey](../../../input/keyboard/enum.NativeKey.html "enum bevy::input::keyboard::NativeKey")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#220)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NativeKeyCode](../../../input/keyboard/enum.NativeKeyCode.html "enum bevy::input::keyboard::NativeKeyCode")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#220)

#### type [This](#associatedtype.This)<'from\_arg> = [NativeKeyCode](../../../input/keyboard/enum.NativeKeyCode.html "enum bevy::input::keyboard::NativeKeyCode")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#107)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NavAction](../../../input_focus/tab_navigation/enum.NavAction.html "enum bevy::input_focus::tab_navigation::NavAction")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#107)

#### type [This](#associatedtype.This)<'from\_arg> = [NavAction](../../../input_focus/tab_navigation/enum.NavAction.html "enum bevy::input_focus::tab_navigation::NavAction")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#159)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NavNeighbor](../../../input_focus/directional_navigation/enum.NavNeighbor.html "enum bevy::input_focus::directional_navigation::NavNeighbor")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#159)

#### type [This](#associatedtype.This)<'from\_arg> = [NavNeighbor](../../../input_focus/directional_navigation/enum.NavNeighbor.html "enum bevy::input_focus::directional_navigation::NavNeighbor")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#187)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NavNeighbors](../../../input_focus/directional_navigation/struct.NavNeighbors.html "struct bevy::input_focus::directional_navigation::NavNeighbors")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#187)

#### type [This](#associatedtype.This)<'from\_arg> = [NavNeighbors](../../../input_focus/directional_navigation/struct.NavNeighbors.html "struct bevy::input_focus::directional_navigation::NavNeighbors")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#550)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NoAutoAabb](../../../camera/visibility/struct.NoAutoAabb.html "struct bevy::camera::visibility::NoAutoAabb")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#550)

#### type [This](#associatedtype.This)<'from\_arg> = [NoAutoAabb](../../../camera/visibility/struct.NoAutoAabb.html "struct bevy::camera::visibility::NoAutoAabb")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#55)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NoBackgroundMotionVectors](../../../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#55)

#### type [This](#associatedtype.This)<'from\_arg> = [NoBackgroundMotionVectors](../../../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#316)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NoFrustumCulling](../../../camera/visibility/struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#316)

#### type [This](#associatedtype.This)<'from\_arg> = [NoFrustumCulling](../../../camera/visibility/struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#868)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NoWireframe](../../../pbr/wireframe/struct.NoWireframe.html "struct bevy::pbr::wireframe::NoWireframe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#868)

#### type [This](#associatedtype.This)<'from\_arg> = [NoWireframe](../../../pbr/wireframe/struct.NoWireframe.html "struct bevy::pbr::wireframe::NoWireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#418)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NoWireframe2d](../../../sprite_render/struct.NoWireframe2d.html "struct bevy::sprite_render::NoWireframe2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#418)

#### type [This](#associatedtype.This)<'from\_arg> = [NoWireframe2d](../../../sprite_render/struct.NoWireframe2d.html "struct bevy::sprite_render::NoWireframe2d")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#471)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Node](../../../prelude/struct.Node.html "struct bevy::prelude::Node")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#471)

#### type [This](#associatedtype.This)<'from\_arg> = [Node](../../../prelude/struct.Node.html "struct bevy::prelude::Node")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#156)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NodeImageMode](../../../prelude/enum.NodeImageMode.html "enum bevy::prelude::NodeImageMode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#156)

#### type [This](#associatedtype.This)<'from\_arg> = [NodeImageMode](../../../prelude/enum.NodeImageMode.html "enum bevy::prelude::NodeImageMode")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/uuid.rs.html#15-22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NonNilUuid](../../../asset/uuid/struct.NonNilUuid.html "struct bevy::asset::uuid::NonNilUuid")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/uuid.rs.html#15-22)

#### type [This](#associatedtype.This)<'from\_arg> = [NonNilUuid](../../../asset/uuid/struct.NonNilUuid.html "struct bevy::asset::uuid::NonNilUuid")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#68)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NormalPrepass](../../../core_pipeline/prepass/struct.NormalPrepass.html "struct bevy::core_pipeline::prepass::NormalPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#68)

#### type [This](#associatedtype.This)<'from\_arg> = [NormalPrepass](../../../core_pipeline/prepass/struct.NormalPrepass.html "struct bevy::core_pipeline::prepass::NormalPrepass")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#940)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NormalizedRenderTarget](../../../camera/enum.NormalizedRenderTarget.html "enum bevy::camera::NormalizedRenderTarget")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#940)

#### type [This](#associatedtype.This)<'from\_arg> = [NormalizedRenderTarget](../../../camera/enum.NormalizedRenderTarget.html "enum bevy::camera::NormalizedRenderTarget")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#105)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NormalizedWindowRef](../../../window/struct.NormalizedWindowRef.html "struct bevy::window::NormalizedWindowRef")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#105)

#### type [This](#associatedtype.This)<'from\_arg> = [NormalizedWindowRef](../../../window/struct.NormalizedWindowRef.html "struct bevy::window::NormalizedWindowRef")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#256)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NotShadowCaster](../../../light/struct.NotShadowCaster.html "struct bevy::light::NotShadowCaster")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#256)

#### type [This](#associatedtype.This)<'from\_arg> = [NotShadowCaster](../../../light/struct.NotShadowCaster.html "struct bevy::light::NotShadowCaster")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#264)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NotShadowReceiver](../../../light/struct.NotShadowReceiver.html "struct bevy::light::NotShadowReceiver")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#264)

#### type [This](#associatedtype.This)<'from\_arg> = [NotShadowReceiver](../../../light/struct.NotShadowReceiver.html "struct bevy::light::NotShadowReceiver")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#131)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NumberFormat](../../../feathers/controls/enum.NumberFormat.html "enum bevy::feathers::controls::NumberFormat")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#131)

#### type [This](#associatedtype.This)<'from\_arg> = [NumberFormat](../../../feathers/controls/enum.NumberFormat.html "enum bevy::feathers::controls::NumberFormat")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#146)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NumberInputValue](../../../feathers/controls/enum.NumberInputValue.html "enum bevy::feathers::controls::NumberInputValue")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#146)

#### type [This](#associatedtype.This)<'from\_arg> = [NumberInputValue](../../../feathers/controls/enum.NumberInputValue.html "enum bevy::feathers::controls::NumberInputValue")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#501)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ObservedBy](../../../ecs/observer/struct.ObservedBy.html "struct bevy::ecs::observer::ObservedBy")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#501)

#### type [This](#associatedtype.This)<'from\_arg> = [ObservedBy](../../../ecs/observer/struct.ObservedBy.html "struct bevy::ecs::observer::ObservedBy")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OcclusionCulling](../../../render/occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

#### type [This](#associatedtype.This)<'from\_arg> = [OcclusionCulling](../../../render/occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#298)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OffsetAccess](../../struct.OffsetAccess.html "struct bevy::reflect::OffsetAccess")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#298)

#### type [This](#associatedtype.This)<'from\_arg> = [OffsetAccess](../../struct.OffsetAccess.html "struct bevy::reflect::OffsetAccess")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#17)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Oklaba](../../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#17)

#### type [This](#associatedtype.This)<'from\_arg> = [Oklaba](../../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Oklcha](../../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)

#### type [This](#associatedtype.This)<'from\_arg> = [Oklcha](../../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/opaque.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OpaqueRendererMethod](../../../material/enum.OpaqueRendererMethod.html "enum bevy::material::OpaqueRendererMethod")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/opaque.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [OpaqueRendererMethod](../../../material/enum.OpaqueRendererMethod.html "enum bevy::material::OpaqueRendererMethod")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OrderIndependentTransparencySettings](../../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

#### type [This](#associatedtype.This)<'from\_arg> = [OrderIndependentTransparencySettings](../../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#578)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OrthographicProjection](../../../prelude/struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#578)

#### type [This](#associatedtype.This)<'from\_arg> = [OrthographicProjection](../../../prelude/struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#240)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Out](../../../prelude/struct.Out.html "struct bevy::prelude::Out")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#240)

#### type [This](#associatedtype.This)<'from\_arg> = [Out](../../../prelude/struct.Out.html "struct bevy::prelude::Out")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2456)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OuterColor](../../../prelude/struct.OuterColor.html "struct bevy::prelude::OuterColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2456)

#### type [This](#associatedtype.This)<'from\_arg> = [OuterColor](../../../prelude/struct.OuterColor.html "struct bevy::prelude::OuterColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2315)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Outline](../../../prelude/struct.Outline.html "struct bevy::prelude::Outline")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2315)

#### type [This](#associatedtype.This)<'from\_arg> = [Outline](../../../prelude/struct.Outline.html "struct bevy::prelude::Outline")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#190)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Over](../../../prelude/struct.Over.html "struct bevy::prelude::Over")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#190)

#### type [This](#associatedtype.This)<'from\_arg> = [Over](../../../prelude/struct.Over.html "struct bevy::prelude::Over")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Overflow](../../../prelude/struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

#### type [This](#associatedtype.This)<'from\_arg> = [Overflow](../../../prelude/struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1347)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OverflowAxis](../../../prelude/enum.OverflowAxis.html "enum bevy::prelude::OverflowAxis")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1347)

#### type [This](#associatedtype.This)<'from\_arg> = [OverflowAxis](../../../prelude/enum.OverflowAxis.html "enum bevy::prelude::OverflowAxis")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1381)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OverflowClipMargin](../../../prelude/struct.OverflowClipMargin.html "struct bevy::prelude::OverflowClipMargin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1381)

#### type [This](#associatedtype.This)<'from\_arg> = [OverflowClipMargin](../../../prelude/struct.OverflowClipMargin.html "struct bevy::prelude::OverflowClipMargin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2416)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OverrideClip](../../../prelude/struct.OverrideClip.html "struct bevy::prelude::OverrideClip")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2416)

#### type [This](#associatedtype.This)<'from\_arg> = [OverrideClip](../../../prelude/struct.OverrideClip.html "struct bevy::prelude::OverrideClip")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#47)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [OverrideCursor](../../../feathers/cursor/struct.OverrideCursor.html "struct bevy::feathers::cursor::OverrideCursor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#47)

#### type [This](#associatedtype.This)<'from\_arg> = [OverrideCursor](../../../feathers/cursor/struct.OverrideCursor.html "struct bevy::feathers::cursor::OverrideCursor")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#84)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PanGesture](../../../input/gestures/struct.PanGesture.html "struct bevy::input::gestures::PanGesture")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#84)

#### type [This](#associatedtype.This)<'from\_arg> = [PanGesture](../../../input/gestures/struct.PanGesture.html "struct bevy::input::gestures::PanGesture")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#410)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ParallaxCorrection](../../../light/enum.ParallaxCorrection.html "enum bevy::light::ParallaxCorrection")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#410)

#### type [This](#associatedtype.This)<'from\_arg> = [ParallaxCorrection](../../../light/enum.ParallaxCorrection.html "enum bevy::light::ParallaxCorrection")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/parallax.rs.html#14)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ParallaxMappingMethod](../../../prelude/enum.ParallaxMappingMethod.html "enum bevy::prelude::ParallaxMappingMethod")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/parallax.rs.html#14)

#### type [This](#associatedtype.This)<'from\_arg> = [ParallaxMappingMethod](../../../prelude/enum.ParallaxMappingMethod.html "enum bevy::prelude::ParallaxMappingMethod")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#367)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ParsedPath](../../struct.ParsedPath.html "struct bevy::reflect::ParsedPath")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#367)

#### type [This](#associatedtype.This)<'from\_arg> = [ParsedPath](../../struct.ParsedPath.html "struct bevy::reflect::ParsedPath")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/pathtracer/mod.rs.html#63)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Pathtracer](../../../solari/pathtracer/struct.Pathtracer.html "struct bevy::solari::pathtracer::Pathtracer")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/pathtracer/mod.rs.html#63)

#### type [This](#associatedtype.This)<'from\_arg> = [Pathtracer](../../../solari/pathtracer/struct.Pathtracer.html "struct bevy::solari::pathtracer::Pathtracer")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#281)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PerspectiveProjection](../../../prelude/struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#281)

#### type [This](#associatedtype.This)<'from\_arg> = [PerspectiveProjection](../../../prelude/struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#196)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Pickable](../../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#196)

#### type [This](#associatedtype.This)<'from\_arg> = [Pickable](../../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#224)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PickingInteraction](../../../picking/hover/enum.PickingInteraction.html "enum bevy::picking::hover::PickingInteraction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#224)

#### type [This](#associatedtype.This)<'from\_arg> = [PickingInteraction](../../../picking/hover/enum.PickingInteraction.html "enum bevy::picking::hover::PickingInteraction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#296)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PickingSettings](../../../picking/struct.PickingSettings.html "struct bevy::picking::PickingSettings")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#296)

#### type [This](#associatedtype.This)<'from\_arg> = [PickingSettings](../../../picking/struct.PickingSettings.html "struct bevy::picking::PickingSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#25)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PinchGesture](../../../input/gestures/struct.PinchGesture.html "struct bevy::input::gestures::PinchGesture")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#25)

#### type [This](#associatedtype.This)<'from\_arg> = [PinchGesture](../../../input/gestures/struct.PinchGesture.html "struct bevy::input::gestures::PinchGesture")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1192)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Plane2d](../../../prelude/struct.Plane2d.html "struct bevy::prelude::Plane2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1192)

#### type [This](#associatedtype.This)<'from\_arg> = [Plane2d](../../../prelude/struct.Plane2d.html "struct bevy::prelude::Plane2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#96)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Plane3d](../../../prelude/struct.Plane3d.html "struct bevy::prelude::Plane3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#96)

#### type [This](#associatedtype.This)<'from\_arg> = [Plane3d](../../../prelude/struct.Plane3d.html "struct bevy::prelude::Plane3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/plane.rs.html#7)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PlaneMeshBuilder](../../../mesh/struct.PlaneMeshBuilder.html "struct bevy::mesh::PlaneMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/plane.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [PlaneMeshBuilder](../../../mesh/struct.PlaneMeshBuilder.html "struct bevy::mesh::PlaneMeshBuilder")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#9)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PlaybackMode](../../../audio/enum.PlaybackMode.html "enum bevy::audio::PlaybackMode")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#9)

#### type [This](#associatedtype.This)<'from\_arg> = [PlaybackMode](../../../audio/enum.PlaybackMode.html "enum bevy::audio::PlaybackMode")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#33)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PlaybackSettings](../../../prelude/struct.PlaybackSettings.html "struct bevy::prelude::PlaybackSettings")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#33)

#### type [This](#associatedtype.This)<'from\_arg> = [PlaybackSettings](../../../prelude/struct.PlaybackSettings.html "struct bevy::prelude::PlaybackSettings")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#38)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointLight](../../../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#38)

#### type [This](#associatedtype.This)<'from\_arg> = [PointLight](../../../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#177)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointLightShadowMap](../../../light/struct.PointLightShadowMap.html "struct bevy::light::PointLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#177)

#### type [This](#associatedtype.This)<'from\_arg> = [PointLightShadowMap](../../../light/struct.PointLightShadowMap.html "struct bevy::light::PointLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#159)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointLightTexture](../../../light/struct.PointLightTexture.html "struct bevy::light::PointLightTexture")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#159)

#### type [This](#associatedtype.This)<'from\_arg> = [PointLightTexture](../../../light/struct.PointLightTexture.html "struct bevy::light::PointLightTexture")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#248)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointerAction](../../../picking/pointer/enum.PointerAction.html "enum bevy::picking::pointer::PointerAction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#248)

#### type [This](#associatedtype.This)<'from\_arg> = [PointerAction](../../../picking/pointer/enum.PointerAction.html "enum bevy::picking::pointer::PointerAction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#159)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointerButton](../../../prelude/enum.PointerButton.html "enum bevy::prelude::PointerButton")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#159)

#### type [This](#associatedtype.This)<'from\_arg> = [PointerButton](../../../prelude/enum.PointerButton.html "enum bevy::prelude::PointerButton")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#91)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointerHits](../../../picking/backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#91)

#### type [This](#associatedtype.This)<'from\_arg> = [PointerHits](../../../picking/backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#31)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointerId](../../../picking/pointer/enum.PointerId.html "enum bevy::picking::pointer::PointerId")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#31)

#### type [This](#associatedtype.This)<'from\_arg> = [PointerId](../../../picking/pointer/enum.PointerId.html "enum bevy::picking::pointer::PointerId")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#278)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointerInput](../../../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#278)

#### type [This](#associatedtype.This)<'from\_arg> = [PointerInput](../../../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#42)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointerInputSettings](../../../picking/input/struct.PointerInputSettings.html "struct bevy::picking::input::PointerInputSettings")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#42)

#### type [This](#associatedtype.This)<'from\_arg> = [PointerInputSettings](../../../picking/input/struct.PointerInputSettings.html "struct bevy::picking::input::PointerInputSettings")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#71)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointerInteraction](../../../picking/pointer/struct.PointerInteraction.html "struct bevy::picking::pointer::PointerInteraction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#71)

#### type [This](#associatedtype.This)<'from\_arg> = [PointerInteraction](../../../picking/pointer/struct.PointerInteraction.html "struct bevy::picking::pointer::PointerInteraction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#178)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointerLocation](../../../picking/pointer/struct.PointerLocation.html "struct bevy::picking::pointer::PointerLocation")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#178)

#### type [This](#associatedtype.This)<'from\_arg> = [PointerLocation](../../../picking/pointer/struct.PointerLocation.html "struct bevy::picking::pointer::PointerLocation")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#114)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PointerPress](../../../picking/pointer/struct.PointerPress.html "struct bevy::picking::pointer::PointerPress")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#114)

#### type [This](#associatedtype.This)<'from\_arg> = [PointerPress](../../../picking/pointer/struct.PointerPress.html "struct bevy::picking::pointer::PointerPress")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1894)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Polygon](../../../prelude/struct.Polygon.html "struct bevy::prelude::Polygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1894)

#### type [This](#associatedtype.This)<'from\_arg> = [Polygon](../../../prelude/struct.Polygon.html "struct bevy::prelude::Polygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1566)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Polyline2d](../../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1566)

#### type [This](#associatedtype.This)<'from\_arg> = [Polyline2d](../../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#701)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Polyline2dMeshBuilder](../../../mesh/struct.Polyline2dMeshBuilder.html "struct bevy::mesh::Polyline2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#701)

#### type [This](#associatedtype.This)<'from\_arg> = [Polyline2dMeshBuilder](../../../mesh/struct.Polyline2dMeshBuilder.html "struct bevy::mesh::Polyline2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#624)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Polyline3d](../../../prelude/struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#624)

#### type [This](#associatedtype.This)<'from\_arg> = [Polyline3d](../../../prelude/struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#84)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Popover](../../../ui_widgets/popover/struct.Popover.html "struct bevy::ui_widgets::popover::Popover")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#84)

#### type [This](#associatedtype.This)<'from\_arg> = [Popover](../../../ui_widgets/popover/struct.Popover.html "struct bevy::ui_widgets::popover::Popover")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#52)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PopoverAlign](../../../ui_widgets/popover/enum.PopoverAlign.html "enum bevy::ui_widgets::popover::PopoverAlign")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#52)

#### type [This](#associatedtype.This)<'from\_arg> = [PopoverAlign](../../../ui_widgets/popover/enum.PopoverAlign.html "enum bevy::ui_widgets::popover::PopoverAlign")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#69)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PopoverPlacement](../../../ui_widgets/popover/struct.PopoverPlacement.html "struct bevy::ui_widgets::popover::PopoverPlacement")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#69)

#### type [This](#associatedtype.This)<'from\_arg> = [PopoverPlacement](../../../ui_widgets/popover/struct.PopoverPlacement.html "struct bevy::ui_widgets::popover::PopoverPlacement")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#23)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PopoverSide](../../../ui_widgets/popover/enum.PopoverSide.html "enum bevy::ui_widgets::popover::PopoverSide")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#23)

#### type [This](#associatedtype.This)<'from\_arg> = [PopoverSide](../../../ui_widgets/popover/enum.PopoverSide.html "enum bevy::ui_widgets::popover::PopoverSide")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1453)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PositionType](../../../prelude/enum.PositionType.html "enum bevy::prelude::PositionType")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1453)

#### type [This](#associatedtype.This)<'from\_arg> = [PositionType](../../../prelude/enum.PositionType.html "enum bevy::prelude::PositionType")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#13)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PositionedGlyph](../../../text/struct.PositionedGlyph.html "struct bevy::text::PositionedGlyph")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#13)

#### type [This](#associatedtype.This)<'from\_arg> = [PositionedGlyph](../../../text/struct.PositionedGlyph.html "struct bevy::text::PositionedGlyph")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#16)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PreeditCursor](../../../text/struct.PreeditCursor.html "struct bevy::text::PreeditCursor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#16)

#### type [This](#associatedtype.This)<'from\_arg> = [PreeditCursor](../../../text/struct.PreeditCursor.html "struct bevy::text::PreeditCursor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1214)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PresentMode](../../../window/enum.PresentMode.html "enum bevy::window::PresentMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1214)

#### type [This](#associatedtype.This)<'from\_arg> = [PresentMode](../../../window/enum.PresentMode.html "enum bevy::window::PresentMode")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#286)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Press](../../../prelude/struct.Press.html "struct bevy::prelude::Press")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#286)

#### type [This](#associatedtype.This)<'from\_arg> = [Press](../../../prelude/struct.Press.html "struct bevy::prelude::Press")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#149)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PressDirection](../../../picking/pointer/enum.PressDirection.html "enum bevy::picking::pointer::PressDirection")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#149)

#### type [This](#associatedtype.This)<'from\_arg> = [PressDirection](../../../picking/pointer/enum.PressDirection.html "enum bevy::picking::pointer::PressDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#44)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Pressed](../../../ui/struct.Pressed.html "struct bevy::ui::Pressed")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#44)

#### type [This](#associatedtype.This)<'from\_arg> = [Pressed](../../../ui/struct.Pressed.html "struct bevy::ui::Pressed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#53)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PrimaryMonitor](../../../window/struct.PrimaryMonitor.html "struct bevy::window::PrimaryMonitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#53)

#### type [This](#associatedtype.This)<'from\_arg> = [PrimaryMonitor](../../../window/struct.PrimaryMonitor.html "struct bevy::window::PrimaryMonitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#53)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PrimaryWindow](../../../window/struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#53)

#### type [This](#associatedtype.This)<'from\_arg> = [PrimaryWindow](../../../window/struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#214)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Projection](../../../prelude/enum.Projection.html "enum bevy::prelude::Projection")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#214)

#### type [This](#associatedtype.This)<'from\_arg> = [Projection](../../../prelude/enum.Projection.html "enum bevy::prelude::Projection")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Quat](../../../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### type [This](#associatedtype.This)<'from\_arg> = [Quat](../../../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#361)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RadialGradient](../../../prelude/struct.RadialGradient.html "struct bevy::prelude::RadialGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#361)

#### type [This](#associatedtype.This)<'from\_arg> = [RadialGradient](../../../prelude/struct.RadialGradient.html "struct bevy::prelude::RadialGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#558)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RadialGradientShape](../../../prelude/enum.RadialGradientShape.html "enum bevy::prelude::RadialGradientShape")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#558)

#### type [This](#associatedtype.This)<'from\_arg> = [RadialGradientShape](../../../prelude/enum.RadialGradientShape.html "enum bevy::prelude::RadialGradientShape")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#58)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RadioButton](../../../ui_widgets/struct.RadioButton.html "struct bevy::ui_widgets::RadioButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#58)

#### type [This](#associatedtype.This)<'from\_arg> = [RadioButton](../../../ui_widgets/struct.RadioButton.html "struct bevy::ui_widgets::RadioButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#40)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RadioGroup](../../../ui_widgets/struct.RadioGroup.html "struct bevy::ui_widgets::RadioGroup")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#40)

#### type [This](#associatedtype.This)<'from\_arg> = [RadioGroup](../../../ui_widgets/struct.RadioGroup.html "struct bevy::ui_widgets::RadioGroup")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#118)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RawGamepadAxisChangedEvent](../../../input/gamepad/struct.RawGamepadAxisChangedEvent.html "struct bevy::input::gamepad::RawGamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#118)

#### type [This](#associatedtype.This)<'from\_arg> = [RawGamepadAxisChangedEvent](../../../input/gamepad/struct.RawGamepadAxisChangedEvent.html "struct bevy::input::gamepad::RawGamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#86)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RawGamepadButtonChangedEvent](../../../input/gamepad/struct.RawGamepadButtonChangedEvent.html "struct bevy::input::gamepad::RawGamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#86)

#### type [This](#associatedtype.This)<'from\_arg> = [RawGamepadButtonChangedEvent](../../../input/gamepad/struct.RawGamepadButtonChangedEvent.html "struct bevy::input::gamepad::RawGamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#65)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RawGamepadEvent](../../../input/gamepad/enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#65)

#### type [This](#associatedtype.This)<'from\_arg> = [RawGamepadEvent](../../../input/gamepad/enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#17)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Ray2d](../../../prelude/struct.Ray2d.html "struct bevy::prelude::Ray2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#17)

#### type [This](#associatedtype.This)<'from\_arg> = [Ray2d](../../../prelude/struct.Ray2d.html "struct bevy::prelude::Ray2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#74)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Ray3d](../../../prelude/struct.Ray3d.html "struct bevy::prelude::Ray3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#74)

#### type [This](#associatedtype.This)<'from\_arg> = [Ray3d](../../../prelude/struct.Ray3d.html "struct bevy::prelude::Ray3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#12)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RayCast2d](../../../math/bounding/struct.RayCast2d.html "struct bevy::math::bounding::RayCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#12)

#### type [This](#associatedtype.This)<'from\_arg> = [RayCast2d](../../../math/bounding/struct.RayCast2d.html "struct bevy::math::bounding::RayCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#12)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RayCast3d](../../../math/bounding/struct.RayCast3d.html "struct bevy::math::bounding::RayCast3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#12)

#### type [This](#associatedtype.This)<'from\_arg> = [RayCast3d](../../../math/bounding/struct.RayCast3d.html "struct bevy::math::bounding::RayCast3d")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#106)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RayCastBackfaces](../../../prelude/struct.RayCastBackfaces.html "struct bevy::prelude::RayCastBackfaces")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#106)

#### type [This](#associatedtype.This)<'from\_arg> = [RayCastBackfaces](../../../prelude/struct.RayCastBackfaces.html "struct bevy::prelude::RayCastBackfaces")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#27)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RayCastVisibility](../../../prelude/enum.RayCastVisibility.html "enum bevy::prelude::RayCastVisibility")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#27)

#### type [This](#associatedtype.This)<'from\_arg> = [RayCastVisibility](../../../prelude/enum.RayCastVisibility.html "enum bevy::prelude::RayCastVisibility")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#245)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RayId](../../../picking/backend/ray/struct.RayId.html "struct bevy::picking::backend::ray::RayId")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#245)

#### type [This](#associatedtype.This)<'from\_arg> = [RayId](../../../picking/backend/ray/struct.RayId.html "struct bevy::picking::backend::ray::RayId")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/intersections.rs.html#8)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RayMeshHit](../../../picking/mesh_picking/ray_cast/struct.RayMeshHit.html "struct bevy::picking::mesh_picking::ray_cast::RayMeshHit")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/intersections.rs.html#8)

#### type [This](#associatedtype.This)<'from\_arg> = [RayMeshHit](../../../picking/mesh_picking/ray_cast/struct.RayMeshHit.html "struct bevy::picking::mesh_picking::ray_cast::RayMeshHit")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/scene/types.rs.html#19)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RaytracingMesh3d](../../../solari/scene/struct.RaytracingMesh3d.html "struct bevy::solari::scene::RaytracingMesh3d")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/scene/types.rs.html#19)

#### type [This](#associatedtype.This)<'from\_arg> = [RaytracingMesh3d](../../../solari/scene/struct.RaytracingMesh3d.html "struct bevy::solari::scene::RaytracingMesh3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#114)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ReadbackComplete](../../../render/gpu_readback/struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#114)

#### type [This](#associatedtype.This)<'from\_arg> = [ReadbackComplete](../../../render/gpu_readback/struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#44)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Real](../../../prelude/struct.Real.html "struct bevy::prelude::Real")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#44)

#### type [This](#associatedtype.This)<'from\_arg> = [Real](../../../prelude/struct.Real.html "struct bevy::prelude::Real")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/rect.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Rect](../../../prelude/struct.Rect.html "struct bevy::prelude::Rect")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/rect.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [Rect](../../../prelude/struct.Rect.html "struct bevy::prelude::Rect")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/rect_light.rs.html#18)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RectLight](../../../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/rect_light.rs.html#18)

#### type [This](#associatedtype.This)<'from\_arg> = [RectLight](../../../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1801)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Rectangle](../../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1801)

#### type [This](#associatedtype.This)<'from\_arg> = [Rectangle](../../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1041)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RectangleMeshBuilder](../../../mesh/struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1041)

#### type [This](#associatedtype.This)<'from\_arg> = [RectangleMeshBuilder](../../../mesh/struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2036)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RegularPolygon](../../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2036)

#### type [This](#associatedtype.This)<'from\_arg> = [RegularPolygon](../../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#482)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RegularPolygonMeshBuilder](../../../mesh/struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#482)

#### type [This](#associatedtype.This)<'from\_arg> = [RegularPolygonMeshBuilder](../../../mesh/struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#78)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RelativeCursorPosition](../../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#78)

#### type [This](#associatedtype.This)<'from\_arg> = [RelativeCursorPosition](../../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#298)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Release](../../../prelude/struct.Release.html "struct bevy::prelude::Release")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#298)

#### type [This](#associatedtype.This)<'from\_arg> = [Release](../../../prelude/struct.Release.html "struct bevy::prelude::Release")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#376)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Remove](../../../prelude/struct.Remove.html "struct bevy::prelude::Remove")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#376)

#### type [This](#associatedtype.This)<'from\_arg> = [Remove](../../../prelude/struct.Remove.html "struct bevy::prelude::Remove")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#399)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RemovedComponentEntity](../../../ecs/lifecycle/struct.RemovedComponentEntity.html "struct bevy::ecs::lifecycle::RemovedComponentEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#399)

#### type [This](#associatedtype.This)<'from\_arg> = [RemovedComponentEntity](../../../ecs/lifecycle/struct.RemovedComponentEntity.html "struct bevy::ecs::lifecycle::RemovedComponentEntity")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/render_asset.rs.html#29)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderAssetUsages](../../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/render_asset.rs.html#29)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderAssetUsages](../../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#319)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderDebugMode](../../../dev_tools/render_debug/enum.RenderDebugMode.html "enum bevy::dev_tools::render_debug::RenderDebugMode")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#319)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderDebugMode](../../../dev_tools/render_debug/enum.RenderDebugMode.html "enum bevy::dev_tools::render_debug::RenderDebugMode")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderDebugOverlay](../../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderDebugOverlay](../../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#262)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderDebugOverlayEvent](../../../dev_tools/render_debug/enum.RenderDebugOverlayEvent.html "enum bevy::dev_tools::render_debug::RenderDebugOverlayEvent")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#262)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderDebugOverlayEvent](../../../dev_tools/render_debug/enum.RenderDebugOverlayEvent.html "enum bevy::dev_tools::render_debug::RenderDebugOverlayEvent")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#129)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderEntity](../../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#129)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderEntity](../../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/render_layers.rs.html#18)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderLayers](../../../camera/visibility/struct.RenderLayers.html "struct bevy::camera::visibility::RenderLayers")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/render_layers.rs.html#18)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderLayers](../../../camera/visibility/struct.RenderLayers.html "struct bevy::camera::visibility::RenderLayers")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#63)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderShadowMapVisibleEntities](../../../render/view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#63)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderShadowMapVisibleEntities](../../../render/view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#890)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderTarget](../../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#890)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderTarget](../../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#196)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderTargetInfo](../../../camera/struct.RenderTargetInfo.html "struct bevy::camera::RenderTargetInfo")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#196)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderTargetInfo](../../../camera/struct.RenderTargetInfo.html "struct bevy::camera::RenderTargetInfo")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#87)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RenderVisibleEntitiesClass](../../../render/view/struct.RenderVisibleEntitiesClass.html "struct bevy::render::view::RenderVisibleEntitiesClass")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#87)

#### type [This](#associatedtype.This)<'from\_arg> = [RenderVisibleEntitiesClass](../../../render/view/struct.RenderVisibleEntitiesClass.html "struct bevy::render::view::RenderVisibleEntitiesClass")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#469)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RepeatAnimation](../../../animation/enum.RepeatAnimation.html "enum bevy::animation::RepeatAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#469)

#### type [This](#associatedtype.This)<'from\_arg> = [RepeatAnimation](../../../animation/enum.RepeatAnimation.html "enum bevy::animation::RepeatAnimation")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RepeatedGridTrack](../../../prelude/struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

#### type [This](#associatedtype.This)<'from\_arg> = [RepeatedGridTrack](../../../prelude/struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#53)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RequestRedraw](../../../window/struct.RequestRedraw.html "struct bevy::window::RequestRedraw")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#53)

#### type [This](#associatedtype.This)<'from\_arg> = [RequestRedraw](../../../window/struct.RequestRedraw.html "struct bevy::window::RequestRedraw")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2802)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ResolvedBorderRadius](../../../prelude/struct.ResolvedBorderRadius.html "struct bevy::prelude::ResolvedBorderRadius")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2802)

#### type [This](#associatedtype.This)<'from\_arg> = [ResolvedBorderRadius](../../../prelude/struct.ResolvedBorderRadius.html "struct bevy::prelude::ResolvedBorderRadius")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1055)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Rhombus](../../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1055)

#### type [This](#associatedtype.This)<'from\_arg> = [Rhombus](../../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#878)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RhombusMeshBuilder](../../../mesh/struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#878)

#### type [This](#associatedtype.This)<'from\_arg> = [RhombusMeshBuilder](../../../mesh/struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#120)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RootNonCameraView](../../../core_pipeline/schedule/struct.RootNonCameraView.html "struct bevy::core_pipeline::schedule::RootNonCameraView")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#120)

#### type [This](#associatedtype.This)<'from\_arg> = [RootNonCameraView](../../../core_pipeline/schedule/struct.RootNonCameraView.html "struct bevy::core_pipeline::schedule::RootNonCameraView")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rotation2d.rs.html#44)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Rot2](../../../prelude/struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rotation2d.rs.html#44)

#### type [This](#associatedtype.This)<'from\_arg> = [Rot2](../../../prelude/struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#47)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RotationGesture](../../../input/gestures/struct.RotationGesture.html "struct bevy::input::gestures::RotationGesture")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#47)

#### type [This](#associatedtype.This)<'from\_arg> = [RotationGesture](../../../input/gestures/struct.RotationGesture.html "struct bevy::input::gestures::RotationGesture")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#501)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RunGeometry](../../../text/struct.RunGeometry.html "struct bevy::text::RunGeometry")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#501)

#### type [This](#associatedtype.This)<'from\_arg> = [RunGeometry](../../../text/struct.RunGeometry.html "struct bevy::text::RunGeometry")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#521)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScalingMode](../../../camera/enum.ScalingMode.html "enum bevy::camera::ScalingMode")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#521)

#### type [This](#associatedtype.This)<'from\_arg> = [ScalingMode](../../../camera/enum.ScalingMode.html "enum bevy::camera::ScalingMode")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_component.rs.html#22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SceneComponentInfo](../../../scene/struct.SceneComponentInfo.html "struct bevy::scene::SceneComponentInfo")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_component.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [SceneComponentInfo](../../../scene/struct.SceneComponentInfo.html "struct bevy::scene::SceneComponentInfo")

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/mod.rs.html#18)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SchemaTypesMetadata](../../../remote/schemas/struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata")

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/mod.rs.html#18)

#### type [This](#associatedtype.This)<'from\_arg> = [SchemaTypesMetadata](../../../remote/schemas/struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1478)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScreenEdge](../../../window/enum.ScreenEdge.html "enum bevy::window::ScreenEdge")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1478)

#### type [This](#associatedtype.This)<'from\_arg> = [ScreenEdge](../../../window/enum.ScreenEdge.html "enum bevy::window::ScreenEdge")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScreenSpaceAmbientOcclusion](../../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

#### type [This](#associatedtype.This)<'from\_arg> = [ScreenSpaceAmbientOcclusion](../../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#135)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScreenSpaceAmbientOcclusionQualityLevel](../../../pbr/enum.ScreenSpaceAmbientOcclusionQualityLevel.html "enum bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#135)

#### type [This](#associatedtype.This)<'from\_arg> = [ScreenSpaceAmbientOcclusionQualityLevel](../../../pbr/enum.ScreenSpaceAmbientOcclusionQualityLevel.html "enum bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#78)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScreenSpaceReflections](../../../pbr/struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#78)

#### type [This](#associatedtype.This)<'from\_arg> = [ScreenSpaceReflections](../../../pbr/struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScreenSpaceTransmission](../../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

#### type [This](#associatedtype.This)<'from\_arg> = [ScreenSpaceTransmission](../../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#110)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScreenSpaceTransmissionQuality](../../../pbr/enum.ScreenSpaceTransmissionQuality.html "enum bevy::pbr::ScreenSpaceTransmissionQuality")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#110)

#### type [This](#associatedtype.This)<'from\_arg> = [ScreenSpaceTransmissionQuality](../../../pbr/enum.ScreenSpaceTransmissionQuality.html "enum bevy::pbr::ScreenSpaceTransmissionQuality")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#78)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Screenshot](../../../render/view/window/screenshot/struct.Screenshot.html "struct bevy::render::view::window::screenshot::Screenshot")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#78)

#### type [This](#associatedtype.This)<'from\_arg> = [Screenshot](../../../render/view/window/screenshot/struct.Screenshot.html "struct bevy::render::view::window::screenshot::Screenshot")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#47)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScreenshotCaptured](../../../render/view/window/screenshot/struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#47)

#### type [This](#associatedtype.This)<'from\_arg> = [ScreenshotCaptured](../../../render/view/window/screenshot/struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#455)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Scroll](../../../prelude/struct.Scroll.html "struct bevy::prelude::Scroll")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#455)

#### type [This](#associatedtype.This)<'from\_arg> = [Scroll](../../../prelude/struct.Scroll.html "struct bevy::prelude::Scroll")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollarea.rs.html#16)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScrollArea](../../../ui_widgets/struct.ScrollArea.html "struct bevy::ui_widgets::ScrollArea")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollarea.rs.html#16)

#### type [This](#associatedtype.This)<'from\_arg> = [ScrollArea](../../../ui_widgets/struct.ScrollArea.html "struct bevy::ui_widgets::ScrollArea")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#417)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScrollPosition](../../../prelude/struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#417)

#### type [This](#associatedtype.This)<'from\_arg> = [ScrollPosition](../../../prelude/struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#67)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Scrollbar](../../../ui_widgets/struct.Scrollbar.html "struct bevy::ui_widgets::Scrollbar")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#67)

#### type [This](#associatedtype.This)<'from\_arg> = [Scrollbar](../../../ui_widgets/struct.Scrollbar.html "struct bevy::ui_widgets::Scrollbar")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#130)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScrollbarDragState](../../../ui_widgets/struct.ScrollbarDragState.html "struct bevy::ui_widgets::ScrollbarDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#130)

#### type [This](#associatedtype.This)<'from\_arg> = [ScrollbarDragState](../../../ui_widgets/struct.ScrollbarDragState.html "struct bevy::ui_widgets::ScrollbarDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#100)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ScrollbarThumb](../../../ui_widgets/struct.ScrollbarThumb.html "struct bevy::ui_widgets::ScrollbarThumb")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#100)

#### type [This](#associatedtype.This)<'from\_arg> = [ScrollbarThumb](../../../ui_widgets/struct.ScrollbarThumb.html "struct bevy::ui_widgets::ScrollbarThumb")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1254)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Segment2d](../../../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1254)

#### type [This](#associatedtype.This)<'from\_arg> = [Segment2d](../../../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#376)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Segment3d](../../../prelude/struct.Segment3d.html "struct bevy::prelude::Segment3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#376)

#### type [This](#associatedtype.This)<'from\_arg> = [Segment3d](../../../prelude/struct.Segment3d.html "struct bevy::prelude::Segment3d")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/text_input.rs.html#406)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SelectAllOnFocus](../../../ui_widgets/struct.SelectAllOnFocus.html "struct bevy::ui_widgets::SelectAllOnFocus")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/text_input.rs.html#406)

#### type [This](#associatedtype.This)<'from\_arg> = [SelectAllOnFocus](../../../ui_widgets/struct.SelectAllOnFocus.html "struct bevy::ui_widgets::SelectAllOnFocus")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#29)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Sensitivity](../../../anti_alias/fxaa/enum.Sensitivity.html "enum bevy::anti_alias::fxaa::Sensitivity")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#29)

#### type [This](#associatedtype.This)<'from\_arg> = [Sensitivity](../../../anti_alias/fxaa/enum.Sensitivity.html "enum bevy::anti_alias::fxaa::Sensitivity")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#181)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SetChecked](../../../ui_widgets/struct.SetChecked.html "struct bevy::ui_widgets::SetChecked")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#181)

#### type [This](#associatedtype.This)<'from\_arg> = [SetChecked](../../../ui_widgets/struct.SetChecked.html "struct bevy::ui_widgets::SetChecked")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#673)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SetSliderValue](../../../ui_widgets/struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#673)

#### type [This](#associatedtype.This)<'from\_arg> = [SetSliderValue](../../../ui_widgets/struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#27)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ShaderBuffer](../../../render/storage/struct.ShaderBuffer.html "struct bevy::render::storage::ShaderBuffer")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#27)

#### type [This](#associatedtype.This)<'from\_arg> = [ShaderBuffer](../../../render/storage/struct.ShaderBuffer.html "struct bevy::render::storage::ShaderBuffer")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#283)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ShadowFilteringMethod](../../../light/enum.ShadowFilteringMethod.html "enum bevy::light::ShadowFilteringMethod")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#283)

#### type [This](#associatedtype.This)<'from\_arg> = [ShadowFilteringMethod](../../../light/enum.ShadowFilteringMethod.html "enum bevy::light::ShadowFilteringMethod")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#854)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ShadowLodOrigin](../../../camera/struct.ShadowLodOrigin.html "struct bevy::camera::ShadowLodOrigin")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#854)

#### type [This](#associatedtype.This)<'from\_arg> = [ShadowLodOrigin](../../../camera/struct.ShadowLodOrigin.html "struct bevy::camera::ShadowLodOrigin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2868)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ShadowStyle](../../../prelude/struct.ShadowStyle.html "struct bevy::prelude::ShadowStyle")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2868)

#### type [This](#associatedtype.This)<'from\_arg> = [ShadowStyle](../../../prelude/struct.ShadowStyle.html "struct bevy::prelude::ShadowStyle")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#61)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ShowAabbGizmo](../../../prelude/struct.ShowAabbGizmo.html "struct bevy::prelude::ShowAabbGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#61)

#### type [This](#associatedtype.This)<'from\_arg> = [ShowAabbGizmo](../../../prelude/struct.ShowAabbGizmo.html "struct bevy::prelude::ShowAabbGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#96)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ShowFrustumGizmo](../../../prelude/struct.ShowFrustumGizmo.html "struct bevy::prelude::ShowFrustumGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#96)

#### type [This](#associatedtype.This)<'from\_arg> = [ShowFrustumGizmo](../../../prelude/struct.ShowFrustumGizmo.html "struct bevy::prelude::ShowFrustumGizmo")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#210)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ShowLightGizmo](../../../prelude/struct.ShowLightGizmo.html "struct bevy::prelude::ShowLightGizmo")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#210)

#### type [This](#associatedtype.This)<'from\_arg> = [ShowLightGizmo](../../../prelude/struct.ShowLightGizmo.html "struct bevy::prelude::ShowLightGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#76)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ShowSkinnedMeshBoundsGizmo](../../../prelude/struct.ShowSkinnedMeshBoundsGizmo.html "struct bevy::prelude::ShowSkinnedMeshBoundsGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#76)

#### type [This](#associatedtype.This)<'from\_arg> = [ShowSkinnedMeshBoundsGizmo](../../../prelude/struct.ShowSkinnedMeshBoundsGizmo.html "struct bevy::prelude::ShowSkinnedMeshBoundsGizmo")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#113)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SimplifiedMesh](../../../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#113)

#### type [This](#associatedtype.This)<'from\_arg> = [SimplifiedMesh](../../../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#16)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SkinnedMesh](../../../mesh/skinning/struct.SkinnedMesh.html "struct bevy::mesh::skinning::SkinnedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#16)

#### type [This](#associatedtype.This)<'from\_arg> = [SkinnedMesh](../../../mesh/skinning/struct.SkinnedMesh.html "struct bevy::mesh::skinning::SkinnedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#88)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SkinnedMeshBounds](../../../mesh/skinning/struct.SkinnedMeshBounds.html "struct bevy::mesh::skinning::SkinnedMeshBounds")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#88)

#### type [This](#associatedtype.This)<'from\_arg> = [SkinnedMeshBounds](../../../mesh/skinning/struct.SkinnedMeshBounds.html "struct bevy::mesh::skinning::SkinnedMeshBounds")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#52)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SkinnedMeshBoundsGizmoConfigGroup](../../../prelude/struct.SkinnedMeshBoundsGizmoConfigGroup.html "struct bevy::prelude::SkinnedMeshBoundsGizmoConfigGroup")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#52)

#### type [This](#associatedtype.This)<'from\_arg> = [SkinnedMeshBoundsGizmoConfigGroup](../../../prelude/struct.SkinnedMeshBoundsGizmoConfigGroup.html "struct bevy::prelude::SkinnedMeshBoundsGizmoConfigGroup")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#227)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Skybox](../../../core_pipeline/struct.Skybox.html "struct bevy::core_pipeline::Skybox")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#227)

#### type [This](#associatedtype.This)<'from\_arg> = [Skybox](../../../core_pipeline/struct.Skybox.html "struct bevy::core_pipeline::Skybox")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#27)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliceScaleMode](../../../prelude/enum.SliceScaleMode.html "enum bevy::prelude::SliceScaleMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#27)

#### type [This](#associatedtype.This)<'from\_arg> = [SliceScaleMode](../../../prelude/enum.SliceScaleMode.html "enum bevy::prelude::SliceScaleMode")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#103)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Slider](../../../ui_widgets/struct.Slider.html "struct bevy::ui_widgets::Slider")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#103)

#### type [This](#associatedtype.This)<'from\_arg> = [Slider](../../../ui_widgets/struct.Slider.html "struct bevy::ui_widgets::Slider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#147)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliderBaseColor](../../../feathers/controls/struct.SliderBaseColor.html "struct bevy::feathers::controls::SliderBaseColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#147)

#### type [This](#associatedtype.This)<'from\_arg> = [SliderBaseColor](../../../feathers/controls/struct.SliderBaseColor.html "struct bevy::feathers::controls::SliderBaseColor")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#245)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliderDragState](../../../ui_widgets/struct.SliderDragState.html "struct bevy::ui_widgets::SliderDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#245)

#### type [This](#associatedtype.This)<'from\_arg> = [SliderDragState](../../../ui_widgets/struct.SliderDragState.html "struct bevy::ui_widgets::SliderDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#35)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliderOrientation](../../../ui_widgets/enum.SliderOrientation.html "enum bevy::ui_widgets::SliderOrientation")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#35)

#### type [This](#associatedtype.This)<'from\_arg> = [SliderOrientation](../../../ui_widgets/enum.SliderOrientation.html "enum bevy::ui_widgets::SliderOrientation")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#233)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliderPrecision](../../../ui_widgets/struct.SliderPrecision.html "struct bevy::ui_widgets::SliderPrecision")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#233)

#### type [This](#associatedtype.This)<'from\_arg> = [SliderPrecision](../../../ui_widgets/struct.SliderPrecision.html "struct bevy::ui_widgets::SliderPrecision")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#127)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliderRange](../../../ui_widgets/struct.SliderRange.html "struct bevy::ui_widgets::SliderRange")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#127)

#### type [This](#associatedtype.This)<'from\_arg> = [SliderRange](../../../ui_widgets/struct.SliderRange.html "struct bevy::ui_widgets::SliderRange")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#214)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliderStep](../../../ui_widgets/struct.SliderStep.html "struct bevy::ui_widgets::SliderStep")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#214)

#### type [This](#associatedtype.This)<'from\_arg> = [SliderStep](../../../ui_widgets/struct.SliderStep.html "struct bevy::ui_widgets::SliderStep")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#113)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliderThumb](../../../ui_widgets/struct.SliderThumb.html "struct bevy::ui_widgets::SliderThumb")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#113)

#### type [This](#associatedtype.This)<'from\_arg> = [SliderThumb](../../../ui_widgets/struct.SliderThumb.html "struct bevy::ui_widgets::SliderThumb")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#120)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliderValue](../../../ui_widgets/struct.SliderValue.html "struct bevy::ui_widgets::SliderValue")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#120)

#### type [This](#associatedtype.This)<'from\_arg> = [SliderValue](../../../ui_widgets/struct.SliderValue.html "struct bevy::ui_widgets::SliderValue")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#683)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SliderValueChange](../../../ui_widgets/enum.SliderValueChange.html "enum bevy::ui_widgets::SliderValueChange")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#683)

#### type [This](#associatedtype.This)<'from\_arg> = [SliderValueChange](../../../ui_widgets/enum.SliderValueChange.html "enum bevy::ui_widgets::SliderValueChange")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Smaa](../../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

#### type [This](#associatedtype.This)<'from\_arg> = [Smaa](../../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#106)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SmaaPreset](../../../anti_alias/smaa/enum.SmaaPreset.html "enum bevy::anti_alias::smaa::SmaaPreset")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#106)

#### type [This](#associatedtype.This)<'from\_arg> = [SmaaPreset](../../../anti_alias/smaa/enum.SmaaPreset.html "enum bevy::anti_alias::smaa::SmaaPreset")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/realtime/mod.rs.html#85)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SolariLighting](../../../solari/realtime/struct.SolariLighting.html "struct bevy::solari::realtime::SolariLighting")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/realtime/mod.rs.html#85)

#### type [This](#associatedtype.This)<'from\_arg> = [SolariLighting](../../../solari/realtime/struct.SolariLighting.html "struct bevy::solari::realtime::SolariLighting")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#170)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpatialListener](../../../prelude/struct.SpatialListener.html "struct bevy::prelude::SpatialListener")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#170)

#### type [This](#associatedtype.This)<'from\_arg> = [SpatialListener](../../../prelude/struct.SpatialListener.html "struct bevy::prelude::SpatialListener")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#203)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpatialScale](../../../audio/struct.SpatialScale.html "struct bevy::audio::SpatialScale")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#203)

#### type [This](#associatedtype.This)<'from\_arg> = [SpatialScale](../../../audio/struct.SpatialScale.html "struct bevy::audio::SpatialScale")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#23)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for bevy::prelude::[Sphere](../../../prelude/struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#23)

#### type [This](#associatedtype.This)<'from\_arg> = [Sphere](../../../prelude/struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#196)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for bevy::camera::primitives::[Sphere](../../../camera/primitives/struct.Sphere.html "struct bevy::camera::primitives::Sphere")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#196)

#### type [This](#associatedtype.This)<'from\_arg> = [Sphere](../../../camera/primitives/struct.Sphere.html "struct bevy::camera::primitives::Sphere")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#23)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SphereKind](../../../mesh/enum.SphereKind.html "enum bevy::mesh::SphereKind")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#23)

#### type [This](#associatedtype.This)<'from\_arg> = [SphereKind](../../../mesh/enum.SphereKind.html "enum bevy::mesh::SphereKind")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#51)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SphereMeshBuilder](../../../mesh/struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#51)

#### type [This](#associatedtype.This)<'from\_arg> = [SphereMeshBuilder](../../../mesh/struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpotLight](../../../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [SpotLight](../../../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#204)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpotLightTexture](../../../light/struct.SpotLightTexture.html "struct bevy::light::SpotLightTexture")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#204)

#### type [This](#associatedtype.This)<'from\_arg> = [SpotLightTexture](../../../light/struct.SpotLightTexture.html "struct bevy::light::SpotLightTexture")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#15)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Sprite](../../../prelude/struct.Sprite.html "struct bevy::prelude::Sprite")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#15)

#### type [This](#associatedtype.This)<'from\_arg> = [Sprite](../../../prelude/struct.Sprite.html "struct bevy::prelude::Sprite")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#178)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpriteAlphaMode](../../../sprite/enum.SpriteAlphaMode.html "enum bevy::sprite::SpriteAlphaMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#178)

#### type [This](#associatedtype.This)<'from\_arg> = [SpriteAlphaMode](../../../sprite/enum.SpriteAlphaMode.html "enum bevy::sprite::SpriteAlphaMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#166)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpriteImageMode](../../../prelude/enum.SpriteImageMode.html "enum bevy::prelude::SpriteImageMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#166)

#### type [This](#associatedtype.This)<'from\_arg> = [SpriteImageMode](../../../prelude/enum.SpriteImageMode.html "enum bevy::prelude::SpriteImageMode")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpriteMaterial](../../../prelude/struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

#### type [This](#associatedtype.This)<'from\_arg> = [SpriteMaterial](../../../prelude/struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#16)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpriteMesh](../../../prelude/struct.SpriteMesh.html "struct bevy::prelude::SpriteMesh")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#16)

#### type [This](#associatedtype.This)<'from\_arg> = [SpriteMesh](../../../prelude/struct.SpriteMesh.html "struct bevy::prelude::SpriteMesh")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#34)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpritePickingCamera](../../../prelude/struct.SpritePickingCamera.html "struct bevy::prelude::SpritePickingCamera")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#34)

#### type [This](#associatedtype.This)<'from\_arg> = [SpritePickingCamera](../../../prelude/struct.SpritePickingCamera.html "struct bevy::prelude::SpritePickingCamera")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#39)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpritePickingMode](../../../prelude/enum.SpritePickingMode.html "enum bevy::prelude::SpritePickingMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#39)

#### type [This](#associatedtype.This)<'from\_arg> = [SpritePickingMode](../../../prelude/enum.SpritePickingMode.html "enum bevy::prelude::SpritePickingMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#51)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpritePickingSettings](../../../prelude/struct.SpritePickingSettings.html "struct bevy::prelude::SpritePickingSettings")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#51)

#### type [This](#associatedtype.This)<'from\_arg> = [SpritePickingSettings](../../../prelude/struct.SpritePickingSettings.html "struct bevy::prelude::SpritePickingSettings")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#214)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SpriteScalingMode](../../../prelude/enum.SpriteScalingMode.html "enum bevy::prelude::SpriteScalingMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#214)

#### type [This](#associatedtype.This)<'from\_arg> = [SpriteScalingMode](../../../prelude/enum.SpriteScalingMode.html "enum bevy::prelude::SpriteScalingMode")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#20)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Srgba](../../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#20)

#### type [This](#associatedtype.This)<'from\_arg> = [Srgba](../../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [StandardMaterial](../../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [StandardMaterial](../../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#88)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [StaticTransformOptimizations](../../../prelude/enum.StaticTransformOptimizations.html "enum bevy::prelude::StaticTransformOptimizations")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#88)

#### type [This](#associatedtype.This)<'from\_arg> = [StaticTransformOptimizations](../../../prelude/enum.StaticTransformOptimizations.html "enum bevy::prelude::StaticTransformOptimizations")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/stopwatch.rs.html#31)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Stopwatch](../../../time/struct.Stopwatch.html "struct bevy::time::Stopwatch")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/stopwatch.rs.html#31)

#### type [This](#associatedtype.This)<'from\_arg> = [Stopwatch](../../../time/struct.Stopwatch.html "struct bevy::time::Stopwatch")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1132)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Strikethrough](../../../prelude/struct.Strikethrough.html "struct bevy::prelude::Strikethrough")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1132)

#### type [This](#associatedtype.This)<'from\_arg> = [Strikethrough](../../../prelude/struct.Strikethrough.html "struct bevy::prelude::Strikethrough")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1137)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [StrikethroughColor](../../../prelude/struct.StrikethroughColor.html "struct bevy::prelude::StrikethroughColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1137)

#### type [This](#associatedtype.This)<'from\_arg> = [StrikethroughColor](../../../prelude/struct.StrikethroughColor.html "struct bevy::prelude::StrikethroughColor")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [String](../../../prelude/struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### type [This](#associatedtype.This)<'from\_arg> = [String](../../../prelude/struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#174)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SubCameraView](../../../camera/struct.SubCameraView.html "struct bevy::camera::SubCameraView")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#174)

#### type [This](#associatedtype.This)<'from\_arg> = [SubCameraView](../../../camera/struct.SubCameraView.html "struct bevy::camera::SubCameraView")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#121)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SyncToRenderWorld](../../../render/sync_world/struct.SyncToRenderWorld.html "struct bevy::render::sync_world::SyncToRenderWorld")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#121)

#### type [This](#associatedtype.This)<'from\_arg> = [SyncToRenderWorld](../../../render/sync_world/struct.SyncToRenderWorld.html "struct bevy::render::sync_world::SyncToRenderWorld")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/system_cursor.rs.html#89)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SystemCursorIcon](../../../window/enum.SystemCursorIcon.html "enum bevy::window::SystemCursorIcon")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/system_cursor.rs.html#89)

#### type [This](#associatedtype.This)<'from\_arg> = [SystemCursorIcon](../../../window/enum.SystemCursorIcon.html "enum bevy::window::SystemCursorIcon")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#69)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TabGroup](../../../input_focus/tab_navigation/struct.TabGroup.html "struct bevy::input_focus::tab_navigation::TabGroup")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#69)

#### type [This](#associatedtype.This)<'from\_arg> = [TabGroup](../../../input_focus/tab_navigation/struct.TabGroup.html "struct bevy::input_focus::tab_navigation::TabGroup")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#60)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TabIndex](../../../input_focus/tab_navigation/struct.TabIndex.html "struct bevy::input_focus::tab_navigation::TabIndex")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#60)

#### type [This](#associatedtype.This)<'from\_arg> = [TabIndex](../../../input_focus/tab_navigation/struct.TabIndex.html "struct bevy::input_focus::tab_navigation::TabIndex")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#111)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TemporalAntiAliasing](../../../anti_alias/taa/struct.TemporalAntiAliasing.html "struct bevy::anti_alias::taa::TemporalAntiAliasing")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#111)

#### type [This](#associatedtype.This)<'from\_arg> = [TemporalAntiAliasing](../../../anti_alias/taa/struct.TemporalAntiAliasing.html "struct bevy::anti_alias::taa::TemporalAntiAliasing")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#780)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TemporalJitter](../../../render/camera/struct.TemporalJitter.html "struct bevy::render::camera::TemporalJitter")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#780)

#### type [This](#associatedtype.This)<'from\_arg> = [TemporalJitter](../../../render/camera/struct.TemporalJitter.html "struct bevy::render::camera::TemporalJitter")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#190)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TemporaryRenderEntity](../../../render/sync_world/struct.TemporaryRenderEntity.html "struct bevy::render::sync_world::TemporaryRenderEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#190)

#### type [This](#associatedtype.This)<'from\_arg> = [TemporaryRenderEntity](../../../render/sync_world/struct.TemporaryRenderEntity.html "struct bevy::render::sync_world::TemporaryRenderEntity")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1433)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Tetrahedron](../../../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1433)

#### type [This](#associatedtype.This)<'from\_arg> = [Tetrahedron](../../../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#8)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TetrahedronMeshBuilder](../../../mesh/struct.TetrahedronMeshBuilder.html "struct bevy::mesh::TetrahedronMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#8)

#### type [This](#associatedtype.This)<'from\_arg> = [TetrahedronMeshBuilder](../../../mesh/struct.TetrahedronMeshBuilder.html "struct bevy::mesh::TetrahedronMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#97)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Text](../../../prelude/struct.Text.html "struct bevy::prelude::Text")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#97)

#### type [This](#associatedtype.This)<'from\_arg> = [Text](../../../prelude/struct.Text.html "struct bevy::prelude::Text")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#85)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Text2d](../../../prelude/struct.Text2d.html "struct bevy::prelude::Text2d")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#85)

#### type [This](#associatedtype.This)<'from\_arg> = [Text2d](../../../prelude/struct.Text2d.html "struct bevy::prelude::Text2d")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#141)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Text2dShadow](../../../sprite/struct.Text2dShadow.html "struct bevy::sprite::Text2dShadow")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#141)

#### type [This](#associatedtype.This)<'from\_arg> = [Text2dShadow](../../../sprite/struct.Text2dShadow.html "struct bevy::sprite::Text2dShadow")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1088)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextBackgroundColor](../../../prelude/struct.TextBackgroundColor.html "struct bevy::prelude::TextBackgroundColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1088)

#### type [This](#associatedtype.This)<'from\_arg> = [TextBackgroundColor](../../../prelude/struct.TextBackgroundColor.html "struct bevy::prelude::TextBackgroundColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/bounds.rs.html#13)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextBounds](../../../text/struct.TextBounds.html "struct bevy::text::TextBounds")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/bounds.rs.html#13)

#### type [This](#associatedtype.This)<'from\_arg> = [TextBounds](../../../text/struct.TextBounds.html "struct bevy::text::TextBounds")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1064)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextColor](../../../prelude/struct.TextColor.html "struct bevy::prelude::TextColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1064)

#### type [This](#associatedtype.This)<'from\_arg> = [TextColor](../../../prelude/struct.TextColor.html "struct bevy::prelude::TextColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#25)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextEdit](../../../text/enum.TextEdit.html "enum bevy::text::TextEdit")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#25)

#### type [This](#associatedtype.This)<'from\_arg> = [TextEdit](../../../text/enum.TextEdit.html "enum bevy::text::TextEdit")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextEntity](../../../text/struct.TextEntity.html "struct bevy::text::TextEntity")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [TextEntity](../../../text/struct.TextEntity.html "struct bevy::text::TextEntity")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#374)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextFont](../../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#374)

#### type [This](#associatedtype.This)<'from\_arg> = [TextFont](../../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#130)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextLayout](../../../prelude/struct.TextLayout.html "struct bevy::prelude::TextLayout")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#130)

#### type [This](#associatedtype.This)<'from\_arg> = [TextLayout](../../../prelude/struct.TextLayout.html "struct bevy::prelude::TextLayout")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#461)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextLayoutInfo](../../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#461)

#### type [This](#associatedtype.This)<'from\_arg> = [TextLayoutInfo](../../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#32)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextNodeFlags](../../../ui/struct.TextNodeFlags.html "struct bevy::ui::TextNodeFlags")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#32)

#### type [This](#associatedtype.This)<'from\_arg> = [TextNodeFlags](../../../ui/struct.TextNodeFlags.html "struct bevy::ui::TextNodeFlags")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text_input_layout.rs.html#32)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextScroll](../../../ui/widget/struct.TextScroll.html "struct bevy::ui::widget::TextScroll")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text_input_layout.rs.html#32)

#### type [This](#associatedtype.This)<'from\_arg> = [TextScroll](../../../ui/widget/struct.TextScroll.html "struct bevy::ui::widget::TextScroll")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#144)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextShadow](../../../prelude/struct.TextShadow.html "struct bevy::prelude::TextShadow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#144)

#### type [This](#associatedtype.This)<'from\_arg> = [TextShadow](../../../prelude/struct.TextShadow.html "struct bevy::prelude::TextShadow")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextSpan](../../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### type [This](#associatedtype.This)<'from\_arg> = [TextSpan](../../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#211)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextureAtlas](../../../prelude/struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#211)

#### type [This](#associatedtype.This)<'from\_arg> = [TextureAtlas](../../../prelude/struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#95)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextureAtlasLayout](../../../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#95)

#### type [This](#associatedtype.This)<'from\_arg> = [TextureAtlasLayout](../../../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/wgpu_types.rs.html#3-10)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextureFormat](../../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/wgpu_types.rs.html#3-10)

#### type [This](#associatedtype.This)<'from\_arg> = [TextureFormat](../../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#13)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextureSlicer](../../../prelude/struct.TextureSlicer.html "struct bevy::prelude::TextureSlicer")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#13)

#### type [This](#associatedtype.This)<'from\_arg> = [TextureSlicer](../../../prelude/struct.TextureSlicer.html "struct bevy::prelude::TextureSlicer")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#90)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ThemeBackgroundColor](../../../feathers/theme/struct.ThemeBackgroundColor.html "struct bevy::feathers::theme::ThemeBackgroundColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#90)

#### type [This](#associatedtype.This)<'from\_arg> = [ThemeBackgroundColor](../../../feathers/theme/struct.ThemeBackgroundColor.html "struct bevy::feathers::theme::ThemeBackgroundColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#99)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ThemeBorderColor](../../../feathers/theme/struct.ThemeBorderColor.html "struct bevy::feathers::theme::ThemeBorderColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#99)

#### type [This](#associatedtype.This)<'from\_arg> = [ThemeBorderColor](../../../feathers/theme/struct.ThemeBorderColor.html "struct bevy::feathers::theme::ThemeBorderColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#50)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ThemeProps](../../../feathers/theme/struct.ThemeProps.html "struct bevy::feathers::theme::ThemeProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#50)

#### type [This](#associatedtype.This)<'from\_arg> = [ThemeProps](../../../feathers/theme/struct.ThemeProps.html "struct bevy::feathers::theme::ThemeProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#118)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ThemeTextColor](../../../feathers/theme/struct.ThemeTextColor.html "struct bevy::feathers::theme::ThemeTextColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#118)

#### type [This](#associatedtype.This)<'from\_arg> = [ThemeTextColor](../../../feathers/theme/struct.ThemeTextColor.html "struct bevy::feathers::theme::ThemeTextColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#22)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ThemeToken](../../../feathers/theme/struct.ThemeToken.html "struct bevy::feathers::theme::ThemeToken")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [ThemeToken](../../../feathers/theme/struct.ThemeToken.html "struct bevy::feathers::theme::ThemeToken")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#125)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ThemedText](../../../feathers/theme/struct.ThemedText.html "struct bevy::feathers::theme::ThemedText")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#125)

#### type [This](#associatedtype.This)<'from\_arg> = [ThemedText](../../../feathers/theme/struct.ThemedText.html "struct bevy::feathers::theme::ThemedText")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#298)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ThreadedAnimationGraph](../../../prelude/struct.ThreadedAnimationGraph.html "struct bevy::prelude::ThreadedAnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#298)

#### type [This](#associatedtype.This)<'from\_arg> = [ThreadedAnimationGraph](../../../prelude/struct.ThreadedAnimationGraph.html "struct bevy::prelude::ThreadedAnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#288)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ThreadedAnimationGraphs](../../../prelude/struct.ThreadedAnimationGraphs.html "struct bevy::prelude::ThreadedAnimationGraphs")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#288)

#### type [This](#associatedtype.This)<'from\_arg> = [ThreadedAnimationGraphs](../../../prelude/struct.ThreadedAnimationGraphs.html "struct bevy::prelude::ThreadedAnimationGraphs")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#15)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Tick](../../../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#15)

#### type [This](#associatedtype.This)<'from\_arg> = [Tick](../../../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#94)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TileData](../../../sprite_render/struct.TileData.html "struct bevy::sprite_render::TileData")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#94)

#### type [This](#associatedtype.This)<'from\_arg> = [TileData](../../../sprite_render/struct.TileData.html "struct bevy::sprite_render::TileData")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tile_orientation.rs.html#37)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TileOrientation](../../../sprite_render/enum.TileOrientation.html "enum bevy::sprite_render::TileOrientation")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tile_orientation.rs.html#37)

#### type [This](#associatedtype.This)<'from\_arg> = [TileOrientation](../../../sprite_render/enum.TileOrientation.html "enum bevy::sprite_render::TileOrientation")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#52)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TilemapChunk](../../../sprite_render/struct.TilemapChunk.html "struct bevy::sprite_render::TilemapChunk")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#52)

#### type [This](#associatedtype.This)<'from\_arg> = [TilemapChunk](../../../sprite_render/struct.TilemapChunk.html "struct bevy::sprite_render::TilemapChunk")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#46)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TilemapChunkMeshCache](../../../sprite_render/struct.TilemapChunkMeshCache.html "struct bevy::sprite_render::TilemapChunkMeshCache")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#46)

#### type [This](#associatedtype.This)<'from\_arg> = [TilemapChunkMeshCache](../../../sprite_render/struct.TilemapChunkMeshCache.html "struct bevy::sprite_render::TilemapChunkMeshCache")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#130)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TilemapChunkTileData](../../../sprite_render/struct.TilemapChunkTileData.html "struct bevy::sprite_render::TilemapChunkTileData")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#130)

#### type [This](#associatedtype.This)<'from\_arg> = [TilemapChunkTileData](../../../sprite_render/struct.TilemapChunkTileData.html "struct bevy::sprite_render::TilemapChunkTileData")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#31)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Timer](../../../prelude/struct.Timer.html "struct bevy::prelude::Timer")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#31)

#### type [This](#associatedtype.This)<'from\_arg> = [Timer](../../../prelude/struct.Timer.html "struct bevy::prelude::Timer")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#492)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TimerMode](../../../prelude/enum.TimerMode.html "enum bevy::prelude::TimerMode")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#492)

#### type [This](#associatedtype.This)<'from\_arg> = [TimerMode](../../../prelude/enum.TimerMode.html "enum bevy::prelude::TimerMode")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#209)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ToggleChecked](../../../ui_widgets/struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#209)

#### type [This](#associatedtype.This)<'from\_arg> = [ToggleChecked](../../../ui_widgets/struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Tonemapping](../../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

#### type [This](#associatedtype.This)<'from\_arg> = [Tonemapping](../../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1124)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Torus](../../../prelude/struct.Torus.html "struct bevy::prelude::Torus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1124)

#### type [This](#associatedtype.This)<'from\_arg> = [Torus](../../../prelude/struct.Torus.html "struct bevy::prelude::Torus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/torus.rs.html#8)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TorusMeshBuilder](../../../mesh/struct.TorusMeshBuilder.html "struct bevy::mesh::TorusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/torus.rs.html#8)

#### type [This](#associatedtype.This)<'from\_arg> = [TorusMeshBuilder](../../../mesh/struct.TorusMeshBuilder.html "struct bevy::mesh::TorusMeshBuilder")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#45)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TouchInput](../../../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#45)

#### type [This](#associatedtype.This)<'from\_arg> = [TouchInput](../../../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#123)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TouchPhase](../../../input/touch/enum.TouchPhase.html "enum bevy::input::touch::TouchPhase")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#123)

#### type [This](#associatedtype.This)<'from\_arg> = [TouchPhase](../../../input/touch/enum.TouchPhase.html "enum bevy::input::touch::TouchPhase")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#61)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TrackClick](../../../ui_widgets/enum.TrackClick.html "enum bevy::ui_widgets::TrackClick")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#61)

#### type [This](#associatedtype.This)<'from\_arg> = [TrackClick](../../../ui_widgets/enum.TrackClick.html "enum bevy::ui_widgets::TrackClick")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Transform](../../../prelude/struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### type [This](#associatedtype.This)<'from\_arg> = [Transform](../../../prelude/struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#123)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TransformGizmoAxis](../../../prelude/enum.TransformGizmoAxis.html "enum bevy::prelude::TransformGizmoAxis")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#123)

#### type [This](#associatedtype.This)<'from\_arg> = [TransformGizmoAxis](../../../prelude/enum.TransformGizmoAxis.html "enum bevy::prelude::TransformGizmoAxis")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#95)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TransformGizmoCamera](../../../prelude/struct.TransformGizmoCamera.html "struct bevy::prelude::TransformGizmoCamera")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#95)

#### type [This](#associatedtype.This)<'from\_arg> = [TransformGizmoCamera](../../../prelude/struct.TransformGizmoCamera.html "struct bevy::prelude::TransformGizmoCamera")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#85)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TransformGizmoFocus](../../../prelude/struct.TransformGizmoFocus.html "struct bevy::prelude::TransformGizmoFocus")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#85)

#### type [This](#associatedtype.This)<'from\_arg> = [TransformGizmoFocus](../../../prelude/struct.TransformGizmoFocus.html "struct bevy::prelude::TransformGizmoFocus")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#101)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TransformGizmoMode](../../../prelude/enum.TransformGizmoMode.html "enum bevy::prelude::TransformGizmoMode")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#101)

#### type [This](#associatedtype.This)<'from\_arg> = [TransformGizmoMode](../../../prelude/enum.TransformGizmoMode.html "enum bevy::prelude::TransformGizmoMode")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#136)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TransformGizmoSettings](../../../prelude/struct.TransformGizmoSettings.html "struct bevy::prelude::TransformGizmoSettings")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#136)

#### type [This](#associatedtype.This)<'from\_arg> = [TransformGizmoSettings](../../../prelude/struct.TransformGizmoSettings.html "struct bevy::prelude::TransformGizmoSettings")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#113)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TransformGizmoSpace](../../../prelude/enum.TransformGizmoSpace.html "enum bevy::prelude::TransformGizmoSpace")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#113)

#### type [This](#associatedtype.This)<'from\_arg> = [TransformGizmoSpace](../../../prelude/enum.TransformGizmoSpace.html "enum bevy::prelude::TransformGizmoSpace")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#179)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TransformGizmoState](../../../prelude/struct.TransformGizmoState.html "struct bevy::prelude::TransformGizmoState")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#179)

#### type [This](#associatedtype.This)<'from\_arg> = [TransformGizmoState](../../../prelude/struct.TransformGizmoState.html "struct bevy::prelude::TransformGizmoState")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#666)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TransformTreeChanged](../../../prelude/struct.TransformTreeChanged.html "struct bevy::prelude::TransformTreeChanged")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#666)

#### type [This](#associatedtype.This)<'from\_arg> = [TransformTreeChanged](../../../prelude/struct.TransformTreeChanged.html "struct bevy::prelude::TransformTreeChanged")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#274)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TransmittedShadowReceiver](../../../light/struct.TransmittedShadowReceiver.html "struct bevy::light::TransmittedShadowReceiver")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#274)

#### type [This](#associatedtype.This)<'from\_arg> = [TransmittedShadowReceiver](../../../light/struct.TransmittedShadowReceiver.html "struct bevy::light::TransmittedShadowReceiver")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1627)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Triangle2d](../../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1627)

#### type [This](#associatedtype.This)<'from\_arg> = [Triangle2d](../../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#964)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Triangle2dMeshBuilder](../../../mesh/struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#964)

#### type [This](#associatedtype.This)<'from\_arg> = [Triangle2dMeshBuilder](../../../mesh/struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1236)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Triangle3d](../../../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1236)

#### type [This](#associatedtype.This)<'from\_arg> = [Triangle3d](../../../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#7)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Triangle3dMeshBuilder](../../../mesh/struct.Triangle3dMeshBuilder.html "struct bevy::mesh::Triangle3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [Triangle3dMeshBuilder](../../../mesh/struct.Triangle3dMeshBuilder.html "struct bevy::mesh::Triangle3dMeshBuilder")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#166-173)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [U8Vec2](../../../math/struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#166-173)

#### type [This](#associatedtype.This)<'from\_arg> = [U8Vec2](../../../math/struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#174-182)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [U8Vec3](../../../math/struct.U8Vec3.html "struct bevy::math::U8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#174-182)

#### type [This](#associatedtype.This)<'from\_arg> = [U8Vec3](../../../math/struct.U8Vec3.html "struct bevy::math::U8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#183-192)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [U8Vec4](../../../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#183-192)

#### type [This](#associatedtype.This)<'from\_arg> = [U8Vec4](../../../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#194-201)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [U16Vec2](../../../math/struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#194-201)

#### type [This](#associatedtype.This)<'from\_arg> = [U16Vec2](../../../math/struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#202-210)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [U16Vec3](../../../math/struct.U16Vec3.html "struct bevy::math::U16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#202-210)

#### type [This](#associatedtype.This)<'from\_arg> = [U16Vec3](../../../math/struct.U16Vec3.html "struct bevy::math::U16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#211-220)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [U16Vec4](../../../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#211-220)

#### type [This](#associatedtype.This)<'from\_arg> = [U16Vec4](../../../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#222-229)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [U64Vec2](../../../math/struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#222-229)

#### type [This](#associatedtype.This)<'from\_arg> = [U64Vec2](../../../math/struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#230-238)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [U64Vec3](../../../math/struct.U64Vec3.html "struct bevy::math::U64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#230-238)

#### type [This](#associatedtype.This)<'from\_arg> = [U64Vec3](../../../math/struct.U64Vec3.html "struct bevy::math::U64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#239-248)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [U64Vec4](../../../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#239-248)

#### type [This](#associatedtype.This)<'from\_arg> = [U64Vec4](../../../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/urect.rs.html#21)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [URect](../../../prelude/struct.URect.html "struct bevy::prelude::URect")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/urect.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [URect](../../../prelude/struct.URect.html "struct bevy::prelude::URect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UVec2](../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

#### type [This](#associatedtype.This)<'from\_arg> = [UVec2](../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#146-154)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UVec3](../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#146-154)

#### type [This](#associatedtype.This)<'from\_arg> = [UVec3](../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UVec4](../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### type [This](#associatedtype.This)<'from\_arg> = [UVec4](../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#159)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiAntiAlias](../../../prelude/enum.UiAntiAlias.html "enum bevy::prelude::UiAntiAlias")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#159)

#### type [This](#associatedtype.This)<'from\_arg> = [UiAntiAlias](../../../prelude/enum.UiAntiAlias.html "enum bevy::prelude::UiAntiAlias")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#39)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiDebugOptions](../../../prelude/struct.UiDebugOptions.html "struct bevy::prelude::UiDebugOptions")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#39)

#### type [This](#associatedtype.This)<'from\_arg> = [UiDebugOptions](../../../prelude/struct.UiDebugOptions.html "struct bevy::prelude::UiDebugOptions")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#199)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiGlobalTransform](../../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#199)

#### type [This](#associatedtype.This)<'from\_arg> = [UiGlobalTransform](../../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#40)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiPickingCamera](../../../prelude/struct.UiPickingCamera.html "struct bevy::prelude::UiPickingCamera")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#40)

#### type [This](#associatedtype.This)<'from\_arg> = [UiPickingCamera](../../../prelude/struct.UiPickingCamera.html "struct bevy::prelude::UiPickingCamera")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#45)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiPickingSettings](../../../prelude/struct.UiPickingSettings.html "struct bevy::prelude::UiPickingSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#45)

#### type [This](#associatedtype.This)<'from\_arg> = [UiPickingSettings](../../../prelude/struct.UiPickingSettings.html "struct bevy::prelude::UiPickingSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#993)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiPosition](../../../prelude/struct.UiPosition.html "struct bevy::prelude::UiPosition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#993)

#### type [This](#associatedtype.This)<'from\_arg> = [UiPosition](../../../prelude/struct.UiPosition.html "struct bevy::prelude::UiPosition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiRect](../../../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

#### type [This](#associatedtype.This)<'from\_arg> = [UiRect](../../../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#124)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiScale](../../../prelude/struct.UiScale.html "struct bevy::prelude::UiScale")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#124)

#### type [This](#associatedtype.This)<'from\_arg> = [UiScale](../../../prelude/struct.UiScale.html "struct bevy::prelude::UiScale")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#25)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiStack](../../../ui/struct.UiStack.html "struct bevy::ui::UiStack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#25)

#### type [This](#associatedtype.This)<'from\_arg> = [UiStack](../../../ui/struct.UiStack.html "struct bevy::ui::UiStack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2936)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiTargetCamera](../../../prelude/struct.UiTargetCamera.html "struct bevy::prelude::UiTargetCamera")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2936)

#### type [This](#associatedtype.This)<'from\_arg> = [UiTargetCamera](../../../prelude/struct.UiTargetCamera.html "struct bevy::prelude::UiTargetCamera")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#59)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiTheme](../../../feathers/theme/struct.UiTheme.html "struct bevy::feathers::theme::UiTheme")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#59)

#### type [This](#associatedtype.This)<'from\_arg> = [UiTheme](../../../feathers/theme/struct.UiTheme.html "struct bevy::feathers::theme::UiTheme")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#122)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UiTransform](../../../prelude/struct.UiTransform.html "struct bevy::prelude::UiTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#122)

#### type [This](#associatedtype.This)<'from\_arg> = [UiTransform](../../../prelude/struct.UiTransform.html "struct bevy::prelude::UiTransform")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1154)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Underline](../../../prelude/struct.Underline.html "struct bevy::prelude::Underline")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1154)

#### type [This](#associatedtype.This)<'from\_arg> = [Underline](../../../prelude/struct.Underline.html "struct bevy::prelude::Underline")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1159)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UnderlineColor](../../../prelude/struct.UnderlineColor.html "struct bevy::prelude::UnderlineColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1159)

#### type [This](#associatedtype.This)<'from\_arg> = [UnderlineColor](../../../prelude/struct.UnderlineColor.html "struct bevy::prelude::UnderlineColor")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#167)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UntypedAssetId](../../../asset/enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#167)

#### type [This](#associatedtype.This)<'from\_arg> = [UntypedAssetId](../../../asset/enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#474)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UntypedHandle](../../../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#474)

#### type [This](#associatedtype.This)<'from\_arg> = [UntypedHandle](../../../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#170)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UpdateNumberInput](../../../feathers/controls/struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#170)

#### type [This](#associatedtype.This)<'from\_arg> = [UpdateNumberInput](../../../feathers/controls/struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/uuid.rs.html#4-13)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Uuid](../../../asset/uuid/struct.Uuid.html "struct bevy::asset::uuid::Uuid")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/uuid.rs.html#4-13)

#### type [This](#associatedtype.This)<'from\_arg> = [Uuid](../../../asset/uuid/struct.Uuid.html "struct bevy::asset::uuid::Uuid")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#2531)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UvChannel](../../../mesh/enum.UvChannel.html "enum bevy::mesh::UvChannel")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#2531)

#### type [This](#associatedtype.This)<'from\_arg> = [UvChannel](../../../mesh/enum.UvChannel.html "enum bevy::mesh::UvChannel")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Val](../../../prelude/enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

#### type [This](#associatedtype.This)<'from\_arg> = [Val](../../../prelude/enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#15)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Val2](../../../prelude/struct.Val2.html "struct bevy::prelude::Val2")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#15)

#### type [This](#associatedtype.This)<'from\_arg> = [Val2](../../../prelude/struct.Val2.html "struct bevy::prelude::Val2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

#### type [This](#associatedtype.This)<'from\_arg> = [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Vec3](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

#### type [This](#associatedtype.This)<'from\_arg> = [Vec3](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Vec4](../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### type [This](#associatedtype.This)<'from\_arg> = [Vec4](../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#267-275)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Vec3A](../../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#267-275)

#### type [This](#associatedtype.This)<'from\_arg> = [Vec3A](../../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#72)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VideoMode](../../../window/struct.VideoMode.html "struct bevy::window::VideoMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#72)

#### type [This](#associatedtype.This)<'from\_arg> = [VideoMode](../../../window/struct.VideoMode.html "struct bevy::window::VideoMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1175)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VideoModeSelection](../../../prelude/enum.VideoModeSelection.html "enum bevy::prelude::VideoModeSelection")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1175)

#### type [This](#associatedtype.This)<'from\_arg> = [VideoModeSelection](../../../prelude/enum.VideoModeSelection.html "enum bevy::prelude::VideoModeSelection")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/view_frustum.rs.html#18)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ViewFrustum](../../../prelude/struct.ViewFrustum.html "struct bevy::prelude::ViewFrustum")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/view_frustum.rs.html#18)

#### type [This](#associatedtype.This)<'from\_arg> = [ViewFrustum](../../../prelude/struct.ViewFrustum.html "struct bevy::prelude::ViewFrustum")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#224)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ViewVisibility](../../../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#224)

#### type [This](#associatedtype.This)<'from\_arg> = [ViewVisibility](../../../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#60)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Viewport](../../../camera/struct.Viewport.html "struct bevy::camera::Viewport")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#60)

#### type [This](#associatedtype.This)<'from\_arg> = [Viewport](../../../camera/struct.Viewport.html "struct bevy::camera::Viewport")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/viewport.rs.html#36)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ViewportNode](../../../prelude/struct.ViewportNode.html "struct bevy::prelude::ViewportNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/viewport.rs.html#36)

#### type [This](#associatedtype.This)<'from\_arg> = [ViewportNode](../../../prelude/struct.ViewportNode.html "struct bevy::prelude::ViewportNode")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#28)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Vignette](../../../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#28)

#### type [This](#associatedtype.This)<'from\_arg> = [Vignette](../../../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#74)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Virtual](../../../prelude/struct.Virtual.html "struct bevy::prelude::Virtual")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#74)

#### type [This](#associatedtype.This)<'from\_arg> = [Virtual](../../../prelude/struct.Virtual.html "struct bevy::prelude::Virtual")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#80)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Visibility](../../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#80)

#### type [This](#associatedtype.This)<'from\_arg> = [Visibility](../../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#208)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VisibilityClass](../../../camera/visibility/struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#208)

#### type [This](#associatedtype.This)<'from\_arg> = [VisibilityClass](../../../camera/visibility/struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/range.rs.html#78)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VisibilityRange](../../../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/range.rs.html#78)

#### type [This](#associatedtype.This)<'from\_arg> = [VisibilityRange](../../../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#342)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VisibleEntities](../../../camera/visibility/struct.VisibleEntities.html "struct bevy::camera::visibility::VisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#342)

#### type [This](#associatedtype.This)<'from\_arg> = [VisibleEntities](../../../camera/visibility/struct.VisibleEntities.html "struct bevy::camera::visibility::VisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#408)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VisibleMeshEntities](../../../camera/visibility/struct.VisibleMeshEntities.html "struct bevy::camera::visibility::VisibleMeshEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#408)

#### type [This](#associatedtype.This)<'from\_arg> = [VisibleMeshEntities](../../../camera/visibility/struct.VisibleMeshEntities.html "struct bevy::camera::visibility::VisibleMeshEntities")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1435)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VisualBox](../../../prelude/enum.VisualBox.html "enum bevy::prelude::VisualBox")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1435)

#### type [This](#associatedtype.This)<'from\_arg> = [VisualBox](../../../prelude/enum.VisualBox.html "enum bevy::prelude::VisualBox")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#34)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Volume](../../../audio/enum.Volume.html "enum bevy::audio::Volume")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#34)

#### type [This](#associatedtype.This)<'from\_arg> = [Volume](../../../audio/enum.Volume.html "enum bevy::audio::Volume")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#23)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VolumetricFog](../../../light/struct.VolumetricFog.html "struct bevy::light::VolumetricFog")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#23)

#### type [This](#associatedtype.This)<'from\_arg> = [VolumetricFog](../../../light/struct.VolumetricFog.html "struct bevy::light::VolumetricFog")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#14)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VolumetricLight](../../../light/struct.VolumetricLight.html "struct bevy::light::VolumetricLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#14)

#### type [This](#associatedtype.This)<'from\_arg> = [VolumetricLight](../../../light/struct.VolumetricLight.html "struct bevy::light::VolumetricLight")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#64)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WeightsCurveSample](../../../prelude/struct.WeightsCurveSample.html "struct bevy::prelude::WeightsCurveSample")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#64)

#### type [This](#associatedtype.This)<'from\_arg> = [WeightsCurveSample](../../../prelude/struct.WeightsCurveSample.html "struct bevy::prelude::WeightsCurveSample")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#155)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Window](../../../prelude/struct.Window.html "struct bevy::prelude::Window")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#155)

#### type [This](#associatedtype.This)<'from\_arg> = [Window](../../../prelude/struct.Window.html "struct bevy::prelude::Window")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#357)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowBackendScaleFactorChanged](../../../window/struct.WindowBackendScaleFactorChanged.html "struct bevy::window::WindowBackendScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#357)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowBackendScaleFactorChanged](../../../window/struct.WindowBackendScaleFactorChanged.html "struct bevy::window::WindowBackendScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#95)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowCloseRequested](../../../window/struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#95)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowCloseRequested](../../../window/struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#113)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowClosed](../../../window/struct.WindowClosed.html "struct bevy::window::WindowClosed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#113)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowClosed](../../../window/struct.WindowClosed.html "struct bevy::window::WindowClosed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#134)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowClosing](../../../window/struct.WindowClosing.html "struct bevy::window::WindowClosing")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#134)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowClosing](../../../window/struct.WindowClosing.html "struct bevy::window::WindowClosing")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#69)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowCreated](../../../window/struct.WindowCreated.html "struct bevy::window::WindowCreated")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#69)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowCreated](../../../window/struct.WindowCreated.html "struct bevy::window::WindowCreated")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#154)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowDestroyed](../../../window/struct.WindowDestroyed.html "struct bevy::window::WindowDestroyed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#154)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowDestroyed](../../../window/struct.WindowDestroyed.html "struct bevy::window::WindowDestroyed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#496)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowEvent](../../../window/enum.WindowEvent.html "enum bevy::window::WindowEvent")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#496)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowEvent](../../../window/enum.WindowEvent.html "enum bevy::window::WindowEvent")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#292)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowFocused](../../../window/struct.WindowFocused.html "struct bevy::window::WindowFocused")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#292)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowFocused](../../../window/struct.WindowFocused.html "struct bevy::window::WindowFocused")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1382)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowLevel](../../../window/enum.WindowLevel.html "enum bevy::window::WindowLevel")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1382)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowLevel](../../../window/enum.WindowLevel.html "enum bevy::window::WindowLevel")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1334)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowMode](../../../window/enum.WindowMode.html "enum bevy::window::WindowMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1334)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowMode](../../../window/enum.WindowMode.html "enum bevy::window::WindowMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#412)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowMoved](../../../prelude/struct.WindowMoved.html "struct bevy::prelude::WindowMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#412)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowMoved](../../../prelude/struct.WindowMoved.html "struct bevy::prelude::WindowMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#319)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowOccluded](../../../window/struct.WindowOccluded.html "struct bevy::window::WindowOccluded")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#319)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowOccluded](../../../window/struct.WindowOccluded.html "struct bevy::window::WindowOccluded")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#796)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowPosition](../../../prelude/enum.WindowPosition.html "enum bevy::prelude::WindowPosition")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#796)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowPosition](../../../prelude/enum.WindowPosition.html "enum bevy::prelude::WindowPosition")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#64)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowRef](../../../window/enum.WindowRef.html "enum bevy::window::WindowRef")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#64)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowRef](../../../window/enum.WindowRef.html "enum bevy::window::WindowRef")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#675)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowResizeConstraints](../../../prelude/struct.WindowResizeConstraints.html "struct bevy::prelude::WindowResizeConstraints")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#675)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowResizeConstraints](../../../prelude/struct.WindowResizeConstraints.html "struct bevy::prelude::WindowResizeConstraints")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#31)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowResized](../../../window/struct.WindowResized.html "struct bevy::window::WindowResized")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#31)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowResized](../../../window/struct.WindowResized.html "struct bevy::window::WindowResized")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#889)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowResolution](../../../window/struct.WindowResolution.html "struct bevy::window::WindowResolution")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#889)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowResolution](../../../window/struct.WindowResolution.html "struct bevy::window::WindowResolution")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#338)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowScaleFactorChanged](../../../window/struct.WindowScaleFactorChanged.html "struct bevy::window::WindowScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#338)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowScaleFactorChanged](../../../window/struct.WindowScaleFactorChanged.html "struct bevy::window::WindowScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1406)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowTheme](../../../window/enum.WindowTheme.html "enum bevy::window::WindowTheme")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1406)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowTheme](../../../window/enum.WindowTheme.html "enum bevy::window::WindowTheme")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#434)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WindowThemeChanged](../../../window/struct.WindowThemeChanged.html "struct bevy::window::WindowThemeChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#434)

#### type [This](#associatedtype.This)<'from\_arg> = [WindowThemeChanged](../../../window/struct.WindowThemeChanged.html "struct bevy::window::WindowThemeChanged")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#175)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WinitUserEvent](../../../winit/enum.WinitUserEvent.html "enum bevy::winit::WinitUserEvent")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#175)

#### type [This](#associatedtype.This)<'from\_arg> = [WinitUserEvent](../../../winit/enum.WinitUserEvent.html "enum bevy::winit::WinitUserEvent")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#199)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Wireframe](../../../pbr/wireframe/struct.Wireframe.html "struct bevy::pbr::wireframe::Wireframe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#199)

#### type [This](#associatedtype.This)<'from\_arg> = [Wireframe](../../../pbr/wireframe/struct.Wireframe.html "struct bevy::pbr::wireframe::Wireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#163)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Wireframe2d](../../../sprite_render/struct.Wireframe2d.html "struct bevy::sprite_render::Wireframe2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#163)

#### type [This](#associatedtype.This)<'from\_arg> = [Wireframe2d](../../../sprite_render/struct.Wireframe2d.html "struct bevy::sprite_render::Wireframe2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#403)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Wireframe2dColor](../../../sprite_render/struct.Wireframe2dColor.html "struct bevy::sprite_render::Wireframe2dColor")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#403)

#### type [This](#associatedtype.This)<'from\_arg> = [Wireframe2dColor](../../../sprite_render/struct.Wireframe2dColor.html "struct bevy::sprite_render::Wireframe2dColor")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#422)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Wireframe2dConfig](../../../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#422)

#### type [This](#associatedtype.This)<'from\_arg> = [Wireframe2dConfig](../../../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#434)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Wireframe2dMaterial](../../../sprite_render/struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#434)

#### type [This](#associatedtype.This)<'from\_arg> = [Wireframe2dMaterial](../../../sprite_render/struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#843)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WireframeColor](../../../pbr/wireframe/struct.WireframeColor.html "struct bevy::pbr::wireframe::WireframeColor")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#843)

#### type [This](#associatedtype.This)<'from\_arg> = [WireframeColor](../../../pbr/wireframe/struct.WireframeColor.html "struct bevy::pbr::wireframe::WireframeColor")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#883)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WireframeConfig](../../../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#883)

#### type [This](#associatedtype.This)<'from\_arg> = [WireframeConfig](../../../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#852)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WireframeLineWidth](../../../pbr/wireframe/struct.WireframeLineWidth.html "struct bevy::pbr::wireframe::WireframeLineWidth")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#852)

#### type [This](#associatedtype.This)<'from\_arg> = [WireframeLineWidth](../../../pbr/wireframe/struct.WireframeLineWidth.html "struct bevy::pbr::wireframe::WireframeLineWidth")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#910)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WireframeMaterial](../../../pbr/wireframe/struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#910)

#### type [This](#associatedtype.This)<'from\_arg> = [WireframeMaterial](../../../pbr/wireframe/struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#873)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WireframeTopology](../../../pbr/wireframe/enum.WireframeTopology.html "enum bevy::pbr::wireframe::WireframeTopology")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#873)

#### type [This](#associatedtype.This)<'from\_arg> = [WireframeTopology](../../../pbr/wireframe/enum.WireframeTopology.html "enum bevy::pbr::wireframe::WireframeTopology")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#18)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WorldAssetRoot](../../../prelude/struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#18)

#### type [This](#associatedtype.This)<'from\_arg> = [WorldAssetRoot](../../../prelude/struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#31)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WorldInstanceReady](../../../world_serialization/struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#31)

#### type [This](#associatedtype.This)<'from\_arg> = [WorldInstanceReady](../../../world_serialization/struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#17)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Xyza](../../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#17)

#### type [This](#associatedtype.This)<'from\_arg> = [Xyza](../../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2438)

### impl [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ZIndex](../../../prelude/struct.ZIndex.html "struct bevy::prelude::ZIndex")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2438)

#### type [This](#associatedtype.This)<'from\_arg> = [ZIndex](../../../prelude/struct.ZIndex.html "struct bevy::prelude::ZIndex")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/access.rs.html#16)

### impl<'a> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Access](../../enum.Access.html "enum bevy::reflect::Access")<'a>

where [Access](../../enum.Access.html "enum bevy::reflect::Access")<'a>: 'static,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/access.rs.html#16)

#### type [This](#associatedtype.This)<'from\_arg> = [Access](../../enum.Access.html "enum bevy::reflect::Access")<'a>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/path.rs.html#54)

### impl<'a> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AssetPath](../../../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>

where [AssetPath](../../../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>: 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/path.rs.html#54)

#### type [This](#associatedtype.This)<'from\_arg> = [AssetPath](../../../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#303)

### impl<A> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimatableCurveEvaluator](../../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>

where A: [Animatable](../../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AnimatableCurveEvaluator](../../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), BasicAnimationCurveEvaluator<A>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AnimatableProperty](../../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty")<Property = A>>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#303)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimatableCurveEvaluator](../../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/event.rs.html#49)

### impl<A> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AssetEvent](../../../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>

where A: [Asset](../../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AssetEvent](../../../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [AssetId](../../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/event.rs.html#49)

#### type [This](#associatedtype.This)<'from\_arg> = [AssetEvent](../../../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#21)

### impl<A> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AssetId](../../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>

where A: [Asset](../../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AssetId](../../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#21)

#### type [This](#associatedtype.This)<'from\_arg> = [AssetId](../../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#132)

### impl<A> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Handle](../../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Handle](../../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#132)

#### type [This](#associatedtype.This)<'from\_arg> = [Handle](../../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#142)

### impl<B, E> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ExtendedMaterial](../../../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>

where B: [Material](../../../prelude/trait.Material.html "trait bevy::prelude::Material") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, E: [MaterialExtension](../../../pbr/trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, [ExtendedMaterial](../../../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#142)

#### type [This](#associatedtype.This)<'from\_arg> = [ExtendedMaterial](../../../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#96)

### impl<C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Inherited](../../../app/struct.Inherited.html "struct bevy::app::Inherited")<C>

where C: [Component](../../../prelude/trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Inherited](../../../app/struct.Inherited.html "struct bevy::app::Inherited")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#96)

#### type [This](#associatedtype.This)<'from\_arg> = [Inherited](../../../app/struct.Inherited.html "struct bevy::app::Inherited")<C>

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#70)

### impl<C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Propagate](../../../app/struct.Propagate.html "struct bevy::app::Propagate")<C>

where C: [Component](../../../prelude/trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Propagate](../../../app/struct.Propagate.html "struct bevy::app::Propagate")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#70)

#### type [This](#associatedtype.This)<'from\_arg> = [Propagate](../../../app/struct.Propagate.html "struct bevy::app::Propagate")<C>

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#78)

### impl<C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PropagateOver](../../../app/struct.PropagateOver.html "struct bevy::app::PropagateOver")<C>

where [PropagateOver](../../../app/struct.PropagateOver.html "struct bevy::app::PropagateOver")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> C>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#78)

#### type [This](#associatedtype.This)<'from\_arg> = [PropagateOver](../../../app/struct.PropagateOver.html "struct bevy::app::PropagateOver")<C>

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#83)

### impl<C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PropagateStop](../../../app/struct.PropagateStop.html "struct bevy::app::PropagateStop")<C>

where [PropagateStop](../../../app/struct.PropagateStop.html "struct bevy::app::PropagateStop")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> C>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#83)

#### type [This](#associatedtype.This)<'from\_arg> = [PropagateStop](../../../app/struct.PropagateStop.html "struct bevy::app::PropagateStop")<C>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#152)

### impl<C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SampleDerivativeWrapper](../../../prelude/derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>

where [SampleDerivativeWrapper](../../../prelude/derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#152)

#### type [This](#associatedtype.This)<'from\_arg> = [SampleDerivativeWrapper](../../../prelude/derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#185)

### impl<C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SampleTwoDerivativesWrapper](../../../prelude/derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>

where [SampleTwoDerivativesWrapper](../../../prelude/derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#185)

#### type [This](#associatedtype.This)<'from\_arg> = [SampleTwoDerivativesWrapper](../../../prelude/derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#19)

### impl<C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WeightsCurve](../../../prelude/struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>

where [WeightsCurve](../../../prelude/struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#19)

#### type [This](#associatedtype.This)<'from\_arg> = [WeightsCurve](../../../prelude/struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#284)

### impl<Config, Clear> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GizmoBuffer](../../../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where [GizmoBuffer](../../../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), Config: [GizmoConfigGroup](../../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#284)

#### type [This](#associatedtype.This)<'from\_arg> = [GizmoBuffer](../../../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

### impl<E> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Pointer](../../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Pointer](../../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

#### type [This](#associatedtype.This)<'from\_arg> = [Pointer](../../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_map.rs.html#15-21)

### impl<K, V, S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for bevy::platform::collections::[HashMap](../../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<K, V, S>

where K: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_map.rs.html#15-21)

#### type [This](#associatedtype.This)<'from\_arg> = [HashMap](../../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<K, V, S>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#189)

### impl<M> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FocusedInput](../../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>

where M: [Message](../../../prelude/trait.Message.html "trait bevy::prelude::Message") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [FocusedInput](../../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#189)

#### type [This](#associatedtype.This)<'from\_arg> = [FocusedInput](../../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#172)

### impl<M> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MaterialNode](../../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

where M: [UiMaterial](../../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MaterialNode](../../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](../../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<M>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#172)

#### type [This](#associatedtype.This)<'from\_arg> = [MaterialNode](../../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#202)

### impl<M> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MeshMaterial2d](../../../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>

where M: [Material2d](../../../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MeshMaterial2d](../../../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](../../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<M>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#202)

#### type [This](#associatedtype.This)<'from\_arg> = [MeshMaterial2d](../../../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#39)

### impl<M> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MeshMaterial3d](../../../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>

where M: [Material](../../../prelude/trait.Material.html "trait bevy::prelude::Material") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MeshMaterial3d](../../../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](../../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<M>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#39)

#### type [This](#associatedtype.This)<'from\_arg> = [MeshMaterial3d](../../../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/mod.rs.html#117)

### impl<M> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MessageId](../../../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>

where M: [Message](../../../prelude/trait.Message.html "trait bevy::prelude::Message") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MessageId](../../../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/mod.rs.html#117)

#### type [This](#associatedtype.This)<'from\_arg> = [MessageId](../../../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/messages.rs.html#94)

### impl<M> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Messages](../../../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>

where M: [Message](../../../prelude/trait.Message.html "trait bevy::prelude::Message") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Messages](../../../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), MessageSequence<M>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/messages.rs.html#94)

#### type [This](#associatedtype.This)<'from\_arg> = [Messages](../../../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#286)

### impl<P, C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimatableCurve](../../../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>

where [AnimatableCurve](../../../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), P: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#286)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimatableCurve](../../../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#434)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubicBSpline](../../../prelude/struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicBSpline](../../../prelude/struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#434)

#### type [This](#associatedtype.This)<'from\_arg> = [CubicBSpline](../../../prelude/struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#54)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubicBezier](../../../prelude/struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicBezier](../../../prelude/struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[\[P; 4\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#54)

#### type [This](#associatedtype.This)<'from\_arg> = [CubicBezier](../../../prelude/struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#272)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubicCardinalSpline](../../../prelude/struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicCardinalSpline](../../../prelude/struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#272)

#### type [This](#associatedtype.This)<'from\_arg> = [CubicCardinalSpline](../../../prelude/struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1169)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubicCurve](../../../prelude/struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicCurve](../../../prelude/struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[CubicSegment](../../../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1169)

#### type [This](#associatedtype.This)<'from\_arg> = [CubicCurve](../../../prelude/struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#144)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubicHermite](../../../prelude/struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicHermite](../../../prelude/struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[(P, P)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#144)

#### type [This](#associatedtype.This)<'from\_arg> = [CubicHermite](../../../prelude/struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#611)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubicNurbs](../../../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicNurbs](../../../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#611)

#### type [This](#associatedtype.This)<'from\_arg> = [CubicNurbs](../../../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#946)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubicSegment](../../../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicSegment](../../../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [\[P; 4\]](https://doc.rust-lang.org/nightly/std/primitive.array.html): [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#946)

#### type [This](#associatedtype.This)<'from\_arg> = [CubicSegment](../../../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#837)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LinearSpline](../../../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [LinearSpline](../../../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#837)

#### type [This](#associatedtype.This)<'from\_arg> = [LinearSpline](../../../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1470)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RationalCurve](../../../prelude/struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RationalCurve](../../../prelude/struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[RationalSegment](../../../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1470)

#### type [This](#associatedtype.This)<'from\_arg> = [RationalCurve](../../../prelude/struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1328)

### impl<P> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RationalSegment](../../../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>

where P: [VectorSpace](../../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RationalSegment](../../../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [\[P; 4\]](https://doc.rust-lang.org/nightly/std/primitive.array.html): [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1328)

#### type [This](#associatedtype.This)<'from\_arg> = [RationalSegment](../../../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#482)

### impl<S, T, C, D> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ZipCurve](../../../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>

where [ZipCurve](../../../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), S: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#482)

#### type [This](#associatedtype.This)<'from\_arg> = [ZipCurve](../../../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#188)

### impl<S, T, C, F> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MapCurve](../../../prelude/struct.MapCurve.html "struct bevy::prelude::MapCurve")<S, T, C, F>

where [MapCurve](../../../prelude/struct.MapCurve.html "struct bevy::prelude::MapCurve")<S, T, C, F>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, S: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#188)

#### type [This](#associatedtype.This)<'from\_arg> = [MapCurve](../../../prelude/struct.MapCurve.html "struct bevy::prelude::MapCurve")<S, T, C, F>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#229)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DespawnOnEnter](../../../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DespawnOnEnter](../../../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#229)

#### type [This](#associatedtype.This)<'from\_arg> = [DespawnOnEnter](../../../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#148)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DespawnOnExit](../../../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DespawnOnExit](../../../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#148)

#### type [This](#associatedtype.This)<'from\_arg> = [DespawnOnExit](../../../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#66)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DespawnWhen](../../../prelude/struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DespawnWhen](../../../prelude/struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[StateTransitionEvent](../../../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#66)

#### type [This](#associatedtype.This)<'from\_arg> = [DespawnWhen](../../../prelude/struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#476)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DisableOnEnter](../../../prelude/struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DisableOnEnter](../../../prelude/struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#476)

#### type [This](#associatedtype.This)<'from\_arg> = [DisableOnEnter](../../../prelude/struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#395)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DisableOnExit](../../../prelude/struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DisableOnExit](../../../prelude/struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#395)

#### type [This](#associatedtype.This)<'from\_arg> = [DisableOnExit](../../../prelude/struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#313)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DisableWhen](../../../prelude/struct.DisableWhen.html "struct bevy::prelude::DisableWhen")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DisableWhen](../../../prelude/struct.DisableWhen.html "struct bevy::prelude::DisableWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[StateTransitionEvent](../../../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#313)

#### type [This](#associatedtype.This)<'from\_arg> = [DisableWhen](../../../prelude/struct.DisableWhen.html "struct bevy::prelude::DisableWhen")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#723)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EnableOnEnter](../../../prelude/struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [EnableOnEnter](../../../prelude/struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#723)

#### type [This](#associatedtype.This)<'from\_arg> = [EnableOnEnter](../../../prelude/struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#642)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EnableOnExit](../../../prelude/struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [EnableOnExit](../../../prelude/struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#642)

#### type [This](#associatedtype.This)<'from\_arg> = [EnableOnExit](../../../prelude/struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#560)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EnableWhen](../../../prelude/struct.EnableWhen.html "struct bevy::prelude::EnableWhen")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [EnableWhen](../../../prelude/struct.EnableWhen.html "struct bevy::prelude::EnableWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[StateTransitionEvent](../../../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#560)

#### type [This](#associatedtype.This)<'from\_arg> = [EnableWhen](../../../prelude/struct.EnableWhen.html "struct bevy::prelude::EnableWhen")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#178)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [NextState](../../../prelude/enum.NextState.html "enum bevy::prelude::NextState")<S>

where S: [FreelyMutableState](../../../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [NextState](../../../prelude/enum.NextState.html "enum bevy::prelude::NextState")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#178)

#### type [This](#associatedtype.This)<'from\_arg> = [NextState](../../../prelude/enum.NextState.html "enum bevy::prelude::NextState")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#131)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PreviousState](../../../prelude/struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [PreviousState](../../../prelude/struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#131)

#### type [This](#associatedtype.This)<'from\_arg> = [PreviousState](../../../prelude/struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#55)

### impl<S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [State](../../../prelude/struct.State.html "struct bevy::prelude::State")<S>

where S: [States](../../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [State](../../../prelude/struct.State.html "struct bevy::prelude::State")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#55)

#### type [This](#associatedtype.This)<'from\_arg> = [State](../../../prelude/struct.State.html "struct bevy::prelude::State")<S>

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#248)

### impl<Source> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AudioPlayer](../../../prelude/struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>

where [AudioPlayer](../../../prelude/struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), Source: [Asset](../../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [Decodable](../../../prelude/trait.Decodable.html "trait bevy::prelude::Decodable") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Handle](../../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<Source>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#248)

#### type [This](#associatedtype.This)<'from\_arg> = [AudioPlayer](../../../prelude/struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#524)

### impl<T, C, D> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ChainCurve](../../../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>

where [ChainCurve](../../../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#524)

#### type [This](#associatedtype.This)<'from\_arg> = [ChainCurve](../../../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#787)

### impl<T, C, D> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ContinuationCurve](../../../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>

where [ContinuationCurve](../../../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#787)

#### type [This](#associatedtype.This)<'from\_arg> = [ContinuationCurve](../../../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#417)

### impl<T, C, D> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CurveReparamCurve](../../../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>

where [CurveReparamCurve](../../../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#417)

#### type [This](#associatedtype.This)<'from\_arg> = [CurveReparamCurve](../../../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#285)

### impl<T, C, F> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ReparamCurve](../../../prelude/struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, C, F>

where [ReparamCurve](../../../prelude/struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, C, F>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#285)

#### type [This](#associatedtype.This)<'from\_arg> = [ReparamCurve](../../../prelude/struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, C, F>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#677)

### impl<T, C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ForeverCurve](../../../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>

where [ForeverCurve](../../../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#677)

#### type [This](#associatedtype.This)<'from\_arg> = [ForeverCurve](../../../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#451)

### impl<T, C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [GraphCurve](../../../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>

where [GraphCurve](../../../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#451)

#### type [This](#associatedtype.This)<'from\_arg> = [GraphCurve](../../../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#381)

### impl<T, C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [LinearReparamCurve](../../../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>

where [LinearReparamCurve](../../../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#381)

#### type [This](#associatedtype.This)<'from\_arg> = [LinearReparamCurve](../../../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#732)

### impl<T, C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [PingPongCurve](../../../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>

where [PingPongCurve](../../../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#732)

#### type [This](#associatedtype.This)<'from\_arg> = [PingPongCurve](../../../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#617)

### impl<T, C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [RepeatCurve](../../../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>

where [RepeatCurve](../../../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#617)

#### type [This](#associatedtype.This)<'from\_arg> = [RepeatCurve](../../../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#575)

### impl<T, C> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ReverseCurve](../../../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>

where [ReverseCurve](../../../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#575)

#### type [This](#associatedtype.This)<'from\_arg> = [ReverseCurve](../../../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#86)

### impl<T, F> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [FunctionCurve](../../../prelude/struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")<T, F>

where [FunctionCurve](../../../prelude/struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")<T, F>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#86)

#### type [This](#associatedtype.This)<'from\_arg> = [FunctionCurve](../../../prelude/struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")<T, F>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/hash.rs.html#7)

### impl<T, H> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Hashed](../../../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<T, H>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), H: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Hashed](../../../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<T, H>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/hash.rs.html#7)

#### type [This](#associatedtype.This)<'from\_arg> = [Hashed](../../../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<T, H>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#26)

### impl<T, I> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SampleCurve](../../../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>

where [SampleCurve](../../../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [EvenCore](../../../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#26)

#### type [This](#associatedtype.This)<'from\_arg> = [SampleCurve](../../../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#186)

### impl<T, I> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UnevenSampleCurve](../../../prelude/struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>

where [UnevenSampleCurve](../../../prelude/struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [UnevenCore](../../../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#186)

#### type [This](#associatedtype.This)<'from\_arg> = [UnevenSampleCurve](../../../prelude/struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#722)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [AnimatableKeyframeCurve](../../../prelude/struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>

where [AnimatableKeyframeCurve](../../../prelude/struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [UnevenCore](../../../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#722)

#### type [This](#associatedtype.This)<'from\_arg> = [AnimatableKeyframeCurve](../../../prelude/struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#3)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#3)

#### type [This](#associatedtype.This)<'from\_arg> = [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#301)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ArcMutexValue](../../../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>

where T: [Asset](../../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ArcMutexValue](../../../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#301)

#### type [This](#associatedtype.This)<'from\_arg> = [ArcMutexValue](../../../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/axis.rs.html#16)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Axis](../../../prelude/struct.Axis.html "struct bevy::prelude::Axis")<T>

where [Axis](../../../prelude/struct.Axis.html "struct bevy::prelude::Axis")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [HashMap](../../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<T, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/axis.rs.html#16)

#### type [This](#associatedtype.This)<'from\_arg> = [Axis](../../../prelude/struct.Axis.html "struct bevy::prelude::Axis")<T>

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/button_input.rs.html#124)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ButtonInput](../../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ButtonInput](../../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/button_input.rs.html#124)

#### type [This](#associatedtype.This)<'from\_arg> = [ButtonInput](../../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#467)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ChunkedUnevenCore](../../../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>

where [ChunkedUnevenCore](../../../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#467)

#### type [This](#associatedtype.This)<'from\_arg> = [ChunkedUnevenCore](../../../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_gradient.rs.html#11)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ColorCurve](../../../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>

where [ColorCurve](../../../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [EvenCore](../../../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_gradient.rs.html#11)

#### type [This](#associatedtype.This)<'from\_arg> = [ColorCurve](../../../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#46)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ConstantCurve](../../../prelude/struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>

where [ConstantCurve](../../../prelude/struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#46)

#### type [This](#associatedtype.This)<'from\_arg> = [ConstantCurve](../../../prelude/struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#50)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [CubicKeyframeCurve](../../../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<T>

where [CubicKeyframeCurve](../../../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](../../../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#50)

#### type [This](#associatedtype.This)<'from\_arg> = [CubicKeyframeCurve](../../../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#298)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EasingCurve](../../../prelude/struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>

where [EasingCurve](../../../prelude/struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#298)

#### type [This](#associatedtype.This)<'from\_arg> = [EasingCurve](../../../prelude/struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#122)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EvenCore](../../../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>

where [EvenCore](../../../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#122)

#### type [This](#associatedtype.This)<'from\_arg> = [EvenCore](../../../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#272)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [HandleTemplate](../../../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>

where T: [Asset](../../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [HandleTemplate](../../../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](../../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, [ArcMutexValue](../../../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#272)

#### type [This](#associatedtype.This)<'from\_arg> = [HandleTemplate](../../../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#45)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Interned](../../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<T>

where T: [Internable](../../../ecs/intern/trait.Internable.html "trait bevy::ecs::intern::Internable") + 'static + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [Interned](../../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [&'static T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#45)

#### type [This](#associatedtype.This)<'from\_arg> = [Interned](../../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#25)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [InterpolationDatum](../../../prelude/cores/enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")<T>

where [InterpolationDatum](../../../prelude/cores/enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#25)

#### type [This](#associatedtype.This)<'from\_arg> = [InterpolationDatum](../../../prelude/cores/enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/maybe_location.rs.html#20)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [MaybeLocation](../../../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")<T>

where [MaybeLocation](../../../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/maybe_location.rs.html#20)

#### type [This](#associatedtype.This)<'from\_arg> = [MaybeLocation](../../../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#139)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SampleAutoCurve](../../../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>

where [SampleAutoCurve](../../../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [EvenCore](../../../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#139)

#### type [This](#associatedtype.This)<'from\_arg> = [SampleAutoCurve](../../../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#12)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [SteppedKeyframeCurve](../../../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>

where [SteppedKeyframeCurve](../../../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [UnevenCore](../../../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#12)

#### type [This](#associatedtype.This)<'from\_arg> = [SteppedKeyframeCurve](../../../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Time](../../../prelude/struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](../../../prelude/struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### type [This](#associatedtype.This)<'from\_arg> = [Time](../../../prelude/struct.Time.html "struct bevy::prelude::Time")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#326)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UnevenCore](../../../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>

where [UnevenCore](../../../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#326)

#### type [This](#associatedtype.This)<'from\_arg> = [UnevenCore](../../../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#314)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UnevenSampleAutoCurve](../../../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>

where [UnevenSampleAutoCurve](../../../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [UnevenCore](../../../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#314)

#### type [This](#associatedtype.This)<'from\_arg> = [UnevenSampleAutoCurve](../../../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#88)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ValueChange](../../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>

where [ValueChange](../../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#88)

#### type [This](#associatedtype.This)<'from\_arg> = [ValueChange](../../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/vec.rs.html#20)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/vec.rs.html#20)

#### type [This](#associatedtype.This)<'from\_arg> = [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#93)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VirtualKeyPressed](../../../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>

where [VirtualKeyPressed](../../../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#93)

#### type [This](#associatedtype.This)<'from\_arg> = [VirtualKeyPressed](../../../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#22)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [VirtualKeyboard](../../../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [VirtualKeyboard](../../../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#22)

#### type [This](#associatedtype.This)<'from\_arg> = [VirtualKeyboard](../../../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#285)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WideCubicKeyframeCurve](../../../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>

where [WideCubicKeyframeCurve](../../../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](../../../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#285)

#### type [This](#associatedtype.This)<'from\_arg> = [WideCubicKeyframeCurve](../../../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#174)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WideLinearKeyframeCurve](../../../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>

where [WideLinearKeyframeCurve](../../../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](../../../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#174)

#### type [This](#associatedtype.This)<'from\_arg> = [WideLinearKeyframeCurve](../../../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#228)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WideSteppedKeyframeCurve](../../../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>

where [WideSteppedKeyframeCurve](../../../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](../../../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#228)

#### type [This](#associatedtype.This)<'from\_arg> = [WideSteppedKeyframeCurve](../../../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#602)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WithDerivative](../../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

where [WithDerivative](../../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [HasTangent](../../../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, <T as [HasTangent](../../../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../../../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"): [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#602)

#### type [This](#associatedtype.This)<'from\_arg> = [WithDerivative](../../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#617)

### impl<T> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [WithTwoDerivatives](../../../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>

where [WithTwoDerivatives](../../../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [HasTangent](../../../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, <T as [HasTangent](../../../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../../../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"): [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, <<T as [HasTangent](../../../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../../../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent") as [HasTangent](../../../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../../../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"): [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#617)

#### type [This](#associatedtype.This)<'from\_arg> = [WithTwoDerivatives](../../../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_set.rs.html#12-17)

### impl<V, S> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for bevy::platform::collections::[HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<V, S>

where V: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_set.rs.html#12-17)

#### type [This](#associatedtype.This)<'from\_arg> = [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<V, S>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#159)

### impl<V, W> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Sum](../../../math/struct.Sum.html "struct bevy::math::Sum")<V, W>

where [Sum](../../../math/struct.Sum.html "struct bevy::math::Sum")<V, W>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, W: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#159)

#### type [This](#associatedtype.This)<'from\_arg> = [Sum](../../../math/struct.Sum.html "struct bevy::math::Sum")<V, W>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_map.rs.html#19)

### impl<V> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityHashMap](../../../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<V>

where [EntityHashMap](../../../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [HashMap](../../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_map.rs.html#19)

#### type [This](#associatedtype.This)<'from\_arg> = [EntityHashMap](../../../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityIndexMap](../../../ecs/entity/struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../../../ecs/entity/struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### type [This](#associatedtype.This)<'from\_arg> = [EntityIndexMap](../../../ecs/entity/struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>