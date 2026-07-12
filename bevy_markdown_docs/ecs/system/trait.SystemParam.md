[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait SystemParam 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#218)

```rust
pub unsafe trait SystemParam: Sized {
    type State: Send + Sync + 'static;
    type Item<'world, 'state>: SystemParam<State = Self::State>;

    // Required methods
    fn init_state(world: &mut World) -> Self::State;
    fn init_access(
        state: &Self::State,
        system_meta: &mut SystemMeta,
        component_access_set: &mut FilteredAccessSet,
        world: &mut World,
    );
    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &SystemMeta,
        world: UnsafeWorldCell<'world>,
        change_tick: Tick,
    ) -> Result<Self::Item<'world, 'state>, SystemParamValidationError>;

    // Provided methods
    fn apply(
        state: &mut Self::State,
        system_meta: &SystemMeta,
        world: &mut World,
    ) { ... }
    fn queue(
        state: &mut Self::State,
        system_meta: &SystemMeta,
        world: DeferredWorld<'_>,
    ) { ... }
}
```

A parameter that can be used in a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

## Derive

This trait can be derived with the [`super::SystemParam`](derive.SystemParam.html "derive bevy::ecs::system::SystemParam") macro. This macro only works if each field on the derived struct implements [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"). Note: There are additional requirements on the field types. See the _Generic `SystemParam`s_ section for details and workarounds of the probable cause if this derive causes an error to be emitted.

Derived `SystemParam` structs may have two lifetimes: `'w` for data stored in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), and `'s` for data stored in the parameter’s state.

The following list shows the most common [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")s and which lifetime they require

```rust
Query<'w, 's, Entity>,
Query<'w, 's, &'static SomeComponent>,
Res<'w, SomeResource>,
ResMut<'w, SomeOtherResource>,
Local<'s, u8>,
Commands<'w, 's>,
MessageReader<'w, 's, SomeMessage>,
MessageWriter<'w, SomeMessage>
```

### `PhantomData`

[`PhantomData`](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData") is a special type of `SystemParam` that does nothing. This is useful for constraining generic types or lifetimes.

## Example

```rust
use std::marker::PhantomData;
use bevy_ecs::system::SystemParam;

#[derive(SystemParam)]
struct MyParam<'w, Marker: 'static> {
    foo: Res<'w, SomeResource>,
    marker: PhantomData<Marker>,
}

fn my_system<T: 'static>(param: MyParam<T>) {
    // Access the resource through `param.foo`
}
```

## Generic `SystemParam`s

When using the derive macro, you may see an error in the form of:

```
expected ... [ParamType]
found associated type `<[ParamType] as SystemParam>::Item<'_, '_>`
```

where `[ParamType]` is the type of one of your fields. To solve this error, you can wrap the field of type `[ParamType]` with [`StaticSystemParam`](struct.StaticSystemParam.html "struct bevy::ecs::system::StaticSystemParam") (i.e. `StaticSystemParam<[ParamType]>`).

### Details

The derive macro requires that the [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") implementation of each field `F`’s [`Item`](trait.SystemParam.html#associatedtype.Item "associated type bevy::ecs::system::SystemParam::Item")’s is itself `F` (ignoring lifetimes for simplicity). This assumption is due to type inference reasons, so that the derived [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") can be used as an argument to a function system. If the compiler cannot validate this property for `[ParamType]`, it will error in the form shown above.

This will most commonly occur when working with `SystemParam`s generically, as the requirement has not been proven to the compiler.

### Custom Validation Messages

When using the derive macro, any [`SystemParamValidationError`](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")s will be propagated from the sub-parameters. If you want to override the error message, add a `#[system_param(validation_message = "New message")]` attribute to the parameter.

```rust
#[derive(SystemParam)]
struct MyParam<'w> {
    #[system_param(validation_message = "Custom Message")]
    foo: Res<'w, SomeResource>,
}

let mut world = World::new();
let err = world.run_system_cached(|param: MyParam| {}).unwrap_err();
let expected = "Parameter `MyParam::foo` failed validation: Custom Message";
assert!(err.to_string().contains(expected));
```

### Builders

If you want to use a [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") with a derived [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") implementation, add a `#[system_param(builder)]` attribute to the struct. This will generate a builder struct whose name is the param struct suffixed with `Builder`. The builder will not be `pub`, so you may want to expose a method that returns an `impl SystemParamBuilder<T>`.

```rust
mod custom_param {
    #[derive(SystemParam)]
    #[system_param(builder)]
    pub struct CustomParam<'w, 's> {
        query: Query<'w, 's, ()>,
        local: Local<'s, usize>,
    }

    impl<'w, 's> CustomParam<'w, 's> {
        pub fn builder(
            local: usize,
            query: impl FnOnce(&mut QueryBuilder<()>),
        ) -> impl SystemParamBuilder<Self> {
            CustomParamBuilder {
                local: LocalBuilder(local),
                query: QueryParamBuilder::new(query),
            }
        }
    }
}

use custom_param::CustomParam;

let system = (CustomParam::builder(100, |builder| {
    builder.with::<A>();
}),)
    .build_state(&mut world)
    .build_system(|param: CustomParam| {});
```

## Safety

The implementor must ensure the following is true.

*   [`SystemParam::init_access`](trait.SystemParam.html#tymethod.init_access "associated function bevy::ecs::system::SystemParam::init_access") correctly registers all [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") accesses used by [`SystemParam::get_param`](trait.SystemParam.html#tymethod.get_param "associated function bevy::ecs::system::SystemParam::get_param") with the provided [`system_meta`](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta").
*   None of the world accesses may conflict with any prior accesses registered on `system_meta`.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#220)

#### type [State](#associatedtype.State): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static

Used to store data which persists across invocations of a system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#226)

#### type [Item](#associatedtype.Item)<'world, 'state>: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")<State = Self::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")\>

The item type returned when constructing this system param. The value of this associated type should be `Self`, instantiated with new lifetimes.

You could think of [`SystemParam::Item<'w, 's>`](trait.SystemParam.html#associatedtype.Item "associated type bevy::ecs::system::SystemParam::Item") as being an _operation_ that changes the lifetimes bound to `Self`.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#229)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> Self::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

Creates a new instance of this param’s [`State`](trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#234-239)

#### fn [init\_access](#tymethod.init_access)( state: &Self::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

Registers any [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") access used by this [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

This method must panic if the access would conflict with any existing access in the [`FilteredAccessSet`](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#279-284)

#### unsafe fn [get\_param](#tymethod.get_param)<'world, 'state>( state: &'state mut Self::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'world>, change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

Creates a parameter to be passed into a [`SystemParamFunction`](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction").

This method also validates that the param can be acquired. If validation fails, an appropriate [`SystemParamValidationError`](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError") should be returned. Systems will convert this to a [`RunSystemError`](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError"), and the built-in executors will ignore any “skipped” validation results, but pass any “invalid” results to the fallback error handler defined in [`bevy_ecs::error`](../error/index.html "mod bevy::ecs::error").

For nested [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")s validation will fail if any delegated validation fails.

##### Safety

*   The passed [`UnsafeWorldCell`](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") must have access to any world data registered in [`init_access`](trait.SystemParam.html#tymethod.init_access "associated function bevy::ecs::system::SystemParam::init_access").
*   [`SystemParam::init_access`](trait.SystemParam.html#tymethod.init_access "associated function bevy::ecs::system::SystemParam::init_access") must not request conflicting access. If `Self` is `ReadOnlySystemParam`, the access is read-only and can never conflict. Otherwise, [`SystemParam::init_access`](trait.SystemParam.html#tymethod.init_access "associated function bevy::ecs::system::SystemParam::init_access") must be called to ensure it does not panic.
*   `world` must be the same [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") that was used to initialize [`state`](trait.SystemParam.html#tymethod.init_state "associated function bevy::ecs::system::SystemParam::init_state").

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#250)

#### fn [apply](#method.apply)(state: &mut Self::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"))

Applies any deferred mutations stored in this [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")’s state. This is used to apply [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") during [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#258)

#### fn [queue](#method.queue)( state: &mut Self::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

Queues any deferred mutations to be applied at the next [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### type [Item](#associatedtype.Item)<'w, 's> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### fn [init\_access](#tymethod.init_access)( state: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), \_system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), \_component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), \_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### fn [apply](#method.apply)( \_: &mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### fn [queue](#method.queue)( \_: &mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### unsafe fn [get\_param](#tymethod.get_param)<'w, 's>( state: &'s mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'w, 's>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

### impl<P> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [(P₁, P₂, …, Pₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where P: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

This trait is implemented for tuples up to 17 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### type [State](#associatedtype.State) = (<P as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"),)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### type [Item](#associatedtype.Item)<'w, 's> = (<P as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'w, 's>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### fn [init\_access](#tymethod.init_access)( state: &<[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), \_system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), \_component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), \_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### fn [apply](#method.apply)( \_: &mut <[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### fn [queue](#method.queue)( \_: &mut <[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

#### unsafe fn [get\_param](#tymethod.get_param)<'w, 's>( state: &'s mut <[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'w, 's>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1961)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

where T: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1962)

#### type [State](#associatedtype.State) = [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<\[<T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1964)

#### type [Item](#associatedtype.Item)<'world, 'state> = [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<\[<T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1966)

#### fn [init\_state](#tymethod.init_state)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1970-1975)

#### fn [init\_access](#tymethod.init_access)( state: &<[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1982-1987)

#### unsafe fn [get\_param](#tymethod.get_param)<'world, 'state>( state: &'state mut <[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'world>, change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1997)

#### fn [apply](#method.apply)( state: &mut <[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2003)

#### fn [queue](#method.queue)( state: &mut <[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1621)

### impl<T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1622)

#### type [State](#associatedtype.State) = <T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1624)

#### type [Item](#associatedtype.Item)<'world, 'state> = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1626)

#### fn [init\_state](#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1630-1635)

#### fn [init\_access](#tymethod.init_access)( state: &<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1640-1645)

#### unsafe fn [get\_param](#tymethod.get_param)<'world, 'state>( state: &'state mut <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'world>, change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1650)

#### fn [apply](#method.apply)( state: &mut <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1654)

#### fn [queue](#method.queue)( state: &mut <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2241)

### impl<T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2242)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2243)

#### type [Item](#associatedtype.Item)<'world, 'state> = [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2245)

#### fn [init\_state](#tymethod.init_state)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2247-2252)

#### fn [init\_access](#tymethod.init_access)( \_state: &<[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), \_system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), \_component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), \_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2256-2261)

#### unsafe fn [get\_param](#tymethod.get_param)<'world, 'state>( \_state: &'state mut <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), \_system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), \_world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'world>, \_change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1663)

### impl<T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

where T: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1664)

#### type [State](#associatedtype.State) = <T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1666)

#### type [Item](#associatedtype.Item)<'world, 'state> = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1668)

#### fn [init\_state](#tymethod.init_state)( world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> <[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1672-1677)

#### fn [init\_access](#tymethod.init_access)( state: &<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1682-1687)

#### unsafe fn [get\_param](#tymethod.get_param)<'world, 'state>( state: &'state mut <[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'world>, change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1692)

#### fn [apply](#method.apply)( state: &mut <[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1696)

#### fn [queue](#method.queue)( state: &mut <[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#788)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for &[World](../../prelude/struct.World.html "struct bevy::prelude::World")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#789)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#790)

#### type [Item](#associatedtype.Item)<'w, 's> = &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/auto_directional_navigation.rs.html#118)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [AutoDirectionalNavigator](../../ui/auto_directional_navigation/struct.AutoDirectionalNavigator.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigator")<'\_, '\_>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/auto_directional_navigation.rs.html#118)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/auto_directional_navigation.rs.html#118)

#### type [Item](#associatedtype.Item)<'w, 's> = [AutoDirectionalNavigator](../../ui/auto_directional_navigation/struct.AutoDirectionalNavigator.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigator")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#128)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#129)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#131)

#### type [Item](#associatedtype.Item)<'w, 's> = [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2982)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [DefaultUiCamera](../../prelude/struct.DefaultUiCamera.html "struct bevy::prelude::DefaultUiCamera")<'\_, '\_>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2982)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2982)

#### type [Item](#associatedtype.Item)<'w, 's> = [DefaultUiCamera](../../prelude/struct.DefaultUiCamera.html "struct bevy::prelude::DefaultUiCamera")<'w, 's>

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#347)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Diagnostics](../../diagnostic/struct.Diagnostics.html "struct bevy::diagnostic::Diagnostics")<'\_, '\_>

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#347)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#347)

#### type [Item](#associatedtype.Item)<'w, 's> = [Diagnostics](../../diagnostic/struct.Diagnostics.html "struct bevy::diagnostic::Diagnostics")<'w, 's>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#395)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [DirectionalNavigation](../../input_focus/directional_navigation/struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation")<'\_>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#395)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#395)

#### type [Item](#associatedtype.Item)<'w, 's> = [DirectionalNavigation](../../input_focus/directional_navigation/struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2518)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [DynSystemParam](struct.DynSystemParam.html "struct bevy::ecs::system::DynSystemParam")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2519)

#### type [State](#associatedtype.State) = [DynSystemParamState](struct.DynSystemParamState.html "struct bevy::ecs::system::DynSystemParamState")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2521)

#### type [Item](#associatedtype.Item)<'world, 'state> = [DynSystemParam](struct.DynSystemParam.html "struct bevy::ecs::system::DynSystemParam")<'world, 'state>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1254)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ExclusiveMarker](struct.ExclusiveMarker.html "struct bevy::ecs::system::ExclusiveMarker")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1255)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1256)

#### type [Item](#associatedtype.Item)<'w, 's> = [ExclusiveMarker](struct.ExclusiveMarker.html "struct bevy::ecs::system::ExclusiveMarker")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#250)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [FallbackImageMsaa](../../render/texture/struct.FallbackImageMsaa.html "struct bevy::render::texture::FallbackImageMsaa")<'\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#250)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#250)

#### type [Item](#associatedtype.Item)<'w, 's> = [FallbackImageMsaa](../../render/texture/struct.FallbackImageMsaa.html "struct bevy::render::texture::FallbackImageMsaa")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2570)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [FilteredResources](../../prelude/struct.FilteredResources.html "struct bevy::prelude::FilteredResources")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2571)

#### type [State](#associatedtype.State) = [Access](../query/struct.Access.html "struct bevy::ecs::query::Access")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2573)

#### type [Item](#associatedtype.Item)<'world, 'state> = [FilteredResources](../../prelude/struct.FilteredResources.html "struct bevy::prelude::FilteredResources")<'world, 'state>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2616)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [FilteredResourcesMut](../../prelude/struct.FilteredResourcesMut.html "struct bevy::prelude::FilteredResourcesMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2617)

#### type [State](#associatedtype.State) = [Access](../query/struct.Access.html "struct bevy::ecs::query::Access")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2619)

#### type [Item](#associatedtype.Item)<'world, 'state> = [FilteredResourcesMut](../../prelude/struct.FilteredResourcesMut.html "struct bevy::prelude::FilteredResourcesMut")<'world, 'state>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#187)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [FlushCommands](../../render/renderer/struct.FlushCommands.html "struct bevy::render::renderer::FlushCommands")<'\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#187)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#187)

#### type [Item](#associatedtype.Item)<'w, 's> = [FlushCommands](../../render/renderer/struct.FlushCommands.html "struct bevy::render::renderer::FlushCommands")<'w>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#415)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [IsFocusedHelper](../../input_focus/struct.IsFocusedHelper.html "struct bevy::input_focus::IsFocusedHelper")<'\_, '\_>

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#415)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#415)

#### type [Item](#associatedtype.Item)<'w, 's> = [IsFocusedHelper](../../input_focus/struct.IsFocusedHelper.html "struct bevy::input_focus::IsFocusedHelper")<'w, 's>

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#172)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [MeshRayCast](../../prelude/struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast")<'\_, '\_>

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#172)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#172)

#### type [Item](#associatedtype.Item)<'w, 's> = [MeshRayCast](../../prelude/struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1288)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [NonSendMarker](struct.NonSendMarker.html "struct bevy::ecs::system::NonSendMarker")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1289)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1290)

#### type [Item](#associatedtype.Item)<'w, 's> = [NonSendMarker](struct.NonSendMarker.html "struct bevy::ecs::system::NonSendMarker")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/parallel_scope.rs.html#52)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParallelCommands](../../prelude/struct.ParallelCommands.html "struct bevy::prelude::ParallelCommands")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/parallel_scope.rs.html#52)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/parallel_scope.rs.html#52)

#### type [Item](#associatedtype.Item)<'w, 's> = [ParallelCommands](../../prelude/struct.ParallelCommands.html "struct bevy::prelude::ParallelCommands")<'w, 's>

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#596)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [PickingMessageWriters](../../prelude/struct.PickingMessageWriters.html "struct bevy::prelude::PickingMessageWriters")<'\_>

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#596)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#596)

#### type [Item](#associatedtype.Item)<'w, 's> = [PickingMessageWriters](../../prelude/struct.PickingMessageWriters.html "struct bevy::prelude::PickingMessageWriters")<'w>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#131)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [RenderContext](../../render/renderer/struct.RenderContext.html "struct bevy::render::renderer::RenderContext")<'\_, '\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#131)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#131)

#### type [Item](#associatedtype.Item)<'w, 's> = [RenderContext](../../render/renderer/struct.RenderContext.html "struct bevy::render::renderer::RenderContext")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1592)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [SystemChangeTick](struct.SystemChangeTick.html "struct bevy::ecs::system::SystemChangeTick")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1593)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1594)

#### type [Item](#associatedtype.Item)<'w, 's> = [SystemChangeTick](struct.SystemChangeTick.html "struct bevy::ecs::system::SystemChangeTick")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_name.rs.html#51)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [SystemName](struct.SystemName.html "struct bevy::ecs::system::SystemName")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_name.rs.html#52)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_name.rs.html#53)

#### type [Item](#associatedtype.Item)<'w, 's> = [SystemName](struct.SystemName.html "struct bevy::ecs::system::SystemName")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/helper.rs.html#20)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [TransformHelper](../../prelude/struct.TransformHelper.html "struct bevy::prelude::TransformHelper")<'\_, '\_>

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/helper.rs.html#20)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/helper.rs.html#20)

#### type [Item](#associatedtype.Item)<'w, 's> = [TransformHelper](../../prelude/struct.TransformHelper.html "struct bevy::prelude::TransformHelper")<'w, 's>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#280)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [UiCameraMap](../../ui_render/struct.UiCameraMap.html "struct bevy::ui_render::UiCameraMap")<'\_, '\_>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#280)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#280)

#### type [Item](#associatedtype.Item)<'w, 's> = [UiCameraMap](../../ui_render/struct.UiCameraMap.html "struct bevy::ui_render::UiCameraMap")<'w, 's>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#56)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [UiChildren](../../ui/experimental/struct.UiChildren.html "struct bevy::ui::experimental::UiChildren")<'\_, '\_>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#56)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#56)

#### type [Item](#associatedtype.Item)<'w, 's> = [UiChildren](../../ui/experimental/struct.UiChildren.html "struct bevy::ui::experimental::UiChildren")<'w, 's>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#31)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [UiRootNodes](../../ui/experimental/struct.UiRootNodes.html "struct bevy::ui::experimental::UiRootNodes")<'\_, '\_>

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#31)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#31)

#### type [Item](#associatedtype.Item)<'w, 's> = [UiRootNodes](../../ui/experimental/struct.UiRootNodes.html "struct bevy::ui::experimental::UiRootNodes")<'w, 's>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#308)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [VisibilityExtractionSystemParam](../../render/view/struct.VisibilityExtractionSystemParam.html "struct bevy::render::view::VisibilityExtractionSystemParam")<'\_, '\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#308)

#### type [State](#associatedtype.State) = FetchState

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#308)

#### type [Item](#associatedtype.Item)<'w, 's> = [VisibilityExtractionSystemParam](../../render/view/struct.VisibilityExtractionSystemParam.html "struct bevy::render::view::VisibilityExtractionSystemParam")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/identifier.rs.html#55)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [WorldId](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/identifier.rs.html#56)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/identifier.rs.html#58)

#### type [Item](#associatedtype.Item)<'world, 'state> = [WorldId](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'\_w, '\_s, P0, P1, P2, P3, P4, P5, P6, P7> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'\_w, '\_s, [(P0, P1, P2, P3, P4, P5, P6, P7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P4: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P5: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P6: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P7: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [State](#associatedtype.State) = (<P0 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P1 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P2 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P3 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P4 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P5 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P6 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P7 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [Item](#associatedtype.Item)<'w, 's> = [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4, P5, P6, P7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'\_w, '\_s, P0, P1, P2, P3, P4, P5, P6> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'\_w, '\_s, [(P0, P1, P2, P3, P4, P5, P6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P4: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P5: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P6: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [State](#associatedtype.State) = (<P0 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P1 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P2 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P3 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P4 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P5 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P6 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [Item](#associatedtype.Item)<'w, 's> = [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4, P5, P6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'\_w, '\_s, P0, P1, P2, P3, P4, P5> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'\_w, '\_s, [(P0, P1, P2, P3, P4, P5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P4: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P5: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [State](#associatedtype.State) = (<P0 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P1 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P2 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P3 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P4 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P5 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [Item](#associatedtype.Item)<'w, 's> = [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4, P5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'\_w, '\_s, P0, P1, P2, P3, P4> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'\_w, '\_s, [(P0, P1, P2, P3, P4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P4: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [State](#associatedtype.State) = (<P0 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P1 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P2 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P3 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P4 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [Item](#associatedtype.Item)<'w, 's> = [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'\_w, '\_s, P0, P1, P2, P3> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'\_w, '\_s, [(P0, P1, P2, P3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [State](#associatedtype.State) = (<P0 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P1 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P2 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P3 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [Item](#associatedtype.Item)<'w, 's> = [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'\_w, '\_s, P0, P1, P2> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'\_w, '\_s, [(P0, P1, P2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [State](#associatedtype.State) = (<P0 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P1 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P2 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [Item](#associatedtype.Item)<'w, 's> = [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'\_w, '\_s, P0, P1> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'\_w, '\_s, [(P0, P1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [State](#associatedtype.State) = (<P0 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), <P1 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [Item](#associatedtype.Item)<'w, 's> = [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'\_w, '\_s, P0> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'\_w, '\_s, [(P0,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [State](#associatedtype.State) = (<P0 as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"),)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

#### type [Item](#associatedtype.Item)<'w, 's> = [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#341-342)

### impl<'a, 'b, D, F> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Single](../../prelude/struct.Single.html "struct bevy::prelude::Single")<'a, 'b, D, F>

where D: [IterQueryData](../query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") + 'static, F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#344)

#### type [State](#associatedtype.State) = [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#345)

#### type [Item](#associatedtype.Item)<'w, 's> = [Single](../../prelude/struct.Single.html "struct bevy::prelude::Single")<'w, 's, D, F>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#234-235)

### impl<'a, D, F> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ViewQuery](../../render/renderer/struct.ViewQuery.html "struct bevy::render::renderer::ViewQuery")<'a, '\_, D, F>

where D: [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#237)

#### type [State](#associatedtype.State) = ViewQueryState<D, F>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#238)

#### type [Item](#associatedtype.Item)<'w, 's> = [ViewQuery](../../render/renderer/struct.ViewQuery.html "struct bevy::render::renderer::ViewQuery")<'w, 's, D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1019)

### impl<'a, T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'a, T>

where T: [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1020)

#### type [State](#associatedtype.State) = [SyncCell](../../platform/cell/struct.SyncCell.html "struct bevy::platform::cell::SyncCell")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1021)

#### type [Item](#associatedtype.Item)<'w, 's> = [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1323)

### impl<'a, T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [NonSend](../../prelude/struct.NonSend.html "struct bevy::prelude::NonSend")<'a, T>

where T: 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1324)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1325)

#### type [Item](#associatedtype.Item)<'w, 's> = [NonSend](../../prelude/struct.NonSend.html "struct bevy::prelude::NonSend")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1368)

### impl<'a, T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [NonSendMut](../../prelude/struct.NonSendMut.html "struct bevy::prelude::NonSendMut")<'a, T>

where T: 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1369)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1370)

#### type [Item](#associatedtype.Item)<'w, 's> = [NonSendMut](../../prelude/struct.NonSendMut.html "struct bevy::prelude::NonSendMut")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#672)

### impl<'a, T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'a, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#673)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#674)

#### type [Item](#associatedtype.Item)<'w, 's> = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#729)

### impl<'a, T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'a, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#730)

#### type [State](#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#731)

#### type [Item](#associatedtype.Item)<'w, 's> = [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1418)

### impl<'a> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for &'a [Archetypes](../archetype/struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1419)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1420)

#### type [Item](#associatedtype.Item)<'w, 's> = &'w [Archetypes](../archetype/struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1534)

### impl<'a> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for &'a [Bundles](../bundle/struct.Bundles.html "struct bevy::ecs::bundle::Bundles")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1535)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1536)

#### type [Item](#associatedtype.Item)<'w, 's> = &'w [Bundles](../bundle/struct.Bundles.html "struct bevy::ecs::bundle::Bundles")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1447)

### impl<'a> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for &'a [Components](../component/struct.Components.html "struct bevy::ecs::component::Components")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1448)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1449)

#### type [Item](#associatedtype.Item)<'w, 's> = &'w [Components](../component/struct.Components.html "struct bevy::ecs::component::Components")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1476)

### impl<'a> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for &'a [Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1477)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1478)

#### type [Item](#associatedtype.Item)<'w, 's> = &'w [Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1505)

### impl<'a> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for &'a [EntityAllocator](../entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1506)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1507)

#### type [Item](#associatedtype.Item)<'w, 's> = &'w [EntityAllocator](../entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#622)

### impl<'a> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for &'a [RemovedComponentMessages](../lifecycle/struct.RemovedComponentMessages.html "struct bevy::ecs::lifecycle::RemovedComponentMessages")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#623)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#624)

#### type [Item](#associatedtype.Item)<'w, 's> = &'w [RemovedComponentMessages](../lifecycle/struct.RemovedComponentMessages.html "struct bevy::ecs::lifecycle::RemovedComponentMessages")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_reader.rs.html#158)

### impl<'w, 's, M> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [PopulatedMessageReader](../../prelude/struct.PopulatedMessageReader.html "struct bevy::prelude::PopulatedMessageReader")<'w, 's, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_reader.rs.html#159)

#### type [State](#associatedtype.State) = <[MessageReader](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader")<'w, 's, M> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_reader.rs.html#160)

#### type [Item](#associatedtype.Item)<'world, 'state> = [PopulatedMessageReader](../../prelude/struct.PopulatedMessageReader.html "struct bevy::prelude::PopulatedMessageReader")<'world, 'state, M>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#825)

### impl<'w> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#826)

#### type [State](#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#827)

#### type [Item](#associatedtype.Item)<'world, 'state> = [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'world>

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#195-198)

### impl<Config, Clear> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Gizmos](../../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos")<'\_, '\_, Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#200)

#### type [State](#associatedtype.State) = GizmosFetchState<Config, Clear>

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#201)

#### type [Item](#associatedtype.Item)<'w, 's> = [Gizmos](../../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos")<'w, 's, Config, Clear>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#394-395)

### impl<D, F> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Populated](../../prelude/struct.Populated.html "struct bevy::prelude::Populated")<'\_, '\_, D, F>

where D: [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#397)

#### type [State](#associatedtype.State) = [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#398)

#### type [Item](#associatedtype.Item)<'w, 's> = [Populated](../../prelude/struct.Populated.html "struct bevy::prelude::Populated")<'w, 's, D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#304)

### impl<D, F> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'\_, '\_, D, F>

where D: [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#305)

#### type [State](#associatedtype.State) = [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#306)

#### type [Item](#associatedtype.Item)<'w, 's> = [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_mutator.rs.html#56)

### impl<M> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [MessageMutator](../../prelude/struct.MessageMutator.html "struct bevy::prelude::MessageMutator")<'\_, '\_, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_mutator.rs.html#56)

#### type [State](#associatedtype.State) = FetchState<M>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_mutator.rs.html#56)

#### type [Item](#associatedtype.Item)<'w, 's> = [MessageMutator](../../prelude/struct.MessageMutator.html "struct bevy::prelude::MessageMutator")<'w, 's, M>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_reader.rs.html#33)

### impl<M> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [MessageReader](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader")<'\_, '\_, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_reader.rs.html#33)

#### type [State](#associatedtype.State) = FetchState<M>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_reader.rs.html#33)

#### type [Item](#associatedtype.Item)<'w, 's> = [MessageReader](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader")<'w, 's, M>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_writer.rs.html#61)

### impl<M> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'\_, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_writer.rs.html#61)

#### type [State](#associatedtype.State) = FetchState<M>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_writer.rs.html#61)

#### type [Item](#associatedtype.Item)<'w, 's> = [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, M>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_param.rs.html#68-70)

### impl<P> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Extract](../../render/struct.Extract.html "struct bevy::render::Extract")<'\_, '\_, P>

where P: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_param.rs.html#72)

#### type [State](#associatedtype.State) = ExtractState<P>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_param.rs.html#73)

#### type [Item](#associatedtype.Item)<'w, 's> = [Extract](../../render/struct.Extract.html "struct bevy::render::Extract")<'w, 's, P>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2203)

### impl<P> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [StaticSystemParam](struct.StaticSystemParam.html "struct bevy::ecs::system::StaticSystemParam")<'\_, '\_, P>

where P: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2204)

#### type [State](#associatedtype.State) = <P as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2205)

#### type [Item](#associatedtype.Item)<'world, 'state> = [StaticSystemParam](struct.StaticSystemParam.html "struct bevy::ecs::system::StaticSystemParam")<'world, 'state, P>

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#44)

### impl<R> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [TextReader](../../text/struct.TextReader.html "struct bevy::text::TextReader")<'\_, '\_, R>

where R: [TextSection](../../text/trait.TextSection.html "trait bevy::text::TextSection"),

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#44)

#### type [State](#associatedtype.State) = FetchState<R>

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#44)

#### type [Item](#associatedtype.Item)<'w, 's> = [TextReader](../../text/struct.TextReader.html "struct bevy::text::TextReader")<'w, 's, R>

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#287)

### impl<R> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [TextWriter](../../text/struct.TextWriter.html "struct bevy::text::TextWriter")<'\_, '\_, R>

where R: [TextSection](../../text/trait.TextSection.html "trait bevy::text::TextSection"),

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#287)

#### type [State](#associatedtype.State) = FetchState<R>

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#287)

#### type [Item](#associatedtype.Item)<'w, 's> = [TextWriter](../../text/struct.TextWriter.html "struct bevy::text::TextWriter")<'w, 's, R>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#749)

### impl<T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ComponentIdFor](../component/struct.ComponentIdFor.html "struct bevy::ecs::component::ComponentIdFor")<'\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#749)

#### type [State](#associatedtype.State) = FetchState<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#749)

#### type [Item](#associatedtype.Item)<'w, 's> = [ComponentIdFor](../component/struct.ComponentIdFor.html "struct bevy::ecs::component::ComponentIdFor")<'s, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1213)

### impl<T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Deferred](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred")<'\_, T>

where T: [SystemBuffer](trait.SystemBuffer.html "trait bevy::ecs::system::SystemBuffer"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1214)

#### type [State](#associatedtype.State) = [SyncCell](../../platform/cell/struct.SyncCell.html "struct bevy::platform::cell::SyncCell")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1215)

#### type [Item](#associatedtype.Item)<'w, 's> = [Deferred](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred")<'s, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1758)

### impl<T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [If](../../prelude/struct.If.html "struct bevy::prelude::If")<T>

where T: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1759)

#### type [State](#associatedtype.State) = <T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1761)

#### type [Item](#associatedtype.Item)<'world, 'state> = [If](../../prelude/struct.If.html "struct bevy::prelude::If")<<T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1858)

### impl<T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'\_, '\_, [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>>

where T: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1859)

#### type [State](#associatedtype.State) = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<<T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1861)

#### type [Item](#associatedtype.Item)<'world, 'state> = [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'world, 'state, [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#509)

### impl<T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [RemovedComponents](../../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents")<'\_, '\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#509)

#### type [State](#associatedtype.State) = FetchState<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#509)

#### type [Item](#associatedtype.Item)<'w, 's> = [RemovedComponents](../../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents")<'w, 's, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1806)

### impl<T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1807)

#### type [State](#associatedtype.State) = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<<T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1809)

#### type [Item](#associatedtype.Item)<'world, 'state> = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<<T as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>>