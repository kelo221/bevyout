[bevy](../index.html)::[asset](index.html)

# Function publish\_asset\_server\_diagnostics 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#2220-2223)

```rust
pub fn publish_asset_server_diagnostics(
    asset_server: Res<'_, AssetServer>,
    diagnostics: Diagnostics<'_, '_>,
)
```

A system publishing asset server statistics to [`bevy_diagnostic`](../diagnostic/index.html "mod bevy::diagnostic").