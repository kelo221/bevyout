[bevy](../../index.html)::[ecs](../index.html)

# Module component 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#33)

Types for declaring and storing [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component")s.

## Structs

[ComponentDescriptor](struct.ComponentDescriptor.html "struct bevy::ecs::component::ComponentDescriptor")

A value describing a component or resource, which may or may not correspond to a Rust type.

[ComponentId](struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

A value which uniquely identifies the type of a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") or [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") within a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[ComponentIdFor](struct.ComponentIdFor.html "struct bevy::ecs::component::ComponentIdFor")

A [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides access to the [`ComponentId`](struct.ComponentId.html "struct bevy::ecs::component::ComponentId") for a specific component type.

[ComponentIds](struct.ComponentIds.html "struct bevy::ecs::component::ComponentIds")

Generates [`ComponentId`](struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s.

[ComponentInfo](struct.ComponentInfo.html "struct bevy::ecs::component::ComponentInfo")

Stores metadata for a type of component or resource stored in a specific [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Components](struct.Components.html "struct bevy::ecs::component::Components")

Stores metadata associated with each kind of [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") in a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[ComponentsQueuedRegistrator](struct.ComponentsQueuedRegistrator.html "struct bevy::ecs::component::ComponentsQueuedRegistrator")

A type that enables queuing registration in [`Components`](struct.Components.html "struct bevy::ecs::component::Components").

[ComponentsRegistrator](struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")

A [`Components`](struct.Components.html "struct bevy::ecs::component::Components") wrapper that enables additional features, like registration.

[Immutable](struct.Immutable.html "struct bevy::ecs::component::Immutable")

Parameter indicating a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") is immutable.

[Mutable](struct.Mutable.html "struct bevy::ecs::component::Mutable")

Parameter indicating a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") is mutable.

[QueuedComponents](struct.QueuedComponents.html "struct bevy::ecs::component::QueuedComponents")

Allows queuing components to be registered.

[RequiredComponent](struct.RequiredComponent.html "struct bevy::ecs::component::RequiredComponent")

Metadata associated with a required component. See [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") for details.

[RequiredComponentConstructor](struct.RequiredComponentConstructor.html "struct bevy::ecs::component::RequiredComponentConstructor")

A Required Component constructor. See [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") for details.

[RequiredComponents](struct.RequiredComponents.html "struct bevy::ecs::component::RequiredComponents")

The collection of metadata for components that are required for a given component.

[RequiredComponentsRegistrator](struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")

This is a safe handle around `ComponentsRegistrator` and `RequiredComponents` to register required components.

## Enums

[ComponentCloneBehavior](enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

The clone behavior to use when cloning or moving a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component").

[RequiredComponentsError](enum.RequiredComponentsError.html "enum bevy::ecs::component::RequiredComponentsError")

An error returned when the registration of a required component fails.

[StorageType](enum.StorageType.html "enum bevy::ecs::component::StorageType")

The storage used for a specific component type.

## Constants

[ADD](constant.ADD.html "constant bevy::ecs::component::ADD")

`usize` for the [`Add`](../../prelude/struct.Add.html "struct bevy::prelude::Add") component used in lifecycle observers.

[DESPAWN](constant.DESPAWN.html "constant bevy::ecs::component::DESPAWN")

`usize` for [`Despawn`](../../prelude/struct.Despawn.html "struct bevy::prelude::Despawn") component used in lifecycle observers.

[DISCARD](constant.DISCARD.html "constant bevy::ecs::component::DISCARD")

`usize` for the [`Discard`](../../prelude/struct.Discard.html "struct bevy::prelude::Discard") component used in lifecycle observers.

[INSERT](constant.INSERT.html "constant bevy::ecs::component::INSERT")

`usize` for the [`Insert`](../../prelude/struct.Insert.html "struct bevy::prelude::Insert") component used in lifecycle observers.

[IS\_RESOURCE](constant.IS_RESOURCE.html "constant bevy::ecs::component::IS_RESOURCE")

`usize` of the [`IsResource`](../resource/struct.IsResource.html "struct bevy::ecs::resource::IsResource") component used to mark entities with resources.

[REMOVE](constant.REMOVE.html "constant bevy::ecs::component::REMOVE")

`usize` for the [`Remove`](../../prelude/struct.Remove.html "struct bevy::prelude::Remove") component used in lifecycle observers.

## Traits

[Component](trait.Component.html "trait bevy::ecs::component::Component")

A data type that can be used to store data for an [entity](../entity/index.html "mod bevy::ecs::entity").

[ComponentMutability](trait.ComponentMutability.html "trait bevy::ecs::component::ComponentMutability")

The mutability option for a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component"). This can either be:

## Functions

[component\_clone\_ignore](fn.component_clone_ignore.html "fn bevy::ecs::component::component_clone_ignore")

Noop implementation of component clone handler function.

[component\_clone\_via\_clone](fn.component_clone_via_clone.html "fn bevy::ecs::component::component_clone_via_clone")

Component [clone handler function](type.ComponentCloneFn.html "type bevy::ecs::component::ComponentCloneFn") implemented using the [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") trait. Can be [set](../../prelude/trait.Component.html#method.clone_behavior "associated function bevy::prelude::Component::clone_behavior") as clone handler for the specific component it is implemented for. It will panic if set as handler for any other component.

[component\_clone\_via\_reflect](fn.component_clone_via_reflect.html "fn bevy::ecs::component::component_clone_via_reflect")`bevy_reflect`

Component [clone handler function](type.ComponentCloneFn.html "type bevy::ecs::component::ComponentCloneFn") implemented using reflect. Can be [set](../../prelude/trait.Component.html#method.clone_behavior "associated function bevy::prelude::Component::clone_behavior") as clone handler for any registered component, but only reflected components will be cloned.

## Type Aliases

[ComponentCloneFn](type.ComponentCloneFn.html "type bevy::ecs::component::ComponentCloneFn")

Function type that can be used to clone a component of an entity.

## Derive Macros

[Component](derive.Component.html "derive bevy::ecs::component::Component")

Cheat sheet for derive syntax, see full explanation and examples on the `Component` trait doc.