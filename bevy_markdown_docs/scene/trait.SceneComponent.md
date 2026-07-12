[bevy](../index.html)::[scene](index.html)

# Trait SceneComponent 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_component.rs.html#13)

```rust
pub trait SceneComponent: Component + FromTemplatewhere
    Self::Template: Default,{
    type Props: Default;

    // Required method
    fn scene(props: Self::Props) -> impl Scene;
}
```

Implemented for [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")s that have an associated [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"), which can be constructed with [`Self::Props`](../prelude/trait.SceneComponent.html#associatedtype.Props "associated type bevy::prelude::SceneComponent::Props").

In general, developers should not implement this manually. Instead, they should derive it, which also derives [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") and adds additional protections and assurances.

See the [“Scene Components”](index.html#scene-components "mod bevy::scene") section of the module docs to see how this is used in practice.

## Required Associated Types

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_component.rs.html#15)

#### type [Props](#associatedtype.Props): [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default")

The “properties” passed into [`Self::scene`](../prelude/trait.SceneComponent.html#tymethod.scene "associated function bevy::prelude::SceneComponent::scene") to build the final scene.

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_component.rs.html#18)

#### fn [scene](#tymethod.scene)(props: Self::[Props](../prelude/trait.SceneComponent.html#associatedtype.Props "type bevy::prelude::SceneComponent::Props")) -> impl [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene")

A function that uses the given `props` to produce a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene")

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#57)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersButton](../feathers/controls/struct.FeathersButton.html "struct bevy::feathers::controls::FeathersButton")

where [FeathersButton](../feathers/controls/struct.FeathersButton.html "struct bevy::feathers::controls::FeathersButton"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#57)

#### type [Props](#associatedtype.Props) = [FeathersButtonProps](../feathers/controls/struct.FeathersButtonProps.html "struct bevy::feathers::controls::FeathersButtonProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#46)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersCheckbox](../feathers/controls/struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox")

where [FeathersCheckbox](../feathers/controls/struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#46)

#### type [Props](#associatedtype.Props) = [FeathersCheckboxProps](../feathers/controls/struct.FeathersCheckboxProps.html "struct bevy::feathers::controls::FeathersCheckboxProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersColorPlane](../feathers/controls/enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane")

where [FeathersColorPlane](../feathers/controls/enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

#### type [Props](#associatedtype.Props) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#160)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersColorSlider](../feathers/controls/struct.FeathersColorSlider.html "struct bevy::feathers::controls::FeathersColorSlider")

where [FeathersColorSlider](../feathers/controls/struct.FeathersColorSlider.html "struct bevy::feathers::controls::FeathersColorSlider"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#160)

#### type [Props](#associatedtype.Props) = [FeathersColorSliderProps](../feathers/controls/struct.FeathersColorSliderProps.html "struct bevy::feathers::controls::FeathersColorSliderProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#27)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersColorSwatch](../feathers/controls/struct.FeathersColorSwatch.html "struct bevy::feathers::controls::FeathersColorSwatch")

where [FeathersColorSwatch](../feathers/controls/struct.FeathersColorSwatch.html "struct bevy::feathers::controls::FeathersColorSwatch"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#27)

#### type [Props](#associatedtype.Props) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/disclosure_toggle.rs.html#33)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersDisclosureToggle](../feathers/controls/struct.FeathersDisclosureToggle.html "struct bevy::feathers::controls::FeathersDisclosureToggle")

where [FeathersDisclosureToggle](../feathers/controls/struct.FeathersDisclosureToggle.html "struct bevy::feathers::controls::FeathersDisclosureToggle"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/disclosure_toggle.rs.html#33)

#### type [Props](#associatedtype.Props) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#106)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersListRow](../feathers/controls/struct.FeathersListRow.html "struct bevy::feathers::controls::FeathersListRow")

where [FeathersListRow](../feathers/controls/struct.FeathersListRow.html "struct bevy::feathers::controls::FeathersListRow"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#106)

#### type [Props](#associatedtype.Props) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#36)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersListView](../feathers/controls/struct.FeathersListView.html "struct bevy::feathers::controls::FeathersListView")

where [FeathersListView](../feathers/controls/struct.FeathersListView.html "struct bevy::feathers::controls::FeathersListView"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#36)

#### type [Props](#associatedtype.Props) = [FeathersListViewProps](../feathers/controls/struct.FeathersListViewProps.html "struct bevy::feathers::controls::FeathersListViewProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#48)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersMenu](../feathers/controls/struct.FeathersMenu.html "struct bevy::feathers::controls::FeathersMenu")

where [FeathersMenu](../feathers/controls/struct.FeathersMenu.html "struct bevy::feathers::controls::FeathersMenu"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#48)

#### type [Props](#associatedtype.Props) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#139)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersMenuButton](../feathers/controls/struct.FeathersMenuButton.html "struct bevy::feathers::controls::FeathersMenuButton")

where [FeathersMenuButton](../feathers/controls/struct.FeathersMenuButton.html "struct bevy::feathers::controls::FeathersMenuButton"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#139)

#### type [Props](#associatedtype.Props) = [FeathersMenuButtonProps](../feathers/controls/struct.FeathersMenuButtonProps.html "struct bevy::feathers::controls::FeathersMenuButtonProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#439)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersMenuDivider](../feathers/controls/struct.FeathersMenuDivider.html "struct bevy::feathers::controls::FeathersMenuDivider")

where [FeathersMenuDivider](../feathers/controls/struct.FeathersMenuDivider.html "struct bevy::feathers::controls::FeathersMenuDivider"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#439)

#### type [Props](#associatedtype.Props) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#248)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersMenuItem](../feathers/controls/struct.FeathersMenuItem.html "struct bevy::feathers::controls::FeathersMenuItem")

where [FeathersMenuItem](../feathers/controls/struct.FeathersMenuItem.html "struct bevy::feathers::controls::FeathersMenuItem"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#248)

#### type [Props](#associatedtype.Props) = [FeathersMenuItemProps](../feathers/controls/struct.FeathersMenuItemProps.html "struct bevy::feathers::controls::FeathersMenuItemProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#195)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersMenuPopup](../feathers/controls/struct.FeathersMenuPopup.html "struct bevy::feathers::controls::FeathersMenuPopup")

where [FeathersMenuPopup](../feathers/controls/struct.FeathersMenuPopup.html "struct bevy::feathers::controls::FeathersMenuPopup"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#195)

#### type [Props](#associatedtype.Props) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#53)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersNumberInput](../feathers/controls/struct.FeathersNumberInput.html "struct bevy::feathers::controls::FeathersNumberInput")

where [FeathersNumberInput](../feathers/controls/struct.FeathersNumberInput.html "struct bevy::feathers::controls::FeathersNumberInput"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#53)

#### type [Props](#associatedtype.Props) = [FeathersNumberInputProps](../feathers/controls/struct.FeathersNumberInputProps.html "struct bevy::feathers::controls::FeathersNumberInputProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/radio.rs.html#45)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersRadio](../feathers/controls/struct.FeathersRadio.html "struct bevy::feathers::controls::FeathersRadio")

where [FeathersRadio](../feathers/controls/struct.FeathersRadio.html "struct bevy::feathers::controls::FeathersRadio"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/radio.rs.html#45)

#### type [Props](#associatedtype.Props) = [FeathersRadioProps](../feathers/controls/struct.FeathersRadioProps.html "struct bevy::feathers::controls::FeathersRadioProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/scrollbar.rs.html#22)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersScrollbar](../feathers/controls/struct.FeathersScrollbar.html "struct bevy::feathers::controls::FeathersScrollbar")

where [FeathersScrollbar](../feathers/controls/struct.FeathersScrollbar.html "struct bevy::feathers::controls::FeathersScrollbar"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/scrollbar.rs.html#22)

#### type [Props](#associatedtype.Props) = [FeathersScrollbarProps](../feathers/controls/struct.FeathersScrollbarProps.html "struct bevy::feathers::controls::FeathersScrollbarProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/slider.rs.html#50)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersSlider](../feathers/controls/struct.FeathersSlider.html "struct bevy::feathers::controls::FeathersSlider")

where [FeathersSlider](../feathers/controls/struct.FeathersSlider.html "struct bevy::feathers::controls::FeathersSlider"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/slider.rs.html#50)

#### type [Props](#associatedtype.Props) = [FeathersSliderProps](../feathers/controls/struct.FeathersSliderProps.html "struct bevy::feathers::controls::FeathersSliderProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#83)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersTextInput](../feathers/controls/struct.FeathersTextInput.html "struct bevy::feathers::controls::FeathersTextInput")

where [FeathersTextInput](../feathers/controls/struct.FeathersTextInput.html "struct bevy::feathers::controls::FeathersTextInput"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#83)

#### type [Props](#associatedtype.Props) = [FeathersTextInputProps](../feathers/controls/struct.FeathersTextInputProps.html "struct bevy::feathers::controls::FeathersTextInputProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#38)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersTextInputContainer](../feathers/controls/struct.FeathersTextInputContainer.html "struct bevy::feathers::controls::FeathersTextInputContainer")

where [FeathersTextInputContainer](../feathers/controls/struct.FeathersTextInputContainer.html "struct bevy::feathers::controls::FeathersTextInputContainer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#38)

#### type [Props](#associatedtype.Props) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/toggle_switch.rs.html#42)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersToggleSwitch](../feathers/controls/struct.FeathersToggleSwitch.html "struct bevy::feathers::controls::FeathersToggleSwitch")

where [FeathersToggleSwitch](../feathers/controls/struct.FeathersToggleSwitch.html "struct bevy::feathers::controls::FeathersToggleSwitch"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/toggle_switch.rs.html#42)

#### type [Props](#associatedtype.Props) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#124)

### impl [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [FeathersToolButton](../feathers/controls/struct.FeathersToolButton.html "struct bevy::feathers::controls::FeathersToolButton")

where [FeathersToolButton](../feathers/controls/struct.FeathersToolButton.html "struct bevy::feathers::controls::FeathersToolButton"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#124)

#### type [Props](#associatedtype.Props) = [FeathersButtonProps](../feathers/controls/struct.FeathersButtonProps.html "struct bevy::feathers::controls::FeathersButtonProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#20)

### impl<T> [SceneComponent](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") for [VirtualKeyboard](../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, [VirtualKeyboard](../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#20)

#### type [Props](#associatedtype.Props) = [VirtualKeyboardProps](../feathers/controls/struct.VirtualKeyboardProps.html "struct bevy::feathers::controls::VirtualKeyboardProps")<T>