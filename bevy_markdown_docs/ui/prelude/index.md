[bevy](../../index.html)::[ui](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#56)

The UI prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[AccessibleLabel](struct.AccessibleLabel.html "struct bevy::ui::prelude::AccessibleLabel")

A component which permits the a11y label to be specified independently from other a11y attributes.

[AngularColorStop](struct.AngularColorStop.html "struct bevy::ui::prelude::AngularColorStop")

An angular color stop for a conic gradient

[BackgroundColor](struct.BackgroundColor.html "struct bevy::ui::prelude::BackgroundColor")

The background color of the node

[BackgroundGradient](struct.BackgroundGradient.html "struct bevy::ui::prelude::BackgroundGradient")

A UI node that displays a gradient

[BorderColor](struct.BorderColor.html "struct bevy::ui::prelude::BorderColor")

The border color of the UI node.

[BorderGradient](struct.BorderGradient.html "struct bevy::ui::prelude::BorderGradient")

A UI node border that displays a gradient

[BorderRadius](struct.BorderRadius.html "struct bevy::ui::prelude::BorderRadius")

Used to add rounded corners to a UI node. You can set a UI node to have uniformly rounded corners or specify different radii for each corner. If a given radius exceeds half the length of the smallest dimension between the node’s height or width, the radius will calculated as half the smallest dimension.

[BorderRect](struct.BorderRect.html "struct bevy::ui::prelude::BorderRect")

Defines border insets that shrink a rectangle from its minimum and maximum corners.

[BoxShadow](struct.BoxShadow.html "struct bevy::ui::prelude::BoxShadow")

List of shadows to draw for a [`Node`](../../prelude/struct.Node.html "struct bevy::prelude::Node").

[Button](struct.Button.html "struct bevy::ui::prelude::Button")

Marker struct for buttons

[CalculatedClip](struct.CalculatedClip.html "struct bevy::ui::prelude::CalculatedClip")

The calculated clip of the node

[ColorStop](struct.ColorStop.html "struct bevy::ui::prelude::ColorStop")

A color stop for a gradient

[ComputedNode](struct.ComputedNode.html "struct bevy::ui::prelude::ComputedNode")

Provides the computed size and layout properties of the node.

[ComputedUiRenderTargetInfo](struct.ComputedUiRenderTargetInfo.html "struct bevy::ui::prelude::ComputedUiRenderTargetInfo")

Derived information about the render target for this UI node.

[ComputedUiTargetCamera](struct.ComputedUiTargetCamera.html "struct bevy::ui::prelude::ComputedUiTargetCamera")

Derived information about the camera target for this UI node.

[ConicGradient](struct.ConicGradient.html "struct bevy::ui::prelude::ConicGradient")

A conic gradient

[DefaultUiCamera](struct.DefaultUiCamera.html "struct bevy::ui::prelude::DefaultUiCamera")

[GlobalZIndex](struct.GlobalZIndex.html "struct bevy::ui::prelude::GlobalZIndex")

`GlobalZIndex` allows a [`Node`](../../prelude/struct.Node.html "struct bevy::prelude::Node") entity anywhere in the UI hierarchy to escape the implicit draw ordering of the UI’s layout tree and be rendered above or below other UI nodes. Nodes with a `GlobalZIndex` of greater than 0 will be drawn on top of nodes without a `GlobalZIndex` or nodes with a lower `GlobalZIndex`. Nodes with a `GlobalZIndex` of less than 0 will be drawn below nodes without a `GlobalZIndex` or nodes with a greater `GlobalZIndex`.

[GridPlacement](struct.GridPlacement.html "struct bevy::ui::prelude::GridPlacement")

Represents the position of a grid item in a single axis.

[GridTrack](struct.GridTrack.html "struct bevy::ui::prelude::GridTrack")

A [`GridTrack`](../../prelude/struct.GridTrack.html "struct bevy::prelude::GridTrack") is a Row or Column of a CSS Grid. This struct specifies what size the track should be. See below for the different “track sizing functions” you can specify.

[IgnoreScroll](struct.IgnoreScroll.html "struct bevy::ui::prelude::IgnoreScroll")

Controls whether a UI element ignores its parent’s [`ScrollPosition`](../../prelude/struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition") along specific axes.

[ImageNode](struct.ImageNode.html "struct bevy::ui::prelude::ImageNode")

A UI Node that renders an image.

[IsDefaultUiCamera](struct.IsDefaultUiCamera.html "struct bevy::ui::prelude::IsDefaultUiCamera")

Marker used to identify default cameras, they will have priority over the [`PrimaryWindow`](../../window/struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow") camera.

[Label](struct.Label.html "struct bevy::ui::prelude::Label")

Marker struct for labels

[LayoutConfig](struct.LayoutConfig.html "struct bevy::ui::prelude::LayoutConfig")

This component can be added to any UI node to modify its layout behavior.

[LinearGradient](struct.LinearGradient.html "struct bevy::ui::prelude::LinearGradient")

A linear gradient

[Node](struct.Node.html "struct bevy::ui::prelude::Node")

The base component for UI entities. It describes UI layout and style properties.

[OuterColor](struct.OuterColor.html "struct bevy::ui::prelude::OuterColor")

Sets a color to fill the regions outside the Node’s border created when a border radius is set.

[Outline](struct.Outline.html "struct bevy::ui::prelude::Outline")

The [`Outline`](../../prelude/struct.Outline.html "struct bevy::prelude::Outline") component adds an outline outside the edge of a UI node. Outlines do not take up space in the layout.

[Overflow](struct.Overflow.html "struct bevy::ui::prelude::Overflow")

Whether to show or hide overflowing items

[OverflowClipMargin](struct.OverflowClipMargin.html "struct bevy::ui::prelude::OverflowClipMargin")

The bounds of the visible area when a UI node is clipped.

[OverrideClip](struct.OverrideClip.html "struct bevy::ui::prelude::OverrideClip")

UI node entities with this component will ignore any clipping rect they inherit, the node will not be clipped regardless of its ancestors’ `Overflow` setting.

[RadialGradient](struct.RadialGradient.html "struct bevy::ui::prelude::RadialGradient")

A radial gradient

[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::ui::prelude::RepeatedGridTrack")

Represents a _possibly_ repeated [`GridTrack`](../../prelude/struct.GridTrack.html "struct bevy::prelude::GridTrack").

[ResolvedBorderRadius](struct.ResolvedBorderRadius.html "struct bevy::ui::prelude::ResolvedBorderRadius")

Represents the resolved border radius values for a UI node.

[ScrollPosition](struct.ScrollPosition.html "struct bevy::ui::prelude::ScrollPosition")

The scroll position of the node. Values are in logical pixels, increasing from top-left to bottom-right.

[ShadowStyle](struct.ShadowStyle.html "struct bevy::ui::prelude::ShadowStyle")

[Text](struct.Text.html "struct bevy::ui::prelude::Text")

The top-level UI text component.

[TextBackgroundColor](struct.TextBackgroundColor.html "struct bevy::ui::prelude::TextBackgroundColor")

The background color of the text for this section.

[TextShadow](struct.TextShadow.html "struct bevy::ui::prelude::TextShadow")

Adds a shadow behind text

[TextureSlicer](struct.TextureSlicer.html "struct bevy::ui::prelude::TextureSlicer")

Slices a texture using the **9-slicing** technique. This allows to reuse an image at various sizes without needing to prepare multiple assets. The associated texture will be split into nine portions, so that on resize the different portions scale or tile in different ways to keep the texture in proportion.

[UiGlobalTransform](struct.UiGlobalTransform.html "struct bevy::ui::prelude::UiGlobalTransform")

Absolute 2D transform for UI nodes

[UiPickingCamera](struct.UiPickingCamera.html "struct bevy::ui::prelude::UiPickingCamera")

An optional component that marks cameras that should be used in the [`UiPickingPlugin`](../../prelude/struct.UiPickingPlugin.html "struct bevy::prelude::UiPickingPlugin").

[UiPickingPlugin](struct.UiPickingPlugin.html "struct bevy::ui::prelude::UiPickingPlugin")

A plugin that adds picking support for UI nodes.

[UiPickingSettings](struct.UiPickingSettings.html "struct bevy::ui::prelude::UiPickingSettings")

Runtime settings for the [`UiPickingPlugin`](../../prelude/struct.UiPickingPlugin.html "struct bevy::prelude::UiPickingPlugin").

[UiPosition](struct.UiPosition.html "struct bevy::ui::prelude::UiPosition")

Responsive position relative to a UI node.

[UiRect](struct.UiRect.html "struct bevy::ui::prelude::UiRect")

A type which is commonly used to define margins, paddings and borders.

[UiScale](struct.UiScale.html "struct bevy::ui::prelude::UiScale")

The current scale of the UI.

[UiTargetCamera](struct.UiTargetCamera.html "struct bevy::ui::prelude::UiTargetCamera")

Indicates that this root [`Node`](../../prelude/struct.Node.html "struct bevy::prelude::Node") entity should be rendered to a specific camera.

[UiTransform](struct.UiTransform.html "struct bevy::ui::prelude::UiTransform")

Relative 2D transform for UI nodes

[Val2](struct.Val2.html "struct bevy::ui::prelude::Val2")

A pair of [`Val`](../../prelude/enum.Val.html "enum bevy::prelude::Val")s used to represent a 2-dimensional size or offset.

[ViewportNode](struct.ViewportNode.html "struct bevy::ui::prelude::ViewportNode")

Component used to render a [`RenderTarget`](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget") to a node.

[ZIndex](struct.ZIndex.html "struct bevy::ui::prelude::ZIndex")

Indicates that this [`Node`](../../prelude/struct.Node.html "struct bevy::prelude::Node") entity’s front-to-back ordering is not controlled solely by its location in the UI hierarchy. A node with a higher z-index will appear on top of sibling nodes with a lower z-index.

## Enums

[AlignContent](enum.AlignContent.html "enum bevy::ui::prelude::AlignContent")

Used to control how items are distributed.

[AlignItems](enum.AlignItems.html "enum bevy::ui::prelude::AlignItems")

Used to control how each individual item is aligned by default within the space they’re given.

[AlignSelf](enum.AlignSelf.html "enum bevy::ui::prelude::AlignSelf")

Used to control how the specified item is aligned within the space it’s given.

[BoxSizing](enum.BoxSizing.html "enum bevy::ui::prelude::BoxSizing")

Which part of a Node’s box length styles like width and height control

[Display](enum.Display.html "enum bevy::ui::prelude::Display")

Defines the layout model used by this node.

[FlexDirection](enum.FlexDirection.html "enum bevy::ui::prelude::FlexDirection")

Defines how flexbox items are ordered within a flexbox

[FlexWrap](enum.FlexWrap.html "enum bevy::ui::prelude::FlexWrap")

Defines if flexbox items appear on a single line or on multiple lines

[Gradient](enum.Gradient.html "enum bevy::ui::prelude::Gradient")

[GridAutoFlow](enum.GridAutoFlow.html "enum bevy::ui::prelude::GridAutoFlow")

Controls whether grid items are placed row-wise or column-wise as well as whether the sparse or dense packing algorithm is used.

[GridPlacementError](enum.GridPlacementError.html "enum bevy::ui::prelude::GridPlacementError")

Errors that occur when setting constraints for a `GridPlacement`

[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::ui::prelude::GridTrackRepetition")

How many times to repeat a repeated grid track

[InlineDirection](enum.InlineDirection.html "enum bevy::ui::prelude::InlineDirection")

Sets the inline axis direction (LTR or RTL) used for layout.

[Interaction](enum.Interaction.html "enum bevy::ui::prelude::Interaction")

Describes what type of input interaction has occurred for a UI node.

[InterpolationColorSpace](enum.InterpolationColorSpace.html "enum bevy::ui::prelude::InterpolationColorSpace")

The color space used for interpolation.

[JustifyContent](enum.JustifyContent.html "enum bevy::ui::prelude::JustifyContent")

Used to control how items are distributed.

[JustifyItems](enum.JustifyItems.html "enum bevy::ui::prelude::JustifyItems")

Used to control how each individual item is aligned by default within the space they’re given.

[JustifySelf](enum.JustifySelf.html "enum bevy::ui::prelude::JustifySelf")

Used to control how the specified item is aligned within the space it’s given.

[MaxTrackSizingFunction](enum.MaxTrackSizingFunction.html "enum bevy::ui::prelude::MaxTrackSizingFunction")

[MinTrackSizingFunction](enum.MinTrackSizingFunction.html "enum bevy::ui::prelude::MinTrackSizingFunction")

[NodeImageMode](enum.NodeImageMode.html "enum bevy::ui::prelude::NodeImageMode")

Controls how the image is altered to fit within the layout and how the layout algorithm determines the space in the layout for the image

[OverflowAxis](enum.OverflowAxis.html "enum bevy::ui::prelude::OverflowAxis")

Whether to show or hide overflowing items

[PositionType](enum.PositionType.html "enum bevy::ui::prelude::PositionType")

The strategy used to position this node

[RadialGradientShape](enum.RadialGradientShape.html "enum bevy::ui::prelude::RadialGradientShape")

[SliceScaleMode](enum.SliceScaleMode.html "enum bevy::ui::prelude::SliceScaleMode")

Defines how a texture slice scales when resized

[SpriteImageMode](enum.SpriteImageMode.html "enum bevy::ui::prelude::SpriteImageMode")

Controls how the image is altered when scaled.

[Val](enum.Val.html "enum bevy::ui::prelude::Val")

Represents the possible value types for layout properties.

[ValArithmeticError](enum.ValArithmeticError.html "enum bevy::ui::prelude::ValArithmeticError")

[ValParseError](enum.ValParseError.html "enum bevy::ui::prelude::ValParseError")

[VisualBox](enum.VisualBox.html "enum bevy::ui::prelude::VisualBox")

Used to determine which region of a UI node is used for visual bounds.

## Traits

[InColorSpace](trait.InColorSpace.html "trait bevy::ui::prelude::InColorSpace")

Set the color space used for interpolation.

[ValNum](trait.ValNum.html "trait bevy::ui::prelude::ValNum")

All the types that should be able to be used in the [`Val`](../../prelude/enum.Val.html "enum bevy::prelude::Val") enum should implement this trait.

## Functions

[auto](fn.auto.html "fn bevy::ui::prelude::auto")

Returns a [`Val::Auto`](../../prelude/enum.Val.html#variant.Auto "variant bevy::prelude::Val::Auto") where the value is automatically determined based on the context and other [`Node`](../../prelude/struct.Node.html "struct bevy::prelude::Node") properties.

[percent](fn.percent.html "fn bevy::ui::prelude::percent")

Returns a [`Val::Percent`](../../prelude/enum.Val.html#variant.Percent "variant bevy::prelude::Val::Percent") representing a percentage of the parent node’s length along a specific axis.

[px](fn.px.html "fn bevy::ui::prelude::px")

Returns a [`Val::Px`](../../prelude/enum.Val.html#variant.Px "variant bevy::prelude::Val::Px") representing a value in logical pixels.

[vh](fn.vh.html "fn bevy::ui::prelude::vh")

Returns a [`Val::Vh`](../../prelude/enum.Val.html#variant.Vh "variant bevy::prelude::Val::Vh") representing a percentage of the viewport height.

[vmax](fn.vmax.html "fn bevy::ui::prelude::vmax")

Returns a [`Val::VMax`](../../prelude/enum.Val.html#variant.VMax "variant bevy::prelude::Val::VMax") representing a percentage of the viewport’s larger dimension.

[vmin](fn.vmin.html "fn bevy::ui::prelude::vmin")

Returns a [`Val::VMin`](../../prelude/enum.Val.html#variant.VMin "variant bevy::prelude::Val::VMin") representing a percentage of the viewport’s smaller dimension.

[vw](fn.vw.html "fn bevy::ui::prelude::vw")

Returns a [`Val::Vw`](../../prelude/enum.Val.html#variant.Vw "variant bevy::prelude::Val::Vw") representing a percentage of the viewport width.

## Type Aliases

[TextUiReader](type.TextUiReader.html "type bevy::ui::prelude::TextUiReader")

UI alias for [`TextReader`](../../text/struct.TextReader.html "struct bevy::text::TextReader").

[TextUiWriter](type.TextUiWriter.html "type bevy::ui::prelude::TextUiWriter")

UI alias for [`TextWriter`](../../text/struct.TextWriter.html "struct bevy::text::TextWriter").