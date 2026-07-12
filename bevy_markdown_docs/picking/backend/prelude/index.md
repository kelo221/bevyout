[bevy](../../../index.html)::[picking](../../index.html)::[backend](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#44)

The picking backend prelude.

This includes the most common types in this module, re-exported for your convenience.

## Structs

[HitData](struct.HitData.html "struct bevy::picking::backend::prelude::HitData")

Holds data from a successful pointer hit test. See [`HitData::depth`](../struct.HitData.html#structfield.depth "field bevy::picking::backend::HitData::depth") for important details.

[Pickable](struct.Pickable.html "struct bevy::picking::backend::prelude::Pickable")

An optional component that marks an entity as usable by a backend, and overrides default picking behavior for an entity.

[PointerHits](struct.PointerHits.html "struct bevy::picking::backend::prelude::PointerHits")

A message produced by a picking backend after it has run its hit tests, describing the entities under a pointer.

[PointerLocation](struct.PointerLocation.html "struct bevy::picking::backend::prelude::PointerLocation")

Component that tracks a pointer’s current [`Location`](../../pointer/struct.Location.html "struct bevy::picking::pointer::Location").

[RayMap](struct.RayMap.html "struct bevy::picking::backend::prelude::RayMap")

A map from [`RayId`](../ray/struct.RayId.html "struct bevy::picking::backend::ray::RayId") to [`Ray3d`](../../../prelude/struct.Ray3d.html "struct bevy::prelude::Ray3d").

## Enums

[PickingSystems](enum.PickingSystems.html "enum bevy::picking::backend::prelude::PickingSystems")

Groups the stages of the picking process under shared labels.

[PointerId](enum.PointerId.html "enum bevy::picking::backend::prelude::PointerId")

Identifies a unique pointer entity. `Mouse` and `Touch` pointers are automatically spawned.

## Traits

[HitDataExtra](trait.HitDataExtra.html "trait bevy::picking::backend::prelude::HitDataExtra")

Extra data attached to a [`HitData`](../struct.HitData.html "struct bevy::picking::backend::HitData") by a picking backend.