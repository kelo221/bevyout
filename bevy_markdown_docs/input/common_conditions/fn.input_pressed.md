[bevy](../../index.html)::[input](../index.html)::[common\_conditions](index.html)

# Function input\_pressed 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/common_conditions.rs.html#66-68)

```rust
pub fn input_pressed<T>(input: T) -> impl FnMut(Res<'_, ButtonInput<T>>) + Clonewhere
    T: Clone + Eq + Hash + Send + Sync + 'static,
```

Run condition that is active if [`ButtonInput::pressed`](../../prelude/struct.ButtonInput.html#method.pressed "method bevy::prelude::ButtonInput::pressed") is true for the given input.