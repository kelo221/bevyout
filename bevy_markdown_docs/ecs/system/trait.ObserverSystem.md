[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait ObserverSystem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/observer_system.rs.html#10-11)

```rust
pub trait ObserverSystem<E, B, Out = ()>:
    System<In = On<'static, 'static, E, B>, Out = Out>
    + Send
    + 'staticwhere
    E: Event,
    B: Bundle,{ }
```

Implemented for [`System`](../../prelude/trait.System.html "trait bevy::prelude::System")s that have [`On`](../../prelude/struct.On.html "struct bevy::prelude::On") as the first argument.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/observer_system.rs.html#15-16)

### impl<E, B, Out, T> [ObserverSystem](trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem")<E, B, Out> for T

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), T: [System](../../prelude/trait.System.html "trait bevy::prelude::System")<In = [On](../../prelude/struct.On.html "struct bevy::prelude::On")<'static, 'static, E, B>, Out = Out> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,