[bevy](../index.html)

# Crate text 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/lib.rs.html#1-149)

This crate provides the tools for positioning and rendering text in Bevy.

## `Font`

Fonts contain information for drawing glyphs, which are shapes that typically represent a single character, but in some cases part of a “character” (grapheme clusters) or more than one character (ligatures).

A font _face_ is part of a font family, and is distinguished by its style (e.g. italic), its weight (e.g. bold) and its stretch (e.g. condensed).

In Bevy, [`Font`](../prelude/struct.Font.html "struct bevy::prelude::Font")s are loaded by the [`FontLoader`](struct.FontLoader.html "struct bevy::text::FontLoader") as [assets](../prelude/struct.AssetPlugin.html "struct bevy::prelude::AssetPlugin").

## `TextPipeline`

The [`TextPipeline`](struct.TextPipeline.html "struct bevy::text::TextPipeline") resource does all of the heavy lifting for rendering text.

UI `Text` is first measured by creating a [`TextMeasureInfo`](struct.TextMeasureInfo.html "struct bevy::text::TextMeasureInfo") in [`TextPipeline::create_text_measure`](struct.TextPipeline.html#method.create_text_measure "method bevy::text::TextPipeline::create_text_measure"), which is called by the `measure_text_system` system of `bevy_ui`.

Note that text measurement is only relevant in a UI context.

With the actual text bounds defined, the `bevy_ui::widget::text::text_system` system (in a UI context) or `bevy_sprite::text2d::update_text2d_layout` system (in a 2d world space context) passes it into [`TextPipeline::update_text_layout_info`](struct.TextPipeline.html#method.update_text_layout_info "method bevy::text::TextPipeline::update_text_layout_info"), which:

1.  updates a [`Layout`](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/layout/layout/struct.Layout.html "struct parley::layout::layout::Layout") from the [`TextSpan`](../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")s, generating new [`FontAtlas`](struct.FontAtlas.html "struct bevy::text::FontAtlas")es if necessary.
2.  iterates over each glyph in the [`Layout`](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/layout/layout/struct.Layout.html "struct parley::layout::layout::Layout") to create a [`PositionedGlyph`](struct.PositionedGlyph.html "struct bevy::text::PositionedGlyph"), retrieving glyphs from the cache, or rasterizing to a [`FontAtlas`](struct.FontAtlas.html "struct bevy::text::FontAtlas") if necessary.
3.  [`PositionedGlyph`](struct.PositionedGlyph.html "struct bevy::text::PositionedGlyph")s are stored in a [`TextLayoutInfo`](struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), which contains all the information that downstream systems need for rendering.

## Modules

[prelude](prelude/index.html "mod bevy::text::prelude")

The text prelude.

## Structs

[ComputedTextBlock](struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock")

Computed information for a text block.

[EditableText](struct.EditableText.html "struct bevy::text::EditableText")

A plain-text text input field.

[EditableTextFilter](struct.EditableTextFilter.html "struct bevy::text::EditableTextFilter")

Sets a per-character filter for this text input. Insert and paste edits are ignored if the filter rejects any character.

[EditableTextGeneration](struct.EditableTextGeneration.html "struct bevy::text::EditableTextGeneration")

Wrapper around a `parley::Generation`. Used to track when `TextLayoutInfo` is stale and needs reupdating. The initial `Generation` of the `PlainEditor` is not equal to the default `Generation` value, so the `TextLayoutInfo` will always be given an initial update.

[EditableTextSystems](struct.EditableTextSystems.html "struct bevy::text::EditableTextSystems")

System set where [`EditableText::pending_edits`](struct.EditableText.html#structfield.pending_edits "field bevy::text::EditableText::pending_edits") are applied.

[Font](struct.Font.html "struct bevy::text::Font")

An [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") that contains the data for a loaded font, if loaded as an asset.

[FontAtlas](struct.FontAtlas.html "struct bevy::text::FontAtlas")

Rasterized glyphs are cached, stored in, and retrieved from, a `FontAtlas`.

[FontAtlasKey](struct.FontAtlasKey.html "struct bevy::text::FontAtlasKey")

Identifies the font atlases for a particular font in [`FontAtlasSet`](struct.FontAtlasSet.html "struct bevy::text::FontAtlasSet")

[FontAtlasSet](struct.FontAtlasSet.html "struct bevy::text::FontAtlasSet")

Set of rasterized fonts stored in [`FontAtlas`](struct.FontAtlas.html "struct bevy::text::FontAtlas")es.

[FontCx](struct.FontCx.html "struct bevy::text::FontCx")

A font database and cache, used for font family resolution and text layout.

[FontFeatureTag](struct.FontFeatureTag.html "struct bevy::text::FontFeatureTag")

An OpenType font feature tag.

[FontFeatures](struct.FontFeatures.html "struct bevy::text::FontFeatures")

OpenType features for .otf fonts that support them.

[FontFeaturesBuilder](struct.FontFeaturesBuilder.html "struct bevy::text::FontFeaturesBuilder")

A builder for [`FontFeatures`](struct.FontFeatures.html "struct bevy::text::FontFeatures").

[FontLoader](struct.FontLoader.html "struct bevy::text::FontLoader")

An [`AssetLoader`](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [`Font`](../prelude/struct.Font.html "struct bevy::prelude::Font")s, for use by the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[FontVariationTag](struct.FontVariationTag.html "struct bevy::text::FontVariationTag")

An OpenType font variation tag.

[FontVariations](struct.FontVariations.html "struct bevy::text::FontVariations")

OpenType font variations for variable fonts that support them.

[FontVariationsBuilder](struct.FontVariationsBuilder.html "struct bevy::text::FontVariationsBuilder")

A builder for [`FontVariations`](struct.FontVariations.html "struct bevy::text::FontVariations").

[FontWeight](struct.FontWeight.html "struct bevy::text::FontWeight")

How thick or bold the strokes of a font appear.

[FontWidth](struct.FontWidth.html "struct bevy::text::FontWidth")

The visual width of a font as a ratio of its normal width, typically 0.5 to 2.0. `<https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswidthclass>`

[GlyphAtlasInfo](struct.GlyphAtlasInfo.html "struct bevy::text::GlyphAtlasInfo")

Information about a glyph in an atlas.

[GlyphAtlasLocation](struct.GlyphAtlasLocation.html "struct bevy::text::GlyphAtlasLocation")

The location of a glyph in an atlas, and how it should be positioned when placed.

[GlyphCacheKey](struct.GlyphCacheKey.html "struct bevy::text::GlyphCacheKey")

Key identifying a glyph

[LayoutCx](struct.LayoutCx.html "struct bevy::text::LayoutCx")

Text layout context

[PositionedGlyph](struct.PositionedGlyph.html "struct bevy::text::PositionedGlyph")

A glyph of a font, typically representing a single character, positioned in screen space.

[PreeditCursor](struct.PreeditCursor.html "struct bevy::text::PreeditCursor")

A selection within IME preedit text, expressed as byte offsets from the start of the preedit.

[RemSize](struct.RemSize.html "struct bevy::text::RemSize")

Base value used to resolve `Rem` units for font sizes.

[RunGeometry](struct.RunGeometry.html "struct bevy::text::RunGeometry")

Geometry of a text run used to render text decorations like background colors, strikethrough, and underline. A run in `bevy_text` is a contiguous sequence of glyphs on a line that share the same text attributes like font, font size, and line height.

[ScaleCx](struct.ScaleCx.html "struct bevy::text::ScaleCx")

Text scaler context

[Strikethrough](struct.Strikethrough.html "struct bevy::text::Strikethrough")

A text entity with this component is drawn with strikethrough.

[StrikethroughColor](struct.StrikethroughColor.html "struct bevy::text::StrikethroughColor")

Color for the text’s strikethrough. If this component is not present, its `TextColor` will be used.

[Text2dUpdateSystems](struct.Text2dUpdateSystems.html "struct bevy::text::Text2dUpdateSystems")

System set in [`PostUpdate`](../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate") where all 2d text update systems are executed.

[TextBackgroundColor](struct.TextBackgroundColor.html "struct bevy::text::TextBackgroundColor")

The background color of the text for this section.

[TextBounds](struct.TextBounds.html "struct bevy::text::TextBounds")

The maximum width and height of text. The text will wrap according to the specified size.

[TextBrush](struct.TextBrush.html "struct bevy::text::TextBrush")

Per-section metadata attached to shaped text runs.

[TextColor](struct.TextColor.html "struct bevy::text::TextColor")

The color of the text for this section.

[TextCursorStyle](struct.TextCursorStyle.html "struct bevy::text::TextCursorStyle")

Controls text cursor appearance.

[TextEditChange](struct.TextEditChange.html "struct bevy::text::TextEditChange")

Triggered after applying all pending [`TextEdit`](enum.TextEdit.html "enum bevy::text::TextEdit")s to the [`EditableText`](struct.EditableText.html "struct bevy::text::EditableText") by [`apply_text_edits`](fn.apply_text_edits.html "fn bevy::text::apply_text_edits").

[TextEntity](struct.TextEntity.html "struct bevy::text::TextEntity")

A sub-entity of a [`ComputedTextBlock`](struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock").

[TextFont](struct.TextFont.html "struct bevy::text::TextFont")

`TextFont` determines the style of a text span within a [`ComputedTextBlock`](struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"), specifically the font face, the font size, the line height, and the antialiasing method.

[TextFontTemplate](struct.TextFontTemplate.html "struct bevy::text::TextFontTemplate")

[TextIterScratch](struct.TextIterScratch.html "struct bevy::text::TextIterScratch")

Scratch buffer used to store intermediate state when iterating over text spans.

[TextLayout](struct.TextLayout.html "struct bevy::text::TextLayout")

Component with text format settings for a block of text.

[TextLayoutInfo](struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo")

Render information for a corresponding text block.

[TextMeasureInfo](struct.TextMeasureInfo.html "struct bevy::text::TextMeasureInfo")

Size information for a corresponding [`ComputedTextBlock`](struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock") component.

[TextPipeline](struct.TextPipeline.html "struct bevy::text::TextPipeline")

The `TextPipeline` is used to layout and render text blocks (see `Text`/`Text2d`).

[TextPlugin](struct.TextPlugin.html "struct bevy::text::TextPlugin")

Adds text rendering support to an app.

[TextReader](struct.TextReader.html "struct bevy::text::TextReader")

System parameter for reading text spans in a text block.

[TextSpan](struct.TextSpan.html "struct bevy::text::TextSpan")

A span of text in a tree of spans.

[TextSpanIter](struct.TextSpanIter.html "struct bevy::text::TextSpanIter")

Iterator returned by [`TextReader::iter`](struct.TextReader.html#method.iter "method bevy::text::TextReader::iter").

[TextWriter](struct.TextWriter.html "struct bevy::text::TextWriter")

System parameter for reading and writing text spans in a text block.

[Underline](struct.Underline.html "struct bevy::text::Underline")

Add to a text entity to draw its text with underline.

[UnderlineColor](struct.UnderlineColor.html "struct bevy::text::UnderlineColor")

Color for the text’s underline. If this component is not present, its `TextColor` will be used.

## Enums

[FontHinting](enum.FontHinting.html "enum bevy::text::FontHinting")

Font hinting strategy, which controls the rasterization for fonts.

[FontLoaderError](enum.FontLoaderError.html "enum bevy::text::FontLoaderError")

Possible errors that can be produced by [`FontLoader`](struct.FontLoader.html "struct bevy::text::FontLoader")

[FontSize](enum.FontSize.html "enum bevy::text::FontSize")

The vertical height of rasterized glyphs in the font atlas in pixels.

[FontSmoothing](enum.FontSmoothing.html "enum bevy::text::FontSmoothing")

Determines which antialiasing method to use when rendering text. By default, text is rendered with grayscale antialiasing, but this can be changed to achieve a pixelated look.

[FontSource](enum.FontSource.html "enum bevy::text::FontSource")

Determines how the font face for a text sections is selected.

[FontSourceTemplate](enum.FontSourceTemplate.html "enum bevy::text::FontSourceTemplate")

[FontStyle](enum.FontStyle.html "enum bevy::text::FontStyle")

The slant style of a font face: normal, italic, or oblique.

[Justify](enum.Justify.html "enum bevy::text::Justify")

Describes the horizontal alignment of multiple lines of text relative to each other.

[LetterSpacing](enum.LetterSpacing.html "enum bevy::text::LetterSpacing")

Specifies the space between each letter of text for `Text` and `Text2d`

[LineBreak](enum.LineBreak.html "enum bevy::text::LineBreak")

Determines how lines will be broken when preventing text from running out of bounds.

[LineHeight](enum.LineHeight.html "enum bevy::text::LineHeight")

Specifies the height of each line of text for `Text` and `Text2d`

[TextEdit](enum.TextEdit.html "enum bevy::text::TextEdit")

Deferred text input edit and navigation actions applied by the `apply_text_edits` system.

[TextError](enum.TextError.html "enum bevy::text::TextError")

Errors related to the textsystem

## Constants

[DEFAULT\_FONT\_DATA](constant.DEFAULT_FONT_DATA.html "constant bevy::text::DEFAULT_FONT_DATA")`default_font`

The raw data for the default font used by `bevy_text`

## Traits

[TextSection](trait.TextSection.html "trait bevy::text::TextSection")

Helper trait for using the [`TextReader`](struct.TextReader.html "struct bevy::text::TextReader") and [`TextWriter`](struct.TextWriter.html "struct bevy::text::TextWriter") system params.

## Functions

[add\_glyph\_to\_atlas](fn.add_glyph_to_atlas.html "fn bevy::text::add_glyph_to_atlas")

Adds the given subpixel-offset glyph to the given font atlases

[apply\_text\_edits](fn.apply_text_edits.html "fn bevy::text::apply_text_edits")

Applies pending text edit actions to all [`EditableText`](struct.EditableText.html "struct bevy::text::EditableText") widgets.

[detect\_text\_needs\_rerender](fn.detect_text_needs_rerender.html "fn bevy::text::detect_text_needs_rerender")

System that detects changes to text blocks and sets `ComputedTextBlock::should_rerender`.

[get\_glyph\_atlas\_info](fn.get_glyph_atlas_info.html "fn bevy::text::get_glyph_atlas_info")

Generates the [`GlyphAtlasInfo`](struct.GlyphAtlasInfo.html "struct bevy::text::GlyphAtlasInfo") for the given subpixel-offset glyph.

[get\_outlined\_glyph\_texture](fn.get_outlined_glyph_texture.html "fn bevy::text::get_outlined_glyph_texture")

Get the texture of the glyph as a rendered image, and its offset

[load\_font\_assets\_into\_font\_collection](fn.load_font_assets_into_font_collection.html "fn bevy::text::load_font_assets_into_font_collection")

Add new font assets to the internal font collection, and set any associated `TextFont`’s changed. If any fonts are removed, the font collection is completely rebuilt, the generic families are remapped, and all `TextFont`s are set changed.

[resolve\_font\_source](fn.resolve_font_source.html "fn bevy::text::resolve_font_source")

Resolve a [`TextFont`](../prelude/struct.TextFont.html "struct bevy::prelude::TextFont")’s [`FontSource`](../prelude/enum.FontSource.html "enum bevy::prelude::FontSource") to a font family.