[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait IntoObserverSystem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/observer_system.rs.html#32)

```rust
pub trait IntoObserverSystem<E, B, M, Out = ()>: Send + 'staticwhere
    E: Event,
    B: Bundle,{
    type System: ObserverSystem<E, B, Out>;

    // Required method
    fn into_system(this: Self) -> Self::System;
}
```

Implemented for systems that convert into [`ObserverSystem`](trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem").

## Usage notes

This trait should only be used as a bound for trait implementations or as an argument to a function. If an observer system needs to be returned from a function or stored somewhere, use [`ObserverSystem`](trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem") instead of this trait.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/observer_system.rs.html#34)

#### type [System](#associatedtype.System): [ObserverSystem](trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem")<E, B, Out>

The type of [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") that this instance converts into.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/observer_system.rs.html#37)

#### fn [into\_system](#tymethod.into_system)(this: Self) -> Self::[System](trait.IntoObserverSystem.html#associatedtype.System "type bevy::ecs::system::IntoObserverSystem::System")

Turns this value into its corresponding [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/observer_system.rs.html#40-45)

### impl<E, B, M, Out, S> [IntoObserverSystem](trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem")<E, B, M, Out> for S

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") + 'static, S: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<[On](../../prelude/struct.On.html "struct bevy::prelude::On")<'static, 'static, E, B>, Out, M> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, <S as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<[On](../../prelude/struct.On.html "struct bevy::prelude::On")<'static, 'static, E, B>, Out, M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"): [ObserverSystem](trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem")<E, B, Out>, B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/observer_system.rs.html#47)

#### type [System](#associatedtype.System) = <S as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<[On](../../prelude/struct.On.html "struct bevy::prelude::On")<'static, 'static, E, B>, Out, M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")