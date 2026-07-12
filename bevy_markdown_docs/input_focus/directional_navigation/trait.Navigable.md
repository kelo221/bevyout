[bevy](../../index.html)::[input\_focus](../index.html)::[directional\_navigation](index.html)

# Trait Navigable 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#485)

```rust
pub trait Navigable {
    // Required method
    fn get_bounds(&self) -> (Vec2, Vec2);
}
```

Trait for extracting position and size from navigable UI components.

This allows the auto-navigation system to work with different UI implementations as long as they can provide position and size information.

## Required Methods

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#487)

#### fn [get\_bounds](#tymethod.get_bounds)(&self) -> ([Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Returns the center position and size in global coordinates.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors