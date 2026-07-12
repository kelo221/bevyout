[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait ContiguousQueryData 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#409)

```rust
pub trait ContiguousQueryData: ArchetypeQueryData + IterQueryData {
    type Contiguous<'w, 's>;

    // Required method
    unsafe fn fetch_contiguous<'w, 's>(
        state: &'s Self::State,
        fetch: &mut Self::Fetch<'w>,
        entities: &'w [Entity],
    ) -> Self::Contiguous<'w, 's>;
}
```

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") which allows getting a direct access to contiguous chunks of components’ values, which may be used to apply simd-operations.

Contiguous iteration may be done via:

*   [`Query::contiguous_iter`](../../prelude/struct.Query.html#method.contiguous_iter "method bevy::prelude::Query::contiguous_iter"),
*   [`Query::contiguous_iter_mut`](../../prelude/struct.Query.html#method.contiguous_iter_mut "method bevy::prelude::Query::contiguous_iter_mut"),

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#412)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's>

Item returned by [`ContiguousQueryData::fetch_contiguous`](trait.ContiguousQueryData.html#tymethod.fetch_contiguous "associated function bevy::ecs::query::ContiguousQueryData::fetch_contiguous"). Represents a contiguous chunk of memory.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#423-427)

#### unsafe fn [fetch\_contiguous](#tymethod.fetch_contiguous)<'w, 's>( state: &'s Self::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut Self::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entities: &'w \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> Self::[Contiguous](trait.ContiguousQueryData.html#associatedtype.Contiguous "type bevy::ecs::query::ContiguousQueryData::Contiguous")<'w, 's>

Fetch [`ContiguousQueryData::Contiguous`](trait.ContiguousQueryData.html#associatedtype.Contiguous "associated type bevy::ecs::query::ContiguousQueryData::Contiguous") which represents a contiguous chunk of memory (e.g., an array) in the current [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table"). This must always be called after [`WorldQuery::set_table`](trait.WorldQuery.html#tymethod.set_table "associated function bevy::ecs::query::WorldQuery::set_table").

##### Safety

*   Must always be called _after_ [`WorldQuery::set_table`](trait.WorldQuery.html#tymethod.set_table "associated function bevy::ecs::query::WorldQuery::set_table").
*   `entities`’s length must match the length of the set table.
*   `entities` must match the entities of the set table.
*   There must not be simultaneous conflicting component access registered in `update_component_access`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### unsafe fn [fetch\_contiguous](#tymethod.fetch_contiguous)<'w, 's>( state: &'s <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entities: &'w \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData")\>::[Contiguous](trait.ContiguousQueryData.html#associatedtype.Contiguous "type bevy::ecs::query::ContiguousQueryData::Contiguous")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl<F> [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = (<F as [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData")\>::[Contiguous](trait.ContiguousQueryData.html#associatedtype.Contiguous "type bevy::ecs::query::ContiguousQueryData::Contiguous")<'w, 's>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### unsafe fn [fetch\_contiguous](#tymethod.fetch_contiguous)<'w, 's>( state: &'s <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entities: &'w \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData")\>::[Contiguous](trait.ContiguousQueryData.html#associatedtype.Contiguous "type bevy::ecs::query::ContiguousQueryData::Contiguous")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1916)

### impl<T> [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1917)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = &'w [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1919-1923)

#### unsafe fn [fetch\_contiguous](#tymethod.fetch_contiguous)<'w, 's>( \_state: &'s <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entities: &'w \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData")\>::[Contiguous](trait.ContiguousQueryData.html#associatedtype.Contiguous "type bevy::ecs::query::ContiguousQueryData::Contiguous")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2448)

### impl<T> [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2449)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = [ContiguousMut](../../prelude/struct.ContiguousMut.html "struct bevy::prelude::ContiguousMut")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2451-2455)

#### unsafe fn [fetch\_contiguous](#tymethod.fetch_contiguous)<'w, 's>( \_state: &'s <[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entities: &'w \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> <[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData")\>::[Contiguous](trait.ContiguousQueryData.html#associatedtype.Contiguous "type bevy::ecs::query::ContiguousQueryData::Contiguous")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3185)

### impl<T> [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3186)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData")\>::[Contiguous](trait.ContiguousQueryData.html#associatedtype.Contiguous "type bevy::ecs::query::ContiguousQueryData::Contiguous")<'w, 's>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3188-3192)

#### unsafe fn [fetch\_contiguous](#tymethod.fetch_contiguous)<'w, 's>( state: &'s <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [WorldQuery](trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entities: &'w \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\], ) -> <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData")\>::[Contiguous](trait.ContiguousQueryData.html#associatedtype.Contiguous "type bevy::ecs::query::ContiguousQueryData::Contiguous")<'w, 's>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#597)

### impl [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#598)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = &'w \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2617)

### impl<'\_\_w, T> [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_\_w, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2618)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = [ContiguousMut](../../prelude/struct.ContiguousMut.html "struct bevy::prelude::ContiguousMut")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F> [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = ([Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<F as [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData")\>::[Contiguous](trait.ContiguousQueryData.html#associatedtype.Contiguous "type bevy::ecs::query::ContiguousQueryData::Contiguous")<'w, 's>>,)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3384)

### impl<T> [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3385)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2184)

### impl<T> [ContiguousQueryData](trait.ContiguousQueryData.html "trait bevy::ecs::query::ContiguousQueryData") for [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2185)

#### type [Contiguous](#associatedtype.Contiguous)<'w, 's> = [ContiguousRef](../../prelude/struct.ContiguousRef.html "struct bevy::prelude::ContiguousRef")<'w, T>