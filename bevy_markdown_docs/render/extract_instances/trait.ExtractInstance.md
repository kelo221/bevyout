[bevy](../../index.html)::[render](../index.html)::[extract\_instances](index.html)

# Trait ExtractInstance 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_instances.rs.html#31)

```rust
pub trait ExtractInstance:
    Sized
    + Send
    + Sync
    + 'static {
    type QueryData: ReadOnlyQueryData;
    type QueryFilter: QueryFilter;

    // Required method
    fn extract(
        item: <Self::QueryData as QueryData>::Item<'_, '_>,
    ) -> Option<Self>;
}
```

Describes how to extract data needed for rendering from a component or components.

Before rendering, any applicable components will be transferred from the main world to the render world in the [`ExtractSchedule`](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule") step.

This is essentially the same as [`ExtractComponent`](../extract_component/trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent"), but higher-performance because it avoids the ECS overhead.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_instances.rs.html#33)

#### type [QueryData](#associatedtype.QueryData): [ReadOnlyQueryData](../../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData")

ECS [`ReadOnlyQueryData`](../../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") to fetch the components to extract.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_instances.rs.html#35)

#### type [QueryFilter](#associatedtype.QueryFilter): [QueryFilter](../../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter")

Filters the entities with additional constraints.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_instances.rs.html#38)

#### fn [extract](#tymethod.extract)(item: <Self::[QueryData](trait.ExtractInstance.html#associatedtype.QueryData "type bevy::render::extract_instances::ExtractInstance::QueryData") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, '\_>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self>

Defines how the component is transferred into the “render world”.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/environment_map.rs.html#142)

### impl [ExtractInstance](trait.ExtractInstance.html "trait bevy::render::extract_instances::ExtractInstance") for [EnvironmentMapIds](../../pbr/environment_map/struct.EnvironmentMapIds.html "struct bevy::pbr::environment_map::EnvironmentMapIds")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/environment_map.rs.html#143)

#### type [QueryData](#associatedtype.QueryData) = &'static [EnvironmentMapLight](../../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/environment_map.rs.html#145)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)