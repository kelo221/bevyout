[bevy](../../index.html)::[ecs](../index.html)::[reflect](index.html)

# Trait ReflectCommandExt 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#12)

```rust
pub trait ReflectCommandExt {
    // Required methods
    fn insert_reflect(
        &mut self,
        component: Box<dyn PartialReflect>,
    ) -> &mut Self;
    fn insert_reflect_with_registry<T>(
        &mut self,
        component: Box<dyn PartialReflect>,
    ) -> &mut Self
       where T: Resource + AsRef<TypeRegistry>;
    fn remove_reflect(
        &mut self,
        component_type_path: impl Into<Cow<'static, str>>,
    ) -> &mut Self;
    fn remove_reflect_with_registry<T>(
        &mut self,
        component_type_path: impl Into<Cow<'static, str>>,
    ) -> &mut Self
       where T: Resource + AsRef<TypeRegistry>;
}
```

Available on **crate feature `bevy_reflect`** only.

An extension trait for [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") for reflection related functions

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#87)

#### fn [insert\_reflect](#tymethod.insert_reflect)(&mut self, component: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>) -> &mut Self

Adds the given boxed reflect component or bundle to the entity using the reflection data in [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry").

This will overwrite any previous component(s) of the same type.

##### Panics

*   If the entity doesn’t exist.
*   If [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") does not have the reflection data for the given [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") or [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle").
*   If the component or bundle data is invalid. See [`PartialReflect::apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") for further details.
*   If [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") is not present in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### Note

Prefer to use the typed [`EntityCommands::insert`](../../prelude/struct.EntityCommands.html#method.insert "method bevy::prelude::EntityCommands::insert") if possible. Adding a reflected component is much slower.

##### Example

```rust
// Note that you need to register the component type in the AppTypeRegistry prior to using
// reflection. You can use the helpers on the App with `app.register_type::<ComponentA>()`
// or write to the TypeRegistry directly to register all your components

// A resource that can hold any component that implements reflect as a boxed reflect component
#[derive(Resource)]
struct Prefab {
    data: Box<dyn Reflect>,
}
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ComponentA(u32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ComponentB(String);

#[derive(Bundle, Reflect, Default)]
#[reflect(Bundle)]
struct BundleA {
    a: ComponentA,
    b: ComponentB,
}

fn insert_reflect_component(
    mut commands: Commands,
    mut prefab: ResMut<Prefab>
    ) {
    // Create a set of new boxed reflect components to use
    let boxed_reflect_component_a: Box<dyn Reflect> = Box::new(ComponentA(916));
    let boxed_reflect_component_b: Box<dyn Reflect>  = Box::new(ComponentB("NineSixteen".to_string()));
    let boxed_reflect_bundle_a: Box<dyn Reflect> = Box::new(BundleA {
        a: ComponentA(24),
        b: ComponentB("Twenty-Four".to_string()),
    });

    // You can overwrite the component in the resource with either ComponentA or ComponentB
    prefab.data = boxed_reflect_component_a;
    prefab.data = boxed_reflect_component_b;

    // Or even with BundleA
    prefab.data = boxed_reflect_bundle_a;

    // No matter which component or bundle is in the resource and without knowing the exact type, you can
    // use the insert_reflect entity command to insert that component/bundle into an entity.
    commands
        .spawn_empty()
        .insert_reflect(prefab.data.reflect_clone().unwrap().into_partial_reflect());
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#99-102)

#### fn [insert\_reflect\_with\_registry](#tymethod.insert_reflect_with_registry)<T>( &mut self, component: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> &mut Self

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry")\>,

Same as [`insert_reflect`](trait.ReflectCommandExt.html#tymethod.insert_reflect "method bevy::ecs::reflect::ReflectCommandExt::insert_reflect"), but using the `T` resource as type registry instead of `AppTypeRegistry`.

##### Panics

*   If the given [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") is not present in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### Note

*   The given [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") is removed from the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") before the command is applied.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#161)

#### fn [remove\_reflect](#tymethod.remove_reflect)( &mut self, component\_type\_path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>, ) -> &mut Self

Removes from the entity the component or bundle with the given type path registered in [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry").

If the type is a bundle, it will remove any components in that bundle regardless if the entity contains all the components.

Does nothing if the type is a component and the entity does not have a component of the same type, if the type is a bundle and the entity does not contain any of the components in the bundle, if [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") does not contain the reflection data for the given component, or if the entity does not exist.

##### Note

Prefer to use the typed [`EntityCommands::remove`](../../prelude/struct.EntityCommands.html#method.remove "method bevy::prelude::EntityCommands::remove") if possible. Removing a reflected component is much slower.

##### Example

```rust
// Note that you need to register the component/bundle type in the AppTypeRegistry prior to using
// reflection. You can use the helpers on the App with `app.register_type::<ComponentA>()`
// or write to the TypeRegistry directly to register all your components and bundles


// A resource that can hold any component or bundle that implements reflect as a boxed reflect
#[derive(Resource)]
struct Prefab{
    entity: Entity,
    data: Box<dyn Reflect>,
}
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ComponentA(u32);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct ComponentB(String);
#[derive(Bundle, Reflect, Default)]
#[reflect(Bundle)]
struct BundleA {
    a: ComponentA,
    b: ComponentB,
}

fn remove_reflect_component(
    mut commands: Commands,
    prefab: Res<Prefab>
    ) {
    // Prefab can hold any boxed reflect component or bundle. In this case either
    // ComponentA, ComponentB, or BundleA. No matter which component or bundle is in the resource though,
    // we can attempt to remove any component (or set of components in the case of a bundle)
    // of that same type from an entity.
    commands.entity(prefab.entity)
        .remove_reflect(prefab.data.reflect_type_path().to_owned());
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#164-167)

#### fn [remove\_reflect\_with\_registry](#tymethod.remove_reflect_with_registry)<T>( &mut self, component\_type\_path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>, ) -> &mut Self

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry")\>,

Same as [`remove_reflect`](trait.ReflectCommandExt.html#tymethod.remove_reflect "method bevy::ecs::reflect::ReflectCommandExt::remove_reflect"), but using the `T` resource as type registry instead of `AppTypeRegistry`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/entity_commands.rs.html#170)

### impl [ReflectCommandExt](trait.ReflectCommandExt.html "trait bevy::ecs::reflect::ReflectCommandExt") for [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>