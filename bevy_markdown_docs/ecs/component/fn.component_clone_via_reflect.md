[bevy](../../index.html)::[ecs](../index.html)::[component](index.html)

# Function component\_clone\_via\_reflect 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/clone.rs.html#85)

```rust
pub fn component_clone_via_reflect(
    source: &SourceComponent<'_>,
    ctx: &mut ComponentCloneCtx<'_, '_>,
)
```

Available on **crate feature `bevy_reflect`** only.

Component [clone handler function](type.ComponentCloneFn.html "type bevy::ecs::component::ComponentCloneFn") implemented using reflect. Can be [set](../../prelude/trait.Component.html#method.clone_behavior "associated function bevy::prelude::Component::clone_behavior") as clone handler for any registered component, but only reflected components will be cloned.

To clone a component using this handler, the following must be true:

*   World has [`AppTypeRegistry`](../../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry")
*   Component has [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")
*   Component is registered
*   Component has [`ReflectFromPtr`](../../reflect/struct.ReflectFromPtr.html "struct bevy::reflect::ReflectFromPtr") registered
*   Component can be cloned via [`PartialReflect::reflect_clone`](../../prelude/trait.PartialReflect.html#method.reflect_clone "method bevy::prelude::PartialReflect::reflect_clone") _or_ has one of the following registered: [`ReflectFromReflect`](../../prelude/struct.ReflectFromReflect.html "struct bevy::prelude::ReflectFromReflect"), [`ReflectDefault`](../../prelude/struct.ReflectDefault.html "struct bevy::prelude::ReflectDefault"), [`ReflectFromWorld`](../../prelude/struct.ReflectFromWorld.html "struct bevy::prelude::ReflectFromWorld")

If any of the conditions is not satisfied, the component will be skipped.

See [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for details.