[bevy](../index.html)::[input\_focus](index.html)

# Function process\_recorded\_focus\_changes 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#61)

```rust
pub fn process_recorded_focus_changes(
    focus: ResMut<'_, InputFocus>,
    commands: Commands<'_, '_>,
)
```

Reads the recorded focus changes from the [`InputFocus`](struct.InputFocus.html "struct bevy::input_focus::InputFocus") resource and sends the appropriate [`FocusGained`](struct.FocusGained.html "struct bevy::input_focus::FocusGained") and [`FocusLost`](struct.FocusLost.html "struct bevy::input_focus::FocusLost") events.

This system is part of [`InputFocusPlugin`](struct.InputFocusPlugin.html "struct bevy::input_focus::InputFocusPlugin").