[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[index\_map](index.html)

# Struct EntityIndexMap 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#35)

```rust
pub struct EntityIndexMap<V>(/* private fields */);
```

A [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap") pre-configured to use [`EntityHash`](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash") hashing.

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#37)

### impl<V> [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#43)

#### pub const fn [new](#method.new)() -> [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

Creates an empty `EntityIndexMap`.

Equivalent to [`IndexMap::with_hasher(EntityHash)`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.with_hasher "associated function indexmap::map::IndexMap::with_hasher").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#52)

#### pub fn [with\_capacity](#method.with_capacity)(n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

Creates an empty `EntityIndexMap` with the specified capacity.

Equivalent to [`IndexMap::with_capacity_and_hasher(n, EntityHash)`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.with_capacity_and_hasher "associated function indexmap::map::IndexMap::with_capacity_and_hasher").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#57)

#### pub const fn [from\_index\_map](#method.from_index_map)( set: [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>, ) -> [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

Constructs an `EntityIndexMap` from an [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#62)

#### pub fn [into\_inner](#method.into_inner)(self) -> [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

Returns the inner [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#69)

#### pub fn [as\_slice](#method.as_slice)(&self) -> &[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Returns a slice of all the key-value pairs in the map.

Equivalent to [`IndexMap::as_slice`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.as_slice "method indexmap::map::IndexMap::as_slice").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#77)

#### pub fn [as\_mut\_slice](#method.as_mut_slice)(&mut self) -> &mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

Returns a mutable slice of all the key-value pairs in the map.

Equivalent to [`IndexMap::as_mut_slice`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.as_mut_slice "method indexmap::map::IndexMap::as_mut_slice").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#85)

#### pub fn [into\_boxed\_slice](#method.into_boxed_slice)(self) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

Converts into a boxed slice of all the key-value pairs in the map.

Equivalent to [`IndexMap::into_boxed_slice`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.into_boxed_slice "method indexmap::map::IndexMap::into_boxed_slice").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#93)

#### pub fn [get\_range](#method.get_range)<R>(&self, range: R) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Returns a slice of key-value pairs in the given range of indices.

Equivalent to [`IndexMap::get_range`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.get_range "method indexmap::map::IndexMap::get_range").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#102)

#### pub fn [get\_range\_mut](#method.get_range_mut)<R>(&mut self, range: R) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>>

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Returns a mutable slice of key-value pairs in the given range of indices.

Equivalent to [`IndexMap::get_range_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.get_range_mut "method indexmap::map::IndexMap::get_range_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#111)

#### pub fn [iter](#method.iter)(&self) -> [Iter](struct.Iter.html "struct bevy::ecs::entity::index_map::Iter")<'\_, V> [ⓘ](#)

Return an iterator over the key-value pairs of the map, in their order.

Equivalent to [`IndexMap::iter`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.iter "method indexmap::map::IndexMap::iter").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#118)

#### pub fn [iter\_mut](#method.iter_mut)(&mut self) -> [IterMut](struct.IterMut.html "struct bevy::ecs::entity::index_map::IterMut")<'\_, V> [ⓘ](#)

Return a mutable iterator over the key-value pairs of the map, in their order.

Equivalent to [`IndexMap::iter_mut`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.iter_mut "method indexmap::map::IndexMap::iter_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#126)

#### pub fn [drain](#method.drain)<R>(&mut self, range: R) -> [Drain](struct.Drain.html "struct bevy::ecs::entity::index_map::Drain")<'\_, V> [ⓘ](#)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Clears the `IndexMap` in the given index range, returning those key-value pairs as a drain iterator.

Equivalent to [`IndexMap::drain`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.drain "method indexmap::map::IndexMap::drain").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#133)

#### pub fn [keys](#method.keys)(&self) -> [Keys](struct.Keys.html "struct bevy::ecs::entity::index_map::Keys")<'\_, V> [ⓘ](#)

Return an iterator over the keys of the map, in their order.

Equivalent to [`IndexMap::keys`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.keys "method indexmap::map::IndexMap::keys").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#140)

#### pub fn [into\_keys](#method.into_keys)(self) -> [IntoKeys](struct.IntoKeys.html "struct bevy::ecs::entity::index_map::IntoKeys")<V> [ⓘ](#)

Return an owning iterator over the keys of the map, in their order.

Equivalent to [`IndexMap::into_keys`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.into_keys "method indexmap::map::IndexMap::into_keys").

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>>

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#215)

#### pub fn [capacity](#method.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Return the number of elements the map can hold without reallocating.

This number is a lower bound; the map might be able to hold more, but is guaranteed to be able to hold at least this many.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#220)

#### pub fn [hasher](#method.hasher)(&self) -> [&S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Return a reference to the map’s `BuildHasher`.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#228)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Return the number of key-value pairs in the map.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#236)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the map contains no elements.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#241)

#### pub fn [iter](#method.iter-1)(&self) -> [Iter](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Iter.html "struct indexmap::map::iter::Iter")<'\_, K, V> [ⓘ](#)

Return an iterator over the key-value pairs of the map, in their order

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#246)

#### pub fn [iter\_mut](#method.iter_mut-1)(&mut self) -> [IterMut](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.IterMut.html "struct indexmap::map::iter::IterMut")<'\_, K, V> [ⓘ](#)

Return an iterator over the key-value pairs of the map, in their order

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#251)

#### pub fn [keys](#method.keys-1)(&self) -> [Keys](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Keys.html "struct indexmap::map::iter::Keys")<'\_, K, V> [ⓘ](#)

Return an iterator over the keys of the map, in their order

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#261)

#### pub fn [values](#method.values)(&self) -> [Values](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Values.html "struct indexmap::map::iter::Values")<'\_, K, V> [ⓘ](#)

Return an iterator over the values of the map, in their order

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/usage/debug\_frustum\_culling.rs ([line 316](../../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#316))

```rust
304fn update_shape_aabb_colors(
305    view_query: Query<&VisibleEntities, With<MyCamera>>,
306    mut gizmo_query: Query<&mut ShowAabbGizmo, With<MyShape>>,
307) -> Result {
308    // Reset the color to use the config's default color
309    for mut shape_gizmo in &mut gizmo_query {
310        shape_gizmo.color = None;
311    }
312
313    // Query for the shape entities visible for this camera
314    // Update the gizmo on any such shape entity to be green
315    let visible_entities = view_query.single()?;
316    for entity in visible_entities.entities.values().flatten() {
317        if let Ok(mut shape_gizmo) = gizmo_query.get_mut(*entity) {
318            shape_gizmo.color = Some(Color::LinearRgba(LinearRgba::GREEN));
319        }
320    }
321    Ok(())
322}
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#267)

#### pub fn [values\_mut](#method.values_mut)(&mut self) -> [ValuesMut](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.ValuesMut.html "struct indexmap::map::iter::ValuesMut")<'\_, K, V> [ⓘ](#)

Return an iterator over mutable references to the values of the map, in their order

##### [Examples found in repository](#scraped-examples-1)[?](../../../../scrape-examples-help.html)

examples/shader\_advanced/custom\_render\_phase.rs ([line 332](../../../../src/custom_render_phase/custom_render_phase.rs.html#332))

```rust
326    fn recalculate_sort_keys(
327        items: &mut IndexMap<(Entity, MainEntity), Self, EntityHash>,
328        view: &ExtractedView,
329    ) {
330        // Determine the distance to the view for each phase item.
331        let rangefinder = view.rangefinder3d();
332        for item in items.values_mut() {
333            item.distance = FloatOrd(item.sorting_info.sort_distance(&rangefinder));
334        }
335    }
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#279)

#### pub fn [clear](#method.clear)(&mut self)

Remove all key-value pairs in the map, while preserving its capacity.

Computes in **O(n)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#286)

#### pub fn [truncate](#method.truncate)(&mut self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shortens the map, keeping the first `len` elements and dropping the rest.

If `len` is greater than the map’s current length, this has no effect.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#304-306)

#### pub fn [drain](#method.drain-1)<R>(&mut self, range: R) -> [Drain](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Drain.html "struct indexmap::map::iter::Drain")<'\_, K, V> [ⓘ](#)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Clears the `IndexMap` in the given index range, returning those key-value pairs as a drain iterator.

The range may be any type that implements [`RangeBounds<usize>`](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds"), including all of the `std::ops::Range*` types, or even a tuple pair of `Bound` start and end values. To drain the map entirely, use `RangeFull` like `map.drain(..)`.

This shifts down all entries following the drained range to fill the gap, and keeps the allocated memory for reuse.

_**Panics**_ if the starting point is greater than the end point or if the end point is greater than the length of the map.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#352-355)

#### pub fn [extract\_if](#method.extract_if)<F, R>(&mut self, range: R, pred: F) -> [ExtractIf](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.ExtractIf.html "struct indexmap::map::iter::ExtractIf")<'\_, K, V, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Creates an iterator which uses a closure to determine if an element should be removed, for all elements in the given range.

If the closure returns true, the element is removed from the map and yielded. If the closure returns false, or panics, the element remains in the map and will not be yielded.

Note that `extract_if` lets you mutate every value in the filter closure, regardless of whether you choose to keep or remove it.

The range may be any type that implements [`RangeBounds<usize>`](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds"), including all of the `std::ops::Range*` types, or even a tuple pair of `Bound` start and end values. To check the entire map, use `RangeFull` like `map.extract_if(.., predicate)`.

If the returned `ExtractIf` is not exhausted, e.g. because it is dropped without iterating or the iteration short-circuits, then the remaining elements will be retained. Use [`retain`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.retain "method indexmap::map::IndexMap::retain") with a negated predicate if you do not need the returned iterator.

_**Panics**_ if the starting point is greater than the end point or if the end point is greater than the length of the map.

##### Examples

Splitting a map into even and odd keys, reusing the original map:

```rust
use indexmap::IndexMap;

let mut map: IndexMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
let extracted: IndexMap<i32, i32> = map.extract_if(.., |k, _v| k % 2 == 0).collect();

let evens = extracted.keys().copied().collect::<Vec<_>>();
let odds = map.keys().copied().collect::<Vec<_>>();

assert_eq!(evens, vec![0, 2, 4, 6]);
assert_eq!(odds, vec![1, 3, 5, 7]);
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#368-370)

#### pub fn [split\_off](#method.split_off)(&mut self, at: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<K, V, S>

where S: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Splits the collection into two at the given index.

Returns a newly allocated map containing the elements in the range `[at, len)`. After the call, the original map will be left containing the elements `[0, at)` with its previous capacity unchanged.

_**Panics**_ if `at > len`.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#381)

#### pub fn [reserve](#method.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserve capacity for `additional` more key-value pairs.

Computes in **O(n)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#393)

#### pub fn [reserve\_exact](#method.reserve_exact)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserve capacity for `additional` more key-value pairs, without over-allocating.

Unlike `reserve`, this does not deliberately over-allocate the entry capacity to avoid frequent re-allocations. However, the underlying data structures may still have internal capacity requirements, and the allocator itself may give more space than requested, so this cannot be relied upon to be precisely minimal.

Computes in **O(n)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#400)

#### pub fn [try\_reserve](#method.try_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/struct.TryReserveError.html "struct indexmap::TryReserveError")\>

Try to reserve capacity for `additional` more key-value pairs.

Computes in **O(n)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#412)

#### pub fn [try\_reserve\_exact](#method.try_reserve_exact)( &mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/struct.TryReserveError.html "struct indexmap::TryReserveError")\>

Try to reserve capacity for `additional` more key-value pairs, without over-allocating.

Unlike `try_reserve`, this does not deliberately over-allocate the entry capacity to avoid frequent re-allocations. However, the underlying data structures may still have internal capacity requirements, and the allocator itself may give more space than requested, so this cannot be relied upon to be precisely minimal.

Computes in **O(n)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#419)

#### pub fn [shrink\_to\_fit](#method.shrink_to_fit)(&mut self)

Shrink the capacity of the map as much as possible.

Computes in **O(n)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#426)

#### pub fn [shrink\_to](#method.shrink_to)(&mut self, min\_capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shrink the capacity of the map with a lower limit.

Computes in **O(n)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#450)

#### pub fn [insert](#method.insert)(&mut self, key: K, value: V) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

Insert a key-value pair in the map.

If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with `value`, and the older value is returned inside `Some(_)`.

If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and `None` is returned.

Computes in **O(1)** time (amortized average).

See also [`entry`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.entry "method indexmap::map::IndexMap::entry") if you want to insert _or_ modify, or [`insert_full`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert_full "method indexmap::map::IndexMap::insert_full") if you need to get the index of the corresponding key-value pair.

##### [Examples found in repository](#scraped-examples-2)[?](../../../../scrape-examples-help.html)

examples/shader\_advanced/manual\_material.rs ([lines 98-107](../../../../src/manual_material/manual_material.rs.html#98-107))

```rust
79fn init_image_material_resources(
80    mut commands: Commands,
81    render_device: Res<RenderDevice>,
82    mut bind_group_allocators: ResMut<MaterialBindGroupAllocators>,
83) {
84    let bind_group_layout = BindGroupLayoutDescriptor::new(
85        "image_material_layout",
86        &BindGroupLayoutEntries::sequential(
87            ShaderStages::FRAGMENT,
88            (
89                texture_2d(TextureSampleType::Float { filterable: false }),
90                sampler(SamplerBindingType::NonFiltering),
91            ),
92        ),
93    );
94    let sampler = render_device.create_sampler(&SamplerDescriptor::default());
95    commands.insert_resource(ImageMaterialBindGroupLayout(bind_group_layout.clone()));
96    commands.insert_resource(ImageMaterialBindGroupSampler(sampler));
97
98    bind_group_allocators.insert(
99        TypeId::of::<ImageMaterial>(),
100        MaterialBindGroupAllocator::new(
101            &render_device,
102            "image_material_allocator",
103            None,
104            bind_group_layout,
105            None,
106        ),
107    );
108}
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#466)

#### pub fn [insert\_full](#method.insert_full)(&mut self, key: K, value: V) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>)

Insert a key-value pair in the map, and get their index.

If an equivalent key already exists in the map: the key remains and retains in its place in the order, its corresponding value is updated with `value`, and the older value is returned inside `(index, Some(_))`.

If no equivalent key existed in the map: the new key-value pair is inserted, last in order, and `(index, None)` is returned.

Computes in **O(1)** time (amortized average).

See also [`entry`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.entry "method indexmap::map::IndexMap::entry") if you want to insert _or_ modify.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#490-492)

#### pub fn [insert\_sorted](#method.insert_sorted)(&mut self, key: K, value: V) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>)

where K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Insert a key-value pair in the map at its ordered position among sorted keys.

This is equivalent to finding the position with [`binary_search_keys`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.binary_search_keys "method indexmap::map::IndexMap::binary_search_keys"), then either updating it or calling [`insert_before`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert_before "method indexmap::map::IndexMap::insert_before") for a new key.

If the sorted key is found in the map, its corresponding value is updated with `value`, and the older value is returned inside `(index, Some(_))`. Otherwise, the new key-value pair is inserted at the sorted position, and `(index, None)` is returned.

If the existing keys are **not** already sorted, then the insertion index is unspecified (like [`slice::binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search")), but the key-value pair is moved to or inserted at that position regardless.

Computes in **O(n)** time (average). Instead of repeating calls to `insert_sorted`, it may be faster to call batched [`insert`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert "method indexmap::map::IndexMap::insert") or [`extend`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.extend "method indexmap::map::IndexMap::extend") and only call [`sort_keys`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_keys "method indexmap::map::IndexMap::sort_keys") or [`sort_unstable_keys`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_unstable_keys "method indexmap::map::IndexMap::sort_unstable_keys") once.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#512-514)

#### pub fn [insert\_sorted\_by](#method.insert_sorted_by)<F>( &mut self, key: K, value: V, cmp: F, ) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Insert a key-value pair in the map at its ordered position among keys sorted by `cmp`.

This is equivalent to finding the position with [`binary_search_by`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.binary_search_by "method indexmap::map::IndexMap::binary_search_by"), then calling [`insert_before`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert_before "method indexmap::map::IndexMap::insert_before") with the given key and value.

If the existing keys are **not** already sorted, then the insertion index is unspecified (like [`slice::binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search")), but the key-value pair is moved to or inserted at that position regardless.

Computes in **O(n)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#532-540)

#### pub fn [insert\_sorted\_by\_key](#method.insert_sorted_by_key)<B, F>( &mut self, key: K, value: V, sort\_key: F, ) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>)

where B: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> B,

Insert a key-value pair in the map at its ordered position using a sort-key extraction function.

This is equivalent to finding the position with [`binary_search_by_key`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.binary_search_by_key "method indexmap::map::IndexMap::binary_search_by_key") with `sort_key(key)`, then calling [`insert_before`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert_before "method indexmap::map::IndexMap::insert_before") with the given key and value.

If the existing keys are **not** already sorted, then the insertion index is unspecified (like [`slice::binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search")), but the key-value pair is moved to or inserted at that position regardless.

Computes in **O(n)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#596)

#### pub fn [insert\_before](#method.insert_before)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), key: K, value: V, ) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>)

Insert a key-value pair in the map before the entry at the given index, or at the end.

If an equivalent key already exists in the map: the key remains and is moved to the new position in the map, its corresponding value is updated with `value`, and the older value is returned inside `Some(_)`. The returned index will either be the given index or one less, depending on how the entry moved. (See [`shift_insert`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.shift_insert "method indexmap::map::IndexMap::shift_insert") for different behavior here.)

If no equivalent key existed in the map: the new key-value pair is inserted exactly at the given index, and `None` is returned.

_**Panics**_ if `index` is out of bounds. Valid indices are `0..=map.len()` (inclusive).

Computes in **O(n)** time (average).

See also [`entry`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.entry "method indexmap::map::IndexMap::entry") if you want to insert _or_ modify, perhaps only using the index for new entries with [`VacantEntry::shift_insert`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/inner/entry/struct.VacantEntry.html#method.shift_insert "method indexmap::inner::entry::VacantEntry::shift_insert").

##### Examples

```rust
use indexmap::IndexMap;
let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();

// The new key '*' goes exactly at the given index.
assert_eq!(map.get_index_of(&'*'), None);
assert_eq!(map.insert_before(10, '*', ()), (10, None));
assert_eq!(map.get_index_of(&'*'), Some(10));

// Moving the key 'a' up will shift others down, so this moves *before* 10 to index 9.
assert_eq!(map.insert_before(10, 'a', ()), (9, Some(())));
assert_eq!(map.get_index_of(&'a'), Some(9));
assert_eq!(map.get_index_of(&'*'), Some(10));

// Moving the key 'z' down will shift others up, so this moves to exactly 10.
assert_eq!(map.insert_before(10, 'z', ()), (10, Some(())));
assert_eq!(map.get_index_of(&'z'), Some(10));
assert_eq!(map.get_index_of(&'*'), Some(11));

// Moving or inserting before the endpoint is also valid.
assert_eq!(map.len(), 27);
assert_eq!(map.insert_before(map.len(), '*', ()), (26, Some(())));
assert_eq!(map.get_index_of(&'*'), Some(26));
assert_eq!(map.insert_before(map.len(), '+', ()), (27, None));
assert_eq!(map.get_index_of(&'+'), Some(27));
assert_eq!(map.len(), 28);
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#681)

#### pub fn [shift\_insert](#method.shift_insert)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), key: K, value: V) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

Insert a key-value pair in the map at the given index.

If an equivalent key already exists in the map: the key remains and is moved to the given index in the map, its corresponding value is updated with `value`, and the older value is returned inside `Some(_)`. Note that existing entries **cannot** be moved to `index == map.len()`! (See [`insert_before`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert_before "method indexmap::map::IndexMap::insert_before") for different behavior here.)

If no equivalent key existed in the map: the new key-value pair is inserted at the given index, and `None` is returned.

_**Panics**_ if `index` is out of bounds. Valid indices are `0..map.len()` (exclusive) when moving an existing entry, or `0..=map.len()` (inclusive) when inserting a new key.

Computes in **O(n)** time (average).

See also [`entry`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.entry "method indexmap::map::IndexMap::entry") if you want to insert _or_ modify, perhaps only using the index for new entries with [`VacantEntry::shift_insert`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/inner/entry/struct.VacantEntry.html#method.shift_insert "method indexmap::inner::entry::VacantEntry::shift_insert").

##### Examples

```rust
use indexmap::IndexMap;
let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();

// The new key '*' goes exactly at the given index.
assert_eq!(map.get_index_of(&'*'), None);
assert_eq!(map.shift_insert(10, '*', ()), None);
assert_eq!(map.get_index_of(&'*'), Some(10));

// Moving the key 'a' up to 10 will shift others down, including the '*' that was at 10.
assert_eq!(map.shift_insert(10, 'a', ()), Some(()));
assert_eq!(map.get_index_of(&'a'), Some(10));
assert_eq!(map.get_index_of(&'*'), Some(9));

// Moving the key 'z' down to 9 will shift others up, including the '*' that was at 9.
assert_eq!(map.shift_insert(9, 'z', ()), Some(()));
assert_eq!(map.get_index_of(&'z'), Some(9));
assert_eq!(map.get_index_of(&'*'), Some(10));

// Existing keys can move to len-1 at most, but new keys can insert at the endpoint.
assert_eq!(map.len(), 27);
assert_eq!(map.shift_insert(map.len() - 1, '*', ()), Some(()));
assert_eq!(map.get_index_of(&'*'), Some(26));
assert_eq!(map.shift_insert(map.len(), '+', ()), None);
assert_eq!(map.get_index_of(&'+'), Some(27));
assert_eq!(map.len(), 28);
```

[ⓘ](# "This example panics")

```rust
use indexmap::IndexMap;
let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();

// This is an invalid index for moving an existing key!
map.shift_insert(map.len(), 'a', ());
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#722)

#### pub fn [replace\_index](#method.replace_index)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), key: K) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<K, ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), K)>

Replaces the key at the given index. The new key does not need to be equivalent to the one it is replacing, but it must be unique to the rest of the map.

Returns `Ok(old_key)` if successful, or `Err((other_index, key))` if an equivalent key already exists at a different index. The map will be unchanged in the error case.

Direct indexing can be used to change the corresponding value: simply `map[index] = value`, or `mem::replace(&mut map[index], value)` to retrieve the old value as well.

_**Panics**_ if `index` is out of bounds.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#741)

#### pub fn [entry](#method.entry)(&mut self, key: K) -> [Entry](../../../utils/enum.TypeIdMapEntry.html "enum bevy::utils::TypeIdMapEntry")<'\_, K, V>

Get the given key’s corresponding entry in the map for insertion and/or in-place manipulation.

Computes in **O(1)** time (amortized average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#777-780)

#### pub fn [splice](#method.splice)<R, I>( &mut self, range: R, replace\_with: I, ) -> [Splice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/iter/struct.Splice.html "struct indexmap::map::iter::Splice")<'\_, <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"), K, V, S> [ⓘ](#)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

Creates a splicing iterator that replaces the specified range in the map with the given `replace_with` key-value iterator and yields the removed items. `replace_with` does not need to be the same length as `range`.

The `range` is removed even if the iterator is not consumed until the end. It is unspecified how many elements are removed from the map if the `Splice` value is leaked.

The input iterator `replace_with` is only consumed when the `Splice` value is dropped. If a key from the iterator matches an existing entry in the map (outside of `range`), then the value will be updated in that position. Otherwise, the new key-value pair will be inserted in the replaced `range`.

_**Panics**_ if the starting point is greater than the end point or if the end point is greater than the length of the map.

##### Examples

```rust
use indexmap::IndexMap;

let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
let new = [(5, 'E'), (4, 'D'), (3, 'C'), (2, 'B'), (1, 'A')];
let removed: Vec<_> = map.splice(2..4, new).collect();

// 1 and 4 got new values, while 5, 3, and 2 were newly inserted.
assert!(map.into_iter().eq([(0, '_'), (1, 'A'), (5, 'E'), (3, 'C'), (2, 'B'), (4, 'D')]));
assert_eq!(removed, &[(2, 'b'), (3, 'c')]);
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#810)

#### pub fn [append](#method.append)<S2>(&mut self, other: &mut [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<K, V, S2>)

Moves all key-value pairs from `other` into `self`, leaving `other` empty.

This is equivalent to calling [`insert`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.insert "method indexmap::map::IndexMap::insert") for each key-value pair from `other` in order, which means that for keys that already exist in `self`, their value is updated in the current position.

##### Examples

```rust
use indexmap::IndexMap;

// Note: Key (3) is present in both maps.
let mut a = IndexMap::from([(3, "c"), (2, "b"), (1, "a")]);
let mut b = IndexMap::from([(3, "d"), (4, "e"), (5, "f")]);
let old_capacity = b.capacity();

a.append(&mut b);

assert_eq!(a.len(), 5);
assert_eq!(b.len(), 0);
assert_eq!(b.capacity(), old_capacity);

assert!(a.keys().eq(&[3, 2, 1, 4, 5]));
assert_eq!(a[&3], "d"); // "c" was overwritten.
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#827-829)

#### pub fn [contains\_key](#method.contains_key)<Q>(&self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return `true` if an equivalent to `key` exists in the map.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#838-840)

#### pub fn [get](#method.get)<Q>(&self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return a reference to the stored value for `key`, if it is present, else `None`.

Computes in **O(1)** time (average).

##### [Examples found in repository](#scraped-examples-3)[?](../../../../scrape-examples-help.html)

examples/3d/occlusion\_culling.rs ([line 396](../../../../src/occlusion_culling/occlusion_culling.rs.html#396))

```rust
388fn readback_indirect_parameters_node(
389    mut render_context: RenderContext,
390    indirect_parameters_buffers: Res<IndirectParametersBuffers>,
391    indirect_parameters_mapping_buffers: Res<IndirectParametersStagingBuffers>,
392) {
393    // Get the indirect parameters buffers corresponding to the opaque 3D
394    // phase, since all our meshes are in that phase.
395    let Some(phase_indirect_parameters_buffers) =
396        indirect_parameters_buffers.get(&TypeId::of::<Opaque3d>())
397    else {
398        return;
399    };
400
401    // Grab both the buffers we're copying from and the staging buffers
402    // we're copying to. Remember that we can't map the indirect parameters
403    // buffers directly, so we have to copy their contents to a staging
404    // buffer.
405    let (
406        Some(indexed_data_buffer),
407        Some(indexed_batch_sets_buffer),
408        Some(indirect_parameters_staging_data_buffer),
409        Some(indirect_parameters_staging_batch_sets_buffer),
410    ) = (
411        phase_indirect_parameters_buffers.indexed.data_buffer(),
412        phase_indirect_parameters_buffers
413            .indexed
414            .batch_sets_buffer(),
415        indirect_parameters_mapping_buffers.data.as_ref(),
416        indirect_parameters_mapping_buffers.batch_sets.as_ref(),
417    )
418    else {
419        return;
420    };
421
422    // Copy from the indirect parameters buffers to the staging buffers.
423    render_context.command_encoder().copy_buffer_to_buffer(
424        indexed_data_buffer,
425        0,
426        indirect_parameters_staging_data_buffer,
427        0,
428        indexed_data_buffer.size(),
429    );
430    render_context.command_encoder().copy_buffer_to_buffer(
431        indexed_batch_sets_buffer,
432        0,
433        indirect_parameters_staging_batch_sets_buffer,
434        0,
435        indexed_batch_sets_buffer.size(),
436    );
437}
438
439/// Creates the staging buffers that we use to read back the indirect parameters
440/// from the GPU to the CPU.
441///
442/// We read the indirect parameters from the GPU to the CPU in order to display
443/// the number of meshes that were culled each frame.
444///
445/// We need these staging buffers because `wgpu` doesn't allow us to read the
446/// contents of the indirect parameters buffers directly. We must first copy
447/// them from the GPU to a staging buffer, and then read the staging buffer.
448fn create_indirect_parameters_staging_buffers(
449    mut indirect_parameters_staging_buffers: ResMut<IndirectParametersStagingBuffers>,
450    indirect_parameters_buffers: Res<IndirectParametersBuffers>,
451    render_device: Res<RenderDevice>,
452) {
453    let Some(phase_indirect_parameters_buffers) =
454        indirect_parameters_buffers.get(&TypeId::of::<Opaque3d>())
455    else {
456        return;
457    };
458
459    // Fetch the indirect parameters buffers that we're going to copy from.
460    let (Some(indexed_data_buffer), Some(indexed_batch_set_buffer)) = (
461        phase_indirect_parameters_buffers.indexed.data_buffer(),
462        phase_indirect_parameters_buffers
463            .indexed
464            .batch_sets_buffer(),
465    ) else {
466        return;
467    };
468
469    // Build the staging buffers. Make sure they have the same sizes as the
470    // buffers we're copying from.
471    indirect_parameters_staging_buffers.data =
472        Some(render_device.create_buffer(&BufferDescriptor {
473            label: Some("indexed data staging buffer"),
474            size: indexed_data_buffer.size(),
475            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
476            mapped_at_creation: false,
477        }));
478    indirect_parameters_staging_buffers.batch_sets =
479        Some(render_device.create_buffer(&BufferDescriptor {
480            label: Some("indexed batch set staging buffer"),
481            size: indexed_batch_set_buffer.size(),
482            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
483            mapped_at_creation: false,
484        }));
485}
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#854-856)

#### pub fn [get\_key\_value](#method.get_key_value)<Q>(&self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return references to the stored key-value pair for the lookup `key`, if it is present, else `None`.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#870-872)

#### pub fn [get\_full](#method.get_full)<Q>(&self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return the index with references to the stored key-value pair for the lookup `key`, if it is present, else `None`.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#885-887)

#### pub fn [get\_index\_of](#method.get_index_of)<Q>(&self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return the item index for `key`, if it is present, else `None`.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#903-905)

#### pub fn [get\_mut](#method.get_mut)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return a mutable reference to the stored value for `key`, if it is present, else `None`.

Computes in **O(1)** time (average).

##### [Examples found in repository](#scraped-examples-4)[?](../../../../scrape-examples-help.html)

examples/shader\_advanced/manual\_material.rs ([line 161](../../../../src/manual_material/manual_material.rs.html#161))

```rust
145    fn prepare_asset(
146        source_asset: Self::SourceAsset,
147        asset_id: AssetId<Self::SourceAsset>,
148        (
149            opaque_draw_functions,
150            material_layout,
151            asset_server,
152            bind_group_allocators,
153            render_material_bindings,
154            gpu_images,
155            image_material_sampler,
156        ): &mut SystemParamItem<Self::Param>,
157    ) -> std::result::Result<Self::ErasedAsset, PrepareAssetError<Self::SourceAsset>> {
158        let material_layout = material_layout.0.clone();
159        let draw_function_id = opaque_draw_functions.read().id::<DrawMaterial>();
160        let bind_group_allocator = bind_group_allocators
161            .get_mut(&TypeId::of::<ImageMaterial>())
162            .unwrap();
163        let Some(image) = gpu_images.get(&source_asset.image) else {
164            return Err(PrepareAssetError::RetryNextUpdate(source_asset));
165        };
166        let unprepared = UnpreparedBindGroup {
167            bindings: BindingResources(vec![
168                (
169                    0,
170                    OwnedBindingResource::TextureView(
171                        TextureViewDimension::D2,
172                        image.texture_view.clone(),
173                    ),
174                ),
175                (
176                    1,
177                    OwnedBindingResource::Sampler(
178                        SamplerBindingType::NonFiltering,
179                        image_material_sampler.0.clone(),
180                    ),
181                ),
182            ]),
183        };
184        let binding = match render_material_bindings.entry(asset_id.into()) {
185            Entry::Occupied(mut occupied_entry) => {
186                bind_group_allocator.free(*occupied_entry.get());
187                let new_binding =
188                    bind_group_allocator.allocate_unprepared(unprepared, &material_layout);
189                *occupied_entry.get_mut() = new_binding;
190                new_binding
191            }
192            Entry::Vacant(vacant_entry) => *vacant_entry
193                .insert(bind_group_allocator.allocate_unprepared(unprepared, &material_layout)),
194        };
195
196        let mut properties = MaterialProperties {
197            material_layout: Some(material_layout),
198            mesh_pipeline_key_bits: ErasedMeshPipelineKey::new(MeshPipelineKey::empty()),
199            base_specialize: Some(base_specialize),
200            ..Default::default()
201        };
202        properties.add_draw_function(MainPassOpaqueDrawFunction, draw_function_id);
203        properties.add_shader(MaterialFragmentShader, asset_server.load(SHADER_ASSET_PATH));
204
205        Ok(PreparedMaterial {
206            binding,
207            properties: Arc::new(properties),
208        })
209    }
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#919-921)

#### pub fn [get\_key\_value\_mut](#method.get_key_value_mut)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return a reference and mutable references to the stored key-value pair for the lookup `key`, if it is present, else `None`.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#935-937)

#### pub fn [get\_full\_mut](#method.get_full_mut)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return the index with a reference and mutable reference to the stored key-value pair for the lookup `key`, if it is present, else `None`.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#958-960)

#### pub fn [get\_disjoint\_mut](#method.get_disjoint_mut)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, keys: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return the values for `N` keys.

_**Panics**_ if any key is duplicated.

##### Examples

```rust
let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
assert_eq!(map.get_disjoint_mut([&2, &1]), [Some(&mut 'c'), Some(&mut 'a')]);
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#985-987)

#### pub fn [remove](#method.remove)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated:

`remove` disrupts the map order – use `swap_remove` or `shift_remove` for explicit behavior.

Remove the key-value pair equivalent to `key` and return its value.

**NOTE:** This is equivalent to [`.swap_remove(key)`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.swap_remove "method indexmap::map::IndexMap::swap_remove"), replacing this entry’s position with the last element, and it is deprecated in favor of calling that explicitly. If you need to preserve the relative order of the keys in the map, use [`.shift_remove(key)`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.shift_remove "method indexmap::map::IndexMap::shift_remove") instead.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1000-1002)

#### pub fn [remove\_entry](#method.remove_entry)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated:

`remove_entry` disrupts the map order – use `swap_remove_entry` or `shift_remove_entry` for explicit behavior.

Remove and return the key-value pair equivalent to `key`.

**NOTE:** This is equivalent to [`.swap_remove_entry(key)`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.swap_remove_entry "method indexmap::map::IndexMap::swap_remove_entry"), replacing this entry’s position with the last element, and it is deprecated in favor of calling that explicitly. If you need to preserve the relative order of the keys in the map, use [`.shift_remove_entry(key)`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.shift_remove_entry "method indexmap::map::IndexMap::shift_remove_entry") instead.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1017-1019)

#### pub fn [swap\_remove](#method.swap_remove)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove the key-value pair equivalent to `key` and return its value.

Like [`Vec::swap_remove`](../../../prelude/struct.Vec.html#method.swap_remove "method bevy::prelude::Vec::swap_remove"), the pair is removed by swapping it with the last element of the map and popping it off. **This perturbs the position of what used to be the last element!**

Return `None` if `key` is not in map.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1033-1035)

#### pub fn [swap\_remove\_entry](#method.swap_remove_entry)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove and return the key-value pair equivalent to `key`.

Like [`Vec::swap_remove`](../../../prelude/struct.Vec.html#method.swap_remove "method bevy::prelude::Vec::swap_remove"), the pair is removed by swapping it with the last element of the map and popping it off. **This perturbs the position of what used to be the last element!**

Return `None` if `key` is not in map.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1053-1055)

#### pub fn [swap\_remove\_full](#method.swap_remove_full)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), K, V)>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove the key-value pair equivalent to `key` and return it and the index it had.

Like [`Vec::swap_remove`](../../../prelude/struct.Vec.html#method.swap_remove "method bevy::prelude::Vec::swap_remove"), the pair is removed by swapping it with the last element of the map and popping it off. **This perturbs the position of what used to be the last element!**

Return `None` if `key` is not in map.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1080-1082)

#### pub fn [shift\_remove](#method.shift_remove)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove the key-value pair equivalent to `key` and return its value.

Like [`Vec::remove`](../../../prelude/struct.Vec.html#method.remove "method bevy::prelude::Vec::remove"), the pair is removed by shifting all of the elements that follow it, preserving their relative order. **This perturbs the index of all of those elements!**

Return `None` if `key` is not in map.

Computes in **O(n)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1096-1098)

#### pub fn [shift\_remove\_entry](#method.shift_remove_entry)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove and return the key-value pair equivalent to `key`.

Like [`Vec::remove`](../../../prelude/struct.Vec.html#method.remove "method bevy::prelude::Vec::remove"), the pair is removed by shifting all of the elements that follow it, preserving their relative order. **This perturbs the index of all of those elements!**

Return `None` if `key` is not in map.

Computes in **O(n)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1116-1118)

#### pub fn [shift\_remove\_full](#method.shift_remove_full)<Q>(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), K, V)>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove the key-value pair equivalent to `key` and return it and the index it had.

Like [`Vec::remove`](../../../prelude/struct.Vec.html#method.remove "method bevy::prelude::Vec::remove"), the pair is removed by shifting all of the elements that follow it, preserving their relative order. **This perturbs the index of all of those elements!**

Return `None` if `key` is not in map.

Computes in **O(n)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1141)

#### pub fn [pop](#method.pop)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

Remove the last key-value pair

This preserves the order of the remaining elements.

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1166)

#### pub fn [pop\_if](#method.pop_if)( &mut self, predicate: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

Removes and returns the last key-value pair from a map if the predicate returns `true`, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the predicate returns false or the map is empty (the predicate will not be called in that case).

This preserves the order of the remaining elements.

Computes in **O(1)** time (average).

##### Examples

```rust
use indexmap::IndexMap;

let init = [(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')];
let mut map = IndexMap::from(init);
let pred = |key: &i32, _value: &mut char| *key % 2 == 0;

assert_eq!(map.pop_if(pred), Some((4, 'd')));
assert_eq!(map.as_slice(), &init[..3]);
assert_eq!(map.pop_if(pred), None);
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1182-1184)

#### pub fn [retain](#method.retain)<F>(&mut self, keep: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Scan through each key-value pair in the map and keep those where the closure `keep` returns `true`.

The elements are visited in order, and remaining elements keep their order.

Computes in **O(n)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1196-1198)

#### pub fn [sort\_keys](#method.sort_keys)(&mut self)

where K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Sort the map’s key-value pairs by the default ordering of the keys.

This is a stable sort – but equivalent keys should not normally coexist in a map at all, so [`sort_unstable_keys`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_unstable_keys "method indexmap::map::IndexMap::sort_unstable_keys") is preferred because it is generally faster and doesn’t allocate auxiliary memory.

See [`sort_by`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_by "method indexmap::map::IndexMap::sort_by") for details.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1213-1215)

#### pub fn [sort\_by](#method.sort_by)<F>(&mut self, cmp: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Sort the map’s key-value pairs in place using the comparison function `cmp`.

The comparison function receives two key and value pairs to compare (you can sort by keys or values or their combination as needed).

Computes in **O(n log n + c)** time and **O(n)** space where _n_ is the length of the map and _c_ the capacity. The sort is stable.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1239-1242)

#### pub fn [sort\_by\_key](#method.sort_by_key)<T, F>(&mut self, sort\_key: F)

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> T,

Sort the map’s key-value pairs in place using a sort-key extraction function.

Computes in **O(n log n + c)** time and **O(n)** space where _n_ is the length of the map and _c_ the capacity. The sort is stable.

##### [Examples found in repository](#scraped-examples-5)[?](../../../../scrape-examples-help.html)

examples/shader\_advanced/custom\_render\_phase.rs ([line 323](../../../../src/custom_render_phase/custom_render_phase.rs.html#323))

```rust
322    fn sort(items: &mut IndexMap<(Entity, MainEntity), Stencil3d, EntityHash>) {
323        items.sort_by_key(|_, phase_item: &Stencil3d| phase_item.distance);
324    }
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1253-1255)

#### pub fn [sort\_unstable\_keys](#method.sort_unstable_keys)(&mut self)

where K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Sort the map’s key-value pairs by the default ordering of the keys, but may not preserve the order of equal elements.

See [`sort_unstable_by`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.sort_unstable_by "method indexmap::map::IndexMap::sort_unstable_by") for details.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1270-1272)

#### pub fn [sort\_unstable\_by](#method.sort_unstable_by)<F>(&mut self, cmp: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Sort the map’s key-value pairs in place using the comparison function `cmp`, but may not preserve the order of equal elements.

The comparison function receives two key and value pairs to compare (you can sort by keys or values or their combination as needed).

Computes in **O(n log n + c)** time where _n_ is the length of the map and _c_ is the capacity. The sort is unstable.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1297-1300)

#### pub fn [sort\_unstable\_by\_key](#method.sort_unstable_by_key)<T, F>(&mut self, sort\_key: F)

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> T,

Sort the map’s key-value pairs in place using a sort-key extraction function.

Computes in **O(n log n + c)** time where _n_ is the length of the map and _c_ is the capacity. The sort is unstable.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1315-1318)

#### pub fn [sort\_by\_cached\_key](#method.sort_by_cached_key)<T, F>(&mut self, sort\_key: F)

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> T,

Sort the map’s key-value pairs in place using a sort-key extraction function.

During sorting, the function is called at most once per entry, by using temporary storage to remember the results of its evaluation. The order of calls to the function is unspecified and may change between versions of `indexmap` or the standard library.

Computes in **O(m n + n log n + c)** time () and **O(n)** space, where the function is **O(m)**, _n_ is the length of the map, and _c_ the capacity. The sort is stable.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1332-1334)

#### pub fn [binary\_search\_keys](#method.binary_search_keys)(&self, x: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Search over a sorted map for a key.

Returns the position where that key is present, or the position where it can be inserted to maintain the sort. See [`slice::binary_search`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search "method slice::binary_search") for more details.

Computes in **O(log(n))** time, which is notably less scalable than looking the key up using [`get_index_of`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html#method.get_index_of "method indexmap::map::IndexMap::get_index_of"), but this can also position missing keys.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1346-1348)

#### pub fn [binary\_search\_by](#method.binary_search_by)<'a, F>(&'a self, f: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Search over a sorted map with a comparator function.

Returns the position where that value is present, or the position where it can be inserted to maintain the sort. See [`slice::binary_search_by`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by "method slice::binary_search_by") for more details.

Computes in **O(log(n))** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1360-1363)

#### pub fn [binary\_search\_by\_key](#method.binary_search_by_key)<'a, B, F>( &'a self, b: [&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html), f: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> B, B: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Search over a sorted map with an extraction function.

Returns the position where that value is present, or the position where it can be inserted to maintain the sort. See [`slice::binary_search_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.binary_search_by_key "method slice::binary_search_by_key") for more details.

Computes in **O(log(n))** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1370-1372)

#### pub fn [is\_sorted](#method.is_sorted)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where K: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if the keys of this map are sorted.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1379-1381)

#### pub fn [is\_sorted\_by](#method.is_sorted_by)<'a, F>(&'a self, cmp: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Checks if this map is sorted using the given comparator function.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1388-1391)

#### pub fn [is\_sorted\_by\_key](#method.is_sorted_by_key)<'a, F, T>(&'a self, sort\_key: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> T, T: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if this map is sorted using the given sort-key function.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1403-1405)

#### pub fn [partition\_point](#method.partition_point)<P>(&self, pred: P) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns the index of the partition point of a sorted map according to the given predicate (the index of the first element of the second partition).

See [`slice::partition_point`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.partition_point "method slice::partition_point") for more details.

Computes in **O(log(n))** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1413)

#### pub fn [reverse](#method.reverse)(&mut self)

Reverses the order of the map’s key-value pairs in place.

Computes in **O(n)** time and **O(1)** space.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1420)

#### pub fn [as\_slice](#method.as_slice-1)(&self) -> &[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>

Returns a slice of all the key-value pairs in the map.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1427)

#### pub fn [as\_mut\_slice](#method.as_mut_slice-1)(&mut self) -> &mut [Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>

Returns a mutable slice of all the key-value pairs in the map.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1443)

#### pub fn [get\_index](#method.get_index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get a key-value pair by index

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1452)

#### pub fn [get\_index\_mut](#method.get_index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get a key-value pair by index

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1461)

#### pub fn [get\_index\_entry](#method.get_index_entry)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IndexedEntry](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/entry/struct.IndexedEntry.html "struct indexmap::map::entry::IndexedEntry")<'\_, K, V>>

Get an entry in the map by index for in-place manipulation.

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1475-1478)

#### pub fn [get\_disjoint\_indices\_mut](#method.get_disjoint_indices_mut)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, indices: \[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [GetDisjointMutError](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/enum.GetDisjointMutError.html "enum indexmap::GetDisjointMutError")\>

Get an array of `N` key-value pairs by `N` indices

Valid indices are _0 <= index < self.len()_ and each index needs to be unique.

##### Examples

```rust
let mut map = indexmap::IndexMap::from([(1, 'a'), (3, 'b'), (2, 'c')]);
assert_eq!(map.get_disjoint_indices_mut([2, 0]), Ok([(&2, &mut 'c'), (&1, &mut 'a')]));
```

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1487)

#### pub fn [get\_range](#method.get_range-1)<R>(&self, range: R) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>>

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Returns a slice of key-value pairs in the given range of indices.

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1498)

#### pub fn [get\_range\_mut](#method.get_range_mut-1)<R>(&mut self, range: R) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [Slice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/slice/struct.Slice.html "struct indexmap::map::slice::Slice")<K, V>>

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Returns a mutable slice of key-value pairs in the given range of indices.

Valid indices are `0 <= index < self.len()`.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1508)

#### pub fn [first](#method.first)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get the first key-value pair

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1515)

#### pub fn [first\_mut](#method.first_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get the first key-value pair, with mutable access to the value

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1522)

#### pub fn [first\_entry](#method.first_entry)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IndexedEntry](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/entry/struct.IndexedEntry.html "struct indexmap::map::entry::IndexedEntry")<'\_, K, V>>

Get the first entry in the map for in-place manipulation.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1530)

#### pub fn [last](#method.last)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get the last key-value pair

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1537)

#### pub fn [last\_mut](#method.last_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Get the last key-value pair, with mutable access to the value

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1544)

#### pub fn [last\_entry](#method.last_entry)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IndexedEntry](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/entry/struct.IndexedEntry.html "struct indexmap::map::entry::IndexedEntry")<'\_, K, V>>

Get the last entry in the map for in-place manipulation.

Computes in **O(1)** time.

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1557)

#### pub fn [swap\_remove\_index](#method.swap_remove_index)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

Remove the key-value pair by index

Valid indices are `0 <= index < self.len()`.

Like [`Vec::swap_remove`](../../../prelude/struct.Vec.html#method.swap_remove "method bevy::prelude::Vec::swap_remove"), the pair is removed by swapping it with the last element of the map and popping it off. **This perturbs the position of what used to be the last element!**

Computes in **O(1)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1570)

#### pub fn [shift\_remove\_index](#method.shift_remove_index)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

Remove the key-value pair by index

Valid indices are `0 <= index < self.len()`.

Like [`Vec::remove`](../../../prelude/struct.Vec.html#method.remove "method bevy::prelude::Vec::remove"), the pair is removed by shifting all of the elements that follow it, preserving their relative order. **This perturbs the index of all of those elements!**

Computes in **O(n)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1584)

#### pub fn [move\_index](#method.move_index)(&mut self, from: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), to: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Moves the position of a key-value pair from one index to another by shifting all other pairs in-between.

*   If `from < to`, the other pairs will shift down while the targeted pair moves up.
*   If `from > to`, the other pairs will shift up while the targeted pair moves down.

_**Panics**_ if `from` or `to` are out of bounds.

Computes in **O(n)** time (average).

[Source](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/src/indexmap/map.rs.html#1594)

#### pub fn [swap\_indices](#method.swap_indices)(&mut self, a: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), b: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Swaps the position of two key-value pairs in the map.

_**Panics**_ if `a` or `b` are out of bounds.

Computes in **O(1)** time (average).

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#34)

### impl<V> [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#34)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#34)

### impl<V> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#34)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#145)

### impl<V> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#146)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#151)

### impl<V> [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#152)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#154)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#159)

### impl<V> [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#160)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#33)

### impl<'de, V> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#33)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>, <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#375)

### impl<V> [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#165)

### impl<'a, V> [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<(&'a [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#166)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<T>(&mut self, iter: T)

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = (&'a [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#171)

### impl<V> [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<([Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V)> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#172)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<T>(&mut self, iter: T)

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V)>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#177)

### impl<V, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[([Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#178)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: \[([Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [FromArg](../../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### type [This](../../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

The type to convert into. [Read more](../../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [from\_arg](../../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)( arg: [Arg](../../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [FromArg](../../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#183)

### impl<V> [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<([Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V)> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#184)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iterable: I) -> [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V)>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [from\_reflect](../../../prelude/trait.FromReflect.html#tymethod.from_reflect)( reflect: &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [GetOwnership](../../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [ownership](../../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [GetTypeRegistration](../../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [get\_type\_registration](../../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [register\_type\_dependencies](../../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#189)

### impl<V, Q> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where Q: [EntityEquivalent](../trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#190)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = V

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#192)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#197)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#198)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#200)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)( &self, key: ([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>), ) -> &<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#206)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#207)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#209)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)( &self, key: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#215)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#216)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#218)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)( &self, key: [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#224)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#225)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#227)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)( &self, key: [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull"), ) -> &<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#233)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#234)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#236)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)( &self, key: [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#242)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#243)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#245)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)( &self, key: [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#251)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#252)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [Slice](struct.Slice.html "struct bevy::ecs::entity::index_map::Slice")<V>

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#254)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)( &self, key: [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#260)

### impl<V> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#261)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = V

The returned type after indexing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#263)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, key: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#268)

### impl<V, Q> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where Q: [EntityEquivalent](../trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#269)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#274)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#275)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: ([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>), ) -> &mut <[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<([Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#281)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#282)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut <[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#288)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#289)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut <[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#295)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#296)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull"), ) -> &mut <[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#302)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#303)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut <[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#309)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#310)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut <[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#316)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#317)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, key: [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> &mut <[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#323)

### impl<V> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#324)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, key: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#329)

### impl<'a, V> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#330)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = (&'a [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&'a V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#331)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [Iter](struct.Iter.html "struct bevy::ecs::entity::index_map::Iter")<'a, V>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#333)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'a [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#338)

### impl<'a, V> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a mut [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#339)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = (&'a [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [&'a mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#340)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [IterMut](struct.IterMut.html "struct bevy::ecs::entity::index_map::IterMut")<'a, V>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#342)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'a mut [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#347)

### impl<V> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#348)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = ([Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V)

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#349)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [IntoIter](struct.IntoIter.html "struct bevy::ecs::entity::index_map::IntoIter")<V>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#351)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [IntoReturn](../../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [into\_return](../../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: 'into\_return,

Converts [`Self`](../../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#366-368)

### impl<V1, V2> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V2>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V1>

where V1: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<V2>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#370)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V2>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#356-359)

### impl<V1, V2, S2> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V2, S2>> for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V1>

where V1: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<V2>, S2: [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#361)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V2, S2>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [get\_represented\_type\_info](../../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [try\_apply](../../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [reflect\_kind](../../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [reflect\_ref](../../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [reflect\_mut](../../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [reflect\_owned](../../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>>) -> [ReflectOwned](../../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [try\_into\_reflect](../../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [try\_as\_reflect](../../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [try\_as\_reflect\_mut](../../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [into\_partial\_reflect](../../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [as\_partial\_reflect](../../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [as\_partial\_reflect\_mut](../../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [reflect\_partial\_eq](../../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [reflect\_partial\_cmp](../../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [into\_any](../../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [as\_any](../../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [as\_any\_mut](../../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [into\_reflect](../../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<[EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [as\_reflect](../../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [as\_reflect\_mut](../../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [set](../../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#33)

### impl<V> [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#33)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [TupleStruct](../../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [field](../../../prelude/trait.TupleStruct.html#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field with index `index` as a `&dyn Reflect`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [field\_mut](../../../prelude/trait.TupleStruct.html#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field with index `index` as a `&mut dyn Reflect`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [field\_len](../../../prelude/trait.TupleStruct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the tuple struct.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [iter\_fields](../../../prelude/trait.TupleStruct.html#tymethod.iter_fields)(&self) -> [TupleStructFieldIter](../../../reflect/tuple_struct/struct.TupleStructFieldIter.html "struct bevy::reflect::tuple_struct::TupleStructFieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the tuple struct’s fields.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [to\_dynamic\_tuple\_struct](../../../prelude/trait.TupleStruct.html#method.to_dynamic_tuple_struct)(&self) -> [DynamicTupleStruct](../../../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct")

Creates a new [`DynamicTupleStruct`](../../../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct") from this tuple struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#71)

#### fn [get\_represented\_tuple\_struct\_info](../../../prelude/trait.TupleStruct.html#method.get_represented_tuple_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TupleStructInfo](../../../reflect/tuple_struct/struct.TupleStructInfo.html "struct bevy::reflect::tuple_struct::TupleStructInfo")\>

Will return `None` if [`TypeInfo`](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [type\_path](../../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [short\_type\_path](../../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [type\_ident](../../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [crate\_name](../../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [module\_path](../../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [Typed](../../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

#### fn [type\_info](../../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl<V> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

### impl<V> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<V> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

### impl<V> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

### impl<V> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

### impl<V> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

### impl<V> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [EntityIndexMap](../struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where V: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Drain<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Drain.html\\" title=\\"struct indexmap::map::iter::Drain\\">Drain</a>&lt;'\_, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Drain.html\\" title=\\"struct indexmap::map::iter::Drain\\">Drain</a>&lt;'\_, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(K, V)</a>;</div>","Drain<'\_, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Drain.html\\" title=\\"struct bevy::ecs::entity::index\_map::Drain\\">Drain</a>&lt;'\_, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.Drain.html\\" title=\\"struct bevy::ecs::entity::index\_map::Drain\\">Drain</a>&lt;'\_, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>, V);</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","ExtractIf<'\_, K, V, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.ExtractIf.html\\" title=\\"struct indexmap::map::iter::ExtractIf\\">ExtractIf</a>&lt;'\_, K, V, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, V, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.ExtractIf.html\\" title=\\"struct indexmap::map::iter::ExtractIf\\">ExtractIf</a>&lt;'\_, K, V, F&gt;<div class=\\"where\\">where\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;K</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;mut V</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(K, V)</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","IntoKeys<V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.IntoKeys.html\\" title=\\"struct bevy::ecs::entity::index\_map::IntoKeys\\">IntoKeys</a>&lt;V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.IntoKeys.html\\" title=\\"struct bevy::ecs::entity::index\_map::IntoKeys\\">IntoKeys</a>&lt;V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","Iter<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Iter.html\\" title=\\"struct indexmap::map::iter::Iter\\">Iter</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Iter.html\\" title=\\"struct indexmap::map::iter::Iter\\">Iter</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a K</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a V</a>);</div>","Iter<'\_, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Iter.html\\" title=\\"struct bevy::ecs::entity::index\_map::Iter\\">Iter</a>&lt;'a, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.Iter.html\\" title=\\"struct bevy::ecs::entity::index\_map::Iter\\">Iter</a>&lt;'a, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a V</a>);</div>","IterMut<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.IterMut.html\\" title=\\"struct indexmap::map::iter::IterMut\\">IterMut</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.IterMut.html\\" title=\\"struct indexmap::map::iter::IterMut\\">IterMut</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a K</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut V</a>);</div>","IterMut<'\_, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.IterMut.html\\" title=\\"struct bevy::ecs::entity::index\_map::IterMut\\">IterMut</a>&lt;'a, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.IterMut.html\\" title=\\"struct bevy::ecs::entity::index\_map::IterMut\\">IterMut</a>&lt;'a, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut V</a>);</div>","Keys<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Keys.html\\" title=\\"struct indexmap::map::iter::Keys\\">Keys</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Keys.html\\" title=\\"struct indexmap::map::iter::Keys\\">Keys</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a K</a>;</div>","Keys<'\_, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Keys.html\\" title=\\"struct bevy::ecs::entity::index\_map::Keys\\">Keys</a>&lt;'a, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.Keys.html\\" title=\\"struct bevy::ecs::entity::index\_map::Keys\\">Keys</a>&lt;'a, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"struct\\" href=\\"../../../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","Splice<'\_, <I as IntoIterator>::IntoIter, K, V, S>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Splice.html\\" title=\\"struct indexmap::map::iter::Splice\\">Splice</a>&lt;'\_, I, K, V, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, K, V, S&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Splice.html\\" title=\\"struct indexmap::map::iter::Splice\\">Splice</a>&lt;'\_, I, K, V, S&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(K, V)</a>&gt;,\\n K: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a>,\\n S: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html\\" title=\\"trait core::hash::BuildHasher\\">BuildHasher</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(K, V)</a>;</div>","TupleStructFieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a (dyn <a class=\\"trait\\" href=\\"../../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static);</div>","Values<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Values.html\\" title=\\"struct indexmap::map::iter::Values\\">Values</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.Values.html\\" title=\\"struct indexmap::map::iter::Values\\">Values</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a V</a>;</div>","ValuesMut<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.ValuesMut.html\\" title=\\"struct indexmap::map::iter::ValuesMut\\">ValuesMut</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/indexmap/2.14.0/x86\_64-unknown-linux-gnu/indexmap/map/iter/struct.ValuesMut.html\\" title=\\"struct indexmap::map::iter::ValuesMut\\">ValuesMut</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut V</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}