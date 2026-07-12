[bevy](../../index.html)::[ui](../index.html)

# Module measurement 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#17)

## Structs

[ContentSize](struct.ContentSize.html "struct bevy::ui::measurement::ContentSize")

A node with a `ContentSize` component is a node where its size is based on its content.

[FixedMeasure](struct.FixedMeasure.html "struct bevy::ui::measurement::FixedMeasure")

A `FixedMeasure` is a `Measure` that ignores all constraints and always returns the same size.

[MeasureArgs](struct.MeasureArgs.html "struct bevy::ui::measurement::MeasureArgs")

Inputs provided to [`Measure::measure`](../trait.Measure.html#tymethod.measure "method bevy::ui::Measure::measure").

[ResolvedAxis](struct.ResolvedAxis.html "struct bevy::ui::measurement::ResolvedAxis")

Resolved values for per-axis size constraints.

## Enums

[AvailableSpace](enum.AvailableSpace.html "enum bevy::ui::measurement::AvailableSpace")

The amount of space available to a node in a given axis [https://www.w3.org/TR/css-sizing-3/#available](https://www.w3.org/TR/css-sizing-3/#available)

[NodeMeasure](enum.NodeMeasure.html "enum bevy::ui::measurement::NodeMeasure")

A type to serve as Taffy’s node context (which allows the content size of leaf nodes to be computed)

## Traits

[Measure](trait.Measure.html "trait bevy::ui::measurement::Measure")

A `Measure` is used to compute the size of a ui node when the size of that node is based on its content.