[bevy](../index.html)::[input\_focus](index.html)

# Trait IsFocused 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#395)

```rust
pub trait IsFocused {
    // Required methods
    fn is_focused(&self, entity: Entity) -> bool;
    fn is_focus_within(&self, entity: Entity) -> bool;
    fn is_focus_visible(&self, entity: Entity) -> bool;
    fn is_focus_within_visible(&self, entity: Entity) -> bool;
}
```

Trait which defines methods to check if an entity currently has focus.

This is implemented for [`World`](../prelude/struct.World.html "struct bevy::prelude::World") and [`IsFocusedHelper`](struct.IsFocusedHelper.html "struct bevy::input_focus::IsFocusedHelper"). [`DeferredWorld`](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld") indirectly implements it through [`Deref`](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref").

For use within systems, use [`IsFocusedHelper`](struct.IsFocusedHelper.html "struct bevy::input_focus::IsFocusedHelper").

Modify the [`InputFocus`](struct.InputFocus.html "struct bevy::input_focus::InputFocus") resource to change the focused entity.

## Required Methods

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#397)

#### fn [is\_focused](#tymethod.is_focused)(&self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the given entity has input focus.

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#402)

#### fn [is\_focus\_within](#tymethod.is_focus_within)(&self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the given entity or any of its descendants has input focus.

Note that for unusual layouts, the focus may not be within the entity’s visual bounds.

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#405)

#### fn [is\_focus\_visible](#tymethod.is_focus_visible)(&self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the given entity has input focus and the focus indicator should be visible.

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#409)

#### fn [is\_focus\_within\_visible](#tymethod.is_focus_within_visible)(&self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the given entity, or any descendant, has input focus and the focus indicator should be visible.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#422)

### impl [IsFocused](trait.IsFocused.html "trait bevy::input_focus::IsFocused") for [IsFocusedHelper](struct.IsFocusedHelper.html "struct bevy::input_focus::IsFocusedHelper")<'\_, '\_>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#449)

### impl [IsFocused](trait.IsFocused.html "trait bevy::input_focus::IsFocused") for [World](../prelude/struct.World.html "struct bevy::prelude::World")