[bevy](../../../index.html)::[render](../../index.html)::[view](../index.html)::[visibility](index.html)

# Type Alias VisibilityExtractionNoCpuCullingChangedQuery 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#316)

```rust
pub type VisibilityExtractionNoCpuCullingChangedQuery = Query<'static, 'static, (Entity, &'static VisibilityClass, &'static InheritedVisibility), (Or<(Changed<NoCpuCulling>, Changed<InheritedVisibility>)>, With<NoCpuCulling>)>;
```

The query, part of [`VisibilityExtractionSystemParam`](../struct.VisibilityExtractionSystemParam.html "struct bevy::render::view::VisibilityExtractionSystemParam"), that searches for entities with [`NoCpuCulling`](../../../camera/visibility/struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling") that might have changed visibility.

## Aliased Type

```rust
pub struct VisibilityExtractionNoCpuCullingChangedQuery { /* private fields */ }
```