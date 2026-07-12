[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)

# Module prelude 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/lib.rs.html#218)

The `tracing-subscriber` prelude.

This brings into scope a number of extension traits that define methods on types defined here and in other crates.

## Traits

[\_](trait._.html "trait bevy::log::tracing_subscriber::prelude::_")

Extension trait adding utility methods for subscriber initialization.

[\_](trait._.html "trait bevy::log::tracing_subscriber::prelude::_")

Extension trait adding combinators for working with types implementing [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter").

[\_\_tracing\_subscriber\_Layer](trait.__tracing_subscriber_Layer.html "trait bevy::log::tracing_subscriber::prelude::__tracing_subscriber_Layer")

A composable handler for `tracing` events.

[\_\_tracing\_subscriber\_SubscriberExt](trait.__tracing_subscriber_SubscriberExt.html "trait bevy::log::tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt")

Extension trait adding a `with(Layer)` combinator to `Subscriber`s.

[\_\_tracing\_subscriber\_field\_MakeExt](trait.__tracing_subscriber_field_MakeExt.html "trait bevy::log::tracing_subscriber::prelude::__tracing_subscriber_field_MakeExt")

Extension trait providing `MakeVisitor` combinators.

[\_\_tracing\_subscriber\_field\_RecordFields](trait.__tracing_subscriber_field_RecordFields.html "trait bevy::log::tracing_subscriber::prelude::__tracing_subscriber_field_RecordFields")

Extension trait implemented by types which can be recorded by a [visitor](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit").