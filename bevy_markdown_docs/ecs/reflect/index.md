[bevy](../../index.html)::[ecs](../index.html)

# Module reflect 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#48)

Available on **crate feature `bevy_reflect`** only.

Types that enable reflection support.

## Structs

[AppFunctionRegistry](struct.AppFunctionRegistry.html "struct bevy::ecs::reflect::AppFunctionRegistry")`reflect_functions`

A [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") storing [`FunctionRegistry`](../../reflect/func/struct.FunctionRegistry.html "struct bevy::reflect::func::FunctionRegistry") for function registrations relevant to a whole app.

[AppTypeRegistry](struct.AppTypeRegistry.html "struct bevy::ecs::reflect::AppTypeRegistry")

A [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") storing [`TypeRegistry`](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry") for type registrations relevant to a whole app.

[ReflectBundle](struct.ReflectBundle.html "struct bevy::ecs::reflect::ReflectBundle")

A struct used to operate on reflected [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") trait of a type.

[ReflectBundleFns](struct.ReflectBundleFns.html "struct bevy::ecs::reflect::ReflectBundleFns")

The raw function pointers needed to make up a [`ReflectBundle`](struct.ReflectBundle.html "struct bevy::ecs::reflect::ReflectBundle").

[ReflectComponent](struct.ReflectComponent.html "struct bevy::ecs::reflect::ReflectComponent")

A struct used to operate on reflected [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") trait of a type.

[ReflectComponentFns](struct.ReflectComponentFns.html "struct bevy::ecs::reflect::ReflectComponentFns")

The raw function pointers needed to make up a [`ReflectComponent`](../../prelude/struct.ReflectComponent.html "struct bevy::prelude::ReflectComponent").

[ReflectEvent](struct.ReflectEvent.html "struct bevy::ecs::reflect::ReflectEvent")

A struct used to operate on reflected [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") trait of a type.

[ReflectEventFns](struct.ReflectEventFns.html "struct bevy::ecs::reflect::ReflectEventFns")

The raw function pointers needed to make up a [`ReflectEvent`](../../prelude/struct.ReflectEvent.html "struct bevy::prelude::ReflectEvent").

[ReflectFromWorld](struct.ReflectFromWorld.html "struct bevy::ecs::reflect::ReflectFromWorld")

A struct used to operate on the reflected [`FromWorld`](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") trait of a type.

[ReflectFromWorldFns](struct.ReflectFromWorldFns.html "struct bevy::ecs::reflect::ReflectFromWorldFns")

The raw function pointers needed to make up a [`ReflectFromWorld`](../../prelude/struct.ReflectFromWorld.html "struct bevy::prelude::ReflectFromWorld").

[ReflectMapEntities](struct.ReflectMapEntities.html "struct bevy::ecs::reflect::ReflectMapEntities")

For a specific type of value, this maps any fields with values of type [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to a new world.

[ReflectMessage](struct.ReflectMessage.html "struct bevy::ecs::reflect::ReflectMessage")

A struct used to operate on reflected [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") trait of a type.

[ReflectMessageFns](struct.ReflectMessageFns.html "struct bevy::ecs::reflect::ReflectMessageFns")

The raw function pointers needed to make up a [`ReflectMessage`](../../prelude/struct.ReflectMessage.html "struct bevy::prelude::ReflectMessage").

[ReflectResource](struct.ReflectResource.html "struct bevy::ecs::reflect::ReflectResource")

A struct that marks a reflected [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") of a type.

## Traits

[ReflectCommandExt](trait.ReflectCommandExt.html "trait bevy::ecs::reflect::ReflectCommandExt")

An extension trait for [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") for reflection related functions

## Functions

[from\_reflect\_with\_fallback](fn.from_reflect_with_fallback.html "fn bevy::ecs::reflect::from_reflect_with_fallback")

Creates a `T` from a `&dyn PartialReflect`.