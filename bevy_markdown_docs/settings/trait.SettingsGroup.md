[bevy](../index.html)::[settings](index.html)

# Trait SettingsGroup 

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#139)

```rust
pub trait SettingsGroup: Resource {
    // Required methods
    fn settings_group_name() -> &'static str;
    fn settings_key_name() -> Option<&'static str>;
    fn settings_source() -> Option<&'static str>;
}
```

Trait which identifies a type as corresponding to a section with a settings file.

You can override the name of the section with `settings_group(group = "<name>")`. For enum `SettingGroup`s, you can also override the name of its key with `settings_group(key = "<name>")` The name should be in `snake_case` to be consistent with TOML style. If there is a collision between names (multiple resources have the same name) then the resulting properties will be merged into a single section.

You can also control which file the type gets saved to via `settings_group(file = "<filename>")`. This should be the base name of the file without the extension. The default name is `settings`, which will cause the settings to be written out to `settings.toml` in the app’s settings directory.

## Required Methods

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#141)

#### fn [settings\_group\_name](#tymethod.settings_group_name)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

The name of the logical section within the settings file.

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#147)

#### fn [settings\_key\_name](#tymethod.settings_key_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

The key name within the settings file. For structs, this should be set to `None`; The struct’s field names will be used as keys. For enums, the `SettingsGroup` will use this key name within the settings file for its sole key-value pair. This is typically the same as the group name, but can be customized.

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#151)

#### fn [settings\_source](#tymethod.settings_source)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

The name of the configuration file that contains this settings group.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors