[bevy](../../index.html)::[feathers](../index.html)::[controls](index.html)

# Function radio\_bundle 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/radio.rs.html#135-138)

```rust
pub fn radio_bundle<C, B>(overrides: B, label: C) -> impl Bundlewhere
    C: SpawnableList<ChildOf> + Send + Sync + 'static,
    B: Bundle,
```

👎Deprecated since 0.19.0:

Use the radio() BSN function

Template function to spawn a radio.

This version does not take any props. A caption can be set by appending a child entity.

## Emitted events

*   [`bevy_ui_widgets::ValueChange<bool>`](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") with the value true when it becomes checked.
*   [`bevy_ui_widgets::ValueChange<Entity>`](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") with the selected entity’s id when a new radio button is selected.

These events can be disabled by adding an [`bevy_ui::InteractionDisabled`](../../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled") component to the entity