[bevy](../../index.html)::[ecs](../index.html)::[message](index.html)

# Struct MessageIteratorWithId 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#46)

```rust
pub struct MessageIteratorWithId<'a, M>where
    M: Message,{ /* private fields */ }
```

An iterator that yields any unread messages (and their IDs) from a [`MessageReader`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader") or [`MessageCursor`](struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor").

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#52)

### impl<'a, M> [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#54)

#### pub fn [new](#method.new)( reader: &'a mut [MessageCursor](struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor")<M>, messages: &'a [Messages](../../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>, ) -> [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M> [ⓘ](#)

Creates a new iterator that yields any `messages` that have not yet been seen by `reader`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#79)

#### pub fn [without\_id](#method.without_id)(self) -> [MessageIterator](struct.MessageIterator.html "struct bevy::ecs::message::MessageIterator")<'a, M> [ⓘ](#)

Iterate over only the messages.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#45)

### impl<'a, M> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

where M: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#45)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#141)

### impl<'a, M> [ExactSizeIterator](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html "trait core::iter::traits::exact_size::ExactSizeIterator") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#142)

#### fn [len](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the exact remaining length of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html#method.len)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/exact_size.rs.html#148)

#### fn [is\_empty](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

🔬This is a nightly-only experimental API. (`exact_size_is_empty`)

Returns `true` if the iterator is empty. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html#method.is_empty)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#84)

### impl<'a, M> [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#85)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item) = ([&'a M](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [MessageId](struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>)

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#86)

#### fn [next](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M> as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

Advances the iterator and returns the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#103)

#### fn [size\_hint](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

Returns the bounds on the remaining length of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.size_hint)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#107)

#### fn [count](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.count)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Consumes the iterator, counting the number of iterations and returning it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.count)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#112-114)

#### fn [last](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.last)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M> as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Consumes the iterator, returning the last element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.last)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/iterators.rs.html#124)

#### fn [nth](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.nth)( &mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M> as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

Returns the `n`th element of the iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.nth)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#112-116)

#### fn [next\_chunk](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.next_chunk)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [IntoIter](https://doc.rust-lang.org/nightly/core/array/iter/struct.IntoIter.html "struct core::array::iter::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), N>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`iter_next_chunk`)

Advances the iterator and returns an array containing the next `N` values. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.next_chunk)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#310)

#### fn [advance\_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.advance_by)(&mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>

🔬This is a nightly-only experimental API. (`iter_advance_by`)

Advances the iterator by `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.advance_by)

1.28.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#439-441)

#### fn [step\_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.step_by)(self, step: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [StepBy](https://doc.rust-lang.org/nightly/core/iter/adapters/step_by/struct.StepBy.html "struct core::iter::adapters::step_by::StepBy")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator starting at the same point, but stepping by the given amount at each iteration. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.step_by)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#510-513)

#### fn [chain](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.chain)<U>(self, other: U) -> [Chain](https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html "struct core::iter::adapters::chain::Chain")<Self, <U as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Takes two iterators and creates a new iterator over both in sequence. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.chain)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#629-632)

#### fn [zip](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.zip)<U>(self, other: U) -> [Zip](https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html "struct core::iter::adapters::zip::Zip")<Self, <U as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

‘Zips up’ two iterators into a single iterator of pairs. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.zip)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#693-696)

#### fn [intersperse](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse)(self, separator: Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Intersperse](https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.Intersperse.html "struct core::iter::adapters::intersperse::Intersperse")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

🔬This is a nightly-only experimental API. (`iter_intersperse`)

Creates a new iterator which places a copy of `separator` between items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#772-775)

#### fn [intersperse\_with](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse_with)<G>(self, separator: G) -> [IntersperseWith](https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.IntersperseWith.html "struct core::iter::adapters::intersperse::IntersperseWith")<Self, G> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), G: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")() -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"),

🔬This is a nightly-only experimental API. (`iter_intersperse`)

Creates a new iterator which places an item generated by `separator` between items of the original iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.intersperse_with)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#831-834)

#### fn [map](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map)<B, F>(self, f: F) -> [Map](https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html "struct core::iter::adapters::map::Map")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> B,

Takes a closure and creates an iterator which calls that closure on each element. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map)

1.21.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#877-880)

#### fn [for\_each](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each)<F>(self, f: F)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")),

Calls a closure on each element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.for_each)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#952-955)

#### fn [filter](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter)<P>(self, predicate: P) -> [Filter](https://doc.rust-lang.org/nightly/core/iter/adapters/filter/struct.Filter.html "struct core::iter::adapters::filter::Filter")<Self, P> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Creates an iterator which uses a closure to determine if an element should be yielded. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#997-1000)

#### fn [filter\_map](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter_map)<B, F>(self, f: F) -> [FilterMap](https://doc.rust-lang.org/nightly/core/iter/adapters/filter_map/struct.FilterMap.html "struct core::iter::adapters::filter_map::FilterMap")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Creates an iterator that both filters and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.filter_map)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1044-1046)

#### fn [enumerate](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.enumerate)(self) -> [Enumerate](https://doc.rust-lang.org/nightly/core/iter/adapters/enumerate/struct.Enumerate.html "struct core::iter::adapters::enumerate::Enumerate")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator which gives the current iteration count as well as the next value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.enumerate)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1115-1117)

#### fn [peekable](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.peekable)(self) -> [Peekable](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html "struct core::iter::adapters::peekable::Peekable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator which can use the [`peek`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek "method core::iter::adapters::peekable::Peekable::peek") and [`peek_mut`](https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html#method.peek_mut "method core::iter::adapters::peekable::Peekable::peek_mut") methods to look at the next element of the iterator without consuming it. See their documentation for more information. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.peekable)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1180-1183)

#### fn [skip\_while](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while)<P>(self, predicate: P) -> [SkipWhile](https://doc.rust-lang.org/nightly/core/iter/adapters/skip_while/struct.SkipWhile.html "struct core::iter::adapters::skip_while::SkipWhile")<Self, P> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Creates an iterator that [`skip`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip "method core::iter::traits::iterator::Iterator::skip")s elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip_while)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1258-1261)

#### fn [take\_while](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while)<P>(self, predicate: P) -> [TakeWhile](https://doc.rust-lang.org/nightly/core/iter/adapters/take_while/struct.TakeWhile.html "struct core::iter::adapters::take_while::TakeWhile")<Self, P> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Creates an iterator that yields elements based on a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take_while)

1.57.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1346-1349)

#### fn [map\_while](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_while)<B, P>(self, predicate: P) -> [MapWhile](https://doc.rust-lang.org/nightly/core/iter/adapters/map_while/struct.MapWhile.html "struct core::iter::adapters::map_while::MapWhile")<Self, P> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Creates an iterator that both yields elements based on a predicate and maps. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_while)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1375-1377)

#### fn [skip](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Skip](https://doc.rust-lang.org/nightly/core/iter/adapters/skip/struct.Skip.html "struct core::iter::adapters::skip::Skip")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator that skips the first `n` elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.skip)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1447-1449)

#### fn [take](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Take](https://doc.rust-lang.org/nightly/core/iter/adapters/take/struct.Take.html "struct core::iter::adapters::take::Take")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator that yields the first `n` elements, or fewer if the underlying iterator ends sooner. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.take)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1494-1497)

#### fn [scan](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.scan)<St, B, F>(self, initial\_state: St, f: F) -> [Scan](https://doc.rust-lang.org/nightly/core/iter/adapters/scan/struct.Scan.html "struct core::iter::adapters::scan::Scan")<Self, St, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut St](https://doc.rust-lang.org/nightly/std/primitive.reference.html), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

An iterator adapter which, like [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "method core::iter::traits::iterator::Iterator::fold"), holds internal state, but unlike [`fold`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold "method core::iter::traits::iterator::Iterator::fold"), produces a new iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.scan)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1533-1537)

#### fn [flat\_map](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flat_map)<U, F>(self, f: F) -> [FlatMap](https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.FlatMap.html "struct core::iter::adapters::flatten::FlatMap")<Self, U, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> U,

Creates an iterator that works like map, but flattens nested structure. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flat_map)

1.29.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1617-1620)

#### fn [flatten](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flatten)(self) -> [Flatten](https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.Flatten.html "struct core::iter::adapters::flatten::Flatten")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Creates an iterator that flattens nested structure. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.flatten)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1771-1774)

#### fn [map\_windows](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_windows)<F, R, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(self, f: F) -> [MapWindows](https://doc.rust-lang.org/nightly/core/iter/adapters/map_windows/struct.MapWindows.html "struct core::iter::adapters::map_windows::MapWindows")<Self, F, N> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&\[Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> R,

🔬This is a nightly-only experimental API. (`iter_map_windows`)

Calls the given function `f` for each contiguous window of size `N` over `self` and returns an iterator over the outputs of `f`. Like [`slice::windows()`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.windows "method slice::windows"), the windows during mapping overlap as well. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.map_windows)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1833-1835)

#### fn [fuse](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fuse)(self) -> [Fuse](https://doc.rust-lang.org/nightly/core/iter/adapters/fuse/struct.Fuse.html "struct core::iter::adapters::fuse::Fuse")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an iterator which ends after the first [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fuse)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1917-1920)

#### fn [inspect](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.inspect)<F>(self, f: F) -> [Inspect](https://doc.rust-lang.org/nightly/core/iter/adapters/inspect/struct.Inspect.html "struct core::iter::adapters::inspect::Inspect")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")),

Does something with each element of an iterator, passing the value on. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.inspect)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#1954-1956)

#### fn [by\_ref](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.by_ref)(&mut self) -> &mut Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates a “by reference” adapter for this instance of `Iterator`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.by_ref)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2077-2079)

#### fn [collect](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect)<B>(self) -> B

where B: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Transforms an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2165-2169)

#### fn [try\_collect](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_collect)<B>( &mut self, ) -> <<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")\>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual") as [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<B>>::[TryType](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType "type core::ops::try_trait::Residual::TryType")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try"), <Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")\>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual"): [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<B>, B: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Output "type core::ops::try_trait::Try::Output")\>,

🔬This is a nightly-only experimental API. (`iterator_try_collect`)

Fallibly transforms an iterator into a collection, short circuiting if a failure is encountered. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_collect)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2238-2240)

#### fn [collect\_into](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect_into)<E>(self, collection: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where E: [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`iter_collect_into`)

Collects all the items from an iterator into a collection. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect_into)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2271-2275)

#### fn [partition](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition)<B, F>(self, f: F) -> [(B, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Consumes an iterator, creating two collections from it. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2334-2337)

#### fn [partition\_in\_place](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition_in_place)<'a, T, P>(self, predicate: P) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where T: 'a, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator")<Item = [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

🔬This is a nightly-only experimental API. (`iter_partition_in_place`)

Reorders the elements of this iterator _in-place_ according to the given predicate, such that all those that return `true` precede all those that return `false`. Returns the number of `true` elements found. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition_in_place)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2392-2395)

#### fn [is\_partitioned](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_partitioned)<P>(self, predicate: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

🔬This is a nightly-only experimental API. (`iter_is_partitioned`)

Checks if the elements of this iterator are partitioned according to the given predicate, such that all those that return `true` precede all those that return `false`. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_partitioned)

1.27.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2486-2490)

#### fn [try\_fold](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold)<B, F, R>(&mut self, init: B, f: F) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R, R: [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")<Output = B>,

An iterator method that applies a function as long as it returns successfully, producing a single, final value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_fold)

1.27.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2545-2549)

#### fn [try\_for\_each](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_for_each)<F, R>(&mut self, f: F) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R, R: [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")<Output = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>,

An iterator method that applies a fallible function to each item in the iterator, stopping at the first error and returning that error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_for_each)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2664-2667)

#### fn [fold](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold)<B, F>(self, init: B, f: F) -> B

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> B,

Folds every element into an accumulator by applying an operation, returning the final result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.fold)

1.51.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2701-2704)

#### fn [reduce](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce)<F>(self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"),

Reduces the elements to a single one, by repeatedly applying a reducing operation. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.reduce)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2772-2778)

#### fn [try\_reduce](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_reduce)<R>( &mut self, f: impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R, ) -> <<R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")\>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual") as [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Output "type core::ops::try_trait::Try::Output")\>>>::[TryType](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType "type core::ops::try_trait::Residual::TryType")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")<Output = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, <R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")\>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual"): [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>>,

🔬This is a nightly-only experimental API. (`iterator_try_reduce`)

Reduces the elements to a single one by repeatedly applying a reducing operation. If the closure returns a failure, the failure is propagated back to the caller immediately. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_reduce)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2831-2834)

#### fn [all](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.all)<F>(&mut self, f: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Tests if every element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.all)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2885-2888)

#### fn [any](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.any)<F>(&mut self, f: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Tests if any element of the iterator matches a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.any)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2959-2962)

#### fn [find](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find)<P>(&mut self, predicate: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Searches for an element of an iterator that satisfies a predicate. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find)

1.30.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#2991-2994)

#### fn [find\_map](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find_map)<B, F>(&mut self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Applies function to the elements of iterator and returns the first non-none result. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.find_map)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3050-3056)

#### fn [try\_find](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_find)<R>( &mut self, f: impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R, ) -> <<R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")\>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual") as [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>>>::[TryType](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html#associatedtype.TryType "type core::ops::try_trait::Residual::TryType")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>, <R as [Try](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html "trait core::ops::try_trait::Try")\>::[Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Try.html#associatedtype.Residual "type core::ops::try_trait::Try::Residual"): [Residual](https://doc.rust-lang.org/nightly/core/ops/try_trait/trait.Residual.html "trait core::ops::try_trait::Residual")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>>,

🔬This is a nightly-only experimental API. (`try_find`)

Applies function to the elements of iterator and returns the first true result or the first error. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.try_find)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3134-3137)

#### fn [position](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position)<P>(&mut self, predicate: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Searches for an element in an iterator, returning its index. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.position)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3200-3203)

#### fn [rposition](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rposition)<P>(&mut self, predicate: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [ExactSizeIterator](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html "trait core::iter::traits::exact_size::ExactSizeIterator") + [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator"),

Searches for an element in an iterator from the right, returning its index. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rposition)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3250-3253)

#### fn [max](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Returns the maximum element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3287-3290)

#### fn [min](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Returns the minimum element of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min)

1.6.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3310-3313)

#### fn [max\_by\_key](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by_key)<B, F>(self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where B: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> B,

Returns the element that gives the maximum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by_key)

1.15.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3344-3347)

#### fn [max\_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by)<F>(self, compare: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Returns the element that gives the maximum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.max_by)

1.6.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3372-3375)

#### fn [min\_by\_key](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by_key)<B, F>(self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where B: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> B,

Returns the element that gives the minimum value from the specified function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by_key)

1.15.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3406-3409)

#### fn [min\_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by)<F>(self, compare: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Returns the element that gives the minimum value with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.min_by)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3443-3445)

#### fn [rev](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rev)(self) -> [Rev](https://doc.rust-lang.org/nightly/core/iter/adapters/rev/struct.Rev.html "struct core::iter::adapters::rev::Rev")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator"),

Reverses an iterator’s direction. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.rev)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3480-3484)

#### fn [unzip](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.unzip)<A, B, FromA, FromB>(self) -> [(FromA, FromB)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where FromA: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<A>, FromB: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<B>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

Converts an iterator of pairs into a pair of containers. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.unzip)

1.36.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3511-3514)

#### fn [copied](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.copied)<'a, T>(self) -> [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<Self> [ⓘ](#)

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + 'a, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

Creates an iterator which copies all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.copied)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3559-3562)

#### fn [cloned](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cloned)<'a, T>(self) -> [Cloned](https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html "struct core::iter::adapters::cloned::Cloned")<Self> [ⓘ](#)

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + 'a, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

Creates an iterator which [`clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone "method core::clone::Clone::clone")s all of its elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cloned)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3590-3592)

#### fn [cycle](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cycle)(self) -> [Cycle](https://doc.rust-lang.org/nightly/core/iter/adapters/cycle/struct.Cycle.html "struct core::iter::adapters::cycle::Cycle")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Repeats an iterator endlessly. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cycle)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3633-3635)

#### fn [array\_chunks](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.array_chunks)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(self) -> [ArrayChunks](https://doc.rust-lang.org/nightly/core/iter/adapters/array_chunks/struct.ArrayChunks.html "struct core::iter::adapters::array_chunks::ArrayChunks")<Self, N> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`iter_array_chunks`)

Returns an iterator over `N` elements of the iterator at a time. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.array_chunks)

1.11.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3669-3672)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.sum)<S>(self) -> S

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Sums the elements of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.sum)

1.11.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3701-3704)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.product)<P>(self) -> P

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Iterates over the entire iterator, multiplying all the elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.product)

1.5.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3723-3727)

#### fn [cmp](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp)<I>(self, other: I) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3751-3755)

#### fn [cmp\_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp_by)<I, F>(self, other: I, cmp: F) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

🔬This is a nightly-only experimental API. (`iter_order_by`)

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.cmp_by)

1.5.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3808-3812)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp)<I>(self, other: I) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another. The comparison works like short-circuit evaluation, returning a result without comparing the remaining elements. As soon as an order can be determined, the evaluation stops and a result is returned. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3845-3849)

#### fn [partial\_cmp\_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp_by)<I, F>(self, other: I, partial\_cmp: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>,

🔬This is a nightly-only experimental API. (`iter_order_by`)

[Lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") compares the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") with those of another with respect to the specified comparison function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partial_cmp_by)

1.5.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3879-3883)

#### fn [eq](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3903-3907)

#### fn [eq\_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq_by)<I, F>(self, other: I, eq: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

🔬This is a nightly-only experimental API. (`iter_order_by`)

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are equal to those of another with respect to the specified equality function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.eq_by)

1.5.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3933-3937)

#### fn [ne](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ne)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are not equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ne)

1.5.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3955-3959)

#### fn [lt](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.lt)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") less than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.lt)

1.5.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3977-3981)

#### fn [le](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.le)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") less or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.le)

1.5.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3999-4003)

#### fn [gt](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.gt)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") greater than those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.gt)

1.5.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#4021-4025)

#### fn [ge](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ge)<I>(self, other: I) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Determines if the elements of this [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") are [lexicographically](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison "trait core::cmp::Ord") greater than or equal to those of another. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.ge)

1.82.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#4051-4054)

#### fn [is\_sorted](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if the elements of this iterator are sorted. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted)

1.82.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#4078-4081)

#### fn [is\_sorted\_by](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by)<F>(self, compare: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Checks if the elements of this iterator are sorted using the given comparator function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by)

1.82.0 (const: [unstable](https://github.com/rust-lang/rust/issues/92476 "Tracking issue for const_iter")) · [Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#4123-4127)

#### fn [is\_sorted\_by\_key](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by_key)<F, K>(self, f: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K, K: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Checks if the elements of this iterator are sorted using the given key extraction function. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.is_sorted_by_key)

## Auto Trait Implementations

### impl<'a, M> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

### impl<'a, M> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

### impl<'a, M> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

where M: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<'a, M> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

### impl<'a, M> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

### impl<'a, M> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

### impl<'a, M> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")<'a, M>

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

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

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

### impl<T> [Instrument](../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.in_current_span)

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

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#317)

### impl<I> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for I

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"),

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#318)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = <I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")

The type of the elements being iterated over.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#319)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = I

Which kind of iterator are we turning this into?

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#322)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> I

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#285)

### impl<I> [IteratorRandom](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html "trait rand::seq::iterator::IteratorRandom") for I

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"),

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#66-68)

#### fn [choose](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Uniformly sample one element [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#145-147)

#### fn [choose\_stable](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_stable)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Uniformly sample one element (stable) [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_stable)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#203-205)

#### fn [sample\_fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.sample_fill)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html), buf: &mut \[Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\]) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Uniformly sample `amount` distinct elements into a buffer [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.sample_fill)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#244-246)

#### fn [sample](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.sample)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html), amount: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

Uniformly sample `amount` distinct elements into a [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.sample)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#267-269)

#### fn [choose\_multiple\_fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_multiple_fill)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html), buf: &mut \[Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\]) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated since 0.10.0:

Renamed to `sample_fill`

Deprecated: use [`Self::sample_fill`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.sample_fill "method rand::seq::iterator::IteratorRandom::sample_fill") instead

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#277-279)

#### fn [choose\_multiple](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_multiple)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html), amount: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated since 0.10.0:

Renamed to `sample`

Available on **crate feature `alloc`** only.

Deprecated: use [`Self::sample`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.sample "method rand::seq::iterator::IteratorRandom::sample") instead

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#271)

### impl<I> [IteratorRandom](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html "trait rand::seq::iterator::IteratorRandom") for I

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"),

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#66-68)

#### fn [choose](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where R: [Rng](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html "trait rand::rng::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Uniformly sample one element [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#145-147)

#### fn [choose\_stable](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_stable)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where R: [Rng](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html "trait rand::rng::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Uniformly sample one element (stable) [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_stable)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#203-205)

#### fn [choose\_multiple\_fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_multiple_fill)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html), buf: &mut \[Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\]) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where R: [Rng](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html "trait rand::rng::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Uniformly sample `amount` distinct elements into a buffer [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_multiple_fill)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/seq/iterator.rs.html#244-246)

#### fn [choose\_multiple](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_multiple)<R>(self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html), amount: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where R: [Rng](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html "trait rand::rng::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `alloc`** only.

Uniformly sample `amount` distinct elements into a [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/seq/iterator/trait.IteratorRandom.html#method.choose_multiple)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4586)

### impl<T> [Itertools](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html "trait itertools::Itertools") for T

where T: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#453-456)

#### fn [interleave](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.interleave)<J>( self, other: J, ) -> [Interleave](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.Interleave.html "struct itertools::adaptors::Interleave")<Self, <J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where J: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Alternate elements from two iterators until both have run out. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.interleave)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#472-475)

#### fn [interleave\_shortest](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.interleave_shortest)<J>( self, other: J, ) -> [InterleaveShortest](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.InterleaveShortest.html "struct itertools::adaptors::InterleaveShortest")<Self, <J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where J: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Alternate elements from two iterators until at least one of them has run out. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.interleave_shortest)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#492-495)

#### fn [intersperse](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.intersperse)( self, element: Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), ) -> [IntersperseWith](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/intersperse/struct.IntersperseWith.html "struct itertools::intersperse::IntersperseWith")<Self, IntersperseElementSimple<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

An iterator adaptor to insert a particular value between each element of the adapted iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.intersperse)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#514-517)

#### fn [intersperse\_with](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.intersperse_with)<F>(self, element: F) -> [IntersperseWith](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/intersperse/struct.IntersperseWith.html "struct itertools::intersperse::IntersperseWith")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")() -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"),

An iterator adaptor to insert a particular value created by a function between each element of the adapted iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.intersperse_with)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#564-567)

#### fn [get](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.get)<R>(self, index: R) -> <R as [IteratorIndex](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/iter_index/trait.IteratorIndex.html "trait itertools::iter_index::IteratorIndex")<Self>>::[Output](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/iter_index/trait.IteratorIndex.html#associatedtype.Output "type itertools::iter_index::IteratorIndex::Output")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: [IteratorIndex](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/iter_index/trait.IteratorIndex.html "trait itertools::iter_index::IteratorIndex")<Self>,

Returns an iterator over a subsection of the iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.get)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#599-602)

#### fn [zip\_longest](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.zip_longest)<J>( self, other: J, ) -> [ZipLongest](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/zip_longest/struct.ZipLongest.html "struct itertools::zip_longest::ZipLongest")<Self, <J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where J: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Create an iterator which iterates over both this and the specified iterator simultaneously, yielding pairs of two optional elements. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.zip_longest)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#613-616)

#### fn [zip\_eq](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.zip_eq)<J>(self, other: J) -> [ZipEq](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/zip_eq_impl/struct.ZipEq.html "struct itertools::zip_eq_impl::ZipEq")<Self, <J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where J: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Create an iterator which iterates over both this and the specified iterator simultaneously, yielding pairs of elements. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.zip_eq)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#644-647)

#### fn [batching](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.batching)<B, F>(self, f: F) -> [Batching](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.Batching.html "struct itertools::adaptors::Batching")<Self, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut Self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

A “meta iterator adaptor”. Its closure receives a reference to the iterator and may pick off as many elements as it likes, to produce the next iterator element. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.batching)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#687-691)

#### fn [chunk\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.chunk_by)<K, F>(self, key: F) -> [ChunkBy](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/groupbylazy/struct.ChunkBy.html "struct itertools::groupbylazy::ChunkBy")<K, Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K, K: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Available on **crate feature `use_alloc`** only.

Return an _iterable_ that can group iterator elements. Consecutive elements that map to the same key (“runs”), are assigned to the same group. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.chunk_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#699-703)

#### fn [group\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.group_by)<K, F>(self, key: F) -> [ChunkBy](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/groupbylazy/struct.ChunkBy.html "struct itertools::groupbylazy::ChunkBy")<K, Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K, K: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

👎Deprecated since 0.13.0:

Use .chunk\_by() instead

Available on **crate feature `use_alloc`** only.

See [`.chunk_by()`](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.chunk_by "method itertools::Itertools::chunk_by").

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#736-738)

#### fn [chunks](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.chunks)(self, size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [IntoChunks](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/groupbylazy/struct.IntoChunks.html "struct itertools::groupbylazy::IntoChunks")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `use_alloc`** only.

Return an _iterable_ that can chunk the iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.chunks)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#777-781)

#### fn [tuple\_windows](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tuple_windows)<T>(self) -> [TupleWindows](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/tuple_impl/struct.TupleWindows.html "struct itertools::tuple_impl::TupleWindows")<Self, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = <T as TupleCollect>::Item>, T: [HomogeneousTuple](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/tuple_impl/trait.HomogeneousTuple.html "trait itertools::tuple_impl::HomogeneousTuple"), <T as TupleCollect>::Item: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Return an iterator over all contiguous windows producing tuples of a specific size (up to 12). [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tuple_windows)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#813-817)

#### fn [circular\_tuple\_windows](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.circular_tuple_windows)<T>(self) -> [CircularTupleWindows](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/tuple_impl/struct.CircularTupleWindows.html "struct itertools::tuple_impl::CircularTupleWindows")<Self, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = <T as TupleCollect>::Item> + [ExactSizeIterator](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html "trait core::iter::traits::exact_size::ExactSizeIterator"), T: TupleCollect + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), <T as TupleCollect>::Item: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Return an iterator over all windows, wrapping back to the first elements when the window would otherwise exceed the length of the iterator, producing tuples of a specific size (up to 12). [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.circular_tuple_windows)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#852-855)

#### fn [tuples](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tuples)<T>(self) -> [Tuples](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/tuple_impl/struct.Tuples.html "struct itertools::tuple_impl::Tuples")<Self, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = <T as TupleCollect>::Item>, T: [HomogeneousTuple](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/tuple_impl/trait.HomogeneousTuple.html "trait itertools::tuple_impl::HomogeneousTuple"),

Return an iterator that groups the items in tuples of a specific size (up to 12). [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tuples)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#878-881)

#### fn [tee](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tee)(self) -> ([Tee](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/tee/struct.Tee.html "struct itertools::tee::Tee")<Self>, [Tee](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/tee/struct.Tee.html "struct itertools::tee::Tee")<Self>)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **crate feature `use_alloc`** only.

Split into an iterator pair that both yield all elements from the original iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tee)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#893-896)

#### fn [map\_into](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.map_into)<R>(self) -> MapSpecialCase<Self, MapSpecialCaseFnInto<R>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<R>,

Convert each item of the iterator using the [`Into`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") trait. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.map_into)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#912-915)

#### fn [map\_ok](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.map_ok)<F, T, U, E>(self, f: F) -> MapSpecialCase<Self, MapSpecialCaseFnOk<F>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T) -> U,

Return an iterator adaptor that applies the provided closure to every `Result::Ok` value. `Result::Err` values are unchanged. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.map_ok)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#931-934)

#### fn [filter\_ok](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.filter_ok)<F, T, E>(self, f: F) -> [FilterOk](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.FilterOk.html "struct itertools::adaptors::FilterOk")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Return an iterator adaptor that filters every `Result::Ok` value with the provided closure. `Result::Err` values are unchanged. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.filter_ok)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#950-953)

#### fn [filter\_map\_ok](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.filter_map_ok)<F, T, U, E>(self, f: F) -> [FilterMapOk](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.FilterMapOk.html "struct itertools::adaptors::FilterMapOk")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<U>,

Return an iterator adaptor that filters and transforms every `Result::Ok` value with the provided closure. `Result::Err` values are unchanged. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.filter_map_ok)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#975-978)

#### fn [flatten\_ok](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.flatten_ok)<T, E>(self) -> [FlattenOk](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/flatten_ok/struct.FlattenOk.html "struct itertools::flatten_ok::FlattenOk")<Self, T, E> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Return an iterator adaptor that flattens every `Result::Ok` value into a series of `Result::Ok` values. `Result::Err` values are unchanged. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.flatten_ok)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1013-1016)

#### fn [process\_results](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.process_results)<F, T, E, R>(self, processor: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<R, E>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([ProcessResults](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/process_results_impl/struct.ProcessResults.html "struct itertools::process_results_impl::ProcessResults")<'\_, Self, E>) -> R,

“Lift” a function of the values of the current iterator so as to process an iterator of `Result` values instead. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.process_results)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1035-1039)

#### fn [merge](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.merge)<J>( self, other: J, ) -> [MergeBy](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/merge_join/struct.MergeBy.html "struct itertools::merge_join::MergeBy")<Self, <J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"), MergeLte> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"), J: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Return an iterator adaptor that merges the two base iterators in ascending order. If both base iterators are sorted (ascending), the result is sorted. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.merge)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1059-1063)

#### fn [merge\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.merge_by)<J, F>( self, other: J, is\_first: F, ) -> [MergeBy](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/merge_join/struct.MergeBy.html "struct itertools::merge_join::MergeBy")<Self, <J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"), F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), J: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Return an iterator adaptor that merges the two base iterators in order. This is much like [`.merge()`](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.merge "method itertools::Itertools::merge") but allows for a custom ordering. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.merge_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1129-1133)

#### fn [merge\_join\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.merge_join_by)<J, F, T>( self, other: J, cmp\_fn: F, ) -> [MergeBy](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/merge_join/struct.MergeBy.html "struct itertools::merge_join::MergeBy")<Self, <J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"), MergeFuncLR<F, <F as FuncLR<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), <<J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter") as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>>::T>> [ⓘ](#)

where J: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &<J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")) -> T, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Create an iterator that merges items from both this and the specified iterator in ascending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.merge_join_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1155-1159)

#### fn [kmerge](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.kmerge)(self) -> [KMergeBy](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/kmerge_impl/struct.KMergeBy.html "struct itertools::kmerge_impl::KMergeBy")<<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"), KMergeByLt> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Available on **crate feature `use_alloc`** only.

Return an iterator adaptor that flattens an iterator of iterators by merging them in ascending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.kmerge)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1185-1189)

#### fn [kmerge\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.kmerge_by)<F>( self, first: F, ) -> [KMergeBy](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/kmerge_impl/struct.KMergeBy.html "struct itertools::kmerge_impl::KMergeBy")<<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"), F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"), &<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Available on **crate feature `use_alloc`** only.

Return an iterator adaptor that flattens an iterator of iterators by merging them according to the given closure. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.kmerge_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1205-1210)

#### fn [cartesian\_product](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.cartesian_product)<J>( self, other: J, ) -> [Product](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.Product.html "struct itertools::adaptors::Product")<Self, <J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), J: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <J as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Return an iterator adaptor that iterates over the cartesian product of the element sets of two iterators `self` and `J`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.cartesian_product)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1252-1257)

#### fn [multi\_cartesian\_product](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.multi_cartesian_product)( self, ) -> [MultiProduct](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/multi_product/struct.MultiProduct.html "struct itertools::adaptors::multi_product::MultiProduct")<<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), <Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **crate feature `use_alloc`** only.

Return an iterator adaptor that iterates over the cartesian product of all subiterators returned by meta-iterator `self`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.multi_cartesian_product)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1290-1293)

#### fn [coalesce](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.coalesce)<F>(self, f: F) -> CoalesceBy<Self, F, NoCount>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), (Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"))>,

Return an iterator adaptor that uses the passed-in closure to optionally merge together consecutive elements. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.coalesce)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1312-1315)

#### fn [dedup](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dedup)(self) -> CoalesceBy<Self, DedupPred2CoalescePred<DedupEq>, NoCount>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Remove duplicates from sections of consecutive identical elements. If the iterator is sorted, all elements will be unique. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dedup)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1335-1338)

#### fn [dedup\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dedup_by)<Cmp>( self, cmp: Cmp, ) -> CoalesceBy<Self, DedupPred2CoalescePred<Cmp>, NoCount>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Cmp: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Remove duplicates from sections of consecutive identical elements, determining equality using a comparison function. If the iterator is sorted, all elements will be unique. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dedup_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1358-1360)

#### fn [dedup\_with\_count](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dedup_with_count)( self, ) -> CoalesceBy<Self, DedupPredWithCount2CoalescePred<DedupEq>, WithCount>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove duplicates from sections of consecutive identical elements, while keeping a count of how many repeated elements were present. If the iterator is sorted, all elements will be unique. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dedup_with_count)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1381-1384)

#### fn [dedup\_by\_with\_count](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dedup_by_with_count)<Cmp>( self, cmp: Cmp, ) -> CoalesceBy<Self, DedupPredWithCount2CoalescePred<Cmp>, WithCount>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Cmp: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Remove duplicates from sections of consecutive identical elements, while keeping a count of how many repeated elements were present. This will determine equality using a comparison function. If the iterator is sorted, all elements will be unique. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dedup_by_with_count)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1404-1407)

#### fn [duplicates](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.duplicates)(self) -> DuplicatesBy<Self, Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), ById>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

Available on **crate feature `use_std`** only.

Return an iterator adaptor that produces elements that appear more than once during the iteration. Duplicates are detected using hash and equality. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.duplicates)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1430-1434)

#### fn [duplicates\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.duplicates_by)<V, F>(self, f: F) -> DuplicatesBy<Self, V, ByFn<F>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), V: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> V,

Available on **crate feature `use_std`** only.

Return an iterator adaptor that produces elements that appear more than once during the iteration. Duplicates are detected using hash and equality. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.duplicates_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1458-1461)

#### fn [unique](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.unique)(self) -> [Unique](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/unique_impl/struct.Unique.html "struct itertools::unique_impl::Unique")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

Available on **crate feature `use_std`** only.

Return an iterator adaptor that filters out elements that have already been produced once during the iteration. Duplicates are detected using hash and equality. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.unique)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1485-1489)

#### fn [unique\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.unique_by)<V, F>(self, f: F) -> [UniqueBy](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/unique_impl/struct.UniqueBy.html "struct itertools::unique_impl::UniqueBy")<Self, V, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), V: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> V,

Available on **crate feature `use_std`** only.

Return an iterator adaptor that filters out elements that have already been produced once during the iteration. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.unique_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1506-1509)

#### fn [peeking\_take\_while](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.peeking_take_while)<F>(&mut self, accept: F) -> [PeekingTakeWhile](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/peeking_take_while/struct.PeekingTakeWhile.html "struct itertools::peeking_take_while::PeekingTakeWhile")<'\_, Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [PeekingNext](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/peeking_take_while/trait.PeekingNext.html "trait itertools::peeking_take_while::PeekingNext"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Return an iterator adaptor that borrows from this iterator and takes items while the closure `accept` returns `true`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.peeking_take_while)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1532-1535)

#### fn [take\_while\_ref](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.take_while_ref)<F>(&mut self, accept: F) -> [TakeWhileRef](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.TakeWhileRef.html "struct itertools::adaptors::TakeWhileRef")<'\_, Self, F> [ⓘ](#)

where Self: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Return an iterator adaptor that borrows from a `Clone`\-able iterator to only pick off elements while the predicate `accept` returns `true`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.take_while_ref)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1601-1604)

#### fn [take\_while\_inclusive](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.take_while_inclusive)<F>(self, accept: F) -> [TakeWhileInclusive](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/take_while_inclusive/struct.TakeWhileInclusive.html "struct itertools::take_while_inclusive::TakeWhileInclusive")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns an iterator adaptor that consumes elements while the given predicate is `true`, _including_ the element for which the predicate first returned `false`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.take_while_inclusive)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1623-1625)

#### fn [while\_some](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.while_some)<A>(self) -> [WhileSome](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.WhileSome.html "struct itertools::adaptors::WhileSome")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<A>>,

Return an iterator adaptor that filters `Option<A>` iterator elements and produces `A`. Stops on the first `None` encountered. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.while_some)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1668-1672)

#### fn [tuple\_combinations](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tuple_combinations)<T>(self) -> [TupleCombinations](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.TupleCombinations.html "struct itertools::adaptors::TupleCombinations")<Self, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), T: HasCombination<Self>,

Return an iterator adaptor that iterates over the combinations of the elements from an iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tuple_combinations)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1716-1719)

#### fn [array\_combinations](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.array_combinations)<const K: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( self, ) -> CombinationsGeneric<Self, \[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [K](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **crate feature `use_alloc`** only.

Return an iterator adaptor that iterates over the combinations of the elements from an iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.array_combinations)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1759-1762)

#### fn [combinations](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.combinations)(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> CombinationsGeneric<Self, [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **crate feature `use_alloc`** only.

Return an iterator adaptor that iterates over the `k`\-length combinations of the elements from an iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.combinations)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1787-1790)

#### fn [combinations\_with\_replacement](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.combinations_with_replacement)( self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [CombinationsWithReplacement](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/combinations_with_replacement/struct.CombinationsWithReplacement.html "struct itertools::combinations_with_replacement::CombinationsWithReplacement")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **crate feature `use_alloc`** only.

Return an iterator that iterates over the `k`\-length combinations of the elements from an iterator, with replacement. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.combinations_with_replacement)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1836-1839)

#### fn [permutations](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.permutations)(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Permutations](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/permutations/struct.Permutations.html "struct itertools::permutations::Permutations")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **crate feature `use_alloc`** only.

Return an iterator adaptor that iterates over all k-permutations of the elements from an iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.permutations)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1873-1876)

#### fn [powerset](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.powerset)(self) -> [Powerset](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/powerset/struct.Powerset.html "struct itertools::powerset::Powerset")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **crate feature `use_alloc`** only.

Return an iterator that iterates through the powerset of the elements from an iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.powerset)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1898-1901)

#### fn [pad\_using](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.pad_using)<F>(self, min: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), f: F) -> [PadUsing](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/pad_tail/struct.PadUsing.html "struct itertools::pad_tail::PadUsing")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"),

Return an iterator adaptor that pads the sequence to a minimum length of `min` by filling missing elements using a closure `f`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.pad_using)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1925-1927)

#### fn [with\_position](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.with_position)(self) -> [WithPosition](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/with_position/struct.WithPosition.html "struct itertools::with_position::WithPosition")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Return an iterator adaptor that combines each element with a `Position` to ease special-case handling of the first or last elements. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.with_position)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1945-1948)

#### fn [positions](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.positions)<P>(self, predicate: P) -> [Positions](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.Positions.html "struct itertools::adaptors::Positions")<Self, P> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Return an iterator adaptor that yields the indices of all elements satisfying a predicate, counted from the start of the iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.positions)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1963-1966)

#### fn [update](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.update)<F>(self, updater: F) -> [Update](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/adaptors/struct.Update.html "struct itertools::adaptors::Update")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")),

Return an iterator adaptor that applies a mutating function to each element before yielding it. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.update)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#1985-1987)

#### fn [next\_array](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.next_array)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<\[Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Advances the iterator and returns the next items grouped in an array of a specific size. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.next_array)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2009-2011)

#### fn [collect\_array](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.collect_array)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<\[Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Collects all items from the iterator into an array of a specific size. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.collect_array)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2029-2032)

#### fn [next\_tuple](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.next_tuple)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = <T as TupleCollect>::Item>, T: [HomogeneousTuple](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/tuple_impl/trait.HomogeneousTuple.html "trait itertools::tuple_impl::HomogeneousTuple"),

Advances the iterator and returns the next items grouped in a tuple of a specific size (up to 12). [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.next_tuple)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2055-2058)

#### fn [collect\_tuple](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.collect_tuple)<T>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = <T as TupleCollect>::Item>, T: [HomogeneousTuple](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/tuple_impl/trait.HomogeneousTuple.html "trait itertools::tuple_impl::HomogeneousTuple"),

Collects all items from the iterator into a tuple of a specific size (up to 12). [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.collect_tuple)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2079-2081)

#### fn [find\_position](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.find_position)<P>(&mut self, pred: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"))>

where P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Find the position and value of the first element satisfying a predicate. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.find_position)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2106-2109)

#### fn [find\_or\_last](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.find_or_last)<P>(self, predicate: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Find the value of the first element satisfying a predicate or return the last element, if any. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.find_or_last)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2143-2146)

#### fn [find\_or\_first](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.find_or_first)<P>(self, predicate: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Find the value of the first element satisfying a predicate or return the first element, if any. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.find_or_first)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2180-2184)

#### fn [contains](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.contains)<Q>(&mut self, query: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q>, Q: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns `true` if the given item is present in this iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.contains)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2205-2208)

#### fn [all\_equal](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.all_equal)(&mut self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Check whether all elements compare equal. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.all_equal)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2234-2237)

#### fn [all\_equal\_value](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.all_equal_value)( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"))>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

If there are elements and they are all equal, return a single copy of that element. If there are no elements, return an Error containing None. If there are elements and they are not all equal, return a tuple containing the first two non-equal elements found. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.all_equal_value)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2264-2267)

#### fn [all\_unique](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.all_unique)(&mut self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

Available on **crate feature `use_std`** only.

Check whether all elements are unique (non equal). [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.all_unique)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2288-2290)

#### fn [dropping](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dropping)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Consume the first `n` elements from the iterator eagerly, and return the same iterator again. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dropping)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2313-2315)

#### fn [dropping\_back](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dropping_back)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator"),

Consume the last `n` elements from the iterator eagerly, and return the same iterator again. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.dropping_back)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2336-2340)

#### fn [concat](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.concat)(self) -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\> + [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Combine all an iterator’s elements into one element by using [`Extend`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend"). [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.concat)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2348-2350)

#### fn [collect\_vec](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.collect_vec)(self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `use_alloc`** only.

`.collect_vec()` is simply a type specialization of [`Iterator::collect`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect "method core::iter::traits::iterator::Iterator::collect"), for convenience.

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2378-2381)

#### fn [try\_collect](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.try_collect)<T, U, E>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, E>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, E>: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>,

`.try_collect()` is more convenient way of writing `.collect::<Result<_, _>>()` [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.try_collect)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2402-2405)

#### fn [set\_from](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.set_from)<'a, A, J>(&mut self, from: J) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where A: 'a, Self: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [&'a mut A](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, J: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = A>,

Assign to each reference in `self` from the `from` iterator, stopping at the shortest of the two iterators. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.set_from)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2424-2426)

#### fn [join](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.join)(&mut self, sep: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

where Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Available on **crate feature `use_alloc`** only.

Combine all iterator elements into one `String`, separated by `sep`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.join)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2459-2461)

#### fn [format](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.format)(self, sep: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Format](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/format/struct.Format.html "struct itertools::format::Format")<'\_, Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Format all iterator elements, separated by `sep`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.format)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2498-2501)

#### fn [format\_with](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.format_with)<F>(self, sep: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), format: F) -> [FormatWith](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/format/struct.FormatWith.html "struct itertools::format::FormatWith")<'\_, Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &mut dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&dyn [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>,

Format all iterator elements, separated by `sep`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.format_with)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2551-2554)

#### fn [fold\_ok](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.fold_ok)<A, E, B, F>(&mut self, start: B, f: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<B, E>

where Self: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<A, E>>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, A) -> B,

Fold `Result` values from an iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.fold_ok)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2584-2587)

#### fn [fold\_options](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.fold_options)<A, B, F>(&mut self, start: B, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>

where Self: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<A>>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, A) -> B,

Fold `Option` values from an iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.fold_options)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2614-2617)

#### fn [fold1](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.fold1)<F>(self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated since 0.10.2:

Use [`Iterator::reduce`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce) instead

Accumulator of the elements in the iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.fold1)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2711-2714)

#### fn [tree\_reduce](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tree_reduce)<F>(self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Accumulate the elements in the iterator in a tree-like manner. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tree_reduce)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2776-2779)

#### fn [tree\_fold1](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tree_fold1)<F>(self, f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated since 0.13.0:

Use .tree\_reduce() instead

See [`.tree_reduce()`](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tree_reduce "method itertools::Itertools::tree_reduce").

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2823-2826)

#### fn [fold\_while](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.fold_while)<B, F>(&mut self, init: B, f: F) -> [FoldWhile](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/enum.FoldWhile.html "enum itertools::FoldWhile")<B>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [FoldWhile](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/enum.FoldWhile.html "enum itertools::FoldWhile")<B>,

An iterator method that applies a function, producing a single, final value. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.fold_while)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2866-2869)

#### fn [sum1](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sum1)<S>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Iterate over the entire iterator and add all the elements. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sum1)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2894-2897)

#### fn [product1](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.product1)<P>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<P>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Iterate over the entire iterator and multiply all the elements. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.product1)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2922-2925)

#### fn [sorted\_unstable](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_unstable)(self) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Sort all iterator elements into a new iterator in ascending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_unstable)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2960-2963)

#### fn [sorted\_unstable\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_unstable_by)<F>(self, cmp: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Available on **crate feature `use_alloc`** only.

Sort all iterator elements into a new iterator in ascending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_unstable_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#2996-3000)

#### fn [sorted\_unstable\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_unstable_by_key)<K, F>(self, f: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Available on **crate feature `use_alloc`** only.

Sort all iterator elements into a new iterator in ascending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_unstable_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3027-3030)

#### fn [sorted](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted)(self) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Sort all iterator elements into a new iterator in ascending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3065-3068)

#### fn [sorted\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_by)<F>(self, cmp: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Available on **crate feature `use_alloc`** only.

Sort all iterator elements into a new iterator in ascending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3101-3105)

#### fn [sorted\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_by_key)<K, F>(self, f: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Available on **crate feature `use_alloc`** only.

Sort all iterator elements into a new iterator in ascending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3139-3143)

#### fn [sorted\_by\_cached\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_by_cached_key)<K, F>(self, f: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Available on **crate feature `use_alloc`** only.

Sort all iterator elements into a new iterator in ascending order. The key function is called exactly once per key. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.sorted_by_cached_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3178-3181)

#### fn [k\_smallest](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest)(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Sort the k smallest elements into a new iterator, in ascending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3236-3239)

#### fn [k\_smallest\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_by)<F>(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), cmp: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Available on **crate feature `use_alloc`** only.

Sort the k smallest elements into a new iterator using the provided comparison. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3268-3272)

#### fn [k\_smallest\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_by_key)<F, K>(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), key: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Return the elements producing the k smallest outputs of the provided function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3307-3310)

#### fn [k\_smallest\_relaxed](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_relaxed)(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Sort the k smallest elements into a new iterator, in ascending order, relaxing the amount of memory required. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_relaxed)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3337-3340)

#### fn [k\_smallest\_relaxed\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_relaxed_by)<F>(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), cmp: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Available on **crate feature `use_alloc`** only.

Sort the k smallest elements into a new iterator using the provided comparison, relaxing the amount of memory required. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_relaxed_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3367-3371)

#### fn [k\_smallest\_relaxed\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_relaxed_by_key)<F, K>( self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), key: F, ) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Return the elements producing the k smallest outputs of the provided function, relaxing the amount of memory required. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_smallest_relaxed_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3399-3402)

#### fn [k\_largest](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest)(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Sort the k largest elements into a new iterator, in descending order. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3428-3431)

#### fn [k\_largest\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_by)<F>(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), cmp: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Available on **crate feature `use_alloc`** only.

Sort the k largest elements into a new iterator using the provided comparison. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3457-3461)

#### fn [k\_largest\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_by_key)<F, K>(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), key: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Return the elements producing the k largest outputs of the provided function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3487-3490)

#### fn [k\_largest\_relaxed](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_relaxed)(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Sort the k largest elements into a new iterator, in descending order, relaxing the amount of memory required. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_relaxed)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3516-3519)

#### fn [k\_largest\_relaxed\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_relaxed_by)<F>(self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), cmp: F) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Available on **crate feature `use_alloc`** only.

Sort the k largest elements into a new iterator using the provided comparison, relaxing the amount of memory required. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_relaxed_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3545-3549)

#### fn [k\_largest\_relaxed\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_relaxed_by_key)<F, K>( self, k: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), key: F, ) -> [IntoIter](../../prelude/vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K, K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Return the elements producing the k largest outputs of the provided function, relaxing the amount of memory required. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.k_largest_relaxed_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3579-3581)

#### fn [tail](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tail)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [IntoIter](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/into_iter/struct.IntoIter.html "struct alloc::collections::vec_deque::into_iter::IntoIter")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `use_alloc`** only.

Consumes the iterator and return an iterator of the last `n` elements. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.tail)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3636-3641)

#### fn [partition\_map](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.partition_map)<A, B, F, L, R>(self, predicate: F) -> [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<L, R>, A: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<L>, B: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<R>,

Collect all iterator elements into one of two partitions. Unlike [`Iterator::partition`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.partition "method core::iter::traits::iterator::Iterator::partition"), each partition may have a distinct type. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.partition_map)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3669-3673)

#### fn [partition\_result](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.partition_result)<A, B, T, E>(self) -> [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, A: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<T>, B: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<E>,

Partition a sequence of `Result`s into one list of all the `Ok` elements and another list of all the `Err` elements. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.partition_result)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3698-3701)

#### fn [into\_group\_map](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.into_group_map)<K, V>(self) -> [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<V>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, K: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

Available on **crate feature `use_std`** only.

Return a `HashMap` of keys mapped to `Vec`s of values. Keys and values are taken from `(Key, Value)` tuple pairs yielded by the input iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.into_group_map)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3734-3738)

#### fn [into\_group\_map\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.into_group_map_by)<K, V, F>(self, f: F) -> [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<V>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = V>, K: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K,

Available on **crate feature `use_std`** only.

Return a `HashMap` of keys mapped to `Vec`s of values. The key is specified in the closure. The values are taken from the input iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.into_group_map_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3753-3756)

#### fn [into\_grouping\_map](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.into_grouping_map)<K, V>(self) -> [GroupingMap](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/grouping_map/struct.GroupingMap.html "struct itertools::grouping_map::GroupingMap")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, K: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

Available on **crate feature `use_std`** only.

Constructs a `GroupingMap` to be used later with one of the efficient group-and-fold operations it allows to perform. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.into_grouping_map)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3770-3774)

#### fn [into\_grouping\_map\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.into_grouping_map_by)<K, V, F>( self, key\_mapper: F, ) -> [GroupingMap](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/grouping_map/struct.GroupingMap.html "struct itertools::grouping_map::GroupingMap")<MapSpecialCase<Self, GroupingMapFn<F>>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = V>, K: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> K,

Available on **crate feature `use_std`** only.

Constructs a `GroupingMap` to be used later with one of the efficient group-and-fold operations it allows to perform. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.into_grouping_map_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3802-3805)

#### fn [min\_set](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.min_set)(self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Return all minimum elements of an iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.min_set)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3835-3838)

#### fn [min\_set\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.min_set_by)<F>(self, compare: F) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Available on **crate feature `use_alloc`** only.

Return all minimum elements of an iterator, as determined by the specified function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.min_set_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3867-3871)

#### fn [min\_set\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.min_set_by_key)<K, F>(self, key: F) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Available on **crate feature `use_alloc`** only.

Return all minimum elements of an iterator, as determined by the specified function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.min_set_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3899-3902)

#### fn [max\_set](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.max_set)(self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Available on **crate feature `use_alloc`** only.

Return all maximum elements of an iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.max_set)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3932-3935)

#### fn [max\_set\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.max_set_by)<F>(self, compare: F) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Available on **crate feature `use_alloc`** only.

Return all maximum elements of an iterator, as determined by the specified function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.max_set_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#3964-3968)

#### fn [max\_set\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.max_set_by_key)<K, F>(self, key: F) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Available on **crate feature `use_alloc`** only.

Return all maximum elements of an iterator, as determined by the specified function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.max_set_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4008-4011)

#### fn [minmax](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.minmax)(self) -> [MinMaxResult](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/minmax/enum.MinMaxResult.html "enum itertools::minmax::MinMaxResult")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Return the minimum and maximum elements in the iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.minmax)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4027-4031)

#### fn [minmax\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.minmax_by_key)<K, F>(self, key: F) -> [MinMaxResult](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/minmax/enum.MinMaxResult.html "enum itertools::minmax::MinMaxResult")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Return the minimum and maximum element of an iterator, as determined by the specified function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.minmax_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4044-4047)

#### fn [minmax\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.minmax_by)<F>(self, compare: F) -> [MinMaxResult](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/minmax/enum.MinMaxResult.html "enum itertools::minmax::MinMaxResult")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Return the minimum and maximum element of an iterator, as determined by the specified comparison function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.minmax_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4071-4074)

#### fn [position\_max](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_max)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Return the position of the maximum element in the iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_max)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4101-4105)

#### fn [position\_max\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_max_by_key)<K, F>(self, key: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Return the position of the maximum element in the iterator, as determined by the specified function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_max_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4132-4135)

#### fn [position\_max\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_max_by)<F>(self, compare: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Return the position of the maximum element in the iterator, as determined by the specified comparison function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_max_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4161-4164)

#### fn [position\_min](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_min)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Return the position of the minimum element in the iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_min)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4191-4195)

#### fn [position\_min\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_min_by_key)<K, F>(self, key: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Return the position of the minimum element in the iterator, as determined by the specified function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_min_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4222-4225)

#### fn [position\_min\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_min_by)<F>(self, compare: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Return the position of the minimum element in the iterator, as determined by the specified comparison function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_min_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4274-4277)

#### fn [position\_minmax](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_minmax)(self) -> [MinMaxResult](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/minmax/enum.MinMaxResult.html "enum itertools::minmax::MinMaxResult")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"),

Return the positions of the minimum and maximum elements in the iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_minmax)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4321-4325)

#### fn [position\_minmax\_by\_key](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_minmax_by_key)<K, F>(self, key: F) -> [MinMaxResult](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/minmax/enum.MinMaxResult.html "enum itertools::minmax::MinMaxResult")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Return the postions of the minimum and maximum elements of an iterator, as determined by the specified function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_minmax_by_key)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4366-4369)

#### fn [position\_minmax\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_minmax_by)<F>(self, compare: F) -> [MinMaxResult](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/minmax/enum.MinMaxResult.html "enum itertools::minmax::MinMaxResult")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Return the postions of the minimum and maximum elements of an iterator, as determined by the specified comparison function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.position_minmax_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4396-4398)

#### fn [exactly\_one](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.exactly_one)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), [ExactlyOneError](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/exactly_one_err/struct.ExactlyOneError.html "struct itertools::exactly_one_err::ExactlyOneError")<Self>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

If the iterator yields exactly one element, that element will be returned, otherwise an error will be returned containing an iterator that has the same output as the input iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.exactly_one)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4429-4431)

#### fn [at\_most\_one](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.at_most_one)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, [ExactlyOneError](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/exactly_one_err/struct.ExactlyOneError.html "struct itertools::exactly_one_err::ExactlyOneError")<Self>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

If the iterator yields no elements, `Ok(None)` will be returned. If the iterator yields exactly one element, that element will be returned, otherwise an error will be returned containing an iterator that has the same output as the input iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.at_most_one)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4460-4462)

#### fn [multipeek](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.multipeek)(self) -> [MultiPeek](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/multipeek_impl/struct.MultiPeek.html "struct itertools::multipeek_impl::MultiPeek")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `use_alloc`** only.

An iterator adaptor that allows the user to peek at multiple `.next()` values without advancing the base iterator. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.multipeek)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4481-4484)

#### fn [counts](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.counts)(self) -> [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

Available on **crate feature `use_std`** only.

Collect the items in this iterator and return a `HashMap` which contains each item that appears in the iterator and the number of times it appears. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.counts)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4525-4529)

#### fn [counts\_by](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.counts_by)<K, F>(self, f: F) -> [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> K,

Available on **crate feature `use_std`** only.

Collect the items in this iterator and return a `HashMap` which contains each item that appears in the iterator and the number of times it appears, determining identity using a keying function. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.counts_by)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4554-4556)

#### fn [multiunzip](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.multiunzip)<FromI>(self) -> FromI

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [MultiUnzip](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/unziptuple/trait.MultiUnzip.html "trait itertools::unziptuple::MultiUnzip")<FromI>,

Converts an iterator of tuples into a tuple of containers. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.multiunzip)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/lib.rs.html#4577)

#### fn [try\_len](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.try_len)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)>

Returns the length of the iterator if one exists. Otherwise return `self.size_hint()`. [Read more](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/trait.Itertools.html#method.try_len)

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/unziptuple.rs.html#70)

### impl<IT, A, FromA, B, FromB> [MultiUnzip](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/unziptuple/trait.MultiUnzip.html "trait itertools::unziptuple::MultiUnzip")<[(FromA, FromB)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\> for IT

where IT: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, FromA: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<A>, FromB: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<B>,

[Source](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/src/itertools/unziptuple.rs.html#70)

#### fn [multiunzip](https://docs.rs/itertools/0.14.0/x86_64-unknown-linux-gnu/itertools/unziptuple/trait.MultiUnzip.html#tymethod.multiunzip)(self) -> [(FromA, FromB)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

Unzip this iterator into multiple collections.

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

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","ArrayChunks<Self, N>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/array\_chunks/struct.ArrayChunks.html\\" title=\\"struct core::iter::adapters::array\_chunks::ArrayChunks\\">ArrayChunks</a>&lt;I, N&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, const N: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/array\_chunks/struct.ArrayChunks.html\\" title=\\"struct core::iter::adapters::array\_chunks::ArrayChunks\\">ArrayChunks</a>&lt;I, N&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = \[&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.array.html\\">N</a>\];</div>","Batching<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Batching.html\\" title=\\"struct itertools::adaptors::Batching\\">Batching</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B, F, I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Batching.html\\" title=\\"struct itertools::adaptors::Batching\\">Batching</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;mut I</a>) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;B&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = B;</div>","Chain<Self, <U as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html\\" title=\\"struct core::iter::adapters::chain::Chain\\">Chain</a>&lt;A, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A, B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/chain/struct.Chain.html\\" title=\\"struct core::iter::adapters::chain::Chain\\">Chain</a>&lt;A, B&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n B: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;A as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;A as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","CircularTupleWindows<Self, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/tuple\_impl/struct.CircularTupleWindows.html\\" title=\\"struct itertools::tuple\_impl::CircularTupleWindows\\">CircularTupleWindows</a>&lt;I, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/tuple\_impl/struct.CircularTupleWindows.html\\" title=\\"struct itertools::tuple\_impl::CircularTupleWindows\\">CircularTupleWindows</a>&lt;I, T&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;T as TupleCollect&gt;::Item&gt; + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,\\n T: TupleCollect + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,\\n &lt;T as TupleCollect&gt;::Item: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","Cloned<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html\\" title=\\"struct core::iter::adapters::cloned::Cloned\\">Cloned</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, I, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html\\" title=\\"struct core::iter::adapters::cloned::Cloned\\">Cloned</a>&lt;I&gt;<div class=\\"where\\">where\\n T: 'a + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","CombinationsWithReplacement<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/combinations\_with\_replacement/struct.CombinationsWithReplacement.html\\" title=\\"struct itertools::combinations\_with\_replacement::CombinationsWithReplacement\\">CombinationsWithReplacement</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/combinations\_with\_replacement/struct.CombinationsWithReplacement.html\\" title=\\"struct itertools::combinations\_with\_replacement::CombinationsWithReplacement\\">CombinationsWithReplacement</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"../../prelude/struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;;</div>","Copied<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html\\" title=\\"struct core::iter::adapters::copied::Copied\\">Copied</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, I, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html\\" title=\\"struct core::iter::adapters::copied::Copied\\">Copied</a>&lt;I&gt;<div class=\\"where\\">where\\n T: 'a + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\\" title=\\"trait core::marker::Copy\\">Copy</a>,\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","Cycle<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/cycle/struct.Cycle.html\\" title=\\"struct core::iter::adapters::cycle::Cycle\\">Cycle</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/cycle/struct.Cycle.html\\" title=\\"struct core::iter::adapters::cycle::Cycle\\">Cycle</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Enumerate<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/enumerate/struct.Enumerate.html\\" title=\\"struct core::iter::adapters::enumerate::Enumerate\\">Enumerate</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/enumerate/struct.Enumerate.html\\" title=\\"struct core::iter::adapters::enumerate::Enumerate\\">Enumerate</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>);</div>","Filter<Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/filter/struct.Filter.html\\" title=\\"struct core::iter::adapters::filter::Filter\\">Filter</a>&lt;I, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/filter/struct.Filter.html\\" title=\\"struct core::iter::adapters::filter::Filter\\">Filter</a>&lt;I, P&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","FilterMap<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/filter\_map/struct.FilterMap.html\\" title=\\"struct core::iter::adapters::filter\_map::FilterMap\\">FilterMap</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B, I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/filter\_map/struct.FilterMap.html\\" title=\\"struct core::iter::adapters::filter\_map::FilterMap\\">FilterMap</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;B&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = B;</div>","FilterMapOk<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.FilterMapOk.html\\" title=\\"struct itertools::adaptors::FilterMapOk\\">FilterMapOk</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F, T, U, E&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.FilterMapOk.html\\" title=\\"struct itertools::adaptors::FilterMapOk\\">FilterMapOk</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;&gt;,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(T) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;U&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;U, E&gt;;</div>","FilterOk<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.FilterOk.html\\" title=\\"struct itertools::adaptors::FilterOk\\">FilterOk</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F, T, E&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.FilterOk.html\\" title=\\"struct itertools::adaptors::FilterOk\\">FilterOk</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;&gt;,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;;</div>","FlatMap<Self, U, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.FlatMap.html\\" title=\\"struct core::iter::adapters::flatten::FlatMap\\">FlatMap</a>&lt;I, U, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, U, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.FlatMap.html\\" title=\\"struct core::iter::adapters::flatten::FlatMap\\">FlatMap</a>&lt;I, U, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n U: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\\" title=\\"trait core::iter::traits::collect::IntoIterator\\">IntoIterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; U,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;U as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\\" title=\\"trait core::iter::traits::collect::IntoIterator\\">IntoIterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::collect::IntoIterator::Item\\">Item</a>;</div>","Flatten<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.Flatten.html\\" title=\\"struct core::iter::adapters::flatten::Flatten\\">Flatten</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, U&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/flatten/struct.Flatten.html\\" title=\\"struct core::iter::adapters::flatten::Flatten\\">Flatten</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\\" title=\\"trait core::iter::traits::collect::IntoIterator\\">IntoIterator</a>&lt;IntoIter = U, Item = &lt;U as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,\\n U: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;U as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","FlattenOk<Self, T, E>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/flatten\_ok/struct.FlattenOk.html\\" title=\\"struct itertools::flatten\_ok::FlattenOk\\">FlattenOk</a>&lt;I, T, E&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, T, E&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/flatten\_ok/struct.FlattenOk.html\\" title=\\"struct itertools::flatten\_ok::FlattenOk\\">FlattenOk</a>&lt;I, T, E&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;&gt;,\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\\" title=\\"trait core::iter::traits::collect::IntoIterator\\">IntoIterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;&lt;T as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\\" title=\\"trait core::iter::traits::collect::IntoIterator\\">IntoIterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::collect::IntoIterator::Item\\">Item</a>, E&gt;;</div>","Fuse<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/fuse/struct.Fuse.html\\" title=\\"struct core::iter::adapters::fuse::Fuse\\">Fuse</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/fuse/struct.Fuse.html\\" title=\\"struct core::iter::adapters::fuse::Fuse\\">Fuse</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Inspect<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/inspect/struct.Inspect.html\\" title=\\"struct core::iter::adapters::inspect::Inspect\\">Inspect</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/inspect/struct.Inspect.html\\" title=\\"struct core::iter::adapters::inspect::Inspect\\">Inspect</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>),</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Interleave<Self, <J as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Interleave.html\\" title=\\"struct itertools::adaptors::Interleave\\">Interleave</a>&lt;I, J&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, J&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Interleave.html\\" title=\\"struct itertools::adaptors::Interleave\\">Interleave</a>&lt;I, J&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n J: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","InterleaveShortest<Self, <J as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.InterleaveShortest.html\\" title=\\"struct itertools::adaptors::InterleaveShortest\\">InterleaveShortest</a>&lt;I, J&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, J&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.InterleaveShortest.html\\" title=\\"struct itertools::adaptors::InterleaveShortest\\">InterleaveShortest</a>&lt;I, J&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n J: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Intersperse<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.Intersperse.html\\" title=\\"struct core::iter::adapters::intersperse::Intersperse\\">Intersperse</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.Intersperse.html\\" title=\\"struct core::iter::adapters::intersperse::Intersperse\\">Intersperse</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","IntersperseWith<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/intersperse/struct.IntersperseWith.html\\" title=\\"struct itertools::intersperse::IntersperseWith\\">IntersperseWith</a>&lt;I, ElemF&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, ElemF&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/intersperse/struct.IntersperseWith.html\\" title=\\"struct itertools::intersperse::IntersperseWith\\">IntersperseWith</a>&lt;I, ElemF&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n ElemF: IntersperseElement&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","IntersperseWith<Self, G>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.IntersperseWith.html\\" title=\\"struct core::iter::adapters::intersperse::IntersperseWith\\">IntersperseWith</a>&lt;I, G&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, G&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/intersperse/struct.IntersperseWith.html\\" title=\\"struct core::iter::adapters::intersperse::IntersperseWith\\">IntersperseWith</a>&lt;I, G&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n G: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>() -&gt; &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","IntersperseWith<Self, IntersperseElementSimple<Self::Item>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/intersperse/struct.IntersperseWith.html\\" title=\\"struct itertools::intersperse::IntersperseWith\\">IntersperseWith</a>&lt;I, ElemF&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, ElemF&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/intersperse/struct.IntersperseWith.html\\" title=\\"struct itertools::intersperse::IntersperseWith\\">IntersperseWith</a>&lt;I, ElemF&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n ElemF: IntersperseElement&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","IntoIter<Self::Item>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/into\_iter/struct.IntoIter.html\\" title=\\"struct alloc::collections::vec\_deque::into\_iter::IntoIter\\">IntoIter</a>&lt;T, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/into\_iter/struct.IntoIter.html\\" title=\\"struct alloc::collections::vec\_deque::into\_iter::IntoIter\\">IntoIter</a>&lt;T, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","KMergeBy<<Self::Item as IntoIterator>::IntoIter, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/kmerge\_impl/struct.KMergeBy.html\\" title=\\"struct itertools::kmerge\_impl::KMergeBy\\">KMergeBy</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/kmerge\_impl/struct.KMergeBy.html\\" title=\\"struct itertools::kmerge\_impl::KMergeBy\\">KMergeBy</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: KMergePredicate&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","KMergeBy<<Self::Item as IntoIterator>::IntoIter, KMergeByLt>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/kmerge\_impl/struct.KMergeBy.html\\" title=\\"struct itertools::kmerge\_impl::KMergeBy\\">KMergeBy</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/kmerge\_impl/struct.KMergeBy.html\\" title=\\"struct itertools::kmerge\_impl::KMergeBy\\">KMergeBy</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: KMergePredicate&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Map<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html\\" title=\\"struct core::iter::adapters::map::Map\\">Map</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B, I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/map/struct.Map.html\\" title=\\"struct core::iter::adapters::map::Map\\">Map</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; B,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = B;</div>","MapWhile<Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/map\_while/struct.MapWhile.html\\" title=\\"struct core::iter::adapters::map\_while::MapWhile\\">MapWhile</a>&lt;I, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B, I, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/map\_while/struct.MapWhile.html\\" title=\\"struct core::iter::adapters::map\_while::MapWhile\\">MapWhile</a>&lt;I, P&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;B&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = B;</div>","MapWindows<Self, F, N>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/map\_windows/struct.MapWindows.html\\" title=\\"struct core::iter::adapters::map\_windows::MapWindows\\">MapWindows</a>&lt;I, F, N&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F, R, const N: <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/map\_windows/struct.MapWindows.html\\" title=\\"struct core::iter::adapters::map\_windows::MapWindows\\">MapWindows</a>&lt;I, F, N&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;\[&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.array.html\\">N</a>\]) -&gt; R,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = R;</div>","MergeBy<Self, <J as IntoIterator>::IntoIter, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/merge\_join/struct.MergeBy.html\\" title=\\"struct itertools::merge\_join::MergeBy\\">MergeBy</a>&lt;I, J, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, J, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/merge\_join/struct.MergeBy.html\\" title=\\"struct itertools::merge\_join::MergeBy\\">MergeBy</a>&lt;I, J, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n J: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: OrderingOrBool&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;J as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;F as OrderingOrBool&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;J as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;&gt;::MergeResult;</div>","MergeBy<Self, <J as IntoIterator>::IntoIter, MergeFuncLR<F, <F as FuncLR<Self::Item, <<J as IntoIterator>::IntoIter as Iterator>::Item>>::T>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/merge\_join/struct.MergeBy.html\\" title=\\"struct itertools::merge\_join::MergeBy\\">MergeBy</a>&lt;I, J, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, J, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/merge\_join/struct.MergeBy.html\\" title=\\"struct itertools::merge\_join::MergeBy\\">MergeBy</a>&lt;I, J, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n J: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: OrderingOrBool&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;J as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;F as OrderingOrBool&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;J as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;&gt;::MergeResult;</div>","MergeBy<Self, <J as IntoIterator>::IntoIter, MergeLte>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/merge\_join/struct.MergeBy.html\\" title=\\"struct itertools::merge\_join::MergeBy\\">MergeBy</a>&lt;I, J, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, J, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/merge\_join/struct.MergeBy.html\\" title=\\"struct itertools::merge\_join::MergeBy\\">MergeBy</a>&lt;I, J, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n J: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: OrderingOrBool&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;J as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;F as OrderingOrBool&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;J as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;&gt;::MergeResult;</div>","MessageIterator<'a, M>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.MessageIterator.html\\" title=\\"struct bevy::ecs::message::MessageIterator\\">MessageIterator</a>&lt;'a, M&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, M&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.MessageIterator.html\\" title=\\"struct bevy::ecs::message::MessageIterator\\">MessageIterator</a>&lt;'a, M&gt;<div class=\\"where\\">where\\n M: <a class=\\"trait\\" href=\\"../../prelude/trait.Message.html\\" title=\\"trait bevy::prelude::Message\\">Message</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a M</a>;</div>","MessageIteratorWithId<'a, M>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.MessageIteratorWithId.html\\" title=\\"struct bevy::ecs::message::MessageIteratorWithId\\">MessageIteratorWithId</a>&lt;'a, M&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, M&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.MessageIteratorWithId.html\\" title=\\"struct bevy::ecs::message::MessageIteratorWithId\\">MessageIteratorWithId</a>&lt;'a, M&gt;<div class=\\"where\\">where\\n M: <a class=\\"trait\\" href=\\"../../prelude/trait.Message.html\\" title=\\"trait bevy::prelude::Message\\">Message</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a M</a>, <a class=\\"struct\\" href=\\"struct.MessageId.html\\" title=\\"struct bevy::ecs::message::MessageId\\">MessageId</a>&lt;M&gt;);</div>","MultiPeek<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/multipeek\_impl/struct.MultiPeek.html\\" title=\\"struct itertools::multipeek\_impl::MultiPeek\\">MultiPeek</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/multipeek\_impl/struct.MultiPeek.html\\" title=\\"struct itertools::multipeek\_impl::MultiPeek\\">MultiPeek</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","MultiProduct<<Self::Item as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/multi\_product/struct.MultiProduct.html\\" title=\\"struct itertools::adaptors::multi\_product::MultiProduct\\">MultiProduct</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/multi\_product/struct.MultiProduct.html\\" title=\\"struct itertools::adaptors::multi\_product::MultiProduct\\">MultiProduct</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"../../prelude/struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;;</div>","PadUsing<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/pad\_tail/struct.PadUsing.html\\" title=\\"struct itertools::pad\_tail::PadUsing\\">PadUsing</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/pad\_tail/struct.PadUsing.html\\" title=\\"struct itertools::pad\_tail::PadUsing\\">PadUsing</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>) -&gt; &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Peekable<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html\\" title=\\"struct core::iter::adapters::peekable::Peekable\\">Peekable</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/peekable/struct.Peekable.html\\" title=\\"struct core::iter::adapters::peekable::Peekable\\">Peekable</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","PeekingTakeWhile<'\_, Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/peeking\_take\_while/struct.PeekingTakeWhile.html\\" title=\\"struct itertools::peeking\_take\_while::PeekingTakeWhile\\">PeekingTakeWhile</a>&lt;'\_, I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/peeking\_take\_while/struct.PeekingTakeWhile.html\\" title=\\"struct itertools::peeking\_take\_while::PeekingTakeWhile\\">PeekingTakeWhile</a>&lt;'\_, I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/peeking\_take\_while/trait.PeekingNext.html\\" title=\\"trait itertools::peeking\_take\_while::PeekingNext\\">PeekingNext</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Permutations<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/permutations/struct.Permutations.html\\" title=\\"struct itertools::permutations::Permutations\\">Permutations</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/permutations/struct.Permutations.html\\" title=\\"struct itertools::permutations::Permutations\\">Permutations</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"../../prelude/struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;;</div>","Positions<Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Positions.html\\" title=\\"struct itertools::adaptors::Positions\\">Positions</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Positions.html\\" title=\\"struct itertools::adaptors::Positions\\">Positions</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>;</div>","Powerset<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/powerset/struct.Powerset.html\\" title=\\"struct itertools::powerset::Powerset\\">Powerset</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/powerset/struct.Powerset.html\\" title=\\"struct itertools::powerset::Powerset\\">Powerset</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"../../prelude/struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;;</div>","Product<Self, <J as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Product.html\\" title=\\"struct itertools::adaptors::Product\\">Product</a>&lt;I, J&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, J&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Product.html\\" title=\\"struct itertools::adaptors::Product\\">Product</a>&lt;I, J&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n J: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;J as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>);</div>","Rev<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/rev/struct.Rev.html\\" title=\\"struct core::iter::adapters::rev::Rev\\">Rev</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/rev/struct.Rev.html\\" title=\\"struct core::iter::adapters::rev::Rev\\">Rev</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/double\_ended/trait.DoubleEndedIterator.html\\" title=\\"trait core::iter::traits::double\_ended::DoubleEndedIterator\\">DoubleEndedIterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Scan<Self, St, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/scan/struct.Scan.html\\" title=\\"struct core::iter::adapters::scan::Scan\\">Scan</a>&lt;I, St, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B, I, St, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/scan/struct.Scan.html\\" title=\\"struct core::iter::adapters::scan::Scan\\">Scan</a>&lt;I, St, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;mut St</a>, &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;B&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = B;</div>","Skip<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/skip/struct.Skip.html\\" title=\\"struct core::iter::adapters::skip::Skip\\">Skip</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/skip/struct.Skip.html\\" title=\\"struct core::iter::adapters::skip::Skip\\">Skip</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","SkipWhile<Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/skip\_while/struct.SkipWhile.html\\" title=\\"struct core::iter::adapters::skip\_while::SkipWhile\\">SkipWhile</a>&lt;I, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/skip\_while/struct.SkipWhile.html\\" title=\\"struct core::iter::adapters::skip\_while::SkipWhile\\">SkipWhile</a>&lt;I, P&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","StepBy<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/step\_by/struct.StepBy.html\\" title=\\"struct core::iter::adapters::step\_by::StepBy\\">StepBy</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/step\_by/struct.StepBy.html\\" title=\\"struct core::iter::adapters::step\_by::StepBy\\">StepBy</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Take<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/take/struct.Take.html\\" title=\\"struct core::iter::adapters::take::Take\\">Take</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/take/struct.Take.html\\" title=\\"struct core::iter::adapters::take::Take\\">Take</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","TakeWhile<Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/take\_while/struct.TakeWhile.html\\" title=\\"struct core::iter::adapters::take\_while::TakeWhile\\">TakeWhile</a>&lt;I, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/take\_while/struct.TakeWhile.html\\" title=\\"struct core::iter::adapters::take\_while::TakeWhile\\">TakeWhile</a>&lt;I, P&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","TakeWhileInclusive<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/take\_while\_inclusive/struct.TakeWhileInclusive.html\\" title=\\"struct itertools::take\_while\_inclusive::TakeWhileInclusive\\">TakeWhileInclusive</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/take\_while\_inclusive/struct.TakeWhileInclusive.html\\" title=\\"struct itertools::take\_while\_inclusive::TakeWhileInclusive\\">TakeWhileInclusive</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","TakeWhileRef<'\_, Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.TakeWhileRef.html\\" title=\\"struct itertools::adaptors::TakeWhileRef\\">TakeWhileRef</a>&lt;'\_, I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.TakeWhileRef.html\\" title=\\"struct itertools::adaptors::TakeWhileRef\\">TakeWhileRef</a>&lt;'\_, I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","TupleCombinations<Self, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.TupleCombinations.html\\" title=\\"struct itertools::adaptors::TupleCombinations\\">TupleCombinations</a>&lt;I, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.TupleCombinations.html\\" title=\\"struct itertools::adaptors::TupleCombinations\\">TupleCombinations</a>&lt;I, T&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n T: HasCombination&lt;I&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","TupleWindows<Self, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/tuple\_impl/struct.TupleWindows.html\\" title=\\"struct itertools::tuple\_impl::TupleWindows\\">TupleWindows</a>&lt;I, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/tuple\_impl/struct.TupleWindows.html\\" title=\\"struct itertools::tuple\_impl::TupleWindows\\">TupleWindows</a>&lt;I, T&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;T as TupleCollect&gt;::Item&gt;,\\n T: <a class=\\"trait\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/tuple\_impl/trait.HomogeneousTuple.html\\" title=\\"trait itertools::tuple\_impl::HomogeneousTuple\\">HomogeneousTuple</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,\\n &lt;T as TupleCollect&gt;::Item: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","Tuples<Self, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/tuple\_impl/struct.Tuples.html\\" title=\\"struct itertools::tuple\_impl::Tuples\\">Tuples</a>&lt;I, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/tuple\_impl/struct.Tuples.html\\" title=\\"struct itertools::tuple\_impl::Tuples\\">Tuples</a>&lt;I, T&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;T as TupleCollect&gt;::Item&gt;,\\n T: <a class=\\"trait\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/tuple\_impl/trait.HomogeneousTuple.html\\" title=\\"trait itertools::tuple\_impl::HomogeneousTuple\\">HomogeneousTuple</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","Unique<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/unique\_impl/struct.Unique.html\\" title=\\"struct itertools::unique\_impl::Unique\\">Unique</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/unique\_impl/struct.Unique.html\\" title=\\"struct itertools::unique\_impl::Unique\\">Unique</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\\" title=\\"trait core::clone::Clone\\">Clone</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","UniqueBy<Self, V, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/unique\_impl/struct.UniqueBy.html\\" title=\\"struct itertools::unique\_impl::UniqueBy\\">UniqueBy</a>&lt;I, V, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, V, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/unique\_impl/struct.UniqueBy.html\\" title=\\"struct itertools::unique\_impl::UniqueBy\\">UniqueBy</a>&lt;I, V, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n V: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\\" title=\\"trait core::cmp::Eq\\">Eq</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html\\" title=\\"trait core::hash::Hash\\">Hash</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>) -&gt; V,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Update<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Update.html\\" title=\\"struct itertools::adaptors::Update\\">Update</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.Update.html\\" title=\\"struct itertools::adaptors::Update\\">Update</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;mut &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>),</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","WhileSome<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.WhileSome.html\\" title=\\"struct itertools::adaptors::WhileSome\\">WhileSome</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/adaptors/struct.WhileSome.html\\" title=\\"struct itertools::adaptors::WhileSome\\">WhileSome</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;A&gt;&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = A;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithPosition<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/with\_position/struct.WithPosition.html\\" title=\\"struct itertools::with\_position::WithPosition\\">WithPosition</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/with\_position/struct.WithPosition.html\\" title=\\"struct itertools::with\_position::WithPosition\\">WithPosition</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"enum\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/with\_position/enum.Position.html\\" title=\\"enum itertools::with\_position::Position\\">Position</a>, &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>);</div>","Zip<Self, <U as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html\\" title=\\"struct core::iter::adapters::zip::Zip\\">Zip</a>&lt;A, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A, B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/adapters/zip/struct.Zip.html\\" title=\\"struct core::iter::adapters::zip::Zip\\">Zip</a>&lt;A, B&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n B: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&lt;A as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;B as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>);</div>","ZipEq<Self, <J as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/zip\_eq\_impl/struct.ZipEq.html\\" title=\\"struct itertools::zip\_eq\_impl::ZipEq\\">ZipEq</a>&lt;I, J&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, J&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/zip\_eq\_impl/struct.ZipEq.html\\" title=\\"struct itertools::zip\_eq\_impl::ZipEq\\">ZipEq</a>&lt;I, J&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n J: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;J as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>);</div>","ZipLongest<Self, <J as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/zip\_longest/struct.ZipLongest.html\\" title=\\"struct itertools::zip\_longest::ZipLongest\\">ZipLongest</a>&lt;T, U&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, U&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/zip\_longest/struct.ZipLongest.html\\" title=\\"struct itertools::zip\_longest::ZipLongest\\">ZipLongest</a>&lt;T, U&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n U: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"enum\\" href=\\"https://docs.rs/itertools/0.14.0/x86\_64-unknown-linux-gnu/itertools/either\_or\_both/enum.EitherOrBoth.html\\" title=\\"enum itertools::either\_or\_both::EitherOrBoth\\">EitherOrBoth</a>&lt;&lt;T as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>, &lt;U as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;;</div>"}