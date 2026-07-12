[bevy](../index.html)::[prelude](index.html)

# Trait Bundle 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/mod.rs.html#207)

```rust
pub unsafe trait Bundle:
    DynamicBundle
    + Send
    + Sync
    + 'static {
    // Required method
    fn get_component_ids(
        components: &Components,
    ) -> impl Iterator<Item = Option<ComponentId>>;
}
```

The `Bundle` trait enables insertion and removal of [`Component`](trait.Component.html "trait bevy::prelude::Component")s from an entity.

Implementers of the `Bundle` trait are called ‘bundles’.

Each bundle represents a static set of [`Component`](trait.Component.html "trait bevy::prelude::Component") types. Currently, bundles can only contain one of each [`Component`](trait.Component.html "trait bevy::prelude::Component"), and will panic once initialized if this is not met.

### Insertion

The primary use for bundles is to add a useful collection of components to an entity.

Adding a value of bundle to an entity will add the components from the set it represents to the entity. The values of these components are taken from the bundle. If an entity already had one of these components, the entity’s original component value will be overwritten.

Importantly, bundles are only their constituent set of components. You **should not** use bundles as a unit of behavior. The behavior of your app can only be considered in terms of components, as systems, which drive the behavior of a `bevy` application, operate on combinations of components.

This rule is also important because multiple bundles may contain the same component type, calculated in different ways — adding both of these bundles to one entity would create incoherent behavior. This would be unexpected if bundles were treated as an abstraction boundary, as the abstraction would be unmaintainable for these cases.

For this reason, there is intentionally no [`Query`](struct.Query.html "struct bevy::prelude::Query") to match whether an entity contains the components of a bundle. Queries should instead only select the components they logically operate on.

### Removal

Bundles are also used when removing components from an entity.

Removing a bundle from an entity will remove any of its components attached to the entity from the entity. That is, if the entity does not have all the components of the bundle, those which are present will be removed.

## Implementers

Every type which implements [`Component`](trait.Component.html "trait bevy::prelude::Component") also implements `Bundle`, since [`Component`](trait.Component.html "trait bevy::prelude::Component") types can be added to or removed from an entity.

Additionally, [Tuples](https://doc.rust-lang.org/nightly/std/primitive.tuple.html "primitive tuple") of bundles are also [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle") (with up to 15 bundles). These bundles contain the items of the ‘inner’ bundles. This is a convenient shorthand which is primarily used when spawning entities.

[`unit`](https://doc.rust-lang.org/nightly/std/primitive.unit.html "primitive unit"), otherwise known as [`()`](https://doc.rust-lang.org/nightly/std/primitive.unit.html "primitive unit"), is a [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle") containing no components (since it can also be considered as the empty tuple). This can be useful for spawning large numbers of empty entities using [`World::spawn_batch`](struct.World.html#method.spawn_batch "method bevy::prelude::World::spawn_batch").

Tuple bundles can be nested, which can be used to create an anonymous bundle with more than 15 items. However, in most cases where this is required, the derive macro [`Bundle`](derive.Bundle.html "derive bevy::prelude::Bundle") should be used instead. The derived `Bundle` implementation contains the items of its fields, which all must implement `Bundle`. As explained above, this includes any [`Component`](trait.Component.html "trait bevy::prelude::Component") type, and other derived bundles.

If you want to add `PhantomData` to your `Bundle` you have to mark it with `#[bundle(ignore)]`.

```rust
use bevy_ecs::{component::Component, bundle::Bundle};

#[derive(Component)]
struct XPosition(i32);
#[derive(Component)]
struct YPosition(i32);

#[derive(Bundle)]
struct PositionBundle {
    // A bundle can contain components
    x: XPosition,
    y: YPosition,
}

// You have to implement `Default` for ignored field types in bundle structs.
#[derive(Default)]
struct Other(f32);

#[derive(Bundle)]
struct NamedPointBundle<T: Send + Sync + 'static> {
    // Or other bundles
    a: PositionBundle,
    // In addition to more components
    z: PointName,

    // when you need to use `PhantomData` you have to mark it as ignored
    #[bundle(ignore)]
    _phantom_data: PhantomData<T>
}

#[derive(Component)]
struct PointName(String);
```

## Safety

Manual implementations of this trait are unsupported. That is, there is no safe way to implement this trait, and you must not do so. If you want a type to implement [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle"), you must use [`derive@Bundle`](derive.Bundle.html "derive bevy::prelude::Bundle").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/mod.rs.html#216)

#### fn [get\_component\_ids](#tymethod.get_component_ids)( components: &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

### impl [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

#### fn [get\_component\_ids](#tymethod.get_component_ids)( components: &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

### impl<B> [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle") for [(B₁, B₂, …, Bₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where B: [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#176-183)

#### fn [get\_component\_ids](#tymethod.get_component_ids)( components: &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#16)

### impl<C> [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/observe.rs.html#20-25)

### impl<E, B, M, I> [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle") for [AddObserver](../ui_widgets/struct.AddObserver.html "struct bevy::ui_widgets::AddObserver")<E, B, M, I>

where E: [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent"), B: [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"), M: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, I: [IntoObserverSystem](../ecs/system/trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem")<E, B, M> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#398)

### impl<R, B> [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle") for [SpawnOneRelated](../ecs/spawn/struct.SpawnOneRelated.html "struct bevy::ecs::spawn::SpawnOneRelated")<R, B>

where R: [Relationship](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), B: [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#298-299)

### impl<R, L> [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle") for [SpawnRelatedBundle](../ecs/spawn/struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle")<R, L>

where R: [Relationship](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), L: [SpawnableList](../ecs/spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,