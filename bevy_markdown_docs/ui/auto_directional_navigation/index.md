[bevy](../../index.html)::[ui](../index.html)

# Module auto\_directional\_navigation 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#15)

An automatic directional navigation system, powered by the [`AutoDirectionalNavigation`](struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation") component.

Unlike the navigation system provided by `bevy_input_focus`, the automatic directional navigation system does not require specifying navigation edges. Just simply add the [`AutoDirectionalNavigation`](struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation") component to your entities, and the system will automatically calculate the navigation edges between entities based on screen position.

[`AutoDirectionalNavigator`](struct.AutoDirectionalNavigator.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigator") replaces the manual directional navigation system provided by the [`DirectionalNavigation`](../../input_focus/directional_navigation/struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation") system parameter from `bevy_input_focus`. The [`AutoDirectionalNavigator`](struct.AutoDirectionalNavigator.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigator") will first navigate using manual override edges defined in the [`DirectionalNavigationMap`](../../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap"). If no manual overrides are defined, automatic navigation will occur between entities based on screen position.

If any resulting navigation behavior is undesired, [`AutoNavigationConfig`](../../input_focus/directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig") can be tweaked or manual overrides can be specified using the [`DirectionalNavigationMap`](../../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap").

## Structs

[AutoDirectionalNavigation](struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation")

Marker component to enable automatic directional navigation to and from the entity.

[AutoDirectionalNavigator](struct.AutoDirectionalNavigator.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigator")

A system parameter for combining manual and auto navigation between focusable entities in a directional way. This wraps the [`DirectionalNavigation`](../../input_focus/directional_navigation/struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation") system parameter provided by `bevy_input_focus` and augments it with auto directional navigation. To use, the [`DirectionalNavigationPlugin`](../../input_focus/directional_navigation/struct.DirectionalNavigationPlugin.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationPlugin") must be added to the app.