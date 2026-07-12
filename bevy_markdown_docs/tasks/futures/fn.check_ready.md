[bevy](../../index.html)::[tasks](../index.html)::[futures](index.html)

# Function check\_ready 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/futures.rs.html#22)

```rust
pub fn check_ready<F>(future: &mut F) -> Option<<F as Future>::Output>where
    F: Future + Unpin,
```

Polls a future once, and returns the output if ready or returns `None` if it wasn’t ready yet.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/async\_tasks/async\_compute.rs ([line 134](../../../src/async_compute/async_compute.rs.html#134))

```rust
128fn handle_tasks(
129    mut commands: Commands,
130    mut transform_tasks: Query<(Entity, &mut ComputeTransform)>,
131) {
132    for (entity, mut task) in &mut transform_tasks {
133        // Use `check_ready` to efficiently poll the task without blocking the main thread.
134        if let Some(mut commands_queue) = check_ready(&mut task.0) {
135            // Append the returned command queue to execute it later.
136            commands.append(&mut commands_queue);
137            // Task is complete, so remove the task component from the entity.
138            commands.entity(entity).remove::<ComputeTransform>();
139        }
140    }
141}
```