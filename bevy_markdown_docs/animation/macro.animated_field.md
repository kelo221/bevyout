[bevy](../index.html)::[animation](index.html)

# Macro animated\_field 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#794)

```rust
macro_rules! animated_field {
    ($component:ident::$field:tt) => { ... };
}
```

Returns an [`AnimatedField`](../prelude/struct.AnimatedField.html "struct bevy::prelude::AnimatedField") with a given `$component` and `$field`.

This can be used in the following way:

```rust
#[derive(Component, Reflect)]
struct Transform {
    translation: Vec3,
}

let field = animated_field!(Transform::translation);

#[derive(Component, Reflect)]
struct Color(Srgba);

let tuple_field = animated_field!(Color::0);
```