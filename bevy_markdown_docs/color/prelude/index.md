[bevy](../../index.html)::[color](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lib.rs.html#125)

The color prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[Hsla](struct.Hsla.html "struct bevy::color::prelude::Hsla")

Color in Hue-Saturation-Lightness (HSL) color space with alpha. Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).

[Hsva](struct.Hsva.html "struct bevy::color::prelude::Hsva")

Color in Hue-Saturation-Value (HSV) color space with alpha. Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).

[Hwba](struct.Hwba.html "struct bevy::color::prelude::Hwba")

Color in Hue-Whiteness-Blackness (HWB) color space with alpha. Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HWB_color_model).

[Laba](struct.Laba.html "struct bevy::color::prelude::Laba")

Color in LAB color space, with alpha

[Lcha](struct.Lcha.html "struct bevy::color::prelude::Lcha")

Color in LCH color space, with alpha

[LinearRgba](struct.LinearRgba.html "struct bevy::color::prelude::LinearRgba")

Linear RGB color with alpha.

[Oklaba](struct.Oklaba.html "struct bevy::color::prelude::Oklaba")

Color in Oklab color space, with alpha

[Oklcha](struct.Oklcha.html "struct bevy::color::prelude::Oklcha")

Color in Oklch color space, with alpha

[Srgba](struct.Srgba.html "struct bevy::color::prelude::Srgba")

Non-linear standard RGB with alpha.

[Xyza](struct.Xyza.html "struct bevy::color::prelude::Xyza")

[CIE 1931](https://en.wikipedia.org/wiki/CIE_1931_color_space) color space, also known as XYZ, with an alpha channel.

## Enums

[Color](enum.Color.html "enum bevy::color::prelude::Color")

An enumerated type that can represent any of the color types in this crate.

[HexColorError](enum.HexColorError.html "enum bevy::color::prelude::HexColorError")

Error returned if a hex string could not be parsed as a color.

## Traits

[Alpha](trait.Alpha.html "trait bevy::color::prelude::Alpha")

Methods for manipulating alpha values.

[ColorToComponents](trait.ColorToComponents.html "trait bevy::color::prelude::ColorToComponents")

Trait with methods for converting colors to non-color types

[ColorToPacked](trait.ColorToPacked.html "trait bevy::color::prelude::ColorToPacked")

Trait with methods for converting colors to packed non-color types

[Gray](trait.Gray.html "trait bevy::color::prelude::Gray")

Trait for returning a grayscale color of a provided lightness.

[Hue](trait.Hue.html "trait bevy::color::prelude::Hue")

Trait for manipulating the hue of a color.

[Luminance](trait.Luminance.html "trait bevy::color::prelude::Luminance")

Methods for changing the luminance of a color. Note that these methods are not guaranteed to produce consistent results across color spaces, but will be within a given space.

[Mix](trait.Mix.html "trait bevy::color::prelude::Mix")

Linear interpolation of two colors within a given color space.

[Saturation](trait.Saturation.html "trait bevy::color::prelude::Saturation")

Trait for manipulating the saturation of a color.