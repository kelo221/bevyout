[bevy](../index.html)::[asset](index.html)

# Macro embedded\_asset 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#344)

```rust
macro_rules! embedded_asset {
    ($app: expr, $path: expr) => { ... };
    ($app: expr, $source_path: expr, $path: expr) => { ... };
}
```

Creates a new `embedded` asset by embedding the bytes of the given path into the current binary and registering those bytes with the `embedded` [`AssetSource`](io/struct.AssetSource.html "struct bevy::asset::io::AssetSource").

This accepts the current [`App`](../prelude/struct.App.html "struct bevy::prelude::App") as the first parameter and a path `&str` (relative to the current file) as the second.

By default this will generate an [`AssetPath`](struct.AssetPath.html "struct bevy::asset::AssetPath") using the following rules:

1.  Search for the first `$crate_name/src/` in the path and trim to the path past that point.
2.  Re-add the current `$crate_name` to the front of the path

For example, consider the following file structure in the theoretical `bevy_rock` crate, which provides a Bevy [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that renders fancy rocks for scenes.

```
bevy_rock
├── src
│   ├── render
│   │   ├── rock.wgsl
│   │   └── mod.rs
│   └── lib.rs
└── Cargo.toml
```

`rock.wgsl` is a WGSL shader asset that the `bevy_rock` plugin author wants to bundle with their crate. They invoke the following in `bevy_rock/src/render/mod.rs`:

`embedded_asset!(app, "rock.wgsl")`

`rock.wgsl` can now be loaded by the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") as follows:

```rust
// If we are loading the shader in the same module we used `embedded_asset!`:
let shader = load_embedded_asset!(&asset_server, "rock.wgsl");

// If the goal is to expose the asset **to the end user**:
let shader = asset_server.load::<Shader>("embedded://bevy_rock/render/rock.wgsl");
```

Some things to note in the path:

1.  The non-default `embedded://` [`AssetSource`](io/struct.AssetSource.html "struct bevy::asset::io::AssetSource")
2.  `src` is trimmed from the path

The default behavior also works for cargo workspaces. Pretend the `bevy_rock` crate now exists in a larger workspace in `$SOME_WORKSPACE/crates/bevy_rock`. The asset path would remain the same, because [`embedded_asset`](macro.embedded_asset.html "macro bevy::asset::embedded_asset") searches for the _first instance_ of `bevy_rock/src` in the path.

For most “standard crate structures” the default works just fine. But for some niche cases (such as cargo examples), the `src` path will not be present. You can override this behavior by adding it as the second argument to [`embedded_asset`](macro.embedded_asset.html "macro bevy::asset::embedded_asset"):

`embedded_asset!(app, "/examples/rock_stuff/", "rock.wgsl")`

When there are three arguments, the second argument will replace the default `/src/` value. Note that these two are equivalent:

`embedded_asset!(app, "rock.wgsl")` `embedded_asset!(app, "/src/", "rock.wgsl")`

This macro uses the [`include_bytes`](https://doc.rust-lang.org/nightly/core/macro.include_bytes.html "macro core::include_bytes") macro internally and _will not_ reallocate the bytes. Generally the [`AssetPath`](struct.AssetPath.html "struct bevy::asset::AssetPath") generated will be predictable, but if your asset isn’t available for some reason, you can use the [`embedded_path`](macro.embedded_path.html "macro bevy::asset::embedded_path") macro to debug.

Hot-reloading `embedded` assets is supported. Just enable the `embedded_watcher` cargo feature.