[bevy](../../index.html)::[feathers](../index.html)

# Module focus 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/lib.rs.html#55)

This module contains the infrastructure needed for displaying focus outlines.

## Structs

[FocusIndicator](struct.FocusIndicator.html "struct bevy::feathers::focus::FocusIndicator")

A marker component which indicates that this entity should display a visible focus outline when either it, or its ancestor, are focused. Insert this into a widget on the entity that you wish to display a focus outline.

[FocusOutlinesPlugin](struct.FocusOutlinesPlugin.html "struct bevy::feathers::focus::FocusOutlinesPlugin")

Plugin which registers the systems for updating focus outlines.

[FocusWithinIndicator](struct.FocusWithinIndicator.html "struct bevy::feathers::focus::FocusWithinIndicator")

A marker component which indicates that this entity should display a visible focus outline when either it, or any descendant, are focused. Insert this into a widget on the entity that you wish to display a focus outline.