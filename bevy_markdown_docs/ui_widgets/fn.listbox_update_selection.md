[bevy](../index.html)::[ui\_widgets](index.html)

# Function listbox\_update\_selection 

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#315-322)

```rust
pub fn listbox_update_selection(
    value_change: On<'_, '_, ValueChange<Entity>>,
    q_listbox: Query<'_, '_, (), With<ListBox>>,
    q_listitems: Query<'_, '_, (Has<Selected>, Has<InteractionDisabled>), With<ListItem>>,
    q_parents: Query<'_, '_, &ChildOf>,
    q_children: Query<'_, '_, &Children>,
    commands: Commands<'_, '_>,
)
```

Observer function for updating list row selection state.