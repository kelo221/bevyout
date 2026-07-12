[bevy](../../index.html)::[render](../index.html)::[extract\_component](index.html)

# Derive Macro ExtractComponent 

[Source](https://docs.rs/bevy_render_macros/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render_macros/lib.rs.html#60)

```rust
#[derive(ExtractComponent)]
{
    // Attributes available to this derive:
    #[extract_component_filter]
    #[extract_component_sync_target]
}
```

Implements `ExtractComponent` trait for a component.

The component must implement \[`Clone`\]. The component will be extracted into the render world via cloning. Note that this only enables extraction of the component, it does not execute the extraction. See `ExtractComponentPlugin` to actually perform the extraction.

If you only want to extract a component conditionally, you may use the `extract_component_filter` attribute. To specify `SyncComponent::Target`, you can use the `extract_component_sync_target` attribute.

## Example

```
use bevy_ecs::component::Component;
use bevy_render_macros::ExtractComponent;

#[derive(Component, Clone, ExtractComponent)]
#[extract_component_filter(With<Camera>)]
#[extract_component_sync_target((Self, OtherNeedsCleanup))]
pub struct Foo {
    pub should_foo: bool,
}

// Without a filter (unconditional).
#[derive(Component, Clone, ExtractComponent)]
pub struct Bar {
    pub should_bar: bool,
}
```