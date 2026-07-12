[bevy](../../index.html)::[ui](../index.html)

# Module widget 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#19)

This module contains the basic building blocks of Bevy’s UI

## Structs

[Button](struct.Button.html "struct bevy::ui::widget::Button")

Marker struct for buttons

[ImageMeasure](struct.ImageMeasure.html "struct bevy::ui::widget::ImageMeasure")

Used to calculate the size of UI image nodes

[ImageNode](struct.ImageNode.html "struct bevy::ui::widget::ImageNode")

A UI Node that renders an image.

[ImageNodeSize](struct.ImageNodeSize.html "struct bevy::ui::widget::ImageNodeSize")

The size of the image’s texture

[ImageNodeTemplate](struct.ImageNodeTemplate.html "struct bevy::ui::widget::ImageNodeTemplate")

[Label](struct.Label.html "struct bevy::ui::widget::Label")

Marker struct for labels

[Text](struct.Text.html "struct bevy::ui::widget::Text")

The top-level UI text component.

[TextMeasure](struct.TextMeasure.html "struct bevy::ui::widget::TextMeasure")

Text measurement for UI layout. See [`NodeMeasure`](../enum.NodeMeasure.html "enum bevy::ui::NodeMeasure").

[TextNodeFlags](struct.TextNodeFlags.html "struct bevy::ui::widget::TextNodeFlags")

UI text system flags.

[TextScroll](struct.TextScroll.html "struct bevy::ui::widget::TextScroll")

[TextShadow](struct.TextShadow.html "struct bevy::ui::widget::TextShadow")

Adds a shadow behind text

[ViewportNode](struct.ViewportNode.html "struct bevy::ui::widget::ViewportNode")

Component used to render a [`RenderTarget`](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget") to a node.

## Enums

[NodeImageMode](enum.NodeImageMode.html "enum bevy::ui::widget::NodeImageMode")

Controls how the image is altered to fit within the layout and how the layout algorithm determines the space in the layout for the image

## Functions

[measure\_text\_system](fn.measure_text_system.html "fn bevy::ui::widget::measure_text_system")

Generates a new [`Measure`](../trait.Measure.html "trait bevy::ui::Measure") for a text node on changes to its [`Text`](../../prelude/struct.Text.html "struct bevy::prelude::Text") component.

[scroll\_editable\_text](fn.scroll_editable_text.html "fn bevy::ui::widget::scroll_editable_text")

Scroll editable text to keep cursor in view after edits.

[text\_system](fn.text_system.html "fn bevy::ui::widget::text_system")

Updates the layout and size information for a UI text node on changes to the size value of its [`Node`](../../prelude/struct.Node.html "struct bevy::prelude::Node") component, or when the `needs_recompute` field of [`TextNodeFlags`](../struct.TextNodeFlags.html "struct bevy::ui::TextNodeFlags") is set to true. This information is computed by the [`TextPipeline`](../../text/struct.TextPipeline.html "struct bevy::text::TextPipeline") and then stored in [`TextLayoutInfo`](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo").

[update\_editable\_text\_content\_size](fn.update_editable_text_content_size.html "fn bevy::ui::widget::update_editable_text_content_size")

If `visible_lines` or `visible_width` are `Some`, sets a `ContentSize` that determines:

[update\_editable\_text\_layout](fn.update_editable_text_layout.html "fn bevy::ui::widget::update_editable_text_layout")

Refreshes the [`EditableText`](../../text/struct.EditableText.html "struct bevy::text::EditableText")’s layout if stale and then writes it it to [`TextLayoutInfo`](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo") for rendering and picking. Adds required glyphs to the texture atlas

[update\_editable\_text\_styles](fn.update_editable_text_styles.html "fn bevy::ui::widget::update_editable_text_styles")

Syncs each [`EditableText`](../../text/struct.EditableText.html "struct bevy::text::EditableText") entity’s [`PlainEditor`](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/editing/editor/struct.PlainEditor.html "struct parley::editing::editor::PlainEditor") style properties to match its [`TextFont`](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont"), [`LineHeight`](../../text/enum.LineHeight.html "enum bevy::text::LineHeight"), and [`TextLayout`](../../prelude/struct.TextLayout.html "struct bevy::prelude::TextLayout") components.

[update\_image\_content\_size\_system](fn.update_image_content_size_system.html "fn bevy::ui::widget::update_image_content_size_system")

Updates content size of the node based on the image provided

[update\_viewport\_render\_target\_size](fn.update_viewport_render_target_size.html "fn bevy::ui::widget::update_viewport_render_target_size")

Updates the size of the associated render target for viewports when the node size changes.

[viewport\_picking](fn.viewport_picking.html "fn bevy::ui::widget::viewport_picking")`bevy_picking`

Handles viewport picking logic.

## Type Aliases

[TextUiReader](type.TextUiReader.html "type bevy::ui::widget::TextUiReader")

UI alias for [`TextReader`](../../text/struct.TextReader.html "struct bevy::text::TextReader").

[TextUiWriter](type.TextUiWriter.html "type bevy::ui::widget::TextUiWriter")

UI alias for [`TextWriter`](../../text/struct.TextWriter.html "struct bevy::text::TextWriter").