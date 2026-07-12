[bevy](../../index.html)::[ecs](../index.html)

# Module error 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#36)

Error handling for Bevy systems, commands, and observers.

When a system is added to a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"), and its return type is that of [`Result`](../../prelude/type.Result.html "type bevy::prelude::Result"), then Bevy considers those systems to be “fallible”, and the ECS scheduler will special-case the [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") variant of the returned `Result`.

All [`BevyError`](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")s returned by a system, observer or command are handled by an “error handler”. By default, the [`match_severity`](fn.match_severity.html "fn bevy::ecs::error::match_severity") error handler function is used, which defers to an error’s [`Severity`](../../prelude/enum.Severity.html "enum bevy::prelude::Severity").

You can change the default behavior by registering a custom error handler: Use [`FallbackErrorHandler`](struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler") to set a custom error handler function for a world, or `App::set_error_handler` for a whole app. In practice, this is generally feature-flagged: panicking or loudly logging errors in development, and quietly logging or ignoring them in production to avoid crashing the app.

Bevy provides a number of pre-built error-handlers for you to use:

*   [`match_severity`](fn.match_severity.html "fn bevy::ecs::error::match_severity") defers to an error’s [`Severity`](../../prelude/enum.Severity.html "enum bevy::prelude::Severity"), using one of the handlers listed below.
*   [`panic`](fn.panic.html "fn bevy::ecs::error::panic") – panics with the system error
*   [`error`](fn.error.html "fn bevy::ecs::error::error") – logs the system error at the `error` level
*   [`warn`](fn.warn.html "fn bevy::ecs::error::warn") – logs the system error at the `warn` level
*   [`info`](fn.info.html "fn bevy::ecs::error::info") – logs the system error at the `info` level
*   [`debug`](fn.debug.html "fn bevy::ecs::error::debug") – logs the system error at the `debug` level
*   [`trace`](fn.trace.html "fn bevy::ecs::error::trace") – logs the system error at the `trace` level
*   [`ignore`](fn.ignore.html "fn bevy::ecs::error::ignore") – ignores the system error

However, you can use any custom error handler logic by providing your own function (or non-capturing closure that coerces to the function signature) as long as it matches the signature:

[ⓘ](# "This example is not tested")

```rust
fn(BevyError, ErrorContext)
```

The [`ErrorContext`](enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext") allows you to access additional details relevant to providing context surrounding the error – such as the system’s [`name`](../../prelude/trait.System.html#tymethod.name "method bevy::prelude::System::name") – in your error messages.

[ⓘ](# "This example is not tested")

```rust
use bevy_ecs::error::{BevyError, ErrorContext, FallbackErrorHandler};
use log::trace;

fn my_error_handler(error: BevyError, ctx: ErrorContext) {
   if ctx.name().ends_with("plz_ignore") {
      trace!("Nothing to see here, move along.");
      return;
  }
  bevy_ecs::error::error(error, ctx);
}

fn main() {
    let mut world = World::new();
    world.insert_resource(FallbackErrorHandler(my_error_handler));
    // Use your world here
}
```

If you need special handling of individual fallible systems, you can use Bevy’s [`system piping feature`](../../prelude/struct.In.html "struct bevy::prelude::In") to capture the [`Result`](../../prelude/type.Result.html "type bevy::prelude::Result") output of the system and handle it accordingly.

When working with commands, you can handle the result of each command separately using the [`Command::handle_error_with`](../../prelude/trait.Command.html#method.handle_error_with "method bevy::prelude::Command::handle_error_with") method.

## Structs

[BevyError](struct.BevyError.html "struct bevy::ecs::error::BevyError")

The built in “universal” Bevy error type. This has a blanket [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") impl for any type that implements Rust’s [`Error`](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error"), meaning it can be used as a “catch all” error.

[FallbackErrorHandler](struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler")

Fallback error handler to call when an error is not handled otherwise. Defaults to [`match_severity()`](fn.match_severity.html "fn bevy::ecs::error::match_severity").

## Enums

[ErrorContext](enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext")

Context for a [`BevyError`](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError") to aid in debugging.

[Severity](enum.Severity.html "enum bevy::ecs::error::Severity")

Indicates how severe a [`BevyError`](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError") is.

## Traits

[CommandOutput](trait.CommandOutput.html "trait bevy::ecs::error::CommandOutput")

A trait implemented for types that can be used as the output of a [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command").

[EntityCommandOutput](trait.EntityCommandOutput.html "trait bevy::ecs::error::EntityCommandOutput")

A trait implemented for types that can be used as the output of an [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand").

[ResultSeverityExt](trait.ResultSeverityExt.html "trait bevy::ecs::error::ResultSeverityExt")

Extension methods for annotating errors with a [`Severity`](../../prelude/enum.Severity.html "enum bevy::prelude::Severity").

## Functions

[bevy\_error\_panic\_hook](fn.bevy_error_panic_hook.html "fn bevy::ecs::error::bevy_error_panic_hook")`backtrace`

When called, this will skip the currently configured panic hook when a [`BevyError`](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError") backtrace has already been printed.

[debug](fn.debug.html "fn bevy::ecs::error::debug")

Error handler that logs the system error at the `debug` level.

[error](fn.error.html "fn bevy::ecs::error::error")

Error handler that logs the system error at the `error` level.

[ignore](fn.ignore.html "fn bevy::ecs::error::ignore")

Error handler that ignores the system error.

[info](fn.info.html "fn bevy::ecs::error::info")

Error handler that logs the system error at the `info` level.

[match\_severity](fn.match_severity.html "fn bevy::ecs::error::match_severity")

Error handler that defers to an error’s [`Severity`](../../prelude/enum.Severity.html "enum bevy::prelude::Severity").

[panic](fn.panic.html "fn bevy::ecs::error::panic")

Error handler that panics with the system error.

[trace](fn.trace.html "fn bevy::ecs::error::trace")

Error handler that logs the system error at the `trace` level.

[warn](fn.warn.html "fn bevy::ecs::error::warn")

Error handler that logs the system error at the `warn` level.

## Type Aliases

[DefaultErrorHandler](type.DefaultErrorHandler.html "type bevy::ecs::error::DefaultErrorHandler")Deprecated

Deprecated alias for [`FallbackErrorHandler`](struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler").

[ErrorHandler](type.ErrorHandler.html "type bevy::ecs::error::ErrorHandler")

Defines how Bevy reacts to errors.

[Result](type.Result.html "type bevy::ecs::error::Result")

A result type for use in fallible systems, commands and observers.