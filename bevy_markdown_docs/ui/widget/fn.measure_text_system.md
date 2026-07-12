[bevy](../../index.html)::[ui](../index.html)::[widget](index.html)

# Function measure\_text\_system 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#244-264)

```rust
pub fn measure_text_system(
    fonts: Res<'_, Assets<Font>>,
    text_query: Query<'_, '_, (Entity, Ref<'_, Text>, Ref<'_, TextLayout>, &mut ContentSize, &mut TextNodeFlags, &mut ComputedTextBlock, Ref<'_, ComputedUiRenderTargetInfo>, &ComputedNode), With<Node>>,
    text_reader: TextReader<'_, '_, Text>,
    text_pipeline: ResMut<'_, TextPipeline>,
    font_system: ResMut<'_, FontCx>,
    layout_cx: ResMut<'_, LayoutCx>,
    rem_size: Res<'_, RemSize>,
)
```

Generates a new [`Measure`](../trait.Measure.html "trait bevy::ui::Measure") for a text node on changes to its [`Text`](../../prelude/struct.Text.html "struct bevy::prelude::Text") component.

A `Measure` is used by the UI’s layout algorithm to determine the appropriate amount of space to provide for the text given the fonts, the text itself and the constraints of the layout.

*   Measures are regenerated on changes to either [`ComputedTextBlock`](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock") or [`ComputedUiRenderTargetInfo`](../../prelude/struct.ComputedUiRenderTargetInfo.html "struct bevy::prelude::ComputedUiRenderTargetInfo").
*   Changes that only modify the colors of a `Text` do not require a new `Measure`. This system is only able to detect that a `Text` component has changed and will regenerate the `Measure` on color changes. This can be expensive, particularly for large blocks of text, and the [`bypass_change_detection`](../../prelude/trait.DetectChangesMut.html#tymethod.bypass_change_detection "method bevy::prelude::DetectChangesMut::bypass_change_detection") method should be called when only changing the `Text`’s colors.