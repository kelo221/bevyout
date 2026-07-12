[bevy](../index.html)::[prelude](index.html)

# Trait EntityMapper 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#231)

```rust
pub trait EntityMapper {
    // Required methods
    fn get_mapped(&mut self, source: Entity) -> Entity;
    fn set_mapped(&mut self, source: Entity, target: Entity);
}
```

An implementor of this trait knows how to map an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") into another [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

Usually this is done by using an [`EntityHashMap<Entity>`](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") to map source entities (mapper inputs) to the current world’s entities (mapper outputs).

More generally, this can be used to map [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") references between any two [`Worlds`](struct.World.html "struct bevy::prelude::World").

This is used by [`MapEntities`](../ecs/entity/trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") implementers.

### Example

```rust
pub struct SimpleEntityMapper {
  map: EntityHashMap<Entity>,
}

// Example implementation of EntityMapper where we map an entity to another entity if it exists
// in the underlying `EntityHashMap`, otherwise we just return the original entity.
impl EntityMapper for SimpleEntityMapper {
    fn get_mapped(&mut self, entity: Entity) -> Entity {
        self.map.get(&entity).copied().unwrap_or(entity)
    }

    fn set_mapped(&mut self, source: Entity, target: Entity) {
        self.map.insert(source, target);
    }
}
```

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#233)

#### fn [get\_mapped](#tymethod.get_mapped)(&mut self, source: [Entity](struct.Entity.html "struct bevy::prelude::Entity")) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

Returns the “target” entity that maps to the given `source`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#237)

#### fn [set\_mapped](#tymethod.set_mapped)(&mut self, source: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), target: [Entity](struct.Entity.html "struct bevy::prelude::Entity"))

Maps the `target` entity to the given `source`. For some implementations this might not actually determine the result of [`EntityMapper::get_mapped`](trait.EntityMapper.html#tymethod.get_mapped "method bevy::prelude::EntityMapper::get_mapped").

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#263)

### impl [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper") for &mut dyn [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#264)

#### fn [get\_mapped](trait.EntityMapper.html#tymethod.get_mapped)(&mut self, source: [Entity](struct.Entity.html "struct bevy::prelude::Entity")) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

Returns the “target” entity that maps to the given `source`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#268)

#### fn [set\_mapped](trait.EntityMapper.html#tymethod.set_mapped)(&mut self, source: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), target: [Entity](struct.Entity.html "struct bevy::prelude::Entity"))

Maps the `target` entity to the given `source`. For some implementations this might not actually determine the result of [`EntityMapper::get_mapped`](trait.EntityMapper.html#tymethod.get_mapped "method bevy::prelude::EntityMapper::get_mapped").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#240)

### impl [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#242)

#### fn [get\_mapped](#tymethod.get_mapped)(&mut self, source: [Entity](struct.Entity.html "struct bevy::prelude::Entity")) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#247)

#### fn [set\_mapped](#tymethod.set_mapped)(&mut self, \_source: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), \_target: [Entity](struct.Entity.html "struct bevy::prelude::Entity"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#250)

### impl [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper") for ([Entity](struct.Entity.html "struct bevy::prelude::Entity"), [Entity](struct.Entity.html "struct bevy::prelude::Entity"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#252)

#### fn [get\_mapped](#tymethod.get_mapped)(&mut self, source: [Entity](struct.Entity.html "struct bevy::prelude::Entity")) -> [Entity](struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#260)

#### fn [set\_mapped](#tymethod.set_mapped)(&mut self, \_source: [Entity](struct.Entity.html "struct bevy::prelude::Entity"), \_target: [Entity](struct.Entity.html "struct bevy::prelude::Entity"))

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#263)

### impl [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper") for &mut dyn [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#297)

### impl [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper") for [EntityHashMap](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<[Entity](struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#273)

### impl [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper") for [SceneEntityMapper](../ecs/entity/struct.SceneEntityMapper.html "struct bevy::ecs::entity::SceneEntityMapper")<'\_>