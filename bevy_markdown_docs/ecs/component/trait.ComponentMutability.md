[bevy](../../index.html)::[ecs](../index.html)::[component](index.html)

# Trait ComponentMutability 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#688)

```rust
pub trait ComponentMutability: Seal + 'static {
    const MUTABLE: bool;
}
```

The mutability option for a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component"). This can either be:

*   [`Mutable`](struct.Mutable.html "struct bevy::ecs::component::Mutable")
*   [`Immutable`](struct.Immutable.html "struct bevy::ecs::component::Immutable")

This is controlled through either [`Component::Mutability`](../../prelude/trait.Component.html#associatedtype.Mutability "associated type bevy::prelude::Component::Mutability") or `#[component(immutable)]` when using the derive macro.

Immutable components are guaranteed to never have an exclusive reference, `&mut ...`, created while inserted onto an entity. In all other ways, they are identical to mutable components. This restriction allows hooks to observe all changes made to an immutable component, effectively turning the `Insert` and `Discard` hooks into a `OnMutate` hook. This is not practical for mutable components, as the runtime cost of invoking a hook for every exclusive reference created would be far too high.

## Examples

```rust
#[derive(Component)]
#[component(immutable)]
struct ImmutableFoo;
```

## Required Associated Constants

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#691)

#### const [MUTABLE](#associatedconstant.MUTABLE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Boolean to indicate if this mutability setting implies a mutable or immutable component.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#701)

### impl [ComponentMutability](trait.ComponentMutability.html "trait bevy::ecs::component::ComponentMutability") for [Immutable](struct.Immutable.html "struct bevy::ecs::component::Immutable")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#702)

#### const [MUTABLE](#associatedconstant.MUTABLE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#712)

### impl [ComponentMutability](trait.ComponentMutability.html "trait bevy::ecs::component::ComponentMutability") for [Mutable](struct.Mutable.html "struct bevy::ecs::component::Mutable")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#713)

#### const [MUTABLE](#associatedconstant.MUTABLE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true