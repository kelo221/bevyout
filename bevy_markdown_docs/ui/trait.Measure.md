[bevy](../index.html)::[ui](index.html)

# Trait Measure 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/measurement.rs.html#97)

```rust
pub trait Measure:
    Send
    + Sync
    + 'static {
    // Required method
    fn measure(&mut self, measure_args: MeasureArgs<'_>) -> Vec2;
}
```

A `Measure` is used to compute the size of a ui node when the size of that node is based on its content.

## Required Methods

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/measurement.rs.html#99)

#### fn [measure](#tymethod.measure)(&mut self, measure\_args: [MeasureArgs](struct.MeasureArgs.html "struct bevy::ui::MeasureArgs")<'\_>) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Calculate the size of the node given the constraints.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/measurement.rs.html#131)

### impl [Measure](trait.Measure.html "trait bevy::ui::Measure") for [FixedMeasure](struct.FixedMeasure.html "struct bevy::ui::FixedMeasure")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#218)

### impl [Measure](trait.Measure.html "trait bevy::ui::Measure") for [ImageMeasure](widget/struct.ImageMeasure.html "struct bevy::ui::widget::ImageMeasure")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/measurement.rs.html#113)

### impl [Measure](trait.Measure.html "trait bevy::ui::Measure") for [NodeMeasure](enum.NodeMeasure.html "enum bevy::ui::NodeMeasure")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#182)

### impl [Measure](trait.Measure.html "trait bevy::ui::Measure") for [TextMeasure](widget/struct.TextMeasure.html "struct bevy::ui::widget::TextMeasure")