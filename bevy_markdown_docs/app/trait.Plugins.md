[bevy](../index.html)::[app](index.html)

# Trait Plugins 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#126)

```rust
pub trait Plugins<Marker>: Plugins<Marker> { }
```

Types that represent a set of [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")s.

This is implemented for all types which implement [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"), [`PluginGroup`](../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup"), and tuples over [`Plugins`](trait.Plugins.html "trait bevy::app::Plugins").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#128)

### impl<Marker, T> [Plugins](trait.Plugins.html "trait bevy::app::Plugins")<Marker> for T

where T: Plugins<Marker>,