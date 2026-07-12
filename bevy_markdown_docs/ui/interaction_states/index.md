[bevy](../../index.html)::[ui](../index.html)

# Module interaction\_states 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#16)

## Structs

[Checkable](struct.Checkable.html "struct bevy::ui::interaction_states::Checkable")

Component that indicates that a widget can be checked.

[Checked](struct.Checked.html "struct bevy::ui::interaction_states::Checked")

Component that indicates whether a checkbox or radio button is in a checked state.

[InteractionDisabled](struct.InteractionDisabled.html "struct bevy::ui::interaction_states::InteractionDisabled")

A component indicating that a widget is disabled and should be “grayed out”. This is used to prevent user interaction with the widget. It should not, however, prevent the widget from being updated or rendered, or from acquiring keyboard focus.

[Pressed](struct.Pressed.html "struct bevy::ui::interaction_states::Pressed")

Component that indicates whether a button or widget is currently in a pressed or “held down” state.

[Selectable](struct.Selectable.html "struct bevy::ui::interaction_states::Selectable")

Component that indicates that a widget can be selected. Similar to [`Checkable`](../struct.Checkable.html "struct bevy::ui::Checkable"), but works for the ARIA “selected” state instead of “checked”.

[Selected](struct.Selected.html "struct bevy::ui::interaction_states::Selected")

Similar to [`Checked`](../struct.Checked.html "struct bevy::ui::Checked"), but works for the ARIA “selected” state instead of “checked”.