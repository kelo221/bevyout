[bevy](../index.html)::[prelude](index.html)

# Trait Command 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#52)

```rust
pub trait Command: Send + 'static {
    type Out: CommandOutput;

    // Required method
    fn apply(self, world: &mut World) -> Self::Out;

    // Provided methods
    fn handle_error_with(
        self,
        error_handler: fn(BevyError, ErrorContext),
    ) -> impl Command<Out = ()>
       where Self: Sized { ... }
    fn handle_error(self) -> impl Command<Out = ()>
       where Self: Sized { ... }
    fn ignore_error(self) -> impl Command<Out = ()>
       where Self: Sized { ... }
}
```

A [`World`](struct.World.html "struct bevy::prelude::World") mutation.

Should be used with [`Commands::queue`](struct.Commands.html#method.queue "method bevy::prelude::Commands::queue").

The `Out` generic parameter is the returned “output” of the command.

## Usage

```rust
// Our world resource
#[derive(Resource, Default)]
struct Counter(u64);

// Our custom command
struct AddToCounter(u64);

impl Command for AddToCounter {
    type Out = ();

    fn apply(self, world: &mut World) {
        let mut counter = world.get_resource_or_insert_with(Counter::default);
        counter.0 += self.0;
    }
}

fn some_system(mut commands: Commands) {
    commands.queue(AddToCounter(42));
}
```

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#54)

#### type [Out](#associatedtype.Out): [CommandOutput](../ecs/error/trait.CommandOutput.html "trait bevy::ecs::error::CommandOutput")

The return type of [`apply`](trait.Command.html#tymethod.apply "method bevy::prelude::Command::apply").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#61)

#### fn [apply](#tymethod.apply)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> Self::[Out](trait.Command.html#associatedtype.Out "type bevy::prelude::Command::Out")

Applies this command, causing it to mutate the provided `world`.

This method is used to define what a command “does” when it is ultimately applied. Because this method takes `self`, you can store data or settings on the type that implements this trait. This data is set by the system or other source of the command, and then ultimately read in this method.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#66-68)

#### fn [handle\_error\_with](#method.handle_error_with)( self, error\_handler: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([BevyError](struct.BevyError.html "struct bevy::prelude::BevyError"), [ErrorContext](../ecs/error/enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext")), ) -> impl [Command](trait.Command.html "trait bevy::prelude::Command")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Takes a [`Command`](trait.Command.html "trait bevy::prelude::Command") that returns a Result and uses a given error handler function to convert it into a [`Command`](trait.Command.html "trait bevy::prelude::Command") that internally handles an error if it occurs and returns `()`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#85-87)

#### fn [handle\_error](#method.handle_error)(self) -> impl [Command](trait.Command.html "trait bevy::prelude::Command")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Takes a [`Command`](trait.Command.html "trait bevy::prelude::Command") that returns a Result and uses the fallback error handler function to convert it into a [`Command`](trait.Command.html "trait bevy::prelude::Command") that internally handles an error if it occurs and returns `()`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#103-105)

#### fn [ignore\_error](#method.ignore_error)(self) -> impl [Command](trait.Command.html "trait bevy::prelude::Command")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Takes a [`Command`](trait.Command.html "trait bevy::prelude::Command") that returns a Result and ignores any error that occurs.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#231)

### impl [Command](trait.Command.html "trait bevy::prelude::Command") for [SaveSettings](../settings/enum.SaveSettings.html "enum bevy::settings::SaveSettings")

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#232)

#### type [Out](#associatedtype.Out) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#251)

### impl [Command](trait.Command.html "trait bevy::prelude::Command") for [SaveSettingsDeferred](../settings/struct.SaveSettingsDeferred.html "struct bevy::settings::SaveSettingsDeferred")

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#252)

#### type [Out](#associatedtype.Out) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#213)

### impl [Command](trait.Command.html "trait bevy::prelude::Command") for [SaveSettingsSync](../settings/enum.SaveSettingsSync.html "enum bevy::settings::SaveSettingsSync")

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#214)

#### type [Out](#associatedtype.Out) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#113-116)

### impl<F, Out> [Command](trait.Command.html "trait bevy::prelude::Command") for F

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [World](struct.World.html "struct bevy::prelude::World")) -> Out + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, Out: [CommandOutput](../ecs/error/trait.CommandOutput.html "trait bevy::ecs::error::CommandOutput"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#118)

#### type [Out](#associatedtype.Out) = Out