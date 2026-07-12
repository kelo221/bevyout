[bevy](../../index.html)::[input\_focus](../index.html)::[tab\_navigation](index.html)

# Function handle\_tab\_navigation 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#429-435)

```rust
pub fn handle_tab_navigation(
    event: On<'_, '_, FocusedInput<KeyboardInput>>,
    nav: TabNavigation<'_, '_>,
    focus: ResMut<'_, InputFocus>,
    visible: ResMut<'_, InputFocusVisible>,
    keys: Res<'_, ButtonInput<KeyCode>>,
)
```

Observer function which handles tab navigation.

This observer responds to [`KeyCode::Tab`](../../prelude/enum.KeyCode.html#variant.Tab "variant bevy::prelude::KeyCode::Tab") events and Shift+Tab events, cycling through focusable entities in the order determined by their tab index.

Any [`TabNavigationError`](enum.TabNavigationError.html "enum bevy::input_focus::tab_navigation::TabNavigationError")s that occur during tab navigation are logged as warnings.