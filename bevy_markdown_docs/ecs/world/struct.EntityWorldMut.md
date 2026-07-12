[bevy](../../index.html)::[ecs](../index.html)::[world](index.html)

# Struct EntityWorldMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#53)

```rust
pub struct EntityWorldMut<'w> { /* private fields */ }
```

A mutable reference to a particular [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), and the entire world.

This is essentially a performance-optimized `(Entity, &mut World)` tuple, which caches the [`EntityLocation`](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation") to reduce duplicate lookups.

Since this type provides mutable access to the entire world, only one [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") can exist at a time for a given world.

See also [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut"), which allows disjoint mutable access to multiple entities at once. Unlike `EntityMut`, this type allows adding and removing components, and despawning the entity.

## Invariants and Risk

An [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") may point to a despawned entity. You can check this via [`is_despawned`](../../prelude/struct.EntityWorldMut.html#method.is_despawned "method bevy::prelude::EntityWorldMut::is_despawned"). Using an [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") of a despawned entity may panic in some contexts, so read method documentation carefully.

Unless you have strong reason to assume these invariants, you should generally avoid keeping an [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") to an entity that is potentially not spawned. For example, when inserting a component, that component insert may trigger an observer that despawns the entity. So, when you don’t have full knowledge of what commands may interact with this entity, do not further use this value without first checking [`is_despawned`](../../prelude/struct.EntityWorldMut.html#method.is_despawned "method bevy::prelude::EntityWorldMut::is_despawned").

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#269)

### impl<'w> [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#272)

#### pub fn [with\_children](#method.with_children)( &mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [RelatedSpawner](../relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner")<'\_, [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>), ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Spawns children of this entity (with a [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship) by taking a function that operates on a [`ChildSpawner`](../../prelude/type.ChildSpawner.html "type bevy::prelude::ChildSpawner"). See also [`with_related`](../../prelude/struct.EntityWorldMut.html#method.with_related "method bevy::prelude::EntityWorldMut::with_related").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#279)

#### pub fn [add\_children](#method.add_children)(&mut self, children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\]) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Adds the given children to this entity. See also [`add_related`](../../prelude/struct.EntityWorldMut.html#method.add_related "method bevy::prelude::EntityWorldMut::add_related").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#286)

#### pub fn [clear\_children](#method.clear_children)(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

👎Deprecated:

Use detach\_all\_children() instead

Removes all the children from this entity. See also [`detach_all_related`](../../prelude/struct.EntityWorldMut.html#method.detach_all_related "method bevy::prelude::EntityWorldMut::detach_all_related")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#293)

#### pub fn [detach\_all\_children](#method.detach_all_children)(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Removes all the parent-child relationships from this entity. To despawn the child entities, instead use [`EntityWorldMut::despawn_children`](../../prelude/struct.EntityWorldMut.html#method.despawn_children "method bevy::prelude::EntityWorldMut::despawn_children"). See also [`detach_all_related`](../../prelude/struct.EntityWorldMut.html#method.detach_all_related "method bevy::prelude::EntityWorldMut::detach_all_related")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#299)

#### pub fn [insert\_children](#method.insert_children)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Insert children at specific index. See also [`insert_related`](../../prelude/struct.EntityWorldMut.html#method.insert_related "method bevy::prelude::EntityWorldMut::insert_related").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#305)

#### pub fn [insert\_child](#method.insert_child)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), child: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Insert child at specific index. See also [`insert_related`](../../prelude/struct.EntityWorldMut.html#method.insert_related "method bevy::prelude::EntityWorldMut::insert_related").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#311)

#### pub fn [add\_child](#method.add_child)(&mut self, child: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Adds the given child to this entity. See also [`add_related`](../../prelude/struct.EntityWorldMut.html#method.add_related "method bevy::prelude::EntityWorldMut::add_related").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#317)

#### pub fn [remove\_children](#method.remove_children)( &mut self, children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

👎Deprecated:

Use detach\_children() instead

Removes the relationship between this entity and the given entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#323)

#### pub fn [detach\_children](#method.detach_children)( &mut self, children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Removes the parent-child relationship between this entity and the given entities. Does not despawn the children.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#329)

#### pub fn [remove\_child](#method.remove_child)(&mut self, child: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

👎Deprecated:

Use detach\_child() instead

Removes the relationship between this entity and the given entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#335)

#### pub fn [detach\_child](#method.detach_child)(&mut self, child: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Removes the parent-child relationship between this entity and the given entity. Does not despawn the child.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#340)

#### pub fn [replace\_children](#method.replace_children)( &mut self, children: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Replaces all the related children with a new set of children.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#354-359)

#### pub fn [replace\_children\_with\_difference](#method.replace_children_with_difference)( &mut self, entities\_to\_unrelate: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], entities\_to\_relate: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], newly\_related\_entities: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Replaces all the related children with a new set of children.

##### Warning

Failing to maintain the functions invariants may lead to erratic engine behavior including random crashes. Refer to [`Self::replace_related_with_difference`](../../prelude/struct.EntityWorldMut.html#method.replace_related_with_difference "method bevy::prelude::EntityWorldMut::replace_related_with_difference") for a list of these invariants.

##### Panics

Panics when debug assertions are enabled if an invariant is broken and the command is executed.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#372)

#### pub fn [with\_child](#method.with_child)(&mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Spawns the passed bundle and adds it to this entity as a child.

For efficient spawning of multiple children, use [`with_children`](../../prelude/struct.EntityWorldMut.html#method.with_children "method bevy::prelude::EntityWorldMut::with_children").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#204)

### impl<'w> [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#222)

#### pub fn [insert\_reflect](#method.insert_reflect)( &mut self, component: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Adds the given boxed reflect component or bundle to the entity using the reflection data in [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry").

This will overwrite any previous component(s) of the same type.

##### Panics

*   If the entity has been despawned while this `EntityWorldMut` is still alive.
*   If [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") does not have the reflection data for the given [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") or [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle").
*   If the component or bundle data is invalid. See [`PartialReflect::apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") for further details.
*   If [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") is not present in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### Note

Prefer to use the typed [`EntityWorldMut::insert`](../../prelude/struct.EntityWorldMut.html#method.insert "method bevy::prelude::EntityWorldMut::insert") if possible. Adding a reflected component is much slower.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#243-246)

#### pub fn [insert\_reflect\_with\_registry](#method.insert_reflect_with_registry)<T>( &mut self, component: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry")\>,

Same as [`insert_reflect`](../../prelude/struct.EntityWorldMut.html#method.insert_reflect "method bevy::prelude::EntityWorldMut::insert_reflect"), but using the `T` resource as type registry instead of [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry").

This will overwrite any previous component(s) of the same type.

##### Panics

*   If the entity has been despawned while this `EntityWorldMut` is still alive.
*   If the given [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") does not have the reflection data for the given [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") or [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle").
*   If the component or bundle data is invalid. See [`PartialReflect::apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") for further details.
*   If the given [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") is not present in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#273)

#### pub fn [remove\_reflect](#method.remove_reflect)( &mut self, component\_type\_path: [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Removes from the entity the component or bundle with the given type path registered in [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry").

If the type is a bundle, it will remove any components in that bundle regardless if the entity contains all the components.

Does nothing if the type is a component and the entity does not have a component of the same type, if the type is a bundle and the entity does not contain any of the components in the bundle, or if [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") does not contain the reflection data for the given component.

##### Panics

*   If the entity has been despawned while this `EntityWorldMut` is still alive.
*   If [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") is not present in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### Note

Prefer to use the typed [`EntityWorldMut::remove`](../../prelude/struct.EntityWorldMut.html#method.remove "method bevy::prelude::EntityWorldMut::remove") if possible. Removing a reflected component is much slower.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#296-299)

#### pub fn [remove\_reflect\_with\_registry](#method.remove_reflect_with_registry)<T>( &mut self, component\_type\_path: [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry")\>,

Same as [`remove_reflect`](../../prelude/struct.EntityWorldMut.html#method.remove_reflect "method bevy::prelude::EntityWorldMut::remove_reflect"), but using the `T` resource as type registry instead of `AppTypeRegistry`.

If the given type is a bundle, it will remove any components in that bundle regardless if the entity contains all the components.

Does nothing if the type is a component and the entity does not have a component of the same type, if the type is a bundle and the entity does not contain any of the components in the bundle, or if [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") does not contain the reflection data for the given component.

##### Panics

*   If the entity has been despawned while this `EntityWorldMut` is still alive.
*   If [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") is not present in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#323-326)

#### pub fn [take\_reflect](#method.take_reflect)( &mut self, component\_type\_path: [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Takes from the entity the component or bundle with the given type path registered in [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry").

Does nothing and returns None if the type is a component and the entity does not have a component of the same type, if the type is a bundle and the entity does not contain **every** component in the bundle, or if [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") does not contain the reflection data for the given component.

##### Panics

*   If the entity has been despawned while this `EntityWorldMut` is still alive.
*   If [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") is not present in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### Note

Prefer to use the typed [`EntityWorldMut::take`](../../prelude/struct.EntityWorldMut.html#method.take "method bevy::prelude::EntityWorldMut::take") if possible. Taking a reflected component is much slower.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#345-348)

#### pub fn [take\_reflect\_with\_registry](#method.take_reflect_with_registry)<T>( &mut self, component\_type\_path: [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry")\>,

Same as [`take_reflect`](../../prelude/struct.EntityWorldMut.html#method.take_reflect "method bevy::prelude::EntityWorldMut::take_reflect"), but using the `T` resource as type registry instead of `AppTypeRegistry`.

Does nothing and returns None if the type is a component and the entity does not have a component of the same type, if the type is a bundle and the entity does not contain **every** component in the bundle, or if [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") does not contain the reflection data for the given component.

##### Panics

*   If the entity has been despawned while this `EntityWorldMut` is still alive.
*   If [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") is not present in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#16)

### impl<'w> [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#18)

#### pub fn [with\_related](#method.with_related)<R>( &mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Spawns a entity related to this entity (with the `R` relationship) by taking a bundle

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#27-30)

#### pub fn [with\_related\_entities](#method.with_related_entities)<R>( &mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [RelatedSpawner](../relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner")<'\_, R>), ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Spawns entities related to this entity (with the `R` relationship) by taking a function that operates on a [`RelatedSpawner`](../relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#41)

#### pub fn [add\_related](#method.add_related)<R>(&mut self, related: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\]) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Relates the given entities to this entity with the relation `R`.

See [`add_one_related`](../../prelude/struct.EntityWorldMut.html#method.add_one_related "method bevy::prelude::EntityWorldMut::add_one_related") if you want relate only one entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#57)

#### pub fn [detach\_all\_related](#method.detach_all_related)<R>(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Removes the relation `R` between this entity and all its related entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#87-90)

#### pub fn [insert\_related](#method.insert_related)<R>( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), related: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), <<R as [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship")\>::[RelationshipTarget](../relationship/trait.Relationship.html#associatedtype.RelationshipTarget "type bevy::ecs::relationship::Relationship::RelationshipTarget") as [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget")\>::[Collection](../../prelude/trait.RelationshipTarget.html#associatedtype.Collection "type bevy::prelude::RelationshipTarget::Collection"): [OrderedRelationshipSourceCollection](../relationship/trait.OrderedRelationshipSourceCollection.html "trait bevy::ecs::relationship::OrderedRelationshipSourceCollection"),

Relates the given entities to this entity with the relation `R`, starting at this particular index.

If the `related` has duplicates, a related entity will take the index of its last occurrence in `related`. If the indices go out of bounds, they will be clamped into bounds. This will not re-order existing related entities unless they are in `related`.

##### Example

```rust
use bevy_ecs::prelude::*;

let mut world = World::new();
let e0 = world.spawn_empty().id();
let e1 = world.spawn_empty().id();
let e2 = world.spawn_empty().id();
let e3 = world.spawn_empty().id();
let e4 = world.spawn_empty().id();

let mut main_entity = world.spawn_empty();
main_entity.add_related::<ChildOf>(&[e0, e1, e2, e2]);
main_entity.insert_related::<ChildOf>(1, &[e0, e3, e4, e4]);
let main_id = main_entity.id();

let relationship_source = main_entity.get::<Children>().unwrap().collection();
assert_eq!(relationship_source, &[e1, e0, e3, e2, e4]);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#125)

#### pub fn [remove\_related](#method.remove_related)<R>( &mut self, related: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Removes the relation `R` between this entity and the given entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#142)

#### pub fn [replace\_related](#method.replace_related)<R>( &mut self, related: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Replaces all the related entities with a new set of entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#216-221)

#### pub fn [replace\_related\_with\_difference](#method.replace_related_with_difference)<R>( &mut self, entities\_to\_unrelate: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], entities\_to\_relate: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], newly\_related\_entities: &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Replaces all the related entities with a new set of entities.

This is a more efficient of [`Self::replace_related`](../../prelude/struct.EntityWorldMut.html#method.replace_related "method bevy::prelude::EntityWorldMut::replace_related") which doesn’t allocate. The passed in arguments must adhere to these invariants:

*   `entities_to_unrelate`: A slice of entities to remove from the relationship source. Entities need not be related to this entity, but must not appear in `entities_to_relate`
*   `entities_to_relate`: A slice of entities to relate to this entity. This must contain all entities that will remain related (i.e. not those in `entities_to_unrelate`) plus the newly related entities.
*   `newly_related_entities`: A subset of `entities_to_relate` containing only entities not already related to this entity.
*   Slices **must not** contain any duplicates

##### Warning

Violating these invariants may lead to panics, crashes or unpredictable engine behavior.

##### Panics

Panics when debug assertions are enabled and any invariants are broken.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#295)

#### pub fn [add\_one\_related](#method.add_one_related)<R>(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

Relates the given entity to this with the relation `R`.

See [`add_related`](../../prelude/struct.EntityWorldMut.html#method.add_related "method bevy::prelude::EntityWorldMut::add_related") if you want to relate more than one entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#301)

#### pub fn [despawn\_related](#method.despawn_related)<S>(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where S: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"),

Despawns entities that relate to this one via the given [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"). This entity will not be despawned.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#321)

#### pub fn [despawn\_children](#method.despawn_children)(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Despawns the children of this entity. This entity will not be despawned.

This is a specialization of [`despawn_related`](../../prelude/struct.EntityWorldMut.html#method.despawn_related "method bevy::prelude::EntityWorldMut::despawn_related"), a more general method for despawning via relationships.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#335-338)

#### pub fn [insert\_recursive](#method.insert_recursive)<S>( &mut self, bundle: impl [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where S: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"),

Inserts a component or bundle of components into the entity and all related entities, traversing the relationship tracked in `S` in a breadth-first manner.

##### Warning

This method should only be called on relationships that form a tree-like structure. Any cycles will cause this method to loop infinitely.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/related_methods.rs.html#361)

#### pub fn [remove\_recursive](#method.remove_recursive)<S, B>(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where S: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes a component or bundle of components of type `B` from the entity and all related entities, traversing the relationship tracked in `S` in a breadth-first manner.

##### Warning

This method should only be called on relationships that form a tree-like structure. Any cycles will cause this method to loop infinitely.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#59)

### impl<'w> [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#144)

#### pub fn [into\_readonly](#method.into_readonly)(self) -> [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>

Consumes `self` and returns read-only access to all of the entity’s components, with the world `'w` lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#153)

#### pub fn [as\_readonly](#method.as_readonly)(&self) -> [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>

Gets read-only access to all of the entity’s components.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#162)

#### pub fn [into\_mutable](#method.into_mutable)(self) -> [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>

Consumes `self` and returns non-structural mutable access to all of the entity’s components, with the world `'w` lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#171)

#### pub fn [as\_mutable](#method.as_mutable)(&mut self) -> [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>

Gets non-structural mutable access to all of the entity’s components.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#181)

#### pub fn [id](#method.id)(&self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Returns the [ID](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") of the current entity.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/app/externally\_driven\_headless\_renderer.rs ([line 101](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#101))

```rust
96    fn spawn_camera(&mut self, target: RenderTarget) -> Entity {
97        self.0
98            .main
99            .world_mut()
100            .spawn((Camera3d::default(), target, Transform::IDENTITY))
101            .id()
102    }
```

Hide additional examples

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 252](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#252))

```rust
245    fn on_gltf_node(
246        &mut self,
247        _load_context: &mut LoadContext<'_>,
248        gltf_node: &gltf::Node,
249        entity: &mut EntityWorldMut,
250    ) {
251        if self.animation_root_indices.contains(&gltf_node.index()) {
252            self.animation_root_entities.insert(entity.id());
253        }
254    }
```

examples/ecs/immutable\_components.rs ([line 108](../../../src/immutable_components/immutable_components.rs.html#108))

```rust
103fn demo_2(world: &mut World) {
104    // Setup our name index
105    world.init_resource::<NameIndex>();
106
107    // Spawn some entities!
108    let alyssa = world.spawn(Name("Alyssa")).id();
109    let javier = world.spawn(Name("Javier")).id();
110
111    // Check our index
112    let index = world.resource::<NameIndex>();
113
114    assert_eq!(index.get_entity("Alyssa"), Some(alyssa));
115    assert_eq!(index.get_entity("Javier"), Some(javier));
116
117    // Changing the name of an entity is also fully capture by our index
118    world.entity_mut(javier).insert(Name("Steven"));
119
120    // Javier changed their name to Steven
121    let steven = javier;
122
123    // Check our index
124    let index = world.resource::<NameIndex>();
125
126    assert_eq!(index.get_entity("Javier"), None);
127    assert_eq!(index.get_entity("Steven"), Some(steven));
128}
```

examples/ui/scroll\_and\_overflow/scrollbars.rs ([line 95](../../../src/scrollbars/scrollbars.rs.html#95))

```rust
51fn scroll_area_demo() -> impl Bundle {
52    (
53        // Frame element which contains the scroll area and scrollbars.
54        Node {
55            display: Display::Grid,
56            width: px(200),
57            height: px(150),
58            grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
59            grid_template_rows: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
60            row_gap: px(2),
61            column_gap: px(2),
62            ..default()
63        },
64        Children::spawn((SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
65            // The actual scrolling area.
66            // Note that we're using `SpawnWith` here because we need to get the entity id of the
67            // scroll area in order to set the target of the scrollbars.
68            let scroll_area_id = parent
69                .spawn((
70                    Node {
71                        display: Display::Flex,
72                        flex_direction: FlexDirection::Column,
73                        padding: UiRect::all(px(4)),
74                        overflow: Overflow::scroll(),
75                        ..default()
76                    },
77                    BackgroundColor(colors::GRAY1.into()),
78                    ScrollPosition(Vec2::new(0.0, 10.0)),
79                    Children::spawn((
80                        // The actual content of the scrolling area
81                        Spawn(text_row("Alpha Wolf")),
82                        Spawn(text_row("Beta Blocker")),
83                        Spawn(text_row("Delta Sleep")),
84                        Spawn(text_row("Gamma Ray")),
85                        Spawn(text_row("Epsilon Eridani")),
86                        Spawn(text_row("Zeta Function")),
87                        Spawn(text_row("Lambda Calculus")),
88                        Spawn(text_row("Nu Metal")),
89                        Spawn(text_row("Pi Day")),
90                        Spawn(text_row("Chi Pants")),
91                        Spawn(text_row("Psi Powers")),
92                        Spawn(text_row("Omega Fatty Acid")),
93                    )),
94                ))
95                .id();
96
97            // Vertical scrollbar
98            parent.spawn((
99                Node {
100                    min_width: px(8),
101                    grid_row: GridPlacement::start(1),
102                    grid_column: GridPlacement::start(2),
103                    ..default()
104                },
105                Scrollbar {
106                    orientation: ControlOrientation::Vertical,
107                    target: scroll_area_id,
108                    min_thumb_length: 8.0,
109                },
110                Children::spawn(Spawn((
111                    Hovered::default(),
112                    BackgroundColor(colors::GRAY2.into()),
113                    BorderColor::all(colors::GRAY3),
114                    ScrollbarThumb {
115                        border_radius: BorderRadius::all(px(4)),
116                        border: px(1).all(),
117                    },
118                ))),
119            ));
120
121            // Horizontal scrollbar
122            parent.spawn((
123                Node {
124                    min_height: px(8),
125                    grid_row: GridPlacement::start(2),
126                    grid_column: GridPlacement::start(1),
127                    ..default()
128                },
129                Scrollbar {
130                    orientation: ControlOrientation::Horizontal,
131                    target: scroll_area_id,
132                    min_thumb_length: 8.0,
133                },
134                Children::spawn(Spawn((
135                    Hovered::default(),
136                    BackgroundColor(colors::GRAY2.into()),
137                    BorderColor::all(colors::GRAY3),
138                    ScrollbarThumb {
139                        border_radius: BorderRadius::all(px(4)),
140                        border: px(1).all(),
141                    },
142                ))),
143            ));
144        }),)),
145    )
146}
```

examples/ecs/dynamic.rs ([line 173](../../../src/dynamic/dynamic.rs.html#173))

```rust
69fn main() {
70    let mut world = World::new();
71    let mut lines = std::io::stdin().lines();
72    let mut component_names = HashMap::<String, ComponentId>::new();
73    let mut component_info = HashMap::<ComponentId, ComponentInfo>::new();
74    let mut event_names = HashMap::<String, EventKey>::new();
75
76    println!("{PROMPT}");
77    loop {
78        print!("\n> ");
79        let _ = std::io::stdout().flush();
80        let Some(Ok(line)) = lines.next() else {
81            return;
82        };
83
84        if line.is_empty() {
85            return;
86        };
87
88        let Some((first, rest)) = line.trim().split_once(|c: char| c.is_whitespace()) else {
89            match &line.chars().next() {
90                Some('c') => println!("{COMPONENT_PROMPT}"),
91                Some('s') => println!("{ENTITY_PROMPT}"),
92                Some('q') => println!("{QUERY_PROMPT}"),
93                Some('e') => println!("{EVENT_PROMPT}"),
94                Some('t') => println!("{EMIT_PROMPT}"),
95                _ => println!("{PROMPT}"),
96            }
97            continue;
98        };
99
100        match &first[0..1] {
101            "c" => {
102                rest.split(',').for_each(|component| {
103                    let mut component = component.split_whitespace();
104                    let Some(name) = component.next() else {
105                        return;
106                    };
107                    let size = match component.next().map(str::parse) {
108                        Some(Ok(size)) => size,
109                        _ => 0,
110                    };
111                    // Register our new component to the world with a layout specified by it's size
112                    // SAFETY: [u64] is Send + Sync
113                    let id = world.register_component_with_descriptor(unsafe {
114                        ComponentDescriptor::new_with_layout(
115                            name.to_string(),
116                            StorageType::Table,
117                            Layout::array::<u64>(size).unwrap(),
118                            None,
119                            true,
120                            ComponentCloneBehavior::Default,
121                            None,
122                        )
123                    });
124                    let Some(info) = world.components().get_info(id) else {
125                        return;
126                    };
127                    component_names.insert(name.to_string(), id);
128                    component_info.insert(id, info.clone());
129                    println!("Component {} created with id: {}", name, id.index());
130                });
131            }
132            "s" => {
133                let mut to_insert_ids = Vec::new();
134                let mut to_insert_data = Vec::new();
135                rest.split(',').for_each(|component| {
136                    let mut component = component.split_whitespace();
137                    let Some(name) = component.next() else {
138                        return;
139                    };
140
141                    // Get the id for the component with the given name
142                    let Some(&id) = component_names.get(name) else {
143                        println!("Component {name} does not exist");
144                        return;
145                    };
146
147                    // Calculate the length for the array based on the layout created for this component id
148                    let info = world.components().get_info(id).unwrap();
149                    let len = info.layout().size() / size_of::<u64>();
150                    let mut values: Vec<u64> = component
151                        .take(len)
152                        .filter_map(|value| value.parse::<u64>().ok())
153                        .collect();
154                    values.resize(len, 0);
155
156                    // Collect the id and array to be inserted onto our entity
157                    to_insert_ids.push(id);
158                    to_insert_data.push(values);
159                });
160
161                let mut entity = world.spawn_empty();
162
163                // Construct an `OwningPtr` for each component in `to_insert_data`
164                let to_insert_ptr = to_owning_ptrs(&mut to_insert_data);
165
166                // SAFETY:
167                // - Component ids have been taken from the same world
168                // - Each array is created to the layout specified in the world
169                unsafe {
170                    entity.insert_by_ids(&to_insert_ids, to_insert_ptr.into_iter());
171                }
172
173                println!("Entity spawned with id: {}", entity.id());
174            }
175            "q" => {
176                let mut builder = QueryBuilder::<FilteredEntityMut>::new(&mut world);
177                parse_query(rest, &mut builder, &component_names);
178                let mut query = builder.build();
179                query.iter_mut(&mut world).for_each(|filtered_entity| {
180                    let terms = filtered_entity
181                        .access()
182                        .try_iter_access()
183                        .unwrap()
184                        .map(|component_access| {
185                            let id = *component_access.index();
186                            let ptr = filtered_entity.get_by_id(id).unwrap();
187                            let info = component_info.get(&id).unwrap();
188                            let len = info.layout().size() / size_of::<u64>();
189
190                            // SAFETY:
191                            // - All components are created with layout [u64]
192                            // - len is calculated from the component descriptor
193                            let data = unsafe {
194                                std::slice::from_raw_parts_mut(
195                                    ptr.assert_unique().as_ptr().cast::<u64>(),
196                                    len,
197                                )
198                            };
199
200                            // If we have write access, increment each value once
201                            if matches!(component_access, ComponentAccessKind::Exclusive(_)) {
202                                data.iter_mut().for_each(|data| {
203                                    *data += 1;
204                                });
205                            }
206
207                            format!("{}: {:?}", info.name(), data[0..len].to_vec())
208                        })
209                        .collect::<Vec<_>>()
210                        .join(", ");
211
212                    println!("{}: {}", filtered_entity.id(), terms);
213                });
214            }
215            "e" => {
216                rest.split(',').for_each(|event| {
217                    let name = event.trim();
218                    if name.is_empty() {
219                        return;
220                    }
221
222                    // Register a ComponentId for this event, no Rust type needed.
223                    // SAFETY: ZST with no drop
224                    let event_component_id = world.register_component_with_descriptor(unsafe {
225                        ComponentDescriptor::new_with_layout(
226                            format!("event:{name}"),
227                            StorageType::Table,
228                            Layout::new::<()>(),
229                            None,
230                            false,
231                            ComponentCloneBehavior::Ignore,
232                            None,
233                        )
234                    });
235                    // SAFETY: event_component_id was just registered for this event
236                    let event_key = unsafe { EventKey::new(event_component_id) };
237                    event_names.insert(name.to_string(), event_key);
238
239                    // Build a dynamic observer that prints when the event fires.
240                    let runner: ObserverRunner = |mut world, _observer, ctx, _event, _trigger| {
241                        println!("  Observer fired!");
242                        if let Some(mut counts) = world.get_resource_mut::<EventFireCount>() {
243                            *counts.0.entry(ctx.event_key).or_insert(0) += 1;
244                        }
245                    };
246
247                    // SAFETY: event_key was just registered, runner ignores pointers
248                    let observer =
249                        unsafe { Observer::with_dynamic_runner(runner).with_event_key(event_key) };
250                    world.spawn(observer);
251
252                    println!(
253                        "Event '{name}' registered (key: {}) with a dynamic observer",
254                        event_component_id.index()
255                    );
256                });
257
258                // Ensure the counter resource exists.
259                world.init_resource::<EventFireCount>();
260            }
261            "t" => {
262                let name = rest.trim();
263                let Some(&event_key) = event_names.get(name) else {
264                    println!(
265                        "Event '{name}' does not exist. Register it first with 'event {name}'"
266                    );
267                    continue;
268                };
269
270                let mut event_data = ();
271                let mut trigger_data = ();
272                // SAFETY: event_key was registered in this world, both pointers are valid ZSTs
273                unsafe {
274                    world.trigger_dynamic(
275                        event_key,
276                        PtrMut::from(&mut event_data),
277                        PtrMut::from(&mut trigger_data),
278                    );
279                }
280
281                let count = world
282                    .get_resource::<EventFireCount>()
283                    .map_or(0, |c| c.0.get(&event_key).copied().unwrap_or(0));
284                println!("Event '{name}' triggered ({count} fires)");
285            }
286            _ => continue,
287        }
288    }
289}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#187)

#### pub fn [try\_location](#method.try_location)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")\>

Gets metadata indicating the location where the current entity is stored.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#193)

#### pub fn [is\_spawned](#method.is_spawned)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns if the entity is spawned or not.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#199)

#### pub fn [try\_archetype](#method.try_archetype)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")\>

Returns the archetype that the current entity belongs to.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#210)

#### pub fn [location](#method.location)(&self) -> [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

Gets metadata indicating the location where the current entity is stored.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#223)

#### pub fn [archetype](#method.archetype)(&self) -> &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

Returns the archetype that the current entity belongs to.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#242)

#### pub fn [contains](#method.contains)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Returns `true` if the current entity has a component of type `T`. Otherwise, this returns `false`.

###### Notes

If you do not know the concrete type of a component, consider using [`Self::contains_id`](../../prelude/struct.EntityWorldMut.html#method.contains_id "method bevy::prelude::EntityWorldMut::contains_id") or [`Self::contains_type_id`](../../prelude/struct.EntityWorldMut.html#method.contains_type_id "method bevy::prelude::EntityWorldMut::contains_type_id").

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#259)

#### pub fn [contains\_id](#method.contains_id)(&self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the current entity has a component identified by `component_id`. Otherwise, this returns false.

###### Notes

*   If you know the concrete type of the component, you should prefer [`Self::contains`](../../prelude/struct.EntityWorldMut.html#method.contains "method bevy::prelude::EntityWorldMut::contains").
*   If you know the component’s [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") but not its [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), consider using [`Self::contains_type_id`](../../prelude/struct.EntityWorldMut.html#method.contains_type_id "method bevy::prelude::EntityWorldMut::contains_type_id").

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#276)

#### pub fn [contains\_type\_id](#method.contains_type_id)(&self, type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the current entity has a component with the type identified by `type_id`. Otherwise, this returns false.

###### Notes

*   If you know the concrete type of the component, you should prefer [`Self::contains`](../../prelude/struct.EntityWorldMut.html#method.contains "method bevy::prelude::EntityWorldMut::contains").
*   If you have a [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") instead of a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), consider using [`Self::contains_id`](../../prelude/struct.EntityWorldMut.html#method.contains_id "method bevy::prelude::EntityWorldMut::contains_id").

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#288)

#### pub fn [get](#method.get)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Gets access to the component of type `T` for the current entity. Returns `None` if the entity does not have a component of type `T`.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/gltf/gltf\_extension\_mesh\_2d.rs ([line 108](../../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#108))

```rust
99    fn on_spawn_mesh_and_material(
100        &mut self,
101        load_context: &mut LoadContext<'_>,
102        _primitive: &gltf::Primitive,
103        _mesh: &gltf::Mesh,
104        _material: &gltf::Material,
105        entity: &mut EntityWorldMut,
106        _material_label: &str,
107    ) {
108        if let Some(mesh3d) = entity.get::<Mesh3d>() {
109            let material_handle =
110                load_context.add_labeled_asset("AColorMaterial".to_string(), CustomMaterial {});
111            let mesh_handle = mesh3d.0.clone();
112            entity
113                .remove::<Mesh3d>()
114                .insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle.clone())));
115        }
116    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#299-301)

#### pub fn [components](#method.components)<Q>(&self) -> <Q as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 'static>

where Q: [ReadOnlyQueryData](../query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") + [ReleaseStateQueryData](../query/trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") + [SingleEntityQueryData](../query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Returns read-only components for the current entity that match the query `Q`.

##### Panics

If the entity does not have the components required by the query `Q` or if the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#312-314)

#### pub fn [get\_components](#method.get_components)<Q>( &self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Q as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 'static>, [QueryAccessError](../query/enum.QueryAccessError.html "enum bevy::ecs::query::QueryAccessError")\>

where Q: [ReadOnlyQueryData](../query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") + [ReleaseStateQueryData](../query/trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") + [SingleEntityQueryData](../query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Returns read-only components for the current entity that match the query `Q`, or `None` if the entity does not have the components required by the query `Q`.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#350-352)

#### pub unsafe fn [get\_components\_mut\_unchecked](#method.get_components_mut_unchecked)<Q>( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Q as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 'static>, [QueryAccessError](../query/enum.QueryAccessError.html "enum bevy::ecs::query::QueryAccessError")\>

where Q: [ReleaseStateQueryData](../query/trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") + [SingleEntityQueryData](../query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Returns components for the current entity that match the query `Q`, or `None` if the entity does not have the components required by the query `Q`.

##### Example

```rust
#[derive(Component)]
struct X(usize);
#[derive(Component)]
struct Y(usize);

let mut entity = world.spawn((X(0), Y(0)));
// Get mutable access to two components at once
// SAFETY: X and Y are different components
let (mut x, mut y) =
    unsafe { entity.get_components_mut_unchecked::<(&mut X, &mut Y)>() }.unwrap();
*x = X(1);
*y = Y(1);
// This would trigger undefined behavior, as the `&mut X`s would alias:
// entity.get_components_mut_unchecked::<(&mut X, &mut X)>();
```

##### Safety

It is the caller’s responsibility to ensure that the `QueryData` does not provide aliasing mutable references to the same component.

/// # See also

*   [`Self::get_components_mut`](../../prelude/struct.EntityWorldMut.html#method.get_components_mut "method bevy::prelude::EntityWorldMut::get_components_mut") for the safe version that performs aliasing checks

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#380-382)

#### pub fn [get\_components\_mut](#method.get_components_mut)<Q>( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Q as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, 'static>, [QueryAccessError](../query/enum.QueryAccessError.html "enum bevy::ecs::query::QueryAccessError")\>

where Q: [ReleaseStateQueryData](../query/trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") + [SingleEntityQueryData](../query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Returns components for the current entity that match the query `Q`. In the case of conflicting [`QueryData`](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), unregistered components, or missing components, this will return a [`QueryAccessError`](../query/enum.QueryAccessError.html "enum bevy::ecs::query::QueryAccessError")

##### Example

```rust
#[derive(Component)]
struct X(usize);
#[derive(Component)]
struct Y(usize);

let mut entity = world.spawn((X(0), Y(0))).into_mutable();
// Get mutable access to two components at once
// SAFETY: X and Y are different components
let (mut x, mut y) = entity.get_components_mut::<(&mut X, &mut Y)>().unwrap();
```

Note that this does a O(n^2) check that the [`QueryData`](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData") does not conflict. If performance is a consideration you should use [`Self::get_components_mut_unchecked`](../../prelude/struct.EntityWorldMut.html#method.get_components_mut_unchecked "method bevy::prelude::EntityWorldMut::get_components_mut_unchecked") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#418-422)

#### pub unsafe fn [into\_components\_mut\_unchecked](#method.into_components_mut_unchecked)<Q>( self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Q as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 'static>, [QueryAccessError](../query/enum.QueryAccessError.html "enum bevy::ecs::query::QueryAccessError")\>

where Q: [ReleaseStateQueryData](../query/trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") + [SingleEntityQueryData](../query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Consumes self and returns components for the current entity that match the query `Q` for the world lifetime `'w`, or `None` if the entity does not have the components required by the query `Q`.

##### Example

```rust
#[derive(Component)]
struct X(usize);
#[derive(Component)]
struct Y(usize);

let mut entity = world.spawn((X(0), Y(0)));
// Get mutable access to two components at once
// SAFETY: X and Y are different components
let (mut x, mut y) =
    unsafe { entity.into_components_mut_unchecked::<(&mut X, &mut Y)>() }.unwrap();
*x = X(1);
*y = Y(1);
// This would trigger undefined behavior, as the `&mut X`s would alias:
// entity.into_components_mut_unchecked::<(&mut X, &mut X)>();
```

##### Safety

It is the caller’s responsibility to ensure that the `QueryData` does not provide aliasing mutable references to the same component.

##### See also

*   [`Self::into_components_mut`](../../prelude/struct.EntityWorldMut.html#method.into_components_mut "method bevy::prelude::EntityWorldMut::into_components_mut") for the safe version that performs aliasing checks

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#468-470)

#### pub fn [into\_components\_mut](#method.into_components_mut)<Q>( self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Q as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 'static>, [QueryAccessError](../query/enum.QueryAccessError.html "enum bevy::ecs::query::QueryAccessError")\>

where Q: [ReleaseStateQueryData](../query/trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") + [SingleEntityQueryData](../query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

Consumes self and returns components for the current entity that match the query `Q` for the world lifetime `'w`, or `None` if the entity does not have the components required by the query `Q`.

The checks for aliasing mutable references may be expensive. If performance is a concern, consider making multiple calls to [`Self::get_mut`](../../prelude/struct.EntityWorldMut.html#method.get_mut "method bevy::prelude::EntityWorldMut::get_mut"). If that is not possible, consider using [`Self::into_components_mut_unchecked`](../../prelude/struct.EntityWorldMut.html#method.into_components_mut_unchecked "method bevy::prelude::EntityWorldMut::into_components_mut_unchecked") to skip the checks.

##### Panics

*   If the `QueryData` provides aliasing mutable references to the same component.
*   If the entity has been despawned while this `EntityWorldMut` is still alive.

##### Example

```rust
#[derive(Component)]
struct X(usize);
#[derive(Component)]
struct Y(usize);

let mut entity = world.spawn((X(0), Y(0)));
// Get mutable access to two components at once
let (mut x, mut y) = entity.into_components_mut::<(&mut X, &mut Y)>().unwrap();
*x = X(1);
*y = Y(1);
```

[ⓘ](# "This example panics")

```rust
let mut entity = world.spawn((X(0)));
// This panics, as the `&mut X`s would alias:
entity.into_components_mut::<(&mut X, &mut X)>();
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#484)

#### pub fn [into\_borrow](#method.into_borrow)<T>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&'w T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Consumes `self` and gets access to the component of type `T` with the world `'w` lifetime for the current entity. Returns `None` if the entity does not have a component of type `T`.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#497)

#### pub fn [get\_ref](#method.get_ref)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Gets access to the component of type `T` for the current entity, including change detection information as a [`Ref`](../../prelude/struct.Ref.html "struct bevy::prelude::Ref").

Returns `None` if the entity does not have a component of type `T`.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#511)

#### pub fn [into\_ref](#method.into_ref)<T>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'w, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Consumes `self` and gets access to the component of type `T` with the world `'w` lifetime for the current entity, including change detection information as a [`Ref`](../../prelude/struct.Ref.html "struct bevy::prelude::Ref").

Returns `None` if the entity does not have a component of type `T`.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#522)

#### pub fn [get\_mut](#method.get_mut)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets mutable access to the component of type `T` for the current entity. Returns `None` if the entity does not have a component of type `T`.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 35](../../../src/immutable_components/immutable_components.rs.html#35))

```rust
30fn demo_1(world: &mut World) {
31    // Immutable components can be inserted just like mutable components.
32    let mut entity = world.spawn((MyMutableComponent(false), MyImmutableComponent(false)));
33
34    // But where mutable components can be mutated...
35    let mut my_mutable_component = entity.get_mut::<MyMutableComponent>().unwrap();
36    my_mutable_component.0 = true;
37
38    // ...immutable ones cannot. The below fails to compile as `MyImmutableComponent`
39    // is declared as immutable.
40    // let mut my_immutable_component = entity.get_mut::<MyImmutableComponent>().unwrap();
41
42    // Instead, you could take or replace the immutable component to update its value.
43    let mut my_immutable_component = entity.take::<MyImmutableComponent>().unwrap();
44    my_immutable_component.0 = true;
45    entity.insert(my_immutable_component);
46}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#566)

#### pub fn [modify\_component](#method.modify_component)<T, R>( &mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<R>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Temporarily removes a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") `T` from this [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") and runs the provided closure on it, returning the result if `T` was available. This will trigger the `Remove` and `Discard` component hooks without causing an archetype move.

This is most useful with immutable components, where removal and reinsertion is the only way to modify a value.

If you do not need to ensure the above hooks are triggered, and your component is mutable, prefer using [`get_mut`](../../prelude/struct.EntityWorldMut.html#method.get_mut "method bevy::prelude::EntityWorldMut::get_mut").

##### Examples

```rust
#[derive(Component, PartialEq, Eq, Debug)]
#[component(immutable)]
struct Foo(bool);

entity.modify_component(|foo: &mut Foo| {
    foo.0 = true;
});
```

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#594-598)

#### pub fn [modify\_component\_by\_id](#method.modify_component_by_id)<R>( &mut self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), f: impl for<'a> [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'a>) -> R, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<R>

Temporarily removes a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") `T` from this [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") and runs the provided closure on it, returning the result if `T` was available. This will trigger the `Remove` and `Discard` component hooks without causing an archetype move.

This is most useful with immutable components, where removal and reinsertion is the only way to modify a value.

If you do not need to ensure the above hooks are triggered, and your component is mutable, prefer using [`get_mut`](../../prelude/struct.EntityWorldMut.html#method.get_mut "method bevy::prelude::EntityWorldMut::get_mut").

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#618)

#### pub unsafe fn [get\_mut\_assume\_mutable](#method.get_mut_assume_mutable)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Gets mutable access to the component of type `T` for the current entity. Returns `None` if the entity does not have a component of type `T`.

##### Safety

*   `T` must be a mutable component

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#630)

#### pub fn [into\_mut](#method.into_mut)<T>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Consumes `self` and gets mutable access to the component of type `T` with the world `'w` lifetime for the current entity. Returns `None` if the entity does not have a component of type `T`.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#647)

#### pub unsafe fn [into\_mut\_assume\_mutable](#method.into_mut_assume_mutable)<T>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Consumes `self` and gets mutable access to the component of type `T` with the world `'w` lifetime for the current entity. Returns `None` if the entity does not have a component of type `T`.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### Safety

*   `T` must be a mutable component

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#660)

#### pub fn [resource](#method.resource)<R>(&self) -> [&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference to the resource of the given type

##### Panics

Panics if the resource does not exist. Use [`get_resource`](../../prelude/struct.EntityWorldMut.html#method.get_resource "method bevy::prelude::EntityWorldMut::get_resource") instead if you want to handle this case.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#675)

#### pub fn [resource\_mut](#method.resource_mut)<R>(&mut self) -> [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets a mutable reference to the resource of the given type

##### Panics

Panics if the resource does not exist. Use [`get_resource_mut`](../../prelude/struct.World.html#method.get_resource_mut "method bevy::prelude::World::get_resource_mut") instead if you want to handle this case.

If you want to instead insert a value if the resource does not exist, use [`get_resource_or_insert_with`](../../prelude/struct.World.html#method.get_resource_or_insert_with "method bevy::prelude::World::get_resource_or_insert_with").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#681)

#### pub fn [get\_resource](#method.get_resource)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference to the resource of the given type if it exists

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#687)

#### pub fn [get\_resource\_mut](#method.get_resource_mut)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets a mutable reference to the resource of the given type if it exists

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#701-704)

#### pub fn [resource\_scope](#method.resource_scope)<R, U>( &mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>) -> U, ) -> U

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Temporarily removes the requested resource from the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), runs custom user code, then re-adds the resource before returning.

##### Panics

Panics if the resource does not exist. Use [`try_resource_scope`](../../prelude/struct.EntityWorldMut.html#method.try_resource_scope "method bevy::prelude::EntityWorldMut::try_resource_scope") instead if you want to handle this case.

See [`World::resource_scope`](../../prelude/struct.World.html#method.resource_scope "method bevy::prelude::World::resource_scope") for further details.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#720-723)

#### pub fn [try\_resource\_scope](#method.try_resource_scope)<R, U>( &mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>) -> U, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<U>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Temporarily removes the requested resource from the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") if it exists, runs custom user code, then re-adds the resource before returning. Returns `None` if the resource does not exist in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

See [`World::try_resource_scope`](../../prelude/struct.World.html#method.try_resource_scope "method bevy::prelude::World::try_resource_scope") for further details.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#738)

#### pub fn [resource\_entities](#method.resource_entities)(&self) -> &[ResourceEntities](../resource/struct.ResourceEntities.html "struct bevy::ecs::resource::ResourceEntities")

Retrieves this world’s [`ResourceEntities`](../resource/struct.ResourceEntities.html "struct bevy::ecs::resource::ResourceEntities").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#745)

#### pub fn [resource\_entity](#method.resource_entity)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Retrieves the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") associated with the resource of type `R`, if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#757)

#### pub fn [get\_change\_ticks](#method.get_change_ticks)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentTicks](../change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")\>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Retrieves the change ticks for the given component. This can be useful for implementing change detection in custom runtimes.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#769)

#### pub fn [get\_changed\_by](#method.get_changed_by)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MaybeLocation](../change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")\>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Get the [`MaybeLocation`](../change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation") from where the given [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") was last changed from. This contains information regarding the last place (in code) that changed this component and can be useful for debugging. For more information, see [`Location`](https://doc.rust-lang.org/nightly/core/panic/struct.Location.html), and enable the `track_location` feature.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#784)

#### pub fn [get\_change\_ticks\_by\_id](#method.get_change_ticks_by_id)( &self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentTicks](../change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")\>

Retrieves the change ticks for the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"). This can be useful for implementing change detection in custom runtimes.

**You should prefer to use the typed API [`EntityWorldMut::get_change_ticks`](../../prelude/struct.EntityWorldMut.html#method.get_change_ticks "method bevy::prelude::EntityWorldMut::get_change_ticks") where possible and only use this in cases where the actual component types are not known at compile time.**

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#812-815)

#### pub fn [get\_by\_id](#method.get_by_id)<F>( &self, component\_ids: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Ref](trait.DynamicComponentFetch.html#associatedtype.Ref "type bevy::ecs::world::DynamicComponentFetch::Ref")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

where F: [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch"),

Returns untyped read-only reference(s) to component(s) for the current entity, based on the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s.

**You should prefer to use the typed API [`EntityWorldMut::get`](../../prelude/struct.EntityWorldMut.html#method.get "method bevy::prelude::EntityWorldMut::get") where possible and only use this in cases where the actual component types are not known at compile time.**

Unlike [`EntityWorldMut::get`](../../prelude/struct.EntityWorldMut.html#method.get "method bevy::prelude::EntityWorldMut::get"), this returns untyped reference(s) to component(s), and it’s the job of the caller to ensure the correct type(s) are dereferenced (if necessary).

##### Errors

Returns [`EntityComponentError::MissingComponent`](error/enum.EntityComponentError.html#variant.MissingComponent "variant bevy::ecs::world::error::EntityComponentError::MissingComponent") if the entity does not have a component.

##### Examples

For examples on how to use this method, see [`EntityRef::get_by_id`](../../prelude/struct.EntityRef.html#method.get_by_id "method bevy::prelude::EntityRef::get_by_id").

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 189](../../../src/immutable_components/immutable_components.rs.html#189))

```rust
135fn demo_3(world: &mut World) {
136    // This is a list of dynamic components we will create.
137    // The first item is the name of the component, and the second is the size
138    // in bytes.
139    let my_dynamic_components = [("Foo", 1), ("Bar", 2), ("Baz", 4)];
140
141    // This pipeline takes our component descriptions, registers them, and gets
142    // their ComponentId's.
143    let my_registered_components = my_dynamic_components
144        .into_iter()
145        .map(|(name, size)| {
146            // SAFETY:
147            // - No drop command is required
148            // - The component will store [u8; size], which is Send + Sync
149            let descriptor = unsafe {
150                ComponentDescriptor::new_with_layout(
151                    name.to_string(),
152                    StorageType::Table,
153                    Layout::array::<u8>(size).unwrap(),
154                    None,
155                    false,
156                    ComponentCloneBehavior::Default,
157                    None,
158                )
159            };
160
161            (name, size, descriptor)
162        })
163        .map(|(name, size, descriptor)| {
164            let component_id = world.register_component_with_descriptor(descriptor);
165
166            (name, size, component_id)
167        })
168        .collect::<Vec<(&str, usize, ComponentId)>>();
169
170    // Now that our components are registered, let's add them to an entity
171    let mut entity = world.spawn_empty();
172
173    for (_name, size, component_id) in &my_registered_components {
174        // We're just storing some zeroes for the sake of demonstration.
175        let data = core::iter::repeat_n(0, *size).collect::<Vec<u8>>();
176
177        OwningPtr::make(data, |ptr| {
178            // SAFETY:
179            // - ComponentId has been taken from the same world
180            // - Array is created to the layout specified in the world
181            unsafe {
182                entity.insert_by_id(*component_id, ptr);
183            }
184        });
185    }
186
187    for (_name, _size, component_id) in &my_registered_components {
188        // With immutable components, we can read the values...
189        assert!(entity.get_by_id(*component_id).is_ok());
190
191        // ...but we cannot gain a mutable reference.
192        assert!(entity.get_mut_by_id(*component_id).is_err());
193
194        // Instead, you must either remove or replace the value.
195    }
196}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#844-847)

#### pub fn [into\_borrow\_by\_id](#method.into_borrow_by_id)<F>( self, component\_ids: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Ref](trait.DynamicComponentFetch.html#associatedtype.Ref "type bevy::ecs::world::DynamicComponentFetch::Ref")<'w>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

where F: [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch"),

Consumes `self` and returns untyped read-only reference(s) to component(s) with lifetime `'w` for the current entity, based on the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s.

**You should prefer to use the typed API [`EntityWorldMut::into_borrow`](../../prelude/struct.EntityWorldMut.html#method.into_borrow "method bevy::prelude::EntityWorldMut::into_borrow") where possible and only use this in cases where the actual component types are not known at compile time.**

Unlike [`EntityWorldMut::into_borrow`](../../prelude/struct.EntityWorldMut.html#method.into_borrow "method bevy::prelude::EntityWorldMut::into_borrow"), this returns untyped reference(s) to component(s), and it’s the job of the caller to ensure the correct type(s) are dereferenced (if necessary).

##### Errors

Returns [`EntityComponentError::MissingComponent`](error/enum.EntityComponentError.html#variant.MissingComponent "variant bevy::ecs::world::error::EntityComponentError::MissingComponent") if the entity does not have a component.

##### Examples

For examples on how to use this method, see [`EntityRef::get_by_id`](../../prelude/struct.EntityRef.html#method.get_by_id "method bevy::prelude::EntityRef::get_by_id").

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#877-880)

#### pub fn [get\_mut\_by\_id](#method.get_mut_by_id)<F>( &mut self, component\_ids: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

where F: [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch"),

Returns [untyped mutable reference(s)](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped") to component(s) for the current entity, based on the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s.

**You should prefer to use the typed API [`EntityWorldMut::get_mut`](../../prelude/struct.EntityWorldMut.html#method.get_mut "method bevy::prelude::EntityWorldMut::get_mut") where possible and only use this in cases where the actual component types are not known at compile time.**

Unlike [`EntityWorldMut::get_mut`](../../prelude/struct.EntityWorldMut.html#method.get_mut "method bevy::prelude::EntityWorldMut::get_mut"), this returns untyped reference(s) to component(s), and it’s the job of the caller to ensure the correct type(s) are dereferenced (if necessary).

##### Errors

*   Returns [`EntityComponentError::MissingComponent`](error/enum.EntityComponentError.html#variant.MissingComponent "variant bevy::ecs::world::error::EntityComponentError::MissingComponent") if the entity does not have a component.
*   Returns [`EntityComponentError::AliasedMutability`](error/enum.EntityComponentError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityComponentError::AliasedMutability") if a component is requested multiple times.

##### Examples

For examples on how to use this method, see [`EntityMut::get_mut_by_id`](../../prelude/struct.EntityMut.html#method.get_mut_by_id "method bevy::prelude::EntityMut::get_mut_by_id").

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 192](../../../src/immutable_components/immutable_components.rs.html#192))

```rust
135fn demo_3(world: &mut World) {
136    // This is a list of dynamic components we will create.
137    // The first item is the name of the component, and the second is the size
138    // in bytes.
139    let my_dynamic_components = [("Foo", 1), ("Bar", 2), ("Baz", 4)];
140
141    // This pipeline takes our component descriptions, registers them, and gets
142    // their ComponentId's.
143    let my_registered_components = my_dynamic_components
144        .into_iter()
145        .map(|(name, size)| {
146            // SAFETY:
147            // - No drop command is required
148            // - The component will store [u8; size], which is Send + Sync
149            let descriptor = unsafe {
150                ComponentDescriptor::new_with_layout(
151                    name.to_string(),
152                    StorageType::Table,
153                    Layout::array::<u8>(size).unwrap(),
154                    None,
155                    false,
156                    ComponentCloneBehavior::Default,
157                    None,
158                )
159            };
160
161            (name, size, descriptor)
162        })
163        .map(|(name, size, descriptor)| {
164            let component_id = world.register_component_with_descriptor(descriptor);
165
166            (name, size, component_id)
167        })
168        .collect::<Vec<(&str, usize, ComponentId)>>();
169
170    // Now that our components are registered, let's add them to an entity
171    let mut entity = world.spawn_empty();
172
173    for (_name, size, component_id) in &my_registered_components {
174        // We're just storing some zeroes for the sake of demonstration.
175        let data = core::iter::repeat_n(0, *size).collect::<Vec<u8>>();
176
177        OwningPtr::make(data, |ptr| {
178            // SAFETY:
179            // - ComponentId has been taken from the same world
180            // - Array is created to the layout specified in the world
181            unsafe {
182                entity.insert_by_id(*component_id, ptr);
183            }
184        });
185    }
186
187    for (_name, _size, component_id) in &my_registered_components {
188        // With immutable components, we can read the values...
189        assert!(entity.get_by_id(*component_id).is_ok());
190
191        // ...but we cannot gain a mutable reference.
192        assert!(entity.get_mut_by_id(*component_id).is_err());
193
194        // Instead, you must either remove or replace the value.
195    }
196}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#911-914)

#### pub unsafe fn [get\_mut\_assume\_mutable\_by\_id](#method.get_mut_assume_mutable_by_id)<F>( &mut self, component\_ids: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

where F: [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch"),

Returns [untyped mutable reference(s)](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped") to component(s) for the current entity, based on the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s. Assumes the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s refer to mutable components.

**You should prefer to use the typed API [`EntityWorldMut::get_mut_assume_mutable`](../../prelude/struct.EntityWorldMut.html#method.get_mut_assume_mutable "method bevy::prelude::EntityWorldMut::get_mut_assume_mutable") where possible and only use this in cases where the actual component types are not known at compile time.**

Unlike [`EntityWorldMut::get_mut_assume_mutable`](../../prelude/struct.EntityWorldMut.html#method.get_mut_assume_mutable "method bevy::prelude::EntityWorldMut::get_mut_assume_mutable"), this returns untyped reference(s) to component(s), and it’s the job of the caller to ensure the correct type(s) are dereferenced (if necessary).

##### Errors

*   Returns [`EntityComponentError::MissingComponent`](error/enum.EntityComponentError.html#variant.MissingComponent "variant bevy::ecs::world::error::EntityComponentError::MissingComponent") if the entity does not have a component.
*   Returns [`EntityComponentError::AliasedMutability`](error/enum.EntityComponentError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityComponentError::AliasedMutability") if a component is requested multiple times.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### Safety

It is the callers responsibility to ensure that

*   the provided [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s must refer to mutable components.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#949-952)

#### pub fn [into\_mut\_by\_id](#method.into_mut_by_id)<F>( self, component\_ids: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'w>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

where F: [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch"),

Consumes `self` and returns [untyped mutable reference(s)](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped") to component(s) with lifetime `'w` for the current entity, based on the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s.

**You should prefer to use the typed API [`EntityWorldMut::into_mut`](../../prelude/struct.EntityWorldMut.html#method.into_mut "method bevy::prelude::EntityWorldMut::into_mut") where possible and only use this in cases where the actual component types are not known at compile time.**

Unlike [`EntityWorldMut::into_mut`](../../prelude/struct.EntityWorldMut.html#method.into_mut "method bevy::prelude::EntityWorldMut::into_mut"), this returns untyped reference(s) to component(s), and it’s the job of the caller to ensure the correct type(s) are dereferenced (if necessary).

##### Errors

*   Returns [`EntityComponentError::MissingComponent`](error/enum.EntityComponentError.html#variant.MissingComponent "variant bevy::ecs::world::error::EntityComponentError::MissingComponent") if the entity does not have a component.
*   Returns [`EntityComponentError::AliasedMutability`](error/enum.EntityComponentError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityComponentError::AliasedMutability") if a component is requested multiple times.

##### Examples

For examples on how to use this method, see [`EntityMut::get_mut_by_id`](../../prelude/struct.EntityMut.html#method.get_mut_by_id "method bevy::prelude::EntityMut::get_mut_by_id").

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#984-987)

#### pub unsafe fn [into\_mut\_assume\_mutable\_by\_id](#method.into_mut_assume_mutable_by_id)<F>( self, component\_ids: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'w>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

where F: [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch"),

Consumes `self` and returns [untyped mutable reference(s)](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped") to component(s) with lifetime `'w` for the current entity, based on the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s. Assumes the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s refer to mutable components.

**You should prefer to use the typed API [`EntityWorldMut::into_mut_assume_mutable`](../../prelude/struct.EntityWorldMut.html#method.into_mut_assume_mutable "method bevy::prelude::EntityWorldMut::into_mut_assume_mutable") where possible and only use this in cases where the actual component types are not known at compile time.**

Unlike [`EntityWorldMut::into_mut_assume_mutable`](../../prelude/struct.EntityWorldMut.html#method.into_mut_assume_mutable "method bevy::prelude::EntityWorldMut::into_mut_assume_mutable"), this returns untyped reference(s) to component(s), and it’s the job of the caller to ensure the correct type(s) are dereferenced (if necessary).

##### Errors

*   Returns [`EntityComponentError::MissingComponent`](error/enum.EntityComponentError.html#variant.MissingComponent "variant bevy::ecs::world::error::EntityComponentError::MissingComponent") if the entity does not have a component.
*   Returns [`EntityComponentError::AliasedMutability`](error/enum.EntityComponentError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityComponentError::AliasedMutability") if a component is requested multiple times.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### Safety

It is the callers responsibility to ensure that

*   the provided [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s must refer to mutable components.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1003)

#### pub fn [insert](#method.insert)<T>(&mut self, bundle: T) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where T: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity.

This will overwrite any previous value(s) of the same component type.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/gltf/gltf\_extension\_mesh\_2d.rs ([line 114](../../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#114))

```rust
99    fn on_spawn_mesh_and_material(
100        &mut self,
101        load_context: &mut LoadContext<'_>,
102        _primitive: &gltf::Primitive,
103        _mesh: &gltf::Mesh,
104        _material: &gltf::Material,
105        entity: &mut EntityWorldMut,
106        _material_label: &str,
107    ) {
108        if let Some(mesh3d) = entity.get::<Mesh3d>() {
109            let material_handle =
110                load_context.add_labeled_asset("AColorMaterial".to_string(), CustomMaterial {});
111            let mesh_handle = mesh3d.0.clone();
112            entity
113                .remove::<Mesh3d>()
114                .insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle.clone())));
115        }
116    }
```

Hide additional examples

examples/ecs/immutable\_components.rs ([line 45](../../../src/immutable_components/immutable_components.rs.html#45))

```rust
30fn demo_1(world: &mut World) {
31    // Immutable components can be inserted just like mutable components.
32    let mut entity = world.spawn((MyMutableComponent(false), MyImmutableComponent(false)));
33
34    // But where mutable components can be mutated...
35    let mut my_mutable_component = entity.get_mut::<MyMutableComponent>().unwrap();
36    my_mutable_component.0 = true;
37
38    // ...immutable ones cannot. The below fails to compile as `MyImmutableComponent`
39    // is declared as immutable.
40    // let mut my_immutable_component = entity.get_mut::<MyImmutableComponent>().unwrap();
41
42    // Instead, you could take or replace the immutable component to update its value.
43    let mut my_immutable_component = entity.take::<MyImmutableComponent>().unwrap();
44    my_immutable_component.0 = true;
45    entity.insert(my_immutable_component);
46}
47
48/// This is an example of a component like [`Name`](bevy::prelude::Name), but immutable.
49#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Reflect)]
50#[reflect(Hash, Component)]
51#[component(
52    immutable,
53    // Since this component is immutable, we can fully capture all mutations through
54    // these component hooks. This allows for keeping other parts of the ECS synced
55    // to a component's value at all times.
56    on_insert = on_insert_name,
57    on_discard = on_discard_name,
58)]
59pub struct Name(pub &'static str);
60
61/// This index allows for O(1) lookups of an [`Entity`] by its [`Name`].
62#[derive(Resource, Default)]
63struct NameIndex {
64    name_to_entity: HashMap<Name, Entity>,
65}
66
67impl NameIndex {
68    fn get_entity(&self, name: &'static str) -> Option<Entity> {
69        self.name_to_entity.get(&Name(name)).copied()
70    }
71}
72
73/// When a [`Name`] is inserted, we will add it to our [`NameIndex`].
74///
75/// Since all mutations to [`Name`] are captured by hooks, we know it is not currently
76/// inserted in the index, and its value will not change without triggering a hook.
77fn on_insert_name(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
78    let Some(&name) = world.entity(entity).get::<Name>() else {
79        unreachable!("Insert hook guarantees `Name` is available on entity")
80    };
81    let Some(mut index) = world.get_resource_mut::<NameIndex>() else {
82        return;
83    };
84
85    index.name_to_entity.insert(name, entity);
86}
87
88/// When a [`Name`] is removed or replaced, remove it from our [`NameIndex`].
89///
90/// Since all mutations to [`Name`] are captured by hooks, we know it is currently
91/// inserted in the index.
92fn on_discard_name(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
93    let Some(&name) = world.entity(entity).get::<Name>() else {
94        unreachable!("Discard hook guarantees `Name` is available on entity")
95    };
96    let Some(mut index) = world.get_resource_mut::<NameIndex>() else {
97        return;
98    };
99
100    index.name_to_entity.remove(&name);
101}
102
103fn demo_2(world: &mut World) {
104    // Setup our name index
105    world.init_resource::<NameIndex>();
106
107    // Spawn some entities!
108    let alyssa = world.spawn(Name("Alyssa")).id();
109    let javier = world.spawn(Name("Javier")).id();
110
111    // Check our index
112    let index = world.resource::<NameIndex>();
113
114    assert_eq!(index.get_entity("Alyssa"), Some(alyssa));
115    assert_eq!(index.get_entity("Javier"), Some(javier));
116
117    // Changing the name of an entity is also fully capture by our index
118    world.entity_mut(javier).insert(Name("Steven"));
119
120    // Javier changed their name to Steven
121    let steven = javier;
122
123    // Check our index
124    let index = world.resource::<NameIndex>();
125
126    assert_eq!(index.get_entity("Javier"), None);
127    assert_eq!(index.get_entity("Steven"), Some(steven));
128}
```

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 279](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#279))

```rust
257    fn on_scene_completed(
258        &mut self,
259        load_context: &mut LoadContext<'_>,
260        _scene: &gltf::Scene,
261        _world_root_id: Entity,
262        world: &mut World,
263    ) {
264        // Create an AnimationGraph from the desired clip
265        let (graph, index) = AnimationGraph::from_clip(self.clip.clone().unwrap());
266        // Store the animation graph as an asset with an arbitrary label
267        // We only have one graph, so this label will be unique
268        let graph_handle =
269            load_context.add_labeled_asset("MyAnimationGraphLabel".to_string(), graph);
270
271        // Create a component that stores a reference to our animation
272        let animation_to_play = AnimationToPlay {
273            graph_handle,
274            index,
275        };
276
277        // Insert the `AnimationToPlay` component on the first animation root
278        let mut entity = world.entity_mut(*self.animation_root_entities.iter().next().unwrap());
279        entity.insert(animation_to_play);
280    }
```

examples/async\_tasks/async\_compute.rs ([lines 102-106](../../../src/async_compute/async_compute.rs.html#102-106))

```rust
66fn spawn_tasks(mut commands: Commands) {
67    let thread_pool = AsyncComputeTaskPool::get();
68    for x in 0..NUM_CUBES {
69        for y in 0..NUM_CUBES {
70            for z in 0..NUM_CUBES {
71                // Spawn new task on the AsyncComputeTaskPool; the task will be
72                // executed in the background, and the Task future returned by
73                // spawn() can be used to poll for the result
74                let entity = commands.spawn_empty().id();
75                let task = thread_pool.spawn(async move {
76                    let duration = Duration::from_secs_f32(rand::rng().random_range(0.05..5.0));
77
78                    // Pretend this is a time-intensive function. :)
79                    Delay::new(duration).await;
80
81                    // Such hard work, all done!
82                    let transform = Transform::from_xyz(x as f32, y as f32, z as f32);
83                    let mut command_queue = CommandQueue::default();
84
85                    // we use a raw command queue to pass a FnOnce(&mut World) back to be
86                    // applied in a deferred manner.
87                    command_queue.push(move |world: &mut World| {
88                        let (box_mesh_handle, box_material_handle) = {
89                            let mut system_state = SystemState::<(
90                                Res<BoxMeshHandle>,
91                                Res<BoxMaterialHandle>,
92                            )>::new(world);
93                            let (box_mesh_handle, box_material_handle) =
94                                system_state.get_mut(world).unwrap();
95
96                            (box_mesh_handle.clone(), box_material_handle.clone())
97                        };
98
99                        world
100                            .entity_mut(entity)
101                            // Add our new `Mesh3d` and `MeshMaterial3d` to our tagged entity
102                            .insert((
103                                Mesh3d(box_mesh_handle),
104                                MeshMaterial3d(box_material_handle),
105                                transform,
106                            ));
107                    });
108
109                    command_queue
110                });
111
112                // Add our new task as a component
113                commands.entity(entity).insert(ComputeTransform(task));
114            }
115        }
116    }
117}
```

examples/showcase/game\_menu.rs ([line 599](../../../src/game_menu/game_menu.rs.html#599))

```rust
527    fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
528        fn button_node() -> Node {
529            Node {
530                width: px(200),
531                height: px(65),
532                margin: UiRect::all(px(20)),
533                justify_content: JustifyContent::Center,
534                align_items: AlignItems::Center,
535                ..default()
536            }
537        }
538        fn button_text_style() -> impl Bundle {
539            (
540                TextFont {
541                    font_size: FontSize::Px(33.0),
542                    ..default()
543                },
544                TextColor(TEXT_COLOR),
545            )
546        }
547
548        let display_quality = *display_quality;
549        commands.spawn((
550            DespawnOnExit(MenuState::SettingsDisplay),
551            Node {
552                width: percent(100),
553                height: percent(100),
554                align_items: AlignItems::Center,
555                justify_content: JustifyContent::Center,
556                ..default()
557            },
558            OnDisplaySettingsMenuScreen,
559            children![(
560                Node {
561                    flex_direction: FlexDirection::Column,
562                    align_items: AlignItems::Center,
563                    ..default()
564                },
565                BackgroundColor(CRIMSON.into()),
566                children![
567                    // Create a new `Node`, this time not setting its `flex_direction`. It will
568                    // use the default value, `FlexDirection::Row`, from left to right.
569                    (
570                        Node {
571                            align_items: AlignItems::Center,
572                            ..default()
573                        },
574                        BackgroundColor(CRIMSON.into()),
575                        Children::spawn((
576                            // Display a label for the current setting
577                            Spawn((Text::new("Display Quality"), button_text_style())),
578                            SpawnWith(move |parent: &mut ChildSpawner| {
579                                for quality_setting in [
580                                    DisplayQuality::Low,
581                                    DisplayQuality::Medium,
582                                    DisplayQuality::High,
583                                ] {
584                                    let mut entity = parent.spawn((
585                                        Button,
586                                        Node {
587                                            width: px(150),
588                                            height: px(65),
589                                            ..button_node()
590                                        },
591                                        BackgroundColor(NORMAL_BUTTON),
592                                        Setting(quality_setting),
593                                        children![(
594                                            Text::new(format!("{quality_setting:?}")),
595                                            button_text_style(),
596                                        )],
597                                    ));
598                                    if display_quality == quality_setting {
599                                        entity.insert(SelectedOption);
600                                    }
601                                }
602                            })
603                        ))
604                    ),
605                    // Display the back button to return to the settings screen
606                    (
607                        Button,
608                        button_node(),
609                        BackgroundColor(NORMAL_BUTTON),
610                        MenuButtonAction::BackToSettings,
611                        children![(Text::new("Back"), button_text_style())]
612                    )
613                ]
614            )],
615        ));
616    }
617
618    fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
619        let button_node = Node {
620            width: px(200),
621            height: px(65),
622            margin: UiRect::all(px(20)),
623            justify_content: JustifyContent::Center,
624            align_items: AlignItems::Center,
625            ..default()
626        };
627        let button_text_style = (
628            TextFont {
629                font_size: FontSize::Px(33.0),
630                ..default()
631            },
632            TextColor(TEXT_COLOR),
633        );
634
635        let volume = *volume;
636        let button_node_clone = button_node.clone();
637        commands.spawn((
638            DespawnOnExit(MenuState::SettingsSound),
639            Node {
640                width: percent(100),
641                height: percent(100),
642                align_items: AlignItems::Center,
643                justify_content: JustifyContent::Center,
644                ..default()
645            },
646            OnSoundSettingsMenuScreen,
647            children![(
648                Node {
649                    flex_direction: FlexDirection::Column,
650                    align_items: AlignItems::Center,
651                    ..default()
652                },
653                BackgroundColor(CRIMSON.into()),
654                children![
655                    (
656                        Node {
657                            align_items: AlignItems::Center,
658                            ..default()
659                        },
660                        BackgroundColor(CRIMSON.into()),
661                        Children::spawn((
662                            Spawn((Text::new("Volume"), button_text_style.clone())),
663                            SpawnWith(move |parent: &mut ChildSpawner| {
664                                for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
665                                    let mut entity = parent.spawn((
666                                        Button,
667                                        Node {
668                                            width: px(30),
669                                            height: px(65),
670                                            ..button_node_clone.clone()
671                                        },
672                                        BackgroundColor(NORMAL_BUTTON),
673                                        Setting(Volume(volume_setting)),
674                                    ));
675                                    if volume == Volume(volume_setting) {
676                                        entity.insert(SelectedOption);
677                                    }
678                                }
679                            })
680                        ))
681                    ),
682                    (
683                        Button,
684                        button_node,
685                        BackgroundColor(NORMAL_BUTTON),
686                        MenuButtonAction::BackToSettings,
687                        children![(Text::new("Back"), button_text_style)]
688                    )
689                ]
690            )],
691        ));
692    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1028-1032)

#### pub fn [insert\_with\_relationship\_hook\_mode](#method.insert_with_relationship_hook_mode)<T>( &mut self, bundle: T, relationship\_hook\_mode: [RelationshipHookMode](../relationship/enum.RelationshipHookMode.html "enum bevy::ecs::relationship::RelationshipHookMode"), ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where T: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity. [`Relationship`](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship") components in the bundle will follow the configuration in `relationship_hook_mode`.

This will overwrite any previous value(s) of the same component type.

##### Warning

This can easily break the integrity of relationships. This is intended to be used for cloning and spawning code internals, not most user-facing scenarios.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1051)

#### pub fn [insert\_if\_new](#method.insert_if_new)<T>(&mut self, bundle: T) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where T: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Adds a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of components to the entity without overwriting.

This will leave any previous value(s) of the same component type unchanged.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1119-1123)

#### pub unsafe fn [insert\_by\_id](#method.insert_by_id)( &mut self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), component: [OwningPtr](../ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Inserts a dynamic [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") into the entity.

This will overwrite any previous value(s) of the same component type.

You should prefer to use the typed API [`EntityWorldMut::insert`](../../prelude/struct.EntityWorldMut.html#method.insert "method bevy::prelude::EntityWorldMut::insert") where possible.

##### Safety

*   [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") must be from the same world as [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")
*   [`OwningPtr`](../ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr") must be a valid reference to the type represented by [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 182](../../../src/immutable_components/immutable_components.rs.html#182))

```rust
135fn demo_3(world: &mut World) {
136    // This is a list of dynamic components we will create.
137    // The first item is the name of the component, and the second is the size
138    // in bytes.
139    let my_dynamic_components = [("Foo", 1), ("Bar", 2), ("Baz", 4)];
140
141    // This pipeline takes our component descriptions, registers them, and gets
142    // their ComponentId's.
143    let my_registered_components = my_dynamic_components
144        .into_iter()
145        .map(|(name, size)| {
146            // SAFETY:
147            // - No drop command is required
148            // - The component will store [u8; size], which is Send + Sync
149            let descriptor = unsafe {
150                ComponentDescriptor::new_with_layout(
151                    name.to_string(),
152                    StorageType::Table,
153                    Layout::array::<u8>(size).unwrap(),
154                    None,
155                    false,
156                    ComponentCloneBehavior::Default,
157                    None,
158                )
159            };
160
161            (name, size, descriptor)
162        })
163        .map(|(name, size, descriptor)| {
164            let component_id = world.register_component_with_descriptor(descriptor);
165
166            (name, size, component_id)
167        })
168        .collect::<Vec<(&str, usize, ComponentId)>>();
169
170    // Now that our components are registered, let's add them to an entity
171    let mut entity = world.spawn_empty();
172
173    for (_name, size, component_id) in &my_registered_components {
174        // We're just storing some zeroes for the sake of demonstration.
175        let data = core::iter::repeat_n(0, *size).collect::<Vec<u8>>();
176
177        OwningPtr::make(data, |ptr| {
178            // SAFETY:
179            // - ComponentId has been taken from the same world
180            // - Array is created to the layout specified in the world
181            unsafe {
182                entity.insert_by_id(*component_id, ptr);
183            }
184        });
185    }
186
187    for (_name, _size, component_id) in &my_registered_components {
188        // With immutable components, we can read the values...
189        assert!(entity.get_by_id(*component_id).is_ok());
190
191        // ...but we cannot gain a mutable reference.
192        assert!(entity.get_mut_by_id(*component_id).is_err());
193
194        // Instead, you must either remove or replace the value.
195    }
196}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1193-1197)

#### pub unsafe fn [insert\_by\_ids](#method.insert_by_ids)<'a, I>( &mut self, component\_ids: &\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\], iter\_components: I, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [OwningPtr](../ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'a>>,

Inserts a dynamic [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") into the entity.

This will overwrite any previous value(s) of the same component type.

You should prefer to use the typed API [`EntityWorldMut::insert`](../../prelude/struct.EntityWorldMut.html#method.insert "method bevy::prelude::EntityWorldMut::insert") where possible. If your [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") only has one component, use the cached API [`EntityWorldMut::insert_by_id`](../../prelude/struct.EntityWorldMut.html#method.insert_by_id "method bevy::prelude::EntityWorldMut::insert_by_id").

If possible, pass a sorted slice of `ComponentId` to maximize caching potential.

##### Safety

*   Each [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") must be from the same world as [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")
*   Each [`OwningPtr`](../ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr") must be a valid reference to the type represented by [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_components.rs ([line 156](../../../src/many_components/many_components.rs.html#156))

```rust
78fn stress_test(num_entities: u32, num_components: u32, num_systems: u32) {
79    let mut rng = ChaCha8Rng::seed_from_u64(42);
80    let mut app = App::default();
81    let world = app.world_mut();
82
83    // register a bunch of components
84    let component_ids: Vec<ComponentId> = (1..=num_components)
85        .map(|i| {
86            world.register_component_with_descriptor(
87                // SAFETY:
88                // * We don't implement a drop function
89                // * u8 is Sync and Send
90                unsafe {
91                    ComponentDescriptor::new_with_layout(
92                        format!("Component{i}").to_string(),
93                        StorageType::Table,
94                        Layout::new::<u8>(),
95                        None,
96                        true, // is mutable
97                        ComponentCloneBehavior::Default,
98                        None,
99                    )
100                },
101            )
102        })
103        .collect();
104
105    // fill the schedule with systems
106    let mut schedule = Schedule::new(Update);
107    for _ in 1..=num_systems {
108        let num_access_components = rng.random_range(1..10);
109        let access_components: Vec<ComponentId> = component_ids
110            .sample(&mut rng, num_access_components)
111            .copied()
112            .collect();
113        let system = (QueryParamBuilder::new(|builder| {
114            for &access_component in &access_components {
115                if rand::random::<bool>() {
116                    builder.mut_id(access_component);
117                } else {
118                    builder.ref_id(access_component);
119                }
120            }
121        }),)
122            .build_state(world)
123            .build_any_system(base_system);
124        schedule.add_systems((move || access_components.clone()).pipe(system));
125    }
126
127    // spawn a bunch of entities
128    for _ in 1..=num_entities {
129        let num_components = rng.random_range(1..10);
130        let components: Vec<ComponentId> = component_ids
131            .sample(&mut rng, num_components)
132            .copied()
133            .collect();
134
135        let mut entity = world.spawn_empty();
136        // We use `ManuallyDrop` here as we need to avoid dropping the u8's when `values` is dropped
137        // since ownership of the values is passed to the world in `insert_by_ids`.
138        // But we do want to deallocate the memory when values is dropped.
139        let mut values: Vec<ManuallyDrop<u8>> = components
140            .iter()
141            .map(|_id| ManuallyDrop::new(rng.random_range(0..255)))
142            .collect();
143        let ptrs: Vec<OwningPtr> = values
144            .iter_mut()
145            .map(|value| {
146                // SAFETY:
147                // * We don't read/write `values` binding after this and values are `ManuallyDrop`,
148                // so we have the right to drop/move the values
149                unsafe { PtrMut::from(value).promote() }
150            })
151            .collect();
152        // SAFETY:
153        // * component_id's are from the same world
154        // * `values` was initialized above, so references are valid
155        unsafe {
156            entity.insert_by_ids(&components, ptrs.into_iter());
157        }
158    }
159
160    // overwrite Update schedule in the app
161    app.add_schedule(schedule);
162    app.add_plugins(MinimalPlugins)
163        .add_plugins(DiagnosticsPlugin)
164        .add_plugins(LogPlugin::default())
165        .add_plugins(FrameTimeDiagnosticsPlugin::default())
166        .add_plugins(LogDiagnosticsPlugin::filtered(HashSet::from_iter([
167            DiagnosticPath::new("fps"),
168        ])));
169    app.run();
170}
```

Hide additional examples

examples/ecs/dynamic.rs ([line 170](../../../src/dynamic/dynamic.rs.html#170))

```rust
69fn main() {
70    let mut world = World::new();
71    let mut lines = std::io::stdin().lines();
72    let mut component_names = HashMap::<String, ComponentId>::new();
73    let mut component_info = HashMap::<ComponentId, ComponentInfo>::new();
74    let mut event_names = HashMap::<String, EventKey>::new();
75
76    println!("{PROMPT}");
77    loop {
78        print!("\n> ");
79        let _ = std::io::stdout().flush();
80        let Some(Ok(line)) = lines.next() else {
81            return;
82        };
83
84        if line.is_empty() {
85            return;
86        };
87
88        let Some((first, rest)) = line.trim().split_once(|c: char| c.is_whitespace()) else {
89            match &line.chars().next() {
90                Some('c') => println!("{COMPONENT_PROMPT}"),
91                Some('s') => println!("{ENTITY_PROMPT}"),
92                Some('q') => println!("{QUERY_PROMPT}"),
93                Some('e') => println!("{EVENT_PROMPT}"),
94                Some('t') => println!("{EMIT_PROMPT}"),
95                _ => println!("{PROMPT}"),
96            }
97            continue;
98        };
99
100        match &first[0..1] {
101            "c" => {
102                rest.split(',').for_each(|component| {
103                    let mut component = component.split_whitespace();
104                    let Some(name) = component.next() else {
105                        return;
106                    };
107                    let size = match component.next().map(str::parse) {
108                        Some(Ok(size)) => size,
109                        _ => 0,
110                    };
111                    // Register our new component to the world with a layout specified by it's size
112                    // SAFETY: [u64] is Send + Sync
113                    let id = world.register_component_with_descriptor(unsafe {
114                        ComponentDescriptor::new_with_layout(
115                            name.to_string(),
116                            StorageType::Table,
117                            Layout::array::<u64>(size).unwrap(),
118                            None,
119                            true,
120                            ComponentCloneBehavior::Default,
121                            None,
122                        )
123                    });
124                    let Some(info) = world.components().get_info(id) else {
125                        return;
126                    };
127                    component_names.insert(name.to_string(), id);
128                    component_info.insert(id, info.clone());
129                    println!("Component {} created with id: {}", name, id.index());
130                });
131            }
132            "s" => {
133                let mut to_insert_ids = Vec::new();
134                let mut to_insert_data = Vec::new();
135                rest.split(',').for_each(|component| {
136                    let mut component = component.split_whitespace();
137                    let Some(name) = component.next() else {
138                        return;
139                    };
140
141                    // Get the id for the component with the given name
142                    let Some(&id) = component_names.get(name) else {
143                        println!("Component {name} does not exist");
144                        return;
145                    };
146
147                    // Calculate the length for the array based on the layout created for this component id
148                    let info = world.components().get_info(id).unwrap();
149                    let len = info.layout().size() / size_of::<u64>();
150                    let mut values: Vec<u64> = component
151                        .take(len)
152                        .filter_map(|value| value.parse::<u64>().ok())
153                        .collect();
154                    values.resize(len, 0);
155
156                    // Collect the id and array to be inserted onto our entity
157                    to_insert_ids.push(id);
158                    to_insert_data.push(values);
159                });
160
161                let mut entity = world.spawn_empty();
162
163                // Construct an `OwningPtr` for each component in `to_insert_data`
164                let to_insert_ptr = to_owning_ptrs(&mut to_insert_data);
165
166                // SAFETY:
167                // - Component ids have been taken from the same world
168                // - Each array is created to the layout specified in the world
169                unsafe {
170                    entity.insert_by_ids(&to_insert_ids, to_insert_ptr.into_iter());
171                }
172
173                println!("Entity spawned with id: {}", entity.id());
174            }
175            "q" => {
176                let mut builder = QueryBuilder::<FilteredEntityMut>::new(&mut world);
177                parse_query(rest, &mut builder, &component_names);
178                let mut query = builder.build();
179                query.iter_mut(&mut world).for_each(|filtered_entity| {
180                    let terms = filtered_entity
181                        .access()
182                        .try_iter_access()
183                        .unwrap()
184                        .map(|component_access| {
185                            let id = *component_access.index();
186                            let ptr = filtered_entity.get_by_id(id).unwrap();
187                            let info = component_info.get(&id).unwrap();
188                            let len = info.layout().size() / size_of::<u64>();
189
190                            // SAFETY:
191                            // - All components are created with layout [u64]
192                            // - len is calculated from the component descriptor
193                            let data = unsafe {
194                                std::slice::from_raw_parts_mut(
195                                    ptr.assert_unique().as_ptr().cast::<u64>(),
196                                    len,
197                                )
198                            };
199
200                            // If we have write access, increment each value once
201                            if matches!(component_access, ComponentAccessKind::Exclusive(_)) {
202                                data.iter_mut().for_each(|data| {
203                                    *data += 1;
204                                });
205                            }
206
207                            format!("{}: {:?}", info.name(), data[0..len].to_vec())
208                        })
209                        .collect::<Vec<_>>()
210                        .join(", ");
211
212                    println!("{}: {}", filtered_entity.id(), terms);
213                });
214            }
215            "e" => {
216                rest.split(',').for_each(|event| {
217                    let name = event.trim();
218                    if name.is_empty() {
219                        return;
220                    }
221
222                    // Register a ComponentId for this event, no Rust type needed.
223                    // SAFETY: ZST with no drop
224                    let event_component_id = world.register_component_with_descriptor(unsafe {
225                        ComponentDescriptor::new_with_layout(
226                            format!("event:{name}"),
227                            StorageType::Table,
228                            Layout::new::<()>(),
229                            None,
230                            false,
231                            ComponentCloneBehavior::Ignore,
232                            None,
233                        )
234                    });
235                    // SAFETY: event_component_id was just registered for this event
236                    let event_key = unsafe { EventKey::new(event_component_id) };
237                    event_names.insert(name.to_string(), event_key);
238
239                    // Build a dynamic observer that prints when the event fires.
240                    let runner: ObserverRunner = |mut world, _observer, ctx, _event, _trigger| {
241                        println!("  Observer fired!");
242                        if let Some(mut counts) = world.get_resource_mut::<EventFireCount>() {
243                            *counts.0.entry(ctx.event_key).or_insert(0) += 1;
244                        }
245                    };
246
247                    // SAFETY: event_key was just registered, runner ignores pointers
248                    let observer =
249                        unsafe { Observer::with_dynamic_runner(runner).with_event_key(event_key) };
250                    world.spawn(observer);
251
252                    println!(
253                        "Event '{name}' registered (key: {}) with a dynamic observer",
254                        event_component_id.index()
255                    );
256                });
257
258                // Ensure the counter resource exists.
259                world.init_resource::<EventFireCount>();
260            }
261            "t" => {
262                let name = rest.trim();
263                let Some(&event_key) = event_names.get(name) else {
264                    println!(
265                        "Event '{name}' does not exist. Register it first with 'event {name}'"
266                    );
267                    continue;
268                };
269
270                let mut event_data = ();
271                let mut trigger_data = ();
272                // SAFETY: event_key was registered in this world, both pointers are valid ZSTs
273                unsafe {
274                    world.trigger_dynamic(
275                        event_key,
276                        PtrMut::from(&mut event_data),
277                        PtrMut::from(&mut trigger_data),
278                    );
279                }
280
281                let count = world
282                    .get_resource::<EventFireCount>()
283                    .map_or(0, |c| c.0.get(&event_key).copied().unwrap_or(0));
284                println!("Event '{name}' triggered ({count} fires)");
285            }
286            _ => continue,
287        }
288    }
289}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1246)

#### pub fn [take](#method.take)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") + [BundleFromComponents](../bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents"),

Removes all components in the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") from the entity and returns their previous values.

**Note:** If the entity does not have every component in the bundle, this method will not remove any of them.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 43](../../../src/immutable_components/immutable_components.rs.html#43))

```rust
30fn demo_1(world: &mut World) {
31    // Immutable components can be inserted just like mutable components.
32    let mut entity = world.spawn((MyMutableComponent(false), MyImmutableComponent(false)));
33
34    // But where mutable components can be mutated...
35    let mut my_mutable_component = entity.get_mut::<MyMutableComponent>().unwrap();
36    my_mutable_component.0 = true;
37
38    // ...immutable ones cannot. The below fails to compile as `MyImmutableComponent`
39    // is declared as immutable.
40    // let mut my_immutable_component = entity.get_mut::<MyImmutableComponent>().unwrap();
41
42    // Instead, you could take or replace the immutable component to update its value.
43    let mut my_immutable_component = entity.take::<MyImmutableComponent>().unwrap();
44    my_immutable_component.0 = true;
45    entity.insert(my_immutable_component);
46}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1304)

#### pub fn [remove](#method.remove)<T>(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where T: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes any components in the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") from the entity.

See [`EntityCommands::remove`](../../prelude/struct.EntityCommands.html#method.remove "method bevy::prelude::EntityCommands::remove") for more details.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/gltf/gltf\_extension\_mesh\_2d.rs ([line 113](../../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#113))

```rust
99    fn on_spawn_mesh_and_material(
100        &mut self,
101        load_context: &mut LoadContext<'_>,
102        _primitive: &gltf::Primitive,
103        _mesh: &gltf::Mesh,
104        _material: &gltf::Material,
105        entity: &mut EntityWorldMut,
106        _material_label: &str,
107    ) {
108        if let Some(mesh3d) = entity.get::<Mesh3d>() {
109            let material_handle =
110                load_context.add_labeled_asset("AColorMaterial".to_string(), CustomMaterial {});
111            let mesh_handle = mesh3d.0.clone();
112            entity
113                .remove::<Mesh3d>()
114                .insert((Mesh2d(mesh_handle), MeshMaterial2d(material_handle.clone())));
115        }
116    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1343)

#### pub fn [remove\_with\_requires](#method.remove_with_requires)<T>(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where T: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes all components in the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") and remove all required components for each component in the bundle

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1387)

#### pub fn [retain](#method.retain)<T>(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where T: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Removes any components except those in the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") (and its Required Components) from the entity.

See [`EntityCommands::retain`](../../prelude/struct.EntityCommands.html#method.retain "method bevy::prelude::EntityCommands::retain") for more details.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1446)

#### pub fn [remove\_by\_id](#method.remove_by_id)( &mut self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Removes a dynamic [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") from the entity if it exists.

You should prefer to use the typed API [`EntityWorldMut::remove`](../../prelude/struct.EntityWorldMut.html#method.remove "method bevy::prelude::EntityWorldMut::remove") where possible.

##### Panics

Panics if the provided [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") does not exist in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") or if the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1499)

#### pub fn [remove\_by\_ids](#method.remove_by_ids)( &mut self, component\_ids: &\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\], ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Removes a dynamic bundle from the entity if it exists.

You should prefer to use the typed API [`EntityWorldMut::remove`](../../prelude/struct.EntityWorldMut.html#method.remove "method bevy::prelude::EntityWorldMut::remove") where possible.

##### Panics

Panics if any of the provided [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s do not exist in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") or if the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1554)

#### pub fn [clear](#method.clear)(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Removes all components associated with the entity.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1608)

#### pub fn [despawn\_no\_free](#method.despawn_no_free)(self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Despawns the entity without freeing it to the allocator. This returns the new [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), which you must manage. Note that this still increases the generation to differentiate different spawns of the same row.

Additionally, keep in mind the limitations documented in the type-level docs. Unless you have full knowledge of this [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")’s lifetime, you may not assume that nothing else has taken responsibility of this [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"). If you are not careful, this could cause a double free.

This may be later [`spawn_at`](../../prelude/struct.World.html#method.spawn_at "method bevy::prelude::World::spawn_at"). See [`World::despawn_no_free`](../../prelude/struct.World.html#method.despawn_no_free "method bevy::prelude::World::despawn_no_free") for details and usage examples.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1614-1617)

#### pub fn [template\_context](#method.template_context)<T>( &mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [TemplateContext](../template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Creates a new [`TemplateContext`](../template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext") for this entity and passes it into the given `func`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1624)

#### pub fn [build\_template](#method.build_template)<T>( &mut self, template: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template"),

Builds the given template using a [`TemplateContext`](../template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext") generated for this entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1805)

#### pub fn [despawn](#method.despawn)(self)

Despawns the current entity.

See [`World::despawn`](../../prelude/struct.World.html#method.despawn "method bevy::prelude::World::despawn") for more details.

##### Note

This will also despawn any [`Children`](../../prelude/struct.Children.html "struct bevy::prelude::Children") entities, and any other [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") that is configured to despawn descendants. This results in “recursive despawn” behavior.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1821)

#### pub fn [flush](#method.flush)(self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Ensures any commands triggered by the actions of Self are applied, equivalent to [`World::flush`](../../prelude/struct.World.html#method.flush "method bevy::prelude::World::flush")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1828)

#### pub fn [world](#method.world)(&self) -> &[World](../../prelude/struct.World.html "struct bevy::prelude::World")

Gets read-only access to the world that the current entity belongs to.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1841)

#### pub unsafe fn [world\_mut](#method.world_mut)(&mut self) -> &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")

Returns this entity’s world.

See [`EntityWorldMut::world_scope`](../../prelude/struct.EntityWorldMut.html#method.world_scope "method bevy::prelude::EntityWorldMut::world_scope") or [`EntityWorldMut::into_world_mut`](../../prelude/struct.EntityWorldMut.html#method.into_world_mut "method bevy::prelude::EntityWorldMut::into_world_mut") for a safe alternative.

##### Safety

Caller must not modify the world in a way that changes the current entity’s location If the caller _does_ do something that could change the location, `self.update_location()` must be called before using any other methods on this [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1847)

#### pub fn [into\_world\_mut](#method.into_world_mut)(self) -> &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")

Returns this entity’s [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), consuming itself.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1875)

#### pub fn [world\_scope](#method.world_scope)<U>(&mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> U) -> U

Gives mutable access to this entity’s [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") in a temporary scope. This is a safe alternative to using [`EntityWorldMut::world_mut`](../../prelude/struct.EntityWorldMut.html#method.world_mut "method bevy::prelude::EntityWorldMut::world_mut").

##### Examples

```rust
#[derive(Resource, Default, Clone, Copy)]
struct R(u32);

// This closure gives us temporary access to the world.
let new_r = entity.world_scope(|world: &mut World| {
    // Mutate the world while we have access to it.
    let mut r = world.resource_mut::<R>();
    r.0 += 1;

    // Return a value from the world before giving it back to the `EntityWorldMut`.
    *r
});
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1902)

#### pub fn [update\_location](#method.update_location)(&mut self)

Updates the internal entity location to match the current location in the internal [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

This is _only_ required when using the unsafe function [`EntityWorldMut::world_mut`](../../prelude/struct.EntityWorldMut.html#method.world_mut "method bevy::prelude::EntityWorldMut::world_mut"), which enables the location to change.

Note that if the entity is not spawned for any reason, this will have a location of `None`, leading some methods to panic.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1914)

#### pub fn [is\_despawned](#method.is_despawned)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns if the entity has been despawned.

Normally it shouldn’t be needed to explicitly check if the entity has been despawned between commands as this shouldn’t happen. However, for some special cases where it is known that a hook or an observer might despawn the entity while a [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") reference is still held, this method can be used to check if the entity is still alive to avoid panicking when calling further methods.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1943)

#### pub fn [entry](#method.entry)<'a, T>(&'a mut self) -> [ComponentEntry](enum.ComponentEntry.html "enum bevy::ecs::world::ComponentEntry")<'w, 'a, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Gets an Entry into the world for this entity and component for in-place manipulation.

The type parameter specifies which component to get.

##### Examples

```rust
#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
struct Comp(u32);

let mut entity = world.spawn_empty();
entity.entry().or_insert_with(|| Comp(4));
assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 4);

entity.entry::<Comp>().and_modify(|mut c| c.0 += 1);
assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 5);
```

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#1966)

#### pub fn [observe](#method.observe)<M>( &mut self, observer: impl [IntoEntityObserver](../observer/trait.IntoEntityObserver.html "trait bevy::ecs::observer::IntoEntityObserver")<M>, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Creates an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") watching for an [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") of type `E` whose [`EntityEvent::event_target`](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") targets this entity.

##### Panics

If the entity has been despawned while this `EntityWorldMut` is still alive.

Panics if the given system is an exclusive system.

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/app/externally\_driven\_headless\_renderer.rs ([line 126](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#126))

```rust
121    fn screenshot(&mut self, target: RenderTarget, i: u32) {
122        self.0
123            .main
124            .world_mut()
125            .spawn(Screenshot::image(target.as_image().unwrap().clone()))
126            .observe(save_to_disk(format!("test_images/screenshot{i}.png")));
127    }
```

Hide additional examples

examples/usage/context\_menu.rs ([lines 184-194](../../../src/context_menu/context_menu.rs.html#184-194))

```rust
146fn background_and_button() -> impl Bundle {
147    (
148        Name::new("background"),
149        Node {
150            width: percent(100),
151            height: percent(100),
152            align_items: AlignItems::Center,
153            justify_content: JustifyContent::Center,
154            ..default()
155        },
156        ZIndex(-10),
157        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
158            parent
159                .spawn((
160                    Name::new("button"),
161                    Button,
162                    Node {
163                        width: px(250),
164                        height: px(65),
165                        border: UiRect::all(px(5)),
166                        justify_content: JustifyContent::Center,
167                        align_items: AlignItems::Center,
168                        border_radius: BorderRadius::MAX,
169                        ..default()
170                    },
171                    BorderColor::all(Color::BLACK),
172                    BackgroundColor(Color::BLACK),
173                    children![(
174                        Pickable::IGNORE,
175                        Text::new("Context Menu"),
176                        TextFont {
177                            font_size: FontSize::Px(28.0),
178                            ..default()
179                        },
180                        TextColor(Color::WHITE),
181                        TextShadow::default(),
182                    )],
183                ))
184                .observe(|mut event: On<Pointer<Press>>, mut commands: Commands| {
185                    // by default this event would bubble up further leading to the `CloseContextMenus`
186                    // event being triggered and undoing the opening of one here right away.
187                    event.propagate(false);
188
189                    debug!("click: {}", event.pointer_location.position);
190
191                    commands.trigger(OpenContextMenu {
192                        pos: event.pointer_location.position,
193                    });
194                });
195        })),
196    )
197}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2016-2020)

#### pub fn [clone\_with\_opt\_out](#method.clone_with_opt_out)( &mut self, target: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), config: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityClonerBuilder](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder")<'\_, [OptOut](../entity/struct.OptOut.html "struct bevy::ecs::entity::OptOut")\>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Clones parts of an entity (components, observers, etc.) onto another entity, configured through [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder").

The other entity will receive all the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") except those that are [denied](../entity/struct.EntityClonerBuilder.html#method.deny "method bevy::ecs::entity::EntityClonerBuilder::deny") in the `config`.

##### Example

```rust
// Clone all components except ComponentA onto the target.
world.entity_mut(entity).clone_with_opt_out(target, |builder| {
    builder.deny::<ComponentA>();
});
```

See [`EntityClonerBuilder<OptOut>`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for more options.

##### Panics

*   If this entity has been despawned while this `EntityWorldMut` is still alive.
*   If the target entity does not exist.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2064-2068)

#### pub fn [clone\_with\_opt\_in](#method.clone_with_opt_in)( &mut self, target: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), config: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityClonerBuilder](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder")<'\_, [OptIn](../entity/struct.OptIn.html "struct bevy::ecs::entity::OptIn")\>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

Clones parts of an entity (components, observers, etc.) onto another entity, configured through [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder").

The other entity will receive only the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and are [allowed](../entity/struct.EntityClonerBuilder.html#method.allow "method bevy::ecs::entity::EntityClonerBuilder::allow") in the `config`.

##### Example

```rust
// Clone only ComponentA onto the target.
world.entity_mut(entity).clone_with_opt_in(target, |builder| {
    builder.allow::<ComponentA>();
});
```

See [`EntityClonerBuilder<OptIn>`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for more options.

##### Panics

*   If this entity has been despawned while this `EntityWorldMut` is still alive.
*   If the target entity does not exist.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2092)

#### pub fn [clone\_and\_spawn](#method.clone_and_spawn)(&mut self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Spawns a clone of this entity and returns the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") of the clone.

The clone will receive all the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

To configure cloning behavior (such as only cloning certain components), use [`EntityWorldMut::clone_and_spawn_with_opt_out`](../../prelude/struct.EntityWorldMut.html#method.clone_and_spawn_with_opt_out "method bevy::prelude::EntityWorldMut::clone_and_spawn_with_opt_out")/ [`opt_in`](../../prelude/struct.EntityWorldMut.html#method.clone_and_spawn_with_opt_in "method bevy::prelude::EntityWorldMut::clone_and_spawn_with_opt_in").

##### Panics

If this entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2126-2129)

#### pub fn [clone\_and\_spawn\_with\_opt\_out](#method.clone_and_spawn_with_opt_out)( &mut self, config: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityClonerBuilder](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder")<'\_, [OptOut](../entity/struct.OptOut.html "struct bevy::ecs::entity::OptOut")\>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Spawns a clone of this entity and allows configuring cloning behavior using [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder"), returning the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") of the clone.

The clone will receive all the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") except those that are [denied](../entity/struct.EntityClonerBuilder.html#method.deny "method bevy::ecs::entity::EntityClonerBuilder::deny") in the `config`.

##### Example

```rust
// Create a clone of an entity but without ComponentA.
let entity_clone = world.entity_mut(entity).clone_and_spawn_with_opt_out(|builder| {
    builder.deny::<ComponentA>();
});
```

See [`EntityClonerBuilder<OptOut>`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for more options.

##### Panics

If this entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2172-2175)

#### pub fn [clone\_and\_spawn\_with\_opt\_in](#method.clone_and_spawn_with_opt_in)( &mut self, config: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [EntityClonerBuilder](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder")<'\_, [OptIn](../entity/struct.OptIn.html "struct bevy::ecs::entity::OptIn")\>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Spawns a clone of this entity and allows configuring cloning behavior using [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder"), returning the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") of the clone.

The clone will receive only the components of the original that implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and are [allowed](../entity/struct.EntityClonerBuilder.html#method.allow "method bevy::ecs::entity::EntityClonerBuilder::allow") in the `config`.

##### Example

```rust
// Create a clone of an entity but only with ComponentA.
let entity_clone = world.entity_mut(entity).clone_and_spawn_with_opt_in(|builder| {
    builder.allow::<ComponentA>();
});
```

See [`EntityClonerBuilder<OptIn>`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for more options.

##### Panics

If this entity has been despawned while this `EntityWorldMut` is still alive.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2197)

#### pub fn [clone\_components](#method.clone_components)<B>(&mut self, target: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Clones the specified components of this entity and inserts them into another entity.

Components can only be cloned if they implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

##### Panics

*   If this entity has been despawned while this `EntityWorldMut` is still alive.
*   If the target entity does not exist.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2219)

#### pub fn [move\_components](#method.move_components)<B>(&mut self, target: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Clones the specified components of this entity and inserts them into another entity, then removes the components from this entity.

Components can only be cloned if they implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

##### Panics

*   If this entity has been despawned while this `EntityWorldMut` is still alive.
*   If the target entity does not exist.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2233)

#### pub fn [spawned\_by](#method.spawned_by)(&self) -> [MaybeLocation](../change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")

Returns the source code location from which this entity has last been spawned.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2241)

#### pub fn [spawn\_tick](#method.spawn_tick)(&self) -> [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Returns the [`Tick`](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick") at which this entity has last been spawned.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2256)

#### pub fn [reborrow\_scope](#method.reborrow_scope)<U>( &mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>) -> U, ) -> U

Reborrows this entity in a temporary scope. This is useful for executing a function that requires a `EntityWorldMut` but you do not want to move out the entity ownership.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2274-2277)

#### pub fn [trigger](#method.trigger)<'t, E>( &mut self, event\_fn: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> E, ) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

where E: [EntityEvent](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent"), <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'t>: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Passes the current entity into the given function, and triggers the [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") returned by that function. See [`EntityCommands::trigger`](../../prelude/struct.EntityCommands.html#method.trigger "method bevy::prelude::EntityCommands::trigger") for usage examples

## Trait Implementations

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#45)

### impl [BuildChildrenTransformExt](../../prelude/trait.BuildChildrenTransformExt.html "trait bevy::prelude::BuildChildrenTransformExt") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#46)

#### fn [set\_parent\_in\_place](../../prelude/trait.BuildChildrenTransformExt.html#tymethod.set_parent_in_place)(&mut self, parent: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>

Change this entity’s parent while preserving this entity’s [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") by updating its [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform"). [Read more](../../prelude/trait.BuildChildrenTransformExt.html#tymethod.set_parent_in_place)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/commands.rs.html#68)

#### fn [remove\_parent\_in\_place](../../prelude/trait.BuildChildrenTransformExt.html#tymethod.remove_parent_in_place)(&mut self) -> &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>

Make this entity parentless while preserving this entity’s [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") by updating its [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform") to be equal to its current [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform"). [Read more](../../prelude/trait.BuildChildrenTransformExt.html#tymethod.remove_parent_in_place)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#477)

### impl [EntityWorldMutSceneExt](../../prelude/trait.EntityWorldMutSceneExt.html "trait bevy::prelude::EntityWorldMutSceneExt") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#478)

#### fn [queue\_spawn\_related\_scenes](../../prelude/trait.EntityWorldMutSceneExt.html#tymethod.queue_spawn_related_scenes)<T>( self, scenes: impl [SceneList](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), ) -> [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>

where T: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"),

Spawns a [`SceneList`](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), where each entity is related to the current entity using [`RelationshipTarget::Relationship`](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship"). [Read more](../../prelude/trait.EntityWorldMutSceneExt.html#tymethod.queue_spawn_related_scenes)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#499)

#### fn [apply\_scene](../../prelude/trait.EntityWorldMutSceneExt.html#tymethod.apply_scene)<S>(&mut self, scene: S) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SpawnSceneError](../../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError")\>

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Applies the given [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") to the current entity immediately. This will resolve the Scene (using [`Scene::resolve`](../../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve")). If that fails (for example, if there are dependencies that have not been loaded yet), it will return a [`SpawnSceneError`](../../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError"). If resolving the [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is successful, the scene will be spawned. [Read more](../../prelude/trait.EntityWorldMutSceneExt.html#tymethod.apply_scene)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#506)

#### fn [queue\_apply\_scene](../../prelude/trait.EntityWorldMutSceneExt.html#tymethod.queue_apply_scene)<S>(&mut self, scene: S)

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Queues the `scene` to be applied. This will evaluate the `scene`’s dependencies (via [`Scene::register_dependencies`](../../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](../../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error. [Read more](../../prelude/trait.EntityWorldMutSceneExt.html#tymethod.queue_apply_scene)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2298)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>> for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2300)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: &'a [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>) -> [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2326)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>> for [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'a, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2328)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: &'a [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>) -> [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'a, 'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2312)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>> for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2314)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: &'a mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>) -> [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2340)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>> for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2342)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: &'a mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2319)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'a>> for [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'a, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2321)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'a>) -> [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'a, 'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2333)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'a>> for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2335)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'a>) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2291)

### impl<'w> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>> for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2293)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>) -> [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2305)

### impl<'w> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>> for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2307)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>) -> [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>

Converts to this type from the input type.

## Auto Trait Implementations

### impl<'w> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

### impl<'w> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

### impl<'w> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

### impl<'w> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

### impl<'w> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

### impl<'w> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

### impl<'w> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}