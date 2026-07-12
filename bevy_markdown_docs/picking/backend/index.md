[bevy](../../index.html)::[picking](../index.html)

# Module backend 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#159)

This module provides a simple interface for implementing a picking backend.

Don’t be dissuaded by terminology like “backend”; the idea is dead simple. `bevy_picking` will tell you where pointers are, all you have to do is send an event if the pointers are hitting something. That’s it. The rest of this documentation explains the requirements in more detail.

Because `bevy_picking` is very loosely coupled with its backends, you can mix and match as many backends as you want. For example, you could use the `rapier` backend to raycast against physics objects, a picking shader backend to pick non-physics meshes, and the `bevy_ui` backend for your UI. The [`PointerHits`](struct.PointerHits.html "struct bevy::picking::backend::PointerHits") instances produced by these various backends will be combined, sorted, and used as a homogeneous input for the picking systems that consume these events.

### Implementation

*   A picking backend only has one job: read [`PointerLocation`](../pointer/struct.PointerLocation.html "struct bevy::picking::pointer::PointerLocation") components and produce [`PointerHits`](struct.PointerHits.html "struct bevy::picking::backend::PointerHits") events. In plain English, a backend is provided the location of pointers, and is asked to provide a list of entities under those pointers.
    
*   The [`PointerHits`](struct.PointerHits.html "struct bevy::picking::backend::PointerHits") events produced by a backend do **not** need to be sorted or filtered, all that is needed is an unordered list of entities and their [`HitData`](struct.HitData.html "struct bevy::picking::backend::HitData").
    
*   Backends do not need to consider the [`Pickable`](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable") component, though they may use it for optimization purposes. For example, a backend that traverses a spatial hierarchy may want to exit early if it intersects an entity that blocks lower entities from being picked.
    

#### Raycasting Backends

Backends that require a ray to cast into the scene should use [`ray::RayMap`](prelude/struct.RayMap.html "struct bevy::picking::backend::prelude::RayMap"). This automatically constructs rays in world space for all cameras and pointers, handling details like viewports and DPI for you.

## Modules

[prelude](prelude/index.html "mod bevy::picking::backend::prelude")

The picking backend prelude.

[ray](ray/index.html "mod bevy::picking::backend::ray")

Types and systems for constructing rays from cameras and pointers.

## Structs

[HitData](struct.HitData.html "struct bevy::picking::backend::HitData")

Holds data from a successful pointer hit test. See [`HitData::depth`](struct.HitData.html#structfield.depth "field bevy::picking::backend::HitData::depth") for important details.

[PointerHits](struct.PointerHits.html "struct bevy::picking::backend::PointerHits")

A message produced by a picking backend after it has run its hit tests, describing the entities under a pointer.

## Traits

[HitDataExtra](trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra")

Extra data attached to a [`HitData`](struct.HitData.html "struct bevy::picking::backend::HitData") by a picking backend.