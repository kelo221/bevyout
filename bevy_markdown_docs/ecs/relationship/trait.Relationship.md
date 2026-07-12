[bevy](../../index.html)::[ecs](../index.html)::[relationship](index.html)

# Trait Relationship 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#111)

```rust
pub trait Relationship: Sized + Component {
    type RelationshipTarget: RelationshipTarget<Relationship = Self>;

    const ALLOW_SELF_REFERENTIAL: bool = false;

    // Required methods
    fn get(&self) -> Entity;
    fn from(entity: Entity) -> Self;
    fn set_risky(&mut self, entity: Entity);

    // Provided methods
    fn on_insert(world: DeferredWorld<'_>, _: HookContext) { ... }
    fn on_discard(world: DeferredWorld<'_>, _: HookContext) { ... }
}
```

A [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") on a “source” [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") that references another target [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), creating a “relationship” between them. Every [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") has a corresponding [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") type (and vice-versa), which exists on the “target” entity of a relationship and contains the list of all “source” entities that relate to the given “target”.

A [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") may only be one-to-many (or one-to-one): an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") may point to at most one [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") through the [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") component.

The [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") component is the “source of truth” and the [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component reflects that source of truth. When a [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") component is inserted on an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), the corresponding [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component is immediately inserted on the target component if it does not already exist, and the “source” entity is automatically added to the [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") collection (this is done via “component hooks”).

A common example of a [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") is the parent / child relationship. Bevy ECS includes a canonical form of this via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") and the [`Children`](../../prelude/struct.Children.html "struct bevy::prelude::Children") [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget").

[`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") and [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") should always be derived via the [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") trait to ensure the hooks are set up properly.

### Derive

[`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") and [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") can only be derived for structs with a single unnamed field, single named field or for named structs where one field is annotated with `#[relationship]`. If there are additional fields, they must all implement [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default").

[`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") also requires that the relationship field is private to prevent direct mutation, ensuring the correctness of relationships.

```rust
#[derive(Component)]
#[relationship(relationship_target = Children)]
pub struct ChildOf {
    #[relationship]
    pub parent: Entity,
    internal: u8,
};

#[derive(Component)]
#[relationship_target(relationship = ChildOf)]
pub struct Children(Vec<Entity>);
```

A one-to-one relationship can be created by putting a single [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") in the [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget")’s field. In that case, if another entity is added to the relationship, the original entity is removed.

```rust
#[derive(Component)]
#[relationship(relationship_target = View)]
pub struct ViewOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ViewOf)]
pub struct View(Entity);
```

When deriving [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") you can specify the `#[relationship_target(linked_spawn)]` attribute to automatically despawn entities stored in an entity’s [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") when that entity is despawned:

```rust
#[derive(Component)]
#[relationship(relationship_target = Children)]
pub struct ChildOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ChildOf, linked_spawn)]
pub struct Children(Vec<Entity>);
```

By default, relationships cannot point to their own entity. If you want to allow self-referential relationships, you can use the `allow_self_referential` attribute:

```rust
#[derive(Component)]
#[relationship(relationship_target = PeopleILike, allow_self_referential)]
pub struct LikedBy(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = LikedBy)]
pub struct PeopleILike(Vec<Entity>);
```

## Provided Associated Constants

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#127)

#### const [ALLOW\_SELF\_REFERENTIAL](#associatedconstant.ALLOW_SELF_REFERENTIAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

If `true`, a relationship is allowed to point to its own entity.

Set this to `true` when self-relationships are semantically valid for your use case, such as `Likes(self)`, `EmployedBy(self)`, or a `ColliderOf` relationship where a collider can be attached to its own entity.

##### Warning

When `ALLOW_SELF` is `true`, be careful when using recursive traversal methods like `iter_ancestors` or `root_ancestor`, as they will loop infinitely if an entity points to itself.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#114)

#### type [RelationshipTarget](#associatedtype.RelationshipTarget): [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget")<Relationship = Self>

The [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") added to the “target” entities of this [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), which contains the list of all “source” entities that relate to the “target”.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#130)

#### fn [get](#tymethod.get)(&self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Gets the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") ID of the related entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#133)

#### fn [from](#tymethod.from)(entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> Self

Creates this [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") from the given `entity`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#147)

#### fn [set\_risky](#tymethod.set_risky)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

Changes the current [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") ID of the entity containing the [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") to another one.

This is useful for updating the relationship without overwriting other fields stored in `Self`.

##### Warning

This should generally not be called by user code, as modifying the related entity could invalidate the relationship. If this method is used, then the hooks [`on_discard`](trait.Relationship.html#method.on_discard "associated function bevy::ecs::relationship::Relationship::on_discard") have to run before and [`on_insert`](trait.Relationship.html#method.on_insert "associated function bevy::ecs::relationship::Relationship::on_insert") after it. This happens automatically when this method is called with [`EntityWorldMut::modify_component`](../../prelude/struct.EntityWorldMut.html#method.modify_component "method bevy::prelude::EntityWorldMut::modify_component").

Prefer to use regular means of insertions when possible.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#150-158)

#### fn [on\_insert](#method.on_insert)(world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, \_: [HookContext](../lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))

The `on_insert` component hook that maintains the [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") / [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") connection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#219-226)

#### fn [on\_discard](#method.on_discard)(world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, \_: [HookContext](../lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))

The `on_discard` component hook that maintains the [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") / [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") connection.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#94)

### impl [Relationship](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") for [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#94)

#### const [ALLOW\_SELF\_REFERENTIAL](#associatedconstant.ALLOW_SELF_REFERENTIAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#94)

#### type [RelationshipTarget](#associatedtype.RelationshipTarget) = [Children](../../prelude/struct.Children.html "struct bevy::prelude::Children")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1556)

### impl [Relationship](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") for [OnMonitor](../../window/struct.OnMonitor.html "struct bevy::window::OnMonitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1556)

#### const [ALLOW\_SELF\_REFERENTIAL](#associatedconstant.ALLOW_SELF_REFERENTIAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1556)

#### type [RelationshipTarget](#associatedtype.RelationshipTarget) = [HasWindows](../../window/struct.HasWindows.html "struct bevy::window::HasWindows")