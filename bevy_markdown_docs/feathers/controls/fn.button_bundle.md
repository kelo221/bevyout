[bevy](../../index.html)::[feathers](../index.html)::[controls](index.html)

# Function button\_bundle 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#167-171)

```rust
pub fn button_bundle<C, B>(
    props: ButtonBundleProps,
    overrides: B,
    children: C,
) -> impl Bundlewhere
    C: SpawnableList<ChildOf> + Send + Sync + 'static,
    B: Bundle,
```

👎Deprecated since 0.19.0:

Use the button() BSN function

Template function to spawn a button.

## Arguments

*   `props` - construction properties for the button.

## Emitted events

*   [`bevy_ui_widgets::Activate`](../../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate") when any of the following happens:
    *   the pointer is released while hovering over the button.
    *   the ENTER or SPACE key is pressed while the button has keyboard focus.

These events can be disabled by adding an [`bevy_ui::InteractionDisabled`](../../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled") component to the entity