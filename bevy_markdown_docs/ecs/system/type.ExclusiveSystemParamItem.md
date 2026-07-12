[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Type Alias ExclusiveSystemParamItem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_system_param.rs.html#38)

```rust
pub type ExclusiveSystemParamItem<'s, P> = <P as ExclusiveSystemParam>::Item<'s>;
```

Shorthand way of accessing the associated type [`ExclusiveSystemParam::Item`](trait.ExclusiveSystemParam.html#associatedtype.Item "associated type bevy::ecs::system::ExclusiveSystemParam::Item") for a given [`ExclusiveSystemParam`](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam").