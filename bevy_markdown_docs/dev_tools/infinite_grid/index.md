[bevy](../../index.html)::[dev\_tools](../index.html)

# Module infinite\_grid 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/lib.rs.html#32)

This module implements an infinite grid with colored major axis.

The rendering is not actually infinite and fades out over a customizable distance to avoid artifacts. This fade out is relative to the camera.

## Structs

[InfiniteGrid](struct.InfiniteGrid.html "struct bevy::dev_tools::infinite_grid::InfiniteGrid")

The component used to represent an infinite grid.

[InfiniteGridPlugin](struct.InfiniteGridPlugin.html "struct bevy::dev_tools::infinite_grid::InfiniteGridPlugin")

The plugin required to make the infinite grid work

[InfiniteGridSettings](struct.InfiniteGridSettings.html "struct bevy::dev_tools::infinite_grid::InfiniteGridSettings")

Component to configure the infinite grid