[bevy](../index.html)

# Crate ui\_widgets 

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#1-100)

This crate provides a set of standard widgets for Bevy UI, such as buttons, checkboxes, and sliders. These widgets have no inherent styling, it’s the responsibility of the user to add styling appropriate for their game or application.

### Warning: Experimental

This crate is currently experimental and under active development. The API is likely to change substantially: be prepared to migrate your code.

We are actively seeking feedback on the design and implementation of this crate, so please file issues or create PRs if you have any comments or suggestions.

### State Management

Most of the widgets use external state management: this means that the widgets do not automatically update their own internal state, but instead rely on the app to update the widget state (as well as any other related game state) in response to a change event emitted by the widget. The primary motivation for this is to avoid two-way data binding in scenarios where the user interface is showing a live view of dynamic data coming from deeper within the game engine.

### Best practices for event propagation

Generally, when a widget handles an event, propagation of that event to parent entities should be stopped. This is important when writing your custom widgets, and understanding the behavior of existing widgets.

For more guidance on this, see the documentation for [`EntityEvent`](../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent").

## Modules

[popover](popover/index.html "mod bevy::ui_widgets::popover")

Framework for positioning of popups, tooltips, and other popover UI elements.

## Structs

[Activate](struct.Activate.html "struct bevy::ui_widgets::Activate")

Notification sent by a button or menu item.

[ActivateOnPress](struct.ActivateOnPress.html "struct bevy::ui_widgets::ActivateOnPress")

Optional marker component that indicates we want the button to activate on the pointer down event, this is used for menu buttons.

[ActiveDescendant](struct.ActiveDescendant.html "struct bevy::ui_widgets::ActiveDescendant")

Component used for keyboard navigation. Individual rows should not be focusable in the normal way, as this would make tabbing through a long list tedious. Instead, we track the current “active” row separately using a component on the list box. The active row will be displayed with an outline.

[AddObserver](struct.AddObserver.html "struct bevy::ui_widgets::AddObserver")

Helper struct that adds an observer when inserted as a [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle").

[Button](struct.Button.html "struct bevy::ui_widgets::Button")

Headless button widget. This widget maintains a “pressed” state, which is used to indicate whether the button is currently being pressed by the user. It emits an [`Activate`](struct.Activate.html "struct bevy::ui_widgets::Activate") event when the button is un-pressed.

[ButtonPlugin](struct.ButtonPlugin.html "struct bevy::ui_widgets::ButtonPlugin")

Plugin that adds the observers for the [`Button`](struct.Button.html "struct bevy::ui_widgets::Button") widget.

[Checkbox](struct.Checkbox.html "struct bevy::ui_widgets::Checkbox")

Headless widget implementation for checkboxes. The [`Checked`](../ui/struct.Checked.html "struct bevy::ui::Checked") component represents the current state of the checkbox. The widget will emit a [`ValueChange<bool>`](struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") event when clicked, or when the `Enter` or `Space` key is pressed while the checkbox is focused.

[CheckboxPlugin](struct.CheckboxPlugin.html "struct bevy::ui_widgets::CheckboxPlugin")

Plugin that adds the observers for the [`Checkbox`](struct.Checkbox.html "struct bevy::ui_widgets::Checkbox") widget.

[EditableTextInputPlugin](struct.EditableTextInputPlugin.html "struct bevy::ui_widgets::EditableTextInputPlugin")

Enables support for the [`EditableText`](../text/struct.EditableText.html "struct bevy::text::EditableText") widget.

[ListBox](struct.ListBox.html "struct bevy::ui_widgets::ListBox")

Headless widget implementation for a list box. This component contains multiple [`ListItem`](struct.ListItem.html "struct bevy::ui_widgets::ListItem") entities. It implements the tab navigation logic and keyboard shortcuts for list items.

[ListBoxMultiSelect](struct.ListBoxMultiSelect.html "struct bevy::ui_widgets::ListBoxMultiSelect")

Marker component that indicates we want to support multiple selection of list items.

[ListBoxPlugin](struct.ListBoxPlugin.html "struct bevy::ui_widgets::ListBoxPlugin")

Plugin that adds the observers for the [`ListBox`](struct.ListBox.html "struct bevy::ui_widgets::ListBox") widget.

[ListItem](struct.ListItem.html "struct bevy::ui_widgets::ListItem")

Headless widget implementation for listbox items. These should be enclosed within a [`ListBox`](struct.ListBox.html "struct bevy::ui_widgets::ListBox") widget, which is responsible for the mutual exclusion logic.

[MenuButton](struct.MenuButton.html "struct bevy::ui_widgets::MenuButton")

Headless menu button widget. This is meant to be combined with the `Button` component, and adds a few more key codes - arrow keys to open the popup.

[MenuEvent](struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent")

Event used to control the state of the open menu. This bubbles upwards from the menu items and the menu container, through the portal relation, and to the menu owner entity.

[MenuItem](struct.MenuItem.html "struct bevy::ui_widgets::MenuItem")

Component that defines a menu item.

[MenuPlugin](struct.MenuPlugin.html "struct bevy::ui_widgets::MenuPlugin")

Plugin that adds the observers for the [`MenuItem`](struct.MenuItem.html "struct bevy::ui_widgets::MenuItem") component.

[MenuPopup](struct.MenuPopup.html "struct bevy::ui_widgets::MenuPopup")

Component that defines a popup menu container.

[RadioButton](struct.RadioButton.html "struct bevy::ui_widgets::RadioButton")

Headless widget implementation for radio buttons. They can be used independently, but enclosing them in a [`RadioGroup`](struct.RadioGroup.html "struct bevy::ui_widgets::RadioGroup") widget allows them to behave as a single, mutually exclusive unit.

[RadioGroup](struct.RadioGroup.html "struct bevy::ui_widgets::RadioGroup")

Headless widget implementation for a “radio button group”. This component is used to group multiple [`RadioButton`](struct.RadioButton.html "struct bevy::ui_widgets::RadioButton") components together, allowing them to behave as a single unit. It implements the tab navigation logic and keyboard shortcuts for radio buttons.

[RadioGroupPlugin](struct.RadioGroupPlugin.html "struct bevy::ui_widgets::RadioGroupPlugin")

Plugin that adds the observers for [`RadioButton`](struct.RadioButton.html "struct bevy::ui_widgets::RadioButton") and [`RadioGroup`](struct.RadioGroup.html "struct bevy::ui_widgets::RadioGroup") widget.

[ScrollArea](struct.ScrollArea.html "struct bevy::ui_widgets::ScrollArea")

Marker component to enable trackpad / mouse wheel scrolling. This should be placed on an entity that has overflow: scroll.

[ScrollAreaPlugin](struct.ScrollAreaPlugin.html "struct bevy::ui_widgets::ScrollAreaPlugin")

Plugin that adds the observers for the [`ScrollArea`](struct.ScrollArea.html "struct bevy::ui_widgets::ScrollArea") widget.

[ScrollIntoView](struct.ScrollIntoView.html "struct bevy::ui_widgets::ScrollIntoView")

An event which indicates that we want to scroll the specified item into view (adjusting the scroll position of it’s parent).

[Scrollbar](struct.Scrollbar.html "struct bevy::ui_widgets::Scrollbar")

A headless scrollbar widget, which can be used to build custom scrollbars.

[ScrollbarDragState](struct.ScrollbarDragState.html "struct bevy::ui_widgets::ScrollbarDragState")

Component used to manage the state of a scrollbar during dragging. This component is inserted on the thumb entity.

[ScrollbarPlugin](struct.ScrollbarPlugin.html "struct bevy::ui_widgets::ScrollbarPlugin")

Plugin that adds the observers for the [`Scrollbar`](struct.Scrollbar.html "struct bevy::ui_widgets::Scrollbar") widget.

[ScrollbarTemplate](struct.ScrollbarTemplate.html "struct bevy::ui_widgets::ScrollbarTemplate")

[ScrollbarThumb](struct.ScrollbarThumb.html "struct bevy::ui_widgets::ScrollbarThumb")

This component indicates that the entity is a scrollbar thumb (the moving, draggable part of the scrollbar). This should be a child of the scrollbar entity.

[SelectAllOnFocus](struct.SelectAllOnFocus.html "struct bevy::ui_widgets::SelectAllOnFocus")

Marker component for [`EditableText`](../text/struct.EditableText.html "struct bevy::text::EditableText") widgets that should select all text on focus.

[SetChecked](struct.SetChecked.html "struct bevy::ui_widgets::SetChecked")

Event which can be triggered on a checkbox to set the checked state. This can be used to control the checkbox via gamepad buttons or other inputs.

[SetSliderValue](struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue")

An [`EntityEvent`](../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") that can be triggered on a slider to modify its value (it will actually trigger a [`ValueChange`](struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") event, hooking up a corresponding change to [`SliderValue`](struct.SliderValue.html "struct bevy::ui_widgets::SliderValue") is still the app’s responsibility, see [`slider_self_update`](fn.slider_self_update.html "fn bevy::ui_widgets::slider_self_update")). This can be used to control the slider via gamepad buttons or other inputs. The value will be clamped when the event is processed.

[Slider](struct.Slider.html "struct bevy::ui_widgets::Slider")

A headless slider widget, which can be used to build custom sliders. Sliders have a value (represented by the [`SliderValue`](struct.SliderValue.html "struct bevy::ui_widgets::SliderValue") component) and a range (represented by [`SliderRange`](struct.SliderRange.html "struct bevy::ui_widgets::SliderRange")). An optional step size can be specified via [`SliderStep`](struct.SliderStep.html "struct bevy::ui_widgets::SliderStep"), and you can control the rounding during dragging with [`SliderPrecision`](struct.SliderPrecision.html "struct bevy::ui_widgets::SliderPrecision").

[SliderDragState](struct.SliderDragState.html "struct bevy::ui_widgets::SliderDragState")

Component used to manage the state of a slider during dragging.

[SliderPlugin](struct.SliderPlugin.html "struct bevy::ui_widgets::SliderPlugin")

Plugin that adds the observers for the [`Slider`](struct.Slider.html "struct bevy::ui_widgets::Slider") widget.

[SliderPrecision](struct.SliderPrecision.html "struct bevy::ui_widgets::SliderPrecision")

A component which controls the rounding of the slider value during dragging.

[SliderRange](struct.SliderRange.html "struct bevy::ui_widgets::SliderRange")

A component which represents the allowed range of the slider value. Defaults to 0.0..=1.0.

[SliderStep](struct.SliderStep.html "struct bevy::ui_widgets::SliderStep")

Defines the amount by which to increment or decrement the slider value when using keyboard shortcuts. Defaults to 1.0.

[SliderThumb](struct.SliderThumb.html "struct bevy::ui_widgets::SliderThumb")

Marker component that identifies which descendant element is the slider thumb.

[SliderValue](struct.SliderValue.html "struct bevy::ui_widgets::SliderValue")

A component which stores the current value of the slider.

[ToggleChecked](struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked")

Event which can be triggered on a checkbox to toggle the checked state. This can be used to control the checkbox via gamepad buttons or other inputs.

[UiWidgetsPlugins](struct.UiWidgetsPlugins.html "struct bevy::ui_widgets::UiWidgetsPlugins")

A plugin group that registers the observers for all of the widgets in this crate. If you don’t want to use all of the widgets, you can import the individual widget plugins instead.

[ValueChange](struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")

Notification sent by a widget that edits a scalar value.

## Enums

[ControlOrientation](enum.ControlOrientation.html "enum bevy::ui_widgets::ControlOrientation")

Used to select the orientation of a scrollbar, slider, or other oriented control.

[ImeSystems](enum.ImeSystems.html "enum bevy::ui_widgets::ImeSystems")

System sets for IME-related systems used by [`EditableTextInputPlugin`](struct.EditableTextInputPlugin.html "struct bevy::ui_widgets::EditableTextInputPlugin").

[MenuAction](enum.MenuAction.html "enum bevy::ui_widgets::MenuAction")

Action type for [`MenuEvent`](struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent").

[MenuFocusState](enum.MenuFocusState.html "enum bevy::ui_widgets::MenuFocusState")

Component used to manage focus on the popup. Menu popups remain open only so long as they contain focus.

[MenuLayout](enum.MenuLayout.html "enum bevy::ui_widgets::MenuLayout")

Specifies the layout direction of the menu, for keyboard navigation

[SliderOrientation](enum.SliderOrientation.html "enum bevy::ui_widgets::SliderOrientation")

Controls the orientation of the slider.

[SliderValueChange](enum.SliderValueChange.html "enum bevy::ui_widgets::SliderValueChange")

The type of slider value change to apply in [`SetSliderValue`](struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue").

[TrackClick](enum.TrackClick.html "enum bevy::ui_widgets::TrackClick")

Defines how the slider should behave when you click on the track (not the thumb).

## Functions

[checkbox\_self\_update](fn.checkbox_self_update.html "fn bevy::ui_widgets::checkbox_self_update")

Observer function which updates the checkbox value in response to a [`ValueChange`](struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") event. This can be used to make the checkbox automatically update its own state when clicked, as opposed to managing the checkbox state externally.

[listbox\_update\_selection](fn.listbox_update_selection.html "fn bevy::ui_widgets::listbox_update_selection")

Observer function for updating list row selection state.

[observe](fn.observe.html "fn bevy::ui_widgets::observe")

Adds an observer as a bundle effect.

[radio\_self\_update](fn.radio_self_update.html "fn bevy::ui_widgets::radio_self_update")

Observer function which updates the radio buttons in a group in response to a [`ValueChange`](struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") event. This can be used to make the radio buttons automatically update their own states and within the correct radio group when clicked, as opposed to managing the states externally.

[slider\_self\_update](fn.slider_self_update.html "fn bevy::ui_widgets::slider_self_update")

Observer function which updates the slider value in response to a [`ValueChange`](struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") event. This can be used to make the slider automatically update its own state when dragged, as opposed to managing the slider state externally.