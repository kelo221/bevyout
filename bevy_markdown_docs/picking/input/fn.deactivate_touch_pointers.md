[bevy](../../index.html)::[picking](../index.html)::[input](index.html)

# Function deactivate\_touch\_pointers 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#280-285)

```rust
pub fn deactivate_touch_pointers(
    commands: Commands<'_, '_>,
    despawn_list: Local<'_, HashSet<(Entity, PointerId)>>,
    pointers: Query<'_, '_, (Entity, &PointerId)>,
    touches: MessageReader<'_, '_, TouchInput>,
)
```

Deactivates unused touch pointers.

Because each new touch gets assigned a new ID, we need to remove the pointers associated with touches that are no longer active.