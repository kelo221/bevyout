[bevy](../../index.html)::[ecs](../index.html)::[component](index.html)

# Function component\_clone\_via\_clone 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/clone.rs.html#58-61)

```rust
pub fn component_clone_via_clone<C>(
    source: &SourceComponent<'_>,
    ctx: &mut ComponentCloneCtx<'_, '_>,
)where
    C: Clone + Component,
```

Component [clone handler function](type.ComponentCloneFn.html "type bevy::ecs::component::ComponentCloneFn") implemented using the [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") trait. Can be [set](../../prelude/trait.Component.html#method.clone_behavior "associated function bevy::prelude::Component::clone_behavior") as clone handler for the specific component it is implemented for. It will panic if set as handler for any other component.