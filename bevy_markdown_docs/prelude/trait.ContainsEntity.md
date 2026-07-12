[bevy](../index.html)::[prelude](index.html)

# Trait ContainsEntity 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#27)

```rust
pub trait ContainsEntity {
    // Required method
    fn entity(&self) -> Entity;
}
```

A trait for types that contain an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

This trait behaves similarly to `Borrow<Entity>`, but yielding `Entity` directly.

It should only be implemented when:

*   Retrieving the [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") is a simple operation.
*   The [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") contained by the type is unambiguous.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#29)

#### fn [entity](#tymethod.entity)(&self) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

Returns the contained entity.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#80)

### impl<T> [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#81)

#### fn [entity](#tymethod.entity)(&self) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#92)

### impl<T> [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#93)

#### fn [entity](#tymethod.entity)(&self) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#116)

### impl<T> [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>

where T: [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#117)

#### fn [entity](#tymethod.entity)(&self) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#70)

### impl [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [Entity](struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/entity_mut.rs.html#864)

### impl [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [EntityMut](struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/entity_ref.rs.html#354)

### impl [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [EntityRef](struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#791)

### impl [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [FilteredEntityMut](../ecs/world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#274)

### impl [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [FilteredEntityRef](../ecs/world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'\_, '\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#174)

### impl [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [MainEntity](../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#115)

### impl [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [NormalizedWindowRef](../window/struct.NormalizedWindowRef.html "struct bevy::window::NormalizedWindowRef")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#146)

### impl [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [RenderEntity](../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/unsafe_world_cell.rs.html#1416)

### impl [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [UnsafeEntityCell](../ecs/world/unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/except.rs.html#536)

### impl<B> [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [EntityMutExcept](../ecs/world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, '\_, B>

where B: [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/except.rs.html#252)

### impl<B> [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [EntityRefExcept](../ecs/world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'\_, '\_, B>

where B: [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#128)

### impl<T> [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#104)

### impl<T> [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [Box](struct.Box.html "struct bevy::prelude::Box")<T>

where T: [ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity"),