[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Trait Schedulable 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#39)

```rust
pub trait Schedulable {
    type Metadata;
    type GroupMetadata;

    // Required method
    fn into_config(self) -> ScheduleConfig<Self>
       where Self: Sized;
}
```

Stores data to differentiate different schedulable structs.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#41)

#### type [Metadata](#associatedtype.Metadata)

Additional data used to configure independent scheduling. Stored in [`ScheduleConfig`](struct.ScheduleConfig.html "struct bevy::ecs::schedule::ScheduleConfig").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#43)

#### type [GroupMetadata](#associatedtype.GroupMetadata)

Additional data used to configure a schedulable group. Stored in [`ScheduleConfigs`](enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#46-48)

#### fn [into\_config](#tymethod.into_config)(self) -> [ScheduleConfig](struct.ScheduleConfig.html "struct bevy::ecs::schedule::ScheduleConfig")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Initializes a configuration from this node.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#51)

### impl [Schedulable](trait.Schedulable.html "trait bevy::ecs::schedule::Schedulable") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#52)

#### type [Metadata](#associatedtype.Metadata) = [GraphInfo](struct.GraphInfo.html "struct bevy::ecs::schedule::GraphInfo")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#53)

#### type [GroupMetadata](#associatedtype.GroupMetadata) = [Chain](enum.Chain.html "enum bevy::ecs::schedule::Chain")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#68)

### impl [Schedulable](trait.Schedulable.html "trait bevy::ecs::schedule::Schedulable") for [Interned](../intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#69)

#### type [Metadata](#associatedtype.Metadata) = [GraphInfo](struct.GraphInfo.html "struct bevy::ecs::schedule::GraphInfo")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#70)

#### type [GroupMetadata](#associatedtype.GroupMetadata) = [Chain](enum.Chain.html "enum bevy::ecs::schedule::Chain")