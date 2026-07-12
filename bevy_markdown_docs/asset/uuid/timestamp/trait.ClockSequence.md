[bevy](../../../index.html)::[asset](../../index.html)::[uuid](../index.html)::[timestamp](index.html)

# Trait ClockSequence 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#411)

```rust
pub trait ClockSequence {
    type Output;

    // Required method
    fn generate_sequence(&self, seconds: u64, subsec_nanos: u32) -> Self::Output;

    // Provided methods
    fn generate_timestamp_sequence(
        &self,
        seconds: u64,
        subsec_nanos: u32,
    ) -> (Self::Output, u64, u32) { ... }
    fn usable_bits(&self) -> usize
       where Self::Output: Sized { ... }
}
```

A counter that can be used by versions 1 and 6 UUIDs to support the uniqueness of timestamps.

## References

*   [UUID Version 1 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.1)
*   [UUID Version 6 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.6)
*   [UUID Generator States in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-6.3)

## Required Associated Types

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#413)

#### type [Output](#associatedtype.Output)

The type of sequence returned by this counter.

## Required Methods

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#420)

#### fn [generate\_sequence](#tymethod.generate_sequence)(&self, seconds: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), subsec\_nanos: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> Self::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output")

Get the next value in the sequence to feed into a timestamp.

This method will be called each time a [`Timestamp`](../struct.Timestamp.html "struct bevy::asset::uuid::Timestamp") is constructed.

Any bits beyond [`ClockSequence::usable_bits`](../trait.ClockSequence.html#method.usable_bits "method bevy::asset::uuid::ClockSequence::usable_bits") in the output must be unset.

## Provided Methods

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#427-431)

#### fn [generate\_timestamp\_sequence](#method.generate_timestamp_sequence)( &self, seconds: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), subsec\_nanos: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> (Self::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output"), [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Get the next value in the sequence, potentially also adjusting the timestamp.

This method should be preferred over `generate_sequence`.

Any bits beyond [`ClockSequence::usable_bits`](../trait.ClockSequence.html#method.usable_bits "method bevy::asset::uuid::ClockSequence::usable_bits") in the output must be unset.

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#446-448)

#### fn [usable\_bits](#method.usable_bits)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where Self::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The number of usable bits from the least significant bit in the result of [`ClockSequence::generate_sequence`](../trait.ClockSequence.html#tymethod.generate_sequence "method bevy::asset::uuid::ClockSequence::generate_sequence") or [`ClockSequence::generate_timestamp_sequence`](../trait.ClockSequence.html#method.generate_timestamp_sequence "method bevy::asset::uuid::ClockSequence::generate_timestamp_sequence").

The number of usable bits must not exceed 128.

The number of usable bits is not expected to change between calls. An implementation of `ClockSequence` should always return the same value from this method.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#649)

### impl<C> [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") for [AssertUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/struct.AssertUnwindSafe.html "struct core::panic::unwind_safe::AssertUnwindSafe")<C>

where C: [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence"),

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#650)

#### type [Output](#associatedtype.Output) = <C as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output")

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#652)

#### fn [generate\_sequence](#tymethod.generate_sequence)( &self, seconds: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), subsec\_nanos: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> <[AssertUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/struct.AssertUnwindSafe.html "struct core::panic::unwind_safe::AssertUnwindSafe")<C> as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output") [ⓘ](#)

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#656-660)

#### fn [generate\_timestamp\_sequence](#method.generate_timestamp_sequence)( &self, seconds: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), subsec\_nanos: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> (<[AssertUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/struct.AssertUnwindSafe.html "struct core::panic::unwind_safe::AssertUnwindSafe")<C> as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output"), [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#664-666)

#### fn [usable\_bits](#method.usable_bits)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where <[AssertUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/struct.AssertUnwindSafe.html "struct core::panic::unwind_safe::AssertUnwindSafe")<C> as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#454)

### impl<T> [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#455)

#### type [Output](#associatedtype.Output) = <T as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output")

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#457)

#### fn [generate\_sequence](#tymethod.generate_sequence)( &self, seconds: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), subsec\_nanos: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output")

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#461-465)

#### fn [generate\_timestamp\_sequence](#method.generate_timestamp_sequence)( &self, seconds: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), subsec\_nanos: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> (<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output"), [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#469-471)

#### fn [usable\_bits](#method.usable_bits)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

## Implementors

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#1191)

### impl [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") for [NoContext](../struct.NoContext.html "struct bevy::asset::uuid::NoContext")

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#1192)

#### type [Output](#associatedtype.Output) = [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#672)

### impl<C> [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") for [Mutex](../../../platform/sync/struct.Mutex.html "struct bevy::platform::sync::Mutex")<C>

where C: [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") + [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#673)

#### type [Output](#associatedtype.Output) = <C as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output")

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#627)

### impl<C> [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") for [ThreadLocalContext](context/struct.ThreadLocalContext.html "struct bevy::asset::uuid::timestamp::context::ThreadLocalContext")<C>

where C: [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") + 'static,

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#628)

#### type [Output](#associatedtype.Output) = <C as [ClockSequence](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output")

{"<AssertUnwindSafe<C> as ClockSequence>::Output":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/panic/unwind\_safe/struct.AssertUnwindSafe.html\\" title=\\"struct core::panic::unwind\_safe::AssertUnwindSafe\\">AssertUnwindSafe</a>&lt;F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;F&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/panic/unwind\_safe/struct.AssertUnwindSafe.html\\" title=\\"struct core::panic::unwind\_safe::AssertUnwindSafe\\">AssertUnwindSafe</a>&lt;F&gt;<div class=\\"where\\">where\\n F: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;F as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}