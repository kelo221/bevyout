[bevy](../index.html)::[prelude](index.html)

# Trait EntityEvent 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/mod.rs.html#327)

```rust
pub trait EntityEvent: Event {
    // Required method
    fn event_target(&self) -> Entity;
}
```

An [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") is an [`Event`](trait.Event.html "trait bevy::prelude::Event") that is triggered for a specific [`EntityEvent::event_target`](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") entity:

```rust
#[derive(EntityEvent)]
struct Explode {
    entity: Entity,
}

world.add_observer(|event: On<Explode>, mut commands: Commands| {
    println!("Entity {} goes BOOM!", event.entity);
    commands.entity(event.entity).despawn();
});

world.trigger(Explode { entity });
```

[`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") will set [`EntityEvent::event_target`](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") automatically for named structs with an `entity` field name (as seen above). It also works for tuple structs whose only field is [`Entity`](struct.Entity.html "struct bevy::prelude::Entity"):

```rust
#[derive(EntityEvent)]
struct Explode(Entity);
```

The [`EntityEvent::event_target`](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") can also be manually set using the `#[event_target]` field attribute:

```rust
#[derive(EntityEvent)]
struct Explode {
    #[event_target]
    exploded_entity: Entity,
}
```

```rust
#[derive(EntityEvent)]
struct Explode(#[event_target] Entity);
```

You may also use any type which implements [`ContainsEntity`](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") as the event target:

```rust
struct Bomb(Entity);

impl ContainsEntity for Bomb {
    fn entity(&self) -> Entity {
        self.0
    }
}

#[derive(EntityEvent)]
struct Explode(Bomb);
```

By default, an [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") is immutable. This means the event data, including the target, does not change while the event is triggered. However, to support event propagation, your event must also implement the [`SetEntityEventTarget`](../ecs/event/trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget") trait.

This trait is automatically implemented for you if you enable event propagation:

```rust
#[derive(EntityEvent)]
#[entity_event(propagate)]
struct Explode(Entity);
```

### Trigger Behavior

When derived, [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") defaults to setting [`Event::Trigger`](trait.Event.html#associatedtype.Trigger "associated type bevy::prelude::Event::Trigger") to [`EntityTrigger`](../ecs/event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger"), which will run all normal “untargeted” observers added via [`World::add_observer`](struct.World.html#method.add_observer "method bevy::prelude::World::add_observer"), just like a default [`Event`](trait.Event.html "trait bevy::prelude::Event") would (see the example above).

However it will _also_ run all observers that watch _specific_ entities, which enables you to assign entity-specific logic:

```rust
world.entity_mut(e1).observe(|event: On<Explode>, mut commands: Commands| {
    println!("Boom!");
    commands.entity(event.entity).despawn();
});

world.entity_mut(e2).observe(|event: On<Explode>, mut commands: Commands| {
    println!("The explosion fizzles! This entity is immune!");
});
```

### [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") Propagation

When deriving [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent"), you can enable “event propagation” (also known as “event bubbling”) by specifying the `#[entity_event(propagate)]` attribute:

```rust
#[derive(EntityEvent)]
#[entity_event(propagate)]
struct Click {
    entity: Entity,
}
```

This will default to using the [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") component to propagate the [`Event`](trait.Event.html "trait bevy::prelude::Event") “up” the hierarchy (from child to parent).

You can also specify your own [`Traversal`](../ecs/traversal/trait.Traversal.html "trait bevy::ecs::traversal::Traversal") implementation. A common pattern is to use [`Relationship`](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship") components, which will follow the relationships to their root (just be sure to avoid cycles … these aren’t detected for performance reasons):

```rust
#[derive(Component)]
#[relationship(relationship_target = ClickableBy)]
struct Clickable(Entity);

#[derive(Component)]
#[relationship_target(relationship = Clickable)]
struct ClickableBy(Vec<Entity>);

#[derive(EntityEvent)]
#[entity_event(propagate = &'static Clickable)]
struct Click {
    entity: Entity,
}
```

By default, propagation requires observers to opt-in:

```rust
#[derive(EntityEvent)]
#[entity_event(propagate)]
struct Click {
    entity: Entity,
}

world.add_observer(|mut click: On<Click>| {
  // this will propagate the event up to the parent, using `ChildOf`
  click.propagate(true);
});
```

But you can enable auto propagation using the `#[entity_event(auto_propagate)]` attribute:

```rust
#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
struct Click {
    entity: Entity,
}
```

You can also _stop_ propagation like this:

```rust
world.add_observer(|mut click: On<Click>| {
  if is_finished_propagating() {
    click.propagate(false);
  }
});
```

### Best practices for event propagation

Propagation is useful for events that should be handled by multiple entities in a hierarchy, such as UI events. In these cases, it is common for the event to be triggered on a “leaf” entity, and then propagate up to “root” entities. In this pattern, it is generally recommended to trigger the event on the most specific entity possible (the leaf), and then use propagation to have it handled by more general entities (the roots).

Once an event is handled by a given entity, you should stop propagation. This ensures that only a single “behavior” resolves per event sent, avoiding unexpected behavior from entities higher up the hierarchy.

This advice has one notable wrinkle: if an entity is “disabled” (e.g. if a UI node is grayed out), the event should still be considered “handled” by that entity, even though the observer logic should not be run. This ensures consistent behavior regardless of the enabled/disabled state of entities.

### Naming and Usage Conventions

In most cases, it is recommended to use a named struct field for the “event target” entity, and to use a name that is descriptive as possible, as this makes events easier to understand and read.

For events with only one [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") field, `entity` is often a reasonable name. But if there are multiple [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") fields, it is often a good idea to use a more descriptive name.

It is also generally recommended to _consume_ “event target” entities directly via their named field, as this can make the context clearer, allows for more specific documentation hints in IDEs, and it generally reads better.

### Manually spawning [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") observers

The examples above that call [`EntityWorldMut::observe`](struct.EntityWorldMut.html#method.observe "method bevy::prelude::EntityWorldMut::observe") to add entity-specific observer logic are just shorthand for spawning an [`Observer`](struct.Observer.html "struct bevy::prelude::Observer") directly and manually watching the entity:

```rust
let mut observer = Observer::new(|event: On<Explode>| {});
observer.watch_entity(entity);
world.spawn(observer);
```

Note that the [`Observer`](struct.Observer.html "struct bevy::prelude::Observer") component is not added to the entity it is observing. Observers should always be their own entities, as there can be multiple observers of the same entity!

You can call [`Observer::watch_entity`](struct.Observer.html#method.watch_entity "method bevy::prelude::Observer::watch_entity") more than once or [`Observer::watch_entities`](struct.Observer.html#method.watch_entities "method bevy::prelude::Observer::watch_entities") to watch multiple entities with the same [`Observer`](struct.Observer.html "struct bevy::prelude::Observer").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/mod.rs.html#329)

#### fn [event\_target](#tymethod.event_target)(&self) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

The [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") “target” of this [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent"). When triggered, this will run observers that watch for this specific entity.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#204)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [AcquireFocus](../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")

where [AcquireFocus](../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#80)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [Activate](../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate")

where [Activate](../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#331)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [Add](struct.Add.html "struct bevy::prelude::Add")

where [Add](struct.Add.html "struct bevy::prelude::Add"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#386)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [Despawn](struct.Despawn.html "struct bevy::prelude::Despawn")

where [Despawn](struct.Despawn.html "struct bevy::prelude::Despawn"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#359)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [Discard](struct.Discard.html "struct bevy::prelude::Discard")

where [Discard](struct.Discard.html "struct bevy::prelude::Discard"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#29)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [FocusGained](../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained")

where [FocusGained](../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#46)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [FocusLost](../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost")

where [FocusLost](../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#344)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [Insert](struct.Insert.html "struct bevy::prelude::Insert")

where [Insert](struct.Insert.html "struct bevy::prelude::Insert"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#77)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [MenuEvent](../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent")

where [MenuEvent](../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#114)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [ReadbackComplete](../render/gpu_readback/struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete")

where [ReadbackComplete](../render/gpu_readback/struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#374)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [Remove](struct.Remove.html "struct bevy::prelude::Remove")

where [Remove](struct.Remove.html "struct bevy::prelude::Remove"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#47)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [ScreenshotCaptured](../render/view/window/screenshot/struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured")

where [ScreenshotCaptured](../render/view/window/screenshot/struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#39)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [ScrollIntoView](../ui_widgets/struct.ScrollIntoView.html "struct bevy::ui_widgets::ScrollIntoView")

where [ScrollIntoView](../ui_widgets/struct.ScrollIntoView.html "struct bevy::ui_widgets::ScrollIntoView"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#181)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [SetChecked](../ui_widgets/struct.SetChecked.html "struct bevy::ui_widgets::SetChecked")

where [SetChecked](../ui_widgets/struct.SetChecked.html "struct bevy::ui_widgets::SetChecked"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#673)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [SetSliderValue](../ui_widgets/struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue")

where [SetSliderValue](../ui_widgets/struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/editing.rs.html#330)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [TextEditChange](../text/struct.TextEditChange.html "struct bevy::text::TextEditChange")

where [TextEditChange](../text/struct.TextEditChange.html "struct bevy::text::TextEditChange"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#209)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [ToggleChecked](../ui_widgets/struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked")

where [ToggleChecked](../ui_widgets/struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#170)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [UpdateNumberInput](../feathers/controls/struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput")

where [UpdateNumberInput](../feathers/controls/struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#31)

### impl [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [WorldInstanceReady](../world_serialization/struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady")

where [WorldInstanceReady](../world_serialization/struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

### impl<E> [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [Pointer](struct.Pointer.html "struct bevy::prelude::Pointer")<E>

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"), [Pointer](struct.Pointer.html "struct bevy::prelude::Pointer")<E>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#185)

### impl<M> [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [FocusedInput](../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>

where M: [Message](trait.Message.html "trait bevy::prelude::Message") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), [FocusedInput](../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#88)

### impl<T> [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [ValueChange](../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>

where [ValueChange](../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#93)

### impl<T> [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") for [VirtualKeyPressed](../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>

where [VirtualKeyPressed](../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,