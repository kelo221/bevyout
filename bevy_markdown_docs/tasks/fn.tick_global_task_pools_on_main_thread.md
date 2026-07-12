[bevy](../index.html)::[tasks](index.html)

# Function tick\_global\_task\_pools\_on\_main\_thread 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/usages.rs.html#86)

```rust
pub fn tick_global_task_pools_on_main_thread()
```

A function used by `bevy_app` to tick the global tasks pools on the main thread. This will run a maximum of 100 local tasks per executor per call to this function.

## Warning

This function _must_ be called on the main thread, or the task pools will not be updated appropriately.