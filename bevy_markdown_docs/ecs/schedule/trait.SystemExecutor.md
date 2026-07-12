[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Trait SystemExecutor 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#30)

```rust
pub trait SystemExecutor: Send + Sync {
    // Required methods
    fn init(&mut self, schedule: &SystemSchedule);
    fn run(
        &mut self,
        schedule: &mut SystemSchedule,
        world: &mut World,
        skip_systems: Option<&FixedBitSet>,
        error_handler: fn(BevyError, ErrorContext),
    );
    fn set_apply_final_deferred(&mut self, value: bool);
}
```

Types that can run a [`SystemSchedule`](struct.SystemSchedule.html "struct bevy::ecs::schedule::SystemSchedule") on a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#32)

#### fn [init](#tymethod.init)(&mut self, schedule: &[SystemSchedule](struct.SystemSchedule.html "struct bevy::ecs::schedule::SystemSchedule"))

Called once after the schedule is built or rebuilt.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#34-40)

#### fn [run](#tymethod.run)( &mut self, schedule: &mut [SystemSchedule](struct.SystemSchedule.html "struct bevy::ecs::schedule::SystemSchedule"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), skip\_systems: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[FixedBitSet](struct.FixedBitSet.html "struct bevy::ecs::schedule::FixedBitSet")\>, error\_handler: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError"), [ErrorContext](../error/enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext")), )

Runs the systems in the schedule.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#42)

#### fn [set\_apply\_final\_deferred](#tymethod.set_apply_final_deferred)(&mut self, value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

Sets whether deferred system buffers should be applied after all systems have run.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/multi_threaded.rs.html#150)

### impl [SystemExecutor](trait.SystemExecutor.html "trait bevy::ecs::schedule::SystemExecutor") for [MultiThreadedExecutor](struct.MultiThreadedExecutor.html "struct bevy::ecs::schedule::MultiThreadedExecutor")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/single_threaded.rs.html#40)

### impl [SystemExecutor](trait.SystemExecutor.html "trait bevy::ecs::schedule::SystemExecutor") for [SingleThreadedExecutor](struct.SingleThreadedExecutor.html "struct bevy::ecs::schedule::SingleThreadedExecutor")