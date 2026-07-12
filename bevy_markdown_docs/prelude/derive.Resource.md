[bevy](../index.html)::[prelude](index.html)

# Derive Macro Resource 

[Source](https://docs.rs/bevy_ecs_macros/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs_macros/lib.rs.html#586)

```rust
#[derive(Resource)]
{
    // Attributes available to this derive:
    #[component]
    #[require]
}
```

Implement the `Resource` trait.

### Immutability

[ⓘ](# "This example is not tested")

```rust
#[derive(Resource)]
#[component(immutable)]
struct MyResource;
```

### Hooks

[ⓘ](# "This example is not tested")

```rust
#[derive(Resource)]
#[component(hook_name = function)]
struct MyResource;
```

where `hook_name` is `on_add`, `on_insert`, `on_discard` or `on_remove`; `function` can be either a path, e.g. `some_function::<Self>`, or a function call that returns a function that can be turned into a `ComponentHook`, e.g. `get_closure("Hi!")`. `function` can be elided if the path is `Self::on_add`, `Self::on_insert` etc.