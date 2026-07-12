[bevy](../index.html)

# Crate picking 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#1-457)

This crate provides ‘picking’ capabilities for the Bevy game engine, allowing pointers to interact with entities using hover, click, and drag events.

### Overview

In the simplest case, this plugin allows you to click on things in the scene. However, it also allows you to express more complex interactions, like detecting when a touch input drags a UI element and drops it on a 3d mesh rendered to a different camera.

Pointer events bubble up the entity hierarchy and can be used with observers, allowing you to succinctly express rich interaction behaviors by attaching pointer callbacks to entities:

```rust
world.spawn(MyComponent)
    .observe(|mut event: On<Pointer<Click>>| {
        // Read the underlying pointer event data
        println!("Pointer {:?} was just clicked!", event.pointer_id);
        // Stop the event from bubbling up the entity hierarchy
        event.propagate(false);
    });
```

At its core, this crate provides a robust abstraction for computing picking state regardless of pointing devices, or what you are hit testing against. It is designed to work with any input, including mouse, touch, pens, or virtual pointers controlled by gamepads.

### Expressive Events

Although the events in this module (see [`events`](events/index.html "mod bevy::picking::events")) can be listened to with normal `MessageReader`s, using observers is often more expressive, with less boilerplate. This is because observers allow you to attach event handling logic to specific entities, as well as make use of event bubbling.

When events are generated, they bubble up the entity hierarchy starting from their target, until they reach the root or bubbling is halted with a call to [`On::propagate`](../prelude/struct.On.html#method.propagate "method bevy::prelude::On::propagate"). See [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer") for details.

This allows you to run callbacks when any children of an entity are interacted with, and leads to succinct, expressive code:

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Transform::default())
        // Spawn your entity here, e.g. a `Mesh3d`.
        // When dragged, mutate the `Transform` component on the dragged target entity:
        .observe(|drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>| {
            let mut transform = transforms.get_mut(drag.entity).unwrap();
            transform.rotate_local_y(drag.delta.x / 50.0);
        })
        .observe(|click: On<Pointer<Click>>, mut commands: Commands| {
            println!("Entity {} goes BOOM!", click.entity);
            commands.entity(click.entity).despawn();
        })
        .observe(|over: On<Pointer<Over>>, mut greetings: MessageWriter<Greeting>| {
            greetings.write(Greeting);
        });
}
```

### Modularity

##### Mix and Match Hit Testing Backends

The plugin attempts to handle all the hard parts for you, all you need to do is tell it when a pointer is hitting any entities. Multiple backends can be used at the same time! [Use this simple API to write your own backend](backend/index.html "mod bevy::picking::backend") in about 100 lines of code.

##### Input Agnostic

Picking provides a generic Pointer abstraction, which is useful for reacting to many different types of input devices. Pointers can be controlled with anything, whether it’s the included mouse or touch inputs, or a custom gamepad input system you write yourself to control a virtual pointer.

### Robustness

In addition to these features, this plugin also correctly handles multitouch, multiple windows, multiple cameras, viewports, and render layers. Using this as a library allows you to write a picking backend that can interoperate with any other picking backend.

## Getting Started

TODO: This section will need to be re-written once more backends are introduced.

##### Next Steps

To learn more, take a look at the examples in the [examples](https://github.com/bevyengine/bevy/tree/main/examples/picking). You can read the next section to understand how the plugin works.

## The Picking Pipeline

This plugin is designed to be extremely modular. To do so, it works in well-defined stages that form a pipeline, where events are used to pass data between each stage.

##### Pointers ([`pointer`](pointer/index.html "mod bevy::picking::pointer"))

The first stage of the pipeline is to gather inputs and update pointers. This stage is ultimately responsible for generating [`PointerInput`](pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") events. The provided crate does this automatically for mouse, touch, and pen inputs. If you wanted to implement your own pointer, controlled by some other input, you can do that here. The ordering of events within the [`PointerInput`](pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") stream is meaningful for events with the same [`PointerId`](pointer/enum.PointerId.html "enum bevy::picking::pointer::PointerId"), but not between different pointers.

Because pointer positions and presses are driven by these events, you can use them to mock inputs for testing.

After inputs are generated, they are then collected to update the current [`PointerLocation`](pointer/struct.PointerLocation.html "struct bevy::picking::pointer::PointerLocation") for each pointer.

##### Backend ([`backend`](backend/index.html "mod bevy::picking::backend"))

A picking backend only has one job: reading [`PointerLocation`](pointer/struct.PointerLocation.html "struct bevy::picking::pointer::PointerLocation") components, and producing [`PointerHits`](backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits"). You can find all documentation and types needed to implement a backend at [`backend`](backend/index.html "mod bevy::picking::backend").

You will eventually need to choose which picking backend(s) you want to use. This crate does not supply any backends, and expects you to select some from the other bevy crates or the third-party ecosystem.

It’s important to understand that you can mix and match backends! For example, you might have a backend for your UI, and one for the 3d scene, with each being specialized for their purpose. Bevy provides some backends out of the box, but you can even write your own. It’s been made as easy as possible intentionally; the `bevy_mod_raycast` backend is 50 lines of code.

##### Hover ([`hover`](hover/index.html "mod bevy::picking::hover"))

The next step is to use the data from the backends, combine and sort the results, and determine what each cursor is hovering over, producing a [`HoverMap`](hover/struct.HoverMap.html "struct bevy::picking::hover::HoverMap"). Note that just because a pointer is over an entity, it is not necessarily _hovering_ that entity. Although multiple backends may be reporting that a pointer is hitting an entity, the hover system needs to determine which entities are actually being hovered by this pointer based on the pick depth, order of the backend, and the optional [`Pickable`](../prelude/struct.Pickable.html "struct bevy::prelude::Pickable") component of the entity. In other words, if one entity is in front of another, usually only the topmost one will be hovered.

##### Events ([`events`](events/index.html "mod bevy::picking::events"))

In the final step, the high-level pointer events are generated, such as events that trigger when a pointer hovers or clicks an entity. These simple events are then used to generate more complex events for dragging and dropping.

Because it is completely agnostic to the earlier stages of the pipeline, you can easily extend the plugin with arbitrary backends and input methods, yet still use all the high level features.

## Modules

[backend](backend/index.html "mod bevy::picking::backend")

This module provides a simple interface for implementing a picking backend.

[events](events/index.html "mod bevy::picking::events")

This module defines a stateful set of interaction events driven by the `PointerInput` stream and the hover state of each Pointer.

[hover](hover/index.html "mod bevy::picking::hover")

Determines which entities are being hovered by which pointers.

[input](input/index.html "mod bevy::picking::input")

This module provides unsurprising default inputs to `bevy_picking` through [`PointerInput`](pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput"). The included systems are responsible for sending mouse and touch inputs to their respective `Pointer`s.

[mesh\_picking](mesh_picking/index.html "mod bevy::picking::mesh_picking")`mesh_picking`

A [mesh ray casting](mesh_picking/ray_cast/index.html "mod bevy::picking::mesh_picking::ray_cast") backend for [`bevy_picking`](index.html "mod bevy::picking").

[pointer](pointer/index.html "mod bevy::picking::pointer")

Types and systems for pointer inputs, such as position and buttons.

[prelude](prelude/index.html "mod bevy::picking::prelude")

The picking prelude.

[window](window/index.html "mod bevy::picking::window")

This module contains a basic backend that implements picking for window entities.

## Structs

[DefaultPickingPlugins](struct.DefaultPickingPlugins.html "struct bevy::picking::DefaultPickingPlugins")

One plugin that contains the [`PointerInputPlugin`](../prelude/struct.PointerInputPlugin.html "struct bevy::prelude::PointerInputPlugin"), [`PickingPlugin`](../prelude/struct.PickingPlugin.html "struct bevy::prelude::PickingPlugin") and the [`InteractionPlugin`](../prelude/struct.InteractionPlugin.html "struct bevy::prelude::InteractionPlugin"), this is probably the plugin that will be most used.

[InteractionPlugin](struct.InteractionPlugin.html "struct bevy::picking::InteractionPlugin")

Generates [`Pointer`](../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") events and handles event bubbling.

[Pickable](struct.Pickable.html "struct bevy::picking::Pickable")

An optional component that marks an entity as usable by a backend, and overrides default picking behavior for an entity.

[PickingPlugin](struct.PickingPlugin.html "struct bevy::picking::PickingPlugin")

This plugin sets up the core picking infrastructure. It receives input events, and provides the shared types used by other picking plugins.

[PickingSettings](struct.PickingSettings.html "struct bevy::picking::PickingSettings")

Controls the behavior of picking

## Enums

[PickingSystems](enum.PickingSystems.html "enum bevy::picking::PickingSystems")

Groups the stages of the picking process under shared labels.