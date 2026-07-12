[bevy](../../index.html)::[text](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/lib.rs.html#67)

The text prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[Font](struct.Font.html "struct bevy::text::prelude::Font")

An [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") that contains the data for a loaded font, if loaded as an asset.

[FontWeight](struct.FontWeight.html "struct bevy::text::prelude::FontWeight")

How thick or bold the strokes of a font appear.

[FontWidth](struct.FontWidth.html "struct bevy::text::prelude::FontWidth")

The visual width of a font as a ratio of its normal width, typically 0.5 to 2.0. `<https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswidthclass>`

[Strikethrough](struct.Strikethrough.html "struct bevy::text::prelude::Strikethrough")

A text entity with this component is drawn with strikethrough.

[StrikethroughColor](struct.StrikethroughColor.html "struct bevy::text::prelude::StrikethroughColor")

Color for the text’s strikethrough. If this component is not present, its `TextColor` will be used.

[TextColor](struct.TextColor.html "struct bevy::text::prelude::TextColor")

The color of the text for this section.

[TextFont](struct.TextFont.html "struct bevy::text::prelude::TextFont")

`TextFont` determines the style of a text span within a [`ComputedTextBlock`](../struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"), specifically the font face, the font size, the line height, and the antialiasing method.

[TextLayout](struct.TextLayout.html "struct bevy::text::prelude::TextLayout")

Component with text format settings for a block of text.

[TextSpan](struct.TextSpan.html "struct bevy::text::prelude::TextSpan")

A span of text in a tree of spans.

[Underline](struct.Underline.html "struct bevy::text::prelude::Underline")

Add to a text entity to draw its text with underline.

[UnderlineColor](struct.UnderlineColor.html "struct bevy::text::prelude::UnderlineColor")

Color for the text’s underline. If this component is not present, its `TextColor` will be used.

## Enums

[FontHinting](enum.FontHinting.html "enum bevy::text::prelude::FontHinting")

Font hinting strategy, which controls the rasterization for fonts.

[FontSize](enum.FontSize.html "enum bevy::text::prelude::FontSize")

The vertical height of rasterized glyphs in the font atlas in pixels.

[FontSmoothing](enum.FontSmoothing.html "enum bevy::text::prelude::FontSmoothing")

Determines which antialiasing method to use when rendering text. By default, text is rendered with grayscale antialiasing, but this can be changed to achieve a pixelated look.

[FontSource](enum.FontSource.html "enum bevy::text::prelude::FontSource")

Determines how the font face for a text sections is selected.

[FontStyle](enum.FontStyle.html "enum bevy::text::prelude::FontStyle")

The slant style of a font face: normal, italic, or oblique.

[Justify](enum.Justify.html "enum bevy::text::prelude::Justify")

Describes the horizontal alignment of multiple lines of text relative to each other.

[LineBreak](enum.LineBreak.html "enum bevy::text::prelude::LineBreak")

Determines how lines will be broken when preventing text from running out of bounds.

[TextError](enum.TextError.html "enum bevy::text::prelude::TextError")

Errors related to the textsystem