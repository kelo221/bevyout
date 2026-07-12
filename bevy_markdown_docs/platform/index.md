[bevy](../index.html)

# Crate platform 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/lib.rs.html#1-67)

Platform compatibility support for first-party [Bevy](https://bevy.org/) engine crates.

## Modules

[cell](cell/index.html "mod bevy::platform::cell")

Provides cell primitives.

[cfg](cfg/index.html "mod bevy::platform::cfg")

Provides helpful configuration macros, allowing detection of platform features such as [`alloc`](crate::cfg::alloc) or [`std`](crate::cfg::std) without explicit features.

[collections](collections/index.html "mod bevy::platform::collections")

Provides [`HashMap`](collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") and [`HashSet`](collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") from [`hashbrown`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/index.html "mod hashbrown") with some customized defaults.

[dirs](dirs/index.html "mod bevy::platform::dirs")

APIs that return the location of standard user directories.

[future](future/index.html "mod bevy::platform::future")

Platform-aware future utilities.

[hash](hash/index.html "mod bevy::platform::hash")

Provides replacements for `std::hash` items using [`foldhash`](https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/index.html "mod foldhash").

[prelude](prelude/index.html "mod bevy::platform::prelude")

Frequently used items which would typically be included in most contexts.

[sync](sync/index.html "mod bevy::platform::sync")

Provides various synchronization alternatives to language primitives.

[thread](thread/index.html "mod bevy::platform::thread")

Provides `sleep` for all platforms.

[time](time/index.html "mod bevy::platform::time")

Provides `Instant` for all platforms.