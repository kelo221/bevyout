[bevy](../index.html)::[asset](index.html)

# Macro load\_embedded\_asset 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#197)

```rust
macro_rules! load_embedded_asset {
    (@get: $path: literal, $provider: expr) => { ... };
    ($provider: expr, $path: literal, $settings: expr) => { ... };
    ($provider: expr, $path: literal) => { ... };
}
```

Load an [embedded asset](macro.embedded_asset.html "macro bevy::asset::embedded_asset").

This is useful if the embedded asset in question is not publicly exposed, but you need to use it internally.

## Syntax

This macro takes two arguments and an optional third one:

1.  The asset source. It may be `AssetServer`, `World` or `App`.
2.  The path to the asset to embed, as a string literal.
3.  Optionally, a closure of the same type as in [`LoadBuilder::with_settings`](struct.LoadBuilder.html#method.with_settings "method bevy::asset::LoadBuilder::with_settings"). Consider explicitly typing the closure argument in case of type error.

## Usage

The advantage compared to using directly [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load") is:

*   This also accepts [`World`](../prelude/struct.World.html "struct bevy::prelude::World") and [`App`](../prelude/struct.App.html "struct bevy::prelude::App") arguments.
*   This uses the exact same path as `embedded_asset!`, so you can keep it consistent.

As a rule of thumb:

*   If the asset in used in the same module as it is declared using `embedded_asset!`, use this macro.
*   Otherwise, use `AssetServer::load`.