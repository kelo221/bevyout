[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Struct Error 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#77)

```rust
pub struct Error { /* private fields */ }
```

Available on **crate feature `std`** only.

The error type for I/O operations of the [`Read`](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"), [`Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"), [`Seek`](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek"), and associated traits.

Errors mostly originate from the underlying OS, but custom instances of `Error` can be created with crafted error messages and a particular value of [`ErrorKind`](enum.ErrorKind.html "enum bevy::tasks::futures_lite::io::ErrorKind").

## Implementations

[Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#90)

### impl [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

This impl block contains no public items.

Common errors constants for use in std

[Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#226)

### impl [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#255-257)

#### pub fn [new](#method.new)<E>(kind: [ErrorKind](enum.ErrorKind.html "enum bevy::tasks::futures_lite::io::ErrorKind"), error: E) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

where E: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>>,

Creates a new I/O error from a known kind of error as well as an arbitrary error payload.

This function is used to generically create I/O errors which do not originate from the OS itself. The `error` argument is an arbitrary payload which will be contained in this [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error").

Note that this function allocates memory on the heap. If no extra payload is required, use the `From` conversion from `ErrorKind`.

##### Examples

```rust
use std::io::{Error, ErrorKind};

// errors can be created from strings
let custom_error = Error::new(ErrorKind::Other, "oh no!");

// errors can also be created from other errors
let custom_error2 = Error::new(ErrorKind::Interrupted, custom_error);

// creating an error without payload (and without memory allocation)
let eof_error = Error::from(ErrorKind::UnexpectedEof);
```

1.74.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#280-282)

#### pub fn [other](#method.other)<E>(error: E) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

where E: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>>,

Creates a new I/O error from an arbitrary error payload.

This function is used to generically create I/O errors which do not originate from the OS itself. It is a shortcut for [`Error::new`](struct.Error.html#method.new "associated function bevy::tasks::futures_lite::io::Error::new") with [`ErrorKind::Other`](enum.ErrorKind.html#variant.Other "variant bevy::tasks::futures_lite::io::ErrorKind::Other").

##### Examples

```rust
use std::io::Error;

// errors can be created from strings
let custom_error = Error::other("oh no!");

// errors can also be created from other errors
let custom_error2 = Error::other(custom_error);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#332)

#### pub fn [last\_os\_error](#method.last_os_error)() -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Returns an error representing the last OS error which occurred.

This function reads the value of `errno` for the target platform (e.g. `GetLastError` on Windows) and will return a corresponding instance of [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error") for the error code.

This should be called immediately after a call to a platform function, otherwise the state of the error value is indeterminate. In particular, other standard library functions may call platform functions that may (or may not) reset the error value even if they succeed.

##### Examples

```rust
use std::io::Error;

let os_error = Error::last_os_error();
println!("last OS error: {os_error:?}");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#364)

#### pub fn [from\_raw\_os\_error](#method.from_raw_os_error)(code: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Creates a new instance of an [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error") from a particular OS error code.

##### Examples

On Linux:

```rust
use std::io;

let error = io::Error::from_raw_os_error(22);
assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
```

On Windows:

```rust
use std::io;

let error = io::Error::from_raw_os_error(10022);
assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#400)

#### pub fn [raw\_os\_error](#method.raw_os_error)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

Returns the OS error that this error represents (if any).

If this [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error") was constructed via [`last_os_error`](struct.Error.html#method.last_os_error "associated function bevy::tasks::futures_lite::io::Error::last_os_error") or [`from_raw_os_error`](struct.Error.html#method.from_raw_os_error "associated function bevy::tasks::futures_lite::io::Error::from_raw_os_error"), then this function will return [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some"), otherwise it will return [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None").

##### Examples

```rust
use std::io::{Error, ErrorKind};

fn print_os_error(err: &Error) {
    if let Some(raw_os_err) = err.raw_os_error() {
        println!("raw OS error: {raw_os_err:?}");
    } else {
        println!("Not an OS error");
    }
}

fn main() {
    // Will print "raw OS error: ...".
    print_os_error(&Error::last_os_error());
    // Will print "Not an OS error".
    print_os_error(&Error::new(ErrorKind::Other, "oh no!"));
}
```

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#439)

#### pub fn [get\_ref](#method.get_ref)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static)>

Returns a reference to the inner error wrapped by this error (if any).

If this [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error") was constructed via [`new`](struct.Error.html#method.new "associated function bevy::tasks::futures_lite::io::Error::new") then this function will return [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some"), otherwise it will return [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None").

##### Examples

```rust
use std::io::{Error, ErrorKind};

fn print_error(err: &Error) {
    if let Some(inner_err) = err.get_ref() {
        println!("Inner error: {inner_err:?}");
    } else {
        println!("No inner error");
    }
}

fn main() {
    // Will print "No inner error".
    print_error(&Error::last_os_error());
    // Will print "Inner error: ...".
    print_error(&Error::new(ErrorKind::Other, "oh no!"));
}
```

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#513)

#### pub fn [get\_mut](#method.get_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static)>

Returns a mutable reference to the inner error wrapped by this error (if any).

If this [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error") was constructed via [`new`](struct.Error.html#method.new "associated function bevy::tasks::futures_lite::io::Error::new") then this function will return [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some"), otherwise it will return [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None").

##### Examples

```rust
use std::io::{Error, ErrorKind};
use std::{error, fmt};
use std::fmt::Display;

#[derive(Debug)]
struct MyError {
    v: String,
}

impl MyError {
    fn new() -> MyError {
        MyError {
            v: "oh no!".to_string()
        }
    }

    fn change_message(&mut self, new_message: &str) {
        self.v = new_message.to_string();
    }
}

impl error::Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.v)
    }
}

fn change_error(mut err: Error) -> Error {
    if let Some(inner_err) = err.get_mut() {
        inner_err.downcast_mut::<MyError>().unwrap().change_message("I've been changed!");
    }
    err
}

fn print_error(err: &Error) {
    if let Some(inner_err) = err.get_ref() {
        println!("Inner error: {inner_err}");
    } else {
        println!("No inner error");
    }
}

fn main() {
    // Will print "No inner error".
    print_error(&change_error(Error::last_os_error()));
    // Will print "Inner error: ...".
    print_error(&change_error(Error::new(ErrorKind::Other, MyError::new())));
}
```

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#554)

#### pub fn [into\_inner](#method.into_inner)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>>

Consumes the `Error`, returning its inner error (if any).

If this [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error") was constructed via [`new`](struct.Error.html#method.new "associated function bevy::tasks::futures_lite::io::Error::new") or [`other`](struct.Error.html#method.other "associated function bevy::tasks::futures_lite::io::Error::other"), then this function will return [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some"), otherwise it will return [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None").

##### Examples

```rust
use std::io::{Error, ErrorKind};

fn print_error(err: Error) {
    if let Some(inner_err) = err.into_inner() {
        println!("Inner error: {inner_err}");
    } else {
        println!("No inner error");
    }
}

fn main() {
    // Will print "No inner error".
    print_error(Error::last_os_error());
    // Will print "Inner error: ...".
    print_error(Error::new(ErrorKind::Other, "oh no!"));
}
```

1.79.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#633-635)

#### pub fn [downcast](#method.downcast)<E>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<E, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where E: [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Attempts to downcast the custom boxed error to `E`.

If this [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error") contains a custom boxed error, then it would attempt downcasting on the boxed error, otherwise it will return [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err").

If the custom boxed error has the same type as `E`, it will return [`Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok"), otherwise it will also return [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err").

This method is meant to be a convenience routine for calling `Box<dyn Error + Sync + Send>::downcast` on the custom boxed error, returned by [`Error::into_inner`](struct.Error.html#method.into_inner "method bevy::tasks::futures_lite::io::Error::into_inner").

##### Examples

```rust
use std::fmt;
use std::io;
use std::error::Error;

#[derive(Debug)]
enum E {
    Io(io::Error),
    SomeOtherVariant,
}

impl fmt::Display for E {
   // ...
}
impl Error for E {}

impl From<io::Error> for E {
    fn from(err: io::Error) -> E {
        err.downcast::<E>()
            .unwrap_or_else(E::Io)
    }
}

impl From<E> for io::Error {
    fn from(err: E) -> io::Error {
        match err {
            E::Io(io_error) => io_error,
            e => io::Error::new(io::ErrorKind::Other, e),
        }
    }
}

let e = E::SomeOtherVariant;
// Convert it to an io::Error
let io_error = io::Error::from(e);
// Cast it back to the original variant
let e = E::from(io_error);
assert!(matches!(e, E::SomeOtherVariant));

let io_error = io::Error::from(io::ErrorKind::AlreadyExists);
// Convert it to E
let e = E::from(io_error);
// Cast it back to the original variant
let io_error = io::Error::from(e);
assert_eq!(io_error.kind(), io::ErrorKind::AlreadyExists);
assert!(io_error.get_ref().is_none());
assert!(io_error.raw_os_error().is_none());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#682)

#### pub fn [kind](#method.kind)(&self) -> [ErrorKind](enum.ErrorKind.html "enum bevy::tasks::futures_lite::io::ErrorKind")

Returns the corresponding [`ErrorKind`](enum.ErrorKind.html "enum bevy::tasks::futures_lite::io::ErrorKind") for this error.

This may be a value set by Rust code constructing custom `io::Error`s, or if this `io::Error` was sourced from the operating system, it will be a value inferred from the system’s error encoding. See [`last_os_error`](struct.Error.html#method.last_os_error "associated function bevy::tasks::futures_lite::io::Error::last_os_error") for more details.

##### Examples

```rust
use std::io::{Error, ErrorKind};

fn print_error(err: Error) {
    println!("{:?}", err.kind());
}

fn main() {
    // As no error has (visibly) occurred, this may print anything!
    // It likely prints a placeholder for unidentified (non-)errors.
    print_error(Error::last_os_error());
    // Will print "AddrInUse".
    print_error(Error::new(ErrorKind::AddrInUse, "oh no!"));
}
```

## Trait Implementations

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#82)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#83)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#723)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#724)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#738)

### impl [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#740)

#### fn [cause](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error")\>

👎Deprecated since 1.33.0:

replaced by Error::source, which can support downcasting

[Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#749)

#### fn [source](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + 'static)>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)

#### fn [description](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.42.0:

use the Display impl or to\_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#260)

#### fn [provide](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)<'a>(&'a self, request: &mut [Request](https://doc.rust-lang.org/nightly/core/error/struct.Request.html "struct core::error::Request")<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

[Source](https://docs.rs/flate2/1.1.9/x86_64-unknown-linux-gnu/src/flate2/mem.rs.html#619)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[CompressError](https://docs.rs/flate2/1.1.9/x86_64-unknown-linux-gnu/flate2/mem/struct.CompressError.html "struct flate2::mem::CompressError")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/flate2/1.1.9/x86_64-unknown-linux-gnu/src/flate2/mem.rs.html#620)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(data: [CompressError](https://docs.rs/flate2/1.1.9/x86_64-unknown-linux-gnu/flate2/mem/struct.CompressError.html "struct flate2::mem::CompressError")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/png/0.18.1/x86_64-unknown-linux-gnu/src/png/decoder/stream.rs.html#447)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[DecodingError](https://docs.rs/png/0.18.1/x86_64-unknown-linux-gnu/png/decoder/stream/enum.DecodingError.html "enum png::decoder::stream::DecodingError")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/png/0.18.1/x86_64-unknown-linux-gnu/src/png/decoder/stream.rs.html#448)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [DecodingError](https://docs.rs/png/0.18.1/x86_64-unknown-linux-gnu/png/decoder/stream/enum.DecodingError.html "enum png::decoder::stream::DecodingError")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/flate2/1.1.9/x86_64-unknown-linux-gnu/src/flate2/mem.rs.html#591)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[DecompressError](https://docs.rs/flate2/1.1.9/x86_64-unknown-linux-gnu/flate2/mem/struct.DecompressError.html "struct flate2::mem::DecompressError")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/flate2/1.1.9/x86_64-unknown-linux-gnu/src/flate2/mem.rs.html#592)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(data: [DecompressError](https://docs.rs/flate2/1.1.9/x86_64-unknown-linux-gnu/flate2/mem/struct.DecompressError.html "struct flate2::mem::DecompressError")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/png/0.18.1/x86_64-unknown-linux-gnu/src/png/encoder.rs.html#121)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[EncodingError](https://docs.rs/png/0.18.1/x86_64-unknown-linux-gnu/png/encoder/enum.EncodingError.html "enum png::encoder::EncodingError")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/png/0.18.1/x86_64-unknown-linux-gnu/src/png/encoder.rs.html#122)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [EncodingError](https://docs.rs/png/0.18.1/x86_64-unknown-linux-gnu/png/encoder/enum.EncodingError.html "enum png::encoder::EncodingError")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/io/errno.rs.html#58)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/io/errno.rs.html#60)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/nix/0.31.3/x86_64-unknown-linux-gnu/src/nix/errno.rs.html#184)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Errno](https://docs.rs/nix/0.31.3/x86_64-unknown-linux-gnu/nix/errno/consts/enum.Errno.html "enum nix::errno::consts::Errno")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/nix/0.31.3/x86_64-unknown-linux-gnu/src/nix/errno.rs.html#185)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [Errno](https://docs.rs/nix/0.31.3/x86_64-unknown-linux-gnu/nix/errno/consts/enum.Errno.html "enum nix::errno::consts::Errno")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/io/errno.rs.html#56)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/io/errno.rs.html#58)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/errno/0.3.14/x86_64-unknown-linux-gnu/src/errno/lib.rs.html#87)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Errno](https://docs.rs/errno/0.3.14/x86_64-unknown-linux-gnu/errno/struct.Errno.html "struct errno::Errno")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/errno/0.3.14/x86_64-unknown-linux-gnu/src/errno/lib.rs.html#88)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(errno: [Errno](https://docs.rs/errno/0.3.14/x86_64-unknown-linux-gnu/errno/struct.Errno.html "struct errno::Errno")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/getrandom/0.2.17/x86_64-unknown-linux-gnu/src/getrandom/error_std_impls.rs.html#6)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](https://docs.rs/getrandom/0.2.17/x86_64-unknown-linux-gnu/getrandom/error/struct.Error.html "struct getrandom::error::Error")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/getrandom/0.2.17/x86_64-unknown-linux-gnu/src/getrandom/error_std_impls.rs.html#7)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [Error](https://docs.rs/getrandom/0.2.17/x86_64-unknown-linux-gnu/getrandom/error/struct.Error.html "struct getrandom::error::Error")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/getrandom/0.2.17/x86_64-unknown-linux-gnu/src/getrandom/error_std_impls.rs.html#6)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](https://docs.rs/getrandom/0.2.17/x86_64-unknown-linux-gnu/getrandom/error/struct.Error.html "struct getrandom::error::Error")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/getrandom/0.2.17/x86_64-unknown-linux-gnu/src/getrandom/error_std_impls.rs.html#7)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [Error](https://docs.rs/getrandom/0.2.17/x86_64-unknown-linux-gnu/getrandom/error/struct.Error.html "struct getrandom::error::Error")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/src/serde_json/error.rs.html#189)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/serde_json/error/struct.Error.html "struct serde_json::error::Error")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/src/serde_json/error.rs.html#217)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(j: [Error](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/serde_json/error/struct.Error.html "struct serde_json::error::Error")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Convert a `serde_json::Error` into an `io::Error`.

JSON syntax and data errors are turned into `InvalidData` I/O errors. EOF errors are turned into `UnexpectedEof` I/O errors.

```rust
use std::io;

enum MyError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => {
                MyError::Io(err.into())
            }
            Category::Syntax | Category::Data | Category::Eof => {
                MyError::Json(err)
            }
        }
    }
}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#252)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [AnimationGraphSaveError](../../../prelude/enum.AnimationGraphSaveError.html "enum bevy::prelude::AnimationGraphSaveError")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#248)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [AnimationGraphSaveError](../../../prelude/enum.AnimationGraphSaveError.html "enum bevy::prelude::AnimationGraphSaveError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#266)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [AnimationGraphLoadError](../../../prelude/enum.AnimationGraphLoadError.html "enum bevy::prelude::AnimationGraphLoadError")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#262)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [AnimationGraphLoadError](../../../prelude/enum.AnimationGraphLoadError.html "enum bevy::prelude::AnimationGraphLoadError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#76)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [AssetReaderError](../../../asset/io/enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#77)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [AssetReaderError](../../../asset/io/enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#319)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [AssetWriterError](../../../asset/io/enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#315)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [AssetWriterError](../../../asset/io/enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#245)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [ReadLogError](../../../asset/processor/enum.ReadLogError.html "enum bevy::asset::processor::ReadLogError")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#238)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [ReadLogError](../../../asset/processor/enum.ReadLogError.html "enum bevy::asset::processor::ReadLogError")

Converts to this type from the input type.

[Source](https://docs.rs/walkdir/2.5.0/x86_64-unknown-linux-gnu/src/walkdir/error.rs.html#241)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](https://docs.rs/walkdir/2.5.0/x86_64-unknown-linux-gnu/walkdir/error/struct.Error.html "struct walkdir::error::Error")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/walkdir/2.5.0/x86_64-unknown-linux-gnu/src/walkdir/error.rs.html#253)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(walk\_err: [Error](https://docs.rs/walkdir/2.5.0/x86_64-unknown-linux-gnu/walkdir/error/struct.Error.html "struct walkdir::error::Error")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Convert the [`Error`](struct.Error.html) to an [`io::Error`](https://doc.rust-lang.org/stable/std/io/struct.Error.html), preserving the original [`Error`](struct.Error.html) as the [“inner error”](https://doc.rust-lang.org/std/io/struct.Error.html#method.into_inner). Note that this also makes the display of the error include the context.

This is different from [`into_io_error`](struct.WalkDir.html#method.into_io_error) which returns the original [`io::Error`](https://doc.rust-lang.org/stable/std/io/struct.Error.html).

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/compressed_image_saver.rs.html#21)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [CompressedImageSaverError](../../../image/enum.CompressedImageSaverError.html "enum bevy::image::CompressedImageSaverError")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/compressed_image_saver.rs.html#17)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [CompressedImageSaverError](../../../image/enum.CompressedImageSaverError.html "enum bevy::image::CompressedImageSaverError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/exr_texture_loader.rs.html#29)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [ExrTextureLoaderError](../../../image/enum.ExrTextureLoaderError.html "enum bevy::image::ExrTextureLoaderError")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/exr_texture_loader.rs.html#24)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [ExrTextureLoaderError](../../../image/enum.ExrTextureLoaderError.html "enum bevy::image::ExrTextureLoaderError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/hdr_texture_loader.rs.html#27)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [HdrTextureLoaderError](../../../image/enum.HdrTextureLoaderError.html "enum bevy::image::HdrTextureLoaderError")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/hdr_texture_loader.rs.html#23)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [HdrTextureLoaderError](../../../image/enum.HdrTextureLoaderError.html "enum bevy::image::HdrTextureLoaderError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_loader.rs.html#180)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [ImageLoaderError](../../../image/enum.ImageLoaderError.html "enum bevy::image::ImageLoaderError")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_loader.rs.html#176)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [ImageLoaderError](../../../image/enum.ImageLoaderError.html "enum bevy::image::ImageLoaderError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/saver.rs.html#151)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [SaveImageError](../../../image/enum.SaveImageError.html "enum bevy::image::SaveImageError")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/saver.rs.html#127)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [SaveImageError](../../../image/enum.SaveImageError.html "enum bevy::image::SaveImageError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#311)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [ShaderLoaderError](../../../shader/enum.ShaderLoaderError.html "enum bevy::shader::ShaderLoaderError")

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#307)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [ShaderLoaderError](../../../shader/enum.ShaderLoaderError.html "enum bevy::shader::ShaderLoaderError")

Converts to this type from the input type.

[Source](https://docs.rs/lz4_flex/0.13.1/x86_64-unknown-linux-gnu/src/lz4_flex/frame/mod.rs.html#74)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](https://docs.rs/lz4_flex/0.13.1/x86_64-unknown-linux-gnu/lz4_flex/frame/enum.Error.html "enum lz4_flex::frame::Error")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/lz4_flex/0.13.1/x86_64-unknown-linux-gnu/src/lz4_flex/frame/mod.rs.html#75)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(e: [Error](https://docs.rs/lz4_flex/0.13.1/x86_64-unknown-linux-gnu/lz4_flex/frame/enum.Error.html "enum lz4_flex::frame::Error")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/mod.rs.html#139)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [GltfError](../../../gltf/enum.GltfError.html "enum bevy::gltf::GltfError")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/mod.rs.html#84)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [GltfError](../../../gltf/enum.GltfError.html "enum bevy::gltf::GltfError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_loader.rs.html#43)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [WorldAssetLoaderError](../../../world_serialization/enum.WorldAssetLoaderError.html "enum bevy::world_serialization::WorldAssetLoaderError")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_loader.rs.html#39)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [WorldAssetLoaderError](../../../world_serialization/enum.WorldAssetLoaderError.html "enum bevy::world_serialization::WorldAssetLoaderError")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_loader.rs.html#19)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\> for [FontLoaderError](../../../text/enum.FontLoaderError.html "enum bevy::text::FontLoaderError")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_loader.rs.html#12)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(source: [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")) -> [FontLoaderError](../../../text/enum.FontLoaderError.html "enum bevy::text::FontLoaderError")

Converts to this type from the input type.

[Source](https://docs.rs/httpdate/1.0.3/x86_64-unknown-linux-gnu/src/httpdate/lib.rs.html#42)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](https://docs.rs/httpdate/1.0.3/x86_64-unknown-linux-gnu/httpdate/struct.Error.html "struct httpdate::Error")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/httpdate/1.0.3/x86_64-unknown-linux-gnu/src/httpdate/lib.rs.html#43)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(e: [Error](https://docs.rs/httpdate/1.0.3/x86_64-unknown-linux-gnu/httpdate/struct.Error.html "struct httpdate::Error")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

[Source](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/src/calloop/error.rs.html#44)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Error](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/calloop/error/enum.Error.html "enum calloop::error::Error")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/src/calloop/error.rs.html#46)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [Error](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/calloop/error/enum.Error.html "enum calloop::error::Error")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts Calloop’s error type into a [`std::io::Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error").

1.14.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#206)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[ErrorKind](enum.ErrorKind.html "enum bevy::tasks::futures_lite::io::ErrorKind")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Intended for use for errors not exposed to the user, where allocating onto the heap (for normal construction via Error::new) is too costly.

[Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#221)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(kind: [ErrorKind](enum.ErrorKind.html "enum bevy::tasks::futures_lite::io::ErrorKind")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts an [`ErrorKind`](enum.ErrorKind.html "enum bevy::tasks::futures_lite::io::ErrorKind") into an [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error").

This conversion creates a new error with a simple representation of error kind.

##### Examples

```rust
use std::io::{Error, ErrorKind};

let not_found = ErrorKind::NotFound;
let error = Error::from(not_found);
assert_eq!("entity not found", format!("{error}"));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/buffered/mod.rs.html#175)

### impl<W> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[IntoInnerError](https://doc.rust-lang.org/nightly/std/io/buffered/struct.IntoInnerError.html "struct std::io::buffered::IntoInnerError")<W>> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://doc.rust-lang.org/nightly/src/std/io/buffered/mod.rs.html#176)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(iie: [IntoInnerError](https://doc.rust-lang.org/nightly/std/io/buffered/struct.IntoInnerError.html "struct std::io::buffered::IntoInnerError")<W>) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#116)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[NulError](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.NulError.html "struct alloc::ffi::c_str::NulError")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#118)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: [NulError](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.NulError.html "struct alloc::ffi::c_str::NulError")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts a [`alloc::ffi::NulError`](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.NulError.html "struct alloc::ffi::c_str::NulError") into a [`Error`](struct.Error.html "struct bevy::tasks::futures_lite::io::Error").

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/lib.rs.html#161)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[TryGetError](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/struct.TryGetError.html "struct bytes::TryGetError")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/lib.rs.html#162)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(error: [TryGetError](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/struct.TryGetError.html "struct bytes::TryGetError")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

1.89.0 · [Source](https://doc.rust-lang.org/nightly/src/std/fs.rs.html#532)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[TryLockError](https://doc.rust-lang.org/nightly/std/fs/enum.TryLockError.html "enum std::fs::TryLockError")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://doc.rust-lang.org/nightly/src/std/fs.rs.html#533)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [TryLockError](https://doc.rust-lang.org/nightly/std/fs/enum.TryLockError.html "enum std::fs::TryLockError")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts to this type from the input type.

1.78.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#124)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[TryReserveError](https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html "struct alloc::collections::TryReserveError")\> for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://doc.rust-lang.org/nightly/src/std/io/error.rs.html#129)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: [TryReserveError](https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html "struct alloc::collections::TryReserveError")) -> [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

Converts `TryReserveError` to an error with [`ErrorKind::OutOfMemory`](enum.ErrorKind.html#variant.OutOfMemory "variant bevy::tasks::futures_lite::io::ErrorKind::OutOfMemory").

`TryReserveError` won’t be available as the error `source()`, but this may change in the future.

[Source](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/src/tungstenite/util.rs.html#16)

### impl [NonBlockingError](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/tungstenite/util/trait.NonBlockingError.html "trait tungstenite::util::NonBlockingError") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/src/tungstenite/util.rs.html#17)

#### fn [into\_non\_blocking](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/tungstenite/util/trait.NonBlockingError.html#tymethod.into_non_blocking)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Convert WouldBlock to None and don’t touch other errors.

## Auto Trait Implementations

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/src/tracing_error/error.rs.html#273-275)

### impl<E> [InstrumentError](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/tracing_error/error/trait.InstrumentError.html "trait tracing_error::error::InstrumentError") for E

where [TracedError](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/tracing_error/error/struct.TracedError.html "struct tracing_error::error::TracedError")<E>: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<E>,

[Source](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/src/tracing_error/error.rs.html#277)

#### type [Instrumented](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/tracing_error/error/trait.InstrumentError.html#associatedtype.Instrumented) = [TracedError](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/tracing_error/error/struct.TracedError.html "struct tracing_error::error::TracedError")<E>

The type of the wrapped error after instrumentation

[Source](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/src/tracing_error/error.rs.html#279)

#### fn [in\_current\_span](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/tracing_error/error/trait.InstrumentError.html#tymethod.in_current_span)(self) -> <E as [InstrumentError](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/tracing_error/error/trait.InstrumentError.html "trait tracing_error::error::InstrumentError")\>::[Instrumented](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/tracing_error/error/trait.InstrumentError.html#associatedtype.Instrumented "type tracing_error::error::InstrumentError::Instrumented")

Instrument an Error by bundling it with a SpanTrace [Read more](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/tracing_error/error/trait.InstrumentError.html#tymethod.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)

### impl<T> [ToSmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/trait.ToSmolStr.html "trait smol_str::ToSmolStr") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)

#### fn [to\_smolstr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/trait.ToSmolStr.html#tymethod.to_smolstr)(&self) -> [SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2900)

### impl<T> [ToString](../../../prelude/trait.ToString.html "trait bevy::prelude::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2902)

#### fn [to\_string](../../../prelude/trait.ToString.html#tymethod.to_string)(&self) -> [String](../../../prelude/struct.String.html "struct bevy::prelude::String")

Converts the given value to a `String`. [Read more](../../../prelude/trait.ToString.html#tymethod.to_string)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}