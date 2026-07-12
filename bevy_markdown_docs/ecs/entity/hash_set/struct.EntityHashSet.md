[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[hash\_set](index.html)

# Struct EntityHashSet 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#25)

```rust
pub struct EntityHashSet(/* private fields */);
```

A [`HashSet`](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#27)

### impl [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#33)

#### pub const fn [new](#method.new)() -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

Creates an empty `EntityHashSet`.

Equivalent to [`HashSet::with_hasher(EntityHash)`](../../../platform/collections/struct.HashSet.html#method.with_hasher "associated function bevy::platform::collections::HashSet::with_hasher").

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/ecs/relationships.rs ([line 161](../../../../src/relationships/relationships.rs.html#161))

```rust
152    fn check_for_cycles(
153        // We want to check every entity for cycles
154        query_to_check: Query<Entity, With<Targeting>>,
155        // Fetch the names for easier debugging.
156        name_query: Query<&Name>,
157        // The targeting_query allows us to traverse the relationship graph.
158        targeting_query: Query<&Targeting>,
159    ) -> Result<(), TargetingCycle> {
160        for initial_entity in query_to_check.iter() {
161            let mut visited = EntityHashSet::new();
162            let mut targeting_name = name_query.get(initial_entity).unwrap().clone();
163            println!("Checking for cycles starting at {targeting_name}",);
164
165            // There's all sorts of methods like this; check the `Query` docs for more!
166            // This would also be easy to do by just manually checking the `Targeting` component,
167            // and calling `query.get(targeted_entity)` on the entity that it targets in a loop.
168            for targeting in targeting_query.iter_ancestors(initial_entity) {
169                let target_name = name_query.get(targeting).unwrap();
170                println!("{targeting_name} is targeting {target_name}",);
171                targeting_name = target_name.clone();
172
173                if !visited.insert(targeting) {
174                    return Err(TargetingCycle {
175                        initial_entity,
176                        visited,
177                    });
178                }
179            }
180        }
181
182        // If we've checked all the entities and haven't found a cycle, we're good!
183        Ok(())
184    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#42)

#### pub fn [with\_capacity](#method.with_capacity)(n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

Creates an empty `EntityHashSet` with the specified capacity.

Equivalent to [`HashSet::with_capacity_and_hasher(n, EntityHash)`](../../../platform/collections/struct.HashSet.html#method.with_capacity_and_hasher "associated function bevy::platform::collections::HashSet::with_capacity_and_hasher").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#47)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the set contains no elements.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#52)

#### pub const fn [from\_hash\_set](#method.from_hash_set)(set: [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>) -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

Constructs an `EntityHashSet` from an [`HashSet`](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#57)

#### pub fn [into\_inner](#method.into_inner)(self) -> [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

Returns the inner [`HashSet`](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#64)

#### pub fn [drain](#method.drain)(&mut self) -> [Drain](struct.Drain.html "struct bevy::ecs::entity::hash_set::Drain")<'\_> [ⓘ](#)

Clears the set, returning all elements in an iterator.

Equivalent to [`HashSet::drain`](../../../platform/collections/struct.HashSet.html#method.drain "method bevy::platform::collections::HashSet::drain").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#72)

#### pub fn [iter](#method.iter)(&self) -> [Iter](struct.Iter.html "struct bevy::ecs::entity::hash_set::Iter")<'\_> [ⓘ](#)

An iterator visiting all elements in arbitrary order. The iterator element type is `&'a Entity`.

Equivalent to [`HashSet::iter`](../../../platform/collections/struct.HashSet.html#method.iter "method bevy::platform::collections::HashSet::iter").

##### [Examples found in repository](#scraped-examples-1)[?](../../../../scrape-examples-help.html)

examples/ecs/observers.rs ([line 230](../../../../src/observers/observers.rs.html#230))

```rust
221    fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
222        let tile = (
223            (pos.x / CELL_SIZE).floor() as i32,
224            (pos.y / CELL_SIZE).floor() as i32,
225        );
226        let mut nearby = Vec::new();
227        for x in -1..2 {
228            for y in -1..2 {
229                if let Some(mines) = self.map.get(&(tile.0 + x, tile.1 + y)) {
230                    nearby.extend(mines.iter());
231                }
232            }
233        }
234        nearby
235    }
```

Hide additional examples

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 278](../../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#278))

```rust
257    fn on_scene_completed(
258        &mut self,
259        load_context: &mut LoadContext<'_>,
260        _scene: &gltf::Scene,
261        _world_root_id: Entity,
262        world: &mut World,
263    ) {
264        // Create an AnimationGraph from the desired clip
265        let (graph, index) = AnimationGraph::from_clip(self.clip.clone().unwrap());
266        // Store the animation graph as an asset with an arbitrary label
267        // We only have one graph, so this label will be unique
268        let graph_handle =
269            load_context.add_labeled_asset("MyAnimationGraphLabel".to_string(), graph);
270
271        // Create a component that stores a reference to our animation
272        let animation_to_play = AnimationToPlay {
273            graph_handle,
274            index,
275        };
276
277        // Insert the `AnimationToPlay` component on the first animation root
278        let mut entity = world.entity_mut(*self.animation_root_entities.iter().next().unwrap());
279        entity.insert(animation_to_play);
280    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#80)

#### pub fn [extract\_if](#method.extract_if)<F>(&mut self, f: F) -> [ExtractIf](struct.ExtractIf.html "struct bevy::ecs::entity::hash_set::ExtractIf")<'\_, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

Equivalent to [`HashSet::extract_if`](../../../platform/collections/struct.HashSet.html#method.extract_if "method bevy::platform::collections::HashSet::extract_if").

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>>

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#342)

#### pub fn [capacity](#method.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements the set can hold without reallocating.

Refer to [`capacity`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.capacity "method hashbrown::set::HashSet::capacity") for further details.

##### Examples

```rust
let map = HashSet::with_capacity(5);

assert!(map.capacity() >= 5);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#370)

#### pub fn [iter](#method.iter-1)(&self) -> [Iter](../../../platform/collections/hash_set/struct.Iter.html "struct bevy::platform::collections::hash_set::Iter")<'\_, T> [ⓘ](#)

An iterator visiting all elements in arbitrary order. The iterator element type is `&'a T`.

Refer to [`iter`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.iter "method hashbrown::set::HashSet::iter") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");
map.insert("bar");
map.insert("baz");

for value in map.iter() {
    // "foo", "bar", "baz"
    // Note that the above order is not guaranteed
}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#391)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the set.

Refer to [`len`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.len "method hashbrown::set::HashSet::len") for further details.

##### Examples

```rust
let mut map = HashSet::new();

assert_eq!(map.len(), 0);

map.insert("foo");

assert_eq!(map.len(), 1);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#412)

#### pub fn [is\_empty](#method.is_empty-1)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the set contains no elements.

Refer to [`is_empty`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.is_empty "method hashbrown::set::HashSet::is_empty") for further details.

##### Examples

```rust
let mut map = HashSet::new();

assert!(map.is_empty());

map.insert("foo");

assert!(!map.is_empty());
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#439)

#### pub fn [drain](#method.drain-1)(&mut self) -> [Drain](../../../platform/collections/hash_set/struct.Drain.html "struct bevy::platform::collections::hash_set::Drain")<'\_, T> [ⓘ](#)

Clears the set, returning all elements in an iterator.

Refer to [`drain`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.drain "method hashbrown::set::HashSet::drain") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");
map.insert("bar");
map.insert("baz");

for value in map.drain() {
    // "foo", "bar", "baz"
    // Note that the above order is not guaranteed
}

assert!(map.is_empty());
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#463-465)

#### pub fn [retain](#method.retain)<F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Retains only the elements specified by the predicate.

Refer to [`retain`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.retain "method hashbrown::set::HashSet::retain") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");
map.insert("bar");
map.insert("baz");

map.retain(|value| *value == "baz");

assert_eq!(map.len(), 1);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#494-496)

#### pub fn [extract\_if](#method.extract_if-1)<F>(&mut self, f: F) -> [ExtractIf](../../../platform/collections/hash_set/struct.ExtractIf.html "struct bevy::platform::collections::hash_set::ExtractIf")<'\_, T, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

Refer to [`extract_if`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.extract_if "method hashbrown::set::HashSet::extract_if") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");
map.insert("bar");
map.insert("baz");

let extracted = map
    .extract_if(|value| *value == "baz")
    .collect::<Vec<_>>();

assert_eq!(map.len(), 2);
assert_eq!(extracted.len(), 1);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#521)

#### pub fn [clear](#method.clear)(&mut self)

Clears the set, removing all values.

Refer to [`clear`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.clear "method hashbrown::set::HashSet::clear") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");
map.insert("bar");
map.insert("baz");

map.clear();

assert!(map.is_empty());
```

##### [Examples found in repository](#scraped-examples-2)[?](../../../../scrape-examples-help.html)

examples/ui/navigation/directional\_navigation.rs ([line 315](../../../../src/directional_navigation/directional_navigation.rs.html#315))

```rust
310fn process_inputs(
311    mut action_state: ResMut<ActionState>,
312    keyboard_input: Res<ButtonInput<KeyCode>>,
313    gamepad_input: Query<&Gamepad>,
314) {
315    action_state.pressed_actions.clear();
316
317    for action in DirectionalNavigationAction::variants() {
318        if keyboard_input.just_pressed(action.keycode()) {
319            action_state.pressed_actions.insert(action);
320        }
321    }
322
323    for gamepad in gamepad_input.iter() {
324        for action in DirectionalNavigationAction::variants() {
325            if gamepad.just_pressed(action.gamepad_button()) {
326                action_state.pressed_actions.insert(action);
327            }
328        }
329    }
330}
```

Hide additional examples

examples/ui/navigation/directional\_navigation\_overrides.rs ([line 687](../../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#687))

```rust
682fn process_inputs(
683    mut action_state: ResMut<ActionState>,
684    keyboard_input: Res<ButtonInput<KeyCode>>,
685    gamepad_input: Query<&Gamepad>,
686) {
687    action_state.pressed_actions.clear();
688
689    for action in DirectionalNavigationAction::variants() {
690        if keyboard_input.just_pressed(action.keycode()) {
691            action_state.pressed_actions.insert(action);
692        }
693    }
694
695    for gamepad in gamepad_input.iter() {
696        for action in DirectionalNavigationAction::variants() {
697            if gamepad.just_pressed(action.gamepad_button()) {
698                action_state.pressed_actions.insert(action);
699            }
700        }
701    }
702}
```

examples/shader\_advanced/custom\_render\_phase.rs ([line 505](../../../../src/custom_render_phase/custom_render_phase.rs.html#505))

```rust
500fn extract_camera_phases(
501    mut stencil_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
502    cameras: Extract<Query<(Entity, &Camera), With<Camera3d>>>,
503    mut live_entities: Local<HashSet<RetainedViewEntity>>,
504) {
505    live_entities.clear();
506    for (main_entity, camera) in &cameras {
507        if !camera.is_active {
508            continue;
509        }
510        // This is the main camera, so we use the first subview index (0)
511        let retained_view_entity = RetainedViewEntity::new(main_entity.into(), None, 0);
512
513        stencil_phases.prepare_for_new_frame(retained_view_entity);
514        live_entities.insert(retained_view_entity);
515    }
516
517    // Clear out all dead views.
518    stencil_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
519}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#573)

#### pub fn [hasher](#method.hasher)(&self) -> [&S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a reference to the set’s [`BuildHasher`](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher").

Refer to [`hasher`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.hasher "method hashbrown::set::HashSet::hasher") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#618)

#### pub fn [reserve](#method.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserves capacity for at least `additional` more elements to be inserted in the [`HashSet`](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet"). The collection may reserve more space to avoid frequent reallocations.

Refer to [`reserve`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.reserve "method hashbrown::set::HashSet::reserve") for further details.

##### Examples

```rust
let mut map = HashSet::with_capacity(5);

assert!(map.capacity() >= 5);

map.reserve(10);

assert!(map.capacity() - map.len() >= 10);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#643)

#### pub fn [try\_reserve](#method.try_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/enum.TryReserveError.html "enum hashbrown::TryReserveError")\>

Tries to reserve capacity for at least `additional` more elements to be inserted in the given `HashSet<K,V>`. The collection may reserve more space to avoid frequent reallocations.

Refer to [`try_reserve`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.try_reserve "method hashbrown::set::HashSet::try_reserve") for further details.

##### Examples

```rust
let mut map = HashSet::with_capacity(5);

assert!(map.capacity() >= 5);

map.try_reserve(10).expect("Out of Memory!");

assert!(map.capacity() - map.len() >= 10);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#670)

#### pub fn [shrink\_to\_fit](#method.shrink_to_fit)(&mut self)

Shrinks the capacity of the set as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to [`shrink_to_fit`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.shrink_to_fit "method hashbrown::set::HashSet::shrink_to_fit") for further details.

##### Examples

```rust
let mut map = HashSet::with_capacity(5);

map.insert("foo");
map.insert("bar");
map.insert("baz");

assert!(map.capacity() >= 5);

map.shrink_to_fit();

assert_eq!(map.capacity(), 3);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#680)

#### pub fn [shrink\_to](#method.shrink_to)(&mut self, min\_capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shrinks the capacity of the set with a lower limit. It will drop down no lower than the supplied limit while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to [`shrink_to`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.shrink_to "method hashbrown::set::HashSet::shrink_to") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#689)

#### pub fn [difference](#method.difference)<'a>( &'a self, other: &'a [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T, S>, ) -> [Difference](../../../platform/collections/hash_set/struct.Difference.html "struct bevy::platform::collections::hash_set::Difference")<'a, T, S> [ⓘ](#)

Visits the values representing the difference, i.e., the values that are in `self` but not in `other`.

Refer to [`difference`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.difference "method hashbrown::set::HashSet::difference") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#698)

#### pub fn [symmetric\_difference](#method.symmetric_difference)<'a>( &'a self, other: &'a [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T, S>, ) -> [SymmetricDifference](../../../platform/collections/hash_set/struct.SymmetricDifference.html "struct bevy::platform::collections::hash_set::SymmetricDifference")<'a, T, S> [ⓘ](#)

Visits the values representing the symmetric difference, i.e., the values that are in `self` or in `other` but not in both.

Refer to [`symmetric_difference`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.symmetric_difference "method hashbrown::set::HashSet::symmetric_difference") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#707)

#### pub fn [intersection](#method.intersection)<'a>( &'a self, other: &'a [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T, S>, ) -> [Intersection](../../../platform/collections/hash_set/struct.Intersection.html "struct bevy::platform::collections::hash_set::Intersection")<'a, T, S> [ⓘ](#)

Visits the values representing the intersection, i.e., the values that are both in `self` and `other`.

Refer to [`intersection`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.intersection "method hashbrown::set::HashSet::intersection") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#716)

#### pub fn [union](#method.union)<'a>(&'a self, other: &'a [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T, S>) -> [Union](../../../platform/collections/hash_set/struct.Union.html "struct bevy::platform::collections::hash_set::Union")<'a, T, S> [ⓘ](#)

Visits the values representing the union, i.e., all the values in `self` or `other`, without duplicates.

Refer to [`union`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.union "method hashbrown::set::HashSet::union") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#735-737)

#### pub fn [contains](#method.contains)<Q>(&self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns `true` if the set contains a value.

Refer to [`contains`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.contains "method hashbrown::set::HashSet::contains") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");

assert!(map.contains("foo"));
```

##### [Examples found in repository](#scraped-examples-3)[?](../../../../scrape-examples-help.html)

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 251](../../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#251))

```rust
245    fn on_gltf_node(
246        &mut self,
247        _load_context: &mut LoadContext<'_>,
248        gltf_node: &gltf::Node,
249        entity: &mut EntityWorldMut,
250    ) {
251        if self.animation_root_indices.contains(&gltf_node.index()) {
252            self.animation_root_entities.insert(entity.id());
253        }
254    }
```

Hide additional examples

examples/shader\_advanced/custom\_render\_phase.rs ([line 518](../../../../src/custom_render_phase/custom_render_phase.rs.html#518))

```rust
500fn extract_camera_phases(
501    mut stencil_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
502    cameras: Extract<Query<(Entity, &Camera), With<Camera3d>>>,
503    mut live_entities: Local<HashSet<RetainedViewEntity>>,
504) {
505    live_entities.clear();
506    for (main_entity, camera) in &cameras {
507        if !camera.is_active {
508            continue;
509        }
510        // This is the main camera, so we use the first subview index (0)
511        let retained_view_entity = RetainedViewEntity::new(main_entity.into(), None, 0);
512
513        stencil_phases.prepare_for_new_frame(retained_view_entity);
514        live_entities.insert(retained_view_entity);
515    }
516
517    // Clear out all dead views.
518    stencil_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
519}
```

examples/testbed/helpers.rs ([line 26](../../../../src/testbed_2d/helpers.rs.html#26))

```rust
12pub fn switch_scene_in_ci<Scene: States + FreelyMutableState + Next>(
13    mut ci_config: ResMut<CiTestingConfig>,
14    scene: Res<State<Scene>>,
15    mut next_scene: ResMut<NextState<Scene>>,
16    mut scenes_visited: Local<HashSet<Scene>>,
17    frame_count: Res<FrameCount>,
18    captured: RemovedComponents<Captured>,
19) {
20    if scene.is_changed() {
21        // Changed scene! trigger a screenshot in 100 frames
22        ci_config.events.push(CiTestingEventOnFrame(
23            frame_count.0 + 100,
24            CiTestingEvent::NamedScreenshot(format!("{:?}", scene.get())),
25        ));
26        if scenes_visited.contains(scene.get()) {
27            // Exit once all scenes have been screenshotted
28            ci_config.events.push(CiTestingEventOnFrame(
29                frame_count.0 + 1,
30                CiTestingEvent::AppExit,
31            ));
32        }
33        return;
34    }
35
36    if !captured.is_empty() {
37        // Screenshot taken! Switch to the next scene
38        scenes_visited.insert(scene.get().clone());
39        next_scene.set(scene.get().next());
40    }
41}
```

examples/shader\_advanced/compute\_mesh.rs ([line 194](../../../../src/compute_mesh/compute_mesh.rs.html#194))

```rust
173fn prepare_chunks(
174    meshes_to_generate: Query<&GenerateMesh>,
175    mut chunks: ResMut<ChunksToProcess>,
176    pipeline_cache: Res<PipelineCache>,
177    pipeline: Res<ComputePipeline>,
178    mut processed: Local<HashSet<AssetId<Mesh>>>,
179) {
180    // If the pipeline isn't ready, then meshes
181    // won't be processed. So we want to wait until
182    // the pipeline is ready before considering any mesh processed.
183    if pipeline_cache
184        .get_compute_pipeline(pipeline.pipeline)
185        .is_some()
186    {
187        // get the AssetId for each Handle<Mesh>
188        // which we'll use later to get the relevant buffers
189        // from the mesh_allocator
190        let chunk_data: Vec<AssetId<Mesh>> = meshes_to_generate
191            .iter()
192            .filter_map(|gmesh| {
193                let id = gmesh.0.id();
194                processed.contains(&id).not().then_some(id)
195            })
196            .collect();
197
198        // Cache any meshes we're going to process this frame
199        for id in &chunk_data {
200            processed.insert(*id);
201        }
202
203        chunks.0 = chunk_data;
204    }
205}
```

examples/ui/navigation/directional\_navigation.rs ([line 338](../../../../src/directional_navigation/directional_navigation.rs.html#338))

```rust
332fn navigate(
333    action_state: Res<ActionState>,
334    mut auto_directional_navigator: AutoDirectionalNavigator,
335) {
336    let net_east_west = action_state
337        .pressed_actions
338        .contains(&DirectionalNavigationAction::Right) as i8
339        - action_state
340            .pressed_actions
341            .contains(&DirectionalNavigationAction::Left) as i8;
342
343    let net_north_south = action_state
344        .pressed_actions
345        .contains(&DirectionalNavigationAction::Up) as i8
346        - action_state
347            .pressed_actions
348            .contains(&DirectionalNavigationAction::Down) as i8;
349
350    // Use Dir2::from_xy to convert input to direction, then convert to CompassOctant
351    let maybe_direction = Dir2::from_xy(net_east_west as f32, net_north_south as f32)
352        .ok()
353        .map(CompassOctant::from);
354
355    if let Some(direction) = maybe_direction {
356        match auto_directional_navigator.navigate(direction) {
357            Ok(_entity) => {
358                // Successfully navigated
359            }
360            Err(_e) => {
361                // Navigation failed (no neighbor in that direction)
362            }
363        }
364    }
365}
366
367fn update_focus_display(
368    input_focus: Res<InputFocus>,
369    button_query: Query<&Name, With<Button>>,
370    mut display_query: Query<&mut Text, With<FocusDisplay>>,
371) {
372    if let Ok(mut text) = display_query.single_mut() {
373        if let Some(focused_entity) = input_focus.get() {
374            if let Ok(name) = button_query.get(focused_entity) {
375                **text = format!("Focused: {}", name);
376            } else {
377                **text = "Focused: Unknown".to_string();
378            }
379        } else {
380            **text = "Focused: None".to_string();
381        }
382    }
383}
384
385fn update_key_display(
386    keyboard_input: Res<ButtonInput<KeyCode>>,
387    gamepad_input: Query<&Gamepad>,
388    mut display_query: Query<&mut Text, With<KeyDisplay>>,
389) {
390    if let Ok(mut text) = display_query.single_mut() {
391        // Check for keyboard inputs
392        for action in DirectionalNavigationAction::variants() {
393            if keyboard_input.just_pressed(action.keycode()) {
394                let key_name = match action {
395                    DirectionalNavigationAction::Up => "Up Arrow",
396                    DirectionalNavigationAction::Down => "Down Arrow",
397                    DirectionalNavigationAction::Left => "Left Arrow",
398                    DirectionalNavigationAction::Right => "Right Arrow",
399                    DirectionalNavigationAction::Select => "Enter",
400                };
401                **text = format!("Last Key: {}", key_name);
402                return;
403            }
404        }
405
406        // Check for gamepad inputs
407        for gamepad in gamepad_input.iter() {
408            for action in DirectionalNavigationAction::variants() {
409                if gamepad.just_pressed(action.gamepad_button()) {
410                    let button_name = match action {
411                        DirectionalNavigationAction::Up => "D-Pad Up",
412                        DirectionalNavigationAction::Down => "D-Pad Down",
413                        DirectionalNavigationAction::Left => "D-Pad Left",
414                        DirectionalNavigationAction::Right => "D-Pad Right",
415                        DirectionalNavigationAction::Select => "A Button",
416                    };
417                    **text = format!("Last Key: {}", button_name);
418                    return;
419                }
420            }
421        }
422    }
423}
424
425fn highlight_focused_element(
426    input_focus: Res<InputFocus>,
427    input_focus_visible: Res<InputFocusVisible>,
428    mut query: Query<(Entity, &mut BorderColor)>,
429) {
430    for (entity, mut border_color) in query.iter_mut() {
431        if input_focus.get() == Some(entity) && input_focus_visible.0 {
432            *border_color = BorderColor::all(FOCUSED_BORDER);
433        } else {
434            *border_color = BorderColor::DEFAULT;
435        }
436    }
437}
438
439fn interact_with_focused_button(
440    action_state: Res<ActionState>,
441    input_focus: Res<InputFocus>,
442    mut commands: Commands,
443) {
444    if action_state
445        .pressed_actions
446        .contains(&DirectionalNavigationAction::Select)
447        && let Some(focused_entity) = input_focus.get()
448    {
449        commands.trigger(Pointer::new(
450            PointerId::Mouse,
451            Location {
452                target: NormalizedRenderTarget::None {
453                    width: 0,
454                    height: 0,
455                },
456                position: Vec2::ZERO,
457            },
458            Click {
459                button: PointerButton::Primary,
460                hit: HitData {
461                    camera: Entity::PLACEHOLDER,
462                    depth: 0.0,
463                    position: None,
464                    normal: None,
465                    extra: None,
466                },
467                count: 1,
468                duration: Duration::from_secs_f32(0.1),
469            },
470            focused_entity,
471        ));
472    }
473}
```

examples/ui/navigation/directional\_navigation\_overrides.rs ([line 712](../../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#712))

```rust
704fn navigate(
705    action_state: Res<ActionState>,
706    parent_query: Query<&ChildOf>,
707    mut visibility_query: Query<&mut Visibility>,
708    mut auto_directional_navigator: AutoDirectionalNavigator,
709) {
710    let net_east_west = action_state
711        .pressed_actions
712        .contains(&DirectionalNavigationAction::Right) as i8
713        - action_state
714            .pressed_actions
715            .contains(&DirectionalNavigationAction::Left) as i8;
716
717    let net_north_south = action_state
718        .pressed_actions
719        .contains(&DirectionalNavigationAction::Up) as i8
720        - action_state
721            .pressed_actions
722            .contains(&DirectionalNavigationAction::Down) as i8;
723
724    // Use Dir2::from_xy to convert input to direction, then convert to CompassOctant
725    let maybe_direction = Dir2::from_xy(net_east_west as f32, net_north_south as f32)
726        .ok()
727        .map(CompassOctant::from);
728
729    // Store the previous focus in case navigation switches pages.
730    let previous_focus = auto_directional_navigator.input_focus();
731    if let Some(direction) = maybe_direction {
732        match auto_directional_navigator.navigate(direction) {
733            Ok(new_focus) => {
734                // Successfully navigated!
735
736                // If navigation switches between pages, change the visibilities of pages
737                if let Ok(current_child_of) = parent_query.get(new_focus)
738                    && let Ok(mut current_page_visibility) =
739                        visibility_query.get_mut(current_child_of.parent())
740                {
741                    *current_page_visibility = Visibility::Visible;
742
743                    if let Some(previous_focus_entity) = previous_focus
744                        && let Ok(previous_child_of) = parent_query.get(previous_focus_entity)
745                        && previous_child_of.parent() != current_child_of.parent()
746                        && let Ok(mut previous_page_visibility) =
747                            visibility_query.get_mut(previous_child_of.parent())
748                    {
749                        *previous_page_visibility = Visibility::Hidden;
750                    }
751                }
752            }
753            Err(_e) => {
754                // Navigation failed (no neighbor in that direction)
755            }
756        }
757    }
758}
759
760fn update_focus_display(
761    input_focus: Res<InputFocus>,
762    button_query: Query<&Name, With<Button>>,
763    mut display_query: Query<&mut Text, With<FocusDisplay>>,
764) {
765    if let Ok(mut text) = display_query.single_mut() {
766        if let Some(focused_entity) = input_focus.get() {
767            if let Ok(name) = button_query.get(focused_entity) {
768                **text = format!("Focused: {}", name);
769            } else {
770                **text = "Focused: Unknown".to_string();
771            }
772        } else {
773            **text = "Focused: None".to_string();
774        }
775    }
776}
777
778fn update_key_display(
779    keyboard_input: Res<ButtonInput<KeyCode>>,
780    gamepad_input: Query<&Gamepad>,
781    mut display_query: Query<&mut Text, With<KeyDisplay>>,
782) {
783    if let Ok(mut text) = display_query.single_mut() {
784        // Check for keyboard inputs
785        for action in DirectionalNavigationAction::variants() {
786            if keyboard_input.just_pressed(action.keycode()) {
787                let key_name = match action {
788                    DirectionalNavigationAction::Up => "Up Arrow",
789                    DirectionalNavigationAction::Down => "Down Arrow",
790                    DirectionalNavigationAction::Left => "Left Arrow",
791                    DirectionalNavigationAction::Right => "Right Arrow",
792                    DirectionalNavigationAction::Select => "Enter",
793                };
794                **text = format!("Last Key: {}", key_name);
795                return;
796            }
797        }
798
799        // Check for gamepad inputs
800        for gamepad in gamepad_input.iter() {
801            for action in DirectionalNavigationAction::variants() {
802                if gamepad.just_pressed(action.gamepad_button()) {
803                    let button_name = match action {
804                        DirectionalNavigationAction::Up => "D-Pad Up",
805                        DirectionalNavigationAction::Down => "D-Pad Down",
806                        DirectionalNavigationAction::Left => "D-Pad Left",
807                        DirectionalNavigationAction::Right => "D-Pad Right",
808                        DirectionalNavigationAction::Select => "A Button",
809                    };
810                    **text = format!("Last Key: {}", button_name);
811                    return;
812                }
813            }
814        }
815    }
816}
817
818fn highlight_focused_element(
819    input_focus: Res<InputFocus>,
820    input_focus_visible: Res<InputFocusVisible>,
821    mut query: Query<(Entity, &mut BorderColor, &Page)>,
822) {
823    for (entity, mut border_color, page) in query.iter_mut() {
824        if input_focus.get() == Some(entity) && input_focus_visible.0 {
825            *border_color = BorderColor::all(FOCUSED_BORDER_COLORS[page.0]);
826        } else {
827            *border_color = BorderColor::DEFAULT;
828        }
829    }
830}
831
832fn interact_with_focused_button(
833    action_state: Res<ActionState>,
834    input_focus: Res<InputFocus>,
835    mut commands: Commands,
836) {
837    if action_state
838        .pressed_actions
839        .contains(&DirectionalNavigationAction::Select)
840        && let Some(focused_entity) = input_focus.get()
841    {
842        commands.trigger(Pointer::new(
843            PointerId::Mouse,
844            Location {
845                target: NormalizedRenderTarget::None {
846                    width: 0,
847                    height: 0,
848                },
849                position: Vec2::ZERO,
850            },
851            Click {
852                button: PointerButton::Primary,
853                hit: HitData {
854                    camera: Entity::PLACEHOLDER,
855                    depth: 0.0,
856                    position: None,
857                    normal: None,
858                    extra: None,
859                },
860                count: 1,
861                duration: Duration::from_secs_f32(0.1),
862            },
863            focused_entity,
864        ));
865    }
866}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#757-759)

#### pub fn [get](#method.get)<Q>(&self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a reference to the value in the set, if any, that is equal to the given value.

Refer to [`get`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.get "method hashbrown::set::HashSet::get") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");

assert_eq!(map.get("foo"), Some(&"foo"));
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#778)

#### pub fn [get\_or\_insert](#method.get_or_insert)(&mut self, value: T) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Inserts the given `value` into the set if it is not present, then returns a reference to the value in the set.

Refer to [`get_or_insert`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.get_or_insert "method hashbrown::set::HashSet::get_or_insert") for further details.

##### Examples

```rust
let mut map = HashSet::new();

assert_eq!(map.get_or_insert("foo"), &"foo");
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#796-799)

#### pub fn [get\_or\_insert\_with](#method.get_or_insert_with)<Q, F>(&mut self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html), f: F) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> T,

Inserts a value computed from `f` into the set if the given `value` is not present, then returns a reference to the value in the set.

Refer to [`get_or_insert_with`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.get_or_insert_with "method hashbrown::set::HashSet::get_or_insert_with") for further details.

##### Examples

```rust
let mut map = HashSet::new();

assert_eq!(map.get_or_insert_with(&"foo", |_| "foo"), &"foo");
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#819)

#### pub fn [entry](#method.entry)(&mut self, value: T) -> [Entry](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/enum.Entry.html "enum hashbrown::set::Entry")<'\_, T, S>

Gets the given value’s corresponding entry in the set for in-place manipulation.

Refer to [`entry`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.entry "method hashbrown::set::HashSet::entry") for further details.

##### Examples

```rust
let mut map = HashSet::new();

let value = map.entry("foo").or_insert();
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#828)

#### pub fn [is\_disjoint](#method.is_disjoint)(&self, other: &[HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if `self` has no elements in common with `other`. This is equivalent to checking for an empty intersection.

Refer to [`is_disjoint`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.is_disjoint "method hashbrown::set::HashSet::is_disjoint") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#837)

#### pub fn [is\_subset](#method.is_subset)(&self, other: &[HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the set is a subset of another, i.e., `other` contains at least all the values in `self`.

Refer to [`is_subset`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.is_subset "method hashbrown::set::HashSet::is_subset") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#846)

#### pub fn [is\_superset](#method.is_superset)(&self, other: &[HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the set is a superset of another, i.e., `self` contains at least all the values in `other`.

Refer to [`is_superset`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.is_superset "method hashbrown::set::HashSet::is_superset") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#865)

#### pub fn [insert](#method.insert)(&mut self, value: T) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Adds a value to the set.

Refer to [`insert`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.insert "method hashbrown::set::HashSet::insert") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");

assert!(map.contains("foo"));
```

##### [Examples found in repository](#scraped-examples-4)[?](../../../../scrape-examples-help.html)

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 252](../../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#252))

```rust
245    fn on_gltf_node(
246        &mut self,
247        _load_context: &mut LoadContext<'_>,
248        gltf_node: &gltf::Node,
249        entity: &mut EntityWorldMut,
250    ) {
251        if self.animation_root_indices.contains(&gltf_node.index()) {
252            self.animation_root_entities.insert(entity.id());
253        }
254    }
```

Hide additional examples

examples/ecs/observers.rs ([line 148](../../../../src/observers/observers.rs.html#148))

```rust
142fn on_add_mine(add: On<Add, Mine>, query: Query<&Mine>, mut index: ResMut<SpatialIndex>) {
143    let mine = query.get(add.entity).unwrap();
144    let tile = (
145        (mine.pos.x / CELL_SIZE).floor() as i32,
146        (mine.pos.y / CELL_SIZE).floor() as i32,
147    );
148    index.map.entry(tile).or_default().insert(add.entity);
149}
```

examples/shader\_advanced/manual\_material.rs ([line 316](../../../../src/manual_material/manual_material.rs.html#316))

```rust
307fn extract_image_materials_needing_specialization(
308    entities_needing_specialization: Extract<Res<EntitiesNeedingSpecialization<ImageMaterial>>>,
309    mut dirty_specializations: ResMut<DirtySpecializations>,
310) {
311    // Drain the list of entities needing specialization from the main world
312    // into the render-world `DirtySpecializations` table.
313    for entity in entities_needing_specialization.changed.iter() {
314        dirty_specializations
315            .changed_renderables
316            .insert(MainEntity::from(*entity));
317    }
318}
319
320/// A system that adds entities that were judged to need their specializations
321/// removed to the appropriate table in [`DirtySpecializations`].
322fn extract_image_materials_that_need_specializations_removed(
323    entities_needing_specialization: Extract<Res<EntitiesNeedingSpecialization<ImageMaterial>>>,
324    mut dirty_specializations: ResMut<DirtySpecializations>,
325) {
326    for entity in entities_needing_specialization.removed.iter() {
327        dirty_specializations
328            .removed_renderables
329            .insert(MainEntity::from(*entity));
330    }
331}
```

examples/ui/navigation/directional\_navigation.rs ([line 319](../../../../src/directional_navigation/directional_navigation.rs.html#319))

```rust
310fn process_inputs(
311    mut action_state: ResMut<ActionState>,
312    keyboard_input: Res<ButtonInput<KeyCode>>,
313    gamepad_input: Query<&Gamepad>,
314) {
315    action_state.pressed_actions.clear();
316
317    for action in DirectionalNavigationAction::variants() {
318        if keyboard_input.just_pressed(action.keycode()) {
319            action_state.pressed_actions.insert(action);
320        }
321    }
322
323    for gamepad in gamepad_input.iter() {
324        for action in DirectionalNavigationAction::variants() {
325            if gamepad.just_pressed(action.gamepad_button()) {
326                action_state.pressed_actions.insert(action);
327            }
328        }
329    }
330}
```

examples/ui/navigation/directional\_navigation\_overrides.rs ([line 691](../../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#691))

```rust
682fn process_inputs(
683    mut action_state: ResMut<ActionState>,
684    keyboard_input: Res<ButtonInput<KeyCode>>,
685    gamepad_input: Query<&Gamepad>,
686) {
687    action_state.pressed_actions.clear();
688
689    for action in DirectionalNavigationAction::variants() {
690        if keyboard_input.just_pressed(action.keycode()) {
691            action_state.pressed_actions.insert(action);
692        }
693    }
694
695    for gamepad in gamepad_input.iter() {
696        for action in DirectionalNavigationAction::variants() {
697            if gamepad.just_pressed(action.gamepad_button()) {
698                action_state.pressed_actions.insert(action);
699            }
700        }
701    }
702}
```

examples/shader\_advanced/custom\_render\_phase.rs ([line 514](../../../../src/custom_render_phase/custom_render_phase.rs.html#514))

```rust
500fn extract_camera_phases(
501    mut stencil_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
502    cameras: Extract<Query<(Entity, &Camera), With<Camera3d>>>,
503    mut live_entities: Local<HashSet<RetainedViewEntity>>,
504) {
505    live_entities.clear();
506    for (main_entity, camera) in &cameras {
507        if !camera.is_active {
508            continue;
509        }
510        // This is the main camera, so we use the first subview index (0)
511        let retained_view_entity = RetainedViewEntity::new(main_entity.into(), None, 0);
512
513        stencil_phases.prepare_for_new_frame(retained_view_entity);
514        live_entities.insert(retained_view_entity);
515    }
516
517    // Clear out all dead views.
518    stencil_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
519}
520
521/// A resource that stores meshes that couldn't be specialized yet because their
522/// materials hadn't loaded.
523///
524/// See the documentation for [`PendingQueues`] for more information.
525#[derive(Default, Deref, DerefMut, Resource)]
526struct PendingCustomMeshQueues(pub PendingQueues);
527
528// This is a very important step when writing a custom phase.
529//
530// This system determines which meshes will be added to the phase.
531fn queue_custom_meshes(
532    custom_draw_functions: Res<DrawFunctions<Stencil3d>>,
533    mut pipelines: ResMut<SpecializedMeshPipelines<StencilPipeline>>,
534    pipeline_cache: Res<PipelineCache>,
535    custom_draw_pipeline: Res<StencilPipeline>,
536    render_meshes: Res<RenderAssets<RenderMesh>>,
537    render_mesh_instances: Res<RenderMeshInstances>,
538    maybe_batched_instance_buffers: Option<
539        Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
540    >,
541    mut custom_render_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
542    mut views: Query<(&ExtractedView, &RenderVisibleEntities)>,
543    view_key_cache: Res<ViewKeyCache>,
544    dirty_specializations: Res<DirtySpecializations>,
545    mut pending_custom_mesh_queues: ResMut<PendingCustomMeshQueues>,
546    has_marker: Query<(), With<DrawStencil>>,
547) {
548    for (view, visible_entities) in &mut views {
549        let Some(custom_phase) = custom_render_phases.get_mut(&view.retained_view_entity) else {
550            continue;
551        };
552        let draw_custom = custom_draw_functions.read().id::<DrawMesh3dStencil>();
553
554        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
555            continue;
556        };
557
558        // Since our phase can work on any 3d mesh we can reuse the default mesh 3d filter
559        let Some(render_visible_mesh_entities) = visible_entities.get::<Mesh3d>() else {
560            continue;
561        };
562
563        let view_pending_custom_mesh_queues =
564            pending_custom_mesh_queues.prepare_for_new_frame(view.retained_view_entity);
565
566        // First, remove meshes that need to be respecialized, and those that were removed, from the bins.
567        for &main_entity in dirty_specializations
568            .iter_to_dequeue(view.retained_view_entity, render_visible_mesh_entities)
569        {
570            custom_phase.remove(Entity::PLACEHOLDER, main_entity);
571        }
572
573        for (render_entity, visible_entity) in dirty_specializations.iter_to_queue(
574            view.retained_view_entity,
575            render_visible_mesh_entities,
576            &view_pending_custom_mesh_queues.prev_frame,
577        ) {
578            // We only want meshes with the marker component to be queued to our phase.
579            if has_marker.get(*render_entity).is_err() {
580                continue;
581            }
582            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*visible_entity)
583            else {
584                // We couldn't fetch the mesh, probably because it hasn't been
585                // loaded yet. Add the entity to the list of pending custom mesh
586                // queues and bail.
587                view_pending_custom_mesh_queues
588                    .current_frame
589                    .insert((*render_entity, *visible_entity));
590                continue;
591            };
592            let Some(mesh) = render_meshes.get(mesh_instance.mesh_asset_id()) else {
593                continue;
594            };
595
596            // Specialize the key for the current mesh entity
597            // For this example we only specialize based on the mesh topology
598            // but you could have more complex keys and that's where you'd need to create those keys
599            let mut mesh_key = view_key;
600            mesh_key |= MeshPipelineKey::from_primitive_topology_and_strip_index(
601                mesh.primitive_topology(),
602                mesh.index_format(),
603            );
604
605            let pipeline_id = pipelines.specialize(
606                &pipeline_cache,
607                &custom_draw_pipeline,
608                mesh_key,
609                &mesh.layout,
610            );
611            let pipeline_id = match pipeline_id {
612                Ok(id) => id,
613                Err(err) => {
614                    error!("{}", err);
615                    continue;
616                }
617            };
618            // At this point we have all the data we need to create a phase item and add it to our
619            // phase
620            custom_phase.add_retained(Stencil3d {
621                sorting_info: TransparentSortingInfo3d::Sorted {
622                    mesh_center: pbr::get_mesh_instance_world_from_local(
623                        *visible_entity,
624                        mesh_instance.current_uniform_index,
625                        &render_mesh_instances,
626                        maybe_batched_instance_buffers.as_deref(),
627                    )
628                    .transform_point3(
629                        render_meshes
630                            .get(mesh_instance.mesh_asset_id())
631                            .unwrap()
632                            .aabb_center,
633                    ),
634                    depth_bias: 0.0,
635                },
636                distance: FloatOrd(0.0),
637                entity: (Entity::PLACEHOLDER, *visible_entity),
638                pipeline: pipeline_id,
639                draw_function: draw_custom,
640                // Sorted phase items aren't batched
641                batch_range: 0..1,
642                extra_index: PhaseItemExtraIndex::None,
643                indexed: mesh.indexed(),
644            });
645        }
646    }
647}
```

Additional examples can be found in:  

*   [examples/testbed/helpers.rs](../../../../src/testbed_2d/helpers.rs.html#38)
*   [examples/shader\_advanced/compute\_mesh.rs](../../../../src/compute_mesh/compute_mesh.rs.html#200)
*   [examples/ecs/relationships.rs](../../../../src/relationships/relationships.rs.html#173)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#346)

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#885)

#### pub fn [replace](#method.replace)(&mut self, value: T) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Adds a value to the set, replacing the existing value, if any, that is equal to the given one. Returns the replaced value.

Refer to [`replace`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.replace "method hashbrown::set::HashSet::replace") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");

assert_eq!(map.replace("foo"), Some("foo"));
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#907-909)

#### pub fn [remove](#method.remove)<Q>(&mut self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Removes a value from the set. Returns whether the value was present in the set.

Refer to [`remove`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.remove "method hashbrown::set::HashSet::remove") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");

assert!(map.remove("foo"));

assert!(map.is_empty());
```

##### [Examples found in repository](#scraped-examples-5)[?](../../../../scrape-examples-help.html)

examples/ecs/observers.rs ([line 159](../../../../src/observers/observers.rs.html#159))

```rust
152fn on_remove_mine(remove: On<Remove, Mine>, query: Query<&Mine>, mut index: ResMut<SpatialIndex>) {
153    let mine = query.get(remove.entity).unwrap();
154    let tile = (
155        (mine.pos.x / CELL_SIZE).floor() as i32,
156        (mine.pos.y / CELL_SIZE).floor() as i32,
157    );
158    index.map.entry(tile).and_modify(|set| {
159        set.remove(&remove.entity);
160    });
161}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#931-933)

#### pub fn [take](#method.take)<Q>(&mut self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Removes and returns the value in the set, if any, that is equal to the given one.

Refer to [`take`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.take "method hashbrown::set::HashSet::take") for further details.

##### Examples

```rust
let mut map = HashSet::new();

map.insert("foo");

assert_eq!(map.take("foo"), Some("foo"));

assert!(map.is_empty());
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#956)

#### pub fn [allocation\_size](#method.allocation_size)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the total amount of memory allocated internally by the hash set, in bytes.

Refer to [`allocation_size`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.allocation_size "method hashbrown::set::HashSet::allocation_size") for further details.

##### Examples

```rust
let mut map = HashSet::new();

assert_eq!(map.allocation_size(), 0);

map.insert("foo");

assert!(map.allocation_size() >= size_of::<&'static str>());
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#983)

#### pub unsafe fn [insert\_unique\_unchecked](#method.insert_unique_unchecked)(&mut self, value: T) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Insert a value the set without checking if the value already exists in the set.

Refer to [`insert_unique_unchecked`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.insert_unique_unchecked "method hashbrown::set::HashSet::insert_unique_unchecked") for further details.

##### Safety

This operation is safe if a value does not exist in the set.

However, if a value exists in the set already, the behavior is unspecified: this operation may panic, loop forever, or any following operation with the set may panic, loop forever or return arbitrary result.

That said, this operation (and following operations) are guaranteed to not violate memory safety.

However this operation is still unsafe because the resulting `HashSet` may be passed to unsafe code which does expect the set to behave correctly, and would cause unsoundness as a result.

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [HashSet](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<T, S>>

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#269)

#### pub fn [capacity](#method.capacity-1)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements the set can hold without reallocating.

##### Examples

```rust
use hashbrown::HashSet;
let set: HashSet<i32> = HashSet::with_capacity(100);
assert!(set.capacity() >= 100);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#290)

#### pub fn [iter](#method.iter-2)(&self) -> [Iter](../../../platform/collections/hash_set/struct.Iter.html "struct bevy::platform::collections::hash_set::Iter")<'\_, T> [ⓘ](#)

An iterator visiting all elements in arbitrary order. The iterator element type is `&'a T`.

##### Examples

```rust
use hashbrown::HashSet;
let mut set = HashSet::new();
set.insert("a");
set.insert("b");

// Will print in an arbitrary order.
for x in set.iter() {
    println!("{}", x);
}
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#309)

#### pub fn [len](#method.len-1)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the set.

##### Examples

```rust
use hashbrown::HashSet;

let mut v = HashSet::new();
assert_eq!(v.len(), 0);
v.insert(1);
assert_eq!(v.len(), 1);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#326)

#### pub fn [is\_empty](#method.is_empty-2)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the set contains no elements.

##### Examples

```rust
use hashbrown::HashSet;

let mut v = HashSet::new();
assert!(v.is_empty());
v.insert(1);
assert!(!v.is_empty());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#348)

#### pub fn [drain](#method.drain-2)(&mut self) -> [Drain](../../../platform/collections/hash_set/struct.Drain.html "struct bevy::platform::collections::hash_set::Drain")<'\_, T, A> [ⓘ](#)

Clears the set, returning all elements in an iterator.

##### Examples

```rust
use hashbrown::HashSet;

let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
assert!(!set.is_empty());

// print 1, 2, 3 in an arbitrary order
for i in set.drain() {
    println!("{}", i);
}

assert!(set.is_empty());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#368-370)

#### pub fn [retain](#method.retain-1)<F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Retains only the elements specified by the predicate.

In other words, remove all elements `e` such that `f(&e)` returns `false`.

##### Examples

```rust
use hashbrown::HashSet;

let xs = [1,2,3,4,5,6];
let mut set: HashSet<i32> = xs.into_iter().collect();
set.retain(|&k| k % 2 == 0);
assert_eq!(set.len(), 3);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#404-406)

#### pub fn [extract\_if](#method.extract_if-2)<F>(&mut self, f: F) -> [ExtractIf](../../../platform/collections/hash_set/struct.ExtractIf.html "struct bevy::platform::collections::hash_set::ExtractIf")<'\_, T, F, A> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

In other words, move all elements `e` such that `f(&e)` returns `true` out into another iterator.

If the returned `ExtractIf` is not exhausted, e.g. because it is dropped without iterating or the iteration short-circuits, then the remaining elements will be retained. Use [`retain()`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.retain "method hashbrown::set::HashSet::retain") with a negated predicate if you do not need the returned iterator.

##### Examples

```rust
use hashbrown::HashSet;

let mut set: HashSet<i32> = (0..8).collect();
let drained: HashSet<i32> = set.extract_if(|v| v % 2 == 0).collect();

let mut evens = drained.into_iter().collect::<Vec<_>>();
let mut odds = set.into_iter().collect::<Vec<_>>();
evens.sort();
odds.sort();

assert_eq!(evens, vec![0, 2, 4, 6]);
assert_eq!(odds, vec![1, 3, 5, 7]);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#430)

#### pub fn [clear](#method.clear-1)(&mut self)

Clears the set, removing all values.

##### Examples

```rust
use hashbrown::HashSet;

let mut v = HashSet::new();
v.insert(1);
v.clear();
assert!(v.is_empty());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#520)

#### pub fn [allocator](#method.allocator)(&self) -> [&A](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a reference to the underlying allocator.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#616)

#### pub fn [hasher](#method.hasher-1)(&self) -> [&S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a reference to the set’s [`BuildHasher`](https://doc.rust-lang.org/std/hash/trait.BuildHasher.html).

##### Examples

```rust
use hashbrown::HashSet;
use hashbrown::DefaultHashBuilder;

let hasher = DefaultHashBuilder::default();
let set: HashSet<i32> = HashSet::with_hasher(hasher);
let hasher: &DefaultHashBuilder = set.hasher();
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#649)

#### pub fn [reserve](#method.reserve-1)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserves capacity for at least `additional` more elements to be inserted in the `HashSet`. The collection may reserve more space to avoid frequent reallocations.

##### Panics

Panics if the new capacity exceeds [`isize::MAX`](https://doc.rust-lang.org/std/primitive.isize.html) bytes and [`abort`](https://doc.rust-lang.org/alloc/alloc/fn.handle_alloc_error.html) the program in case of allocation error. Use [`try_reserve`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html#method.try_reserve "method hashbrown::set::HashSet::try_reserve") instead if you want to handle memory allocation failure.

##### Examples

```rust
use hashbrown::HashSet;
let mut set: HashSet<i32> = HashSet::new();
set.reserve(10);
assert!(set.capacity() >= 10);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#670)

#### pub fn [try\_reserve](#method.try_reserve-1)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/enum.TryReserveError.html "enum hashbrown::TryReserveError")\>

Tries to reserve capacity for at least `additional` more elements to be inserted in the given `HashSet<K,V>`. The collection may reserve more space to avoid frequent reallocations.

##### Errors

If the capacity overflows, or the allocator reports a failure, then an error is returned.

##### Examples

```rust
use hashbrown::HashSet;
let mut set: HashSet<i32> = HashSet::new();
set.try_reserve(10).expect("why is the test harness OOMing on 10 bytes?");
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#691)

#### pub fn [shrink\_to\_fit](#method.shrink_to_fit-1)(&mut self)

Shrinks the capacity of the set as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

##### Examples

```rust
use hashbrown::HashSet;

let mut set = HashSet::with_capacity(100);
set.insert(1);
set.insert(2);
assert!(set.capacity() >= 100);
set.shrink_to_fit();
assert!(set.capacity() >= 2);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#717)

#### pub fn [shrink\_to](#method.shrink_to-1)(&mut self, min\_capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shrinks the capacity of the set with a lower limit. It will drop down no lower than the supplied limit while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Panics if the current capacity is smaller than the supplied minimum capacity.

##### Examples

```rust
use hashbrown::HashSet;

let mut set = HashSet::with_capacity(100);
set.insert(1);
set.insert(2);
assert!(set.capacity() >= 100);
set.shrink_to(10);
assert!(set.capacity() >= 10);
set.shrink_to(0);
assert!(set.capacity() >= 2);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#745)

#### pub fn [difference](#method.difference-1)<'a>( &'a self, other: &'a [HashSet](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<T, S, A>, ) -> [Difference](../../../platform/collections/hash_set/struct.Difference.html "struct bevy::platform::collections::hash_set::Difference")<'a, T, S, A> [ⓘ](#)

Visits the values representing the difference, i.e., the values that are in `self` but not in `other`.

##### Examples

```rust
use hashbrown::HashSet;
let a: HashSet<_> = [1, 2, 3].into_iter().collect();
let b: HashSet<_> = [4, 2, 3, 4].into_iter().collect();

// Can be seen as `a - b`.
for x in a.difference(&b) {
    println!("{}", x); // Print 1
}

let diff: HashSet<_> = a.difference(&b).collect();
assert_eq!(diff, [1].iter().collect());

// Note that difference is not symmetric,
// and `b - a` means something else:
let diff: HashSet<_> = b.difference(&a).collect();
assert_eq!(diff, [4].iter().collect());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#774)

#### pub fn [symmetric\_difference](#method.symmetric_difference-1)<'a>( &'a self, other: &'a [HashSet](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<T, S, A>, ) -> [SymmetricDifference](../../../platform/collections/hash_set/struct.SymmetricDifference.html "struct bevy::platform::collections::hash_set::SymmetricDifference")<'a, T, S, A> [ⓘ](#)

Visits the values representing the symmetric difference, i.e., the values that are in `self` or in `other` but not in both.

##### Examples

```rust
use hashbrown::HashSet;
let a: HashSet<_> = [1, 2, 3].into_iter().collect();
let b: HashSet<_> = [4, 2, 3, 4].into_iter().collect();

// Print 1, 4 in arbitrary order.
for x in a.symmetric_difference(&b) {
    println!("{}", x);
}

let diff1: HashSet<_> = a.symmetric_difference(&b).collect();
let diff2: HashSet<_> = b.symmetric_difference(&a).collect();

assert_eq!(diff1, diff2);
assert_eq!(diff1, [1, 4].iter().collect());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#799)

#### pub fn [intersection](#method.intersection-1)<'a>( &'a self, other: &'a [HashSet](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<T, S, A>, ) -> [Intersection](../../../platform/collections/hash_set/struct.Intersection.html "struct bevy::platform::collections::hash_set::Intersection")<'a, T, S, A> [ⓘ](#)

Visits the values representing the intersection, i.e., the values that are both in `self` and `other`.

##### Examples

```rust
use hashbrown::HashSet;
let a: HashSet<_> = [1, 2, 3].into_iter().collect();
let b: HashSet<_> = [4, 2, 3, 4].into_iter().collect();

// Print 2, 3 in arbitrary order.
for x in a.intersection(&b) {
    println!("{}", x);
}

let intersection: HashSet<_> = a.intersection(&b).collect();
assert_eq!(intersection, [2, 3].iter().collect());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#830)

#### pub fn [union](#method.union-1)<'a>(&'a self, other: &'a [HashSet](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<T, S, A>) -> [Union](../../../platform/collections/hash_set/struct.Union.html "struct bevy::platform::collections::hash_set::Union")<'a, T, S, A> [ⓘ](#)

Visits the values representing the union, i.e., all the values in `self` or `other`, without duplicates.

##### Examples

```rust
use hashbrown::HashSet;
let a: HashSet<_> = [1, 2, 3].into_iter().collect();
let b: HashSet<_> = [4, 2, 3, 4].into_iter().collect();

// Print 1, 2, 3, 4 in arbitrary order.
for x in a.union(&b) {
    println!("{}", x);
}

let union: HashSet<_> = a.union(&b).collect();
assert_eq!(union, [1, 2, 3, 4].iter().collect());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#862-864)

#### pub fn [contains](#method.contains-1)<Q>(&self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns `true` if the set contains a value.

The value may be any borrowed form of the set’s value type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the value type.

##### Examples

```rust
use hashbrown::HashSet;

let set: HashSet<_> = [1, 2, 3].into_iter().collect();
assert_eq!(set.contains(&1), true);
assert_eq!(set.contains(&4), false);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#888-890)

#### pub fn [get](#method.get-1)<Q>(&self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a reference to the value in the set, if any, that is equal to the given value.

The value may be any borrowed form of the set’s value type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the value type.

##### Examples

```rust
use hashbrown::HashSet;

let set: HashSet<_> = [1, 2, 3].into_iter().collect();
assert_eq!(set.get(&2), Some(&2));
assert_eq!(set.get(&4), None);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#914)

#### pub fn [get\_or\_insert](#method.get_or_insert-1)(&mut self, value: T) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Inserts the given `value` into the set if it is not present, then returns a reference to the value in the set.

##### Examples

```rust
use hashbrown::HashSet;

let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
assert_eq!(set.len(), 3);
assert_eq!(set.get_or_insert(2), &2);
assert_eq!(set.get_or_insert(100), &100);
assert_eq!(set.len(), 4); // 100 was inserted
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#949-952)

#### pub fn [get\_or\_insert\_with](#method.get_or_insert_with-1)<Q, F>(&mut self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html), f: F) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> T,

Inserts a value computed from `f` into the set if the given `value` is not present, then returns a reference to the value in the set.

##### Examples

```rust
use hashbrown::HashSet;

let mut set: HashSet<String> = ["cat", "dog", "horse"]
    .iter().map(|&pet| pet.to_owned()).collect();

assert_eq!(set.len(), 3);
for &pet in &["cat", "dog", "fish"] {
    let value = set.get_or_insert_with(pet, str::to_owned);
    assert_eq!(value, pet);
}
assert_eq!(set.len(), 4); // a new "fish" was inserted
```

The following example will panic because the new value doesn’t match.

[ⓘ](# "This example panics")

```rust
let mut set = hashbrown::HashSet::new();
set.get_or_insert_with("rust", |_| String::new());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1000)

#### pub fn [entry](#method.entry-1)(&mut self, value: T) -> [Entry](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/enum.Entry.html "enum hashbrown::set::Entry")<'\_, T, S, A>

Gets the given value’s corresponding entry in the set for in-place manipulation.

##### Examples

```rust
use hashbrown::HashSet;
use hashbrown::hash_set::Entry::*;

let mut singles = HashSet::new();
let mut dupes = HashSet::new();

for ch in "a short treatise on fungi".chars() {
    if let Vacant(dupe_entry) = dupes.entry(ch) {
        // We haven't already seen a duplicate, so
        // check if we've at least seen it once.
        match singles.entry(ch) {
            Vacant(single_entry) => {
                // We found a new character for the first time.
                single_entry.insert();
            }
            Occupied(single_entry) => {
                // We've already seen this once, "move" it to dupes.
                single_entry.remove();
                dupe_entry.insert();
            }
        }
    }
}

assert!(!singles.contains(&'t') && dupes.contains(&'t'));
assert!(singles.contains(&'u') && !dupes.contains(&'u'));
assert!(!singles.contains(&'v') && !dupes.contains(&'v'));
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1024)

#### pub fn [is\_disjoint](#method.is_disjoint-1)(&self, other: &[HashSet](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<T, S, A>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if `self` has no elements in common with `other`. This is equivalent to checking for an empty intersection.

##### Examples

```rust
use hashbrown::HashSet;

let a: HashSet<_> = [1, 2, 3].into_iter().collect();
let mut b = HashSet::new();

assert_eq!(a.is_disjoint(&b), true);
b.insert(4);
assert_eq!(a.is_disjoint(&b), true);
b.insert(1);
assert_eq!(a.is_disjoint(&b), false);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1045)

#### pub fn [is\_subset](#method.is_subset-1)(&self, other: &[HashSet](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<T, S, A>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the set is a subset of another, i.e., `other` contains at least all the values in `self`.

##### Examples

```rust
use hashbrown::HashSet;

let sup: HashSet<_> = [1, 2, 3].into_iter().collect();
let mut set = HashSet::new();

assert_eq!(set.is_subset(&sup), true);
set.insert(2);
assert_eq!(set.is_subset(&sup), true);
set.insert(4);
assert_eq!(set.is_subset(&sup), false);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1070)

#### pub fn [is\_superset](#method.is_superset-1)(&self, other: &[HashSet](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet")<T, S, A>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the set is a superset of another, i.e., `self` contains at least all the values in `other`.

##### Examples

```rust
use hashbrown::HashSet;

let sub: HashSet<_> = [1, 2].into_iter().collect();
let mut set = HashSet::new();

assert_eq!(set.is_superset(&sub), false);

set.insert(0);
set.insert(1);
assert_eq!(set.is_superset(&sub), false);

set.insert(2);
assert_eq!(set.is_superset(&sub), true);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1092)

#### pub fn [insert](#method.insert-1)(&mut self, value: T) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Adds a value to the set.

If the set did not have this value present, `true` is returned.

If the set did have this value present, `false` is returned.

##### Examples

```rust
use hashbrown::HashSet;

let mut set = HashSet::new();

assert_eq!(set.insert(2), true);
assert_eq!(set.insert(2), false);
assert_eq!(set.len(), 1);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1120)

#### pub unsafe fn [insert\_unique\_unchecked](#method.insert_unique_unchecked-1)(&mut self, value: T) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Insert a value the set without checking if the value already exists in the set.

This operation is faster than regular insert, because it does not perform lookup before insertion.

This operation is useful during initial population of the set. For example, when constructing a set from another set, we know that values are unique.

##### Safety

This operation is safe if a value does not exist in the set.

However, if a value exists in the set already, the behavior is unspecified: this operation may panic, loop forever, or any following operation with the set may panic, loop forever or return arbitrary result.

That said, this operation (and following operations) are guaranteed to not violate memory safety.

However this operation is still unsafe because the resulting `HashSet` may be passed to unsafe code which does expect the set to behave correctly, and would cause unsoundness as a result.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1140)

#### pub fn [replace](#method.replace-1)(&mut self, value: T) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Adds a value to the set, replacing the existing value, if any, that is equal to the given one. Returns the replaced value.

##### Examples

```rust
use hashbrown::HashSet;

let mut set = HashSet::new();
set.insert(Vec::<i32>::new());

assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
set.replace(Vec::with_capacity(10));
assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1175-1177)

#### pub fn [remove](#method.remove-1)<Q>(&mut self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Removes a value from the set. Returns whether the value was present in the set.

The value may be any borrowed form of the set’s value type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the value type.

##### Examples

```rust
use hashbrown::HashSet;

let mut set = HashSet::new();

set.insert(2);
assert_eq!(set.remove(&2), true);
assert_eq!(set.remove(&2), false);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1201-1203)

#### pub fn [take](#method.take-1)<Q>(&mut self, value: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Removes and returns the value in the set, if any, that is equal to the given one.

The value may be any borrowed form of the set’s value type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the value type.

##### Examples

```rust
use hashbrown::HashSet;

let mut set: HashSet<_> = [1, 2, 3].into_iter().collect();
assert_eq!(set.take(&2), Some(2));
assert_eq!(set.take(&2), None);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/set.rs.html#1218)

#### pub fn [allocation\_size](#method.allocation_size-1)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the total amount of memory allocated internally by the hash set, in bytes.

The returned number is informational only. It is intended to be primarily used for memory profiling.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#119)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd") for &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#120)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#122)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")) -> <&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#127)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#128)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#133)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr") for &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#134)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#136)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")) -> <&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#141)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#142)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#147)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor") for &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#148)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#150)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")) -> <&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#155)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#156)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#85)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#86)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#88)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#93)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#94)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#23)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#23)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#175)

### impl<'a> [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<&'a [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#176)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<T>(&mut self, iter: T)

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#181)

### impl [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#182)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<T>(&mut self, iter: T)

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#213)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#214)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [HashSet](../../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>) -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#187)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#188)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: \[[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [FromArg](../../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### type [This](../../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

The type to convert into. [Read more](../../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [from\_arg](../../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)( arg: [Arg](../../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [FromArg](../../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#199)

### impl [FromEntitySetIterator](../trait.FromEntitySetIterator.html "trait bevy::ecs::entity::FromEntitySetIterator")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#200)

#### fn [from\_entity\_set\_iter](../trait.FromEntitySetIterator.html#tymethod.from_entity_set_iter)<I>(set\_iter: I) -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

where I: [EntitySet](../trait.EntitySet.html "trait bevy::ecs::entity::EntitySet")<Item = [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>,

Creates a value from an [`EntitySetIterator`](../trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#193)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#194)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iterable: I) -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [from\_reflect](../../../prelude/trait.FromReflect.html#tymethod.from_reflect)( reflect: &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [GetOwnership](../../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [ownership](../../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [GetTypeRegistration](../../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [get\_type\_registration](../../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [register\_type\_dependencies](../../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#99)

### impl<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#100)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = &'a [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#102)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [Iter](struct.Iter.html "struct bevy::ecs::entity::hash_set::Iter")<'a>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#104)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'a [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#109)

### impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#110)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#112)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [IntoIter](struct.IntoIter.html "struct bevy::ecs::entity::hash_set::IntoIter")

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#114)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [IntoReturn](../../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [into\_return](../../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet"): 'into\_return,

Converts [`Self`](../../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [get\_represented\_type\_info](../../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [try\_apply](../../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [reflect\_kind](../../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [reflect\_ref](../../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [reflect\_mut](../../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [reflect\_owned](../../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\>) -> [ReflectOwned](../../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [try\_into\_reflect](../../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [try\_as\_reflect](../../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [try\_as\_reflect\_mut](../../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [into\_partial\_reflect](../../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [as\_partial\_reflect](../../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [as\_partial\_reflect\_mut](../../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [reflect\_partial\_eq](../../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [reflect\_partial\_cmp](../../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [reflect\_clone](../../../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../../../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../../../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../../../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](../../../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../../../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../../../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../../../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](../../../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../../../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#363)

#### fn [debug](../../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../../../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../../../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [into\_any](../../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [as\_any](../../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [as\_any\_mut](../../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [into\_reflect](../../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [as\_reflect](../../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [as\_reflect\_mut](../../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [set](../../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#239)

### impl [RelationshipSourceCollection](../../relationship/trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#240)

#### type [SourceIter](../../relationship/trait.RelationshipSourceCollection.html#associatedtype.SourceIter)<'a> = [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<[Iter](struct.Iter.html "struct bevy::ecs::entity::hash_set::Iter")<'a>>

The type of iterator returned by the `iter` method. [Read more](../../relationship/trait.RelationshipSourceCollection.html#associatedtype.SourceIter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#242)

#### fn [new](../../relationship/trait.RelationshipSourceCollection.html#tymethod.new)() -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

Creates a new empty instance.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#246)

#### fn [reserve](../../relationship/trait.RelationshipSourceCollection.html#tymethod.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserves capacity for at least `additional` more entities to be inserted. [Read more](../../relationship/trait.RelationshipSourceCollection.html#tymethod.reserve)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#250)

#### fn [with\_capacity](../../relationship/trait.RelationshipSourceCollection.html#tymethod.with_capacity)(capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

Returns an instance with the given pre-allocated entity `capacity`. [Read more](../../relationship/trait.RelationshipSourceCollection.html#tymethod.with_capacity)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#254)

#### fn [add](../../relationship/trait.RelationshipSourceCollection.html#tymethod.add)(&mut self, entity: [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Adds the given `entity` to the collection. [Read more](../../relationship/trait.RelationshipSourceCollection.html#tymethod.add)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#258)

#### fn [remove](../../relationship/trait.RelationshipSourceCollection.html#tymethod.remove)(&mut self, entity: [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Removes the given `entity` from the collection. [Read more](../../relationship/trait.RelationshipSourceCollection.html#tymethod.remove)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#262)

#### fn [iter](../../relationship/trait.RelationshipSourceCollection.html#tymethod.iter)( &self, ) -> <[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [RelationshipSourceCollection](../../relationship/trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection")\>::[SourceIter](../../relationship/trait.RelationshipSourceCollection.html#associatedtype.SourceIter "type bevy::ecs::relationship::RelationshipSourceCollection::SourceIter")<'\_>

Iterates all entities in the collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#266)

#### fn [len](../../relationship/trait.RelationshipSourceCollection.html#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the current length of the collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#270)

#### fn [clear](../../relationship/trait.RelationshipSourceCollection.html#tymethod.clear)(&mut self)

Clears the collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#274)

#### fn [shrink\_to\_fit](../../relationship/trait.RelationshipSourceCollection.html#tymethod.shrink_to_fit)(&mut self)

Attempts to save memory by shrinking the capacity to fit the current length. [Read more](../../relationship/trait.RelationshipSourceCollection.html#tymethod.shrink_to_fit)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#278)

#### fn [extend\_from\_iter](../../relationship/trait.RelationshipSourceCollection.html#tymethod.extend_from_iter)(&mut self, entities: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>)

Add multiple entities to collection at once. [Read more](../../relationship/trait.RelationshipSourceCollection.html#tymethod.extend_from_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#68)

#### fn [is\_empty](../../relationship/trait.RelationshipSourceCollection.html#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the collection contains no entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#74)

#### fn [source\_to\_remove\_before\_add](../../relationship/trait.RelationshipSourceCollection.html#method.source_to_remove_before_add)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

For one-to-one relationships, returns the entity that should be removed before adding a new one. Returns `None` for one-to-many relationships or when no entity needs to be removed.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#23)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#23)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#24)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#161)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#162)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#164)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")) -> <&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output "type core::ops::arith::Sub::Output")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#169)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")\> for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#170)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [TupleStruct](../../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [field](../../../prelude/trait.TupleStruct.html#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field with index `index` as a `&dyn Reflect`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [field\_mut](../../../prelude/trait.TupleStruct.html#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field with index `index` as a `&mut dyn Reflect`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [field\_len](../../../prelude/trait.TupleStruct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the tuple struct.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [iter\_fields](../../../prelude/trait.TupleStruct.html#tymethod.iter_fields)(&self) -> [TupleStructFieldIter](../../../reflect/tuple_struct/struct.TupleStructFieldIter.html "struct bevy::reflect::tuple_struct::TupleStructFieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the tuple struct’s fields.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [to\_dynamic\_tuple\_struct](../../../prelude/trait.TupleStruct.html#method.to_dynamic_tuple_struct)(&self) -> [DynamicTupleStruct](../../../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct")

Creates a new [`DynamicTupleStruct`](../../../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct") from this tuple struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#71)

#### fn [get\_represented\_tuple\_struct\_info](../../../prelude/trait.TupleStruct.html#method.get_represented_tuple_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TupleStructInfo](../../../reflect/tuple_struct/struct.TupleStructInfo.html "struct bevy::reflect::tuple_struct::TupleStructInfo")\>

Will return `None` if [`TypeInfo`](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [type\_path](../../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [short\_type\_path](../../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [type\_ident](../../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [crate\_name](../../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [module\_path](../../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [Typed](../../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

#### fn [type\_info](../../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#401)

### impl [WorldEntityFetch](../../world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch") for &[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#402)

#### type [Ref](../../world/trait.WorldEntityFetch.html#associatedtype.Ref)<'w> = [EntityHashMap](../struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<[EntityRef](../../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>>

The read-only reference type returned by [`WorldEntityFetch::fetch_ref`](../../world/trait.WorldEntityFetch.html#tymethod.fetch_ref "method bevy::ecs::world::WorldEntityFetch::fetch_ref").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#403)

#### type [Mut](../../world/trait.WorldEntityFetch.html#associatedtype.Mut)<'w> = [EntityHashMap](../struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<[EntityMut](../../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>>

The mutable reference type returned by [`WorldEntityFetch::fetch_mut`](../../world/trait.WorldEntityFetch.html#tymethod.fetch_mut "method bevy::ecs::world::WorldEntityFetch::fetch_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#404)

#### type [DeferredMut](../../world/trait.WorldEntityFetch.html#associatedtype.DeferredMut)<'w> = [EntityHashMap](../struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<[EntityMut](../../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>>

The mutable reference type returned by [`WorldEntityFetch::fetch_deferred_mut`](../../world/trait.WorldEntityFetch.html#tymethod.fetch_deferred_mut "method bevy::ecs::world::WorldEntityFetch::fetch_deferred_mut"), but without structural mutability.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#407-410)

#### unsafe fn [fetch\_ref](../../world/trait.WorldEntityFetch.html#tymethod.fetch_ref)( self, cell: [UnsafeWorldCell](../../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [WorldEntityFetch](../../world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Ref](../../world/trait.WorldEntityFetch.html#associatedtype.Ref "type bevy::ecs::world::WorldEntityFetch::Ref")<'\_>, [EntityNotSpawnedError](../enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

Returns read-only reference(s) to the entities with the given [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity") IDs, as determined by `self`. [Read more](../../world/trait.WorldEntityFetch.html#tymethod.fetch_ref)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#421-424)

#### unsafe fn [fetch\_mut](../../world/trait.WorldEntityFetch.html#tymethod.fetch_mut)( self, cell: [UnsafeWorldCell](../../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [WorldEntityFetch](../../world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Mut](../../world/trait.WorldEntityFetch.html#associatedtype.Mut "type bevy::ecs::world::WorldEntityFetch::Mut")<'\_>, [EntityMutableFetchError](../../world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

Returns mutable reference(s) to the entities with the given [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity") IDs, as determined by `self`. [Read more](../../world/trait.WorldEntityFetch.html#tymethod.fetch_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#435-438)

#### unsafe fn [fetch\_deferred\_mut](../../world/trait.WorldEntityFetch.html#tymethod.fetch_deferred_mut)( self, cell: [UnsafeWorldCell](../../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&[EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") as [WorldEntityFetch](../../world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[DeferredMut](../../world/trait.WorldEntityFetch.html#associatedtype.DeferredMut "type bevy::ecs::world::WorldEntityFetch::DeferredMut")<'\_>, [EntityMutableFetchError](../../world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

Returns mutable reference(s) to the entities with the given [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity") IDs, as determined by `self`, but without structural mutability. [Read more](../../world/trait.WorldEntityFetch.html#tymethod.fetch_deferred_mut)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [EntityHashSet](../struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)

### impl<T> [Brush](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/style/brush/trait.Brush.html "trait parley::style::brush::Brush") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../../../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../../../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../../../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../../../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../../../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](../../../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../../../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](../../../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](../../../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../../../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](../../../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../../../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](../../../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../../../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../../../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../../../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#160)

### impl<T> [EntitySet](../trait.EntitySet.html "trait bevy::ecs::entity::EntitySet") for T

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <T as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"): [EntitySetIterator](../trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../../../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Compare self to `key` and return `true` if they are equal.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#151-154)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#156)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](../../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../../../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../../../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../../../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](../../../prelude/trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](../../../prelude/trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](../../../prelude/trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](../../../prelude/trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](../../../prelude/trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](../../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](../../../prelude/trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](../../../prelude/trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../../../prelude/trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#207)

### impl<S> [GetTupleStructField](../../../prelude/trait.GetTupleStructField.html "trait bevy::prelude::GetTupleStructField") for S

where S: [TupleStruct](../../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#208)

#### fn [get\_field](../../../prelude/trait.GetTupleStructField.html#tymethod.get_field)<T>(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a reference to the value of the field with index `index`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#213)

#### fn [get\_field\_mut](../../../prelude/trait.GetTupleStructField.html#tymethod.get_field_mut)<T>(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a mutable reference to the value of the field with index `index`, downcast to `T`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../../../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../../../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../../../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../../../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../../../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../../../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../../../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../../../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../../../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../../../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../../../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../../../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](../../../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../../../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../../../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../../../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../../template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../../../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../../../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../../../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../../prelude/trait.ToOwned.html#method.clone_into)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../../../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Difference<'a, T, S, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Difference.html\\" title=\\"struct bevy::platform::collections::hash\_set::Difference\\">Difference</a>&lt;'a, T, S, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, S, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Difference.html\\" title=\\"struct bevy::platform::collections::hash\_set::Difference\\">Difference</a>&lt;'a, T, S, A&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a>,\\n S: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\\" title=\\"trait core::hash::BuildHasher\\">BuildHasher</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","Difference<'a, T, S>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Difference.html\\" title=\\"struct bevy::platform::collections::hash\_set::Difference\\">Difference</a>&lt;'a, T, S, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, S, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Difference.html\\" title=\\"struct bevy::platform::collections::hash\_set::Difference\\">Difference</a>&lt;'a, T, S, A&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a>,\\n S: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\\" title=\\"trait core::hash::BuildHasher\\">BuildHasher</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","Drain<'\_, T, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Drain.html\\" title=\\"struct bevy::platform::collections::hash\_set::Drain\\">Drain</a>&lt;'\_, K, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Drain.html\\" title=\\"struct bevy::platform::collections::hash\_set::Drain\\">Drain</a>&lt;'\_, K, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = K;</div>","Drain<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Drain.html\\" title=\\"struct bevy::platform::collections::hash\_set::Drain\\">Drain</a>&lt;'\_, K, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Drain.html\\" title=\\"struct bevy::platform::collections::hash\_set::Drain\\">Drain</a>&lt;'\_, K, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = K;</div>","Drain<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Drain.html\\" title=\\"struct bevy::ecs::entity::hash\_set::Drain\\">Drain</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.Drain.html\\" title=\\"struct bevy::ecs::entity::hash\_set::Drain\\">Drain</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","ExtractIf<'\_, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ExtractIf.html\\" title=\\"struct bevy::ecs::entity::hash\_set::ExtractIf\\">ExtractIf</a>&lt;'a, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.ExtractIf.html\\" title=\\"struct bevy::ecs::entity::hash\_set::ExtractIf\\">ExtractIf</a>&lt;'a, F&gt;<div class=\\"where\\">where\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;<a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","ExtractIf<'\_, T, F, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.ExtractIf.html\\" title=\\"struct bevy::platform::collections::hash\_set::ExtractIf\\">ExtractIf</a>&lt;'\_, K, F, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, F, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.ExtractIf.html\\" title=\\"struct bevy::platform::collections::hash\_set::ExtractIf\\">ExtractIf</a>&lt;'\_, K, F, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;K</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = K;</div>","ExtractIf<'\_, T, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.ExtractIf.html\\" title=\\"struct bevy::platform::collections::hash\_set::ExtractIf\\">ExtractIf</a>&lt;'\_, K, F, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, F, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.ExtractIf.html\\" title=\\"struct bevy::platform::collections::hash\_set::ExtractIf\\">ExtractIf</a>&lt;'\_, K, F, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;K</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = K;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Intersection<'a, T, S, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Intersection.html\\" title=\\"struct bevy::platform::collections::hash\_set::Intersection\\">Intersection</a>&lt;'a, T, S, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, S, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Intersection.html\\" title=\\"struct bevy::platform::collections::hash\_set::Intersection\\">Intersection</a>&lt;'a, T, S, A&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a>,\\n S: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\\" title=\\"trait core::hash::BuildHasher\\">BuildHasher</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","Intersection<'a, T, S>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Intersection.html\\" title=\\"struct bevy::platform::collections::hash\_set::Intersection\\">Intersection</a>&lt;'a, T, S, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, S, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Intersection.html\\" title=\\"struct bevy::platform::collections::hash\_set::Intersection\\">Intersection</a>&lt;'a, T, S, A&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a>,\\n S: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\\" title=\\"trait core::hash::BuildHasher\\">BuildHasher</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","Iter<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Iter.html\\" title=\\"struct bevy::platform::collections::hash\_set::Iter\\">Iter</a>&lt;'a, K&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Iter.html\\" title=\\"struct bevy::platform::collections::hash\_set::Iter\\">Iter</a>&lt;'a, K&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a K</a>;</div>","Iter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Iter.html\\" title=\\"struct bevy::ecs::entity::hash\_set::Iter\\">Iter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.Iter.html\\" title=\\"struct bevy::ecs::entity::hash\_set::Iter\\">Iter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","SymmetricDifference<'a, T, S, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.SymmetricDifference.html\\" title=\\"struct bevy::platform::collections::hash\_set::SymmetricDifference\\">SymmetricDifference</a>&lt;'a, T, S, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, S, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.SymmetricDifference.html\\" title=\\"struct bevy::platform::collections::hash\_set::SymmetricDifference\\">SymmetricDifference</a>&lt;'a, T, S, A&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a>,\\n S: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\\" title=\\"trait core::hash::BuildHasher\\">BuildHasher</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","SymmetricDifference<'a, T, S>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.SymmetricDifference.html\\" title=\\"struct bevy::platform::collections::hash\_set::SymmetricDifference\\">SymmetricDifference</a>&lt;'a, T, S, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, S, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.SymmetricDifference.html\\" title=\\"struct bevy::platform::collections::hash\_set::SymmetricDifference\\">SymmetricDifference</a>&lt;'a, T, S, A&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a>,\\n S: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\\" title=\\"trait core::hash::BuildHasher\\">BuildHasher</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","TupleStructFieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a (dyn <a class=\\"trait\\" href=\\"../../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static);</div>","Union<'a, T, S, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Union.html\\" title=\\"struct bevy::platform::collections::hash\_set::Union\\">Union</a>&lt;'a, T, S, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, S, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Union.html\\" title=\\"struct bevy::platform::collections::hash\_set::Union\\">Union</a>&lt;'a, T, S, A&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a>,\\n S: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\\" title=\\"trait core::hash::BuildHasher\\">BuildHasher</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","Union<'a, T, S>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Union.html\\" title=\\"struct bevy::platform::collections::hash\_set::Union\\">Union</a>&lt;'a, T, S, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T, S, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../platform/collections/hash\_set/struct.Union.html\\" title=\\"struct bevy::platform::collections::hash\_set::Union\\">Union</a>&lt;'a, T, S, A&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a>,\\n S: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\\" title=\\"trait core::hash::BuildHasher\\">BuildHasher</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}