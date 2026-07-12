[bevy](../index.html)

# Crate feathers 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/lib.rs.html#1-125)

`bevy_feathers` is a collection of styled and themed widgets for building editors and inspectors.

The aesthetic choices made here are designed with a future Bevy Editor in mind, but this crate is deliberately exposed to the public to allow the broader ecosystem to easily create tooling for themselves and others that fits cohesively together.

While it may be tempting to use this crate for your game’s UI, it’s deliberately not intended for that. We’ve opted for a clean, functional style, and prioritized consistency over customization. That said, if you like what you see, it can be a helpful learning tool. Consider copying this code into your own project, and refining the styles and abstractions provided to meet your needs.

### Best practices for event propagation

Generally, when a widget handles an event, propagation of that event to parent entities should be stopped. This is important when writing your custom widgets, and understanding the behavior of existing widgets.

For more guidance on this, see the documentation for [`EntityEvent`](../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent").

### Warning: Experimental!

All that said, this crate is still experimental and unfinished! It will change in breaking ways, and there will be both bugs and limitations.

Please report issues, submit fixes and propose changes. Thanks for stress-testing; let’s build something better together.

## Modules

[constants](constants/index.html "mod bevy::feathers::constants")

Various non-themable constants for the Feathers look and feel.

[containers](containers/index.html "mod bevy::feathers::containers")

Meta-module containing all feathers containers: passive widgets that hold other widgets.

[controls](controls/index.html "mod bevy::feathers::controls")

Meta-module containing all feathers controls (widgets that are interactive).

[cursor](cursor/index.html "mod bevy::feathers::cursor")

Provides a way to automatically set the mouse cursor based on hovered entity.

[dark\_theme](dark_theme/index.html "mod bevy::feathers::dark_theme")

The standard `bevy_feathers` dark theme.

[display](display/index.html "mod bevy::feathers::display")

Static widgets that only display data and are not interactive.

[focus](focus/index.html "mod bevy::feathers::focus")

This module contains the infrastructure needed for displaying focus outlines.

[font\_styles](font_styles/index.html "mod bevy::feathers::font_styles")

A framework for inheritable font styles.

[palette](palette/index.html "mod bevy::feathers::palette")

The Feathers standard color palette.

[rounded\_corners](rounded_corners/index.html "mod bevy::feathers::rounded_corners")

Mechanism for specifying which corners of a widget are rounded, used for segmented buttons and control groups.

[theme](theme/index.html "mod bevy::feathers::theme")

A framework for theming.

[tokens](tokens/index.html "mod bevy::feathers::tokens")

Design tokens used by Feathers themes.

## Structs

[FeathersCorePlugin](struct.FeathersCorePlugin.html "struct bevy::feathers::FeathersCorePlugin")

Plugin which installs observers and systems for feathers themes, cursors, and all controls.

[FeathersPlugins](struct.FeathersPlugins.html "struct bevy::feathers::FeathersPlugins")

A plugin group that adds all dependencies for Feathers