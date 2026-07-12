[bevy](../index.html)

# Crate color 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lib.rs.html#1-279)

Representations of colors in various color spaces.

This crate provides a number of color representations, including:

*   [`Srgba`](../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") (standard RGBA, with gamma correction)
*   [`LinearRgba`](../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba") (linear RGBA, without gamma correction)
*   [`Hsla`](../prelude/struct.Hsla.html "struct bevy::prelude::Hsla") (hue, saturation, lightness, alpha)
*   [`Hsva`](../prelude/struct.Hsva.html "struct bevy::prelude::Hsva") (hue, saturation, value, alpha)
*   [`Hwba`](../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") (hue, whiteness, blackness, alpha)
*   [`Laba`](../prelude/struct.Laba.html "struct bevy::prelude::Laba") (lightness, a-axis, b-axis, alpha)
*   [`Lcha`](../prelude/struct.Lcha.html "struct bevy::prelude::Lcha") (lightness, chroma, hue, alpha)
*   [`Oklaba`](../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba") (lightness, a-axis, b-axis, alpha)
*   [`Oklcha`](../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha") (lightness, chroma, hue, alpha)
*   [`Xyza`](../prelude/struct.Xyza.html "struct bevy::prelude::Xyza") (x-axis, y-axis, z-axis, alpha)

Each of these color spaces is represented as a distinct Rust type.

## Color Space Usage

Rendering engines typically use linear RGBA colors, which allow for physically accurate lighting calculations. However, linear RGBA colors are not perceptually uniform, because both human eyes and computer monitors have non-linear responses to light. “Standard” RGBA represents an industry-wide compromise designed to encode colors in a way that looks good to humans in as few bits as possible, but it is not suitable for lighting calculations.

Most image file formats and scene graph formats use standard RGBA, because graphic design tools are intended to be used by humans. However, 3D lighting calculations operate in linear RGBA, so it is important to convert standard colors to linear before sending them to the GPU. Most Bevy APIs will handle this conversion automatically, but if you are writing a custom shader, you will need to do this conversion yourself.

HSL and LCH are “cylindrical” color spaces, which means they represent colors as a combination of hue, saturation, and lightness (or chroma). These color spaces are useful for working with colors in an artistic way - for example, when creating gradients or color palettes. A gradient in HSL space from red to violet will produce a rainbow. The LCH color space is more perceptually accurate than HSL, but is less intuitive to work with.

HSV and HWB are very closely related to HSL in their derivation, having identical definitions for hue. Where HSL uses saturation and lightness, HSV uses a slightly modified definition of saturation, and an analog of lightness in the form of value. In contrast, HWB instead uses whiteness and blackness parameters, which can be used to lighten and darken a particular hue respectively.

Oklab and Oklch are perceptually uniform color spaces that are designed to be used for tasks such as image processing. They are not as widely used as the other color spaces, but are useful for tasks such as color correction and image analysis, where it is important to be able to do things like change color saturation without causing hue shifts.

XYZ is a foundational space commonly used in the definition of other more modern color spaces. The space is more formally known as CIE 1931, where the `x` and `z` axes represent a form of chromaticity, while `y` defines an illuminance level.

See also the [Wikipedia article on color spaces](https://en.wikipedia.org/wiki/Color_space).

## Conversion

Conversion between the various color spaces is achieved using Rust’s native [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") trait. Because certain color spaces are defined by their transformation to and from another space, these [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") implementations reflect that set of definitions.

```rust
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

For example, the [sRGB](../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") space is defined by its relationship with [Linear RGB](../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba"), and [HWB](../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") by its with [sRGB](../prelude/struct.Srgba.html "struct bevy::prelude::Srgba"). As such, it is the responsibility of [sRGB](../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") to provide [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") implementations for [Linear RGB](../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba"), and [HWB](../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") for [sRGB](../prelude/struct.Srgba.html "struct bevy::prelude::Srgba"). To then provide conversion between [Linear RGB](../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba") and [HWB](../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") directly, [HWB](../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") is responsible for implementing these conversions, delegating to [sRGB](../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

@import url("https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css");'#graph-div{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:16px;fill:#ccc;}#graph-div .error-icon{fill:#a44141;}#graph-div .error-text{fill:#ddd;stroke:#ddd;}#graph-div .edge-thickness-normal{stroke-width:2px;}#graph-div .edge-thickness-thick{stroke-width:3.5px;}#graph-div .edge-pattern-solid{stroke-dasharray:0;}#graph-div .edge-pattern-dashed{stroke-dasharray:3;}#graph-div .edge-pattern-dotted{stroke-dasharray:2;}#graph-div .marker{fill:lightgrey;stroke:lightgrey;}#graph-div .marker.cross{stroke:lightgrey;}#graph-div svg{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:16px;}#graph-div .label{font-family:"trebuchet ms",verdana,arial,sans-serif;color:#ccc;}#graph-div .cluster-label text{fill:#F9FFFE;}#graph-div .cluster-label span,#graph-div p{color:#F9FFFE;}#graph-div .label text,#graph-div span,#graph-div p{fill:#ccc;color:#ccc;}#graph-div .node rect,#graph-div .node circle,#graph-div .node ellipse,#graph-div .node polygon,#graph-div .node path{fill:#1f2020;stroke:#81B1DB;stroke-width:1px;}#graph-div .flowchart-label text{text-anchor:middle;}#graph-div .node .label{text-align:center;}#graph-div .node.clickable{cursor:pointer;}#graph-div .arrowheadPath{fill:lightgrey;}#graph-div .edgePath .path{stroke:lightgrey;stroke-width:2.0px;}#graph-div .flowchart-link{stroke:lightgrey;fill:none;}#graph-div .edgeLabel{background-color:hsl(0, 0%, 34.4117647059%);text-align:center;}#graph-div .edgeLabel rect{opacity:0.5;background-color:hsl(0, 0%, 34.4117647059%);fill:hsl(0, 0%, 34.4117647059%);}#graph-div .labelBkg{background-color:rgba(87.75, 87.75, 87.75, 0.5);}#graph-div .cluster rect{fill:hsl(180, 1.5873015873%, 28.3529411765%);stroke:rgba(255, 255, 255, 0.25);stroke-width:1px;}#graph-div .cluster text{fill:#F9FFFE;}#graph-div .cluster span,#graph-div p{color:#F9FFFE;}#graph-div div.mermaidTooltip{position:absolute;text-align:center;max-width:200px;padding:2px;font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:12px;background:hsl(20, 1.5873015873%, 12.3529411765%);border:1px solid rgba(255, 255, 255, 0.25);border-radius:2px;pointer-events:none;z-index:100;}#graph-div .flowchartTitleText{text-anchor:middle;font-size:18px;fill:#ccc;}#graph-div .label foreignObject{overflow:visible;}#graph-div :root{--mermaid-font-family:"trebuchet ms",verdana,arial,sans-serif;}

[Conversion](https://bottosson.github.io/posts/oklab/#converting-from-linear-srgb-to-oklab)

[Conversion](https://bottosson.github.io/posts/oklab/#the-oklab-color-space)

[Conversion](http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html)

[Conversion](http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Lab.html)

[Conversion](http://www.brucelindbloom.com/index.html?Eqn_Lab_to_LCH.html)

[Conversion](https://en.wikipedia.org/wiki/SRGB#From_sRGB_to_CIE_XYZ)

[Conversion](http://alvyray.com/Papers/CG/HWB_JGTv208.pdf)

[Conversion](http://alvyray.com/Papers/CG/HWB_JGTv208.pdf)

[Conversion](https://en.wikipedia.org/wiki/HSL_and_HSV#Interconversion)

[Linear  
sRGB](https://en.wikipedia.org/wiki/Rgb)

[Oklab](https://oklch.com/)

[Oklch](https://oklch.com/)

[XYZ](https://en.wikipedia.org/wiki/XYZ_color)

[Lab](https://en.wikipedia.org/wiki/Lab_color)

[Lch](https://en.wikipedia.org/wiki/CIELAB_color_space#Cylindrical_model)

[sRGB](https://en.wikipedia.org/wiki/Srgb)

[HWB](https://en.wikipedia.org/wiki/HWB_color_model)

[HSV](https://en.wikipedia.org/wiki/HSL_and_HSV)

[HSL](https://en.wikipedia.org/wiki/HSL_and_HSV)

GPU

## Other Utilities

The crate also provides a number of color operations, such as blending, color difference, and color range operations.

In addition, there is a [`Color`](../prelude/enum.Color.html "enum bevy::prelude::Color") enum that can represent any of the color types in this crate. This is useful when you need to store a color in a data structure that can’t be generic over the color type.

Color types that are either physically or perceptually linear also implement `Add<Self>`, `Sub<Self>`, `Mul<f32>` and `Div<f32>` allowing you to use them with splines.

Please note that most often adding or subtracting colors is not what you may want. Please have a look at other operations like blending, lightening or mixing colors using e.g. [`Mix`](../prelude/trait.Mix.html "trait bevy::prelude::Mix") or [`Luminance`](../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") instead.

## Example

```rust
use bevy_color::{Srgba, Hsla};

let srgba = Srgba::new(0.5, 0.2, 0.8, 1.0);
let hsla: Hsla = srgba.into();

println!("Srgba: {:?}", srgba);
println!("Hsla: {:?}", hsla);
```

## Modules

[color\_difference](color_difference/index.html "mod bevy::color::color_difference")

Module for calculating distance between two colors in the same color space.

[palettes](palettes/index.html "mod bevy::color::palettes")

Color palettes consisting of collections of const colors.

[prelude](prelude/index.html "mod bevy::color::prelude")

The color prelude.

## Structs

[ColorCurve](struct.ColorCurve.html "struct bevy::color::ColorCurve")

A curve whose samples are defined by a collection of colors.

[Hsla](struct.Hsla.html "struct bevy::color::Hsla")

Color in Hue-Saturation-Lightness (HSL) color space with alpha. Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).

[Hsva](struct.Hsva.html "struct bevy::color::Hsva")

Color in Hue-Saturation-Value (HSV) color space with alpha. Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).

[Hwba](struct.Hwba.html "struct bevy::color::Hwba")

Color in Hue-Whiteness-Blackness (HWB) color space with alpha. Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HWB_color_model).

[Laba](struct.Laba.html "struct bevy::color::Laba")

Color in LAB color space, with alpha

[Lcha](struct.Lcha.html "struct bevy::color::Lcha")

Color in LCH color space, with alpha

[LinearRgba](struct.LinearRgba.html "struct bevy::color::LinearRgba")

Linear RGB color with alpha.

[Oklaba](struct.Oklaba.html "struct bevy::color::Oklaba")

Color in Oklab color space, with alpha

[Oklcha](struct.Oklcha.html "struct bevy::color::Oklcha")

Color in Oklch color space, with alpha

[Srgba](struct.Srgba.html "struct bevy::color::Srgba")

Non-linear standard RGB with alpha.

[Xyza](struct.Xyza.html "struct bevy::color::Xyza")

[CIE 1931](https://en.wikipedia.org/wiki/CIE_1931_color_space) color space, also known as XYZ, with an alpha channel.

## Enums

[Color](enum.Color.html "enum bevy::color::Color")

An enumerated type that can represent any of the color types in this crate.

[HexColorError](enum.HexColorError.html "enum bevy::color::HexColorError")

Error returned if a hex string could not be parsed as a color.

## Traits

[Alpha](trait.Alpha.html "trait bevy::color::Alpha")

Methods for manipulating alpha values.

[ColorRange](trait.ColorRange.html "trait bevy::color::ColorRange")

Represents a range of colors that can be linearly interpolated, defined by a start and end point which must be in the same color space. It works for any color type that implements [`Mix`](../prelude/trait.Mix.html "trait bevy::prelude::Mix").

[ColorToComponents](trait.ColorToComponents.html "trait bevy::color::ColorToComponents")

Trait with methods for converting colors to non-color types

[ColorToPacked](trait.ColorToPacked.html "trait bevy::color::ColorToPacked")

Trait with methods for converting colors to packed non-color types

[Gray](trait.Gray.html "trait bevy::color::Gray")

Trait for returning a grayscale color of a provided lightness.

[Hue](trait.Hue.html "trait bevy::color::Hue")

Trait for manipulating the hue of a color.

[Luminance](trait.Luminance.html "trait bevy::color::Luminance")

Methods for changing the luminance of a color. Note that these methods are not guaranteed to produce consistent results across color spaces, but will be within a given space.

[Mix](trait.Mix.html "trait bevy::color::Mix")

Linear interpolation of two colors within a given color space.

[Saturation](trait.Saturation.html "trait bevy::color::Saturation")

Trait for manipulating the saturation of a color.