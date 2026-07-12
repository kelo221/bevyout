[bevy](../../index.html)::[input\_focus](../index.html)

# Module directional\_navigation 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#26)

A manual navigation framework for moving between focusable elements based on directional input.

Note: If using `bevy_ui`, this manual navigation framework is used to provide overrides for its automatic navigation framework based on the `AutoDirectionalNavigation` component. Most times, the automatic navigation framework alone should be sufficient. If not using `bevy_ui`, this manual navigation framework can still be used by itself.

While virtual cursors are a common way to navigate UIs with a gamepad (or arrow keys!), they are generally both slow and frustrating to use. Instead, directional inputs should provide a direct way to snap between focusable elements.

Like the rest of this crate, the [`InputFocus`](../struct.InputFocus.html "struct bevy::input_focus::InputFocus") resource is manipulated to track the current focus.

This module’s [`DirectionalNavigationMap`](struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap") stores a directed graph of focusable entities. Each entity can have up to 8 neighbors, one for each [`CompassOctant`](../../math/enum.CompassOctant.html "enum bevy::math::CompassOctant"), balancing flexibility and required precision.

Navigating between focusable entities (commonly UI nodes) is done by passing a [`CompassOctant`](../../math/enum.CompassOctant.html "enum bevy::math::CompassOctant") into the [`navigate`](struct.DirectionalNavigation.html#method.navigate "method bevy::input_focus::directional_navigation::DirectionalNavigation::navigate") method from the [`DirectionalNavigation`](struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation") system parameter. Under the hood, the [`DirectionalNavigationMap`](struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap") is used to return the focusable entity in a direction for a given entity.

## Setting up Directional Navigation

### Automatic Navigation (Recommended)

The easiest way to set up navigation is to add the `AutoDirectionalNavigation` component to your UI entities. This component is available in the `bevy_ui` crate. If you choose to include automatic navigation, you should also use the `AutoDirectionalNavigator` system parameter in that crate instead of [`DirectionalNavigation`](struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation").

### Combining Automatic Navigation with Manual Overrides

Following manual edges always take precedence, allowing you to use automatic navigation for most UI elements while overriding specific connections for special cases like wrapping menus or cross-layer navigation. If you need to override automatic navigation behavior, use the [`DirectionalNavigationMap`](struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap") to define overriding edges between UI entities.

### Manual Navigation Only

Manually define your navigation using the [`DirectionalNavigationMap`](struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap"), and use the [`DirectionalNavigation`](struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation") system parameter to navigate between components. You can define navigation connections using methods like [`add_edge`](struct.DirectionalNavigationMap.html#method.add_edge "method bevy::input_focus::directional_navigation::DirectionalNavigationMap::add_edge") and [`add_looping_edges`](struct.DirectionalNavigationMap.html#method.add_looping_edges "method bevy::input_focus::directional_navigation::DirectionalNavigationMap::add_looping_edges").

### When to Use Manual Navigation or Manual Overrides

While automatic navigation is recommended and satisfactory for most use cases, using manual navigation only or integrating manual overrides to automatic navigation provide:

*   **Precise control**: Define exact navigation flow, including non-obvious connections like looping edges
*   **Cross-layer navigation**: Connect elements across different UI layers or z-index levels
*   **Custom behavior**: Implement domain-specific navigation patterns (e.g., spreadsheet-style wrapping)

## Structs

[AutoNavigationConfig](struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig")

Configuration resource for automatic directional navigation and for generating manual navigation edges via [`auto_generate_navigation_edges`](fn.auto_generate_navigation_edges.html "fn bevy::input_focus::directional_navigation::auto_generate_navigation_edges")

[DirectionalNavigation](struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation")

A system parameter for navigating between focusable entities in a directional way.

[DirectionalNavigationMap](struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap")

A resource that stores the manually specified traversable graph of focusable entities.

[DirectionalNavigationPlugin](struct.DirectionalNavigationPlugin.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationPlugin")

A plugin that sets up the directional navigation resources.

[FocusableArea](struct.FocusableArea.html "struct bevy::input_focus::directional_navigation::FocusableArea")

A focusable area with position and size information.

[NavNeighbors](struct.NavNeighbors.html "struct bevy::input_focus::directional_navigation::NavNeighbors")

The up-to-eight neighbors of a focusable entity, one for each [`CompassOctant`](../../math/enum.CompassOctant.html "enum bevy::math::CompassOctant").

## Enums

[DirectionalNavigationError](enum.DirectionalNavigationError.html "enum bevy::input_focus::directional_navigation::DirectionalNavigationError")

An error that can occur when navigating between focusable entities using [directional navigation](index.html "mod bevy::input_focus::directional_navigation").

[NavNeighbor](enum.NavNeighbor.html "enum bevy::input_focus::directional_navigation::NavNeighbor")

Represents what’s near a focusable entity.

## Traits

[Navigable](trait.Navigable.html "trait bevy::input_focus::directional_navigation::Navigable")

Trait for extracting position and size from navigable UI components.

## Functions

[auto\_generate\_navigation\_edges](fn.auto_generate_navigation_edges.html "fn bevy::input_focus::directional_navigation::auto_generate_navigation_edges")

Automatically generates directional navigation edges for a collection of nodes.