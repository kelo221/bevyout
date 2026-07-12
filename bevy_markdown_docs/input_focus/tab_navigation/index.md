[bevy](../../index.html)::[input\_focus](../index.html)

# Module tab\_navigation 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#28)

This module provides a framework for handling linear tab-key navigation in Bevy applications.

The rules of tabbing are derived from the HTML specification, and are as follows:

*   An index >= 0 means that the entity is tabbable via sequential navigation. The order of tabbing is determined by the index, with lower indices being tabbed first. If two entities have the same index, then the order is determined by the order of the entities in the ECS hierarchy (as determined by Parent/Child).
*   An index < 0 means that the entity is not focusable via sequential navigation, but can still be focused via direct selection.

Tabbable entities must be descendants of a [`TabGroup`](struct.TabGroup.html "struct bevy::input_focus::tab_navigation::TabGroup") entity, which is a component that marks a tree of entities as containing tabbable elements. The order of tab groups is determined by the [`TabGroup::order`](struct.TabGroup.html#structfield.order "field bevy::input_focus::tab_navigation::TabGroup::order") field, with lower orders being tabbed first. Modal tab groups are used for ui elements that should only tab within themselves, such as modal dialog boxes.

To enable automatic tabbing, add the [`TabNavigationPlugin`](struct.TabNavigationPlugin.html "struct bevy::input_focus::tab_navigation::TabNavigationPlugin") and [`InputDispatchPlugin`](../struct.InputDispatchPlugin.html "struct bevy::input_focus::InputDispatchPlugin") to your app. This will install a keyboard event observer on the primary window which automatically handles tab navigation for you.

Alternatively, if you want to have more control over tab navigation, or are using an input-action-mapping framework, you can use the \[`TabNavigation`\] system parameter directly instead. This object can be injected into your systems, and provides a [`navigate`](%60TabNavigation::navigate%60) method which can be used to navigate between focusable entities.

## Structs

[TabGroup](struct.TabGroup.html "struct bevy::input_focus::tab_navigation::TabGroup")

A component used to mark a tree of entities as containing tabbable elements.

[TabIndex](struct.TabIndex.html "struct bevy::input_focus::tab_navigation::TabIndex")

A component which indicates that an entity wants to participate in tab navigation.

[TabNavigationPlugin](struct.TabNavigationPlugin.html "struct bevy::input_focus::tab_navigation::TabNavigationPlugin")

Plugin for navigating between focusable entities using keyboard input.

## Enums

[NavAction](enum.NavAction.html "enum bevy::input_focus::tab_navigation::NavAction")

A navigation action that users might take to navigate your user interface in a cyclic fashion.

[TabNavigationError](enum.TabNavigationError.html "enum bevy::input_focus::tab_navigation::TabNavigationError")

An error that can occur during [tab navigation](index.html "mod bevy::input_focus::tab_navigation").

## Functions

[handle\_tab\_navigation](fn.handle_tab_navigation.html "fn bevy::input_focus::tab_navigation::handle_tab_navigation")

Observer function which handles tab navigation.