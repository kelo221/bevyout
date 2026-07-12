[bevy](../../index.html)::[log](../index.html)::[tracing](index.html)

# Trait Subscriber 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#80)

```rust
pub trait Subscriber: 'static {
    // Required methods
    fn enabled(&self, metadata: &Metadata<'_>) -> bool;
    fn new_span(&self, span: &Attributes<'_>) -> Id;
    fn record(&self, span: &Id, values: &Record<'_>);
    fn record_follows_from(&self, span: &Id, follows: &Id);
    fn event(&self, event: &Event<'_>);
    fn enter(&self, span: &Id);
    fn exit(&self, span: &Id);

    // Provided methods
    fn on_register_dispatch(&self, subscriber: &Dispatch) { ... }
    fn register_callsite(
        &self,
        metadata: &'static Metadata<'static>,
    ) -> Interest { ... }
    fn max_level_hint(&self) -> Option<LevelFilter> { ... }
    fn event_enabled(&self, event: &Event<'_>) -> bool { ... }
    fn clone_span(&self, id: &Id) -> Id { ... }
    fn drop_span(&self, _id: Id) { ... }
    fn try_close(&self, id: Id) -> bool { ... }
    fn current_span(&self) -> Current { ... }
    unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()> { ... }
}
```

Trait representing the functions required to collect trace data.

Crates that provide implementations of methods for collecting or recording trace data should implement the `Subscriber` interface. This trait is intended to represent fundamental primitives for collecting trace events and spans — other libraries may offer utility functions and types to make subscriber implementations more modular or improve the ergonomics of writing subscribers.

A subscriber is responsible for the following:

*   Registering new spans as they are created, and providing them with span IDs. Implicitly, this means the subscriber may determine the strategy for determining span equality.
*   Recording the attachment of field values and follows-from annotations to spans.
*   Filtering spans and events, and determining when those filters must be invalidated.
*   Observing spans as they are entered, exited, and closed, and events as they occur.

When a span is entered or exited, the subscriber is provided only with the [ID](struct.Id.html "struct bevy::log::tracing::Id") with which it tagged that span when it was created. This means that it is up to the subscriber to determine whether and how span _data_ — the fields and metadata describing the span — should be stored. The [`new_span`](trait.Subscriber.html#tymethod.new_span "method bevy::log::tracing::Subscriber::new_span") function is called when a new span is created, and at that point, the subscriber _may_ choose to store the associated data if it will be referenced again. However, if the data has already been recorded and will not be needed by the implementations of `enter` and `exit`, the subscriber may freely discard that data without allocating space to store it.

### Overriding default impls

Some trait methods on `Subscriber` have default implementations, either in order to reduce the surface area of implementing `Subscriber`, or for backward-compatibility reasons. However, many subscribers will likely want to override these default implementations.

The following methods are likely of interest:

*   [`register_callsite`](trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite") is called once for each callsite from which a span event may originate, and returns an [`Interest`](subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") value describing whether or not the subscriber wishes to see events or spans from that callsite. By default, it calls [`enabled`](trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled"), and returns `Interest::always()` if `enabled` returns true, or `Interest::never()` if enabled returns false. However, if the subscriber’s interest can change dynamically at runtime, it may want to override this function to return `Interest::sometimes()`. Additionally, subscribers which wish to perform a behaviour once for each callsite, such as allocating storage for data related to that callsite, can perform it in `register_callsite`.
    
    See also the [documentation on the callsite registry](callsite/index.html#registering-callsites "mod bevy::log::tracing::callsite") for details on [`register_callsite`](trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite").
    
*   [`event_enabled`](trait.Subscriber.html#method.event_enabled "method bevy::log::tracing::Subscriber::event_enabled") is called once before every call to the [`event`](trait.Subscriber.html#tymethod.event "method bevy::log::tracing::Subscriber::event") method. This can be used to implement filtering on events once their field values are known, but before any processing is done in the `event` method.
    
*   [`clone_span`](trait.Subscriber.html#method.clone_span "method bevy::log::tracing::Subscriber::clone_span") is called every time a span ID is cloned, and [`try_close`](trait.Subscriber.html#method.try_close "method bevy::log::tracing::Subscriber::try_close") is called when a span ID is dropped. By default, these functions do nothing. However, they can be used to implement reference counting for spans, allowing subscribers to free storage for span data and to determine when a span has _closed_ permanently (rather than being exited). Subscribers which store per-span data or which need to track span closures should override these functions together.
    

## Required Methods

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#203)

#### fn [enabled](#tymethod.enabled)(&self, metadata: &[Metadata](struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if a span or event with the specified [metadata](struct.Metadata.html "struct bevy::log::tracing::Metadata") would be recorded.

By default, it is assumed that this filter needs only be evaluated once for each callsite, so it is called by [`register_callsite`](trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite") when each callsite is registered. The result is used to determine if the subscriber is always [interested](subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") or never interested in that callsite. This is intended primarily as an optimization, so that expensive filters (such as those involving string search, et cetera) need not be re-evaluated.

However, if the subscriber’s interest in a particular span or event may change, or depends on contexts only determined dynamically at runtime, then the `register_callsite` method should be overridden to return [`Interest::sometimes`](subscriber/struct.Interest.html#method.sometimes "associated function bevy::log::tracing::subscriber::Interest::sometimes"). In that case, this function will be called every time that span or event occurs.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#255)

#### fn [new\_span](#tymethod.new_span)(&self, span: &[Attributes](span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>) -> [Id](struct.Id.html "struct bevy::log::tracing::Id")

Visit the construction of a new span, returning a new [span ID](struct.Id.html "struct bevy::log::tracing::Id") for the span being constructed.

The provided [`Attributes`](span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes") contains any field values that were provided when the span was created. The subscriber may pass a [visitor](field/trait.Visit.html "trait bevy::log::tracing::field::Visit") to the `Attributes`’ [`record` method](span/struct.Attributes.html#method.record "method bevy::log::tracing::span::Attributes::record") to record these values.

IDs are used to uniquely identify spans and events within the context of a subscriber, so span equality will be based on the returned ID. Thus, if the subscriber wishes for all spans with the same metadata to be considered equal, it should return the same ID every time it is given a particular set of metadata. Similarly, if it wishes for two separate instances of a span with the same metadata to _not_ be equal, it should return a distinct ID every time this function is called, regardless of the metadata.

Note that the subscriber is free to assign span IDs based on whatever scheme it sees fit. Any guarantees about uniqueness, ordering, or ID reuse are left up to the subscriber implementation to determine.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#295)

#### fn [record](#tymethod.record)(&self, span: &[Id](struct.Id.html "struct bevy::log::tracing::Id"), values: &[Record](span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>)

Record a set of values on a span.

This method will be invoked when value is recorded on a span. Recording multiple values for the same field is possible, but the actual behaviour is defined by the subscriber implementation.

Keep in mind that a span might not provide a value for each field it declares.

The subscriber is expected to provide a [visitor](field/trait.Visit.html "trait bevy::log::tracing::field::Visit") to the `Record`’s [`record` method](span/struct.Record.html#method.record "method bevy::log::tracing::span::Record::record") in order to record the added values.

##### Example

“foo = 3” will be recorded when [`record`](span/struct.Attributes.html#method.record "method bevy::log::tracing::span::Attributes::record") is called on the `Attributes` passed to `new_span`. Since values are not provided for the `bar` and `baz` fields, the span’s `Metadata` will indicate that it _has_ those fields, but values for them won’t be recorded at this time.

[ⓘ](# "This example is not tested")

```rust
let mut span = span!("my_span", foo = 3, bar, baz);

// `Subscriber::record` will be called with a `Record`
// containing "bar = false"
span.record("bar", &false);

// `Subscriber::record` will be called with a `Record`
// containing "baz = "a string""
span.record("baz", &"a string");
```

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#315)

#### fn [record\_follows\_from](#tymethod.record_follows_from)(&self, span: &[Id](struct.Id.html "struct bevy::log::tracing::Id"), follows: &[Id](struct.Id.html "struct bevy::log::tracing::Id"))

Adds an indication that `span` follows from the span with the id `follows`.

This relationship differs somewhat from the parent-child relationship: a span may have any number of prior spans, rather than a single one; and spans are not considered to be executing _inside_ of the spans they follow from. This means that a span may close even if subsequent spans that follow from it are still open, and time spent inside of a subsequent span should not be included in the time its precedents were executing. This is used to model causal relationships such as when a single future spawns several related background tasks, et cetera.

If the subscriber has spans corresponding to the given IDs, it should record this relationship in whatever way it deems necessary. Otherwise, if one or both of the given span IDs do not correspond to spans that the subscriber knows about, or if a cyclical relationship would be created (i.e., some span _a_ which proceeds some other span _b_ may not also follow from _b_), it may silently do nothing.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#346)

#### fn [event](#tymethod.event)(&self, event: &[Event](struct.Event.html "struct bevy::log::tracing::Event")<'\_>)

Records that an [`Event`](struct.Event.html "struct bevy::log::tracing::Event") has occurred.

This method will be invoked when an Event is constructed by the `Event`’s [`dispatch` method](struct.Event.html#method.dispatch "associated function bevy::log::tracing::Event::dispatch"). For example, this happens internally when an event macro from `tracing` is called.

The key difference between this method and `record` is that `record` is called when a value is recorded for a field defined by a span, while `event` is called when a new event occurs.

The provided `Event` struct contains any field values attached to the event. The subscriber may pass a [visitor](field/trait.Visit.html "trait bevy::log::tracing::field::Visit") to the `Event`’s [`record` method](struct.Event.html#method.record "method bevy::log::tracing::Event::record") to record these values.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#356)

#### fn [enter](#tymethod.enter)(&self, span: &[Id](struct.Id.html "struct bevy::log::tracing::Id"))

Records that a span has been entered.

When entering a span, this method is called to notify the subscriber that the span has been entered. The subscriber is provided with the [span ID](struct.Id.html "struct bevy::log::tracing::Id") of the entered span, and should update any internal state tracking the current span accordingly.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#368)

#### fn [exit](#tymethod.exit)(&self, span: &[Id](struct.Id.html "struct bevy::log::tracing::Id"))

Records that a span has been exited.

When exiting a span, this method is called to notify the subscriber that the span has been exited. The subscriber is provided with the [span ID](struct.Id.html "struct bevy::log::tracing::Id") of the exited span, and should update any internal state tracking the current span accordingly.

Exiting a span does not imply that the span will not be re-entered.

## Provided Methods

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#100)

#### fn [on\_register\_dispatch](#method.on_register_dispatch)(&self, subscriber: &[Dispatch](struct.Dispatch.html "struct bevy::log::tracing::Dispatch"))

Invoked when this subscriber becomes a [`Dispatch`](struct.Dispatch.html "struct bevy::log::tracing::Dispatch").

###### Avoiding Memory Leaks

`Subscriber`s should not store their own [`Dispatch`](struct.Dispatch.html "struct bevy::log::tracing::Dispatch"). Because the `Dispatch` owns the `Subscriber`, storing the `Dispatch` within the `Subscriber` will create a reference count cycle, preventing the `Dispatch` from ever being dropped.

Instead, when it is necessary to store a cyclical reference to the `Dispatch` within a `Subscriber`, use [`Dispatch::downgrade`](struct.Dispatch.html#method.downgrade "method bevy::log::tracing::Dispatch::downgrade") to convert a `Dispatch` into a [`WeakDispatch`](dispatcher/struct.WeakDispatch.html "struct bevy::log::tracing::dispatcher::WeakDispatch"). This type is analogous to [`std::sync::Weak`](../../platform/sync/struct.Weak.html "struct bevy::platform::sync::Weak"), and does not create a reference count cycle. A [`WeakDispatch`](dispatcher/struct.WeakDispatch.html "struct bevy::log::tracing::dispatcher::WeakDispatch") can be stored within a `Subscriber` without causing a memory leak, and can be [upgraded](dispatcher/struct.WeakDispatch.html#method.upgrade "method bevy::log::tracing::dispatcher::WeakDispatch::upgrade") into a `Dispatch` temporarily when the `Dispatch` must be accessed by the `Subscriber`.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#175)

#### fn [register\_callsite](#method.register_callsite)(&self, metadata: &'static [Metadata](struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>) -> [Interest](subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")

Registers a new [callsite](callsite/index.html "mod bevy::log::tracing::callsite") with this subscriber, returning whether or not the subscriber is interested in being notified about the callsite.

By default, this function assumes that the subscriber’s [filter](trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled") represents an unchanging view of its interest in the callsite. However, if this is not the case, subscribers may override this function to indicate different interests, or to implement behaviour that should run once for every callsite.

This function is guaranteed to be called at least once per callsite on every active subscriber. The subscriber may store the keys to fields it cares about in order to reduce the cost of accessing fields by name, preallocate storage for that callsite, or perform any other actions it wishes to perform once for each callsite.

The subscriber should then return an [`Interest`](subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest"), indicating whether it is interested in being notified about that callsite in the future. This may be `Always` indicating that the subscriber always wishes to be notified about the callsite, and its filter need not be re-evaluated; `Sometimes`, indicating that the subscriber may sometimes care about the callsite but not always (such as when sampling), or `Never`, indicating that the subscriber never wishes to be notified about that callsite. If all active subscribers return `Never`, a callsite will never be enabled unless a new subscriber expresses interest in it.

`Subscriber`s which require their filters to be run every time an event occurs or a span is entered/exited should return `Interest::sometimes`. If a subscriber returns `Interest::sometimes`, then its [`enabled`](trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled") method will be called every time an event or span is created from that callsite.

For example, suppose a sampling subscriber is implemented by incrementing a counter every time `enabled` is called and only returning `true` when the counter is divisible by a specified sampling rate. If that subscriber returns `Interest::always` from `register_callsite`, then the filter will not be re-evaluated once it has been applied to a given set of metadata. Thus, the counter will not be incremented, and the span or event that corresponds to the metadata will never be `enabled`.

`Subscriber`s that need to change their filters occasionally should call [`rebuild_interest_cache`](callsite/fn.rebuild_interest_cache.html "fn bevy::log::tracing::callsite::rebuild_interest_cache") to re-evaluate `register_callsite` for all callsites.

Similarly, if a `Subscriber` has a filtering strategy that can be changed dynamically at runtime, it would need to re-evaluate that filter if the cached results have changed.

A subscriber which manages fanout to multiple other subscribers should proxy this decision to all of its child subscribers, returning `Interest::never` only if _all_ such children return `Interest::never`. If the set of subscribers to which spans are broadcast may change dynamically, the subscriber should also never return `Interest::Never`, as a new subscriber may be added that _is_ interested.

See the [documentation on the callsite registry](callsite/index.html#registering-callsites "mod bevy::log::tracing::callsite") for more details on how and when the `register_callsite` method is called.

##### Notes

This function may be called again when a new subscriber is created or when the registry is invalidated.

If a subscriber returns `Interest::never` for a particular callsite, it _may_ still see spans and events originating from that callsite, if another subscriber expressed interest in it.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#227)

#### fn [max\_level\_hint](#method.max_level_hint)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[LevelFilter](level_filters/struct.LevelFilter.html "struct bevy::log::tracing::level_filters::LevelFilter")\>

Returns the highest [verbosity level](../struct.Level.html "struct bevy::log::Level") that this `Subscriber` will enable, or `None`, if the subscriber does not implement level-based filtering or chooses not to implement this method.

If this method returns a [`Level`](../struct.Level.html "struct bevy::log::Level"), it will be used as a hint to determine the most verbose level that will be enabled. This will allow spans and events which are more verbose than that level to be skipped more efficiently. Subscribers which perform filtering are strongly encouraged to provide an implementation of this method.

If the maximum level the subscriber will enable can change over the course of its lifetime, it is free to return a different value from multiple invocations of this method. However, note that changes in the maximum level will **only** be reflected after the callsite [`Interest`](subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") cache is rebuilt, by calling the [`callsite::rebuild_interest_cache`](callsite/fn.rebuild_interest_cache.html "fn bevy::log::tracing::callsite::rebuild_interest_cache") function. Therefore, if the subscriber will change the value returned by this method, it is responsible for ensuring that [`rebuild_interest_cache`](callsite/fn.rebuild_interest_cache.html "fn bevy::log::tracing::callsite::rebuild_interest_cache") is called after the value of the max level changes.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#323)

#### fn [event\_enabled](#method.event_enabled)(&self, event: &[Event](struct.Event.html "struct bevy::log::tracing::Event")<'\_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determine if an [`Event`](struct.Event.html "struct bevy::log::tracing::Event") should be recorded.

By default, this returns `true` and `Subscriber`s can filter events in [`event`](trait.Subscriber.html#tymethod.event "method bevy::log::tracing::Subscriber::event") without any penalty. However, when `event` is more complicated, this can be used to determine if `event` should be called at all, separating out the decision from the processing.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#390)

#### fn [clone\_span](#method.clone_span)(&self, id: &[Id](struct.Id.html "struct bevy::log::tracing::Id")) -> [Id](struct.Id.html "struct bevy::log::tracing::Id")

Notifies the subscriber that a [span ID](struct.Id.html "struct bevy::log::tracing::Id") has been cloned.

This function is guaranteed to only be called with span IDs that were returned by this subscriber’s `new_span` function.

Note that the default implementation of this function this is just the identity function, passing through the identifier. However, it can be used in conjunction with [`try_close`](trait.Subscriber.html#method.try_close "method bevy::log::tracing::Subscriber::try_close") to track the number of handles capable of `enter`ing a span. When all the handles have been dropped (i.e., `try_close` has been called one more time than `clone_span` for a given ID), the subscriber may assume that the span will not be entered again. It is then free to deallocate storage for data associated with that span, write data from that span to IO, and so on.

For more unsafe situations, however, if `id` is itself a pointer of some kind this can be used as a hook to “clone” the pointer, depending on what that means for the specified pointer.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#404)

#### fn [drop\_span](#method.drop_span)(&self, \_id: [Id](struct.Id.html "struct bevy::log::tracing::Id"))

👎Deprecated since 0.1.2:

use `Subscriber::try_close` instead

**This method is deprecated.**

Using `drop_span` may result in subscribers composed using `tracing-subscriber` crate’s `Layer` trait from observing close events. Use [`try_close`](trait.Subscriber.html#method.try_close "method bevy::log::tracing::Subscriber::try_close") instead.

The default implementation of this function does nothing.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#442)

#### fn [try\_close](#method.try_close)(&self, id: [Id](struct.Id.html "struct bevy::log::tracing::Id")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Notifies the subscriber that a [span ID](struct.Id.html "struct bevy::log::tracing::Id") has been dropped, and returns `true` if there are now 0 IDs that refer to that span.

Higher-level libraries providing functionality for composing multiple subscriber implementations may use this return value to notify any “layered” subscribers that this subscriber considers the span closed.

The default implementation of this method calls the subscriber’s [`drop_span`](trait.Subscriber.html#method.drop_span "method bevy::log::tracing::Subscriber::drop_span") method and returns `false`. This means that, unless the subscriber overrides the default implementation, close notifications will never be sent to any layered subscribers. In general, if the subscriber tracks reference counts, this method should be implemented, rather than `drop_span`.

This function is guaranteed to only be called with span IDs that were returned by this subscriber’s `new_span` function.

It’s guaranteed that if this function has been called once more than the number of times `clone_span` was called with the same `id`, then no more handles that can enter the span with that `id` exist. This means that it can be used in conjunction with [`clone_span`](trait.Subscriber.html#method.clone_span "method bevy::log::tracing::Subscriber::clone_span") to track the number of handles capable of `enter`ing a span. When all the handles have been dropped (i.e., `try_close` has been called one more time than `clone_span` for a given ID), the subscriber may assume that the span will not be entered again, and should return `true`. It is then free to deallocate storage for data associated with that span, write data from that span to IO, and so on.

**Note**: since this function is called when spans are dropped, implementations should ensure that they are unwind-safe. Panicking from inside of a `try_close` function may cause a double panic, if the span was dropped due to a thread unwinding.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#461)

#### fn [current\_span](#method.current_span)(&self) -> [Current](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/span/struct.Current.html "struct tracing_core::span::Current")

Returns a type representing this subscriber’s view of the current span.

If subscribers track a current span, they should override this function to return [`Current::new`](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/span/struct.Current.html#tymethod.new "struct tracing_core::span::Current") if the thread from which this method is called is inside a span, or [`Current::none`](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/span/struct.Current.html#tymethod.none "struct tracing_core::span::Current") if the thread is not inside a span.

By default, this returns a value indicating that the subscriber does **not** track what span is current. If the subscriber does not implement a current span, it should not override this method.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#492)

#### unsafe fn [downcast\_raw](#method.downcast_raw)(&self, id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[\*const](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

If `self` is the same type as the provided `TypeId`, returns an untyped `*const` pointer to that type. Otherwise, returns `None`.

If you wish to downcast a `Subscriber`, it is strongly advised to use the safe API provided by [`downcast_ref`](#method.downcast_ref) instead.

This API is required for `downcast_raw` to be a trait method; a method signature like [`downcast_ref`](#method.downcast_ref) (with a generic type parameter) is not object-safe, and thus cannot be a trait method for `Subscriber`. This means that if we only exposed `downcast_ref`, `Subscriber` implementations could not override the downcasting behavior

This method may be overridden by “fan out” or “chained” subscriber implementations which consist of multiple composed types. Such subscribers might allow `downcast_raw` by returning references to those component if they contain components with the given `TypeId`.

##### Safety

The [`downcast_ref`](#method.downcast_ref) method expects that the pointer returned by `downcast_raw` is non-null and points to a valid instance of the type with the provided `TypeId`. Failure to ensure this will result in undefined behaviour, so implementing `downcast_raw` is unsafe.

## Implementations

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#501)

### impl dyn [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#503)

#### pub fn [is](#method.is)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns `true` if this `Subscriber` is the same type as `T`.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#509)

#### pub fn [downcast\_ref](#method.downcast_ref)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns some reference to this `Subscriber` value if it is of type `T`, or `None` if it isn’t.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#521)

### impl dyn [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#523)

#### pub fn [is](#method.is-1)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns `true` if this [`Subscriber`](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") is the same type as `T`.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#529)

#### pub fn [downcast\_ref](#method.downcast_ref-1)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns some reference to this [`Subscriber`](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") value if it is of type `T`, or `None` if it isn’t.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#541)

### impl dyn [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#543)

#### pub fn [is](#method.is-2)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns `true` if this [`Subscriber`](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") is the same type as `T`.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#549)

#### pub fn [downcast\_ref](#method.downcast_ref-2)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns some reference to this `[`Subscriber`] value if it is of type` T`, or` None\` if it isn’t.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#561)

### impl dyn [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#563)

#### pub fn [is](#method.is-3)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns `true` if this [`Subscriber`](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") is the same type as `T`.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#569)

#### pub fn [downcast\_ref](#method.downcast_ref-3)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns some reference to this [`Subscriber`](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") value if it is of type `T`, or `None` if it isn’t.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#674)

### impl [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") for [NoSubscriber](subscriber/struct.NoSubscriber.html "struct bevy::log::tracing::subscriber::NoSubscriber")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/sharded.rs.html#221)

### impl [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") for [Registry](../tracing_subscriber/struct.Registry.html "struct bevy::log::tracing_subscriber::Registry")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/layered.rs.html#89-92)

### impl<L, S> [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") for [Layered](../tracing_subscriber/layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<L, S>

where L: [Layer](../tracing_subscriber/trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S>, S: [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#370-377)

### impl<N, E, F, W> [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") for [Subscriber](../tracing_subscriber/struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber")<N, E, F, W>

where N: for<'writer> [FormatFields](../tracing_subscriber/fmt/trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'writer> + 'static, E: [FormatEvent](../tracing_subscriber/fmt/trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent")<[Registry](../tracing_subscriber/struct.Registry.html "struct bevy::log::tracing_subscriber::Registry"), N> + 'static, F: [Layer](../tracing_subscriber/trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<[Layered](../tracing_subscriber/layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<[Layer](../tracing_subscriber/fmt/struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer")<[Registry](../tracing_subscriber/struct.Registry.html "struct bevy::log::tracing_subscriber::Registry"), N, E, W>, [Registry](../tracing_subscriber/struct.Registry.html "struct bevy::log::tracing_subscriber::Registry")\>> + 'static, W: for<'writer> [MakeWriter](../tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'writer> + 'static, [Layered](../tracing_subscriber/layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<F, [Layered](../tracing_subscriber/layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<[Layer](../tracing_subscriber/fmt/struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer")<[Registry](../tracing_subscriber/struct.Registry.html "struct bevy::log::tracing_subscriber::Registry"), N, E, W>, [Registry](../tracing_subscriber/struct.Registry.html "struct bevy::log::tracing_subscriber::Registry")\>>: [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber"), [Layer](../tracing_subscriber/fmt/struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer")<[Registry](../tracing_subscriber/struct.Registry.html "struct bevy::log::tracing_subscriber::Registry"), N, E, W>: [Layer](../tracing_subscriber/trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<[Registry](../tracing_subscriber/struct.Registry.html "struct bevy::log::tracing_subscriber::Registry")\>,

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#792-794)

### impl<S> [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") for [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<S>

where S: [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/subscriber.rs.html#707-709)

### impl<S> [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<S>

where S: [Subscriber](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),