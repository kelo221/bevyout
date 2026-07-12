[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Type Alias Result 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#64)

```rust
pub type Result<T> = Result<T, Error>;
```

Available on **crate feature `std`** only.

A specialized [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") type for I/O operations.

This type is broadly used across [`std::io`](https://doc.rust-lang.org/nightly/std/io/index.html "mod std::io") for any operation which may produce an error.

This type alias is generally used to avoid writing out [`io::Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error") directly and is otherwise a direct mapping to [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result").

While usual Rust style is to import types directly, aliases of [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") often are not, to make it easier to distinguish between them. [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") is generally assumed to be [`std::result::Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result"), and so users of this alias will generally use `io::Result` instead of shadowing the [prelude](https://doc.rust-lang.org/nightly/std/prelude/index.html "mod std::prelude")’s import of [`std::result::Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result").

## Examples

A convenience function that bubbles an `io::Result` to its caller:

```rust
use std::io;

fn get_string() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}
```

## Aliased Type

```rust
pub enum Result<T> {
    Ok(T),
    Err(Error),
}
```

## Variants

1.0.0

### Ok(T)

Contains the success value

1.0.0

### Err([Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error"))

Contains the error value