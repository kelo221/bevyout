[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)

# Module callsite 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/lib.rs.html#283)

Callsites represent the source locations from which spans or events originate.

## What Are Callsites?

Every span or event in `tracing` is associated with a [`Callsite`](../trait.Callsite.html "trait bevy::log::tracing::Callsite"). A callsite is a small `static` value that is responsible for the following:

*   Storing the span or event’s [`Metadata`](../struct.Metadata.html "struct bevy::log::tracing::Metadata"),
*   Uniquely [identifying](struct.Identifier.html "struct bevy::log::tracing::callsite::Identifier") the span or event definition,
*   Caching the subscriber’s [`Interest`](../subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")[1](#fn1) in that span or event, to avoid re-evaluating filters.

## Registering Callsites

When a span or event is recorded for the first time, its callsite [`register`](fn.register.html "fn bevy::log::tracing::callsite::register")s itself with the global callsite registry. Registering a callsite calls the [`Subscriber::register_callsite`](../trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite") method with that callsite’s [`Metadata`](../struct.Metadata.html "struct bevy::log::tracing::Metadata") on every currently active subscriber. This serves two primary purposes: informing subscribers of the callsite’s existence, and performing static filtering.

### Callsite Existence

If a [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") implementation wishes to allocate storage for each unique span/event location in the program, or pre-compute some value that will be used to record that span or event in the future, it can do so in its [`register_callsite`](../trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite") method.

### Performing Static Filtering

The [`register_callsite`](../trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite") method returns an [`Interest`](../subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") value, which indicates that the subscriber either [always](../subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always") wishes to record that span or event, [sometimes](../subscriber/struct.Interest.html#method.sometimes "associated function bevy::log::tracing::subscriber::Interest::sometimes") wishes to record it based on a dynamic filter evaluation, or [never](../subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") wishes to record it.

When registering a new callsite, the [`Interest`](../subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")s returned by every currently active subscriber are combined, and the result is stored at each callsite. This way, when the span or event occurs in the future, the cached [`Interest`](../subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") value can be checked efficiently to determine if the span or event should be recorded, without needing to perform expensive filtering (i.e. calling the [`Subscriber::enabled`](../trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled") method every time a span or event occurs).

#### Rebuilding Cached Interest

When a new [`Dispatch`](../struct.Dispatch.html "struct bevy::log::tracing::Dispatch") is created (i.e. a new subscriber becomes active), any previously cached [`Interest`](../subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") values are re-evaluated for all callsites in the program. This way, if the new subscriber will enable a callsite that was not previously enabled, the [`Interest`](../subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") in that callsite is updated. Similarly, when a subscriber is dropped, the interest cache is also re-evaluated, so that any callsites enabled only by that subscriber are disabled.

In addition, the [`rebuild_interest_cache`](fn.rebuild_interest_cache.html "fn bevy::log::tracing::callsite::rebuild_interest_cache") function in this module can be used to manually invalidate all cached interest and re-register those callsites. This function is useful in situations where a subscriber’s interest can change, but it does so relatively infrequently. The subscriber may wish for its interest to be cached most of the time, and return [`Interest::always`](../subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always") or [`Interest::never`](../subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") in its [`register_callsite`](../trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite") method, so that its [`Subscriber::enabled`](../trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled") method doesn’t need to be evaluated every time a span or event is recorded. However, when the configuration changes, the subscriber can call [`rebuild_interest_cache`](fn.rebuild_interest_cache.html "fn bevy::log::tracing::callsite::rebuild_interest_cache") to re-evaluate the entire interest cache with its new configuration. This is a relatively costly operation, but if the configuration changes infrequently, it may be more efficient than calling [`Subscriber::enabled`](../trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled") frequently.

## Implementing Callsites

In most cases, instrumenting code using `tracing` should _not_ require implementing the [`Callsite`](../trait.Callsite.html "trait bevy::log::tracing::Callsite") trait directly. When using the [`tracing` crate’s macros](https://docs.rs/tracing/latest/tracing/#macros) or the [`#[instrument]` attribute](https://docs.rs/tracing/latest/tracing/attr.instrument.html), a `Callsite` is automatically generated.

However, code which provides alternative forms of `tracing` instrumentation may need to interact with the callsite system directly. If instrumentation-side code needs to produce a `Callsite` to emit spans or events, the [`DefaultCallsite`](struct.DefaultCallsite.html "struct bevy::log::tracing::callsite::DefaultCallsite") struct provided in this module is a ready-made `Callsite` implementation that is suitable for most uses. When possible, the use of `DefaultCallsite` should be preferred over implementing [`Callsite`](../trait.Callsite.html "trait bevy::log::tracing::Callsite") for user types, as `DefaultCallsite` may benefit from additional performance optimizations.

* * *

1.  Returned by the [`Subscriber::register_callsite`](../trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite") method. [↩](#fnref1)
    

## Structs

[DefaultCallsite](struct.DefaultCallsite.html "struct bevy::log::tracing::callsite::DefaultCallsite")

A default [`Callsite`](../trait.Callsite.html "trait bevy::log::tracing::Callsite") implementation.

[Identifier](struct.Identifier.html "struct bevy::log::tracing::callsite::Identifier")

Uniquely identifies a [`Callsite`](../trait.Callsite.html "trait bevy::log::tracing::Callsite")

## Traits

[Callsite](trait.Callsite.html "trait bevy::log::tracing::callsite::Callsite")

Trait implemented by callsites.

## Functions

[rebuild\_interest\_cache](fn.rebuild_interest_cache.html "fn bevy::log::tracing::callsite::rebuild_interest_cache")

Clear and reregister interest on every [`Callsite`](../trait.Callsite.html "trait bevy::log::tracing::Callsite")

[register](fn.register.html "fn bevy::log::tracing::callsite::register")

Register a new [`Callsite`](../trait.Callsite.html "trait bevy::log::tracing::Callsite") with the global registry.