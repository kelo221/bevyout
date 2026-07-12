[bevy](../../index.html)::[ecs](../index.html)

# Module relationship 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#49)

This module provides functionality to link entities to each other using specialized components called “relationships”. See the [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") trait for more info.

## Structs

[AncestorIter](struct.AncestorIter.html "struct bevy::ecs::relationship::AncestorIter")

An [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s over the ancestors of an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[ComponentRelationshipAccessor](struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")

A type-safe convenience wrapper over [`RelationshipAccessor`](enum.RelationshipAccessor.html "enum bevy::ecs::relationship::RelationshipAccessor").

[DescendantDepthFirstIter](struct.DescendantDepthFirstIter.html "struct bevy::ecs::relationship::DescendantDepthFirstIter")

An [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s over the descendants of an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[DescendantIter](struct.DescendantIter.html "struct bevy::ecs::relationship::DescendantIter")

An [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s over the descendants of an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[RelatedSpawner](struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner")

Directly spawns related “source” entities with the given [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), targeting a specific entity.

[RelatedSpawnerCommands](struct.RelatedSpawnerCommands.html "struct bevy::ecs::relationship::RelatedSpawnerCommands")

Uses commands to spawn related “source” entities with the given [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), targeting a specific entity.

## Enums

[RelationshipAccessor](enum.RelationshipAccessor.html "enum bevy::ecs::relationship::RelationshipAccessor")

This enum describes a way to access the entities of [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") and [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") components in a type-erased context.

[RelationshipAccessorInitializer](enum.RelationshipAccessorInitializer.html "enum bevy::ecs::relationship::RelationshipAccessorInitializer")

Initializer enum for [`RelationshipAccessor`](enum.RelationshipAccessor.html "enum bevy::ecs::relationship::RelationshipAccessor") that allows to configure relationship for dynamic components.

[RelationshipHookMode](enum.RelationshipHookMode.html "enum bevy::ecs::relationship::RelationshipHookMode")

Configures the conditions under which the Relationship insert/discard hooks will be run.

## Traits

[OrderedRelationshipSourceCollection](trait.OrderedRelationshipSourceCollection.html "trait bevy::ecs::relationship::OrderedRelationshipSourceCollection")

This trait signals that a [`RelationshipSourceCollection`](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") is ordered.

[Relationship](trait.Relationship.html "trait bevy::ecs::relationship::Relationship")

A [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") on a “source” [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") that references another target [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), creating a “relationship” between them. Every [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") has a corresponding [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") type (and vice-versa), which exists on the “target” entity of a relationship and contains the list of all “source” entities that relate to the given “target”.

[RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection")

The internal [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") collection used by a [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component. This is not intended to be modified directly by users, as it could invalidate the correctness of relationships.

[RelationshipTarget](trait.RelationshipTarget.html "trait bevy::ecs::relationship::RelationshipTarget")

A [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") containing the collection of entities that relate to this [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") via the associated `Relationship` type. See the [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") documentation for more information.

## Functions

[clone\_relationship\_target](fn.clone_relationship_target.html "fn bevy::ecs::relationship::clone_relationship_target")

The “clone behavior” for [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"). The [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") will be populated with the proper components when the corresponding [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") sources of truth are inserted. Cloning the actual entities in the original [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") would result in duplicates, so we don’t do that!

## Type Aliases

[SourceIter](type.SourceIter.html "type bevy::ecs::relationship::SourceIter")

The iterator type for the source entities in a [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") collection, as defined in the [`RelationshipSourceCollection`](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") trait.