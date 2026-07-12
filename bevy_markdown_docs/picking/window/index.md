[bevy](../../index.html)::[picking](../index.html)

# Module window 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#166)

This module contains a basic backend that implements picking for window entities.

Pointers can exist on windows, images, and gpu texture views. With [`update_window_hits`](fn.update_window_hits.html "fn bevy::picking::window::update_window_hits") enabled, when a pointer hovers over a window that window will be inserted as a pointer hit, listed behind all other pointer hits. This means that when the pointer isn’t hovering any other entities, the picking events will be routed to the window.

### Implementation Notes

*   This backend does not provide `normal` in `HitData`.

## Functions

[update\_window\_hits](fn.update_window_hits.html "fn bevy::picking::window::update_window_hits")

Generates pointer hit events for window entities.