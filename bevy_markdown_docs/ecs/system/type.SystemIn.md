[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Type Alias SystemIn 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#57)

```rust
pub type SystemIn<'a, S> = <<S as System>::In as SystemInput>::Inner<'a>;
```

Shorthand way to get the [`System::In`](../../prelude/trait.System.html#associatedtype.In "associated type bevy::prelude::System::In") for a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") as a [`SystemInput::Inner`](../../prelude/trait.SystemInput.html#associatedtype.Inner "associated type bevy::prelude::SystemInput::Inner").