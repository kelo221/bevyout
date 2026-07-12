[bevy](../../index.html)::[ui\_widgets](../index.html)

# Module popover 

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#34)

Framework for positioning of popups, tooltips, and other popover UI elements.

## Structs

[Popover](struct.Popover.html "struct bevy::ui_widgets::popover::Popover")

Component which is inserted into a popover element to make it dynamically position relative to an parent element.

[PopoverPlacement](struct.PopoverPlacement.html "struct bevy::ui_widgets::popover::PopoverPlacement")

Indicates a possible position of a popover element relative to it’s parent. You can specify multiple possible positions; the positioning code will check to see if there is sufficient space to display the popup without being clipped by the window edge. If any position has sufficient room, it will pick the first one; if there are none, then it will pick the least bad one.

[PopoverPlugin](struct.PopoverPlugin.html "struct bevy::ui_widgets::popover::PopoverPlugin")

Plugin that adds systems for the [`Popover`](struct.Popover.html "struct bevy::ui_widgets::popover::Popover") component.

## Enums

[PopoverAlign](enum.PopoverAlign.html "enum bevy::ui_widgets::popover::PopoverAlign")

How the popover element should be aligned to the parent element. The alignment will be along an axis that is perpendicular to the direction of the popover side. So for example, if the popup is positioned below the parent, then the [`PopoverAlign`](enum.PopoverAlign.html "enum bevy::ui_widgets::popover::PopoverAlign") variant controls the horizontal alignment of the popup.

[PopoverSide](enum.PopoverSide.html "enum bevy::ui_widgets::popover::PopoverSide")

Which side of the parent element the popover element should be placed.