[bevy](../index.html)::[prelude](index.html)

# Trait SystemInput 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#41)

```rust
pub trait SystemInput: Sized {
    type Param<'i>: SystemInput;
    type Inner<'i>;

    // Required method
    fn wrap(this: Self::Inner<'_>) -> Self::Param<'_>;
}
```

Trait for types that can be used as input to [`System`](trait.System.html "trait bevy::prelude::System")s.

Provided implementations are:

*   `()`: No input
*   [`In<T>`](struct.In.html "struct bevy::prelude::In"): For values
*   [`InRef<T>`](struct.InRef.html "struct bevy::prelude::InRef"): For read-only references to values
*   [`InMut<T>`](struct.InMut.html "struct bevy::prelude::InMut"): For mutable references to values
*   [`On<E, B>`](struct.On.html "struct bevy::prelude::On"): For [`ObserverSystem`](../ecs/system/trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem")s
*   [`StaticSystemInput<I>`](../ecs/system/struct.StaticSystemInput.html "struct bevy::ecs::system::StaticSystemInput"): For arbitrary [`SystemInput`](trait.SystemInput.html "trait bevy::prelude::SystemInput")s in generic contexts
*   `Option<I>`: For optional inputs of some [`SystemInput`](trait.SystemInput.html "trait bevy::prelude::SystemInput") `I`
*   Tuples of [`SystemInput`](trait.SystemInput.html "trait bevy::prelude::SystemInput")s up to 8 elements

For advanced usecases, you can implement this trait for your own types.

## Examples

### Tuples of [`SystemInput`](trait.SystemInput.html "trait bevy::prelude::SystemInput")s

```rust
use bevy_ecs::prelude::*;

fn add((InMut(a), In(b)): (InMut<usize>, In<usize>)) {
    *a += b;
}
```

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#45)

#### type [Param](#associatedtype.Param)<'i>: [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")

The wrapper input type that is defined as the first argument to [`FunctionSystem`](../ecs/system/struct.FunctionSystem.html "struct bevy::ecs::system::FunctionSystem")s.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#50)

#### type [Inner](#associatedtype.Inner)<'i>

The inner input type that is passed to functions that run systems, such as [`System::run`](trait.System.html#method.run "method bevy::prelude::System::run").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#53)

#### fn [wrap](#tymethod.wrap)(this: Self::[Inner](trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>) -> Self::[Param](trait.SystemInput.html#associatedtype.Param "type bevy::prelude::SystemInput::Param")<'\_>

Converts a [`SystemInput::Inner`](trait.SystemInput.html#associatedtype.Inner "associated type bevy::prelude::SystemInput::Inner") into a [`SystemInput::Param`](trait.SystemInput.html#associatedtype.Param "associated type bevy::prelude::SystemInput::Param").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#320-326)

### impl [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#320-326)

#### type [Param](#associatedtype.Param)<'i> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#320-326)

#### type [Inner](#associatedtype.Inner)<'i> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#320-326)

#### fn [wrap](#tymethod.wrap)(this: <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Param](trait.SystemInput.html#associatedtype.Param "type bevy::prelude::SystemInput::Param")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#320-326)

### impl<I> [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput") for [(I₁, I₂, …, Iₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where I: [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput"),

This trait is implemented for tuples up to 9 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#320-326)

#### type [Param](#associatedtype.Param)<'i> = (<I as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Param](trait.SystemInput.html#associatedtype.Param "type bevy::prelude::SystemInput::Param")<'i>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#320-326)

#### type [Inner](#associatedtype.Inner)<'i> = (<I as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'i>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#320-326)

#### fn [wrap](#tymethod.wrap)( this: <[(I,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, ) -> <[(I,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Param](trait.SystemInput.html#associatedtype.Param "type bevy::prelude::SystemInput::Param")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#284)

### impl<I> [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<I>

where I: [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#285)

#### type [Param](#associatedtype.Param)<'i> = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<I as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Param](trait.SystemInput.html#associatedtype.Param "type bevy::prelude::SystemInput::Param")<'i>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#286)

#### type [Inner](#associatedtype.Inner)<'i> = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<I as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'i>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#288)

#### fn [wrap](#tymethod.wrap)( this: <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<I> as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, ) -> <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<I> as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Param](trait.SystemInput.html#associatedtype.Param "type bevy::prelude::SystemInput::Param")<'\_>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#275)

### impl<'a, I> [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput") for [StaticSystemInput](../ecs/system/struct.StaticSystemInput.html "struct bevy::ecs::system::StaticSystemInput")<'a, I>

where I: [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#276)

#### type [Param](#associatedtype.Param)<'i> = [StaticSystemInput](../ecs/system/struct.StaticSystemInput.html "struct bevy::ecs::system::StaticSystemInput")<'i, I>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#277)

#### type [Inner](#associatedtype.Inner)<'i> = <I as [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'i>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#250)

### impl<E, B> [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput") for [On](struct.On.html "struct bevy::prelude::On")<'\_, '\_, E, B>

where E: [Event](trait.Event.html "trait bevy::prelude::Event"), B: [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"),

Used for [`ObserverSystem`](../ecs/system/trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem")s.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#255)

#### type [Param](#associatedtype.Param)<'i> = [On](struct.On.html "struct bevy::prelude::On")<'i, 'i, E, B>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#256)

#### type [Inner](#associatedtype.Inner)<'i> = [On](struct.On.html "struct bevy::prelude::On")<'i, 'i, E, B>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#116)

### impl<T> [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput") for [In](struct.In.html "struct bevy::prelude::In")<T>

where T: 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#117)

#### type [Param](#associatedtype.Param)<'i> = [In](struct.In.html "struct bevy::prelude::In")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#118)

#### type [Inner](#associatedtype.Inner)<'i> = T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#224)

### impl<T> [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput") for [InMut](struct.InMut.html "struct bevy::prelude::InMut")<'\_, T>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#225)

#### type [Param](#associatedtype.Param)<'i> = [InMut](struct.InMut.html "struct bevy::prelude::InMut")<'i, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#226)

#### type [Inner](#associatedtype.Inner)<'i> = [&'i mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#175)

### impl<T> [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput") for [InRef](struct.InRef.html "struct bevy::prelude::InRef")<'\_, T>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#176)

#### type [Param](#associatedtype.Param)<'i> = [InRef](struct.InRef.html "struct bevy::prelude::InRef")<'i, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#177)

#### type [Inner](#associatedtype.Inner)<'i> = [&'i T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)