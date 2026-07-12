[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Trait Event 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/mod.rs.html#88)

```rust
pub trait Event:
    Sized
    + Send
    + Sync
    + 'static {
    type Trigger<'a>: Trigger<Self>;
}
```

An [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") is something that “happens” at a given moment.

To make an [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") “happen”, you “trigger” it on a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") using [`World::trigger`](../../prelude/struct.World.html#method.trigger "method bevy::prelude::World::trigger") or via a [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") using [`Commands::trigger`](../../prelude/struct.Commands.html#method.trigger "method bevy::prelude::Commands::trigger"). This causes any [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") watching for that [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") to run _immediately_, as part of the [`World::trigger`](../../prelude/struct.World.html#method.trigger "method bevy::prelude::World::trigger") call.

First, we create an [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") type, typically by deriving the trait.

```rust
#[derive(Event)]
struct Speak {
    message: String,
}
```

Then, we add an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") to watch for this event type:

```rust
world.add_observer(|speak: On<Speak>| {
    println!("{}", speak.message);
});
```

Finally, we trigger the event by calling [`World::trigger`](../../prelude/struct.World.html#method.trigger "method bevy::prelude::World::trigger"):

```rust
world.trigger(Speak {
    message: "Hello!".to_string(),
});
```

## Triggers

Every [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") has an associated [`Trigger`](../event/trait.Trigger.html "trait bevy::ecs::event::Trigger") implementation (set via [`Event::Trigger`](../../prelude/trait.Event.html#associatedtype.Trigger "associated type bevy::prelude::Event::Trigger")), which defines which observers will run, what data will be passed to them, and the order they will be run in. Unless you are an internals developer or you have very specific needs, you don’t need to worry too much about [`Trigger`](../event/trait.Trigger.html "trait bevy::ecs::event::Trigger"). When you derive [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") (or a more specific event trait like [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent")), a [`Trigger`](../event/trait.Trigger.html "trait bevy::ecs::event::Trigger") will be provided for you.

The [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") derive defaults [`Event::Trigger`](../../prelude/trait.Event.html#associatedtype.Trigger "associated type bevy::prelude::Event::Trigger") to [`GlobalTrigger`](../event/struct.GlobalTrigger.html "struct bevy::ecs::event::GlobalTrigger"), which will run all observers that watch for the [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event").

## Entity Events

For events that “target” a specific [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), see [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent").

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/mod.rs.html#90)

#### type [Trigger](#associatedtype.Trigger)<'a>: [Trigger](../event/trait.Trigger.html "trait bevy::ecs::event::Trigger")<Self>

Defines which observers will run, what data will be passed to them, and the order they will be run in. See [`Trigger`](../event/trait.Trigger.html "trait bevy::ecs::event::Trigger") for more info.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#204)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [AcquireFocus](../../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")

where [AcquireFocus](../../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#204)

#### type [Trigger](#associatedtype.Trigger)<'a> = [PropagateEntityTrigger](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<true, [AcquireFocus](../../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus"), [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")\>

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#80)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [Activate](../../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate")

where [Activate](../../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#80)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#331)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [Add](../../prelude/struct.Add.html "struct bevy::prelude::Add")

where [Add](../../prelude/struct.Add.html "struct bevy::prelude::Add"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#331)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityComponentsTrigger](../event/struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#113)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [CheckChangeTicks](../change_detection/struct.CheckChangeTicks.html "struct bevy::ecs::change_detection::CheckChangeTicks")

where [CheckChangeTicks](../change_detection/struct.CheckChangeTicks.html "struct bevy::ecs::change_detection::CheckChangeTicks"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#113)

#### type [Trigger](#associatedtype.Trigger)<'a> = [GlobalTrigger](../event/struct.GlobalTrigger.html "struct bevy::ecs::event::GlobalTrigger")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#386)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [Despawn](../../prelude/struct.Despawn.html "struct bevy::prelude::Despawn")

where [Despawn](../../prelude/struct.Despawn.html "struct bevy::prelude::Despawn"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#386)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityComponentsTrigger](../event/struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#359)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [Discard](../../prelude/struct.Discard.html "struct bevy::prelude::Discard")

where [Discard](../../prelude/struct.Discard.html "struct bevy::prelude::Discard"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#359)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityComponentsTrigger](../event/struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger")<'a>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#29)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [FocusGained](../../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained")

where [FocusGained](../../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#29)

#### type [Trigger](#associatedtype.Trigger)<'a> = [PropagateEntityTrigger](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<true, [FocusGained](../../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained"), &'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#46)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [FocusLost](../../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost")

where [FocusLost](../../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#46)

#### type [Trigger](#associatedtype.Trigger)<'a> = [PropagateEntityTrigger](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<true, [FocusLost](../../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost"), &'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#344)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [Insert](../../prelude/struct.Insert.html "struct bevy::prelude::Insert")

where [Insert](../../prelude/struct.Insert.html "struct bevy::prelude::Insert"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#344)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityComponentsTrigger](../event/struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger")<'a>

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#77)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [MenuEvent](../../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent")

where [MenuEvent](../../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#77)

#### type [Trigger](#associatedtype.Trigger)<'a> = [PropagateEntityTrigger](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<true, [MenuEvent](../../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent"), &'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#114)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [ReadbackComplete](../../render/gpu_readback/struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete")

where [ReadbackComplete](../../render/gpu_readback/struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#114)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#374)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [Remove](../../prelude/struct.Remove.html "struct bevy::prelude::Remove")

where [Remove](../../prelude/struct.Remove.html "struct bevy::prelude::Remove"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#374)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityComponentsTrigger](../event/struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/schedule.rs.html#1652)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [ScheduleBuilt](../schedule/struct.ScheduleBuilt.html "struct bevy::ecs::schedule::ScheduleBuilt")

where [ScheduleBuilt](../schedule/struct.ScheduleBuilt.html "struct bevy::ecs::schedule::ScheduleBuilt"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/schedule.rs.html#1652)

#### type [Trigger](#associatedtype.Trigger)<'a> = [GlobalTrigger](../event/struct.GlobalTrigger.html "struct bevy::ecs::event::GlobalTrigger")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#47)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [ScreenshotCaptured](../../render/view/window/screenshot/struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured")

where [ScreenshotCaptured](../../render/view/window/screenshot/struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#47)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#39)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [ScrollIntoView](../../ui_widgets/struct.ScrollIntoView.html "struct bevy::ui_widgets::ScrollIntoView")

where [ScrollIntoView](../../ui_widgets/struct.ScrollIntoView.html "struct bevy::ui_widgets::ScrollIntoView"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#39)

#### type [Trigger](#associatedtype.Trigger)<'a> = [PropagateEntityTrigger](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<false, [ScrollIntoView](../../ui_widgets/struct.ScrollIntoView.html "struct bevy::ui_widgets::ScrollIntoView"), &'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#181)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [SetChecked](../../ui_widgets/struct.SetChecked.html "struct bevy::ui_widgets::SetChecked")

where [SetChecked](../../ui_widgets/struct.SetChecked.html "struct bevy::ui_widgets::SetChecked"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#181)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#673)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [SetSliderValue](../../ui_widgets/struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue")

where [SetSliderValue](../../ui_widgets/struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#673)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/editing.rs.html#330)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [TextEditChange](../../text/struct.TextEditChange.html "struct bevy::text::TextEditChange")

where [TextEditChange](../../text/struct.TextEditChange.html "struct bevy::text::TextEditChange"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/editing.rs.html#330)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#209)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [ToggleChecked](../../ui_widgets/struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked")

where [ToggleChecked](../../ui_widgets/struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#209)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#170)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [UpdateNumberInput](../../feathers/controls/struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput")

where [UpdateNumberInput](../../feathers/controls/struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#170)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#31)

### impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [WorldInstanceReady](../../world_serialization/struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady")

where [WorldInstanceReady](../../world_serialization/struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#31)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

### impl<E> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"), [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

#### type [Trigger](#associatedtype.Trigger)<'a> = [PropagateEntityTrigger](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<true, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>, [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")\>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#185)

### impl<M> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [FocusedInput](../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), [FocusedInput](../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#185)

#### type [Trigger](#associatedtype.Trigger)<'a> = [PropagateEntityTrigger](../event/struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<true, [FocusedInput](../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>, [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")\>

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#88)

### impl<T> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [ValueChange](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>

where [ValueChange](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#88)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#93)

### impl<T> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") for [VirtualKeyPressed](../../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>

where [VirtualKeyPressed](../../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#93)

#### type [Trigger](#associatedtype.Trigger)<'a> = [EntityTrigger](../event/struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")