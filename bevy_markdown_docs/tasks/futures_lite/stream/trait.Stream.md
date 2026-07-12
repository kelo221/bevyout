[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Trait Stream 

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#37)

```rust
pub trait Stream {
    type Item;

    // Required method
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>>;

    // Provided method
    fn size_hint(&self) -> (usize, Option<usize>) { ... }
}
```

A stream of values produced asynchronously.

If `Future<Output = T>` is an asynchronous version of `T`, then `Stream<Item = T>` is an asynchronous version of `Iterator<Item = T>`. A stream represents a sequence of value-producing events that occur asynchronously to the caller.

The trait is modeled after `Future`, but allows `poll_next` to be called even after a value has been produced, yielding `None` once the stream has been fully exhausted.

## Required Associated Types

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#39)

#### type [Item](#associatedtype.Item)

Values yielded by the stream.

## Required Methods

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#76)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted.

##### Return value

There are several possible return values, each indicating a distinct stream state:

*   `Poll::Pending` means that this stream’s next value is not ready yet. Implementations will ensure that the current task will be notified when the next value may be ready.
    
*   `Poll::Ready(Some(val))` means that the stream has successfully produced a value, `val`, and may produce further values on subsequent `poll_next` calls.
    
*   `Poll::Ready(None)` means that the stream has terminated, and `poll_next` should not be invoked again.
    

##### Panics

Once a stream has finished (returned `Ready(None)` from `poll_next`), calling its `poll_next` method again may panic, block forever, or cause other kinds of problems; the `Stream` trait places no requirements on the effects of such a call. However, as the `poll_next` method is not marked `unsafe`, Rust’s usual rules apply: calls must never cause undefined behavior (memory corruption, incorrect use of `unsafe` functions, or the like), regardless of the stream’s state.

If this is difficult to guard against then the [`fuse`](https://docs.rs/futures/0.3/futures/stream/trait.StreamExt.html#method.fuse) adapter can be used to ensure that `poll_next` always returns `Ready(None)` in subsequent calls.

## Provided Methods

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#105)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

Returns the bounds on the remaining length of the stream.

Specifically, `size_hint()` returns a tuple where the first element is the lower bound, and the second element is the upper bound.

The second half of the tuple that is returned is an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")`<`[`usize`](https://doc.rust-lang.org/nightly/std/primitive.usize.html "primitive usize")`>`. A [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") here means that either there is no known upper bound, or the upper bound is larger than [`usize`](https://doc.rust-lang.org/nightly/std/primitive.usize.html "primitive usize").

##### Implementation notes

It is not enforced that a stream implementation yields the declared number of elements. A buggy stream may yield less than the lower bound or more than the upper bound of elements.

`size_hint()` is primarily intended to be used for optimizations such as reserving space for the elements of the stream, but must not be trusted to e.g., omit bounds checks in unsafe code. An incorrect implementation of `size_hint()` should not lead to memory safety violations.

That said, the implementation should provide a correct estimation, because otherwise it would be a violation of the trait’s protocol.

The default implementation returns `(0,` [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None")`)` which is correct for any stream.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/src/async_signal/lib.rs.html#415)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for &[Signals](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/async_signal/struct.Signals.html "struct async_signal::Signals")

[Source](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/src/async_signal/lib.rs.html#416)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Signal](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/async_signal/enum.Signal.html "enum async_signal::Signal"), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/src/async_signal/lib.rs.html#418)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &[Signals](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/async_signal/struct.Signals.html "struct async_signal::Signals")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<&[Signals](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/async_signal/struct.Signals.html "struct async_signal::Signals") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/src/async_signal/lib.rs.html#424)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/object_manager.rs.html#34-38)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [InterfacesAddedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/object_manager/struct.InterfacesAddedStream.html "struct zbus::fdo::object_manager::InterfacesAddedStream")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/object_manager.rs.html#34-38)

#### type [Item](#associatedtype.Item) = [InterfacesAdded](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/object_manager/struct.InterfacesAdded.html "struct zbus::fdo::object_manager::InterfacesAdded")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/object_manager.rs.html#34-38)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [InterfacesAddedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/object_manager/struct.InterfacesAddedStream.html "struct zbus::fdo::object_manager::InterfacesAddedStream")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[InterfacesAddedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/object_manager/struct.InterfacesAddedStream.html "struct zbus::fdo::object_manager::InterfacesAddedStream") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/object_manager.rs.html#34-38)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [InterfacesRemovedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/object_manager/struct.InterfacesRemovedStream.html "struct zbus::fdo::object_manager::InterfacesRemovedStream")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/object_manager.rs.html#34-38)

#### type [Item](#associatedtype.Item) = [InterfacesRemoved](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/object_manager/struct.InterfacesRemoved.html "struct zbus::fdo::object_manager::InterfacesRemoved")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/object_manager.rs.html#34-38)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [InterfacesRemovedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/object_manager/struct.InterfacesRemovedStream.html "struct zbus::fdo::object_manager::InterfacesRemovedStream")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[InterfacesRemovedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/object_manager/struct.InterfacesRemovedStream.html "struct zbus::fdo::object_manager::InterfacesRemovedStream") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/message_stream.rs.html#180)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [MessageStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/message_stream/struct.MessageStream.html "struct zbus::message_stream::MessageStream")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/message_stream.rs.html#181)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Message](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/message/struct.Message.html "struct zbus::message::Message"), [Error](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/error/enum.Error.html "enum zbus::error::Error")\>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/message_stream.rs.html#183)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [MessageStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/message_stream/struct.MessageStream.html "struct zbus::message_stream::MessageStream")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[MessageStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/message_stream/struct.MessageStream.html "struct zbus::message_stream::MessageStream") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/dbus.rs.html#276-280)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [NameAcquiredStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameAcquiredStream.html "struct zbus::fdo::dbus::NameAcquiredStream")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/dbus.rs.html#276-280)

#### type [Item](#associatedtype.Item) = [NameAcquired](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameAcquired.html "struct zbus::fdo::dbus::NameAcquired")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/dbus.rs.html#276-280)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [NameAcquiredStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameAcquiredStream.html "struct zbus::fdo::dbus::NameAcquiredStream")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[NameAcquiredStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameAcquiredStream.html "struct zbus::fdo::dbus::NameAcquiredStream") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/dbus.rs.html#276-280)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [NameLostStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameLostStream.html "struct zbus::fdo::dbus::NameLostStream")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/dbus.rs.html#276-280)

#### type [Item](#associatedtype.Item) = [NameLost](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameLost.html "struct zbus::fdo::dbus::NameLost")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/dbus.rs.html#276-280)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [NameLostStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameLostStream.html "struct zbus::fdo::dbus::NameLostStream")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[NameLostStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameLostStream.html "struct zbus::fdo::dbus::NameLostStream") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/dbus.rs.html#276-280)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [NameOwnerChangedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameOwnerChangedStream.html "struct zbus::fdo::dbus::NameOwnerChangedStream")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/dbus.rs.html#276-280)

#### type [Item](#associatedtype.Item) = [NameOwnerChanged](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameOwnerChanged.html "struct zbus::fdo::dbus::NameOwnerChanged")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/dbus.rs.html#276-280)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [NameOwnerChangedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameOwnerChangedStream.html "struct zbus::fdo::dbus::NameOwnerChangedStream")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[NameOwnerChangedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/dbus/struct.NameOwnerChangedStream.html "struct zbus::fdo::dbus::NameOwnerChangedStream") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/proxy/mod.rs.html#1092)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [OwnerChangedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.OwnerChangedStream.html "struct zbus::proxy::OwnerChangedStream")<'\_>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/proxy/mod.rs.html#1093)

#### type [Item](#associatedtype.Item) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UniqueName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/unique_name/struct.UniqueName.html "struct zbus_names::unique_name::UniqueName")<'static>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/proxy/mod.rs.html#1095)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [OwnerChangedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.OwnerChangedStream.html "struct zbus::proxy::OwnerChangedStream")<'\_>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[OwnerChangedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.OwnerChangedStream.html "struct zbus::proxy::OwnerChangedStream")<'\_> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/properties.rs.html#18-22)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [PropertiesChangedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/properties/struct.PropertiesChangedStream.html "struct zbus::fdo::properties::PropertiesChangedStream")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/properties.rs.html#18-22)

#### type [Item](#associatedtype.Item) = [PropertiesChanged](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/properties/struct.PropertiesChanged.html "struct zbus::fdo::properties::PropertiesChanged")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/fdo/properties.rs.html#18-22)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [PropertiesChangedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/properties/struct.PropertiesChangedStream.html "struct zbus::fdo::properties::PropertiesChangedStream")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[PropertiesChangedStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/fdo/properties/struct.PropertiesChangedStream.html "struct zbus::fdo::properties::PropertiesChangedStream") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#333)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [ReadDir](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.ReadDir.html "struct async_fs::ReadDir")

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#334)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[DirEntry](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.DirEntry.html "struct async_fs::DirEntry"), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#336)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [ReadDir](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.ReadDir.html "struct async_fs::ReadDir")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[ReadDir](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.ReadDir.html "struct async_fs::ReadDir") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/proxy/mod.rs.html#1284)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [SignalStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.SignalStream.html "struct zbus::proxy::SignalStream")<'\_>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/proxy/mod.rs.html#1285)

#### type [Item](#associatedtype.Item) = [Message](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/message/struct.Message.html "struct zbus::message::Message")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/proxy/mod.rs.html#1287)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [SignalStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.SignalStream.html "struct zbus::proxy::SignalStream")<'\_>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[SignalStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.SignalStream.html "struct zbus::proxy::SignalStream")<'\_> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/src/async_signal/lib.rs.html#400)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Signals](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/async_signal/struct.Signals.html "struct async_signal::Signals")

[Source](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/src/async_signal/lib.rs.html#401)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Signal](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/async_signal/enum.Signal.html "enum async_signal::Signal"), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/src/async_signal/lib.rs.html#404)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Signals](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/async_signal/struct.Signals.html "struct async_signal::Signals")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Signals](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/async_signal/struct.Signals.html "struct async_signal::Signals") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/async-signal/0.2.14/x86_64-unknown-linux-gnu/src/async_signal/lib.rs.html#409)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#476)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Timer](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Timer.html "struct async_io::Timer")

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#477)

#### type [Item](#associatedtype.Item) = [Instant](../../../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#479)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Timer](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Timer.html "struct async_io::Timer")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Timer](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Timer.html "struct async_io::Timer") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/proxy/mod.rs.html#219-221)

### impl<'a, T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [PropertyStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.PropertyStream.html "struct zbus::proxy::PropertyStream")<'a, T>

where T: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/proxy/mod.rs.html#223)

#### type [Item](#associatedtype.Item) = [PropertyChanged](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.PropertyChanged.html "struct zbus::proxy::PropertyChanged")<'a, T>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/proxy/mod.rs.html#225)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [PropertyStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.PropertyStream.html "struct zbus::proxy::PropertyStream")<'a, T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[PropertyStream](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/proxy/struct.PropertyStream.html "struct zbus::proxy::PropertyStream")<'a, T> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/either.rs.html#127-130)

### impl<A, B> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<A, B>

where A: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), B: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = <A as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/either.rs.html#132)

#### type [Item](#associatedtype.Item) = <A as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/either.rs.html#134)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<A, B>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<A, B> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/either.rs.html#141)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/repeat_with.rs.html#19)

### impl<A, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [RepeatWith](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/repeat_with/struct.RepeatWith.html "struct futures_util::stream::repeat_with::RepeatWith")<F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")() -> A,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/repeat_with.rs.html#20)

#### type [Item](#associatedtype.Item) = A

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/repeat_with.rs.html#22)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [RepeatWith](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/repeat_with/struct.RepeatWith.html "struct futures_util::stream::repeat_with::RepeatWith")<F>>, \_: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[RepeatWith](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/repeat_with/struct.RepeatWith.html "struct futures_util::stream::repeat_with::RepeatWith")<F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/repeat_with.rs.html#26)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/scan.rs.html#65-69)

### impl<B, St, S, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Scan](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/scan/struct.Scan.html "struct futures_util::stream::stream::scan::Scan")<St, S, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html), <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/scan.rs.html#71)

#### type [Item](#associatedtype.Item) = B

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/scan.rs.html#73)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Scan](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/scan/struct.Scan.html "struct futures_util::stream::stream::scan::Scan")<St, S, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/scan.rs.html#99)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#120-122)

### impl<B> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [BodyDataStream](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.BodyDataStream.html "struct http_body_util::stream::BodyDataStream")<B>

where B: [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body"),

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#124)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<B as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Data](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data "type http_body::Body::Data"), <B as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error")\>

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#126)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [BodyDataStream](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.BodyDataStream.html "struct http_body_util::stream::BodyDataStream")<B>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[BodyDataStream](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.BodyDataStream.html "struct http_body_util::stream::BodyDataStream")<B> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#89-91)

### impl<B> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [BodyStream](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.BodyStream.html "struct http_body_util::stream::BodyStream")<B>

where B: [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body"),

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#93)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Frame](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/frame/struct.Frame.html "struct http_body::frame::Frame")<<B as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Data](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data "type http_body::Body::Data")\>, <B as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error")\>

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#95)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [BodyStream](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.BodyStream.html "struct http_body_util::stream::BodyStream")<B>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[BodyStream](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.BodyStream.html "struct http_body_util::stream::BodyStream")<B> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#36-42)

### impl<F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [FlattenStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.FlattenStream.html "struct futures_util::future::future::FlattenStream")<F>

where Flatten<F, <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#36-42)

#### type [Item](#associatedtype.Item) = <Flatten<F, <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#36-42)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [FlattenStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.FlattenStream.html "struct futures_util::future::future::FlattenStream")<F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[FlattenStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.FlattenStream.html "struct futures_util::future::future::FlattenStream")<F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#36-42)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#54-59)

### impl<F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.IntoStream.html "struct futures_util::future::future::IntoStream")<F>

where [Once](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/once/struct.Once.html "struct futures_util::stream::once::Once")<F>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#54-59)

#### type [Item](#associatedtype.Item) = <[Once](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/once/struct.Once.html "struct futures_util::stream::once::Once")<F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#54-59)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.IntoStream.html "struct futures_util::future::future::IntoStream")<F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.IntoStream.html "struct futures_util::future::future::IntoStream")<F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#54-59)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/futures_ordered.rs.html#191)

### impl<Fut> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [FuturesOrdered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/futures_ordered/struct.FuturesOrdered.html "struct futures_util::stream::futures_ordered::FuturesOrdered")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/futures_ordered.rs.html#192)

#### type [Item](#associatedtype.Item) = <Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/futures_ordered.rs.html#194)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [FuturesOrdered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/futures_ordered/struct.FuturesOrdered.html "struct futures_util::stream::futures_ordered::FuturesOrdered")<Fut>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[FuturesOrdered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/futures_ordered/struct.FuturesOrdered.html "struct futures_util::stream::futures_ordered::FuturesOrdered")<Fut> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/futures_ordered.rs.html#220)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/futures_unordered/mod.rs.html#397)

### impl<Fut> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [FuturesUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/futures_unordered/struct.FuturesUnordered.html "struct futures_util::stream::futures_unordered::FuturesUnordered")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/futures_unordered/mod.rs.html#398)

#### type [Item](#associatedtype.Item) = <Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/futures_unordered/mod.rs.html#400)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [FuturesUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/futures_unordered/struct.FuturesUnordered.html "struct futures_util::stream::futures_unordered::FuturesUnordered")<Fut>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[FuturesUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/futures_unordered/struct.FuturesUnordered.html "struct futures_util::stream::futures_unordered::FuturesUnordered")<Fut> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/futures_unordered/mod.rs.html#556)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/once.rs.html#40)

### impl<Fut> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Once](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/once/struct.Once.html "struct futures_util::stream::once::Once")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/once.rs.html#41)

#### type [Item](#associatedtype.Item) = <Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/once.rs.html#43)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Once](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/once/struct.Once.html "struct futures_util::stream::once::Once")<Fut>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Once](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/once/struct.Once.html "struct futures_util::stream::once::Once")<Fut> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/once.rs.html#54)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#44-50)

### impl<Fut> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryFlattenStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.TryFlattenStream.html "struct futures_util::future::try_future::TryFlattenStream")<Fut>

where TryFlatten<Fut, <Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")\>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#44-50)

#### type [Item](#associatedtype.Item) = <TryFlatten<Fut, <Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")\> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#44-50)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryFlattenStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.TryFlattenStream.html "struct futures_util::future::try_future::TryFlattenStream")<Fut>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryFlattenStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.TryFlattenStream.html "struct futures_util::future::try_future::TryFlattenStream")<Fut> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#44-50)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/iter.rs.html#53-55)

### impl<I> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Iter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/iter/struct.Iter.html "struct futures_util::stream::iter::Iter")<I>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/iter.rs.html#57)

#### type [Item](#associatedtype.Item) = <I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/iter.rs.html#59)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Iter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/iter/struct.Iter.html "struct futures_util::stream::iter::Iter")<I>>, \_: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/iter.rs.html#63)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#122-125)

### impl<P> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>

where P: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), <P as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#127)

#### type [Item](#associatedtype.Item) = <<P as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target") as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#129)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#133)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#110)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#111)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#113)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#117)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#228)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [AssertUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/struct.AssertUnwindSafe.html "struct core::panic::unwind_safe::AssertUnwindSafe")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

Available on **crate feature `std`** only.

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#229)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#231)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [AssertUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/struct.AssertUnwindSafe.html "struct core::panic::unwind_safe::AssertUnwindSafe")<S>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#235)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/fuse.rs.html#43)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Fuse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/fuse/struct.Fuse.html "struct futures_util::stream::stream::fuse::Fuse")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/fuse.rs.html#44)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/fuse.rs.html#46)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Fuse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/fuse/struct.Fuse.html "struct futures_util::stream::stream::fuse::Fuse")<S>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/fuse.rs.html#60)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#471)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [IntoOrdering](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.IntoOrdering.html "struct ordered_stream::adapters::IntoOrdering")<S>

where S: [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream"),

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#472)

#### type [Item](#associatedtype.Item) = <S as [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream")\>::[Ordering](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html#associatedtype.Ordering "type ordered_stream::OrderedStream::Ordering")

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#474)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [IntoOrdering](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.IntoOrdering.html "struct ordered_stream::adapters::IntoOrdering")<S>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[IntoOrdering](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.IntoOrdering.html "struct ordered_stream::adapters::IntoOrdering")<S> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#481)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#405)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [IntoStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.IntoStream.html "struct ordered_stream::adapters::IntoStream")<S>

where S: [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream"),

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#406)

#### type [Item](#associatedtype.Item) = <S as [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream")\>::[Data](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html#associatedtype.Data "type ordered_stream::OrderedStream::Data")

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#408)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [IntoStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.IntoStream.html "struct ordered_stream::adapters::IntoStream")<S>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[IntoStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.IntoStream.html "struct ordered_stream::adapters::IntoStream")<S> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#415)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#438)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [IntoTupleStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.IntoTupleStream.html "struct ordered_stream::adapters::IntoTupleStream")<S>

where S: [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream"),

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#439)

#### type [Item](#associatedtype.Item) = (<S as [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream")\>::[Ordering](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html#associatedtype.Ordering "type ordered_stream::OrderedStream::Ordering"), <S as [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream")\>::[Data](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html#associatedtype.Data "type ordered_stream::OrderedStream::Data"))

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#441)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [IntoTupleStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.IntoTupleStream.html "struct ordered_stream::adapters::IntoTupleStream")<S>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[IntoTupleStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.IntoTupleStream.html "struct ordered_stream::adapters::IntoTupleStream")<S> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#448)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#203)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Peekable](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/peek/struct.Peekable.html "struct futures_util::stream::stream::peek::Peekable")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#204)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#206)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Peekable](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/peek/struct.Peekable.html "struct futures_util::stream::stream::peek::Peekable")<S>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Peekable](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/peek/struct.Peekable.html "struct futures_util::stream::stream::peek::Peekable")<S> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#214)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#46)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [StreamBody](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.StreamBody.html "struct http_body_util::stream::StreamBody")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#47)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#49)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [StreamBody](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.StreamBody.html "struct http_body_util::stream::StreamBody")<S>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[StreamBody](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.StreamBody.html "struct http_body_util::stream::StreamBody")<S> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/stream.rs.html#53)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/select_with_strategy.rs.html#256-260)

### impl<St1, St2, Clos, State> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [SelectWithStrategy](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/select_with_strategy/struct.SelectWithStrategy.html "struct futures_util::stream::select_with_strategy::SelectWithStrategy")<St1, St2, Clos, State>

where St1: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), St2: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = <St1 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>, Clos: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut State](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [PollNext](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/select_with_strategy/enum.PollNext.html "enum futures_util::stream::select_with_strategy::PollNext"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/select_with_strategy.rs.html#262)

#### type [Item](#associatedtype.Item) = <St1 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/select_with_strategy.rs.html#264)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [SelectWithStrategy](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/select_with_strategy/struct.SelectWithStrategy.html "struct futures_util::stream::select_with_strategy::SelectWithStrategy")<St1, St2, Clos, State>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St1 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/chain.rs.html#40-43)

### impl<St1, St2> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Chain](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/chain/struct.Chain.html "struct futures_util::stream::stream::chain::Chain")<St1, St2>

where St1: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), St2: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = <St1 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/chain.rs.html#45)

#### type [Item](#associatedtype.Item) = <St1 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/chain.rs.html#47)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Chain](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/chain/struct.Chain.html "struct futures_util::stream::stream::chain::Chain")<St1, St2>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Chain](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/chain/struct.Chain.html "struct futures_util::stream::stream::chain::Chain")<St1, St2> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/chain.rs.html#59)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/select.rs.html#106-109)

### impl<St1, St2> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Select](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/select/struct.Select.html "struct futures_util::stream::select::Select")<St1, St2>

where St1: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), St2: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = <St1 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/select.rs.html#111)

#### type [Item](#associatedtype.Item) = <St1 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/select.rs.html#113)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Select](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/select/struct.Select.html "struct futures_util::stream::select::Select")<St1, St2>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St1 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/zip.rs.html#71-74)

### impl<St1, St2> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Zip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/zip/struct.Zip.html "struct futures_util::stream::stream::zip::Zip")<St1, St2>

where St1: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), St2: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/zip.rs.html#76)

#### type [Item](#associatedtype.Item) = (<St1 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"), <St2 as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"))

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/zip.rs.html#78)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Zip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/zip/struct.Zip.html "struct futures_util::stream::stream::zip::Zip")<St1, St2>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Zip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/zip/struct.Zip.html "struct futures_util::stream::stream::zip::Zip")<St1, St2> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/zip.rs.html#104)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#29-34)

### impl<St, E> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [ErrInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.ErrInto.html "struct futures_util::stream::try_stream::ErrInto")<St, E>

where [MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapErr.html "struct futures_util::stream::try_stream::MapErr")<St, IntoFn<E>>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#29-34)

#### type [Item](#associatedtype.Item) = <[MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapErr.html "struct futures_util::stream::try_stream::MapErr")<St, IntoFn<E>> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#29-34)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [ErrInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.ErrInto.html "struct futures_util::stream::try_stream::ErrInto")<St, E>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[ErrInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.ErrInto.html "struct futures_util::stream::try_stream::ErrInto")<St, E> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#29-34)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#110-115)

### impl<St, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Inspect.html "struct futures_util::stream::stream::Inspect")<St, F>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<St, InspectFn<F>>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#110-115)

#### type [Item](#associatedtype.Item) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<St, InspectFn<F>> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#110-115)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Inspect.html "struct futures_util::stream::stream::Inspect")<St, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Inspect.html "struct futures_util::stream::stream::Inspect")<St, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#110-115)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#43-48)

### impl<St, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [InspectErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.InspectErr.html "struct futures_util::stream::try_stream::InspectErr")<St, F>

where [Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Inspect.html "struct futures_util::stream::stream::Inspect")<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>, InspectErrFn<F>>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#43-48)

#### type [Item](#associatedtype.Item) = <[Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Inspect.html "struct futures_util::stream::stream::Inspect")<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>, InspectErrFn<F>> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#43-48)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [InspectErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.InspectErr.html "struct futures_util::stream::try_stream::InspectErr")<St, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[InspectErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.InspectErr.html "struct futures_util::stream::try_stream::InspectErr")<St, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#43-48)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#36-41)

### impl<St, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [InspectOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.InspectOk.html "struct futures_util::stream::try_stream::InspectOk")<St, F>

where [Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Inspect.html "struct futures_util::stream::stream::Inspect")<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>, InspectOkFn<F>>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#36-41)

#### type [Item](#associatedtype.Item) = <[Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Inspect.html "struct futures_util::stream::stream::Inspect")<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>, InspectOkFn<F>> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#36-41)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [InspectOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.InspectOk.html "struct futures_util::stream::try_stream::InspectOk")<St, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[InspectOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.InspectOk.html "struct futures_util::stream::try_stream::InspectOk")<St, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#36-41)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/map.rs.html#49-52)

### impl<St, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<St, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: FnMut1<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/map.rs.html#54)

#### type [Item](#associatedtype.Item) = <F as FnOnce1<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>::Output

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/map.rs.html#56)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<St, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<St, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/map.rs.html#62)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#61-66)

### impl<St, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapErr.html "struct futures_util::stream::try_stream::MapErr")<St, F>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>, MapErrFn<F>>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#61-66)

#### type [Item](#associatedtype.Item) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>, MapErrFn<F>> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#61-66)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapErr.html "struct futures_util::stream::try_stream::MapErr")<St, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapErr.html "struct futures_util::stream::try_stream::MapErr")<St, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#61-66)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#54-59)

### impl<St, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapOk.html "struct futures_util::stream::try_stream::MapOk")<St, F>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>, MapOkFn<F>>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#54-59)

#### type [Item](#associatedtype.Item) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>, MapOkFn<F>> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#54-59)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapOk.html "struct futures_util::stream::try_stream::MapOk")<St, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/struct.MapOk.html "struct futures_util::stream::try_stream::MapOk")<St, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/mod.rs.html#54-59)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/filter_map.rs.html#61-65)

### impl<St, Fut, F, T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [FilterMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/filter_map/struct.FilterMap.html "struct futures_util::stream::stream::filter_map::FilterMap")<St, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: FnMut1<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"), Output = Fut>, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/filter_map.rs.html#67)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/filter_map.rs.html#69)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [FilterMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/filter_map/struct.FilterMap.html "struct futures_util::stream::stream::filter_map::FilterMap")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/filter_map.rs.html#89)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_filter_map.rs.html#56-60)

### impl<St, Fut, F, T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryFilterMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_filter_map/struct.TryFilterMap.html "struct futures_util::stream::try_stream::try_filter_map::TryFilterMap")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>, Error = <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_filter_map.rs.html#62)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_filter_map.rs.html#64)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryFilterMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_filter_map/struct.TryFilterMap.html "struct futures_util::stream::try_stream::try_filter_map::TryFilterMap")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryFilterMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_filter_map/struct.TryFilterMap.html "struct futures_util::stream::try_stream::try_filter_map::TryFilterMap")<St, Fut, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_filter_map.rs.html#86)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/and_then.rs.html#49-53)

### impl<St, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [AndThen](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/and_then/struct.AndThen.html "struct futures_util::stream::try_stream::and_then::AndThen")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/and_then.rs.html#55)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/and_then.rs.html#57)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [AndThen](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/and_then/struct.AndThen.html "struct futures_util::stream::try_stream::and_then::AndThen")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[AndThen](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/and_then/struct.AndThen.html "struct futures_util::stream::try_stream::and_then::AndThen")<St, Fut, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/and_then.rs.html#73)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/filter.rs.html#68-72)

### impl<St, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Filter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/filter/struct.Filter.html "struct futures_util::stream::stream::filter::Filter")<St, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: for<'a> FnMut1<&'a <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"), Output = Fut>, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/filter.rs.html#74)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/filter.rs.html#76)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Filter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/filter/struct.Filter.html "struct futures_util::stream::stream::filter::Filter")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/filter.rs.html#95)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/or_else.rs.html#49-53)

### impl<St, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [OrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/or_else/struct.OrElse.html "struct futures_util::stream::try_stream::or_else::OrElse")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/or_else.rs.html#55)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/or_else.rs.html#57)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [OrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/or_else/struct.OrElse.html "struct futures_util::stream::try_stream::or_else::OrElse")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[OrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/or_else/struct.OrElse.html "struct futures_util::stream::try_stream::or_else::OrElse")<St, Fut, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/or_else.rs.html#77)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/skip_while.rs.html#65-69)

### impl<St, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [SkipWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/skip_while/struct.SkipWhile.html "struct futures_util::stream::stream::skip_while::SkipWhile")<St, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/skip_while.rs.html#71)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/skip_while.rs.html#73)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [SkipWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/skip_while/struct.SkipWhile.html "struct futures_util::stream::stream::skip_while::SkipWhile")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/skip_while.rs.html#98)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take_while.rs.html#54-58)

### impl<St, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TakeWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/take_while/struct.TakeWhile.html "struct futures_util::stream::stream::take_while::TakeWhile")<St, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take_while.rs.html#60)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take_while.rs.html#62)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TakeWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/take_while/struct.TakeWhile.html "struct futures_util::stream::stream::take_while::TakeWhile")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take_while.rs.html#89)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/then.rs.html#56-60)

### impl<St, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/then/struct.Then.html "struct futures_util::stream::stream::then::Then")<St, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/then.rs.html#62)

#### type [Item](#associatedtype.Item) = <Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/then.rs.html#64)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/then/struct.Then.html "struct futures_util::stream::stream::then::Then")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/then/struct.Then.html "struct futures_util::stream::stream::then::Then")<St, Fut, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/then.rs.html#80)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_filter.rs.html#64-68)

### impl<St, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryFilter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_filter/struct.TryFilter.html "struct futures_util::stream::try_stream::try_filter::TryFilter")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>, F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_filter.rs.html#70)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_filter.rs.html#72)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryFilter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_filter/struct.TryFilter.html "struct futures_util::stream::try_stream::try_filter::TryFilter")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryFilter](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_filter/struct.TryFilter.html "struct futures_util::stream::try_stream::try_filter::TryFilter")<St, Fut, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_filter.rs.html#92)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_skip_while.rs.html#55-59)

### impl<St, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TrySkipWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_skip_while/struct.TrySkipWhile.html "struct futures_util::stream::try_stream::try_skip_while::TrySkipWhile")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), Error = <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_skip_while.rs.html#61)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_skip_while.rs.html#63)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TrySkipWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_skip_while/struct.TrySkipWhile.html "struct futures_util::stream::try_stream::try_skip_while::TrySkipWhile")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TrySkipWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_skip_while/struct.TrySkipWhile.html "struct futures_util::stream::try_stream::try_skip_while::TrySkipWhile")<St, Fut, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_skip_while.rs.html#89)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_take_while.rs.html#58-62)

### impl<St, Fut, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryTakeWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_take_while/struct.TryTakeWhile.html "struct futures_util::stream::try_stream::try_take_while::TryTakeWhile")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), Error = <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_take_while.rs.html#64)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_take_while.rs.html#66)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryTakeWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_take_while/struct.TryTakeWhile.html "struct futures_util::stream::try_stream::try_take_while::TryTakeWhile")<St, Fut, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryTakeWhile](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_take_while/struct.TryTakeWhile.html "struct futures_util::stream::try_stream::try_take_while::TryTakeWhile")<St, Fut, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_take_while.rs.html#94)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take_until.rs.html#111-114)

### impl<St, Fut> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TakeUntil](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/take_until/struct.TakeUntil.html "struct futures_util::stream::stream::take_until::TakeUntil")<St, Fut>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take_until.rs.html#116)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take_until.rs.html#118)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TakeUntil](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/take_until/struct.TakeUntil.html "struct futures_util::stream::stream::take_until::TakeUntil")<St, Fut>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take_until.rs.html#141)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#121-126)

### impl<St, U, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [FlatMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.FlatMap.html "struct futures_util::stream::stream::FlatMap")<St, U, F>

where Flatten<[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<St, F>, U>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#121-126)

#### type [Item](#associatedtype.Item) = <Flatten<[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<St, F>, U> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#121-126)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [FlatMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.FlatMap.html "struct futures_util::stream::stream::FlatMap")<St, U, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[FlatMap](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.FlatMap.html "struct futures_util::stream::stream::FlatMap")<St, U, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#121-126)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#211-217)

### impl<St, U, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [FlatMapUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.FlatMapUnordered.html "struct futures_util::stream::stream::FlatMapUnordered")<St, U, F>

where FlattenUnorderedWithFlowController<[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<St, F>, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> U,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#211-217)

#### type [Item](#associatedtype.Item) = <FlattenUnorderedWithFlowController<[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/map/struct.Map.html "struct futures_util::stream::stream::map::Map")<St, F>, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#211-217)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [FlatMapUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.FlatMapUnordered.html "struct futures_util::stream::stream::FlatMapUnordered")<St, U, F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[FlatMapUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.FlatMapUnordered.html "struct futures_util::stream::stream::FlatMapUnordered")<St, U, F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#211-217)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/abortable.rs.html#174-176)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Abortable](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/abortable/struct.Abortable.html "struct futures_util::abortable::Abortable")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/abortable.rs.html#178)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/abortable.rs.html#180)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Abortable](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/abortable/struct.Abortable.html "struct futures_util::abortable::Abortable")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Abortable](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/abortable/struct.Abortable.html "struct futures_util::abortable::Abortable")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/buffer_unordered.rs.html#55-58)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [BufferUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/buffer_unordered/struct.BufferUnordered.html "struct futures_util::stream::stream::buffer_unordered::BufferUnordered")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/buffer_unordered.rs.html#60)

#### type [Item](#associatedtype.Item) = <<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item") as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/buffer_unordered.rs.html#62)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [BufferUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/buffer_unordered/struct.BufferUnordered.html "struct futures_util::stream::stream::buffer_unordered::BufferUnordered")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[BufferUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/buffer_unordered/struct.BufferUnordered.html "struct futures_util::stream::stream::buffer_unordered::BufferUnordered")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/buffer_unordered.rs.html#88)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/buffered.rs.html#53-56)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Buffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/buffered/struct.Buffered.html "struct futures_util::stream::stream::buffered::Buffered")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/buffered.rs.html#58)

#### type [Item](#associatedtype.Item) = <<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item") as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/buffered.rs.html#60)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Buffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/buffered/struct.Buffered.html "struct futures_util::stream::stream::buffered::Buffered")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Buffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/buffered/struct.Buffered.html "struct futures_util::stream::stream::buffered::Buffered")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/buffered.rs.html#86)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/catch_unwind.rs.html#28)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [CatchUnwind](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/catch_unwind/struct.CatchUnwind.html "struct futures_util::stream::stream::catch_unwind::CatchUnwind")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/catch_unwind.rs.html#29)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"), [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/catch_unwind.rs.html#31)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [CatchUnwind](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/catch_unwind/struct.CatchUnwind.html "struct futures_util::stream::stream::catch_unwind::CatchUnwind")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[CatchUnwind](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/catch_unwind/struct.CatchUnwind.html "struct futures_util::stream::stream::catch_unwind::CatchUnwind")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/catch_unwind.rs.html#49)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/chunks.rs.html#43)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Chunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/chunks/struct.Chunks.html "struct futures_util::stream::stream::chunks::Chunks")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/chunks.rs.html#44)

#### type [Item](#associatedtype.Item) = [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/chunks.rs.html#46)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Chunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/chunks/struct.Chunks.html "struct futures_util::stream::stream::chunks::Chunks")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Chunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/chunks/struct.Chunks.html "struct futures_util::stream::stream::chunks::Chunks")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/chunks.rs.html#76)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/cycle.rs.html#27-29)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Cycle](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/cycle/struct.Cycle.html "struct futures_util::stream::stream::cycle::Cycle")<St>

where St: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/cycle.rs.html#31)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/cycle.rs.html#33)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Cycle](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/cycle/struct.Cycle.html "struct futures_util::stream::stream::cycle::Cycle")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Cycle](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/cycle/struct.Cycle.html "struct futures_util::stream::stream::cycle::Cycle")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/cycle.rs.html#45)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/enumerate.rs.html#34)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Enumerate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/enumerate/struct.Enumerate.html "struct futures_util::stream::stream::enumerate::Enumerate")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/enumerate.rs.html#35)

#### type [Item](#associatedtype.Item) = ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"))

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/enumerate.rs.html#37)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Enumerate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/enumerate/struct.Enumerate.html "struct futures_util::stream::stream::enumerate::Enumerate")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Enumerate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/enumerate/struct.Enumerate.html "struct futures_util::stream::stream::enumerate::Enumerate")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/enumerate.rs.html#50)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#65-71)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Flatten.html "struct futures_util::stream::stream::Flatten")<St>

where Flatten<St, <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#65-71)

#### type [Item](#associatedtype.Item) = <Flatten<St, <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#65-71)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Flatten.html "struct futures_util::stream::stream::Flatten")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/struct.Flatten.html "struct futures_util::stream::stream::Flatten")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/mod.rs.html#65-71)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/into_stream.rs.html#33)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/into_stream.rs.html#34)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/into_stream.rs.html#37)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[IntoStream](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/into_stream/struct.IntoStream.html "struct futures_util::stream::try_stream::into_stream::IntoStream")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/into_stream.rs.html#41)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/ready_chunks.rs.html#31)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [ReadyChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/ready_chunks/struct.ReadyChunks.html "struct futures_util::stream::stream::ready_chunks::ReadyChunks")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/ready_chunks.rs.html#32)

#### type [Item](#associatedtype.Item) = [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/ready_chunks.rs.html#34)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [ReadyChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/ready_chunks/struct.ReadyChunks.html "struct futures_util::stream::stream::ready_chunks::ReadyChunks")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[ReadyChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/ready_chunks/struct.ReadyChunks.html "struct futures_util::stream::stream::ready_chunks::ReadyChunks")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/ready_chunks.rs.html#71)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/select_all.rs.html#90)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [SelectAll](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/select_all/struct.SelectAll.html "struct futures_util::stream::select_all::SelectAll")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/select_all.rs.html#91)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/select_all.rs.html#93)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [SelectAll](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/select_all/struct.SelectAll.html "struct futures_util::stream::select_all::SelectAll")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[SelectAll](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/select_all/struct.SelectAll.html "struct futures_util::stream::select_all::SelectAll")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/skip.rs.html#34)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Skip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/skip/struct.Skip.html "struct futures_util::stream::stream::skip::Skip")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/skip.rs.html#35)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/skip.rs.html#37)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Skip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/skip/struct.Skip.html "struct futures_util::stream::stream::skip::Skip")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/skip.rs.html#51)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take.rs.html#29-31)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Take](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/take/struct.Take.html "struct futures_util::stream::stream::take::Take")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take.rs.html#33)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take.rs.html#35)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Take](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/take/struct.Take.html "struct futures_util::stream::stream::take::Take")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/take.rs.html#50)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_buffer_unordered.rs.html#42-45)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryBufferUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_buffer_unordered/struct.TryBufferUnordered.html "struct futures_util::stream::try_stream::try_buffer_unordered::TryBufferUnordered")<St>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_buffer_unordered.rs.html#47)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_buffer_unordered.rs.html#49)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryBufferUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_buffer_unordered/struct.TryBufferUnordered.html "struct futures_util::stream::try_stream::try_buffer_unordered::TryBufferUnordered")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryBufferUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_buffer_unordered/struct.TryBufferUnordered.html "struct futures_util::stream::try_stream::try_buffer_unordered::TryBufferUnordered")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_buffered.rs.html#43-46)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryBuffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_buffered/struct.TryBuffered.html "struct futures_util::stream::try_stream::try_buffered::TryBuffered")<St>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_buffered.rs.html#48)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_buffered.rs.html#50)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryBuffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_buffered/struct.TryBuffered.html "struct futures_util::stream::try_stream::try_buffered::TryBuffered")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryBuffered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_buffered/struct.TryBuffered.html "struct futures_util::stream::try_stream::try_buffered::TryBuffered")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_chunks.rs.html#46)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_chunks/struct.TryChunks.html "struct futures_util::stream::try_stream::try_chunks::TryChunks")<St>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_chunks.rs.html#47)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")\>, [TryChunksError](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_chunks/struct.TryChunksError.html "struct futures_util::stream::try_stream::try_chunks::TryChunksError")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_chunks.rs.html#49)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_chunks/struct.TryChunks.html "struct futures_util::stream::try_stream::try_chunks::TryChunks")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_chunks/struct.TryChunks.html "struct futures_util::stream::try_stream::try_chunks::TryChunks")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_chunks.rs.html#84)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_flatten.rs.html#48-52)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryFlatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_flatten/struct.TryFlatten.html "struct futures_util::stream::try_stream::try_flatten::TryFlatten")<St>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), <<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_flatten.rs.html#54)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_flatten.rs.html#56)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryFlatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_flatten/struct.TryFlatten.html "struct futures_util::stream::try_stream::try_flatten::TryFlatten")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryFlatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_flatten/struct.TryFlatten.html "struct futures_util::stream::try_stream::try_flatten::TryFlatten")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_flatten_unordered.rs.html#19-36)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryFlattenUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_flatten_unordered/struct.TryFlattenUnordered.html "struct futures_util::stream::try_stream::try_flatten_unordered::TryFlattenUnordered")<St>

where FlattenUnorderedWithFlowController<NestedTryStreamIntoEitherTryStream<St>, PropagateBaseStreamError<St>>: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), <<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_flatten_unordered.rs.html#19-36)

#### type [Item](#associatedtype.Item) = <FlattenUnorderedWithFlowController<NestedTryStreamIntoEitherTryStream<St>, PropagateBaseStreamError<St>> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_flatten_unordered.rs.html#19-36)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryFlattenUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_flatten_unordered/struct.TryFlattenUnordered.html "struct futures_util::stream::try_stream::try_flatten_unordered::TryFlattenUnordered")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryFlattenUnordered](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_flatten_unordered/struct.TryFlattenUnordered.html "struct futures_util::stream::try_stream::try_flatten_unordered::TryFlattenUnordered")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_flatten_unordered.rs.html#19-36)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_ready_chunks.rs.html#36)

### impl<St> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryReadyChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_ready_chunks/struct.TryReadyChunks.html "struct futures_util::stream::try_stream::try_ready_chunks::TryReadyChunks")<St>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_ready_chunks.rs.html#37)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")\>, [TryReadyChunksError](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_ready_chunks/struct.TryReadyChunksError.html "struct futures_util::stream::try_stream::try_ready_chunks::TryReadyChunksError")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_ready_chunks.rs.html#39)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryReadyChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_ready_chunks/struct.TryReadyChunks.html "struct futures_util::stream::try_stream::try_ready_chunks::TryReadyChunks")<St>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryReadyChunks](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_ready_chunks/struct.TryReadyChunks.html "struct futures_util::stream::try_stream::try_ready_chunks::TryReadyChunks")<St> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_ready_chunks.rs.html#83)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_unfold.rs.html#88-91)

### impl<T, F, Fut, Item> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [TryUnfold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_unfold/struct.TryUnfold.html "struct futures_util::stream::try_stream::try_unfold::TryUnfold")<T, F, Fut>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(Item, T)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_unfold.rs.html#93)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Item, <Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_unfold.rs.html#95)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [TryUnfold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_unfold/struct.TryUnfold.html "struct futures_util::stream::try_stream::try_unfold::TryUnfold")<T, F, Fut>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[TryUnfold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_unfold/struct.TryUnfold.html "struct futures_util::stream::try_stream::try_unfold::TryUnfold")<T, F, Fut> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/unfold.rs.html#92-95)

### impl<T, F, Fut, Item> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Unfold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/unfold/struct.Unfold.html "struct futures_util::stream::unfold::Unfold")<T, F, Fut>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(Item, T)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/unfold.rs.html#97)

#### type [Item](#associatedtype.Item) = Item

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/unfold.rs.html#99)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Unfold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/unfold/struct.Unfold.html "struct futures_util::stream::unfold::Unfold")<T, F, Fut>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Unfold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/unfold/struct.Unfold.html "struct futures_util::stream::unfold::Unfold")<T, F, Fut> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/poll_fn.rs.html#48-50)

### impl<T, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [PollFn](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/poll_fn/struct.PollFn.html "struct futures_util::stream::poll_fn::PollFn")<F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/poll_fn.rs.html#52)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/poll_fn.rs.html#54)

#### fn [poll\_next](#tymethod.poll_next)(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [PollFn](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/poll_fn/struct.PollFn.html "struct futures_util::stream::poll_fn::PollFn")<F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/poll_immediate.rs.html#72-74)

### impl<T, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [PollImmediate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/poll_immediate/struct.PollImmediate.html "struct futures_util::future::poll_immediate::PollImmediate")<F>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>,

A [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") implementation that can be polled repeatedly until the future is done. The stream will never return [Poll::Pending](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Pending "variant core::task::poll::Poll::Pending") so polling it in a tight loop is worse than using a blocking synchronous function.

```rust
use core::pin::pin;

use futures::task::Poll;
use futures::{StreamExt, future};
use future::FusedFuture;

let f = async { 1_u32 };
let f = pin!(f);
let mut r = future::poll_immediate(f);
assert_eq!(r.next().await, Some(Poll::Ready(1)));

let f = async {futures::pending!(); 42_u8};
let f = pin!(f);
let mut p = future::poll_immediate(f);
assert_eq!(p.next().await, Some(Poll::Pending));
assert!(!p.is_terminated());
assert_eq!(p.next().await, Some(Poll::Ready(42)));
assert!(p.is_terminated());
assert_eq!(p.next().await, None);
```

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/poll_immediate.rs.html#76)

#### type [Item](#associatedtype.Item) = [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<T>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/poll_immediate.rs.html#78)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [PollImmediate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/poll_immediate/struct.PollImmediate.html "struct futures_util::future::poll_immediate::PollImmediate")<F>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[PollImmediate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/poll_immediate/struct.PollImmediate.html "struct futures_util::future::poll_immediate::PollImmediate")<F> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/poll_immediate.rs.html#18-20)

### impl<T, S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [PollImmediate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/poll_immediate/struct.PollImmediate.html "struct futures_util::stream::poll_immediate::PollImmediate")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = T>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/poll_immediate.rs.html#22)

#### type [Item](#associatedtype.Item) = [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<T>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/poll_immediate.rs.html#24)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [PollImmediate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/poll_immediate/struct.PollImmediate.html "struct futures_util::stream::poll_immediate::PollImmediate")<S>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[PollImmediate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/poll_immediate/struct.PollImmediate.html "struct futures_util::stream::poll_immediate::PollImmediate")<S> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/poll_immediate.rs.html#42)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/empty.rs.html#29)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Empty](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/empty/struct.Empty.html "struct futures_util::stream::empty::Empty")<T>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/empty.rs.html#30)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/empty.rs.html#32)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Empty](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/empty/struct.Empty.html "struct futures_util::stream::empty::Empty")<T>>, \_: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Empty](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/empty/struct.Empty.html "struct futures_util::stream::empty::Empty")<T> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/empty.rs.html#36)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/pending.rs.html#29)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Pending](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/pending/struct.Pending.html "struct futures_util::stream::pending::Pending")<T>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/pending.rs.html#30)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/pending.rs.html#32)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pending](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/pending/struct.Pending.html "struct futures_util::stream::pending::Pending")<T>>, \_: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Pending](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/pending/struct.Pending.html "struct futures_util::stream::pending::Pending")<T> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/pending.rs.html#36)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/src/async_channel/lib.rs.html#921)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Receiver](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/async_channel/struct.Receiver.html "struct async_channel::Receiver")<T>

[Source](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/src/async_channel/lib.rs.html#922)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/src/async_channel/lib.rs.html#924)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Receiver](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/async_channel/struct.Receiver.html "struct async_channel::Receiver")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Receiver](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/async_channel/struct.Receiver.html "struct async_channel::Receiver")<T> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1126)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Receiver](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/futures_channel/mpsc/struct.Receiver.html "struct futures_channel::mpsc::Receiver")<T>

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1127)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1129)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Receiver](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/futures_channel/mpsc/struct.Receiver.html "struct futures_channel::mpsc::Receiver")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1149)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/src/async_broadcast/lib.rs.html#1513)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Receiver](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/async_broadcast/struct.Receiver.html "struct async_broadcast::Receiver")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/src/async_broadcast/lib.rs.html#1514)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/src/async_broadcast/lib.rs.html#1516)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Receiver](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/async_broadcast/struct.Receiver.html "struct async_broadcast::Receiver")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Receiver](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/async_broadcast/struct.Receiver.html "struct async_broadcast::Receiver")<T> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/repeat.rs.html#36-38)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Repeat](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/repeat/struct.Repeat.html "struct futures_util::stream::repeat::Repeat")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/repeat.rs.html#40)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/repeat.rs.html#42)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Repeat](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/repeat/struct.Repeat.html "struct futures_util::stream::repeat::Repeat")<T>>, \_: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[Repeat](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/repeat/struct.Repeat.html "struct futures_util::stream::repeat::Repeat")<T> as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/repeat.rs.html#46)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#737-739)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>

where T: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, <T as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#741)

#### type [Item](#associatedtype.Item) = <T as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#743)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>>

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1320)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [UnboundedReceiver](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/futures_channel/mpsc/struct.UnboundedReceiver.html "struct futures_channel::mpsc::UnboundedReceiver")<T>

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1321)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1323)

#### fn [poll\_next](#tymethod.poll_next)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [UnboundedReceiver](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/futures_channel/mpsc/struct.UnboundedReceiver.html "struct futures_channel::mpsc::UnboundedReceiver")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1343)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/memory.rs.html#235)

### impl [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [DirStream](../../../asset/io/memory/struct.DirStream.html "struct bevy::asset::io::memory::DirStream")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/memory.rs.html#236)

#### type [Item](#associatedtype.Item) = [PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3001-3004)

### impl<'a, S, T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Cloned](struct.Cloned.html "struct bevy::tasks::futures_lite::stream::Cloned")<S>

where T: 'a + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3006)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3025-3028)

### impl<'a, S, T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Copied](struct.Copied.html "struct bevy::tasks::futures_lite::stream::Copied")<S>

where T: 'a + [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"), S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3030)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3405)

### impl<A, B> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Zip](struct.Zip.html "struct bevy::tasks::futures_lite::stream::Zip")<A, B>

where A: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), B: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3406)

#### type [Item](#associatedtype.Item) = (<A as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"), <B as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"))

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2836-2839)

### impl<B, S, P> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [MapWhile](struct.MapWhile.html "struct bevy::tasks::futures_lite::stream::MapWhile")<S, P>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2841)

#### type [Item](#associatedtype.Item) = B

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#646)

### impl<F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [OnceFuture](struct.OnceFuture.html "struct bevy::tasks::futures_lite::stream::OnceFuture")<F>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#647)

#### type [Item](#associatedtype.Item) = <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#204)

### impl<I> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Iter](struct.Iter.html "struct bevy::tasks::futures_lite::stream::Iter")<I>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#205)

#### type [Item](#associatedtype.Item) = <I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2554)

### impl<R> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Bytes](../io/struct.Bytes.html "struct bevy::tasks::futures_lite::io::Bytes")<R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2555)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1875)

### impl<R> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Lines](../io/struct.Lines.html "struct bevy::tasks::futures_lite::io::Lines")<R>

where R: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1876)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](../../../prelude/struct.String.html "struct bevy::prelude::String"), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1940)

### impl<R> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Split](../io/struct.Split.html "struct bevy::tasks::futures_lite::io::Split")<R>

where R: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1941)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>, [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2502-2506)

### impl<S, F, Fut> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Then](struct.Then.html "struct bevy::tasks::futures_lite::stream::Then")<S, F, Fut>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2508)

#### type [Item](#associatedtype.Item) = <Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2739-2742)

### impl<S, F, T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[FilterMap](struct.FilterMap.html "struct bevy::tasks::futures_lite::stream::FilterMap")<S, F>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2744)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2396-2399)

### impl<S, F, T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Map](struct.Map.html "struct bevy::tasks::futures_lite::stream::Map")<S, F>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> T,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2401)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3110-3113)

### impl<S, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Inspect](struct.Inspect.html "struct bevy::tasks::futures_lite::stream::Inspect")<S, F>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3115)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2546-2549)

### impl<S, P> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Filter](struct.Filter.html "struct bevy::tasks::futures_lite::stream::Filter")<S, P>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2551)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2892-2895)

### impl<S, P> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[SkipWhile](struct.SkipWhile.html "struct bevy::tasks::futures_lite::stream::SkipWhile")<S, P>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2897)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2802-2805)

### impl<S, P> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[TakeWhile](struct.TakeWhile.html "struct bevy::tasks::futures_lite::stream::TakeWhile")<S, P>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2807)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2338-2341)

### impl<S, St, F, B> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Scan](struct.Scan.html "struct bevy::tasks::futures_lite::stream::Scan")<S, St, F>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut St](https://doc.rust-lang.org/nightly/std/primitive.reference.html), <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2343)

#### type [Item](#associatedtype.Item) = B

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2426-2430)

### impl<S, U, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[FlatMap](struct.FlatMap.html "struct bevy::tasks::futures_lite::stream::FlatMap")<S, U, F>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> U,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2432)

#### type [Item](#associatedtype.Item) = <U as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2963)

### impl<S, U> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Chain](struct.Chain.html "struct bevy::tasks::futures_lite::stream::Chain")<S, U>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2964)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2464-2467)

### impl<S, U> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Flatten](struct.Flatten.html "struct bevy::tasks::futures_lite::stream::Flatten")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = U>, U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2469)

#### type [Item](#associatedtype.Item) = <U as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#215)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/src/futures_core/stream.rs.html#216)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3050-3052)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Cycle](struct.Cycle.html "struct bevy::tasks::futures_lite::stream::Cycle")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3054)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3528)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Drain](struct.Drain.html "struct bevy::tasks::futures_lite::stream::Drain")<'\_, S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3529)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3079-3081)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Enumerate](struct.Enumerate.html "struct bevy::tasks::futures_lite::stream::Enumerate")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3083)

#### type [Item](#associatedtype.Item) = ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"))

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2367)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Fuse](struct.Fuse.html "struct bevy::tasks::futures_lite::stream::Fuse")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2368)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2864)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Skip](struct.Skip.html "struct bevy::tasks::futures_lite::stream::Skip")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2865)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2930)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [StepBy](struct.StepBy.html "struct bevy::tasks::futures_lite::stream::StepBy")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2931)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2772)

### impl<S> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Take](struct.Take.html "struct bevy::tasks::futures_lite::stream::Take")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2773)

#### type [Item](#associatedtype.Item) = <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#792-795)

### impl<St, Fut> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [StopAfterFuture](struct.StopAfterFuture.html "struct bevy::tasks::futures_lite::stream::StopAfterFuture")<St, Fut>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#797)

#### type [Item](#associatedtype.Item) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#580-583)

### impl<T, E, F, Fut, Item> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[TryUnfold](struct.TryUnfold.html "struct bevy::tasks::futures_lite::stream::TryUnfold")<T, F, Fut>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(Item, T)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, E>>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#585)

#### type [Item](#associatedtype.Item) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Item, E>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#491-494)

### impl<T, F, Fut, Item> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")<T, F, Fut>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(Item, T)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#496)

#### type [Item](#associatedtype.Item) = Item

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#337-339)

### impl<T, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[PollFn](struct.PollFn.html "struct bevy::tasks::futures_lite::stream::PollFn")<F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#341)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#417-419)

### impl<T, F> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[RepeatWith](struct.RepeatWith.html "struct bevy::tasks::futures_lite::stream::RepeatWith")<F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")() -> T,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#421)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2608-2611)

### impl<T, S1, S2> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Or](struct.Or.html "struct bevy::tasks::futures_lite::stream::Or")<S1, S2>

where S1: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = T>, S2: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = T>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2613)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2699-2702)

### impl<T, S1, S2> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for [Race](struct.Race.html "struct bevy::tasks::futures_lite::stream::Race")<S1, S2>

where S1: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = T>, S2: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = T>,

Available on **crate feature `race`** only.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2704)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#162)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Empty](struct.Empty.html "struct bevy::tasks::futures_lite::stream::Empty")<T>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#163)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#243)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Once](struct.Once.html "struct bevy::tasks::futures_lite::stream::Once")<T>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#244)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#287)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Pending](struct.Pending.html "struct bevy::tasks::futures_lite::stream::Pending")<T>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#288)

#### type [Item](#associatedtype.Item) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#375)

### impl<T> [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") for bevy::tasks::futures\_lite::stream::[Repeat](struct.Repeat.html "struct bevy::tasks::futures_lite::stream::Repeat")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#376)

#### type [Item](#associatedtype.Item) = T