[bevy](../index.html)

# Crate settings 

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#1-1053)

Framework for saving and loading user settings files in Bevy applications.

Refer to [`SettingsPlugin`](struct.SettingsPlugin.html "struct bevy::settings::SettingsPlugin") for detailed usage information.

## Structs

[ReflectSettingsGroup](struct.ReflectSettingsGroup.html "struct bevy::settings::ReflectSettingsGroup")

Reflected data from a [`SettingsGroup`](trait.SettingsGroup.html "trait bevy::settings::SettingsGroup").

[SaveSettingsDeferred](struct.SaveSettingsDeferred.html "struct bevy::settings::SaveSettingsDeferred")

A Command which saves changed settings after a delay. This is debounced: issuing this command multiple times resets the delay timer each time. This is meant to be used for settings which change at a high frequency, such as dragging a slider which controls the game’s audio volume. The default delay is 1.0 seconds.

[SettingsPlugin](struct.SettingsPlugin.html "struct bevy::settings::SettingsPlugin")

Plugin to orchestrate loading and saving settings.

## Enums

[SaveSettings](enum.SaveSettings.html "enum bevy::settings::SaveSettings")

A [`Command`](../prelude/trait.Command.html "trait bevy::prelude::Command") which saves settings to disk. Actual file system operations happen in another thread.

[SaveSettingsSync](enum.SaveSettingsSync.html "enum bevy::settings::SaveSettingsSync")

A Command which saves settings to disk. This blocks the command queue until saving is complete.

## Traits

[SettingsGroup](trait.SettingsGroup.html "trait bevy::settings::SettingsGroup")

Trait which identifies a type as corresponding to a section with a settings file.

## Derive Macros

[SettingsGroup](derive.SettingsGroup.html "derive bevy::settings::SettingsGroup")

Cheat sheet for derive syntax,