[bevy](../../index.html)::[ecs](../index.html)::[change\_detection](index.html)

# Trait DetectChangesMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#121)

```rust
pub trait DetectChangesMut: DetectChanges {
    type Inner: ?Sized;

    // Required methods
    fn set_changed(&mut self);
    fn set_added(&mut self);
    fn set_last_changed(&mut self, last_changed: Tick);
    fn set_last_added(&mut self, last_added: Tick);
    fn bypass_change_detection(&mut self) -> &mut Self::Inner;

    // Provided methods
    fn set_if_neq(&mut self, value: Self::Inner) -> bool
       where Self::Inner: Sized + PartialEq { ... }
    fn replace_if_neq(&mut self, value: Self::Inner) -> Option<Self::Inner>
       where Self::Inner: Sized + PartialEq { ... }
    fn clone_from_if_neq<T>(&mut self, value: &T) -> bool
       where T: ToOwned<Owned = Self::Inner> + ?Sized,
             Self::Inner: PartialEq<T> { ... }
}
```

Types that implement reliable change detection.

### Example

Using types that implement [`DetectChangesMut`](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut"), such as [`ResMut`](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut"), provide a way to query if a value has been mutated in another system. Normally change detection is triggered by either [`DerefMut`](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") or [`AsMut`](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut"), however it can be manually triggered via [`set_changed`](../../prelude/trait.DetectChangesMut.html#tymethod.set_changed "method bevy::prelude::DetectChangesMut::set_changed").

To ensure that changes are only triggered when the value actually differs, check if the value would change before assignment, such as by checking that `new != old`. You must be _sure_ that you are not mutably dereferencing in this process.

[`set_if_neq`](../../prelude/trait.DetectChangesMut.html#method.set_if_neq "method bevy::prelude::DetectChangesMut::set_if_neq") is a helper method for this common functionality.

```rust
use bevy_ecs::prelude::*;

#[derive(Resource)]
struct MyResource(u32);

fn my_system(mut resource: ResMut<MyResource>) {
    if resource.is_changed() {
        println!("My resource was mutated!");
    }

   resource.0 = 42; // triggers change detection via [`DerefMut`]
}
```

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#125)

#### type [Inner](#associatedtype.Inner): ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized")

The type contained within this smart pointer

For example, for `ResMut<T>` this would be `T`.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#133)

#### fn [set\_changed](#tymethod.set_changed)(&mut self)

Flags this value as having been changed.

Mutably accessing this smart pointer will automatically flag this value as having been changed. However, mutation through interior mutability requires manual reporting.

**Note**: This operation cannot be undone.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#142)

#### fn [set\_added](#tymethod.set_added)(&mut self)

Flags this value as having been added.

It is not normally necessary to call this method. The ‘added’ tick is set when the value is first added, and is not normally changed afterwards.

**Note**: This operation cannot be undone.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#150)

#### fn [set\_last\_changed](#tymethod.set_last_changed)(&mut self, last\_changed: [Tick](struct.Tick.html "struct bevy::ecs::change_detection::Tick"))

Manually sets the change tick recording the time when this data was last mutated.

##### Warning

This is a complex and error-prone operation, primarily intended for use with rollback networking strategies. If you merely want to flag this data as changed, use [`set_changed`](../../prelude/trait.DetectChangesMut.html#tymethod.set_changed "method bevy::prelude::DetectChangesMut::set_changed") instead. If you want to avoid triggering change detection, use [`bypass_change_detection`](../../prelude/trait.DetectChangesMut.html#tymethod.bypass_change_detection "method bevy::prelude::DetectChangesMut::bypass_change_detection") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#156)

#### fn [set\_last\_added](#tymethod.set_last_added)(&mut self, last\_added: [Tick](struct.Tick.html "struct bevy::ecs::change_detection::Tick"))

Manually sets the added tick recording the time when this data was last added.

##### Warning

The caveats of [`set_last_changed`](../../prelude/trait.DetectChangesMut.html#tymethod.set_last_changed "method bevy::prelude::DetectChangesMut::set_last_changed") apply. This modifies both the added and changed ticks together.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#165)

#### fn [bypass\_change\_detection](#tymethod.bypass_change_detection)(&mut self) -> &mut Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")

Manually bypasses change detection, allowing you to mutate the underlying value without updating the change tick.

##### Warning

This is a risky operation, that can have unexpected consequences on any system relying on this code. However, it can be an essential escape hatch when, for example, you are trying to synchronize representations using change detection and need to avoid infinite recursion.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#215-217)

#### fn [set\_if\_neq](#method.set_if_neq)(&mut self, value: Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Overwrites this smart pointer with the given value, if and only if `*self != value`. Returns `true` if the value was overwritten, and returns `false` if it was not.

This is useful to ensure change detection is only triggered when the underlying value changes, instead of every time it is mutably accessed.

If you’re dealing with non-trivial structs which have multiple fields of non-trivial size, then consider applying a `map_unchanged` beforehand to allow changing only the relevant field and prevent unnecessary copying and cloning. See the docs of [`Mut::map_unchanged`](../../prelude/struct.Mut.html#method.map_unchanged "method bevy::prelude::Mut::map_unchanged"), [`MutUntyped::map_unchanged`](struct.MutUntyped.html#method.map_unchanged "method bevy::ecs::change_detection::MutUntyped::map_unchanged"), [`ResMut::map_unchanged`](../../prelude/struct.ResMut.html#method.map_unchanged "method bevy::prelude::ResMut::map_unchanged") or [`NonSendMut::map_unchanged`](../../prelude/struct.NonSendMut.html#method.map_unchanged "method bevy::prelude::NonSendMut::map_unchanged") for an example

If you need the previous value, use [`replace_if_neq`](../../prelude/trait.DetectChangesMut.html#method.replace_if_neq "method bevy::prelude::DetectChangesMut::replace_if_neq").

##### Examples

```rust
#[derive(Resource, PartialEq, Eq)]
pub struct Score(u32);

fn reset_score(mut score: ResMut<Score>) {
    // Set the score to zero, unless it is already zero.
    score.set_if_neq(Score(0));
}
```

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/window/persisting\_window\_settings.rs ([lines 116-126](../../../src/persisting_window_settings/persisting_window_settings.rs.html#116-126))

```rust
115fn store_window_settings(mut window_settings: ResMut<WindowSettings>, window: &Window) -> bool {
116    window_settings.set_if_neq(WindowSettings {
117        position: match window.position {
118            WindowPosition::At(pos) => Some(pos),
119            _ => None,
120        },
121        size: Some(UVec2::new(
122            window.resolution.width() as u32,
123            window.resolution.height() as u32,
124        )),
125        fullscreen: window.mode != WindowMode::Windowed,
126    })
127}
```

Hide additional examples

examples/ecs/change\_detection.rs ([line 44](../../../src/change_detection/change_detection.rs.html#44))

```rust
35fn change_component(time: Res<Time>, mut query: Query<(Entity, &mut MyComponent)>) {
36    for (entity, mut component) in &mut query {
37        if rand::rng().random_bool(0.1) {
38            let new_component = MyComponent(time.elapsed_secs().round());
39            info!("New value: {new_component:?} {entity}");
40            // Change detection occurs on mutable dereference, and does not consider whether or not
41            // a value is actually equal. To avoid triggering change detection when nothing has
42            // actually changed, you can use the `set_if_neq` method on any component or resource
43            // that implements PartialEq.
44            component.set_if_neq(new_component);
45        }
46    }
47}
48
49/// This is a duplicate of the `change_component` system, added to show that change tracking can
50/// help you find *where* your component is being changed, when there are multiple possible
51/// locations.
52fn change_component_2(time: Res<Time>, mut query: Query<(Entity, &mut MyComponent)>) {
53    for (entity, mut component) in &mut query {
54        if rand::rng().random_bool(0.1) {
55            let new_component = MyComponent(time.elapsed_secs().round());
56            info!("New value: {new_component:?} {entity}");
57            component.set_if_neq(new_component);
58        }
59    }
60}
61
62/// Change detection concepts for components apply similarly to resources.
63fn change_resource(time: Res<Time>, mut my_resource: ResMut<MyResource>) {
64    if rand::rng().random_bool(0.1) {
65        let new_resource = MyResource(time.elapsed_secs().round());
66        info!("New value: {new_resource:?}");
67        my_resource.set_if_neq(new_resource);
68    }
69}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#297-299)

#### fn [replace\_if\_neq](#method.replace_if_neq)(&mut self, value: Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")\>

where Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Overwrites this smart pointer with the given value, if and only if `*self != value`, returning the previous value if this occurs.

This is useful to ensure change detection is only triggered when the underlying value changes, instead of every time it is mutably accessed.

If you’re dealing with non-trivial structs which have multiple fields of non-trivial size, then consider applying a `map_unchanged` beforehand to allow changing only the relevant field and prevent unnecessary copying and cloning. See the docs of [`Mut::map_unchanged`](../../prelude/struct.Mut.html#method.map_unchanged "method bevy::prelude::Mut::map_unchanged"), [`MutUntyped::map_unchanged`](struct.MutUntyped.html#method.map_unchanged "method bevy::ecs::change_detection::MutUntyped::map_unchanged"), [`ResMut::map_unchanged`](../../prelude/struct.ResMut.html#method.map_unchanged "method bevy::prelude::ResMut::map_unchanged") or [`NonSendMut::map_unchanged`](../../prelude/struct.NonSendMut.html#method.map_unchanged "method bevy::prelude::NonSendMut::map_unchanged") for an example

If you don’t need the previous value, use [`set_if_neq`](../../prelude/trait.DetectChangesMut.html#method.set_if_neq "method bevy::prelude::DetectChangesMut::set_if_neq").

##### Examples

```rust
#[derive(Resource, PartialEq, Eq)]
pub struct Score(u32);

#[derive(Message, PartialEq, Eq)]
pub struct ScoreChanged {
    current: u32,
    previous: u32,
}

fn reset_score(mut score: ResMut<Score>, mut score_changed: MessageWriter<ScoreChanged>) {
    // Set the score to zero, unless it is already zero.
    let new_score = 0;
    if let Some(Score(previous_score)) = score.replace_if_neq(Score(new_score)) {
        // If `score` change, emit a `ScoreChanged` event.
        score_changed.write(ScoreChanged {
            current: new_score,
            previous: previous_score,
        });
    }
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#345-348)

#### fn [clone\_from\_if\_neq](#method.clone_from_if_neq)<T>(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned")<Owned = Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<T>,

Overwrites this smart pointer with a clone of the given value, if and only if `*self != value`. Returns `true` if the value was overwritten, and returns `false` if it was not.

This method is useful when the caller only has a borrowed form of `Inner`, e.g. when writing a `&str` into a `Mut<String>`.

##### Examples

```rust
#[derive(Resource)]
pub struct Message(String);

fn update_message(mut message: ResMut<Message>) {
    // Set the score to zero, unless it is already zero.
    ResMut::map_unchanged(message, |Message(msg)| msg).clone_from_if_neq("another string");
}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#1217)

### impl<'w, T> [DetectChangesMut](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#1217)

#### type [Inner](#associatedtype.Inner) = T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#626)

### impl<'w, T> [DetectChangesMut](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut") for [NonSendMut](../../prelude/struct.NonSendMut.html "struct bevy::prelude::NonSendMut")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#626)

#### type [Inner](#associatedtype.Inner) = T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

### impl<'w, T> [DetectChangesMut](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

#### type [Inner](#associatedtype.Inner) = T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#1360)

### impl<'w> [DetectChangesMut](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut") for [MutUntyped](struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#1361)

#### type [Inner](#associatedtype.Inner) = [PtrMut](../ptr/struct.PtrMut.html "struct bevy::ecs::ptr::PtrMut")<'w>