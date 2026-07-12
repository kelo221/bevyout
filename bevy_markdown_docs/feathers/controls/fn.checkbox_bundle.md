[bevy](../../index.html)::[feathers](../index.html)::[controls](index.html)

# Function checkbox\_bundle 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#146-149)

```rust
pub fn checkbox_bundle<C, B>(overrides: B, label: C) -> impl Bundlewhere
    C: SpawnableList<ChildOf> + Send + Sync + 'static,
    B: Bundle,
```

👎Deprecated since 0.19.0:

Use the checkbox() BSN function

Template function to spawn a checkbox.

This version does not take any props. A caption can be set by appending a child entity.

## Emitted events

*   [`bevy_ui_widgets::ValueChange<bool>`](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") with the new value when the checkbox changes state.

These events can be disabled by adding an [`bevy_ui::InteractionDisabled`](../../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled") component to the entity