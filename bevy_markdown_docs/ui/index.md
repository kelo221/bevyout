[bevy](../index.html)

# Crate ui 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#1-302)

This crate contains Bevy’s UI system, which can be used to create UI for both 2D and 3D games

## Basic usage

Spawn UI elements with [`widget::Button`](../prelude/struct.Button.html "struct bevy::prelude::Button"), [`ImageNode`](../prelude/struct.ImageNode.html "struct bevy::prelude::ImageNode"), [`Text`](../prelude/struct.Text.html "struct bevy::prelude::Text") and [`Node`](../prelude/struct.Node.html "struct bevy::prelude::Node") This UI is laid out with the Flexbox and CSS Grid layout models (see [https://cssreference.io/flexbox/](https://cssreference.io/flexbox/))

## Modules

[auto\_directional\_navigation](auto_directional_navigation/index.html "mod bevy::ui::auto_directional_navigation")

An automatic directional navigation system, powered by the [`AutoDirectionalNavigation`](auto_directional_navigation/struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation") component.

[debug](debug/index.html "mod bevy::ui::debug")

[experimental](experimental/index.html "mod bevy::ui::experimental")

Experimental features are not yet stable and may change or be removed in the future.

[gradients](gradients/index.html "mod bevy::ui::gradients")

[interaction\_states](interaction_states/index.html "mod bevy::ui::interaction_states")

[measurement](measurement/index.html "mod bevy::ui::measurement")

[picking\_backend](picking_backend/index.html "mod bevy::ui::picking_backend")`bevy_picking`

A picking backend for UI nodes.

[prelude](prelude/index.html "mod bevy::ui::prelude")

The UI prelude.

[ui\_surface](ui_surface/index.html "mod bevy::ui::ui_surface")

[ui\_transform](ui_transform/index.html "mod bevy::ui::ui_transform")

[update](update/index.html "mod bevy::ui::update")

This module contains systems that update the UI when something changes

[widget](widget/index.html "mod bevy::ui::widget")

This module contains the basic building blocks of Bevy’s UI

## Structs

[AngularColorStop](struct.AngularColorStop.html "struct bevy::ui::AngularColorStop")

An angular color stop for a conic gradient

[BackgroundColor](struct.BackgroundColor.html "struct bevy::ui::BackgroundColor")

The background color of the node

[BackgroundGradient](struct.BackgroundGradient.html "struct bevy::ui::BackgroundGradient")

A UI node that displays a gradient

[BorderColor](struct.BorderColor.html "struct bevy::ui::BorderColor")

The border color of the UI node.

[BorderGradient](struct.BorderGradient.html "struct bevy::ui::BorderGradient")

A UI node border that displays a gradient

[BorderRadius](struct.BorderRadius.html "struct bevy::ui::BorderRadius")

Used to add rounded corners to a UI node. You can set a UI node to have uniformly rounded corners or specify different radii for each corner. If a given radius exceeds half the length of the smallest dimension between the node’s height or width, the radius will calculated as half the smallest dimension.

[BoxShadow](struct.BoxShadow.html "struct bevy::ui::BoxShadow")

List of shadows to draw for a [`Node`](../prelude/struct.Node.html "struct bevy::prelude::Node").

[CalculatedClip](struct.CalculatedClip.html "struct bevy::ui::CalculatedClip")

The calculated clip of the node

[Checkable](struct.Checkable.html "struct bevy::ui::Checkable")

Component that indicates that a widget can be checked.

[Checked](struct.Checked.html "struct bevy::ui::Checked")

Component that indicates whether a checkbox or radio button is in a checked state.

[ColorStop](struct.ColorStop.html "struct bevy::ui::ColorStop")

A color stop for a gradient

[ComputedNode](struct.ComputedNode.html "struct bevy::ui::ComputedNode")

Provides the computed size and layout properties of the node.

[ComputedStackIndex](struct.ComputedStackIndex.html "struct bevy::ui::ComputedStackIndex")

The order of the node in the UI layout. Nodes with a higher stack index are drawn on top of and receive interactions before nodes with lower stack indices.

[ComputedUiRenderTargetInfo](struct.ComputedUiRenderTargetInfo.html "struct bevy::ui::ComputedUiRenderTargetInfo")

Derived information about the render target for this UI node.

[ComputedUiTargetCamera](struct.ComputedUiTargetCamera.html "struct bevy::ui::ComputedUiTargetCamera")

Derived information about the camera target for this UI node.

[ConicGradient](struct.ConicGradient.html "struct bevy::ui::ConicGradient")

A conic gradient

[ContentSize](struct.ContentSize.html "struct bevy::ui::ContentSize")

A node with a `ContentSize` component is a node where its size is based on its content.

[DefaultUiCamera](struct.DefaultUiCamera.html "struct bevy::ui::DefaultUiCamera")

[FixedMeasure](struct.FixedMeasure.html "struct bevy::ui::FixedMeasure")

A `FixedMeasure` is a `Measure` that ignores all constraints and always returns the same size.

[GlobalZIndex](struct.GlobalZIndex.html "struct bevy::ui::GlobalZIndex")

`GlobalZIndex` allows a [`Node`](../prelude/struct.Node.html "struct bevy::prelude::Node") entity anywhere in the UI hierarchy to escape the implicit draw ordering of the UI’s layout tree and be rendered above or below other UI nodes. Nodes with a `GlobalZIndex` of greater than 0 will be drawn on top of nodes without a `GlobalZIndex` or nodes with a lower `GlobalZIndex`. Nodes with a `GlobalZIndex` of less than 0 will be drawn below nodes without a `GlobalZIndex` or nodes with a greater `GlobalZIndex`.

[GridPlacement](struct.GridPlacement.html "struct bevy::ui::GridPlacement")

Represents the position of a grid item in a single axis.

[GridTrack](struct.GridTrack.html "struct bevy::ui::GridTrack")

A [`GridTrack`](../prelude/struct.GridTrack.html "struct bevy::prelude::GridTrack") is a Row or Column of a CSS Grid. This struct specifies what size the track should be. See below for the different “track sizing functions” you can specify.

[IgnoreScroll](struct.IgnoreScroll.html "struct bevy::ui::IgnoreScroll")

Controls whether a UI element ignores its parent’s [`ScrollPosition`](../prelude/struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition") along specific axes.

[InteractionDisabled](struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled")

A component indicating that a widget is disabled and should be “grayed out”. This is used to prevent user interaction with the widget. It should not, however, prevent the widget from being updated or rendered, or from acquiring keyboard focus.

[IsDefaultUiCamera](struct.IsDefaultUiCamera.html "struct bevy::ui::IsDefaultUiCamera")

Marker used to identify default cameras, they will have priority over the [`PrimaryWindow`](../window/struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow") camera.

[LayoutConfig](struct.LayoutConfig.html "struct bevy::ui::LayoutConfig")

This component can be added to any UI node to modify its layout behavior.

[LayoutContext](struct.LayoutContext.html "struct bevy::ui::LayoutContext")

[LinearGradient](struct.LinearGradient.html "struct bevy::ui::LinearGradient")

A linear gradient

[MeasureArgs](struct.MeasureArgs.html "struct bevy::ui::MeasureArgs")

Inputs provided to [`Measure::measure`](trait.Measure.html#tymethod.measure "method bevy::ui::Measure::measure").

[Node](struct.Node.html "struct bevy::ui::Node")

The base component for UI entities. It describes UI layout and style properties.

[NodeQuery](struct.NodeQuery.html "struct bevy::ui::NodeQuery")

Main query for [`ui_focus_system`](fn.ui_focus_system.html "fn bevy::ui::ui_focus_system")

[NodeQueryItem](struct.NodeQueryItem.html "struct bevy::ui::NodeQueryItem")

Automatically generated [`WorldQuery`](../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") item type for [`NodeQuery`](struct.NodeQuery.html "struct bevy::ui::NodeQuery"), returned when iterating over query results.

[NodeQueryReadOnly](struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

Automatically generated [`WorldQuery`](../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") type for a read-only variant of [`NodeQuery`](struct.NodeQuery.html "struct bevy::ui::NodeQuery").

[NodeQueryReadOnlyItem](struct.NodeQueryReadOnlyItem.html "struct bevy::ui::NodeQueryReadOnlyItem")

Automatically generated [`WorldQuery`](../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") item type for [`NodeQueryReadOnly`](struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly"), returned when iterating over query results.

[OuterColor](struct.OuterColor.html "struct bevy::ui::OuterColor")

Sets a color to fill the regions outside the Node’s border created when a border radius is set.

[Outline](struct.Outline.html "struct bevy::ui::Outline")

The [`Outline`](../prelude/struct.Outline.html "struct bevy::prelude::Outline") component adds an outline outside the edge of a UI node. Outlines do not take up space in the layout.

[Overflow](struct.Overflow.html "struct bevy::ui::Overflow")

Whether to show or hide overflowing items

[OverflowClipMargin](struct.OverflowClipMargin.html "struct bevy::ui::OverflowClipMargin")

The bounds of the visible area when a UI node is clipped.

[OverrideClip](struct.OverrideClip.html "struct bevy::ui::OverrideClip")

UI node entities with this component will ignore any clipping rect they inherit, the node will not be clipped regardless of its ancestors’ `Overflow` setting.

[Pressed](struct.Pressed.html "struct bevy::ui::Pressed")

Component that indicates whether a button or widget is currently in a pressed or “held down” state.

[RadialGradient](struct.RadialGradient.html "struct bevy::ui::RadialGradient")

A radial gradient

[RelativeCursorPosition](struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")

A component storing the position of the mouse relative to the node, (0., 0.) being the center and (0.5, 0.5) being the bottom-right If the mouse is not over the node, the value will go beyond the range of (-0.5, -0.5) to (0.5, 0.5)

[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::ui::RepeatedGridTrack")

Represents a _possibly_ repeated [`GridTrack`](../prelude/struct.GridTrack.html "struct bevy::prelude::GridTrack").

[ResolvedAxis](struct.ResolvedAxis.html "struct bevy::ui::ResolvedAxis")

Resolved values for per-axis size constraints.

[ResolvedBorderRadius](struct.ResolvedBorderRadius.html "struct bevy::ui::ResolvedBorderRadius")

Represents the resolved border radius values for a UI node.

[ScrollPosition](struct.ScrollPosition.html "struct bevy::ui::ScrollPosition")

The scroll position of the node. Values are in logical pixels, increasing from top-left to bottom-right.

[Selectable](struct.Selectable.html "struct bevy::ui::Selectable")

Component that indicates that a widget can be selected. Similar to [`Checkable`](struct.Checkable.html "struct bevy::ui::Checkable"), but works for the ARIA “selected” state instead of “checked”.

[Selected](struct.Selected.html "struct bevy::ui::Selected")

Similar to [`Checked`](struct.Checked.html "struct bevy::ui::Checked"), but works for the ARIA “selected” state instead of “checked”.

[ShadowStyle](struct.ShadowStyle.html "struct bevy::ui::ShadowStyle")

[State](struct.State.html "struct bevy::ui::State")

Contains entities whose Interaction should be set to None

[TextNodeFlags](struct.TextNodeFlags.html "struct bevy::ui::TextNodeFlags")

UI text system flags.

[UiGlobalTransform](struct.UiGlobalTransform.html "struct bevy::ui::UiGlobalTransform")

Absolute 2D transform for UI nodes

[UiPlugin](struct.UiPlugin.html "struct bevy::ui::UiPlugin")

The basic plugin for Bevy UI

[UiPosition](struct.UiPosition.html "struct bevy::ui::UiPosition")

Responsive position relative to a UI node.

[UiRect](struct.UiRect.html "struct bevy::ui::UiRect")

A type which is commonly used to define margins, paddings and borders.

[UiScale](struct.UiScale.html "struct bevy::ui::UiScale")

The current scale of the UI.

[UiStack](struct.UiStack.html "struct bevy::ui::UiStack")

The current UI stack, which contains all UI nodes ordered by their depth (back-to-front).

[UiTargetCamera](struct.UiTargetCamera.html "struct bevy::ui::UiTargetCamera")

Indicates that this root [`Node`](../prelude/struct.Node.html "struct bevy::prelude::Node") entity should be rendered to a specific camera.

[UiTransform](struct.UiTransform.html "struct bevy::ui::UiTransform")

Relative 2D transform for UI nodes

[Val2](struct.Val2.html "struct bevy::ui::Val2")

A pair of [`Val`](../prelude/enum.Val.html "enum bevy::prelude::Val")s used to represent a 2-dimensional size or offset.

[ZIndex](struct.ZIndex.html "struct bevy::ui::ZIndex")

Indicates that this [`Node`](../prelude/struct.Node.html "struct bevy::prelude::Node") entity’s front-to-back ordering is not controlled solely by its location in the UI hierarchy. A node with a higher z-index will appear on top of sibling nodes with a lower z-index.

## Enums

[AlignContent](enum.AlignContent.html "enum bevy::ui::AlignContent")

Used to control how items are distributed.

[AlignItems](enum.AlignItems.html "enum bevy::ui::AlignItems")

Used to control how each individual item is aligned by default within the space they’re given.

[AlignSelf](enum.AlignSelf.html "enum bevy::ui::AlignSelf")

Used to control how the specified item is aligned within the space it’s given.

[AvailableSpace](enum.AvailableSpace.html "enum bevy::ui::AvailableSpace")

The amount of space available to a node in a given axis [https://www.w3.org/TR/css-sizing-3/#available](https://www.w3.org/TR/css-sizing-3/#available)

[BoxSizing](enum.BoxSizing.html "enum bevy::ui::BoxSizing")

Which part of a Node’s box length styles like width and height control

[Display](enum.Display.html "enum bevy::ui::Display")

Defines the layout model used by this node.

[FlexDirection](enum.FlexDirection.html "enum bevy::ui::FlexDirection")

Defines how flexbox items are ordered within a flexbox

[FlexWrap](enum.FlexWrap.html "enum bevy::ui::FlexWrap")

Defines if flexbox items appear on a single line or on multiple lines

[FocusPolicy](enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")

Describes whether the node should block interactions with lower nodes

[Gradient](enum.Gradient.html "enum bevy::ui::Gradient")

[GridAutoFlow](enum.GridAutoFlow.html "enum bevy::ui::GridAutoFlow")

Controls whether grid items are placed row-wise or column-wise as well as whether the sparse or dense packing algorithm is used.

[GridPlacementError](enum.GridPlacementError.html "enum bevy::ui::GridPlacementError")

Errors that occur when setting constraints for a `GridPlacement`

[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::ui::GridTrackRepetition")

How many times to repeat a repeated grid track

[InlineDirection](enum.InlineDirection.html "enum bevy::ui::InlineDirection")

Sets the inline axis direction (LTR or RTL) used for layout.

[Interaction](enum.Interaction.html "enum bevy::ui::Interaction")

Describes what type of input interaction has occurred for a UI node.

[InterpolationColorSpace](enum.InterpolationColorSpace.html "enum bevy::ui::InterpolationColorSpace")

The color space used for interpolation.

[JustifyContent](enum.JustifyContent.html "enum bevy::ui::JustifyContent")

Used to control how items are distributed.

[JustifyItems](enum.JustifyItems.html "enum bevy::ui::JustifyItems")

Used to control how each individual item is aligned by default within the space they’re given.

[JustifySelf](enum.JustifySelf.html "enum bevy::ui::JustifySelf")

Used to control how the specified item is aligned within the space it’s given.

[LayoutError](enum.LayoutError.html "enum bevy::ui::LayoutError")

[MaxTrackSizingFunction](enum.MaxTrackSizingFunction.html "enum bevy::ui::MaxTrackSizingFunction")

[MinTrackSizingFunction](enum.MinTrackSizingFunction.html "enum bevy::ui::MinTrackSizingFunction")

[NodeMeasure](enum.NodeMeasure.html "enum bevy::ui::NodeMeasure")

A type to serve as Taffy’s node context (which allows the content size of leaf nodes to be computed)

[OverflowAxis](enum.OverflowAxis.html "enum bevy::ui::OverflowAxis")

Whether to show or hide overflowing items

[PositionType](enum.PositionType.html "enum bevy::ui::PositionType")

The strategy used to position this node

[RadialGradientShape](enum.RadialGradientShape.html "enum bevy::ui::RadialGradientShape")

[UiSystems](enum.UiSystems.html "enum bevy::ui::UiSystems")

The label enum labeling the types of systems in the Bevy UI

[Val](enum.Val.html "enum bevy::ui::Val")

Represents the possible value types for layout properties.

[ValArithmeticError](enum.ValArithmeticError.html "enum bevy::ui::ValArithmeticError")

[ValParseError](enum.ValParseError.html "enum bevy::ui::ValParseError")

[VisualBox](enum.VisualBox.html "enum bevy::ui::VisualBox")

Used to determine which region of a UI node is used for visual bounds.

## Traits

[InColorSpace](trait.InColorSpace.html "trait bevy::ui::InColorSpace")

Set the color space used for interpolation.

[Measure](trait.Measure.html "trait bevy::ui::Measure")

A `Measure` is used to compute the size of a ui node when the size of that node is based on its content.

[ValNum](trait.ValNum.html "trait bevy::ui::ValNum")

All the types that should be able to be used in the [`Val`](../prelude/enum.Val.html "enum bevy::prelude::Val") enum should implement this trait.

## Functions

[auto](fn.auto.html "fn bevy::ui::auto")

Returns a [`Val::Auto`](../prelude/enum.Val.html#variant.Auto "variant bevy::prelude::Val::Auto") where the value is automatically determined based on the context and other [`Node`](../prelude/struct.Node.html "struct bevy::prelude::Node") properties.

[clip\_check\_recursive](fn.clip_check_recursive.html "fn bevy::ui::clip_check_recursive")

Walk up the tree child-to-parent checking that `point` is not clipped by any ancestor node. If `entity` has an [`OverrideClip`](../prelude/struct.OverrideClip.html "struct bevy::prelude::OverrideClip") component it ignores any inherited clipping and returns true.

[percent](fn.percent.html "fn bevy::ui::percent")

Returns a [`Val::Percent`](../prelude/enum.Val.html#variant.Percent "variant bevy::prelude::Val::Percent") representing a percentage of the parent node’s length along a specific axis.

[px](fn.px.html "fn bevy::ui::px")

Returns a [`Val::Px`](../prelude/enum.Val.html#variant.Px "variant bevy::prelude::Val::Px") representing a value in logical pixels.

[ui\_focus\_system](fn.ui_focus_system.html "fn bevy::ui::ui_focus_system")

The system that sets Interaction for all UI elements based on the mouse cursor activity

[ui\_layout\_system](fn.ui_layout_system.html "fn bevy::ui::ui_layout_system")

Updates the UI’s layout tree, computes the new layout geometry and then updates the sizes and transforms of all the UI nodes.

[vh](fn.vh.html "fn bevy::ui::vh")

Returns a [`Val::Vh`](../prelude/enum.Val.html#variant.Vh "variant bevy::prelude::Val::Vh") representing a percentage of the viewport height.

[vmax](fn.vmax.html "fn bevy::ui::vmax")

Returns a [`Val::VMax`](../prelude/enum.Val.html#variant.VMax "variant bevy::prelude::Val::VMax") representing a percentage of the viewport’s larger dimension.

[vmin](fn.vmin.html "fn bevy::ui::vmin")

Returns a [`Val::VMin`](../prelude/enum.Val.html#variant.VMin "variant bevy::prelude::Val::VMin") representing a percentage of the viewport’s smaller dimension.

[vw](fn.vw.html "fn bevy::ui::vw")

Returns a [`Val::Vw`](../prelude/enum.Val.html#variant.Vw "variant bevy::prelude::Val::Vw") representing a percentage of the viewport width.