[bevy](../../index.html)::[ecs](../index.html)::[event](index.html)

# Trait SetEntityEventTarget 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/mod.rs.html#339)

```rust
pub trait SetEntityEventTarget: EntityEvent {
    // Required method
    fn set_event_target(&mut self, entity: Entity);
}
```

A trait which is used to set the target of an [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent").

By default, entity events are immutable; meaning their target does not change during the lifetime of the event. However, some events may require mutable access to provide features such as event propagation.

You should never need to implement this trait manually if you use `#[derive(EntityEvent)]`. It is automatically implemented for you if you use `#[entity_event(propagate)]`.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/mod.rs.html#344)

#### fn [set\_event\_target](#tymethod.set_event_target)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

Sets the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") “target” of this [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent"). When triggered, this will run observers that watch for this specific entity.

Note: In general, this should not be called from within an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer"), as this will not “retarget” the event in any of Bevy’s built-in [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") implementations.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#204)

### impl [SetEntityEventTarget](trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget") for [AcquireFocus](../../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")

where [AcquireFocus](../../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#29)

### impl [SetEntityEventTarget](trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget") for [FocusGained](../../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained")

where [FocusGained](../../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#46)

### impl [SetEntityEventTarget](trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget") for [FocusLost](../../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost")

where [FocusLost](../../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#77)

### impl [SetEntityEventTarget](trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget") for [MenuEvent](../../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent")

where [MenuEvent](../../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#39)

### impl [SetEntityEventTarget](trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget") for [ScrollIntoView](../../ui_widgets/struct.ScrollIntoView.html "struct bevy::ui_widgets::ScrollIntoView")

where [ScrollIntoView](../../ui_widgets/struct.ScrollIntoView.html "struct bevy::ui_widgets::ScrollIntoView"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

### impl<E> [SetEntityEventTarget](trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget") for [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"), [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#185)

### impl<M> [SetEntityEventTarget](trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget") for [FocusedInput](../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), [FocusedInput](../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,