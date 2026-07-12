[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Type Alias SystemParamItem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#294)

```rust
pub type SystemParamItem<'w, 's, P> = <P as SystemParam>::Item<'w, 's>;
```

Shorthand way of accessing the associated type [`SystemParam::Item`](trait.SystemParam.html#associatedtype.Item "associated type bevy::ecs::system::SystemParam::Item") for a given [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam").