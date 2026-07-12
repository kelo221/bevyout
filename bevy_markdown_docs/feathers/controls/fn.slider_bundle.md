[bevy](../../index.html)::[feathers](../index.html)::[controls](index.html)

# Function slider\_bundle 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/slider.rs.html#148)

```rust
pub fn slider_bundle<B>(props: FeathersSliderProps, overrides: B) -> impl Bundlewhere
    B: Bundle,
```

👎Deprecated since 0.19.0:

Use the slider() BSN function

Spawn a new slider widget.

## Arguments

*   `props` - construction properties for the slider.
*   `overrides` - a bundle of components that are merged in with the normal slider components.

## Emitted events

*   [`bevy_ui_widgets::ValueChange<f32>`](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") when the slider value is changed.

These events can be disabled by adding an [`bevy_ui::InteractionDisabled`](../../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled") component to the entity