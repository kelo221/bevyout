[bevy](../../index.html)::[gizmos](../index.html)::[prelude](index.html)

# Function gizmo 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/global.rs.html#46)

```rust
pub fn gizmo() -> impl DerefMut
```

A global gizmo context for use outside of bevy systems.

## Example

```rust
fn draw() {
    gizmo().sphere(Isometry3d::IDENTITY, 0.5, WHITE);
}
```