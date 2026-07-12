[bevy](../index.html)::[prelude](index.html)

# Trait ResultSeverityExt 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#288)

```rust
pub trait ResultSeverityExt<T, E>: Sized {
    // Required methods
    fn with_severity(self, severity: Severity) -> Result<T, BevyError>;
    fn map_severity(
        self,
        f: impl FnOnce(&E) -> Severity,
    ) -> Result<T, BevyError>;

    // Provided methods
    fn ignore(self) -> Result<T, BevyError> { ... }
    fn trace(self) -> Result<T, BevyError> { ... }
    fn info(self) -> Result<T, BevyError> { ... }
    fn warn(self) -> Result<T, BevyError> { ... }
    fn error(self) -> Result<T, BevyError> { ... }
    fn panic(self) -> Result<T, BevyError> { ... }
}
```

Extension methods for annotating errors with a [`Severity`](enum.Severity.html "enum bevy::prelude::Severity").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#305)

#### fn [with\_severity](#tymethod.with_severity)(self, severity: [Severity](enum.Severity.html "enum bevy::prelude::Severity")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Overrides the [`Severity`](enum.Severity.html "enum bevy::prelude::Severity") of the error if this result is `Err`. This does not change control flow; it only annotates the error.

##### Example

```rust
fn fallible() -> Result<(), BevyError> {
    // This failure is expected in some contexts, so we downgrade its severity.
    let _parsed: usize = "I am not a number"
        .parse()
        .with_severity(Severity::Warning)?;
    Ok(())
}
```

For more fine grained control see [`Result::map_severity`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.map_severity "method core::result::Result::map_severity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#338)

#### fn [map\_severity](#tymethod.map_severity)(self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&E](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Severity](enum.Severity.html "enum bevy::prelude::Severity")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Overrides the [`Severity`](enum.Severity.html "enum bevy::prelude::Severity") of the error if this result is `Err`. This does not change control flow; it only annotates the error.

##### Example

```rust
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Incorrect version")]
    IncorrectVersion,
    #[error("Syntax error")]
    SyntaxError,
}

fn fallible() -> Result<(), BevyError> {
    // This failure is expected in some contexts, so we downgrade its severity.
    let _parsed: usize = validate("I am not a number")
        .map_severity(|e| match e {
            ValidationError::IncorrectVersion => Severity::Debug,
            ValidationError::SyntaxError => Severity::Error,
        })?;
    Ok(())
}
```

If you don’t need to inspect the error, use [`Result::with_severity`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.with_severity "method core::result::Result::with_severity")

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#343)

#### fn [ignore](#method.ignore)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Overrides the severity of the error with [`Severity::Ignore`](enum.Severity.html#variant.Ignore "variant bevy::prelude::Severity::Ignore"). See [`Result::with_severity`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.with_severity "method core::result::Result::with_severity")

This is shorthand for `self.with_severity(Severity::Ignore)`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#350)

#### fn [trace](#method.trace)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Overrides the severity of the error with [`Severity::Trace`](enum.Severity.html#variant.Trace "variant bevy::prelude::Severity::Trace"). See [`Result::with_severity`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.with_severity "method core::result::Result::with_severity")

This is shorthand for `self.with_severity(Severity::Trace)`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#357)

#### fn [info](#method.info)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Overrides the severity of the error with [`Severity::Info`](enum.Severity.html#variant.Info "variant bevy::prelude::Severity::Info"). See [`Result::with_severity`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.with_severity "method core::result::Result::with_severity")

This is shorthand for `self.with_severity(Severity::Info)`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#364)

#### fn [warn](#method.warn)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Overrides the severity of the error with [`Severity::Warning`](enum.Severity.html#variant.Warning "variant bevy::prelude::Severity::Warning"). See [`Result::with_severity`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.with_severity "method core::result::Result::with_severity")

This is shorthand for `self.with_severity(Severity::Warning)`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#371)

#### fn [error](#method.error)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Overrides the severity of the error with [`Severity::Error`](enum.Severity.html#variant.Error "variant bevy::prelude::Severity::Error"). See [`Result::with_severity`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.with_severity "method core::result::Result::with_severity")

This is shorthand for `self.with_severity(Severity::Error)`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#378)

#### fn [panic](#method.panic)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Overrides the severity of the error with [`Severity::Panic`](enum.Severity.html#variant.Panic "variant bevy::prelude::Severity::Panic"). See [`Result::with_severity`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.with_severity "method core::result::Result::with_severity")

This is shorthand for `self.with_severity(Severity::Panic)`

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#383-385)

### impl<T, E> [ResultSeverityExt](trait.ResultSeverityExt.html "trait bevy::prelude::ResultSeverityExt")<T, E> for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>

where E: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#387)

#### fn [with\_severity](#tymethod.with_severity)(self, severity: [Severity](enum.Severity.html "enum bevy::prelude::Severity")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#391)

#### fn [map\_severity](#tymethod.map_severity)(self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&E](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Severity](enum.Severity.html "enum bevy::prelude::Severity")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

## Implementors