[bevy](../../index.html)::[feathers](../index.html)

# Module controls 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/lib.rs.html#51)

Meta-module containing all feathers controls (widgets that are interactive).

## Structs

[ButtonBundleProps](struct.ButtonBundleProps.html "struct bevy::feathers::controls::ButtonBundleProps")

Parameters for the [`button_bundle`](fn.button_bundle.html "fn bevy::feathers::controls::button_bundle") template.

[ButtonPlugin](struct.ButtonPlugin.html "struct bevy::feathers::controls::ButtonPlugin")

Plugin which registers the systems for updating the button styles.

[CheckboxPlugin](struct.CheckboxPlugin.html "struct bevy::feathers::controls::CheckboxPlugin")

Plugin which registers the systems for updating the checkbox styles.

[ColorPlanePlugin](struct.ColorPlanePlugin.html "struct bevy::feathers::controls::ColorPlanePlugin")

Plugin which registers the observers for updating the swatch color.

[ColorPlaneValue](struct.ColorPlaneValue.html "struct bevy::feathers::controls::ColorPlaneValue")

Component that contains the two components of the selected color, as well as the “z” value. The x and y values determine the placement of the thumb element, while the z value controls the background gradient.

[ColorSlider](struct.ColorSlider.html "struct bevy::feathers::controls::ColorSlider")

A color slider widget.

[ColorSliderPlugin](struct.ColorSliderPlugin.html "struct bevy::feathers::controls::ColorSliderPlugin")

Plugin which registers the systems for updating the slider styles.

[ColorSwatchFg](struct.ColorSwatchFg.html "struct bevy::feathers::controls::ColorSwatchFg")

Marker identifying the color swatch foreground, the piece that actually displays the color in front of the alpha pattern. This exists so that users can reach in and change the color dynamically.

[ColorSwatchPlugin](struct.ColorSwatchPlugin.html "struct bevy::feathers::controls::ColorSwatchPlugin")

Plugin which registers the observers for updating the swatch color.

[ColorSwatchValue](struct.ColorSwatchValue.html "struct bevy::feathers::controls::ColorSwatchValue")

Component that contains the value of the color swatch. This is copied to the child element background.

[ControlsPlugin](struct.ControlsPlugin.html "struct bevy::feathers::controls::ControlsPlugin")

Plugin which registers all `bevy_feathers` controls.

[DisclosureTogglePlugin](struct.DisclosureTogglePlugin.html "struct bevy::feathers::controls::DisclosureTogglePlugin")

Plugin which registers the systems for updating the toggle switch styles.

[FeathersButton](struct.FeathersButton.html "struct bevy::feathers::controls::FeathersButton")

A button widget.

[FeathersButtonProps](struct.FeathersButtonProps.html "struct bevy::feathers::controls::FeathersButtonProps")

Props used to construct a [`FeathersButton`](struct.FeathersButton.html "struct bevy::feathers::controls::FeathersButton") scene.

[FeathersCheckbox](struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox")

A checkbox widget.

[FeathersCheckboxProps](struct.FeathersCheckboxProps.html "struct bevy::feathers::controls::FeathersCheckboxProps")

Props used to construct a [`FeathersCheckbox`](struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox") scene.

[FeathersCheckboxTemplate](struct.FeathersCheckboxTemplate.html "struct bevy::feathers::controls::FeathersCheckboxTemplate")

[FeathersColorSlider](struct.FeathersColorSlider.html "struct bevy::feathers::controls::FeathersColorSlider")

A color slider widget.

[FeathersColorSliderProps](struct.FeathersColorSliderProps.html "struct bevy::feathers::controls::FeathersColorSliderProps")

Props used to construct a [`FeathersColorSlider`](struct.FeathersColorSlider.html "struct bevy::feathers::controls::FeathersColorSlider") scene.

[FeathersColorSwatch](struct.FeathersColorSwatch.html "struct bevy::feathers::controls::FeathersColorSwatch")

A color swatch widget.

[FeathersDisclosureToggle](struct.FeathersDisclosureToggle.html "struct bevy::feathers::controls::FeathersDisclosureToggle")

A toggle button which shows a chevron that points either right or down, used to expand or collapse a panel. Functionally, this is equivalent to a checkbox, and has a [`Checked`](../../ui/struct.Checked.html "struct bevy::ui::Checked") state.

[FeathersListRow](struct.FeathersListRow.html "struct bevy::feathers::controls::FeathersListRow")

A selectable row in a list of items

[FeathersListView](struct.FeathersListView.html "struct bevy::feathers::controls::FeathersListView")

A container that displays a scrolling list of items

[FeathersListViewProps](struct.FeathersListViewProps.html "struct bevy::feathers::controls::FeathersListViewProps")

Props used to construct a [`FeathersListView`](struct.FeathersListView.html "struct bevy::feathers::controls::FeathersListView") scene.

[FeathersMenu](struct.FeathersMenu.html "struct bevy::feathers::controls::FeathersMenu")

Top-level menu container. This wraps the menu button and provides an anchor for the popover.

[FeathersMenuButton](struct.FeathersMenuButton.html "struct bevy::feathers::controls::FeathersMenuButton")

A menu button widget. This produces a button that has a dropdown arrow.

[FeathersMenuButtonProps](struct.FeathersMenuButtonProps.html "struct bevy::feathers::controls::FeathersMenuButtonProps")

Props used to construct a [`FeathersMenuButton`](struct.FeathersMenuButton.html "struct bevy::feathers::controls::FeathersMenuButton") scene.

[FeathersMenuDivider](struct.FeathersMenuDivider.html "struct bevy::feathers::controls::FeathersMenuDivider")

A decorative divider between menu items

[FeathersMenuItem](struct.FeathersMenuItem.html "struct bevy::feathers::controls::FeathersMenuItem")

A menu item widget.

[FeathersMenuItemProps](struct.FeathersMenuItemProps.html "struct bevy::feathers::controls::FeathersMenuItemProps")

Props used to construct a [`FeathersMenuItem`](struct.FeathersMenuItem.html "struct bevy::feathers::controls::FeathersMenuItem") scene.

[FeathersMenuPopup](struct.FeathersMenuPopup.html "struct bevy::feathers::controls::FeathersMenuPopup")

A menu popup widget.

[FeathersNumberInput](struct.FeathersNumberInput.html "struct bevy::feathers::controls::FeathersNumberInput")

Widget that permits text entry of floating-point numbers. This widget implements two-way synchronization:

[FeathersNumberInputProps](struct.FeathersNumberInputProps.html "struct bevy::feathers::controls::FeathersNumberInputProps")

Props used to construct a [`FeathersNumberInput`](struct.FeathersNumberInput.html "struct bevy::feathers::controls::FeathersNumberInput") scene.

[FeathersRadio](struct.FeathersRadio.html "struct bevy::feathers::controls::FeathersRadio")

A radio widget.

[FeathersRadioProps](struct.FeathersRadioProps.html "struct bevy::feathers::controls::FeathersRadioProps")

Props used to construct a [`FeathersRadio`](struct.FeathersRadio.html "struct bevy::feathers::controls::FeathersRadio") scene.

[FeathersScrollbar](struct.FeathersScrollbar.html "struct bevy::feathers::controls::FeathersScrollbar")

A scrollbar. The `target` property should point to an entity whose [`ScrollPosition`](../../prelude/struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition") will be synchronized with the scrollbar.

[FeathersScrollbarProps](struct.FeathersScrollbarProps.html "struct bevy::feathers::controls::FeathersScrollbarProps")

Props used to construct a [`FeathersScrollbar`](struct.FeathersScrollbar.html "struct bevy::feathers::controls::FeathersScrollbar") scene.

[FeathersSlider](struct.FeathersSlider.html "struct bevy::feathers::controls::FeathersSlider")

A slider widget.

[FeathersSliderProps](struct.FeathersSliderProps.html "struct bevy::feathers::controls::FeathersSliderProps")

Props used to construct the [`FeathersSlider`](struct.FeathersSlider.html "struct bevy::feathers::controls::FeathersSlider") scene.

[FeathersTextInput](struct.FeathersTextInput.html "struct bevy::feathers::controls::FeathersTextInput")

Scene function to spawn a text input. For proper styling, this should be enclosed by a [`FeathersTextInputContainer`](struct.FeathersTextInputContainer.html "struct bevy::feathers::controls::FeathersTextInputContainer").

[FeathersTextInputContainer](struct.FeathersTextInputContainer.html "struct bevy::feathers::controls::FeathersTextInputContainer")

Decorative frame around a text input widget. This is a separate entity to allow icons (such as “search” or “clear”) to be inserted adjacent to the input.

[FeathersTextInputProps](struct.FeathersTextInputProps.html "struct bevy::feathers::controls::FeathersTextInputProps")

Props used to construct the [`FeathersTextInput`](struct.FeathersTextInput.html "struct bevy::feathers::controls::FeathersTextInput") scene.

[FeathersToggleSwitch](struct.FeathersToggleSwitch.html "struct bevy::feathers::controls::FeathersToggleSwitch")

A toggle switch widget.

[FeathersToolButton](struct.FeathersToolButton.html "struct bevy::feathers::controls::FeathersToolButton")

Tool button scene function: a smaller button for embedding in panel headers.

[ListViewPlugin](struct.ListViewPlugin.html "struct bevy::feathers::controls::ListViewPlugin")

Plugin which registers the systems for updating the listrow styles.

[MenuPlugin](struct.MenuPlugin.html "struct bevy::feathers::controls::MenuPlugin")

Plugin which registers the systems for updating the menu and menu button styles.

[RadioPlugin](struct.RadioPlugin.html "struct bevy::feathers::controls::RadioPlugin")

Plugin which registers the systems for updating the radio styles.

[ScrollbarPlugin](struct.ScrollbarPlugin.html "struct bevy::feathers::controls::ScrollbarPlugin")

Plugin which registers the systems for updating the scrollbar styles.

[SliderBaseColor](struct.SliderBaseColor.html "struct bevy::feathers::controls::SliderBaseColor")

Used to store the color channels that we are not editing: the components of the color that are constant for this slider.

[SliderPlugin](struct.SliderPlugin.html "struct bevy::feathers::controls::SliderPlugin")

Plugin which registers the systems for updating the slider styles.

[TextInputPlugin](struct.TextInputPlugin.html "struct bevy::feathers::controls::TextInputPlugin")

Plugin which registers the systems for updating the text input styles.

[ToggleSwitchPlugin](struct.ToggleSwitchPlugin.html "struct bevy::feathers::controls::ToggleSwitchPlugin")

Plugin which registers the systems for updating the toggle switch styles.

[UpdateNumberInput](struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput")

Event which can be sent to the number input widget to update the displayed value.

[VirtualKeyPressed](struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")

Fired whenever a virtual key is pressed.

[VirtualKeyboard](struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")

A virtual keyboard widget.

[VirtualKeyboardProps](struct.VirtualKeyboardProps.html "struct bevy::feathers::controls::VirtualKeyboardProps")

Props used to construct the [`VirtualKeyboard`](struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard") scene.

[VirtualKeyboardTemplate](struct.VirtualKeyboardTemplate.html "struct bevy::feathers::controls::VirtualKeyboardTemplate")

## Enums

[ButtonVariant](enum.ButtonVariant.html "enum bevy::feathers::controls::ButtonVariant")

Color variants for buttons. This also functions as a component used by the dynamic styling system to identify which entities are buttons.

[ColorChannel](enum.ColorChannel.html "enum bevy::feathers::controls::ColorChannel")

Indicates which color channel we want to edit.

[FeathersColorPlane](enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane")

A “color plane” widget, which is a 2d picker that allows selecting two components of a color space.

[FeathersColorPlaneTemplate](enum.FeathersColorPlaneTemplate.html "enum bevy::feathers::controls::FeathersColorPlaneTemplate")

[NumberFormat](enum.NumberFormat.html "enum bevy::feathers::controls::NumberFormat")

Used to indicate what format of numbers we are editing. This primarily affects the type of [`ValueChange`](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") event that is emitted.

[NumberInputValue](enum.NumberInputValue.html "enum bevy::feathers::controls::NumberInputValue")

Represents numbers in different formats.

## Functions

[button\_bundle](fn.button_bundle.html "fn bevy::feathers::controls::button_bundle")Deprecated

Template function to spawn a button.

[checkbox\_bundle](fn.checkbox_bundle.html "fn bevy::feathers::controls::checkbox_bundle")Deprecated

Template function to spawn a checkbox.

[color\_plane\_bundle](fn.color_plane_bundle.html "fn bevy::feathers::controls::color_plane_bundle")Deprecated

Template function to spawn a “color plane”, which is a 2d picker that allows selecting two components of a color space.

[color\_slider\_bundle](fn.color_slider_bundle.html "fn bevy::feathers::controls::color_slider_bundle")Deprecated

Spawn a new slider widget.

[color\_swatch\_bundle](fn.color_swatch_bundle.html "fn bevy::feathers::controls::color_swatch_bundle")Deprecated

Template function to spawn a color swatch.

[radio\_bundle](fn.radio_bundle.html "fn bevy::feathers::controls::radio_bundle")Deprecated

Template function to spawn a radio.

[slider\_bundle](fn.slider_bundle.html "fn bevy::feathers::controls::slider_bundle")Deprecated

Spawn a new slider widget.

[toggle\_switch\_bundle](fn.toggle_switch_bundle.html "fn bevy::feathers::controls::toggle_switch_bundle")Deprecated

Template function to spawn a toggle switch.

[virtual\_keyboard\_bundle](fn.virtual_keyboard_bundle.html "fn bevy::feathers::controls::virtual_keyboard_bundle")Deprecated

Function to spawn a virtual keyboard