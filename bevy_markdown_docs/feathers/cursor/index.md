[bevy](../../index.html)::[feathers](../index.html)

# Module cursor 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/lib.rs.html#52)

Provides a way to automatically set the mouse cursor based on hovered entity.

## Structs

[CursorIconPlugin](struct.CursorIconPlugin.html "struct bevy::feathers::cursor::CursorIconPlugin")

Plugin that supports automatically changing the cursor based on the hovered entity.

[DefaultCursor](struct.DefaultCursor.html "struct bevy::feathers::cursor::DefaultCursor")

A resource that specifies the cursor icon to be used when the mouse is not hovering over any other entity. This is used to set the default cursor icon for the window.

[OverrideCursor](struct.OverrideCursor.html "struct bevy::feathers::cursor::OverrideCursor")

A resource used to override any [`EntityCursor`](enum.EntityCursor.html "enum bevy::feathers::cursor::EntityCursor") cursor changes.

## Enums

[EntityCursor](enum.EntityCursor.html "enum bevy::feathers::cursor::EntityCursor")

A component that specifies the cursor shape to be used when the pointer hovers over an entity. This is copied to the windows’s [`CursorIcon`](../../window/enum.CursorIcon.html "enum bevy::window::CursorIcon") component.

[EntityCursorTemplate](enum.EntityCursorTemplate.html "enum bevy::feathers::cursor::EntityCursorTemplate")