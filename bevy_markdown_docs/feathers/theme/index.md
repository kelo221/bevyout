[bevy](../../index.html)::[feathers](../index.html)

# Module theme 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/lib.rs.html#59)

A framework for theming.

## Structs

[InheritableThemeTextColor](struct.InheritableThemeTextColor.html "struct bevy::feathers::theme::InheritableThemeTextColor")

Component which causes the inherited text color of an entity to be set based on a theme color.

[ThemeBackgroundColor](struct.ThemeBackgroundColor.html "struct bevy::feathers::theme::ThemeBackgroundColor")

Component which causes the background color of an entity to be set based on a theme color.

[ThemeBorderColor](struct.ThemeBorderColor.html "struct bevy::feathers::theme::ThemeBorderColor")

Component which causes the border color of an entity to be set based on a theme color. Only supports setting all borders to the same color.

[ThemeProps](struct.ThemeProps.html "struct bevy::feathers::theme::ThemeProps")

A collection of properties that make up a theme.

[ThemeTextColor](struct.ThemeTextColor.html "struct bevy::feathers::theme::ThemeTextColor")

Component which causes the color of a text span to be set based on a theme color. Unlike [`InheritableThemeTextColor`](struct.InheritableThemeTextColor.html "struct bevy::feathers::theme::InheritableThemeTextColor"), this can work when set directly on the text span entity, and is not inherited.

[ThemeToken](struct.ThemeToken.html "struct bevy::feathers::theme::ThemeToken")

A design token for the theme. This serves as the lookup key for the theme properties.

[ThemedText](struct.ThemedText.html "struct bevy::feathers::theme::ThemedText")

A marker component that is used to indicate that the text entity wants to opt-in to using inherited text styles.

[UiTheme](struct.UiTheme.html "struct bevy::feathers::theme::UiTheme")

The currently selected user interface theme. Overwriting this resource changes the theme.