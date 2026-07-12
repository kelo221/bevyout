[bevy](../index.html)::[tasks](index.html)

# Function available\_parallelism 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#128)

```rust
pub fn available_parallelism() -> usize
```

Gets the logical CPU core count available to the current process.

This is identical to `std::thread::available_parallelism`, except it will return a default value of 1 if it internally errors out.

This will always return at least 1.