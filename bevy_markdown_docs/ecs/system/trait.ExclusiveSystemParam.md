[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait ExclusiveSystemParam 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#17)

```rust
pub trait ExclusiveSystemParam: Sized {
    type State: Send + Sync + 'static;
    type Item<'s>: ExclusiveSystemParam<State = Self::State>;

    // Required methods
    fn init(world: &mut World, system_meta: &mut SystemMeta) -> Self::State;
    fn get_param<'s>(
        state: &'s mut Self::State,
        system_meta: &SystemMeta,
    ) -> Result<Self::Item<'s>, SystemParamValidationError>;
}
```

A parameter that can be used in an exclusive system (a system with an `&mut World` parameter). Any parameters implementing this trait must come after the `&mut World` parameter.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#19)

#### type [State](#associatedtype.State): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static

Used to store data which persists across invocations of a system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#22)

#### type [Item](#associatedtype.Item)<'s>: [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")<State = Self::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State")\>

The item type returned when constructing this system param. See [`SystemParam::Item`](trait.SystemParam.html#associatedtype.Item "associated type bevy::ecs::system::SystemParam::Item").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#25)

#### fn [init](#tymethod.init)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta")) -> Self::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State")

Creates a new instance of this param’s [`State`](trait.ExclusiveSystemParam.html#associatedtype.State "associated type bevy::ecs::system::ExclusiveSystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#30-33)

#### fn [get\_param](#tymethod.get_param)<'s>( state: &'s mut Self::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Item](trait.ExclusiveSystemParam.html#associatedtype.Item "type bevy::ecs::system::ExclusiveSystemParam::Item")<'s>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

Creates a parameter to be passed into an [`ExclusiveSystemParamFunction`](trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

### impl [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

#### type [Item](#associatedtype.Item)<'s> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

#### fn [init](#tymethod.init)( world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), ) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

#### fn [get\_param](#tymethod.get_param)<'s>( state: &'s mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[Item](trait.ExclusiveSystemParam.html#associatedtype.Item "type bevy::ecs::system::ExclusiveSystemParam::Item")<'s>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

### impl<P> [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam") for [(P₁, P₂, …, Pₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where P: [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam"),

This trait is implemented for tuples up to 17 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

#### type [State](#associatedtype.State) = (<P as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State"),)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

#### type [Item](#associatedtype.Item)<'s> = (<P as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[Item](trait.ExclusiveSystemParam.html#associatedtype.Item "type bevy::ecs::system::ExclusiveSystemParam::Item")<'s>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

#### fn [init](#tymethod.init)( world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), ) -> <[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#145-151)

#### fn [get\_param](#tymethod.get_param)<'s>( state: &'s mut <[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[Item](trait.ExclusiveSystemParam.html#associatedtype.Item "type bevy::ecs::system::ExclusiveSystemParam::Item")<'s>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#90)

### impl<S> [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<S>

where S: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#91)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#92)

#### type [Item](#associatedtype.Item)<'s> = [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<S>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#94)

#### fn [init](#tymethod.init)( \_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), \_system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), ) -> <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<S> as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#96-99)

#### fn [get\_param](#tymethod.get_param)<'s>( \_state: &'s mut <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<S> as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[State](trait.ExclusiveSystemParam.html#associatedtype.State "type bevy::ecs::system::ExclusiveSystemParam::State"), \_system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<S> as [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")\>::[Item](trait.ExclusiveSystemParam.html#associatedtype.Item "type bevy::ecs::system::ExclusiveSystemParam::Item")<'s>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_name.rs.html#79)

### impl [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam") for [SystemName](struct.SystemName.html "struct bevy::ecs::system::SystemName")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_name.rs.html#80)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_name.rs.html#81)

#### type [Item](#associatedtype.Item)<'s> = [SystemName](struct.SystemName.html "struct bevy::ecs::system::SystemName")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/identifier.rs.html#81)

### impl [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam") for [WorldId](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/identifier.rs.html#82)

#### type [State](#associatedtype.State) = [WorldId](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/identifier.rs.html#83)

#### type [Item](#associatedtype.Item)<'s> = [WorldId](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#74)

### impl<'\_s, T> [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam") for [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'\_s, T>

where T: [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#75)

#### type [State](#associatedtype.State) = [SyncCell](../../platform/cell/struct.SyncCell.html "struct bevy::platform::cell::SyncCell")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#76)

#### type [Item](#associatedtype.Item)<'s> = [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#40-41)

### impl<'a, D, F> [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam") for &'a mut [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#43)

#### type [State](#associatedtype.State) = [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#44)

#### type [Item](#associatedtype.Item)<'s> = &'s mut [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#58)

### impl<'a, P> [ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam") for &'a mut [SystemState](struct.SystemState.html "struct bevy::ecs::system::SystemState")<P>

where P: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#59)

#### type [State](#associatedtype.State) = [SystemState](struct.SystemState.html "struct bevy::ecs::system::SystemState")<P>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#60)

#### type [Item](#associatedtype.Item)<'s> = &'s mut [SystemState](struct.SystemState.html "struct bevy::ecs::system::SystemState")<P>