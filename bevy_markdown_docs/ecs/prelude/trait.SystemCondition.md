[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Trait SystemCondition 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#75-76)

```rust
pub trait SystemCondition<Marker, In = ()>: IntoSystem<In, bool, Marker>where
    Self::System: ReadOnlySystem,
    In: SystemInput,{
    // Provided methods
    fn and_then<M, C>(
        self,
        then_run: C,
    ) -> CombinatorSystem<AndThenMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn and_eager<M, C>(
        self,
        other: C,
    ) -> CombinatorSystem<AndEagerMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn and<M, C>(
        self,
        then_run: C,
    ) -> CombinatorSystem<AndThenMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn nand_then<M, C>(
        self,
        then_run: C,
    ) -> CombinatorSystem<NandThenMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn nand_eager<M, C>(
        self,
        other: C,
    ) -> CombinatorSystem<NandEagerMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn nand<M, C>(
        self,
        nand: C,
    ) -> CombinatorSystem<NandThenMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn nor_else<M, C>(
        self,
        else_run: C,
    ) -> CombinatorSystem<NorElseMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn nor_eager<M, C>(
        self,
        other: C,
    ) -> CombinatorSystem<NorEagerMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn nor<M, C>(
        self,
        else_run: C,
    ) -> CombinatorSystem<NorElseMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn or_else<M, C>(
        self,
        else_run: C,
    ) -> CombinatorSystem<OrElseMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn or_eager<M, C>(
        self,
        other: C,
    ) -> CombinatorSystem<OrEagerMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn or<M, C>(
        self,
        else_run: C,
    ) -> CombinatorSystem<OrElseMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn xnor<M, C>(
        self,
        other: C,
    ) -> CombinatorSystem<XnorMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
    fn xor<M, C>(
        self,
        other: C,
    ) -> CombinatorSystem<XorMarker, Self::System, <C as IntoSystem<In, bool, M>>::System>
       where C: SystemCondition<M, In> { ... }
}
```

A system that determines if one or more scheduled systems should run.

Implemented for functions and closures that convert into [`System<Out=bool>`](../../prelude/trait.System.html "trait bevy::prelude::System") with [read-only](../system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") parameters.

## Marker type parameter

`SystemCondition` trait has `Marker` type parameter, which has no special meaning, but exists to work around the limitation of Rust’s trait system.

Type parameter in return type can be set to `<()>` by calling [`IntoSystem::into_system`](../../prelude/trait.IntoSystem.html#tymethod.into_system "associated function bevy::prelude::IntoSystem::into_system"), but usually have to be specified when passing a condition to a function.

```rust
fn not_condition<Marker>(a: impl SystemCondition<Marker>) -> impl SystemCondition<()> {
   IntoSystem::into_system(a.map(|x| !x))
}
```

## Examples

A condition that returns true every other time it’s called.

```rust
fn every_other_time() -> impl SystemCondition<()> {
    IntoSystem::into_system(|mut flag: Local<bool>| {
        *flag = !*flag;
        *flag
    })
}

schedule.add_systems(my_system.run_if(every_other_time()));
```

A condition that takes a bool as an input and returns it unchanged.

```rust
fn identity() -> impl SystemCondition<(), In<bool>> {
    IntoSystem::into_system(|In(x): In<bool>| x)
}

app.add_systems(my_system.run_if(always_true.pipe(identity())));
```

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#135-138)

#### fn [and\_then](#method.and_then)<M, C>( self, then\_run: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<AndThenMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that only returns `true` if both this one and the passed `then_run` return `true`.

The returned run condition is short-circuiting, meaning `then_run` will only be invoked if `self` returns `true`.

Short-circuiting may not be desired in all cases; when utilizing change detection, the `then_run` condition will react to changes since the last time that _`self` returned `true`_, which may introduce subtle inconsistencies if short-circuiting was not intended. Similar issues may arise for run conditions that rely on internal state, such as those using [`Local<T>`](../../prelude/struct.Local.html "struct bevy::prelude::Local") parameters or [`MessageReader<T>`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader"), as they may not be updated every time the combined condition is evaluated.

See also [`and_eager`](../../prelude/trait.SystemCondition.html#method.and_eager "method bevy::prelude::SystemCondition::and_eager"), which always evaluates both conditions.

##### Examples

[ⓘ](# "This example panics")

```rust
use bevy_ecs::prelude::*;

#[derive(Resource, PartialEq)]
struct R(u32);

schedule.add_systems(
    // The `resource_equals` run condition will panic since we don't initialize `R`,
    // just like if we used `Res<R>` in a system.
    my_system.run_if(resource_equals(R(0))),
);
```

Use `.and_then()` to avoid checking the condition.

```rust
schedule.add_systems(
    // `resource_equals` will only get run if the resource `R` exists.
    my_system.run_if(resource_exists::<R>.and_then(resource_equals(R(0)))),
);
```

Note that in this specific case, it’s better to just use the run condition [`resource_exists_and_equals`](../../prelude/fn.resource_exists_and_equals.html "fn bevy::prelude::resource_exists_and_equals").

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/run\_conditions.rs ([lines 36-41](../../../src/run_conditions/run_conditions.rs.html#36-41))

```rust
5fn main() {
6    println!();
7    println!("For the first 2 seconds you will not be able to increment the counter");
8    println!("Once that time has passed you can press space, enter, left mouse, right mouse or touch the screen to increment the counter");
9    println!();
10
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .init_resource::<InputCounter>()
14        .add_systems(
15            Update,
16            (
17                increment_input_counter
18                    // The common_conditions module has a few useful run conditions
19                    // for checking resources and states. These are included in the prelude.
20                    .run_if(resource_exists::<InputCounter>)
21                    // `.or_else()` is a run condition combinator that only evaluates the second condition
22                    // if the first condition returns `false`. This behavior is known as "short-circuiting",
23                    // and is how the `||` operator works in Rust (as well as most C-family languages).
24                    // In this case, the `has_user_input` run condition will be evaluated since the `Unused` resource has not been initialized.
25                    .run_if(resource_exists::<Unused>.or_else(
26                        // This is a custom run condition, defined using a system that returns
27                        // a `bool` and which has read-only `SystemParam`s.
28                        // Only a single run condition must return `true` in order for the system to run.
29                        has_user_input,
30                    )),
31                print_input_counter
32                    // `.and_then()` is a run condition combinator that only evaluates the second condition
33                    // if the first condition returns `true`, analogous to the `&&` operator.
34                    // In this case, the short-circuiting behavior prevents the second run condition from
35                    // panicking if the `InputCounter` resource has not been initialized.
36                    .run_if(resource_exists::<InputCounter>.and_then(
37                        // This is a custom run condition in the form of a closure.
38                        // This is useful for small, simple run conditions you don't need to reuse.
39                        // All the normal rules still apply: all parameters must be read only except for local parameters.
40                        |counter: Res<InputCounter>| counter.is_changed() && !counter.is_added(),
41                    )),
42                print_time_message
43                    // This function returns a custom run condition, much like the common conditions module.
44                    // It will only return true once 2 seconds have passed.
45                    .run_if(time_passed(2.0))
46                    // You can use the `not` condition from the common_conditions module
47                    // to inverse a run condition. In this case it will return true if
48                    // less than 2.5 seconds have elapsed since the app started.
49                    .run_if(not(time_passed(2.5))),
50            ),
51        )
52        .run();
53}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#191-194)

#### fn [and\_eager](#method.and_eager)<M, C>( self, other: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<AndEagerMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that only returns `true` if both this one and the passed `then_run` return `true`.

The returned run condition is eagerly evaluated, meaning it will always execute both run conditions in order.

When applied directly to a system using [`run_if`](../../prelude/trait.IntoScheduleConfigs.html#method.run_if "method bevy::prelude::IntoScheduleConfigs::run_if"), the use of this combinator is behaviorally identical to simply calling `run_if` multiple times. However, `.and_eager` may be more efficient, as it does not erase the types of the inner conditions when evaluating them, which may allow for compiler optimizations that are not possible with separate calls to `run_if`.

See also [`and_then`](../../prelude/trait.SystemCondition.html#method.and_then "method bevy::prelude::SystemCondition::and_then"), which short-circuits if `self` returns `false`.

##### Examples

```rust
schedule.add_systems(
    // both conditions will execute, even though the first one returned false
    my_system.run_if(returns_false.and_eager(returns_true)),
);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#207)

#### fn [and](#method.and)<M, C>( self, then\_run: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<AndThenMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

👎Deprecated since 0.19.0:

use `.and_then(...)` instead, or `.and_eager(...)` to evaluate the conditions eagerly

Returns a new run condition that only returns `true` if both this one and the passed `then_run` return `true`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#319-322)

#### fn [nand\_then](#method.nand_then)<M, C>( self, then\_run: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<NandThenMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that only returns `false` if both this one and the passed `then_run` return `true`.

The returned run condition is short-circuiting, meaning `then_run` will only be invoked if `self` returns `true`.

Short-circuiting may not be desired in all cases; when utilizing change detection, the `then_run` condition will react to changes since the last time that _`self` returned `true`_, which may introduce subtle inconsistencies if short-circuiting was not intended. Similar issues may arise for run conditions that rely on internal state, such as those using [`Local<T>`](../../prelude/struct.Local.html "struct bevy::prelude::Local") parameters or [`MessageReader<T>`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader"), as they may not be updated every time the combined condition is evaluated.

See also [`nand_eager`](../../prelude/trait.SystemCondition.html#method.nand_eager "method bevy::prelude::SystemCondition::nand_eager"), which always evaluates both conditions.

##### Examples

```rust
schedule.add_systems(
    // The game_over_credits system will only execute if either the `in_state(PlayerState::Alive)`
    // run condition or `in_state(EnemyState::Alive)` run condition evaluates to `false`.
    game_over_credits.run_if(
        in_state(PlayerState::Alive).nand_then(in_state(EnemyState::Alive)),
    ),
);
```

Equivalent logic can be achieved by using `not` in concert with `and_then`:

```rust
schedule.add_systems(
    game_over_credits.run_if(
        not(in_state(PlayerState::Alive).and_then(in_state(EnemyState::Alive))),
    ),
);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#338-341)

#### fn [nand\_eager](#method.nand_eager)<M, C>( self, other: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<NandEagerMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that only returns `false` if both this one and the passed `then_run` return `true`.

The returned run condition is eagerly evaluated, meaning it will always execute both run conditions in order.

See also [`nand_then`](../../prelude/trait.SystemCondition.html#method.nand_then "method bevy::prelude::SystemCondition::nand_then"), which short-circuits if `self` returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#354)

#### fn [nand](#method.nand)<M, C>( self, nand: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<NandThenMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

👎Deprecated since 0.19.0:

use `.nand_then(...) instead, or` .nand\_eager(…)\` to evaluate the conditions eagerly

Returns a new run condition that only returns `false` if both this one and the passed `then_run` return `true`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#416-419)

#### fn [nor\_else](#method.nor_else)<M, C>( self, else\_run: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<NorElseMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that only returns `true` if both this one and the passed `else_run` return `false`.

The returned run condition is short-circuiting, meaning `else_run` will only be invoked if `self` returns `true`.

Short-circuiting may not be desired in all cases; when utilizing change detection, the `else_run` condition will react to changes since the last time that _`self` returned `true`_, which may introduce subtle inconsistencies if short-circuiting was not intended. Similar issues may arise for run conditions that rely on internal state, such as those using [`Local<T>`](../../prelude/struct.Local.html "struct bevy::prelude::Local") parameters or [`MessageReader<T>`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader"), as they may not be updated every time the combined condition is evaluated.

See also [`nor_eager`](../../prelude/trait.SystemCondition.html#method.nor_eager "method bevy::prelude::SystemCondition::nor_eager"), which always evaluates both conditions.

##### Examples

[ⓘ](# "This example deliberately fails to compile")

```rust
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WeatherState {
    Sunny,
    Cloudy,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SoilState {
    Fertilized,
    NotFertilized,
}

app.add_systems(
    // The slow_plant_growth system will only execute if both the `in_state(WeatherState::Sunny)`
    // run condition and `in_state(SoilState::Fertilized)` run condition evaluate to `false`.
    slow_plant_growth.run_if(
        in_state(WeatherState::Sunny).nor_else(in_state(SoilState::Fertilized)),
    ),
);
```

Equivalent logic can be achieved by using `not` in concert with `or`:

[ⓘ](# "This example deliberately fails to compile")

```rust
app.add_systems(
    slow_plant_growth.run_if(
        not(in_state(WeatherState::Sunny).or_else(in_state(SoilState::Fertilized))),
    ),
);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#435-438)

#### fn [nor\_eager](#method.nor_eager)<M, C>( self, other: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<NorEagerMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that only returns `true` if both this one and the passed `else_run` return `false`.

The returned run condition is eagerly evaluated, meaning it will always execute both run conditions in order.

See also [`nor_else`](../../prelude/trait.SystemCondition.html#method.nor_else "method bevy::prelude::SystemCondition::nor_else"), which short-circuits if `self` returns `true`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#451)

#### fn [nor](#method.nor)<M, C>( self, else\_run: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<NorElseMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

👎Deprecated since 0.19.0:

use `.nor_else(...)` instead, or `.nor_eager(...)` to evaluate the conditions eagerly

Returns a new run condition that only returns `true` if both this one and the passed `else_run` return `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#508)

#### fn [or\_else](#method.or_else)<M, C>( self, else\_run: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<OrElseMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that returns `true` if either this one or the passed `or` return `true`.

The returned run condition is short-circuiting, meaning `or` will only be invoked if `self` returns `false`.

Short-circuiting may not be desired in all cases; when utilizing change detection, the `else_run` condition will react to changes since the last time that _`self` returned `false`_, which may introduce subtle inconsistencies if short-circuiting was not intended. Similar issues may arise for run conditions that rely on internal state, such as those using [`Local<T>`](../../prelude/struct.Local.html "struct bevy::prelude::Local") parameters or [`MessageReader<T>`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader"), as they may not be updated every time the combined condition is evaluated.

See also [`or_eager`](../../prelude/trait.SystemCondition.html#method.or_eager "method bevy::prelude::SystemCondition::or_eager"), which always evaluates both conditions.

##### Examples

```rust
use bevy_ecs::prelude::*;

#[derive(Resource, PartialEq)]
struct A(u32);

#[derive(Resource, PartialEq)]
struct B(u32);

app.add_systems(
    // Only run the system if either `A` or `B` exist.
    my_system.run_if(resource_exists::<A>.or(resource_exists::<B>)),
);
```

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/ecs/run\_conditions.rs ([lines 25-30](../../../src/run_conditions/run_conditions.rs.html#25-30))

```rust
5fn main() {
6    println!();
7    println!("For the first 2 seconds you will not be able to increment the counter");
8    println!("Once that time has passed you can press space, enter, left mouse, right mouse or touch the screen to increment the counter");
9    println!();
10
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .init_resource::<InputCounter>()
14        .add_systems(
15            Update,
16            (
17                increment_input_counter
18                    // The common_conditions module has a few useful run conditions
19                    // for checking resources and states. These are included in the prelude.
20                    .run_if(resource_exists::<InputCounter>)
21                    // `.or_else()` is a run condition combinator that only evaluates the second condition
22                    // if the first condition returns `false`. This behavior is known as "short-circuiting",
23                    // and is how the `||` operator works in Rust (as well as most C-family languages).
24                    // In this case, the `has_user_input` run condition will be evaluated since the `Unused` resource has not been initialized.
25                    .run_if(resource_exists::<Unused>.or_else(
26                        // This is a custom run condition, defined using a system that returns
27                        // a `bool` and which has read-only `SystemParam`s.
28                        // Only a single run condition must return `true` in order for the system to run.
29                        has_user_input,
30                    )),
31                print_input_counter
32                    // `.and_then()` is a run condition combinator that only evaluates the second condition
33                    // if the first condition returns `true`, analogous to the `&&` operator.
34                    // In this case, the short-circuiting behavior prevents the second run condition from
35                    // panicking if the `InputCounter` resource has not been initialized.
36                    .run_if(resource_exists::<InputCounter>.and_then(
37                        // This is a custom run condition in the form of a closure.
38                        // This is useful for small, simple run conditions you don't need to reuse.
39                        // All the normal rules still apply: all parameters must be read only except for local parameters.
40                        |counter: Res<InputCounter>| counter.is_changed() && !counter.is_added(),
41                    )),
42                print_time_message
43                    // This function returns a custom run condition, much like the common conditions module.
44                    // It will only return true once 2 seconds have passed.
45                    .run_if(time_passed(2.0))
46                    // You can use the `not` condition from the common_conditions module
47                    // to inverse a run condition. In this case it will return true if
48                    // less than 2.5 seconds have elapsed since the app started.
49                    .run_if(not(time_passed(2.5))),
50            ),
51        )
52        .run();
53}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#524)

#### fn [or\_eager](#method.or_eager)<M, C>( self, other: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<OrEagerMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that returns `true` if either this one or the passed `or` return `true`.

The returned run condition is eagerly evaluated, meaning it will always execute both run conditions in order.

See also [`or_else`](../../prelude/trait.SystemCondition.html#method.or_else "method bevy::prelude::SystemCondition::or_else"), which short-circuits if `self` returns `true`.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/math/render\_primitives.rs ([line 45](../../../src/render_primitives/render_primitives.rs.html#45))

```rust
9fn main() {
10    let mut app = App::new();
11
12    app.add_plugins(DefaultPlugins)
13        .init_state::<PrimitiveSelected>()
14        .init_state::<CameraActive>();
15
16    // cameras
17    app.add_systems(Startup, (setup_cameras, setup_lights, setup_ambient_light))
18        .add_systems(
19            Update,
20            (
21                update_active_cameras.run_if(state_changed::<CameraActive>),
22                switch_cameras.run_if(input_just_pressed(KeyCode::KeyC)),
23            ),
24        );
25
26    // text
27
28    // PostStartup since we need the cameras to exist
29    app.add_systems(PostStartup, setup_text);
30    app.add_systems(
31        Update,
32        (update_text.run_if(state_changed::<PrimitiveSelected>),),
33    );
34
35    // primitives
36    app.add_systems(Startup, (spawn_primitive_2d, spawn_primitive_3d))
37        .add_systems(
38            Update,
39            (
40                switch_to_next_primitive.run_if(input_just_pressed(KeyCode::ArrowUp)),
41                switch_to_previous_primitive.run_if(input_just_pressed(KeyCode::ArrowDown)),
42                draw_gizmos_2d.run_if(in_mode(CameraActive::Dim2)),
43                draw_gizmos_3d.run_if(in_mode(CameraActive::Dim3)),
44                update_primitive_meshes.run_if(
45                    state_changed::<PrimitiveSelected>.or_eager(state_changed::<CameraActive>),
46                ),
47                rotate_primitive_2d_meshes,
48                rotate_primitive_3d_meshes,
49            ),
50        );
51
52    app.run();
53}
```

Hide additional examples

examples/diagnostics/log\_diagnostics.rs ([line 51](../../../src/log_diagnostics/log_diagnostics.rs.html#51))

```rust
25fn main() {
26    App::new()
27        .add_plugins((
28            // The diagnostics plugins need to be added after DefaultPlugins as they use e.g. the time plugin for timestamps.
29            DefaultPlugins,
30            // Adds a system that prints diagnostics to the console.
31            // The other diagnostics plugins can still be used without this if you want to use them in an ingame overlay for example.
32            LogDiagnosticsPlugin::default(),
33            // Adds frame time, FPS and frame count diagnostics.
34            FrameTimeDiagnosticsPlugin::default(),
35            // Adds an entity count diagnostic.
36            EntityCountDiagnosticsPlugin::default(),
37            // Adds cpu and memory usage diagnostics for systems and the entire game process.
38            SystemInformationDiagnosticsPlugin,
39            // Forwards various diagnostics from the render app to the main app.
40            // These are pretty verbose but can be useful to pinpoint performance issues.
41            bevy::render::diagnostic::RenderDiagnosticsPlugin,
42        ))
43        // No rendering diagnostics are emitted unless something is drawn to the screen,
44        // so we spawn a small scene.
45        .add_systems(Startup, setup)
46        .add_systems(Update, filters_inputs)
47        .add_systems(
48            Update,
49            update_commands.run_if(
50                resource_exists_and_changed::<LogDiagnosticsStatus>
51                    .or_eager(resource_exists_and_changed::<LogDiagnosticsFilters>),
52            ),
53        )
54        .run();
55}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#537)

#### fn [or](#method.or)<M, C>( self, else\_run: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<OrElseMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

👎Deprecated since 0.19.0:

use `.or_else(...)` instead, or `.or_eager(...)` to eagerly evaluate both conditions

Returns a new run condition that returns `true` if either this one or the passed `or` return `true`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#589)

#### fn [xnor](#method.xnor)<M, C>( self, other: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<XnorMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that only returns `true` if `self` and `xnor` **both** return `false` or **both** return `true`.

The returned run condition is eagerly evaluated, meaning it will always execute both run conditions in order.

##### Examples

[ⓘ](# "This example deliberately fails to compile")

```rust
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoffeeMachineState {
    Heating,
    Brewing,
    Inactive,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TeaKettleState {
    Heating,
    Steeping,
    Inactive,
}

app.add_systems(
    // The take_drink_orders system will only execute if the `in_state(CoffeeMachineState::Inactive)`
    // run condition and `in_state(TeaKettleState::Inactive)` run conditions both evaluate to `false`,
    // or both evaluate to `true`.
    take_drink_orders.run_if(
        in_state(CoffeeMachineState::Inactive).xnor(in_state(TeaKettleState::Inactive))
    ),
);
```

Equivalent logic can be achieved by using `not` in concert with `xor`:

[ⓘ](# "This example deliberately fails to compile")

```rust
app.add_systems(
    take_drink_orders.run_if(
        not(in_state(CoffeeMachineState::Inactive).xor(in_state(TeaKettleState::Inactive)))
    ),
);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#634)

#### fn [xor](#method.xor)<M, C>( self, other: C, ) -> [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<XorMarker, Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <C as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

where C: [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M, In>,

Returns a new run condition that only returns `true` if either `self` or `xor` return `true`, but not both.

The returned run condition is eagerly evaluated, meaning it will always execute both run conditions in order.

##### Examples

[ⓘ](# "This example deliberately fails to compile")

```rust
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoffeeMachineState {
    Heating,
    Brewing,
    Inactive,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TeaKettleState {
    Heating,
    Steeping,
    Inactive,
}

app.add_systems(
    // The prepare_beverage system will only execute if either the `in_state(CoffeeMachineState::Inactive)`
    // run condition or `in_state(TeaKettleState::Inactive)` run condition evaluates to `true`,
    // but not both.
    prepare_beverage.run_if(
        in_state(CoffeeMachineState::Inactive).xor(in_state(TeaKettleState::Inactive))
    ),
);
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#642-643)

### impl<Marker, In, F> [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<Marker, In> for F

where In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput"), F: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), Marker>, <F as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), Marker>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"): [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem"),