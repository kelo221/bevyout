[bevy](../index.html)::[text](index.html)

# Trait TextSection 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#11)

```rust
pub trait TextSection: Component<Mutability = Mutable> + From<String> {
    // Required methods
    fn get_text(&self) -> &str;
    fn get_text_mut(&mut self) -> &mut String;
}
```

Helper trait for using the [`TextReader`](struct.TextReader.html "struct bevy::text::TextReader") and [`TextWriter`](struct.TextWriter.html "struct bevy::text::TextWriter") system params.

## Required Methods

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#13)

#### fn [get\_text](#tymethod.get_text)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the text for this section.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#15)

#### fn [get\_text\_mut](#tymethod.get_text_mut)(&mut self) -> &mut [String](../prelude/struct.String.html "struct bevy::prelude::String")

Returns a mutable reference to the text for this section.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#120)

### impl [TextSection](trait.TextSection.html "trait bevy::text::TextSection") for [Text](../prelude/struct.Text.html "struct bevy::prelude::Text")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#111)

### impl [TextSection](trait.TextSection.html "trait bevy::text::TextSection") for [Text2d](../prelude/struct.Text2d.html "struct bevy::prelude::Text2d")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#202)

### impl [TextSection](trait.TextSection.html "trait bevy::text::TextSection") for [TextSpan](../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")