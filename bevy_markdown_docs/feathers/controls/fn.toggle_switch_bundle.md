[bevy](../../index.html)::[feathers](../index.html)::[controls](index.html)

# Function toggle\_switch\_bundle 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/toggle_switch.rs.html#98)

```rust
pub fn toggle_switch_bundle<B>(overrides: B) -> impl Bundlewhere
    B: Bundle,
```

👎Deprecated since 0.19.0:

Use the toggle\_switch() BSN function

Template function to spawn a toggle switch.

## Arguments

*   `props` - construction properties for the toggle switch.
*   `overrides` - a bundle of components that are merged in with the normal toggle switch components.

## Emitted events

*   [`bevy_ui_widgets::ValueChange<bool>`](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") with the new value when the toggle switch changes state.

These events can be disabled by adding an [`bevy_ui::InteractionDisabled`](../../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled") component to the bundle