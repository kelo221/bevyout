[bevy](../../index.html)::[camera](../index.html)::[visibility](index.html)

# Function add\_visibility\_class 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#933-937)

```rust
pub fn add_visibility_class<C>(world: DeferredWorld<'_>, _: HookContext)where
    C: 'static,
```

A generic component add hook that automatically adds the appropriate [`VisibilityClass`](struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass") to an entity.

This can be handy when creating custom renderable components. To use this hook, add it to your renderable component like this:

[ⓘ](# "This example is not tested")

```rust
#[derive(Component)]
#[component(on_add = add_visibility_class::<MyComponent>)]
struct MyComponent {
    ...
}
```