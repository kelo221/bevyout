[bevy](../../index.html)::[platform](../index.html)::[prelude](index.html)

# Macro format 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/macros.rs.html#111)

```rust
macro_rules! format {
    ($($arg:tt)*) => { ... };
}
```

Creates a `String` using interpolation of runtime expressions.

The first argument `format!` receives is a format string. This must be a string literal. The power of the formatting string is in the `{}`s contained. Additional parameters passed to `format!` replace the `{}`s within the formatting string in the order given unless named or positional parameters are used.

See [the formatting syntax documentation in `std::fmt`](../std/fmt/index.html) for details.

A common use for `format!` is concatenation and interpolation of strings. The same convention is used with [`print!`](../std/macro.print.html) and [`write!`](https://doc.rust-lang.org/nightly/core/macro.write.html "macro core::write") macros, depending on the intended destination of the string; all these macros internally use [`format_args!`](https://doc.rust-lang.org/nightly/core/macro.format_args.html "macro core::format_args").

To convert a single value to a string, use the [`to_string`](../../prelude/trait.ToString.html "trait bevy::prelude::ToString") method. This will use the [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") formatting trait.

To concatenate literals into a `&'static str`, use the [`concat!`](https://doc.rust-lang.org/nightly/core/macro.concat.html "macro core::concat") macro.

## Panics

`format!` panics if a formatting trait implementation returns an error. This indicates an incorrect implementation since `fmt::Write for String` never returns an error itself.

## Examples

```rust
format!("test");                             // => "test"
format!("hello {}", "world!");               // => "hello world!"
format!("x = {}, y = {val}", 10, val = 30);  // => "x = 10, y = 30"
let (x, y) = (1, 2);
format!("{x} + {y} = 3");                    // => "1 + 2 = 3"
```