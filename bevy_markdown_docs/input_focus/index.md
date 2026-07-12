[bevy](../index.html)

# Crate input\_focus 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#1-751)

A UI-centric focus system for Bevy.

This crate provides a system for managing input focus in Bevy applications, including:

*   [`InputFocus`](struct.InputFocus.html "struct bevy::input_focus::InputFocus"), a resource for tracking which entity has input focus.
*   Methods for getting and setting input focus via [`InputFocus`](struct.InputFocus.html "struct bevy::input_focus::InputFocus") and [`IsFocusedHelper`](struct.IsFocusedHelper.html "struct bevy::input_focus::IsFocusedHelper").
*   Events for when entities gain or lose focus: [`FocusGained`](struct.FocusGained.html "struct bevy::input_focus::FocusGained") and [`FocusLost`](struct.FocusLost.html "struct bevy::input_focus::FocusLost").
*   A generic [`FocusedInput`](struct.FocusedInput.html "struct bevy::input_focus::FocusedInput") event to send input events which bubble up from the focused entity.
*   Various navigation frameworks for moving input focus between entities based on user input, such as [`tab_navigation`](tab_navigation/index.html "mod bevy::input_focus::tab_navigation") and [`directional_navigation`](directional_navigation/index.html "mod bevy::input_focus::directional_navigation").

This crate does _not_ provide any integration with UI widgets: this is the responsibility of the widget crate, which should depend on [`bevy_input_focus`](index.html "mod bevy::input_focus").

## Modules

[directional\_navigation](directional_navigation/index.html "mod bevy::input_focus::directional_navigation")

A manual navigation framework for moving between focusable elements based on directional input.

[navigator](navigator/index.html "mod bevy::input_focus::navigator")

Functions used by navigators to determine where to go next.

[tab\_navigation](tab_navigation/index.html "mod bevy::input_focus::tab_navigation")

This module provides a framework for handling linear tab-key navigation in Bevy applications.

## Structs

[AcquireFocus](struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")

An event which is used to set input focus. Trigger this on an entity, and it will bubble until it finds a focusable entity, and then set focus to it.

[AutoFocus](struct.AutoFocus.html "struct bevy::input_focus::AutoFocus")

Indicates that this widget should automatically receive [`InputFocus`](struct.InputFocus.html "struct bevy::input_focus::InputFocus").

[FocusGained](struct.FocusGained.html "struct bevy::input_focus::FocusGained")

An [`EntityEvent`](../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") that is sent when an entity gains [`InputFocus`](struct.InputFocus.html "struct bevy::input_focus::InputFocus").

[FocusLost](struct.FocusLost.html "struct bevy::input_focus::FocusLost")

An [`EntityEvent`](../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") that is sent when an entity loses [`InputFocus`](struct.InputFocus.html "struct bevy::input_focus::InputFocus").

[FocusedInput](struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")

A bubble-able user input event that starts at the currently focused entity.

[InputDispatchPlugin](struct.InputDispatchPlugin.html "struct bevy::input_focus::InputDispatchPlugin")

Plugin which sets up systems for dispatching bubbling keyboard and gamepad button events to the focused entity.

[InputFocus](struct.InputFocus.html "struct bevy::input_focus::InputFocus")

Resource representing which entity has input focus, if any. Input events (other than pointer-like inputs) will be dispatched to the current focus entity, or to the primary window if no entity has focus.

[InputFocusPlugin](struct.InputFocusPlugin.html "struct bevy::input_focus::InputFocusPlugin")

Plugin which sets up the core input focus system.

[InputFocusVisible](struct.InputFocusVisible.html "struct bevy::input_focus::InputFocusVisible")

Resource representing whether the input focus indicator should be visible on UI elements.

[IsFocusedHelper](struct.IsFocusedHelper.html "struct bevy::input_focus::IsFocusedHelper")

A system param that helps get information about the current focused entity.

[WindowTraversal](struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

These are for accessing components defined on the targeted entity

[WindowTraversalItem](struct.WindowTraversalItem.html "struct bevy::input_focus::WindowTraversalItem")

Automatically generated [`WorldQuery`](../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") item type for [`WindowTraversal`](struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal"), returned when iterating over query results.

## Enums

[FocusCause](enum.FocusCause.html "enum bevy::input_focus::FocusCause")

The cause for a [`FocusGained`](struct.FocusGained.html "struct bevy::input_focus::FocusGained")

[InputFocusSystems](enum.InputFocusSystems.html "enum bevy::input_focus::InputFocusSystems")

System sets for [`bevy_input_focus`](index.html "mod bevy::input_focus").

## Traits

[IsFocused](trait.IsFocused.html "trait bevy::input_focus::IsFocused")

Trait which defines methods to check if an entity currently has focus.

## Functions

[dispatch\_focused\_input](fn.dispatch_focused_input.html "fn bevy::input_focus::dispatch_focused_input")

System which dispatches bubbled input events to the focused entity, or to the primary window if no entity has focus.

[process\_recorded\_focus\_changes](fn.process_recorded_focus_changes.html "fn bevy::input_focus::process_recorded_focus_changes")

Reads the recorded focus changes from the [`InputFocus`](struct.InputFocus.html "struct bevy::input_focus::InputFocus") resource and sends the appropriate [`FocusGained`](struct.FocusGained.html "struct bevy::input_focus::FocusGained") and [`FocusLost`](struct.FocusLost.html "struct bevy::input_focus::FocusLost") events.

[set\_initial\_focus](fn.set_initial_focus.html "fn bevy::input_focus::set_initial_focus")

If no entity is focused, sets the focus to the primary window, if any.