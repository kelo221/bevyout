[bevy](../../index.html)::[window](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/lib.rs.html#38)

The windowing prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[CursorEntered](struct.CursorEntered.html "struct bevy::window::prelude::CursorEntered")

An event that is sent whenever the user’s cursor enters a window.

[CursorLeft](struct.CursorLeft.html "struct bevy::window::prelude::CursorLeft")

An event that is sent whenever the user’s cursor leaves a window.

[CursorMoved](struct.CursorMoved.html "struct bevy::window::prelude::CursorMoved")

An event reporting that the mouse cursor has moved inside a window.

[Window](struct.Window.html "struct bevy::window::prelude::Window")

The defining [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") for window entities, storing information about how it should appear and behave.

[WindowMoved](struct.WindowMoved.html "struct bevy::window::prelude::WindowMoved")

An event that is sent when a window is repositioned in physical pixels.

[WindowPlugin](struct.WindowPlugin.html "struct bevy::window::prelude::WindowPlugin")

A [`Plugin`](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that defines an interface for windowing support in Bevy.

[WindowResizeConstraints](struct.WindowResizeConstraints.html "struct bevy::window::prelude::WindowResizeConstraints")

The size limits on a [`Window`](../../prelude/struct.Window.html "struct bevy::prelude::Window").

## Enums

[FileDragAndDrop](enum.FileDragAndDrop.html "enum bevy::window::prelude::FileDragAndDrop")

Events related to files being dragged and dropped on a window.

[Ime](enum.Ime.html "enum bevy::window::prelude::Ime")

An Input Method Editor event.

[MonitorSelection](enum.MonitorSelection.html "enum bevy::window::prelude::MonitorSelection")

References a screen monitor.

[VideoModeSelection](enum.VideoModeSelection.html "enum bevy::window::prelude::VideoModeSelection")

References an exclusive fullscreen video mode.

[WindowPosition](enum.WindowPosition.html "enum bevy::window::prelude::WindowPosition")

Defines where a [`Window`](../../prelude/struct.Window.html "struct bevy::prelude::Window") should be placed on the screen.