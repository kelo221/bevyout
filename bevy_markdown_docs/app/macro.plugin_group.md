[bevy](../index.html)::[app](index.html)

# Macro plugin\_group 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin_group.rs.html#106)

```rust
macro_rules! plugin_group {
    {
        $(#[$group_meta:meta])*
        $vis:vis struct $group:ident {
            $(
                $(#[cfg(feature = $plugin_feature:literal)])?
                $(#[custom($plugin_meta:meta)])*
                $($plugin_path:ident::)* : $plugin_name:ident
            ),*
            $(
                $(,)?$(
                    #[plugin_group]
                    $(#[cfg(feature = $plugin_group_feature:literal)])?
                    $(#[custom($plugin_group_meta:meta)])*
                    $($plugin_group_path:ident::)* : $plugin_group_name:ident
                ),+
            )?
            $(
                $(,)?$(
                    #[doc(hidden)]
                    $(#[cfg(feature = $hidden_plugin_feature:literal)])?
                    $(#[custom($hidden_plugin_meta:meta)])*
                    $($hidden_plugin_path:ident::)* : $hidden_plugin_name:ident
                ),+
            )?

            $(,)?
        }
        $($(#[doc = $post_doc:literal])+)?
    } => { ... };
}
```

A macro for generating a well-documented [`PluginGroup`](../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") from a list of [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") paths.

Every plugin must implement the [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") trait.

## Example

```rust
plugin_group! {
    /// Doc comments and annotations are supported: they will be added to the generated plugin
    /// group.
    #[derive(Debug)]
    pub struct PhysicsPlugins {
        // If referencing a plugin within the same module, you must prefix it with a colon `:`.
        :TickratePlugin,
        // If referencing a plugin within a different module, there must be three colons `:::`
        // between the final module and the plugin name.
        collision::capsule:::CapsuleCollisionPlugin,
        velocity:::VelocityPlugin,
        // If you feature-flag a plugin, it will be automatically documented. There can only be
        // one automatically documented feature flag, and it must be first. All other
        // `#[cfg()]` attributes must be wrapped by `#[custom()]`.
        #[cfg(feature = "external_forces")]
        features:::ForcePlugin,
        // More complicated `#[cfg()]`s and annotations are not supported by automatic doc
        // generation, in which case you must wrap it in `#[custom()]`.
        #[custom(cfg(target_arch = "wasm32"))]
        web:::WebCompatibilityPlugin,
        // You can nest `PluginGroup`s within other `PluginGroup`s, you just need the
        // `#[plugin_group]` attribute.
        #[plugin_group]
        audio:::AudioPlugins,
        // You can hide plugins from documentation. Due to macro limitations, hidden plugins
        // must be last.
        #[doc(hidden)]
        internal:::InternalPlugin
    }
    /// You may add doc comments after the plugin group as well. They will be appended after
    /// the documented list of plugins.
}
```