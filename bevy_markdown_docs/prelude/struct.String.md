[bevy](../index.html)::[prelude](index.html)

# Struct String 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#353)

```rust
pub struct String { /* private fields */ }
```

A UTF-8–encoded, growable string.

`String` is the most common string type. It has ownership over the contents of the string, stored in a heap-allocated buffer (see [Representation](#representation)). It is closely related to its borrowed counterpart, the primitive [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "str").

## Examples

You can create a `String` from [a literal string](https://doc.rust-lang.org/nightly/std/primitive.str.html "&str") with [`String::from`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from "associated function core::convert::From::from"):

```rust
let hello = String::from("Hello, world!");
```

You can append a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") to a `String` with the [`push`](struct.String.html#method.push "method bevy::prelude::String::push") method, and append a [`&str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "&str") with the [`push_str`](struct.String.html#method.push_str "method bevy::prelude::String::push_str") method:

```rust
let mut hello = String::from("Hello, ");

hello.push('w');
hello.push_str("orld!");
```

If you have a vector of UTF-8 bytes, you can create a `String` from it with the [`from_utf8`](struct.String.html#method.from_utf8 "associated function bevy::prelude::String::from_utf8") method:

```rust
// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

// We know these bytes are valid, so we'll use `unwrap()`.
let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

assert_eq!("💖", sparkle_heart);
```

## UTF-8

`String`s are always valid UTF-8. If you need a non-UTF-8 string, consider [`OsString`](../../std/ffi/struct.OsString.html "ffi::OsString"). It is similar, but without the UTF-8 constraint. Because UTF-8 is a variable width encoding, `String`s are typically smaller than an array of the same `char`s:

```rust
// `s` is ASCII which represents each `char` as one byte
let s = "hello";
assert_eq!(s.len(), 5);

// A `char` array with the same contents would be longer because
// every `char` is four bytes
let s = ['h', 'e', 'l', 'l', 'o'];
let size: usize = s.into_iter().map(|c| size_of_val(&c)).sum();
assert_eq!(size, 20);

// However, for non-ASCII strings, the difference will be smaller
// and sometimes they are the same
let s = "💖💖💖💖💖";
assert_eq!(s.len(), 20);

let s = ['💖', '💖', '💖', '💖', '💖'];
let size: usize = s.into_iter().map(|c| size_of_val(&c)).sum();
assert_eq!(size, 20);
```

This raises interesting questions as to how `s[i]` should work. What should `i` be here? Several options include byte indices and `char` indices but, because of UTF-8 encoding, only byte indices would provide constant time indexing. Getting the `i`th `char`, for example, is available using [`chars`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.chars "method str::chars"):

```rust
let s = "hello";
let third_character = s.chars().nth(2);
assert_eq!(third_character, Some('l'));

let s = "💖💖💖💖💖";
let third_character = s.chars().nth(2);
assert_eq!(third_character, Some('💖'));
```

Next, what should `s[i]` return? Because indexing returns a reference to underlying data it could be `&u8`, `&[u8]`, or something similar. Since we’re only providing one index, `&u8` makes the most sense but that might not be what the user expects and can be explicitly achieved with [`as_bytes()`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.as_bytes "method str::as_bytes"):

```rust
// The first byte is 104 - the byte value of `'h'`
let s = "hello";
assert_eq!(s.as_bytes()[0], 104);
// or
assert_eq!(s.as_bytes()[0], b'h');

// The first byte is 240 which isn't obviously useful
let s = "💖💖💖💖💖";
assert_eq!(s.as_bytes()[0], 240);
```

Due to these ambiguities/restrictions, indexing with a `usize` is simply forbidden:

[ⓘ](# "This example deliberately fails to compile")

```rust
let s = "hello";

// The following will not compile!
println!("The first letter of s is {}", s[0]);
```

It is more clear, however, how `&s[i..j]` should work (that is, indexing with a range). It should accept byte indices (to be constant-time) and return a `&str` which is UTF-8 encoded. This is also called “string slicing”. Note this will panic if the byte indices provided are not character boundaries - see [`is_char_boundary`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary "method str::is_char_boundary") for more details. See the implementations for [`SliceIndex<str>`](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex") for more details on string slicing. For a non-panicking version of string slicing, see [`get`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.get "method str::get").

The [`bytes`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.bytes "method str::bytes") and [`chars`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.chars "method str::chars") methods return iterators over the bytes and codepoints of the string, respectively. To iterate over codepoints along with byte indices, use [`char_indices`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.char_indices "method str::char_indices").

## Deref

`String` implements `[Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "ops::Deref")<Target = [str](https://doc.rust-lang.org/nightly/std/primitive.str.html "str")>`, and so inherits all of [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "str")’s methods. In addition, this means that you can pass a `String` to a function which takes a [`&str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "&str") by using an ampersand (`&`):

```rust
fn takes_str(s: &str) { }

let s = String::from("Hello");

takes_str(&s);
```

This will create a [`&str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "&str") from the `String` and pass it in. This conversion is very inexpensive, and so generally, functions will accept [`&str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "&str")s as arguments unless they need a `String` for some specific reason.

In certain cases Rust doesn’t have enough information to make this conversion, known as [`Deref`](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "ops::Deref") coercion. In the following example a string slice [`&'a str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "&str") implements the trait `TraitExample`, and the function `example_func` takes anything that implements the trait. In this case Rust would need to make two implicit conversions, which Rust doesn’t have the means to do. For that reason, the following example will not compile.

[ⓘ](# "This example deliberately fails to compile")

```rust
trait TraitExample {}

impl<'a> TraitExample for &'a str {}

fn example_func<A: TraitExample>(example_arg: A) {}

let example_string = String::from("example_string");
example_func(&example_string);
```

There are two options that would work instead. The first would be to change the line `example_func(&example_string);` to `example_func(example_string.as_str());`, using the method [`as_str()`](struct.String.html#method.as_str "method bevy::prelude::String::as_str") to explicitly extract the string slice containing the string. The second way changes `example_func(&example_string);` to `example_func(&*example_string);`. In this case we are dereferencing a `String` to a [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "str"), then referencing the [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "str") back to [`&str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "&str"). The second way is more idiomatic, however both work to do the conversion explicitly rather than relying on the implicit conversion.

## Representation

A `String` is made up of three components: a pointer to some bytes, a length, and a capacity. The pointer points to the internal buffer which `String` uses to store its data. The length is the number of bytes currently stored in the buffer, and the capacity is the size of the buffer in bytes. As such, the length will always be less than or equal to the capacity.

This buffer is always stored on the heap.

You can look at these with the [`as_ptr`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.as_ptr "method str::as_ptr"), [`len`](struct.String.html#method.len "method bevy::prelude::String::len"), and [`capacity`](struct.String.html#method.capacity "method bevy::prelude::String::capacity") methods:

```rust
let story = String::from("Once upon a time...");

// Deconstruct the String into parts.
let (ptr, len, capacity) = story.into_raw_parts();

// story has nineteen bytes
assert_eq!(19, len);

// We can re-build a String out of ptr, len, and capacity. This is all
// unsafe because we are responsible for making sure the components are
// valid:
let s = unsafe { String::from_raw_parts(ptr, len, capacity) } ;

assert_eq!(String::from("Once upon a time..."), s);
```

If a `String` has enough capacity, adding elements to it will not re-allocate. For example, consider this program:

```rust
let mut s = String::new();

println!("{}", s.capacity());

for _ in 0..5 {
    s.push_str("hello");
    println!("{}", s.capacity());
}
```

This will output the following:

```
0
8
16
16
32
32
```

At first, we have no memory allocated at all, but as we append to the string, it increases its capacity appropriately. If we instead use the [`with_capacity`](struct.String.html#method.with_capacity "associated function bevy::prelude::String::with_capacity") method to allocate the correct capacity initially:

```rust
let mut s = String::with_capacity(25);

println!("{}", s.capacity());

for _ in 0..5 {
    s.push_str("hello");
    println!("{}", s.capacity());
}
```

We end up with a different output:

```
25
25
25
25
25
25
```

Here, there’s no need to allocate more memory inside the loop.

## Implementations

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#424)

### impl [String](struct.String.html "struct bevy::prelude::String")

1.0.0 (const: 1.39.0) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#446)

#### pub const fn [new](#method.new)() -> [String](struct.String.html "struct bevy::prelude::String")

Creates a new empty `String`.

Given that the `String` is empty, this will not allocate any initial buffer. While that means that this initial operation is very inexpensive, it may cause excessive allocation later when you add data. If you have an idea of how much data the `String` will hold, consider the [`with_capacity`](struct.String.html#method.with_capacity "associated function bevy::prelude::String::with_capacity") method to prevent excessive re-allocation.

##### Examples

```rust
let s = String::new();
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#493)

#### pub fn [with\_capacity](#method.with_capacity)(capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Creates a new empty `String` with at least the specified capacity.

`String`s have an internal buffer to hold their data. The capacity is the length of that buffer, and can be queried with the [`capacity`](struct.String.html#method.capacity "method bevy::prelude::String::capacity") method. This method creates an empty `String`, but one with an initial buffer that can hold at least `capacity` bytes. This is useful when you may be appending a bunch of data to the `String`, reducing the number of reallocations it needs to do.

If the given capacity is `0`, no allocation will occur, and this method is identical to the [`new`](struct.String.html#method.new "associated function bevy::prelude::String::new") method.

##### Panics

Panics if the capacity exceeds `isize::MAX` _bytes_.

##### Examples

```rust
let mut s = String::with_capacity(10);

// The String contains no chars, even though it has capacity for more
assert_eq!(s.len(), 0);

// These are all done without reallocating...
let cap = s.capacity();
for _ in 0..10 {
    s.push('a');
}

assert_eq!(s.capacity(), cap);

// ...but this may make the string reallocate
s.push('a');
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#506)

#### pub fn [try\_with\_capacity](#method.try_with_capacity)(capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), [TryReserveError](https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html "struct alloc::collections::TryReserveError")\>

🔬This is a nightly-only experimental API. (`try_with_capacity`)

Creates a new empty `String` with at least the specified capacity.

##### Errors

Returns [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") if the capacity exceeds `isize::MAX` bytes, or if the memory allocator reports failure.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#569)

#### pub fn [from\_utf8](#method.from_utf8)(vec: [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), [FromUtf8Error](https://doc.rust-lang.org/nightly/alloc/string/struct.FromUtf8Error.html "struct alloc::string::FromUtf8Error")\>

Converts a vector of bytes to a `String`.

A string ([`String`](struct.String.html "struct bevy::prelude::String")) is made of bytes ([`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8")), and a vector of bytes ([`Vec<u8>`](struct.Vec.html "Vec")) is made of bytes, so this function converts between the two. Not all byte slices are valid `String`s, however: `String` requires that it is valid UTF-8. `from_utf8()` checks to ensure that the bytes are valid UTF-8, and then does the conversion.

If you are sure that the byte slice is valid UTF-8, and you don’t want to incur the overhead of the validity check, there is an unsafe version of this function, [`from_utf8_unchecked`](struct.String.html#method.from_utf8_unchecked "associated function bevy::prelude::String::from_utf8_unchecked"), which has the same behavior but skips the check.

This method will take care to not copy the vector, for efficiency’s sake.

If you need a [`&str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "&str") instead of a `String`, consider [`str::from_utf8`](https://doc.rust-lang.org/nightly/core/str/converts/fn.from_utf8.html "fn core::str::converts::from_utf8").

The inverse of this method is [`into_bytes`](struct.String.html#method.into_bytes "method bevy::prelude::String::into_bytes").

##### Errors

Returns [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") if the slice is not UTF-8 with a description as to why the provided bytes are not UTF-8. The vector you moved in is also included.

##### Examples

Basic usage:

```rust
// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

// We know these bytes are valid, so we'll use `unwrap()`.
let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

assert_eq!("💖", sparkle_heart);
```

Incorrect bytes:

```rust
// some invalid bytes, in a vector
let sparkle_heart = vec![0, 159, 146, 150];

assert!(String::from_utf8(sparkle_heart).is_err());
```

See the docs for [`FromUtf8Error`](https://doc.rust-lang.org/nightly/alloc/string/struct.FromUtf8Error.html "struct alloc::string::FromUtf8Error") for more details on what you can do with this error.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#628)

#### pub fn [from\_utf8\_lossy](#method.from_utf8_lossy)(v: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **non-`no_global_oom_handling`** only.

Converts a slice of bytes to a string, including invalid characters.

Strings are made of bytes ([`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8")), and a slice of bytes ([`&[u8]`](https://doc.rust-lang.org/nightly/std/primitive.slice.html "primitive slice")) is made of bytes, so this function converts between the two. Not all byte slices are valid strings, however: strings are required to be valid UTF-8. During this conversion, `from_utf8_lossy()` will replace any invalid UTF-8 sequences with [`U+FFFD REPLACEMENT CHARACTER`](https://doc.rust-lang.org/nightly/core/char/constant.REPLACEMENT_CHARACTER.html "constant core::char::REPLACEMENT_CHARACTER"), which looks like this: �

If you are sure that the byte slice is valid UTF-8, and you don’t want to incur the overhead of the conversion, there is an unsafe version of this function, [`from_utf8_unchecked`](struct.String.html#method.from_utf8_unchecked "associated function bevy::prelude::String::from_utf8_unchecked"), which has the same behavior but skips the checks.

This function returns a [`Cow<'a, str>`](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "borrow::Cow"). If our byte slice is invalid UTF-8, then we need to insert the replacement characters, which will change the size of the string, and hence, require a `String`. But if it’s already valid UTF-8, we don’t need a new allocation. This return type allows us to handle both cases.

##### Examples

Basic usage:

```rust
// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);

assert_eq!("💖", sparkle_heart);
```

Incorrect bytes:

```rust
// some invalid bytes
let input = b"Hello \xF0\x90\x80World";
let output = String::from_utf8_lossy(input);

assert_eq!("Hello �World", output);
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#693)

#### pub fn [from\_utf8\_lossy\_owned](#method.from_utf8_lossy_owned)(v: [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>) -> [String](struct.String.html "struct bevy::prelude::String")

🔬This is a nightly-only experimental API. (`string_from_utf8_lossy_owned`)

Available on **non-`no_global_oom_handling`** only.

Converts a [`Vec<u8>`](struct.Vec.html "struct bevy::prelude::Vec") to a `String`, substituting invalid UTF-8 sequences with replacement characters.

See [`from_utf8_lossy`](struct.String.html#method.from_utf8_lossy "associated function bevy::prelude::String::from_utf8_lossy") for more details.

Note that this function does not guarantee reuse of the original `Vec` allocation.

##### Examples

Basic usage:

```rust
#![feature(string_from_utf8_lossy_owned)]
// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

let sparkle_heart = String::from_utf8_lossy_owned(sparkle_heart);

assert_eq!(String::from("💖"), sparkle_heart);
```

Incorrect bytes:

```rust
#![feature(string_from_utf8_lossy_owned)]
// some invalid bytes
let input: Vec<u8> = b"Hello \xF0\x90\x80World".into();
let output = String::from_utf8_lossy_owned(input);

assert_eq!(String::from("Hello �World"), output);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#725)

#### pub fn [from\_utf16](#method.from_utf16)(v: &\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), [FromUtf16Error](https://doc.rust-lang.org/nightly/alloc/string/struct.FromUtf16Error.html "struct alloc::string::FromUtf16Error")\>

Available on **non-`no_global_oom_handling`** only.

Decode a native endian UTF-16–encoded vector `v` into a `String`, returning [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") if `v` contains any invalid data.

##### Examples

```rust
// 𝄞music
let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
          0x0073, 0x0069, 0x0063];
assert_eq!(String::from("𝄞music"),
           String::from_utf16(v).unwrap());

// 𝄞mu<invalid>ic
let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
          0xD800, 0x0069, 0x0063];
assert!(String::from_utf16(v).is_err());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#764)

#### pub fn [from\_utf16\_lossy](#method.from_utf16_lossy)(v: &\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\]) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Decode a native endian UTF-16–encoded slice `v` into a `String`, replacing invalid data with [the replacement character (`U+FFFD`)](https://doc.rust-lang.org/nightly/core/char/constant.REPLACEMENT_CHARACTER.html "constant core::char::REPLACEMENT_CHARACTER").

Unlike [`from_utf8_lossy`](struct.String.html#method.from_utf8_lossy "associated function bevy::prelude::String::from_utf8_lossy") which returns a [`Cow<'a, str>`](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "borrow::Cow"), `from_utf16_lossy` returns a `String` since the UTF-16 to UTF-8 conversion requires a memory allocation.

##### Examples

```rust
// 𝄞mus<invalid>ic<invalid>
let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
          0x0073, 0xDD1E, 0x0069, 0x0063,
          0xD834];

assert_eq!(String::from("𝄞mus\u{FFFD}ic\u{FFFD}"),
           String::from_utf16_lossy(v));
```

1.98.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#791)

#### pub fn [from\_utf16le](#method.from_utf16le)(v: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), [FromUtf16Error](https://doc.rust-lang.org/nightly/alloc/string/struct.FromUtf16Error.html "struct alloc::string::FromUtf16Error")\>

Available on **non-`no_global_oom_handling`** only.

Decode a UTF-16LE–encoded vector `v` into a `String`, returning [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") if `v` contains any invalid data.

##### Examples

Basic usage:

```rust
// 𝄞music
let v = &[0x34, 0xD8, 0x1E, 0xDD, 0x6d, 0x00, 0x75, 0x00,
          0x73, 0x00, 0x69, 0x00, 0x63, 0x00];
assert_eq!(String::from("𝄞music"),
           String::from_utf16le(v).unwrap());

// 𝄞mu<invalid>ic
let v = &[0x34, 0xD8, 0x1E, 0xDD, 0x6d, 0x00, 0x75, 0x00,
          0x00, 0xD8, 0x69, 0x00, 0x63, 0x00];
assert!(String::from_utf16le(v).is_err());
```

1.98.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#829)

#### pub fn [from\_utf16le\_lossy](#method.from_utf16le_lossy)(v: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Decode a UTF-16LE–encoded slice `v` into a `String`, replacing invalid data with [the replacement character (`U+FFFD`)](https://doc.rust-lang.org/nightly/core/char/constant.REPLACEMENT_CHARACTER.html "constant core::char::REPLACEMENT_CHARACTER").

Unlike [`from_utf8_lossy`](struct.String.html#method.from_utf8_lossy "associated function bevy::prelude::String::from_utf8_lossy") which returns a [`Cow<'a, str>`](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "borrow::Cow"), `from_utf16le_lossy` returns a `String` since the UTF-16 to UTF-8 conversion requires a memory allocation.

##### Examples

Basic usage:

```rust
// 𝄞mus<invalid>ic<invalid>
let v = &[0x34, 0xD8, 0x1E, 0xDD, 0x6d, 0x00, 0x75, 0x00,
          0x73, 0x00, 0x1E, 0xDD, 0x69, 0x00, 0x63, 0x00,
          0x34, 0xD8];

assert_eq!(String::from("𝄞mus\u{FFFD}ic\u{FFFD}"),
           String::from_utf16le_lossy(v));
```

1.98.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#864)

#### pub fn [from\_utf16be](#method.from_utf16be)(v: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), [FromUtf16Error](https://doc.rust-lang.org/nightly/alloc/string/struct.FromUtf16Error.html "struct alloc::string::FromUtf16Error")\>

Available on **non-`no_global_oom_handling`** only.

Decode a UTF-16BE–encoded vector `v` into a `String`, returning [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") if `v` contains any invalid data.

##### Examples

Basic usage:

```rust
// 𝄞music
let v = &[0xD8, 0x34, 0xDD, 0x1E, 0x00, 0x6d, 0x00, 0x75,
          0x00, 0x73, 0x00, 0x69, 0x00, 0x63];
assert_eq!(String::from("𝄞music"),
           String::from_utf16be(v).unwrap());

// 𝄞mu<invalid>ic
let v = &[0xD8, 0x34, 0xDD, 0x1E, 0x00, 0x6d, 0x00, 0x75,
          0xD8, 0x00, 0x00, 0x69, 0x00, 0x63];
assert!(String::from_utf16be(v).is_err());
```

1.98.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#902)

#### pub fn [from\_utf16be\_lossy](#method.from_utf16be_lossy)(v: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Decode a UTF-16BE–encoded slice `v` into a `String`, replacing invalid data with [the replacement character (`U+FFFD`)](https://doc.rust-lang.org/nightly/core/char/constant.REPLACEMENT_CHARACTER.html "constant core::char::REPLACEMENT_CHARACTER").

Unlike [`from_utf8_lossy`](struct.String.html#method.from_utf8_lossy "associated function bevy::prelude::String::from_utf8_lossy") which returns a [`Cow<'a, str>`](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "borrow::Cow"), `from_utf16le_lossy` returns a `String` since the UTF-16 to UTF-8 conversion requires a memory allocation.

##### Examples

Basic usage:

```rust
// 𝄞mus<invalid>ic<invalid>
let v = &[0xD8, 0x34, 0xDD, 0x1E, 0x00, 0x6d, 0x00, 0x75,
          0x00, 0x73, 0xDD, 0x1E, 0x00, 0x69, 0x00, 0x63,
          0xD8, 0x34];

assert_eq!(String::from("𝄞mus\u{FFFD}ic\u{FFFD}"),
           String::from_utf16be_lossy(v));
```

1.93.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#943)

#### pub fn [into\_raw\_parts](#method.into_raw_parts)(self) -> ([\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Decomposes a `String` into its raw components: `(pointer, length, capacity)`.

Returns the raw pointer to the underlying data, the length of the string (in bytes), and the allocated capacity of the data (in bytes). These are the same arguments in the same order as the arguments to [`from_raw_parts`](struct.String.html#method.from_raw_parts "associated function bevy::prelude::String::from_raw_parts").

After calling this function, the caller is responsible for the memory previously managed by the `String`. The only way to do this is to convert the raw pointer, length, and capacity back into a `String` with the [`from_raw_parts`](struct.String.html#method.from_raw_parts "associated function bevy::prelude::String::from_raw_parts") function, allowing the destructor to perform the cleanup.

##### Examples

```rust
let s = String::from("hello");

let (ptr, len, cap) = s.into_raw_parts();

let rebuilt = unsafe { String::from_raw_parts(ptr, len, cap) };
assert_eq!(rebuilt, "hello");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#985)

#### pub unsafe fn [from\_raw\_parts](#method.from_raw_parts)( buf: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), length: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [String](struct.String.html "struct bevy::prelude::String")

Creates a new `String` from a pointer, a length and a capacity.

##### Safety

This is highly unsafe, due to the number of invariants that aren’t checked:

*   all safety requirements for [`Vec::<u8>::from_raw_parts`](struct.Vec.html#method.from_raw_parts "associated function bevy::prelude::Vec::from_raw_parts").
*   all safety requirements for [`String::from_utf8_unchecked`](struct.String.html#method.from_utf8_unchecked "associated function bevy::prelude::String::from_utf8_unchecked").

Violating these may cause problems like corrupting the allocator’s internal data structures. For example, it is normally **not** safe to build a `String` from a pointer to a C `char` array containing UTF-8 _unless_ you are certain that array was originally allocated by the Rust standard library’s allocator.

The ownership of `buf` is effectively transferred to the `String` which may then deallocate, reallocate or change the contents of memory pointed to by the pointer at will. Ensure that nothing else uses the pointer after calling this function.

##### Examples

```rust
unsafe {
    let s = String::from("hello");

    // Deconstruct the String into parts.
    let (ptr, len, capacity) = s.into_raw_parts();

    let s = String::from_raw_parts(ptr, len, capacity);

    assert_eq!(String::from("hello"), s);
}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1018)

#### pub unsafe fn [from\_utf8\_unchecked](#method.from_utf8_unchecked)(bytes: [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>) -> [String](struct.String.html "struct bevy::prelude::String")

Converts a vector of bytes to a `String` without checking that the string contains valid UTF-8.

See the safe version, [`from_utf8`](struct.String.html#method.from_utf8 "associated function bevy::prelude::String::from_utf8"), for more details.

##### Safety

This function is unsafe because it does not check that the bytes passed to it are valid UTF-8. If this constraint is violated, it may cause memory unsafety issues with future users of the `String`, as the rest of the standard library assumes that `String`s are valid UTF-8.

##### Examples

```rust
// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

let sparkle_heart = unsafe {
    String::from_utf8_unchecked(sparkle_heart)
};

assert_eq!("💖", sparkle_heart);
```

1.0.0 (const: 1.87.0) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1039)

#### pub const fn [into\_bytes](#method.into_bytes)(self) -> [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> [ⓘ](#)

Converts a `String` into a byte vector.

This consumes the `String`, so we do not need to copy its contents.

##### Examples

```rust
let s = String::from("hello");
let bytes = s.into_bytes();

assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
```

1.7.0 (const: 1.87.0) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1057)

#### pub const fn [as\_str](#method.as_str)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Extracts a string slice containing the entire `String`.

##### Examples

```rust
let s = String::from("foo");

assert_eq!("foo", s.as_str());
```

1.7.0 (const: 1.87.0) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1080)

#### pub const fn [as\_mut\_str](#method.as_mut_str)(&mut self) -> &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Converts a `String` into a mutable string slice.

##### Examples

```rust
let mut s = String::from("foobar");
let s_mut_str = s.as_mut_str();

s_mut_str.make_ascii_uppercase();

assert_eq!("FOOBAR", s_mut_str);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1106)

#### pub fn [push\_str](#method.push_str)(&mut self, string: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Available on **non-`no_global_oom_handling`** only.

Appends a given string slice onto the end of this `String`.

##### Panics

Panics if the new capacity exceeds `isize::MAX` _bytes_.

##### Examples

```rust
let mut s = String::from("foo");

s.push_str("bar");

assert_eq!("foobar", s);
```

1.87.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1152-1154)

#### pub fn [extend\_from\_within](#method.extend_from_within)<R>(&mut self, src: R)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Available on **non-`no_global_oom_handling`** only.

Copies elements from `src` range to the end of the string.

##### Panics

Panics if the range has `start_bound > end_bound`, if the range is bounded on either end and does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary, or if the new capacity exceeds `isize::MAX` bytes.

##### Examples

```rust
let mut string = String::from("abcde");

string.extend_from_within(2..);
assert_eq!(string, "abcdecde");

string.extend_from_within(..2);
assert_eq!(string, "abcdecdeab");

string.extend_from_within(4..8);
assert_eq!(string, "abcdecdeabecde");
```

1.0.0 (const: 1.87.0) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1177)

#### pub const fn [capacity](#method.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns this `String`’s capacity, in bytes.

##### Examples

```rust
let s = String::with_capacity(10);

assert!(s.capacity() >= 10);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1224)

#### pub fn [reserve](#method.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Available on **non-`no_global_oom_handling`** only.

Reserves capacity for at least `additional` bytes more than the current length. The allocator may reserve more space to speculatively avoid frequent allocations. After calling `reserve`, capacity will be greater than or equal to `self.len() + additional`. Does nothing if capacity is already sufficient.

##### Panics

Panics if the new capacity exceeds `isize::MAX` _bytes_.

##### Examples

Basic usage:

```rust
let mut s = String::new();

s.reserve(10);

assert!(s.capacity() >= 10);
```

This might not actually increase the capacity:

```rust
let mut s = String::with_capacity(10);
s.push('a');
s.push('b');

// s now has a length of 2 and a capacity of at least 10
let capacity = s.capacity();
assert_eq!(2, s.len());
assert!(capacity >= 10);

// Since we already have at least an extra 8 capacity, calling this...
s.reserve(8);

// ... doesn't actually increase.
assert_eq!(capacity, s.capacity());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1274)

#### pub fn [reserve\_exact](#method.reserve_exact)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Available on **non-`no_global_oom_handling`** only.

Reserves the minimum capacity for at least `additional` bytes more than the current length. Unlike [`reserve`](struct.String.html#method.reserve "method bevy::prelude::String::reserve"), this will not deliberately over-allocate to speculatively avoid frequent allocations. After calling `reserve_exact`, capacity will be greater than or equal to `self.len() + additional`. Does nothing if the capacity is already sufficient.

##### Panics

Panics if the new capacity exceeds `isize::MAX` _bytes_.

##### Examples

Basic usage:

```rust
let mut s = String::new();

s.reserve_exact(10);

assert!(s.capacity() >= 10);
```

This might not actually increase the capacity:

```rust
let mut s = String::with_capacity(10);
s.push('a');
s.push('b');

// s now has a length of 2 and a capacity of at least 10
let capacity = s.capacity();
assert_eq!(2, s.len());
assert!(capacity >= 10);

// Since we already have at least an extra 8 capacity, calling this...
s.reserve_exact(8);

// ... doesn't actually increase.
assert_eq!(capacity, s.capacity());
```

1.57.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1309)

#### pub fn [try\_reserve](#method.try_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html "struct alloc::collections::TryReserveError")\>

Tries to reserve capacity for at least `additional` bytes more than the current length. The allocator may reserve more space to speculatively avoid frequent allocations. After calling `try_reserve`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`. Does nothing if capacity is already sufficient. This method preserves the contents even if an error occurs.

##### Errors

If the capacity overflows, or the allocator reports a failure, then an error is returned.

##### Examples

```rust
use std::collections::TryReserveError;

fn process_data(data: &str) -> Result<String, TryReserveError> {
    let mut output = String::new();

    // Pre-reserve the memory, exiting if we can't
    output.try_reserve(data.len())?;

    // Now we know this can't OOM in the middle of our complex work
    output.push_str(data);

    Ok(output)
}
```

1.57.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1350)

#### pub fn [try\_reserve\_exact](#method.try_reserve_exact)( &mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html "struct alloc::collections::TryReserveError")\>

Tries to reserve the minimum capacity for at least `additional` bytes more than the current length. Unlike [`try_reserve`](struct.String.html#method.try_reserve "method bevy::prelude::String::try_reserve"), this will not deliberately over-allocate to speculatively avoid frequent allocations. After calling `try_reserve_exact`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`. Does nothing if the capacity is already sufficient.

Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be relied upon to be precisely minimal. Prefer [`try_reserve`](struct.String.html#method.try_reserve "method bevy::prelude::String::try_reserve") if future insertions are expected.

##### Errors

If the capacity overflows, or the allocator reports a failure, then an error is returned.

##### Examples

```rust
use std::collections::TryReserveError;

fn process_data(data: &str) -> Result<String, TryReserveError> {
    let mut output = String::new();

    // Pre-reserve the memory, exiting if we can't
    output.try_reserve_exact(data.len())?;

    // Now we know this can't OOM in the middle of our complex work
    output.push_str(data);

    Ok(output)
}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1370)

#### pub fn [shrink\_to\_fit](#method.shrink_to_fit)(&mut self)

Available on **non-`no_global_oom_handling`** only.

Shrinks the capacity of this `String` to match its length.

##### Examples

```rust
let mut s = String::from("foo");

s.reserve(100);
assert!(s.capacity() >= 100);

s.shrink_to_fit();
assert_eq!(3, s.capacity());
```

1.56.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1397)

#### pub fn [shrink\_to](#method.shrink_to)(&mut self, min\_capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Available on **non-`no_global_oom_handling`** only.

Shrinks the capacity of this `String` with a lower bound.

The capacity will remain at least as large as both the length and the supplied value.

If the current capacity is less than the lower limit, this is a no-op.

##### Examples

```rust
let mut s = String::from("foo");

s.reserve(100);
assert!(s.capacity() >= 100);

s.shrink_to(10);
assert!(s.capacity() >= 10);
s.shrink_to(0);
assert!(s.capacity() >= 3);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1421)

#### pub fn [push](#method.push)(&mut self, ch: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html))

Available on **non-`no_global_oom_handling`** only.

Appends the given [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") to the end of this `String`.

##### Panics

Panics if the new capacity exceeds `isize::MAX` _bytes_.

##### Examples

```rust
let mut s = String::from("abc");

s.push('1');
s.push('2');
s.push('3');

assert_eq!("abc123", s);
```

1.0.0 (const: 1.87.0) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1450)

#### pub const fn [as\_bytes](#method.as_bytes)(&self) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

Returns a byte slice of this `String`’s contents.

The inverse of this method is [`from_utf8`](struct.String.html#method.from_utf8 "associated function bevy::prelude::String::from_utf8").

##### Examples

```rust
let s = String::from("hello");

assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1478)

#### pub fn [truncate](#method.truncate)(&mut self, new\_len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shortens this `String` to the specified length.

If `new_len` is greater than or equal to the string’s current length, this has no effect.

Note that this method has no effect on the allocated capacity of the string

##### Panics

Panics if `new_len` does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::from("hello");

s.truncate(2);

assert_eq!("he", s);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1502)

#### pub fn [pop](#method.pop)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>

Removes the last character from the string buffer and returns it.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if this `String` is empty.

##### Examples

```rust
let mut s = String::from("abč");

assert_eq!(s.pop(), Some('č'));
assert_eq!(s.pop(), Some('b'));
assert_eq!(s.pop(), Some('a'));

assert_eq!(s.pop(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1535)

#### pub fn [remove](#method.remove)(&mut self, idx: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)

Removes a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") from this `String` at byte position `idx` and returns it.

Copies all bytes after the removed char to new positions.

Note that calling this in a loop can result in quadratic behavior.

##### Panics

Panics if `idx` is larger than or equal to the `String`’s length, or if it does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::from("abç");

assert_eq!(s.remove(0), 'a');
assert_eq!(s.remove(1), 'ç');
assert_eq!(s.remove(0), 'b');
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1572)

#### pub fn [remove\_matches](#method.remove_matches)<P>(&mut self, pat: P)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

🔬This is a nightly-only experimental API. (`string_remove_matches`)

Available on **non-`no_global_oom_handling`** only.

Remove all matches of pattern `pat` in the `String`.

##### Examples

```rust
#![feature(string_remove_matches)]
let mut s = String::from("Trees are not green, the sky is not blue.");
s.remove_matches("not ");
assert_eq!("Trees are green, the sky is blue.", s);
```

Matches will be detected and removed iteratively, so in cases where patterns overlap, only the first pattern will be removed:

```rust
#![feature(string_remove_matches)]
let mut s = String::from("banana");
s.remove_matches("ana");
assert_eq!("bna", s);
```

1.26.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1649-1651)

#### pub fn [retain](#method.retain)<F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([char](https://doc.rust-lang.org/nightly/std/primitive.char.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Retains only the characters specified by the predicate.

In other words, remove all characters `c` such that `f(c)` returns `false`. This method operates in place, visiting each character exactly once in the original order, and preserves the order of the retained characters.

##### Examples

```rust
let mut s = String::from("f_o_ob_ar");

s.retain(|c| c != '_');

assert_eq!(s, "foobar");
```

Because the elements are visited exactly once in the original order, external state may be used to decide which elements to keep.

```rust
let mut s = String::from("abcde");
let keep = [false, true, true, false, true];
let mut iter = keep.iter();
s.retain(|_| *iter.next().unwrap());
assert_eq!(s, "bce");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1731)

#### pub fn [insert](#method.insert)(&mut self, idx: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ch: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html))

Available on **non-`no_global_oom_handling`** only.

Inserts a character into this `String` at byte position `idx`.

Reallocates if `self.capacity()` is insufficient, which may involve copying all `self.capacity()` bytes. Makes space for the insertion by copying all bytes of `&self[idx..]` to new positions.

Note that calling this in a loop can result in quadratic behavior.

##### Panics

Panics if `idx` is larger than the `String`’s length, or if it does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::with_capacity(3);

s.insert(0, 'f');
s.insert(1, 'o');
s.insert(2, 'o');

assert_eq!("foo", s);
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1788)

#### pub fn [insert\_str](#method.insert_str)(&mut self, idx: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), string: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Available on **non-`no_global_oom_handling`** only.

Inserts a string slice into this `String` at byte position `idx`.

Reallocates if `self.capacity()` is insufficient, which may involve copying all `self.capacity()` bytes. Makes space for the insertion by copying all bytes of `&self[idx..]` to new positions.

Note that calling this in a loop can result in quadratic behavior.

##### Panics

Panics if `idx` is larger than the `String`’s length, or if it does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::from("bar");

s.insert_str(0, "foo");

assert_eq!("foobar", s);
```

1.0.0 (const: 1.87.0) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1841)

#### pub const unsafe fn [as\_mut\_vec](#method.as_mut_vec)(&mut self) -> &mut [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> [ⓘ](#)

Returns a mutable reference to the contents of this `String`.

##### Safety

This function is unsafe because the returned `&mut Vec` allows writing bytes which are not valid UTF-8. If this constraint is violated, using the original `String` after dropping the `&mut Vec` may violate memory safety, as the rest of the standard library assumes that `String`s are valid UTF-8.

##### Examples

```rust
let mut s = String::from("hello");

unsafe {
    let vec = s.as_mut_vec();
    assert_eq!(&[104, 101, 108, 108, 111][..], &vec[..]);

    vec.reverse();
}
assert_eq!(s, "olleh");
```

1.0.0 (const: 1.87.0) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1865)

#### pub const fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the length of this `String`, in bytes, not [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s or graphemes. In other words, it might not be what a human considers the length of the string.

##### Examples

```rust
let a = String::from("foo");
assert_eq!(a.len(), 3);

let fancy_f = String::from("ƒoo");
assert_eq!(fancy_f.len(), 4);
assert_eq!(fancy_f.chars().count(), 3);
```

1.0.0 (const: 1.87.0) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1885)

#### pub const fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this `String` has a length of zero, and `false` otherwise.

##### Examples

```rust
let mut v = String::new();
assert!(v.is_empty());

v.push('a');
assert!(!v.is_empty());
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1917)

#### pub fn [split\_off](#method.split_off)(&mut self, at: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Splits the string into two at the given byte index.

Returns a newly allocated `String`. `self` contains bytes `[0, at)`, and the returned `String` contains bytes `[at, len)`. `at` must be on the boundary of a UTF-8 code point.

Note that the capacity of `self` does not change.

##### Panics

Panics if `at` is not on a `UTF-8` code point boundary, or if it is beyond the last code point of the string.

##### Examples

```rust
let mut hello = String::from("Hello, World!");
let world = hello.split_off(7);
assert_eq!(hello, "Hello, ");
assert_eq!(world, "World!");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1941)

#### pub fn [clear](#method.clear)(&mut self)

Truncates this `String`, removing all contents.

While this means the `String` will have a length of zero, it does not touch its capacity.

##### Examples

```rust
let mut s = String::from("foo");

s.clear();

assert!(s.is_empty());
assert_eq!(0, s.len());
assert_eq!(3, s.capacity());
```

1.6.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1980-1982)

#### pub fn [drain](#method.drain)<R>(&mut self, range: R) -> [Drain](https://doc.rust-lang.org/nightly/alloc/string/struct.Drain.html "struct alloc::string::Drain")<'\_> [ⓘ](#)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Removes the specified range from the string in bulk, returning all removed characters as an iterator.

The returned iterator keeps a mutable borrow on the string to optimize its implementation.

##### Panics

Panics if the range has `start_bound > end_bound`, or, if the range is bounded on either end and does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Leaking

If the returned iterator goes out of scope without being dropped (due to [`core::mem::forget`](https://doc.rust-lang.org/nightly/core/mem/fn.forget.html "fn core::mem::forget"), for example), the string may still contain a copy of any drained characters, or may have lost characters arbitrarily, including characters outside the range.

##### Examples

```rust
let mut s = String::from("α is alpha, β is beta");
let beta_offset = s.find('β').unwrap_or(s.len());

// Remove the range up until the β from the string
let t: String = s.drain(..beta_offset).collect();
assert_eq!(t, "α is alpha, ");
assert_eq!(s, "β is beta");

// A full range clears the string, like `clear()` does
s.drain(..);
assert_eq!(s, "");
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2054)

#### pub fn [into\_chars](#method.into_chars)(self) -> [IntoChars](https://doc.rust-lang.org/nightly/alloc/string/struct.IntoChars.html "struct alloc::string::IntoChars") [ⓘ](#)

🔬This is a nightly-only experimental API. (`string_into_chars`)

Converts a `String` into an iterator over the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s of the string.

As a string consists of valid UTF-8, we can iterate through a string by [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"). This method returns such an iterator.

It’s important to remember that [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") represents a Unicode Scalar Value, and might not match your idea of what a ‘character’ is. Iteration over grapheme clusters may be what you actually want. That functionality is not provided by Rust’s standard library, check crates.io instead.

##### Examples

Basic usage:

```rust
#![feature(string_into_chars)]

let word = String::from("goodbye");

let mut chars = word.into_chars();

assert_eq!(Some('g'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('d'), chars.next());
assert_eq!(Some('b'), chars.next());
assert_eq!(Some('y'), chars.next());
assert_eq!(Some('e'), chars.next());

assert_eq!(None, chars.next());
```

Remember, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s might not match your intuition about characters:

```rust
#![feature(string_into_chars)]

let y = String::from("y̆");

let mut chars = y.into_chars();

assert_eq!(Some('y'), chars.next()); // not 'y̆'
assert_eq!(Some('\u{0306}'), chars.next());

assert_eq!(None, chars.next());
```

1.27.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2080-2082)

#### pub fn [replace\_range](#method.replace_range)<R>(&mut self, range: R, replace\_with: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Available on **non-`no_global_oom_handling`** only.

Removes the specified range in the string, and replaces it with the given string. The given string doesn’t need to be the same length as the range.

##### Panics

Panics if the range has `start_bound > end_bound`, or, if the range is bounded on either end and does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::from("α is alpha, β is beta");
let beta_offset = s.find('β').unwrap_or(s.len());

// Replace the range up until the β from the string
s.replace_range(..beta_offset, "Α is capital alpha; ");
assert_eq!(s, "Α is capital alpha; β is beta");
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2122)

#### pub fn [replace\_first](#method.replace_first)<P>(&mut self, from: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

🔬This is a nightly-only experimental API. (`string_replace_in_place`)

Available on **non-`no_global_oom_handling`** only.

Replaces the leftmost occurrence of a pattern with another string, in-place.

This method can be preferred over [`string = string.replacen(..., 1);`](../../std/primitive.str.html#method.replacen), as it can use the `String`’s existing capacity to prevent a reallocation if sufficient space is available.

##### Examples

Basic usage:

```rust
#![feature(string_replace_in_place)]

let mut s = String::from("Test Results: ❌❌❌");

// Replace the leftmost ❌ with a ✅
s.replace_first('❌', "✅");
assert_eq!(s, "Test Results: ✅❌❌");
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2148-2150)

#### pub fn [replace\_last](#method.replace_last)<P>(&mut self, from: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

🔬This is a nightly-only experimental API. (`string_replace_in_place`)

Available on **non-`no_global_oom_handling`** only.

Replaces the rightmost occurrence of a pattern with another string, in-place.

##### Examples

Basic usage:

```rust
#![feature(string_replace_in_place)]

let mut s = String::from("Test Results: ❌❌❌");

// Replace the rightmost ❌ with a ✅
s.replace_last('❌', "✅");
assert_eq!(s, "Test Results: ❌❌✅");
```

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2179)

#### pub fn [into\_boxed\_str](#method.into_boxed_str)(self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **non-`no_global_oom_handling`** only.

Converts this `String` into a `[Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html "str")>`.

Before doing the conversion, this method discards excess capacity like [`shrink_to_fit`](struct.String.html#method.shrink_to_fit "method bevy::prelude::String::shrink_to_fit"). Note that this call may reallocate and copy the bytes of the string.

##### Examples

```rust
let s = String::from("hello");

let b = s.into_boxed_str();
```

1.72.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2210)

#### pub fn [leak](#method.leak)<'a>(self) -> &'a mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Consumes and leaks the `String`, returning a mutable reference to the contents, `&'a mut str`.

The caller has free choice over the returned lifetime, including `'static`. Indeed, this function is ideally used for data that lives for the remainder of the program’s life, as dropping the returned reference will cause a memory leak.

It does not reallocate or shrink the `String`, so the leaked allocation may include unused capacity that is not part of the returned slice. If you want to discard excess capacity, call [`into_boxed_str`](struct.String.html#method.into_boxed_str "method bevy::prelude::String::into_boxed_str"), and then [`Box::leak`](struct.Box.html#method.leak "associated function bevy::prelude::Box::leak") instead. However, keep in mind that trimming the capacity may result in a reallocation and copy.

##### Examples

```rust
let x = String::from("bucket");
let static_ref: &'static mut str = x.leak();
assert_eq!(static_ref, "bucket");
```

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#154)

#### pub fn [len](#method.len-1)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the length of `self`.

This length is in bytes, not [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s or graphemes. In other words, it might not be what a human considers the length of the string.

##### Examples

```rust
let len = "foo".len();
assert_eq!(3, len);

assert_eq!("ƒoo".len(), 4); // fancy f!
assert_eq!("ƒoo".chars().count(), 3);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#174)

#### pub fn [is\_empty](#method.is_empty-1)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if `self` has a length of zero bytes.

##### Examples

```rust
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

1.9.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#374)

#### pub fn [is\_char\_boundary](#method.is_char_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks that `index`\-th byte is the first byte in a UTF-8 code point sequence or the end of the string.

The start and end of the string (when `index == self.len()`) are considered to be boundaries.

Returns `false` if `index` is greater than `self.len()`.

##### Examples

```rust
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0));
// start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));

// second byte of `ö`
assert!(!s.is_char_boundary(2));

// third byte of `老`
assert!(!s.is_char_boundary(8));
```

1.91.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#423)

#### pub fn [floor\_char\_boundary](#method.floor_char_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Finds the closest `x` not exceeding `index` where [`is_char_boundary(x)`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary "method str::is_char_boundary") is `true`.

This method can help you truncate a string so that it’s still valid UTF-8, but doesn’t exceed a given number of bytes. Note that this is done purely at the character level and can still visually split graphemes, even though the underlying characters aren’t split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only includes 🧑 (person) instead.

##### Examples

```rust
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.floor_char_boundary(13);
assert_eq!(closest, 10);
assert_eq!(&s[..closest], "❤️🧡");
```

1.91.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#480)

#### pub fn [ceil\_char\_boundary](#method.ceil_char_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Finds the closest `x` not below `index` where [`is_char_boundary(x)`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary "method str::is_char_boundary") is `true`.

If `index` is greater than the length of the string, this returns the length of the string.

This method is the natural complement to [`floor_char_boundary`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.floor_char_boundary "method str::floor_char_boundary"). See that method for more details.

##### Examples

```rust
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.ceil_char_boundary(13);
assert_eq!(closest, 14);
assert_eq!(&s[..closest], "❤️🧡💛");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#513)

#### pub fn [as\_bytes](#method.as_bytes-1)(&self) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

Converts a string slice to a byte slice. To convert the byte slice back into a string slice, use the [`from_utf8`](https://doc.rust-lang.org/nightly/core/str/converts/fn.from_utf8.html "fn core::str::converts::from_utf8") function.

##### Examples

```rust
let bytes = "bors".as_bytes();
assert_eq!(b"bors", bytes);
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#558)

#### pub unsafe fn [as\_bytes\_mut](#method.as_bytes_mut)(&mut self) -> &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

Converts a mutable string slice to a mutable byte slice.

##### Safety

The caller must ensure that the content of the slice is valid UTF-8 before the borrow ends and the underlying `str` is used.

Use of a `str` whose contents are not valid UTF-8 is undefined behavior.

##### Examples

Basic usage:

```rust
let mut s = String::from("Hello");
let bytes = unsafe { s.as_bytes_mut() };

assert_eq!(b"Hello", bytes);
```

Mutability:

```rust
let mut s = String::from("🗻∈🌏");

unsafe {
    let bytes = s.as_bytes_mut();

    bytes[0] = 0xF0;
    bytes[1] = 0x9F;
    bytes[2] = 0x8D;
    bytes[3] = 0x94;
}

assert_eq!("🍔∈🌏", s);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#589)

#### pub fn [as\_ptr](#method.as_ptr)(&self) -> [\*const](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

Converts a string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8"). This pointer will be pointing to the first byte of the string slice.

The caller must ensure that the returned pointer is never written to. If you need to mutate the contents of the string slice, use [`as_mut_ptr`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.as_mut_ptr "method str::as_mut_ptr").

##### Examples

```rust
let s = "Hello";
let ptr = s.as_ptr();
```

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#608)

#### pub fn [as\_mut\_ptr](#method.as_mut_ptr)(&mut self) -> [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

Converts a mutable string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8"). This pointer will be pointing to the first byte of the string slice.

It is your responsibility to make sure that the string slice only gets modified in a way that it remains valid UTF-8.

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#634)

#### pub fn [get](#method.get)<I>(&self, i: I) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")\>

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns a subslice of `str`.

This is the non-panicking alternative to indexing the `str`. Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") whenever equivalent indexing operation would panic.

##### Examples

```rust
let v = String::from("🗻∈🌏");

assert_eq!(Some("🗻"), v.get(0..4));

// indices not on UTF-8 sequence boundaries
assert!(v.get(1..).is_none());
assert!(v.get(..8).is_none());

// out of bounds
assert!(v.get(..42).is_none());
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#667)

#### pub fn [get\_mut](#method.get_mut)<I>( &mut self, i: I, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")\>

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns a mutable subslice of `str`.

This is the non-panicking alternative to indexing the `str`. Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") whenever equivalent indexing operation would panic.

##### Examples

```rust
let mut v = String::from("hello");
// correct length
assert!(v.get_mut(0..5).is_some());
// out of bounds
assert!(v.get_mut(..42).is_none());
assert_eq!(Some("he"), v.get_mut(0..2).map(|v| &*v));

assert_eq!("hello", v);
{
    let s = v.get_mut(0..2);
    let s = s.map(|s| {
        s.make_ascii_uppercase();
        &*s
    });
    assert_eq!(Some("HE"), s);
}
assert_eq!("HEllo", v);
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#699)

#### pub unsafe fn [get\_unchecked](#method.get_unchecked)<I>(&self, i: I) -> &<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns an unchecked subslice of `str`.

This is the unchecked alternative to indexing the `str`.

##### Safety

Callers of this function are responsible that these preconditions are satisfied:

*   The starting index must not exceed the ending index;
*   Indexes must be within bounds of the original slice;
*   Indexes must lie on UTF-8 sequence boundaries.

Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the `str` type.

##### Examples

```rust
let v = "🗻∈🌏";
unsafe {
    assert_eq!("🗻", v.get_unchecked(0..4));
    assert_eq!("∈", v.get_unchecked(4..7));
    assert_eq!("🌏", v.get_unchecked(7..11));
}
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#734)

#### pub unsafe fn [get\_unchecked\_mut](#method.get_unchecked_mut)<I>( &mut self, i: I, ) -> &mut <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns a mutable, unchecked subslice of `str`.

This is the unchecked alternative to indexing the `str`.

##### Safety

Callers of this function are responsible that these preconditions are satisfied:

*   The starting index must not exceed the ending index;
*   Indexes must be within bounds of the original slice;
*   Indexes must lie on UTF-8 sequence boundaries.

Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the `str` type.

##### Examples

```rust
let mut v = String::from("🗻∈🌏");
unsafe {
    assert_eq!("🗻", v.get_unchecked_mut(0..4));
    assert_eq!("∈", v.get_unchecked_mut(4..7));
    assert_eq!("🌏", v.get_unchecked_mut(7..11));
}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#785)

#### pub unsafe fn [slice\_unchecked](#method.slice_unchecked)(&self, begin: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), end: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.29.0:

use `get_unchecked(begin..end)` instead

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "primitive str") and [`Index`](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index").

This new slice goes from `begin` to `end`, including `begin` but excluding `end`.

To get a mutable string slice instead, see the [`slice_mut_unchecked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.slice_mut_unchecked "method str::slice_mut_unchecked") method.

##### Safety

Callers of this function are responsible that three preconditions are satisfied:

*   `begin` must not exceed `end`.
*   `begin` and `end` must be byte positions within the string slice.
*   `begin` and `end` must lie on UTF-8 sequence boundaries.

##### Examples

```rust
let s = "Löwe 老虎 Léopard";

unsafe {
    assert_eq!("Löwe 老虎 Léopard", s.slice_unchecked(0, 21));
}

let s = "Hello, world!";

unsafe {
    assert_eq!("world", s.slice_unchecked(7, 12));
}
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#819)

#### pub unsafe fn [slice\_mut\_unchecked](#method.slice_mut_unchecked)( &mut self, begin: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), end: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.29.0:

use `get_unchecked_mut(begin..end)` instead

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "primitive str") and [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut").

This new slice goes from `begin` to `end`, including `begin` but excluding `end`.

To get an immutable string slice instead, see the [`slice_unchecked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.slice_unchecked "method str::slice_unchecked") method.

##### Safety

Callers of this function are responsible that three preconditions are satisfied:

*   `begin` must not exceed `end`.
*   `begin` and `end` must be byte positions within the string slice.
*   `begin` and `end` must lie on UTF-8 sequence boundaries.

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#859)

#### pub fn [split\_at](#method.split_at)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Divides one string slice into two at an index.

The argument, `mid`, should be a byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut "method str::split_at_mut") method.

##### Panics

Panics if `mid` is not on a UTF-8 code point boundary, or if it is past the end of the last code point of the string slice. For a non-panicking alternative see [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_checked "method str::split_at_checked").

##### Examples

```rust
let s = "Per Martin-Löf";

let (first, last) = s.split_at(3);

assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);
```

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#900)

#### pub fn [split\_at\_mut](#method.split_at_mut)(&mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Divides one mutable string slice into two at an index.

The argument, `mid`, should be a byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get immutable string slices instead, see the [`split_at`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at "method str::split_at") method.

##### Panics

Panics if `mid` is not on a UTF-8 code point boundary, or if it is past the end of the last code point of the string slice. For a non-panicking alternative see [`split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut_checked "method str::split_at_mut_checked").

##### Examples

```rust
let mut s = "Per Martin-Löf".to_string();
{
    let (first, last) = s.split_at_mut(3);
    first.make_ascii_uppercase();
    assert_eq!("PER", first);
    assert_eq!(" Martin-Löf", last);
}
assert_eq!("PER Martin-Löf", s);
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#940)

#### pub fn [split\_at\_checked](#method.split_at_checked)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

Divides one string slice into two at an index.

The argument, `mid`, should be a valid byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point. The method returns `None` if that’s not the case.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut_checked "method str::split_at_mut_checked") method.

##### Examples

```rust
let s = "Per Martin-Löf";

let (first, last) = s.split_at_checked(3).unwrap();
assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);

assert_eq!(None, s.split_at_checked(13));  // Inside “ö”
assert_eq!(None, s.split_at_checked(16));  // Beyond the string length
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#981)

#### pub fn [split\_at\_mut\_checked](#method.split_at_mut_checked)( &mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

Divides one mutable string slice into two at an index.

The argument, `mid`, should be a valid byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point. The method returns `None` if that’s not the case.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get immutable string slices instead, see the [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_checked "method str::split_at_checked") method.

##### Examples

```rust
let mut s = "Per Martin-Löf".to_string();
if let Some((first, last)) = s.split_at_mut_checked(3) {
    first.make_ascii_uppercase();
    assert_eq!("PER", first);
    assert_eq!(" Martin-Löf", last);
}
assert_eq!("PER Martin-Löf", s);

assert_eq!(None, s.split_at_mut_checked(13));  // Inside “ö”
assert_eq!(None, s.split_at_mut_checked(16));  // Beyond the string length
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1078)

#### pub fn [chars](#method.chars)(&self) -> [Chars](https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html "struct core::str::iter::Chars")<'\_> [ⓘ](#)

Returns an iterator over the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s of a string slice.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"). This method returns such an iterator.

It’s important to remember that [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") represents a Unicode Scalar Value, and might not match your idea of what a ‘character’ is. Iteration over grapheme clusters may be what you actually want. This functionality is not provided by Rust’s standard library, check crates.io instead.

##### Examples

Basic usage:

```rust
let word = "goodbye";

let count = word.chars().count();
assert_eq!(7, count);

let mut chars = word.chars();

assert_eq!(Some('g'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('d'), chars.next());
assert_eq!(Some('b'), chars.next());
assert_eq!(Some('y'), chars.next());
assert_eq!(Some('e'), chars.next());

assert_eq!(None, chars.next());
```

Remember, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s might not match your intuition about characters:

```rust
let y = "y̆";

let mut chars = y.chars();

assert_eq!(Some('y'), chars.next()); // not 'y̆'
assert_eq!(Some('\u{0306}'), chars.next());

assert_eq!(None, chars.next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1135)

#### pub fn [char\_indices](#method.char_indices)(&self) -> [CharIndices](https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html "struct core::str::iter::CharIndices")<'\_> [ⓘ](#)

Returns an iterator over the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s of a string slice, and their positions.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"). This method returns an iterator of both these [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, as well as their byte positions.

The iterator yields tuples. The position is first, the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") is second.

##### Examples

Basic usage:

```rust
let word = "goodbye";

let count = word.char_indices().count();
assert_eq!(7, count);

let mut char_indices = word.char_indices();

assert_eq!(Some((0, 'g')), char_indices.next());
assert_eq!(Some((1, 'o')), char_indices.next());
assert_eq!(Some((2, 'o')), char_indices.next());
assert_eq!(Some((3, 'd')), char_indices.next());
assert_eq!(Some((4, 'b')), char_indices.next());
assert_eq!(Some((5, 'y')), char_indices.next());
assert_eq!(Some((6, 'e')), char_indices.next());

assert_eq!(None, char_indices.next());
```

Remember, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s might not match your intuition about characters:

```rust
let yes = "y̆es";

let mut char_indices = yes.char_indices();

assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
assert_eq!(Some((1, '\u{0306}')), char_indices.next());

// note the 3 here - the previous character took up two bytes
assert_eq!(Some((3, 'e')), char_indices.next());
assert_eq!(Some((4, 's')), char_indices.next());

assert_eq!(None, char_indices.next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1158)

#### pub fn [bytes](#method.bytes)(&self) -> [Bytes](https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html "struct core::str::iter::Bytes")<'\_> [ⓘ](#)

Returns an iterator over the bytes of a string slice.

As a string slice consists of a sequence of bytes, we can iterate through a string slice by byte. This method returns such an iterator.

##### Examples

```rust
let mut bytes = "bors".bytes();

assert_eq!(Some(b'b'), bytes.next());
assert_eq!(Some(b'o'), bytes.next());
assert_eq!(Some(b'r'), bytes.next());
assert_eq!(Some(b's'), bytes.next());

assert_eq!(None, bytes.next());
```

1.1.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1210)

#### pub fn [split\_whitespace](#method.split_whitespace)(&self) -> [SplitWhitespace](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html "struct core::str::iter::SplitWhitespace")<'\_> [ⓘ](#)

Splits a string slice by whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of whitespace.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`. If you only want to split on ASCII whitespace instead, use [`split_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_ascii_whitespace "method str::split_ascii_whitespace").

##### Examples

Basic usage:

```rust
let mut iter = "A few words".split_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

All kinds of whitespace are considered:

```rust
let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all whitespace, the iterator yields no string slices:

```rust
assert_eq!("".split_whitespace().next(), None);
assert_eq!("   ".split_whitespace().next(), None);
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1264)

#### pub fn [split\_ascii\_whitespace](#method.split_ascii_whitespace)(&self) -> [SplitAsciiWhitespace](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html "struct core::str::iter::SplitAsciiWhitespace")<'\_> [ⓘ](#)

Splits a string slice by ASCII whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of ASCII whitespace.

This uses the same definition as [`char::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.is_ascii_whitespace "method char::is_ascii_whitespace"). To split by Unicode `Whitespace` instead, use [`split_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_whitespace "method str::split_whitespace"). Note that because of this difference in definition, even if `s.is_ascii()` is `true`, `s.split_ascii_whitespace()` behavior will differ from `s.split_whitespace()` if `s` contains U+000B VERTICAL TAB.

##### Examples

Basic usage:

```rust
let mut iter = "A few words".split_ascii_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

Various kinds of ASCII whitespace are considered (see [`char::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.is_ascii_whitespace "method char::is_ascii_whitespace")):

```rust
let mut iter = " Mary   had\ta little  \n\t lamb".split_ascii_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all ASCII whitespace, the iterator yields no string slices:

```rust
assert_eq!("".split_ascii_whitespace().next(), None);
assert_eq!("   ".split_ascii_whitespace().next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1328)

#### pub fn [lines](#method.lines)(&self) -> [Lines](https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html "struct core::str::iter::Lines")<'\_> [ⓘ](#)

Returns an iterator over the lines of a string, as string slices.

Lines are split at line endings that are either newlines (`\n`) or sequences of a carriage return followed by a line feed (`\r\n`).

Line terminators are not included in the lines returned by the iterator.

Note that any carriage return (`\r`) not immediately followed by a line feed (`\n`) does not split a line. These carriage returns are thereby included in the produced lines.

The final line ending is optional. A string that ends with a final line ending will return the same lines as an otherwise identical string without a final line ending.

An empty string returns an empty iterator.

##### Examples

Basic usage:

```rust
let text = "foo\r\nbar\n\nbaz\r";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
// Trailing carriage return is included in the last line
assert_eq!(Some("baz\r"), lines.next());

assert_eq!(None, lines.next());
```

The final line does not require any ending:

```rust
let text = "foo\nbar\n\r\nbaz";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
assert_eq!(Some("baz"), lines.next());

assert_eq!(None, lines.next());
```

An empty string returns an empty iterator:

```rust
let text = "";
let mut lines = text.lines();

assert_eq!(lines.next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1337)

#### pub fn [lines\_any](#method.lines_any)(&self) -> [LinesAny](https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html "struct core::str::iter::LinesAny")<'\_> [ⓘ](#)

👎Deprecated since 1.4.0:

use lines() instead now

Returns an iterator over the lines of a string.

1.8.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1357)

#### pub fn [encode\_utf16](#method.encode_utf16)(&self) -> [EncodeUtf16](https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html "struct core::str::iter::EncodeUtf16")<'\_> [ⓘ](#)

Returns an iterator of `u16` over the string encoded as native endian UTF-16 (without byte-order mark).

##### Examples

```rust
let text = "Zażółć gęślą jaźń";

let utf8_len = text.len();
let utf16_len = text.encode_utf16().count();

assert!(utf16_len <= utf8_len);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1382)

#### pub fn [contains](#method.contains)<P>(&self, pat: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns `true` if the given pattern matches a sub-slice of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
let bananas = "bananas";

assert!(bananas.contains("nana"));
assert!(!bananas.contains("apples"));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1420)

#### pub fn [starts\_with](#method.starts_with)<P>(&self, pat: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns `true` if the given pattern matches a prefix of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, in which case this function will return true if the `&str` is a prefix of this string slice.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can also be a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches. These will only be checked against the first character of this string slice. Look at the second example below regarding behavior for slices of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s.

##### Examples

```rust
let bananas = "bananas";

assert!(bananas.starts_with("bana"));
assert!(!bananas.starts_with("nana"));
```

```rust
let bananas = "bananas";

// Note that both of these assert successfully.
assert!(bananas.starts_with(&['b', 'a', 'n', 'a']));
assert!(bananas.starts_with(&['a', 'b', 'c', 'd']));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1445-1447)

#### pub fn [ends\_with](#method.ends_with)<P>(&self, pat: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns `true` if the given pattern matches a suffix of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
let bananas = "bananas";

assert!(bananas.ends_with("anas"));
assert!(!bananas.ends_with("nana"));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1496)

#### pub fn [find](#method.find)<P>(&self, pat: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns the byte index of the first character of this string slice that matches the pattern.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the pattern doesn’t match.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

Simple patterns:

```rust
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.find('L'), Some(0));
assert_eq!(s.find('é'), Some(14));
assert_eq!(s.find("pard"), Some(17));
```

More complex patterns using point-free style and closures:

```rust
let s = "Löwe 老虎 Léopard";

assert_eq!(s.find(char::is_whitespace), Some(5));
assert_eq!(s.find(char::is_lowercase), Some(1));
assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
```

Not finding the pattern:

```rust
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.find(x), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1542-1544)

#### pub fn [rfind](#method.rfind)<P>(&self, pat: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns the byte index for the first character of the last match of the pattern in this string slice.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the pattern doesn’t match.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

Simple patterns:

```rust
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.rfind('L'), Some(13));
assert_eq!(s.rfind('é'), Some(14));
assert_eq!(s.rfind("pard"), Some(24));
```

More complex patterns with closures:

```rust
let s = "Löwe 老虎 Léopard";

assert_eq!(s.rfind(char::is_whitespace), Some(12));
assert_eq!(s.rfind(char::is_lowercase), Some(20));
```

Not finding the pattern:

```rust
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.rfind(x), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1670)

#### pub fn [split](#method.split)<P>(&self, pat: P) -> [Split](https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html "struct core::str::iter::Split")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

If there are no matches the full string slice is returned as the only item in the iterator.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit "method str::rsplit") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

let v: Vec<&str> = "".split('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
assert_eq!(v, ["lion", "", "tiger", "leopard"]);

let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);

let v: Vec<&str> = "AABBCC".split("DD").collect();
assert_eq!(v, ["AABBCC"]);

let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
assert_eq!(v, ["abc", "def", "ghi"]);

let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);
```

If the pattern is a slice of chars, split on each occurrence of any of the characters:

```rust
let v: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
assert_eq!(v, ["2020", "11", "03", "23", "59"]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "def", "ghi"]);
```

If a string contains multiple contiguous separators, you will end up with empty strings in the output:

```rust
let x = "||||a||b|c".to_string();
let d: Vec<_> = x.split('|').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

Contiguous separators are separated by the empty string.

```rust
let x = "(///)".to_string();
let d: Vec<_> = x.split('/').collect();

assert_eq!(d, &["(", "", "", ")"]);
```

Separators at the start or end of a string are neighbored by empty strings.

```rust
let d: Vec<_> = "010".split("0").collect();
assert_eq!(d, &["", "1", ""]);
```

When the empty string is used as a separator, it separates every character in the string, along with the beginning and end of the string.

```rust
let f: Vec<_> = "rust".split("").collect();
assert_eq!(f, &["", "r", "u", "s", "t", ""]);
```

Contiguous separators can lead to possibly surprising behavior when whitespace is used as the separator. This code is correct:

```rust
let x = "    a  b c".to_string();
let d: Vec<_> = x.split(' ').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

It does _not_ give you:

[ⓘ](# "This example is not tested")

```rust
assert_eq!(d, &["a", "b", "c"]);
```

Use [`split_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_whitespace "method str::split_whitespace") for this behavior.

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1711)

#### pub fn [split\_inclusive](#method.split_inclusive)<P>(&self, pat: P) -> [SplitInclusive](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html "struct core::str::iter::SplitInclusive")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

Differs from the iterator produced by `split` in that `split_inclusive` leaves the matched part as the terminator of the substring.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
```

If the last element of the string is matched, that element will be considered the terminator of the preceding substring. That substring will be the last item returned by the iterator.

```rust
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb.\n"
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb.\n"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1766-1768)

#### pub fn [rsplit](#method.rsplit)<P>(&self, pat: P) -> [RSplit](https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html "struct core::str::iter::RSplit")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern and yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lamb".rsplit(' ').collect();
assert_eq!(v, ["lamb", "little", "a", "had", "Mary"]);

let v: Vec<&str> = "".rsplit('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplit('X').collect();
assert_eq!(v, ["leopard", "tiger", "", "lion"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplit("::").collect();
assert_eq!(v, ["leopard", "tiger", "lion"]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".rsplit(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "def", "abc"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1815)

#### pub fn [split\_terminator](#method.split_terminator)<P>(&self, pat: P) -> [SplitTerminator](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html "struct core::str::iter::SplitTerminator")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

Equivalent to [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split"), except that the trailing substring is skipped if empty.

This method can be used for string data that is _terminated_, rather than _separated_ by a pattern.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit_terminator`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit_terminator "method str::rsplit_terminator") method can be used.

##### Examples

```rust
let v: Vec<&str> = "A.B.".split_terminator('.').collect();
assert_eq!(v, ["A", "B"]);

let v: Vec<&str> = "A..B..".split_terminator(".").collect();
assert_eq!(v, ["A", "", "B", ""]);

let v: Vec<&str> = "A.B:C.D".split_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["A", "B", "C", "D"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1861-1863)

#### pub fn [rsplit\_terminator](#method.rsplit_terminator)<P>(&self, pat: P) -> [RSplitTerminator](https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html "struct core::str::iter::RSplitTerminator")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over substrings of `self`, separated by characters matched by a pattern and yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

Equivalent to [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split"), except that the trailing substring is skipped if empty.

This method can be used for string data that is _terminated_, rather than _separated_ by a pattern.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be double ended if a forward/reverse search yields the same elements.

For iterating from the front, the [`split_terminator`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_terminator "method str::split_terminator") method can be used.

##### Examples

```rust
let v: Vec<&str> = "A.B.".rsplit_terminator('.').collect();
assert_eq!(v, ["B", "A"]);

let v: Vec<&str> = "A..B..".rsplit_terminator(".").collect();
assert_eq!(v, ["", "B", "", "A"]);

let v: Vec<&str> = "A.B:C.D".rsplit_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["D", "C", "B", "A"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1916)

#### pub fn [splitn](#method.splitn)<P>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pat: P) -> [SplitN](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html "struct core::str::iter::SplitN")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of the given string slice, separated by a pattern, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

If the pattern allows a reverse search, the [`rsplitn`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplitn "method str::rsplitn") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
assert_eq!(v, ["Mary", "had", "a little lambda"]);

let v: Vec<&str> = "lionXXtigerXleopard".splitn(3, "X").collect();
assert_eq!(v, ["lion", "", "tigerXleopard"]);

let v: Vec<&str> = "abcXdef".splitn(1, 'X').collect();
assert_eq!(v, ["abcXdef"]);

let v: Vec<&str> = "".splitn(1, 'X').collect();
assert_eq!(v, [""]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".splitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "defXghi"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1965-1967)

#### pub fn [rsplitn](#method.rsplitn)<P>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pat: P) -> [RSplitN](https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html "struct core::str::iter::RSplitN")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over substrings of this string slice, separated by a pattern, starting from the end of the string, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

For splitting from the front, the [`splitn`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.splitn "method str::splitn") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lamb".rsplitn(3, ' ').collect();
assert_eq!(v, ["lamb", "little", "Mary had a"]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplitn(3, 'X').collect();
assert_eq!(v, ["leopard", "tiger", "lionX"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplitn(2, "::").collect();
assert_eq!(v, ["leopard", "lion::tiger"]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".rsplitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "abc1def"]);
```

1.52.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1985)

#### pub fn [split\_once](#method.split_once)<P>(&self, delimiter: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Splits the string on the first occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### Examples

```rust
assert_eq!("cfg".split_once('='), None);
assert_eq!("cfg=".split_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".split_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".split_once('='), Some(("cfg", "foo=bar")));
```

1.52.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2004-2006)

#### pub fn [rsplit\_once](#method.rsplit_once)<P>(&self, delimiter: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Splits the string on the last occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### Examples

```rust
assert_eq!("cfg".rsplit_once('='), None);
assert_eq!("cfg=".rsplit_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".rsplit_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".rsplit_once('='), Some(("cfg=foo", "bar")));
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2044)

#### pub fn [matches](#method.matches)<P>(&self, pat: P) -> [Matches](https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html "struct core::str::iter::Matches")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over the disjoint matches of a pattern within the given string slice.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatches "method str::rmatches") method can be used.

##### Examples

```rust
let v: Vec<&str> = "abcXXXabcYYYabc".matches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
assert_eq!(v, ["1", "2", "3"]);
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2078-2080)

#### pub fn [rmatches](#method.rmatches)<P>(&self, pat: P) -> [RMatches](https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html "struct core::str::iter::RMatches")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over the disjoint matches of a pattern within this string slice, yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.matches "method str::matches") method can be used.

##### Examples

```rust
let v: Vec<&str> = "abcXXXabcYYYabc".rmatches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".rmatches(char::is_numeric).collect();
assert_eq!(v, ["3", "2", "1"]);
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2122)

#### pub fn [match\_indices](#method.match_indices)<P>(&self, pat: P) -> [MatchIndices](https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html "struct core::str::iter::MatchIndices")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over the disjoint matches of a pattern within this string slice as well as the index that the match starts at.

For matches of `pat` within `self` that overlap, only the indices corresponding to the first match are returned.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatch_indices`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatch_indices "method str::rmatch_indices") method can be used.

##### Examples

```rust
let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);

let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
assert_eq!(v, [(1, "abc"), (4, "abc")]);

let v: Vec<_> = "ababa".match_indices("aba").collect();
assert_eq!(v, [(0, "aba")]); // only the first `aba`
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2162-2164)

#### pub fn [rmatch\_indices](#method.rmatch_indices)<P>(&self, pat: P) -> [RMatchIndices](https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html "struct core::str::iter::RMatchIndices")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over the disjoint matches of a pattern within `self`, yielded in reverse order along with the index of the match.

For matches of `pat` within `self` that overlap, only the indices corresponding to the last match are returned.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`match_indices`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.match_indices "method str::match_indices") method can be used.

##### Examples

```rust
let v: Vec<_> = "abcXXXabcYYYabc".rmatch_indices("abc").collect();
assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);

let v: Vec<_> = "1abcabc2".rmatch_indices("abc").collect();
assert_eq!(v, [(4, "abc"), (1, "abc")]);

let v: Vec<_> = "ababa".rmatch_indices("aba").collect();
assert_eq!(v, [(2, "aba")]); // only the last `aba`
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2186)

#### pub fn [trim](#method.trim)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading and trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### Examples

```rust
let s = "\n Hello\tworld\t\n";

assert_eq!("Hello\tworld", s.trim());
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2225)

#### pub fn [trim\_start](#method.trim_start)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### Examples

Basic usage:

```rust
let s = "\n Hello\tworld\t\n";
assert_eq!("Hello\tworld\t\n", s.trim_start());
```

Directionality:

```rust
let s = "  English  ";
assert!(Some('E') == s.trim_start().chars().next());

let s = "  עברית  ";
assert!(Some('ע') == s.trim_start().chars().next());
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2264)

#### pub fn [trim\_end](#method.trim_end)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### Examples

Basic usage:

```rust
let s = "\n Hello\tworld\t\n";
assert_eq!("\n Hello\tworld", s.trim_end());
```

Directionality:

```rust
let s = "  English  ";
assert!(Some('h') == s.trim_end().chars().rev().next());

let s = "  עברית  ";
assert!(Some('ת') == s.trim_end().chars().rev().next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2304)

#### pub fn [trim\_left](#method.trim_left)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.33.0:

superseded by `trim_start`

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _right_ side, not the left.

##### Examples

Basic usage:

```rust
let s = " Hello\tworld\t";

assert_eq!("Hello\tworld\t", s.trim_left());
```

Directionality:

```rust
let s = "  English";
assert!(Some('E') == s.trim_left().chars().next());

let s = "  עברית";
assert!(Some('ע') == s.trim_left().chars().next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2344)

#### pub fn [trim\_right](#method.trim_right)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.33.0:

superseded by `trim_end`

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _left_ side, not the right.

##### Examples

Basic usage:

```rust
let s = " Hello\tworld\t";

assert_eq!(" Hello\tworld", s.trim_right());
```

Directionality:

```rust
let s = "English  ";
assert!(Some('h') == s.trim_right().chars().rev().next());

let s = "עברית  ";
assert!(Some('ת') == s.trim_right().chars().rev().next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2377-2379)

#### pub fn [trim\_matches](#method.trim_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [DoubleEndedSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.DoubleEndedSearcher.html "trait core::str::pattern::DoubleEndedSearcher")<'a>,

Returns a string slice with all prefixes and suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

Simple patterns:

```rust
assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
```

A more complex pattern, using a closure:

```rust
assert_eq!("1foo1barXX".trim_matches(|c| c == '1' || c == 'X'), "foo1bar");
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2424)

#### pub fn [trim\_start\_matches](#method.trim_start_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### Examples

```rust
assert_eq!("11foo1bar11".trim_start_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_start_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
```

1.45.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2458)

#### pub fn [strip\_prefix](#method.strip_prefix)<P>(&self, prefix: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns a string slice with the prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix, wrapped in `Some`. Unlike [`trim_start_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start_matches "method str::trim_start_matches"), this method removes the prefix exactly once.

If the string does not start with `prefix`, returns `None`.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
assert_eq!("foo:bar".strip_prefix("foo:"), Some("bar"));
assert_eq!("foo:bar".strip_prefix("bar"), None);
assert_eq!("foofoo".strip_prefix("foo"), Some("foo"));
```

1.45.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2486-2488)

#### pub fn [strip\_suffix](#method.strip_suffix)<P>(&self, suffix: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns a string slice with the suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix, wrapped in `Some`. Unlike [`trim_end_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end_matches "method str::trim_end_matches"), this method removes the suffix exactly once.

If the string does not end with `suffix`, returns `None`.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
assert_eq!("bar:foo".strip_suffix(":foo"), Some("bar"));
assert_eq!("bar:foo".strip_suffix("bar"), None);
assert_eq!("foofoo".strip_suffix("foo"), Some("foo"));
```

1.98.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2520-2522)

#### pub fn [strip\_circumfix](#method.strip_circumfix)<P, S>(&self, prefix: P, suffix: S) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), S: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <S as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns a string slice with the prefix and suffix removed.

If the string starts with the pattern `prefix` and ends with the pattern `suffix`, returns the substring after the prefix and before the suffix, wrapped in `Some`. Unlike [`trim_start_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start_matches "method str::trim_start_matches") and [`trim_end_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end_matches "method str::trim_end_matches"), this method removes both the prefix and suffix exactly once.

If the string does not start with `prefix` or does not end with `suffix`, returns `None`.

Each [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
assert_eq!("bar:hello:foo".strip_circumfix("bar:", ":foo"), Some("hello"));
assert_eq!("bar:foo".strip_circumfix("foo", "foo"), None);
assert_eq!("foo:bar;".strip_circumfix("foo:", ';'), Some("bar"));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2560)

#### pub fn [trim\_prefix](#method.trim_prefix)<P>(&self, prefix: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix. Unlike [`strip_prefix`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_prefix "method str::strip_prefix"), this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

If the string does not start with `prefix`, returns the original string unchanged.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
#![feature(trim_prefix_suffix)]

// Prefix present - removes it
assert_eq!("foo:bar".trim_prefix("foo:"), "bar");
assert_eq!("foofoo".trim_prefix("foo"), "foo");

// Prefix absent - returns original string
assert_eq!("foo:bar".trim_prefix("bar"), "foo:bar");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2597-2599)

#### pub fn [trim\_suffix](#method.trim_suffix)<P>(&self, suffix: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix. Unlike [`strip_suffix`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_suffix "method str::strip_suffix"), this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

If the string does not end with `suffix`, returns the original string unchanged.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
#![feature(trim_prefix_suffix)]

// Suffix present - removes it
assert_eq!("bar:foo".trim_suffix(":foo"), "bar");
assert_eq!("foofoo".trim_suffix("foo"), "foo");

// Suffix absent - returns original string
assert_eq!("bar:foo".trim_suffix("bar"), "bar:foo");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2640-2642)

#### pub fn [trim\_end\_matches](#method.trim_end_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### Examples

Simple patterns:

```rust
assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_end_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

```rust
assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2684)

#### pub fn [trim\_left\_matches](#method.trim_left_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

👎Deprecated since 1.33.0:

superseded by `trim_start_matches`

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _right_ side, not the left.

##### Examples

```rust
assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_left_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2727-2729)

#### pub fn [trim\_right\_matches](#method.trim_right_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

👎Deprecated since 1.33.0:

superseded by `trim_end_matches`

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _left_ side, not the right.

##### Examples

Simple patterns:

```rust
assert_eq!("11foo1bar11".trim_right_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_right_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_right_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

```rust
assert_eq!("1fooX".trim_right_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2778)

#### pub fn [parse](#method.parse)<F>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<F, <F as [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr")\>::[Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "type core::str::traits::FromStr::Err")\>

where F: [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr"),

Parses this string slice into another type.

Because `parse` is so general, it can cause problems with type inference. As such, `parse` is one of the few times you’ll see the syntax affectionately known as the ‘turbofish’: `::<>`. This helps the inference algorithm understand specifically which type you’re trying to parse into.

`parse` can parse into any type that implements the [`FromStr`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") trait.

##### Errors

Will return [`Err`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "associated type core::str::traits::FromStr::Err") if it’s not possible to parse this string slice into the desired type.

##### Examples

Basic usage:

```rust
let four: u32 = "4".parse().unwrap();

assert_eq!(4, four);
```

Using the ‘turbofish’ instead of annotating `four`:

```rust
let four = "4".parse::<u32>();

assert_eq!(Ok(4), four);
```

Failing to parse:

```rust
let nope = "j".parse::<u32>();

assert!(nope.is_err());
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2799)

#### pub fn [is\_ascii](#method.is_ascii)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if all characters in this string are within the ASCII range.

An empty string returns `true`.

##### Examples

```rust
let ascii = "hello!\n";
let non_ascii = "Grüße, Jürgen ❤";

assert!(ascii.is_ascii());
assert!(!non_ascii.is_ascii());
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2811)

#### pub fn [as\_ascii](#method.as_ascii)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&\[[AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\]>

🔬This is a nightly-only experimental API. (`ascii_char`)

If this string slice [`is_ascii`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_ascii "method str::is_ascii"), returns it as a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), otherwise returns `None`.

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2825)

#### pub unsafe fn [as\_ascii\_unchecked](#method.as_ascii_unchecked)(&self) -> &\[[AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\]

🔬This is a nightly-only experimental API. (`ascii_char`)

Converts this string slice into a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), without checking whether they are valid.

##### Safety

Every character in this string must be ASCII, or else this is UB.

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2856)

#### pub fn [eq\_ignore\_ascii\_case](#method.eq_ignore_ascii_case)(&self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks that two strings are an ASCII case-insensitive match.

Same as `to_ascii_lowercase(a) == to_ascii_lowercase(b)`, but without allocating and copying temporaries.

For Unicode-aware case-insensitive matching, consider [`str::eq_ignore_case_unnormalized`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.eq_ignore_case_unnormalized "method str::eq_ignore_case_unnormalized").

##### Examples

```rust
assert!("Ferris".eq_ignore_ascii_case("FERRIS"));
assert!("Ferrös".eq_ignore_ascii_case("FERRöS"));
assert!(!"Ferrös".eq_ignore_ascii_case("FERRÖS"));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2907)

#### pub fn [eq\_ignore\_case\_unnormalized](#method.eq_ignore_case_unnormalized)(&self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

🔬This is a nightly-only experimental API. (`casefold`)

Checks that two strings are a caseless match, according to [Definition 144](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G53513) in Chapter 3 of the Unicode Standard.

Same as `a.to_casefold_unnormalized() == b.to_casefold_unnormalized()`, but without allocating. See that method’s documentation, as well as [`char::to_casefold_unnormalized()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_casefold_unnormalized "method char::to_casefold_unnormalized"), for more information about case folding.

No [normalization](https://www.unicode.org/faq/normalization.html) (e.g. NFC) is performed, so visually and semantically identical strings might still compare unequal. For example, `"Å"` (U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE) is considered distinct from `"Å"` (A followed by U+030A COMBINING RING ABOVE), even though Unicode considers them canonically equivalent.

In addition, this method is independent of language/locale, so the special behavior of I/ı/İ/i in Turkish and Azeri is not handled.

##### Examples

```rust
#![feature(casefold)]
assert!("Ferris".eq_ignore_case_unnormalized("FERRIS"));
assert!("Ferrös".eq_ignore_case_unnormalized("FERRÖS"));
assert!("ẞ".eq_ignore_case_unnormalized("ss"));
```

No NFC [normalization](https://www.unicode.org/faq/normalization.html) is performed:

```rust
#![feature(casefold)]
// These two strings are visually and semantically identical...
let comp = "Å";
let decomp = "Å";

// ... but not codepoint-for-codepoint equal.
assert_eq!(comp, "\u{C5}");
assert_eq!(decomp, "A\u{030A}");

// Their case-foldings are likewise unequal:
assert!(!comp.eq_ignore_case_unnormalized(decomp));
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2935)

#### pub fn [make\_ascii\_uppercase](#method.make_ascii_uppercase)(&mut self)

Converts this string to its ASCII upper case equivalent in-place.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To return a new uppercased value without modifying the existing one, use [`to_ascii_uppercase()`](#method.to_ascii_uppercase).

##### Examples

```rust
let mut s = String::from("Grüße, Jürgen ❤");

s.make_ascii_uppercase();

assert_eq!("GRüßE, JüRGEN ❤", s);
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2963)

#### pub fn [make\_ascii\_lowercase](#method.make_ascii_lowercase)(&mut self)

Converts this string to its ASCII lower case equivalent in-place.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To return a new lowercased value without modifying the existing one, use [`to_ascii_lowercase()`](#method.to_ascii_lowercase).

##### Examples

```rust
let mut s = String::from("GRÜßE, JÜRGEN ❤");

s.make_ascii_lowercase();

assert_eq!("grÜße, jÜrgen ❤", s);
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2991)

#### pub fn [trim\_ascii\_start](#method.trim_ascii_start)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace"). Importantly, this definition excludes the U+000B code point even though it has the Unicode [`White_Space`](https://www.unicode.org/reports/tr44/#White_Space) property and is removed by [`str::trim_start`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start "method str::trim_start").

##### Examples

```rust
assert_eq!(" \t \u{3000}hello world\n".trim_ascii_start(), "\u{3000}hello world\n");
assert_eq!("  ".trim_ascii_start(), "");
assert_eq!("".trim_ascii_start(), "");
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3019)

#### pub fn [trim\_ascii\_end](#method.trim_ascii_end)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace"). Importantly, this definition excludes the U+000B code point even though it has the Unicode [`White_Space`](https://www.unicode.org/reports/tr44/#White_Space) property and is removed by [`str::trim_end`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end "method str::trim_end").

##### Examples

```rust
assert_eq!("\r hello world\u{3000}\n ".trim_ascii_end(), "\r hello world\u{3000}");
assert_eq!("  ".trim_ascii_end(), "");
assert_eq!("".trim_ascii_end(), "");
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3048)

#### pub fn [trim\_ascii](#method.trim_ascii)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading and trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace"). Importantly, this definition excludes the U+000B code point even though it has the Unicode [`White_Space`](https://www.unicode.org/reports/tr44/#White_Space) property and is removed by [`str::trim`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim "method str::trim").

##### Examples

```rust
assert_eq!("\r hello world\n ".trim_ascii(), "hello world");
assert_eq!("  ".trim_ascii(), "");
assert_eq!("".trim_ascii(), "");
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3091)

#### pub fn [escape\_debug](#method.escape_debug)(&self) -> [EscapeDebug](https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html "struct core::str::iter::EscapeDebug")<'\_> [ⓘ](#)

Returns an iterator that escapes each char in `self` with [`char::escape_debug`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_debug "method char::escape_debug").

Note: only extended grapheme codepoints that begin the string will be escaped.

##### Examples

As an iterator:

```rust
for c in "❤\n!".escape_debug() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

```rust
println!("{}", "❤\n!".escape_debug());
```

Both are equivalent to:

```rust
println!("❤\\n!");
```

Using `to_string`:

```rust
assert_eq!("❤\n!".escape_debug().to_string(), "❤\\n!");
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3137)

#### pub fn [escape\_default](#method.escape_default)(&self) -> [EscapeDefault](https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html "struct core::str::iter::EscapeDefault")<'\_> [ⓘ](#)

Returns an iterator that escapes each char in `self` with [`char::escape_default`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_default "method char::escape_default").

##### Examples

As an iterator:

```rust
for c in "❤\n!".escape_default() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

```rust
println!("{}", "❤\n!".escape_default());
```

Both are equivalent to:

```rust
println!("\\u{{2764}}\\n!");
```

Using `to_string`:

```rust
assert_eq!("❤\n!".escape_default().to_string(), "\\u{2764}\\n!");
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3175)

#### pub fn [escape\_unicode](#method.escape_unicode)(&self) -> [EscapeUnicode](https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html "struct core::str::iter::EscapeUnicode")<'\_> [ⓘ](#)

Returns an iterator that escapes each char in `self` with [`char::escape_unicode`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_unicode "method char::escape_unicode").

##### Examples

As an iterator:

```rust
for c in "❤\n!".escape_unicode() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

```rust
println!("{}", "❤\n!".escape_unicode());
```

Both are equivalent to:

```rust
println!("\\u{{2764}}\\u{{a}}\\u{{21}}");
```

Using `to_string`:

```rust
assert_eq!("❤\n!".escape_unicode().to_string(), "\\u{2764}\\u{a}\\u{21}");
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3209)

#### pub fn [substr\_range](#method.substr_range)(&self, substr: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Range](https://doc.rust-lang.org/nightly/core/range/struct.Range.html "struct core::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the range that a substring points to.

Returns `None` if `substr` does not point within `self`.

Unlike [`str::find`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.find "method str::find"), **this does not search through the string**. Instead, it uses pointer arithmetic to find where in the string `substr` is derived from.

This is useful for extending [`str::split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split") and similar methods.

Note that this method may return false positives (typically either `Some(0..0)` or `Some(self.len()..self.len())`) if `substr` is a zero-length `str` that points at the beginning or end of another, independent, `str`.

##### Examples

```rust
#![feature(substr_range)]
use core::range::Range;

let data = "a, b, b, a";
let mut iter = data.split(", ").map(|s| data.substr_range(s).unwrap());

assert_eq!(iter.next(), Some(Range { start: 0, end: 1 }));
assert_eq!(iter.next(), Some(Range { start: 3, end: 4 }));
assert_eq!(iter.next(), Some(Range { start: 6, end: 7 }));
assert_eq!(iter.next(), Some(Range { start: 9, end: 10 }));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3220)

#### pub fn [as\_str](#method.as_str-1)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

🔬This is a nightly-only experimental API. (`str_as_str`)

Returns the same string as a string slice `&str`.

This method is redundant when used directly on `&str`, but it helps dereferencing other string-like types to string slices, for example references to `Box<str>` or `Arc<str>`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#308)

#### pub fn [replace](#method.replace)<P>(&self, from: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [String](struct.String.html "struct bevy::prelude::String")

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Available on **non-`no_global_oom_handling`** only.

Replaces all matches of a pattern with another string.

`replace` creates a new [`String`](struct.String.html "struct bevy::prelude::String"), and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice.

##### Examples

```rust
let s = "this is old";

assert_eq!("this is new", s.replace("old", "new"));
assert_eq!("than an old", s.replace("is", "an"));
```

When the pattern doesn’t match, it returns this string slice as [`String`](struct.String.html "struct bevy::prelude::String"):

```rust
let s = "this is old";
assert_eq!(s, s.replace("cookie monster", "little lamb"));
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#366)

#### pub fn [replacen](#method.replacen)<P>(&self, pat: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), count: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](struct.String.html "struct bevy::prelude::String")

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Available on **non-`no_global_oom_handling`** only.

Replaces first N matches of a pattern with another string.

`replacen` creates a new [`String`](struct.String.html "struct bevy::prelude::String"), and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice at most `count` times.

##### Examples

```rust
let s = "foo foo 123 foo";
assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
```

When the pattern doesn’t match, it returns this string slice as [`String`](struct.String.html "struct bevy::prelude::String"):

```rust
let s = "this is old";
assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#433)

#### pub fn [to\_lowercase](#method.to_lowercase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns the lowercase equivalent of this string slice, as a new [`String`](struct.String.html "struct bevy::prelude::String").

‘Lowercase’ is defined according to the terms of [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G34432) of the Unicode standard.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

Unlike [`char::to_lowercase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_lowercase "method char::to_lowercase"), this method fully handles the context-dependent casing of Greek sigma. However, like that method, it does not handle locale-specific casing, like Turkish and Azeri I/ı/İ/i. See its documentation for more information.

##### Examples

Basic usage:

```rust
let s = "HELLO WORLD";

assert_eq!("hello world", s.to_lowercase());
```

Tricky examples, with sigma:

```rust
let sigma = "Σ";

assert_eq!("σ", sigma.to_lowercase());

// but at the end of a word, it's ς, not σ:
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());

let odysseus_king_of_ithaca = "Ο ΟΔΥΣΣΈΑΣ ΒΑΣΙΛΙΆΣ ΤΗΣ ΙΘΆΚΗΣ";

assert_eq!("ο οδυσσέας βασιλιάς της ιθάκης", odysseus_king_of_ithaca.to_lowercase());
```

Languages without case are not changed:

```rust
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_lowercase());
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#554)

#### pub fn [word\_to\_titlecase](#method.word_to_titlecase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

🔬This is a nightly-only experimental API. (`titlecase`)

Available on **non-`no_global_oom_handling`** only.

Returns the titlecase equivalent of this string slice, which is assumed to represent a single word, as a new [`String`](struct.String.html "struct bevy::prelude::String").

Essentially, this consists of uppercasing the first cased letter (with [`char::to_titlecase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_titlecase "method char::to_titlecase")), and lowercasing everything that follows.

‘Titlecase’ is defined according to the terms of [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G34082) of the Unicode standard.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

Unlike [`char::to_lowercase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_lowercase "method char::to_lowercase"), this method fully handles the context-dependent casing of Greek sigma. However, like that method, it does not handle locale-specific casing, like Turkish and Azeri I/ı/İ/i. See its documentation for more information.

This method does not perform any kind of word segmentation.

##### Examples

Basic usage:

```rust
#![feature(titlecase)]
let s = "HELLO WORLD";

assert_eq!("Hello world", s.word_to_titlecase());
```

The first _cased_ letter is uppercased:

```rust
#![feature(titlecase)]
let the_night_before_christmas = "'twas";

assert_eq!("'Twas", the_night_before_christmas.word_to_titlecase());
```

Languages without case are not changed:

```rust
#![feature(titlecase)]
let new_year = "农历新年";

assert_eq!(new_year, new_year.word_to_titlecase());
```

Georgian uppercase (“Mtavruli”) letters are not used in titlecase:

```rust
#![feature(titlecase)]
let georgian = "ერთობაშია";

assert_eq!(georgian, georgian.word_to_titlecase());
```

No word segmentation is performed, so only the first cased letter in the whole string gets uppercased:

```rust
#![feature(titlecase)]
let blazingly_fast = "ferris and I";

assert_eq!("Ferris and i", blazingly_fast.word_to_titlecase());
```

Tricky examples, with sigma:

```rust
#![feature(titlecase)]
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("Ὀδυσσεύς", odysseus.word_to_titlecase());

let odysseus_king_of_ithaca = "Ο ΟΔΥΣΣΈΑΣ ΒΑΣΙΛΙΆΣ ΤΗΣ ΙΘΆΚΗΣ";

assert_eq!("Ο οδυσσέας βασιλιάς της ιθάκης", odysseus_king_of_ithaca.word_to_titlecase());
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#640)

#### pub fn [to\_uppercase](#method.to_uppercase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns the uppercase equivalent of this string slice, as a new [`String`](struct.String.html "struct bevy::prelude::String").

‘Uppercase’ is defined according to the terms of [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G34431) of the Unicode standard.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

Like [`char::to_uppercase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_uppercase "method char::to_uppercase") this method does not handle language-specific casing, like Turkish and Azeri I/ı/İ/i. See that method’s documentation for more information.

##### Examples

Basic usage:

```rust
let s = "hello world";

assert_eq!("HELLO WORLD", s.to_uppercase());
```

Scripts without case are not changed:

```rust
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_uppercase());
```

One character can become multiple:

```rust
let s = "tschüß";

assert_eq!("TSCHÜSS", s.to_uppercase());
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#742)

#### pub fn [to\_casefold\_unnormalized](#method.to_casefold_unnormalized)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

🔬This is a nightly-only experimental API. (`casefold`)

Available on **non-`no_global_oom_handling`** only.

Returns the case-folded equivalent of this string slice, as a new [`String`](struct.String.html "struct bevy::prelude::String").

Case folding is a transformation, mostly matching lowercase, that is meant to be used for case-insensitive string comparisons. Case-folded strings should not usually be exposed directly to users.

For the precise specification of case folding, see [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G63737) of the Unicode standard.

Since some characters can expand into multiple characters when case folding, this function returns a [`String`](struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

No [normalization](https://www.unicode.org/faq/normalization.html) (e.g. NFC) is performed, so visually and semantically identical strings might still casefold differently. For example, `"Å"` (U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE) is considered distinct from `"Å"` (A followed by U+030A COMBINING RING ABOVE), even though Unicode considers them canonically equivalent.

Like [`char::to_casefold_unnormalized()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_casefold_unnormalized "method char::to_casefold_unnormalized") this method does not handle language-specific casing, like Turkish and Azeri I/ı/İ/i. See that method’s documentation for more information.

##### Examples

Basic usage:

```rust
#![feature(casefold)]
let s0 = "HELLO";
let s1 = "Hello";

assert_eq!(s0.to_casefold_unnormalized(), s1.to_casefold_unnormalized());
assert_eq!(s0.to_casefold_unnormalized(), "hello")
```

Scripts without case are not changed:

```rust
#![feature(casefold)]
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_casefold_unnormalized());
```

One character can become multiple:

```rust
#![feature(casefold)]
let s0 = "TSCHÜẞ";
let s1 = "TSCHÜSS";
let s2 = "tschüß";

assert_eq!(s0.to_casefold_unnormalized(), s1.to_casefold_unnormalized());
assert_eq!(s0.to_casefold_unnormalized(), s2.to_casefold_unnormalized());
assert_eq!(s0.to_casefold_unnormalized(), "tschüss");
```

No NFC [normalization](https://www.unicode.org/faq/normalization.html) is performed:

```rust
#![feature(casefold)]
// These two strings are visually and semantically identical...
let comp = "Å";
let decomp = "Å";

// ... but not codepoint-for-codepoint equal.
assert_eq!(comp, "\u{C5}");
assert_eq!(decomp, "A\u{030A}");

// Their case-foldings are likewise unequal:
assert_eq!(comp.to_casefold_unnormalized(), "\u{E5}");
assert_eq!(decomp.to_casefold_unnormalized(), "a\u{030A}");
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#808)

#### pub fn [repeat](#method.repeat)(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Creates a new [`String`](struct.String.html "struct bevy::prelude::String") by repeating a string `n` times.

##### Panics

This function will panic if the capacity would overflow.

##### Examples

Basic usage:

```rust
assert_eq!("abc".repeat(4), String::from("abcabcabcabc"));
```

A panic upon overflow:

[ⓘ](# "This example panics")

```rust
// this will panic at runtime
let huge = "0123456789abcdef".repeat(usize::MAX);
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#838)

#### pub fn [to\_ascii\_uppercase](#method.to_ascii_uppercase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns a copy of this string where each character is mapped to its ASCII upper case equivalent.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To uppercase the value in-place, use [`make_ascii_uppercase`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_uppercase "method str::make_ascii_uppercase").

To uppercase ASCII characters in addition to non-ASCII characters, use [`to_uppercase`](#method.to_uppercase).

##### Examples

```rust
let s = "Grüße, Jürgen ❤";

assert_eq!("GRüßE, JüRGEN ❤", s.to_ascii_uppercase());
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#870)

#### pub fn [to\_ascii\_lowercase](#method.to_ascii_lowercase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns a copy of this string where each character is mapped to its ASCII lower case equivalent.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To lowercase the value in-place, use [`make_ascii_lowercase`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_lowercase "method str::make_ascii_lowercase").

To lowercase ASCII characters in addition to non-ASCII characters, use [`to_lowercase`](#method.to_lowercase).

##### Examples

```rust
let s = "Grüße, Jürgen ❤";

assert_eq!("grüße, jürgen ❤", s.to_ascii_lowercase());
```

## Trait Implementations

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1462)

### impl<'i> [Accumulate](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html "trait winnow::stream::Accumulate")<&'i [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1464)

#### fn [initial](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html#tymethod.initial)(capacity: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [String](struct.String.html "struct bevy::prelude::String")

Create a new `Extend` of the correct type

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1471)

#### fn [accumulate](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html#tymethod.accumulate)(&mut self, acc: &'i [str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Accumulate the input into an accumulator

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1477)

### impl<'i> [Accumulate](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html "trait winnow::stream::Accumulate")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'i, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1479)

#### fn [initial](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html#tymethod.initial)(capacity: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [String](struct.String.html "struct bevy::prelude::String")

Create a new `Extend` of the correct type

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1486)

#### fn [accumulate](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html#tymethod.accumulate)(&mut self, acc: [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'i, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>)

Accumulate the input into an accumulator

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1492)

### impl [Accumulate](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html "trait winnow::stream::Accumulate")<[String](struct.String.html "struct bevy::prelude::String")\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1494)

#### fn [initial](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html#tymethod.initial)(capacity: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [String](struct.String.html "struct bevy::prelude::String")

Create a new `Extend` of the correct type

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1501)

#### fn [accumulate](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html#tymethod.accumulate)(&mut self, acc: [String](struct.String.html "struct bevy::prelude::String"))

Accumulate the input into an accumulator

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1447)

### impl [Accumulate](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html "trait winnow::stream::Accumulate")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1449)

#### fn [initial](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html#tymethod.initial)(capacity: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [String](struct.String.html "struct bevy::prelude::String")

Create a new `Extend` of the correct type

[Source](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/src/winnow/stream/mod.rs.html#1456)

#### fn [accumulate](https://docs.rs/winnow/0.7.15/x86_64-unknown-linux-gnu/winnow/stream/trait.Accumulate.html#tymethod.accumulate)(&mut self, acc: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html))

Accumulate the input into an accumulator

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2780)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Implements the `+` operator for concatenating two strings.

This consumes the `String` on the left-hand side and re-uses its buffer (growing it if necessary). This is done to avoid allocating a new `String` and copying the entire contents on every operation, which would lead to _O_(_n_^2) running time when building an _n_\-byte string by repeated concatenation.

The string on the right-hand side is only borrowed; its contents are copied into the returned `String`.

#### Examples

Concatenating two `String`s takes the first by value and borrows the second:

```rust
let a = String::from("hello");
let b = String::from(" world");
let c = a + &b;
// `a` is moved and can no longer be used here.
```

If you want to keep using the first `String`, you can clone it and append to the clone instead:

```rust
let a = String::from("hello");
let b = String::from(" world");
let c = a.clone() + &b;
// `a` is still valid here.
```

Concatenating `&str` slices can be done by converting the first to a `String`:

```rust
let a = "hello";
let b = " world";
let c = a.to_string() + b;
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2781)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [String](struct.String.html "struct bevy::prelude::String")

The resulting type after applying the `+` operator.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2784)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [String](struct.String.html "struct bevy::prelude::String")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

1.12.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2795)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Implements the `+=` operator for appending to a `String`.

This has the same behavior as the [`push_str`](struct.String.html#method.push_str "method bevy::prelude::String::push_str") method.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2797)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#150)

### impl [Arg](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html "trait rustix::path::arg::Arg") for &[String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#152)

#### fn [as\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.as_str)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

Returns a view of this string as a string slice.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#158)

#### fn [to\_string\_lossy](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.to_string_lossy)(&self) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns a potentially-lossy rendering of this string as a `Cow<'_, str>`.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#164)

#### fn [as\_cow\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.as_cow_c_str)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")\>, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

Returns a view of this string as a maybe-owned [`CStr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr").

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#172-174)

#### fn [into\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.into_c_str)<'b>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'b, [CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")\>, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

where &[String](struct.String.html "struct bevy::prelude::String"): 'b,

Consumes `self` and returns a view of this string as a maybe-owned [`CStr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr").

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#180-183)

#### fn [into\_with\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.into_with_c_str)<T, F>(self, f: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

where &[String](struct.String.html "struct bevy::prelude::String"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&[CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>,

Runs a closure with `self` passed in as a `&CStr`.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#190)

### impl [Arg](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html "trait rustix::path::arg::Arg") for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#192)

#### fn [as\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.as_str)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

Returns a view of this string as a string slice.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#198)

#### fn [to\_string\_lossy](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.to_string_lossy)(&self) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns a potentially-lossy rendering of this string as a `Cow<'_, str>`.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#204)

#### fn [as\_cow\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.as_cow_c_str)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")\>, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

Returns a view of this string as a maybe-owned [`CStr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr").

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#212-214)

#### fn [into\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.into_c_str)<'b>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'b, [CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")\>, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

where [String](struct.String.html "struct bevy::prelude::String"): 'b,

Consumes `self` and returns a view of this string as a maybe-owned [`CStr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr").

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#222-225)

#### fn [into\_with\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.into_with_c_str)<T, F>(self, f: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

where [String](struct.String.html "struct bevy::prelude::String"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&[CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>,

Runs a closure with `self` passed in as a `&CStr`.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#148)

### impl [Arg](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html "trait rustix::path::arg::Arg") for &[String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#150)

#### fn [as\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.as_str)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

Returns a view of this string as a string slice.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#156)

#### fn [to\_string\_lossy](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.to_string_lossy)(&self) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns a potentially-lossy rendering of this string as a `Cow<'_, str>`.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#162)

#### fn [as\_cow\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.as_cow_c_str)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")\>, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

Returns a view of this string as a maybe-owned [`CStr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr").

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#170-172)

#### fn [into\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.into_c_str)<'b>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'b, [CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")\>, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

where &[String](struct.String.html "struct bevy::prelude::String"): 'b,

Consumes `self` and returns a view of this string as a maybe-owned [`CStr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr").

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#178-181)

#### fn [into\_with\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.into_with_c_str)<T, F>(self, f: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

where &[String](struct.String.html "struct bevy::prelude::String"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&[CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>,

Runs a closure with `self` passed in as a `&CStr`.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#188)

### impl [Arg](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html "trait rustix::path::arg::Arg") for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#190)

#### fn [as\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.as_str)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

Returns a view of this string as a string slice.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#196)

#### fn [to\_string\_lossy](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.to_string_lossy)(&self) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns a potentially-lossy rendering of this string as a `Cow<'_, str>`.

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#202)

#### fn [as\_cow\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.as_cow_c_str)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")\>, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

Returns a view of this string as a maybe-owned [`CStr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr").

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#210-212)

#### fn [into\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.into_c_str)<'b>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'b, [CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")\>, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

where [String](struct.String.html "struct bevy::prelude::String"): 'b,

Consumes `self` and returns a view of this string as a maybe-owned [`CStr`](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr").

[Source](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/src/rustix/path/arg.rs.html#220-223)

#### fn [into\_with\_c\_str](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/path/arg/trait.Arg.html#tymethod.into_with_c_str)<T, F>(self, f: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>

where [String](struct.String.html "struct bevy::prelude::String"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&[CStr](https://doc.rust-lang.org/nightly/core/ffi/c_str/struct.CStr.html "struct core::ffi::c_str::CStr")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Errno](https://docs.rs/rustix/0.38.44/x86_64-unknown-linux-gnu/rustix/backend/io/errno/struct.Errno.html "struct rustix::backend::io::errno::Errno")\>,

Runs a closure with `self` passed in as a `&CStr`.

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/header/map.rs.html#3996)

### impl [AsHeaderName](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/header/map/as_header_name/trait.AsHeaderName.html "trait http::header::map::as_header_name::AsHeaderName") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/header/map.rs.html#4014)

### impl [AsHeaderName](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/header/map/as_header_name/trait.AsHeaderName.html "trait http::header::map::as_header_name::AsHeaderName") for &[String](struct.String.html "struct bevy::prelude::String")

1.43.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3100)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3102)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Converts this type into a mutable reference of the (usually inferred) input type.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/ffi/os_str.rs.html#1777)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[OsStr](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html "struct std::ffi::os_str::OsStr")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/std/ffi/os_str.rs.html#1779)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &[OsStr](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsStr.html "struct std::ffi::os_str::OsStr")

Converts this type into a shared reference of the (usually inferred) input type.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#3912)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#3914)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &[Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")

Converts this type into a shared reference of the (usually inferred) input type.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3108)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3110)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

Converts this type into a shared reference of the (usually inferred) input type.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3092)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3094)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/ureq/3.3.0/x86_64-unknown-linux-gnu/src/ureq/send_body.rs.html#309)

### impl [AsSendBody](https://docs.rs/ureq/3.3.0/x86_64-unknown-linux-gnu/ureq/send_body/trait.AsSendBody.html "trait ureq::send_body::AsSendBody") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/ureq/3.3.0/x86_64-unknown-linux-gnu/src/ureq/send_body.rs.html#325)

### impl [AsSendBody](https://docs.rs/ureq/3.3.0/x86_64-unknown-linux-gnu/ureq/send_body/trait.AsSendBody.html "trait ureq::send_body::AsSendBody") for &[String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/basic.rs.html#181)

### impl [Basic](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/basic/trait.Basic.html "trait zvariant::basic::Basic") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/basic.rs.html#182)

#### const [SIGNATURE\_CHAR](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/basic/trait.Basic.html#associatedconstant.SIGNATURE_CHAR): [char](https://doc.rust-lang.org/nightly/std/primitive.char.html) = 's'

The type signature, as a character.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/basic.rs.html#183)

#### const [SIGNATURE\_STR](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/basic/trait.Basic.html#associatedconstant.SIGNATURE_STR): &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html) = "s"

The type signature, as a string.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/basic.rs.html#19)

#### fn [alignment](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/basic/trait.Basic.html#method.alignment)(format: [Format](https://docs.rs/zvariant_utils/3.3.0/x86_64-unknown-linux-gnu/zvariant_utils/serialized/enum.Format.html "enum zvariant_utils::serialized::Format")) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

The required padding alignment for the given format. [Read more](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/basic/trait.Basic.html#method.alignment)

[Source](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/src/http_body/lib.rs.html#185)

### impl [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/src/http_body/lib.rs.html#186)

#### type [Data](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data) = [Bytes](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")

Values yielded by the `Body`.

[Source](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/src/http_body/lib.rs.html#187)

#### type [Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The error type this `Body` might generate.

[Source](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/src/http_body/lib.rs.html#189-192)

#### fn [poll\_frame](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#tymethod.poll_frame)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [String](struct.String.html "struct bevy::prelude::String")\>, \_cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Frame](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/frame/struct.Frame.html "struct http_body::frame::Frame")<<[String](struct.String.html "struct bevy::prelude::String") as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Data](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data "type http_body::Body::Data")\>, <[String](struct.String.html "struct bevy::prelude::String") as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error")\>>>

Attempt to pull out the next data buffer of this stream.

[Source](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/src/http_body/lib.rs.html#201)

#### fn [is\_end\_stream](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.is_end_stream)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` when the end of stream has been reached. [Read more](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.is_end_stream)

[Source](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/src/http_body/lib.rs.html#205)

#### fn [size\_hint](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.size_hint)(&self) -> [SizeHint](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/size_hint/struct.SizeHint.html "struct http_body::size_hint::SizeHint")

Returns the bounds on the remaining length of the stream. [Read more](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#method.size_hint)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#229)

### impl [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#231)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#237)

### impl [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#239)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/sharded-slab/0.1.7/x86_64-unknown-linux-gnu/src/sharded_slab/clear.rs.html#74)

### impl [Clear](https://docs.rs/sharded-slab/0.1.7/x86_64-unknown-linux-gnu/sharded_slab/clear/trait.Clear.html "trait sharded_slab::clear::Clear") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/sharded-slab/0.1.7/x86_64-unknown-linux-gnu/src/sharded_slab/clear.rs.html#76)

#### fn [clear](https://docs.rs/sharded-slab/0.1.7/x86_64-unknown-linux-gnu/sharded_slab/clear/trait.Clear.html#tymethod.clear)(&mut self)

Clear all data in `self`, retaining the allocated capacithy.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2359)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2368)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &[String](struct.String.html "struct bevy::prelude::String"))

Clones the contents of `source` into `self`.

This method is preferred over simply assigning `source.clone()` to `self`, as it avoids reallocation if possible.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2360)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2726)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2728)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143894 "Tracking issue for const_default")) · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2709)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2712)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [String](struct.String.html "struct bevy::prelude::String")

Creates an empty `String`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2827)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2828)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

The resulting type after dereferencing.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2831)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Dereferences the value.

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2840)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2842)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Mutably dereferences the value.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2837)

### impl [DerefPure](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefPure.html "trait core::ops::deref::DerefPure") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/impls.rs.html#688)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate features `alloc` or `std`** only.

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/impls.rs.html#689-691)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2718)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2720)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/encode.rs.html#152)

### impl [EncodeAsVarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html "trait zerovec::ule::encode::EncodeAsVarULE")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/encode.rs.html#153)

#### fn [encode\_var\_ule\_as\_slices](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#tymethod.encode_var_ule_as_slices)<R>(&self, cb: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&\[&\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]\]) -> R) -> R

Calls `cb` with a piecewise list of byte slices that when concatenated produce the memory pattern of the corresponding instance of `T`. [Read more](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#tymethod.encode_var_ule_as_slices)

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/encode.rs.html#72)

#### fn [encode\_var\_ule\_len](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#method.encode_var_ule_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Return the length, in bytes, of the corresponding [`VarULE`](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") type

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/encode.rs.html#78)

#### fn [encode\_var\_ule\_write](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#method.encode_var_ule_write)(&self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

Write the corresponding [`VarULE`](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") type to the `dst` buffer. `dst` should be the size of [`Self::encode_var_ule_len()`](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#method.encode_var_ule_len "method zerovec::ule::encode::EncodeAsVarULE::encode_var_ule_len")

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/encode.rs.html#159)

### impl [EncodeAsVarULE](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html "trait zerovec::ule::encode::EncodeAsVarULE")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for &[String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/encode.rs.html#160)

#### fn [encode\_var\_ule\_as\_slices](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#tymethod.encode_var_ule_as_slices)<R>(&self, cb: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&\[&\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]\]) -> R) -> R

Calls `cb` with a piecewise list of byte slices that when concatenated produce the memory pattern of the corresponding instance of `T`. [Read more](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#tymethod.encode_var_ule_as_slices)

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/encode.rs.html#72)

#### fn [encode\_var\_ule\_len](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#method.encode_var_ule_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Return the length, in bytes, of the corresponding [`VarULE`](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") type

[Source](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/src/zerovec/ule/encode.rs.html#78)

#### fn [encode\_var\_ule\_write](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#method.encode_var_ule_write)(&self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

Write the corresponding [`VarULE`](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/trait.VarULE.html "trait zerovec::ule::VarULE") type to the `dst` buffer. `dst` should be the size of [`Self::encode_var_ule_len()`](https://docs.rs/zerovec/0.11.6/x86_64-unknown-linux-gnu/zerovec/ule/encode/trait.EncodeAsVarULE.html#method.encode_var_ule_len "method zerovec::ule::encode::EncodeAsVarULE::encode_var_ule_len")

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#350)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2602)

### impl<'a> [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<&'a [AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2604)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2609)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, c: &'a [AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"))

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2497)

### impl<'a> [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<&'a [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2498)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2503)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, \_: &'a [char](https://doc.rust-lang.org/nightly/std/primitive.char.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2508)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2515)

### impl<'a> [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2516)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2521)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, s: &'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2588)

### impl [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<[AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2590)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2595)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, c: [AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"))

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

1.45.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2554)

### impl<A> [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<[Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), A>> for [String](struct.String.html "struct bevy::prelude::String")

where A: [Allocator](https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html "trait core::alloc::Allocator"),

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2555)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), A>>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/src/allocator_api2/stable/boxed.rs.html#2203)

### impl<A> [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<[Box](https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/allocator_api2/stable/boxed/struct.Box.html "struct allocator_api2::stable::boxed::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), A>> for [String](struct.String.html "struct bevy::prelude::String")

where A: [Allocator](https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/allocator_api2/stable/alloc/trait.Allocator.html "trait allocator_api2::stable::alloc::Allocator"),

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/src/allocator_api2/stable/boxed.rs.html#2204)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Box](https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/allocator_api2/stable/boxed/struct.Box.html "struct allocator_api2::stable::boxed::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), A>>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

1.19.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2575)

### impl<'a> [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2576)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2581)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, s: [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2562)

### impl [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<[String](struct.String.html "struct bevy::prelude::String")\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2563)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [String](struct.String.html "struct bevy::prelude::String")\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2568)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, s: [String](struct.String.html "struct bevy::prelude::String"))

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2476)

### impl [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2477)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2485)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, c: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2490)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/src/phf_shared/lib.rs.html#206)

### impl [FmtConst](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/phf_shared/trait.FmtConst.html "trait phf_shared::FmtConst") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/src/phf_shared/lib.rs.html#206)

#### fn [fmt\_const](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/phf_shared/trait.FmtConst.html#tymethod.fmt_const)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Print a `const` expression representing this value.

1.28.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3260)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [String](struct.String.html "struct bevy::prelude::String")\> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3275)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(s: &'a [String](struct.String.html "struct bevy::prelude::String")) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Converts a [`String`](struct.String.html "struct bevy::prelude::String") reference into a [`Borrowed`](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html#variant.Borrowed "borrow::Cow::Borrowed") variant. No heap allocation is performed, and the string is not copied.

##### Example

```rust
let s = "eggplant".to_string();
assert_eq!(Cow::from(&s), Cow::Borrowed("eggplant"));
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/path.rs.html#604)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [String](struct.String.html "struct bevy::prelude::String")\> for [AssetPath](../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/path.rs.html#606)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(asset\_path: &'a [String](struct.String.html "struct bevy::prelude::String")) -> [AssetPath](../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>

Converts to this type from the input type.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/into_value.rs.html#54)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [String](struct.String.html "struct bevy::prelude::String")\> for [Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'a>

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/into_value.rs.html#54)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: &'a [String](struct.String.html "struct bevy::prelude::String")) -> [Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'a>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#199)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[Name](struct.Name.html "struct bevy::prelude::Name")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#201)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(val: &[Name](struct.Name.html "struct bevy::prelude::Name")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

1.35.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3141)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[String](struct.String.html "struct bevy::prelude::String")\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3146)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(s: &[String](struct.String.html "struct bevy::prelude::String")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts a `&String` into a [`String`](struct.String.html "struct bevy::prelude::String").

This clones `s` and returns the clone.

1.44.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3129)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3134)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(s: &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [String](struct.String.html "struct bevy::prelude::String")

Converts a `&mut str` into a [`String`](struct.String.html "struct bevy::prelude::String").

The result is allocated on the heap.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3117)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3122)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(s: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [String](struct.String.html "struct bevy::prelude::String")

Converts a `&str` into a [`String`](struct.String.html "struct bevy::prelude::String").

The result is allocated on the heap.

1.18.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3153)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3166)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(s: [Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>) -> [String](struct.String.html "struct bevy::prelude::String")

Converts the given boxed `str` slice to a [`String`](struct.String.html "struct bevy::prelude::String"). It is notable that the `str` slice is owned.

##### Examples

```rust
let s1: String = String::from("hello world");
let s2: Box<str> = s1.into_boxed_str();
let s3: String = String::from(s2);

assert_eq!("hello world", s3)
```

1.14.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3192)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3209)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(s: [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>) -> [String](struct.String.html "struct bevy::prelude::String")

Converts a clone-on-write string to an owned instance of [`String`](struct.String.html "struct bevy::prelude::String").

This extracts the owned string, clones the string if it is not already owned.

##### Example

```rust
// If the string is not owned...
let cow: Cow<'_, str> = Cow::Borrowed("eggplant");
// It will allocate on the heap and copy the string.
let owned: String = String::from(cow);
assert_eq!(&owned[..], "eggplant");
```

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#87)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[DiagnosticPath](../diagnostic/struct.DiagnosticPath.html "struct bevy::diagnostic::DiagnosticPath")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#88)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(path: [DiagnosticPath](../diagnostic/struct.DiagnosticPath.html "struct bevy::diagnostic::DiagnosticPath")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/guid.rs.html#131)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Guid](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/guid/struct.Guid.html "struct zbus::guid::Guid")<'\_>> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/guid.rs.html#132)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(guid: [Guid](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/guid/struct.Guid.html "struct zbus::guid::Guid")<'\_>) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#206)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Name](struct.Name.html "struct bevy::prelude::Name")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#208)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(val: [Name](struct.Name.html "struct bevy::prelude::Name")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/src/read_fonts/tables/meta.rs.html#48)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[ScriptLangTag](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/read_fonts/tables/meta/struct.ScriptLangTag.html "struct read_fonts::tables::meta::ScriptLangTag")<'\_>> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `std`** only.

[Source](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/src/read_fonts/tables/meta.rs.html#49)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [ScriptLangTag](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/read_fonts/tables/meta/struct.ScriptLangTag.html "struct read_fonts::tables::meta::ScriptLangTag")<'\_>) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/src/read_fonts/tables/meta.rs.html#48)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[ScriptLangTag](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/read_fonts/tables/meta/struct.ScriptLangTag.html "struct read_fonts::tables::meta::ScriptLangTag")<'\_>> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `std`** only.

[Source](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/src/read_fonts/tables/meta.rs.html#49)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [ScriptLangTag](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/read_fonts/tables/meta/struct.ScriptLangTag.html "struct read_fonts::tables::meta::ScriptLangTag")<'\_>) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#393)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#395)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(text: [SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/str.rs.html#172)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Str](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/str/struct.Str.html "struct zvariant::str::Str")<'a>> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/str.rs.html#173)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Str](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/str/struct.Str.html "struct zvariant::str::Str")<'a>) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/boxed/convert.rs.html#563)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'a>

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/boxed/convert.rs.html#577)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(err: [String](struct.String.html "struct bevy::prelude::String")) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'a>

Converts a [`String`](struct.String.html "struct bevy::prelude::String") into a box of dyn [`Error`](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync").

##### Examples

```rust
use std::error::Error;

let a_string_error = "a string error".to_string();
let a_boxed_error = Box::<dyn Error + Send + Sync>::from(a_string_error);
assert!(
    size_of::<Box<dyn Error + Send + Sync>>() == size_of_val(&a_boxed_error))
```

1.6.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/boxed/convert.rs.html#601)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + 'a>

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/boxed/convert.rs.html#613)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(str\_err: [String](struct.String.html "struct bevy::prelude::String")) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + 'a>

Converts a [`String`](struct.String.html "struct bevy::prelude::String") into a box of dyn [`Error`](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error").

##### Examples

```rust
use std::error::Error;

let a_string_error = "a string error".to_string();
let a_boxed_error = Box::<dyn Error>::from(a_string_error);
assert!(size_of::<Box<dyn Error>>() == size_of_val(&a_boxed_error))
```

1.21.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/rc.rs.html#2954)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/rc.rs.html#2966)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [String](struct.String.html "struct bevy::prelude::String")) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Allocates a reference-counted string slice and copies `v` into it.

##### Example

```rust
let original: String = "statue".to_owned();
let shared: Rc<str> = Rc::from(original);
assert_eq!("statue", &shared[..]);
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3173)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3185)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(s: [String](struct.String.html "struct bevy::prelude::String")) -> [Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Converts the given [`String`](struct.String.html "struct bevy::prelude::String") to a boxed `str` slice that is owned.

##### Examples

```rust
let s1: String = String::from("hello world");
let s2: Box<str> = Box::from(s1);
let s3: String = String::from(s2);

assert_eq!("hello world", s3)
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3237)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3253)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(s: [String](struct.String.html "struct bevy::prelude::String")) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Converts a [`String`](struct.String.html "struct bevy::prelude::String") into an [`Owned`](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html#variant.Owned "borrow::Cow::Owned") variant. No heap allocation is performed, and the string is not copied.

##### Example

```rust
let s = "eggplant".to_string();
let s2 = "eggplant".to_string();
assert_eq!(Cow::from(s), Cow::<'static, str>::Owned(s2));
```

1.14.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3313)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3326)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(string: [String](struct.String.html "struct bevy::prelude::String")) -> [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> [ⓘ](#)

Converts the given [`String`](struct.String.html "struct bevy::prelude::String") to a vector [`Vec`](struct.Vec.html "struct bevy::prelude::Vec") that holds values of type [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8").

##### Examples

```rust
let s1 = String::from("hello world");
let v1 = Vec::from(s1);

for b in v1 {
    println!("{b}");
}
```

1.21.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#3998)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/sync.rs.html#4010)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [String](struct.String.html "struct bevy::prelude::String")) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> [ⓘ](#)

Allocates a reference-counted `str` and copies `v` into it.

##### Example

```rust
let unique: String = "eggplant".to_owned();
let shared: Arc<str> = Arc::from(unique);
assert_eq!("eggplant", &shared[..]);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#69)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [HashedStr](../ecs/name/struct.HashedStr.html "struct bevy::ecs::name::HashedStr")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#70)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [HashedStr](../ecs/name/struct.HashedStr.html "struct bevy::ecs::name::HashedStr")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#183)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#185)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(name: [String](struct.String.html "struct bevy::prelude::String")) -> [Name](struct.Name.html "struct bevy::prelude::Name")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/debug_info.rs.html#133)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [DebugName](struct.DebugName.html "struct bevy::prelude::DebugName")

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/debug_info.rs.html#134)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [DebugName](struct.DebugName.html "struct bevy::prelude::DebugName")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/source.rs.html#99)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [AssetSourceId](../asset/io/enum.AssetSourceId.html "enum bevy::asset::io::AssetSourceId")<'static>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/source.rs.html#100)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [AssetSourceId](../asset/io/enum.AssetSourceId.html "enum bevy::asset::io::AssetSourceId")<'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/path.rs.html#611)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [AssetPath](../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'static>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/path.rs.html#613)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(asset\_path: [String](struct.String.html "struct bevy::prelude::String")) -> [AssetPath](../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader_cache.rs.html#100)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [ShaderDefVal](../shader/enum.ShaderDefVal.html "enum bevy::shader::ShaderDefVal")

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader_cache.rs.html#101)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(key: [String](struct.String.html "struct bevy::prelude::String")) -> [ShaderDefVal](../shader/enum.ShaderDefVal.html "enum bevy::shader::ShaderDefVal")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#217)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [TextSpan](struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#218)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [TextSpan](struct.TextSpan.html "struct bevy::prelude::TextSpan")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#135)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Text](struct.Text.html "struct bevy::prelude::Text")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#136)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Text](struct.Text.html "struct bevy::prelude::Text")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#126)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Text2d](struct.Text2d.html "struct bevy::prelude::Text2d")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#127)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Text2d](struct.Text2d.html "struct bevy::prelude::Text2d")

Converts to this type from the input type.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/into_value.rs.html#54)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'a>

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/into_value.rs.html#54)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [String](struct.String.html "struct bevy::prelude::String")) -> [Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'a>

Converts to this type from the input type.

[Source](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/src/read_fonts/model/pen.rs.html#187)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[SvgPen](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/read_fonts/model/pen/struct.SvgPen.html "struct read_fonts::model::pen::SvgPen")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/src/read_fonts/model/pen.rs.html#188)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [SvgPen](https://docs.rs/read-fonts/0.37.0/x86_64-unknown-linux-gnu/read_fonts/model/pen/struct.SvgPen.html "struct read_fonts::model::pen::SvgPen")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/skrifa/0.40.0/x86_64-unknown-linux-gnu/src/skrifa/outline/pen.rs.html#210)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[SvgPen](https://docs.rs/skrifa/0.40.0/x86_64-unknown-linux-gnu/skrifa/outline/pen/struct.SvgPen.html "struct skrifa::outline::pen::SvgPen")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/skrifa/0.40.0/x86_64-unknown-linux-gnu/src/skrifa/outline/pen.rs.html#211)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [SvgPen](https://docs.rs/skrifa/0.40.0/x86_64-unknown-linux-gnu/skrifa/outline/pen/struct.SvgPen.html "struct skrifa::outline::pen::SvgPen")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/fmt.rs.html#38)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Uuid](../asset/uuid/struct.Uuid.html "struct bevy::asset::uuid::Uuid")\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `std`** only.

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/fmt.rs.html#39)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(uuid: [Uuid](../asset/uuid/struct.Uuid.html "struct bevy::asset::uuid::Uuid")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

1.46.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3602)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3612)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(c: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)) -> [String](struct.String.html "struct bevy::prelude::String")

Allocates an owned [`String`](struct.String.html "struct bevy::prelude::String") from a single character.

##### Example

```rust
let c: char = 'a';
let s: String = String::from(c);
assert_eq!("a", &s[..]);
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [String](struct.String.html "struct bevy::prelude::String")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[String](struct.String.html "struct bevy::prelude::String") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2465)

### impl<'a> [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<&'a [AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2466)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<T>(iter: T) -> [String](struct.String.html "struct bevy::prelude::String")

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

1.17.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2385)

### impl<'a> [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<&'a [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2386)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [String](struct.String.html "struct bevy::prelude::String")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2395)

### impl<'a> [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2396)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [String](struct.String.html "struct bevy::prelude::String")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2454)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2455)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<T>(iter: T) -> [String](struct.String.html "struct bevy::prelude::String")

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

1.45.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2424)

### impl<A> [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), A>> for [String](struct.String.html "struct bevy::prelude::String")

where A: [Allocator](https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html "trait core::alloc::Allocator"),

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2425)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [String](struct.String.html "struct bevy::prelude::String")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), A>>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

1.19.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2434)

### impl<'a> [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2435)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [String](struct.String.html "struct bevy::prelude::String")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/boxed/iter.rs.html#174)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[String](struct.String.html "struct bevy::prelude::String")\> for [Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/boxed/iter.rs.html#175)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<T>(iter: T) -> [Box](struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [String](struct.String.html "struct bevy::prelude::String")\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2405)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[String](struct.String.html "struct bevy::prelude::String")\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2406)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [String](struct.String.html "struct bevy::prelude::String")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [String](struct.String.html "struct bevy::prelude::String")\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

1.12.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3298)

### impl<'a> [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[String](struct.String.html "struct bevy::prelude::String")\> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3299)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(it: I) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [String](struct.String.html "struct bevy::prelude::String")\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2375)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2376)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<I>(iter: I) -> [String](struct.String.html "struct bevy::prelude::String")

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [from\_reflect](trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[String](struct.String.html "struct bevy::prelude::String")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](trait.FromReflect.html#method.take_from_reflect)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2857)

### impl [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2858)

#### type [Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The associated error which can be returned from parsing.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2860)

#### fn [from\_str](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)(s: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), <[String](struct.String.html "struct bevy::prelude::String") as [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr")\>::[Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "type core::str::traits::FromStr::Err")\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#82)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(\_registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2734)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2736)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, hasher: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/src/quote/ident_fragment.rs.html#88)

### impl [IdentFragment](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/ident_fragment/trait.IdentFragment.html "trait quote::ident_fragment::IdentFragment") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/src/quote/ident_fragment.rs.html#88)

#### fn [fmt](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/ident_fragment/trait.IdentFragment.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Format this value as an identifier fragment.

[Source](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/src/quote/ident_fragment.rs.html#21)

#### fn [span](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/ident_fragment/trait.IdentFragment.html#method.span)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Span](https://docs.rs/proc-macro2/1.0.106/x86_64-unknown-linux-gnu/proc_macro2/struct.Span.html "struct proc_macro2::Span")\>

Span associated with this `IdentFragment`. [Read more](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/ident_fragment/trait.IdentFragment.html#method.span)

[Source](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/src/serde_json/value/index.rs.html#107)

### impl [Index](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/serde_json/value/index/trait.Index.html "trait serde_json::value::index::Index") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/toml/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml/value.rs.html#359)

### impl [Index](https://docs.rs/toml/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/toml/value/trait.Index.html "trait toml::value::Index") for [String](struct.String.html "struct bevy::prelude::String")

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2803-2805)

### impl<I> [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<I> for [String](struct.String.html "struct bevy::prelude::String")

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2807)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

The returned type after indexing.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2810)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: I) -> &<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2816-2818)

### impl<I> [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<I> for [String](struct.String.html "struct bevy::prelude::String")

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2821)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, index: I) -> &mut <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/nodes.rs.html#1014)

### impl [IntoAttributeValue](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/nodes/trait.IntoAttributeValue.html "trait dioxus_core::nodes::IntoAttributeValue") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/nodes.rs.html#1015)

#### fn [into\_value](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/nodes/trait.IntoAttributeValue.html#tymethod.into_value)(self) -> [AttributeValue](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/nodes/enum.AttributeValue.html "enum dioxus_core::nodes::AttributeValue")

Convert into an attribute value

[Source](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/src/tungstenite/client.rs.html#205)

### impl [IntoClientRequest](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/tungstenite/client/trait.IntoClientRequest.html "trait tungstenite::client::IntoClientRequest") for &[String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/src/tungstenite/client.rs.html#206)

#### fn [into\_client\_request](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/tungstenite/client/trait.IntoClientRequest.html#tymethod.into_client_request)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Request](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/request/struct.Request.html "struct http::request::Request")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>, [Error](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/tungstenite/error/enum.Error.html "enum tungstenite::error::Error")\>

Convert into a `Request` that can be used for a client connection.

[Source](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/src/tungstenite/client.rs.html#211)

### impl [IntoClientRequest](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/tungstenite/client/trait.IntoClientRequest.html "trait tungstenite::client::IntoClientRequest") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/src/tungstenite/client.rs.html#212)

#### fn [into\_client\_request](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/tungstenite/client/trait.IntoClientRequest.html#tymethod.into_client_request)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Request](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/request/struct.Request.html "struct http::request::Request")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>, [Error](https://docs.rs/tungstenite/0.28.0/x86_64-unknown-linux-gnu/tungstenite/error/enum.Error.html "enum tungstenite::error::Error")\>

Convert into a `Request` that can be used for a client connection.

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/value.rs.html#654-656)

### impl<'de, E> [IntoDeserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.IntoDeserializer.html "trait serde_core::de::IntoDeserializer")<'de, E> for [String](struct.String.html "struct bevy::prelude::String")

where E: [Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Error.html "trait serde_core::de::Error"),

Available on **crate features `alloc` or `std`** only.

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/value.rs.html#658)

#### type [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.IntoDeserializer.html#associatedtype.Deserializer) = [StringDeserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/value/struct.StringDeserializer.html "struct serde_core::de::value::StringDeserializer")<E>

The type of the deserializer being converted into.

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/value.rs.html#660)

#### fn [into\_deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.IntoDeserializer.html#tymethod.into_deserializer)(self) -> [StringDeserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/value/struct.StringDeserializer.html "struct serde_core::de::value::StringDeserializer")<E>

Convert this value into a deserializer.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/nodes.rs.html#898)

### impl [IntoDynNode](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/nodes/trait.IntoDynNode.html "trait dioxus_core::nodes::IntoDynNode") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/nodes.rs.html#899)

#### fn [into\_dyn\_node](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/nodes/trait.IntoDynNode.html#tymethod.into_dyn_node)(self) -> [DynamicNode](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/nodes/enum.DynamicNode.html "enum dioxus_core::nodes::DynamicNode")

Consume this item and produce a DynamicNode

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [String](struct.String.html "struct bevy::prelude::String"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#350)

### impl [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#350)

#### fn [cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1034-1036)

#### fn [max](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)(self, other: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1073-1075)

#### fn [min](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)(self, other: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1099-1101)

#### fn [clamp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)(self, min: Self, max: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#350)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#350)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2699)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2699)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2699)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: &&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/uri/authority.rs.html#265)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[Authority](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/uri/authority/struct.Authority.html "struct http::uri::authority::Authority")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/uri/authority.rs.html#266)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Authority](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/uri/authority/struct.Authority.html "struct http::uri::authority::Authority")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#665)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[ByteStr](https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html "struct core::bstr::ByteStr")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#665)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[ByteStr](https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html "struct core::bstr::ByteStr")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#525)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[ByteString](https://doc.rust-lang.org/nightly/alloc/bstr/struct.ByteString.html "struct alloc::bstr::ByteString")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#525)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[ByteString](https://doc.rust-lang.org/nightly/alloc/bstr/struct.ByteString.html "struct alloc::bstr::ByteString")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/bytes.rs.html#885)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[Bytes](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/bytes.rs.html#886)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Bytes](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/bytes_mut.rs.html#1665)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[BytesMut](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/bytes_mut/struct.BytesMut.html "struct bytes::bytes_mut::BytesMut")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/bytes_mut.rs.html#1666)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[BytesMut](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/bytes_mut/struct.BytesMut.html "struct bytes::bytes_mut::BytesMut")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2705)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2705)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2705)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: &[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/header/value.rs.html#686)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[HeaderValue](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/header/value/struct.HeaderValue.html "struct http::header::value::HeaderValue")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/header/value.rs.html#688)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[HeaderValue](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/header/value/struct.HeaderValue.html "struct http::header::value::HeaderValue")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.91.0 · [Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#3787)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#3789)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/uri/path.rs.html#342)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[PathAndQuery](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/uri/path/struct.PathAndQuery.html "struct http::uri::path::PathAndQuery")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/uri/path.rs.html#344)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[PathAndQuery](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/uri/path/struct.PathAndQuery.html "struct http::uri::path::PathAndQuery")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.91.0 · [Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#2277)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#2279)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#225)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#226)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#237)

### impl<'a> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")\> for &'a [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#238)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.91.0 · [Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#3779)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[String](struct.String.html "struct bevy::prelude::String")\> for [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")

[Source](https://doc.rust-lang.org/nightly/src/std/path.rs.html#3781)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2698)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[String](struct.String.html "struct bevy::prelude::String")\> for [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2698)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2698)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2699)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[String](struct.String.html "struct bevy::prelude::String")\> for &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2699)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2699)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2705)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[String](struct.String.html "struct bevy::prelude::String")\> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2705)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2705)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/tinystr/0.8.3/x86_64-unknown-linux-gnu/src/tinystr/ascii.rs.html#876)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[String](struct.String.html "struct bevy::prelude::String")\> for [TinyAsciiStr](https://docs.rs/tinystr/0.8.3/x86_64-unknown-linux-gnu/tinystr/ascii/struct.TinyAsciiStr.html "struct tinystr::ascii::TinyAsciiStr")<N>

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/tinystr/0.8.3/x86_64-unknown-linux-gnu/src/tinystr/ascii.rs.html#877)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/tinystr/0.8.3/x86_64-unknown-linux-gnu/src/tinystr/ascii.rs.html#883)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[TinyAsciiStr](https://docs.rs/tinystr/0.8.3/x86_64-unknown-linux-gnu/tinystr/ascii/struct.TinyAsciiStr.html "struct tinystr::ascii::TinyAsciiStr")<N>> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/tinystr/0.8.3/x86_64-unknown-linux-gnu/src/tinystr/ascii.rs.html#884)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[TinyAsciiStr](https://docs.rs/tinystr/0.8.3/x86_64-unknown-linux-gnu/tinystr/ascii/struct.TinyAsciiStr.html "struct tinystr::ascii::TinyAsciiStr")<N>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/src/serde_json/value/partial_eq.rs.html#61)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[Value](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html "enum serde_json::value::Value")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/src/serde_json/value/partial_eq.rs.html#62)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Value](https://docs.rs/serde_json/1.0.150/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html "enum serde_json::value::Value")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2698)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2698)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2698)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#350)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#350)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[String](struct.String.html "struct bevy::prelude::String")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1410)

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1428)

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1446)

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1464)

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/uri/authority.rs.html#329)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<[Authority](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/uri/authority/struct.Authority.html "struct http::uri::authority::Authority")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/uri/authority.rs.html#330)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[Authority](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/uri/authority/struct.Authority.html "struct http::uri::authority::Authority")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1410)

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1428)

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1446)

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1464)

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/bytes.rs.html#891)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<[Bytes](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/bytes.rs.html#892)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[Bytes](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1410)

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1428)

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1446)

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1464)

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/bytes_mut.rs.html#1671)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<[BytesMut](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/bytes_mut/struct.BytesMut.html "struct bytes::bytes_mut::BytesMut")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/src/bytes/bytes_mut.rs.html#1672)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[BytesMut](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/bytes_mut/struct.BytesMut.html "struct bytes::bytes_mut::BytesMut")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1410)

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1428)

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1446)

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1464)

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/header/value.rs.html#693)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<[HeaderValue](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/header/value/struct.HeaderValue.html "struct http::header::value::HeaderValue")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/header/value.rs.html#695)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[HeaderValue](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/header/value/struct.HeaderValue.html "struct http::header::value::HeaderValue")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1410)

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1428)

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1446)

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1464)

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/uri/path.rs.html#391)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd")<[PathAndQuery](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/uri/path/struct.PathAndQuery.html "struct http::uri::path::PathAndQuery")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/src/http/uri/path.rs.html#393)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[PathAndQuery](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/uri/path/struct.PathAndQuery.html "struct http::uri::path::PathAndQuery")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1410)

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1428)

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1446)

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1464)

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [get\_represented\_type\_info](trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [to\_dynamic](trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [try\_apply](trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [reflect\_kind](trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [reflect\_ref](trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [reflect\_mut](trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [reflect\_owned](trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[String](struct.String.html "struct bevy::prelude::String")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [try\_into\_reflect](trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](struct.Box.html "struct bevy::prelude::Box")<[String](struct.String.html "struct bevy::prelude::String")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [try\_as\_reflect](trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [try\_as\_reflect\_mut](trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [into\_partial\_reflect](trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[String](struct.String.html "struct bevy::prelude::String")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [as\_partial\_reflect](trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [as\_partial\_reflect\_mut](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#10)

#### fn [reflect\_hash](trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#11)

#### fn [reflect\_partial\_eq](trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#12)

#### fn [reflect\_partial\_cmp](trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#9)

#### fn [debug](trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#8)

#### fn [reflect\_clone](trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](trait.PartialReflect.html#method.is_dynamic)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2626)

### impl<'b> [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern") for &'b [String](struct.String.html "struct bevy::prelude::String")

A convenience impl that delegates to the impl for `&str`.

#### Examples

```rust
assert_eq!(String::from("Hello world").find("world"), Some(6));
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2627)

#### type [Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher)<'a> = <&'b [str](https://doc.rust-lang.org/nightly/std/primitive.str.html) as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>

🔬This is a nightly-only experimental API. (`pattern`)

Associated searcher for this pattern

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2629)

#### fn [into\_searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#tymethod.into_searcher)(self, haystack: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> <&'b [str](https://doc.rust-lang.org/nightly/std/primitive.str.html) as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'\_>

🔬This is a nightly-only experimental API. (`pattern`)

Constructs the associated searcher from `self` and the `haystack` to search in.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2634)

#### fn [is\_contained\_in](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#method.is_contained_in)(self, haystack: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

🔬This is a nightly-only experimental API. (`pattern`)

Checks whether the pattern matches anywhere in the haystack

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2639)

#### fn [is\_prefix\_of](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#method.is_prefix_of)(self, haystack: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

🔬This is a nightly-only experimental API. (`pattern`)

Checks whether the pattern matches at the front of the haystack

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2644)

#### fn [strip\_prefix\_of](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#method.strip_prefix_of)(self, haystack: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

🔬This is a nightly-only experimental API. (`pattern`)

Removes the pattern from the front of haystack, if it matches.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2649-2651)

#### fn [is\_suffix\_of](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#method.is_suffix_of)<'a>(self, haystack: &'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where <&'b [String](struct.String.html "struct bevy::prelude::String") as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

🔬This is a nightly-only experimental API. (`pattern`)

Checks whether the pattern matches at the back of the haystack

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2657-2659)

#### fn [strip\_suffix\_of](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#method.strip_suffix_of)<'a>(self, haystack: &'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where <&'b [String](struct.String.html "struct bevy::prelude::String") as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

🔬This is a nightly-only experimental API. (`pattern`)

Removes the pattern from the back of haystack, if it matches.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2665)

#### fn [as\_utf8\_pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#method.as_utf8_pattern)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Utf8Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/enum.Utf8Pattern.html "enum core::str::pattern::Utf8Pattern")<'\_>>

🔬This is a nightly-only experimental API. (`pattern`)

Returns the pattern as UTF-8 if possible.

[Source](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/src/phf_shared/lib.rs.html#192)

### impl [PhfBorrow](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/phf_shared/trait.PhfBorrow.html "trait phf_shared::PhfBorrow")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `std`** only.

[Source](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/src/phf_shared/lib.rs.html#193)

#### fn [borrow](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/phf_shared/trait.PhfBorrow.html#tymethod.borrow)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Convert a reference to `self` to a reference to the borrowed type.

[Source](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/src/phf_shared/lib.rs.html#209)

### impl [PhfHash](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/phf_shared/trait.PhfHash.html "trait phf_shared::PhfHash") for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `std`** only.

[Source](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/src/phf_shared/lib.rs.html#211)

#### fn [phf\_hash](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/phf_shared/trait.PhfHash.html#tymethod.phf_hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds the value into the state given, updating the hasher as necessary.

[Source](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/src/phf_shared/lib.rs.html#72-74)

#### fn [phf\_hash\_slice](https://docs.rs/phf_shared/0.13.1/x86_64-unknown-linux-gnu/phf_shared/trait.PhfHash.html#method.phf_hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the state provided.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [into\_any](trait.Reflect.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[String](struct.String.html "struct bevy::prelude::String")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [as\_any](trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [as\_any\_mut](trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [into\_reflect](trait.Reflect.html#tymethod.into_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[String](struct.String.html "struct bevy::prelude::String")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [as\_reflect](trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [as\_reflect\_mut](trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [set](trait.Reflect.html#tymethod.set)(&mut self, value: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/src/regex/regex/string.rs.html#2510)

### impl<'a> [Replacer](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html "trait regex::regex::string::Replacer") for &'a [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/src/regex/regex/string.rs.html#2511)

#### fn [replace\_append](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#tymethod.replace_append)(&mut self, caps: &[Captures](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/struct.Captures.html "struct regex::regex::string::Captures")<'\_>, dst: &mut [String](struct.String.html "struct bevy::prelude::String"))

Appends possibly empty data to `dst` to replace the current match. [Read more](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#tymethod.replace_append)

[Source](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/src/regex/regex/string.rs.html#2515)

#### fn [no\_expansion](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#method.no_expansion)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>

Return a fixed unchanging replacement string. [Read more](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#method.no_expansion)

[Source](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/src/regex/regex/string.rs.html#2495)

#### fn [by\_ref](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#method.by_ref)<'r>(&'r mut self) -> [ReplacerRef](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/struct.ReplacerRef.html "struct regex::regex::string::ReplacerRef")<'r, Self>

Returns a type that implements `Replacer`, but that borrows and wraps this `Replacer`. [Read more](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#method.by_ref)

[Source](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/src/regex/regex/string.rs.html#2520)

### impl [Replacer](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html "trait regex::regex::string::Replacer") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/src/regex/regex/string.rs.html#2521)

#### fn [replace\_append](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#tymethod.replace_append)(&mut self, caps: &[Captures](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/struct.Captures.html "struct regex::regex::string::Captures")<'\_>, dst: &mut [String](struct.String.html "struct bevy::prelude::String"))

Appends possibly empty data to `dst` to replace the current match. [Read more](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#tymethod.replace_append)

[Source](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/src/regex/regex/string.rs.html#2525)

#### fn [no\_expansion](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#method.no_expansion)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>

Return a fixed unchanging replacement string. [Read more](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#method.no_expansion)

[Source](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/src/regex/regex/string.rs.html#2495)

#### fn [by\_ref](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#method.by_ref)<'r>(&'r mut self) -> [ReplacerRef](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/struct.ReplacerRef.html "struct regex::regex::string::ReplacerRef")<'r, Self>

Returns a type that implements `Replacer`, but that borrows and wraps this `Replacer`. [Read more](https://docs.rs/regex/1.12.3/x86_64-unknown-linux-gnu/regex/regex/string/trait.Replacer.html#method.by_ref)

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/ser/impls.rs.html#52)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate features `alloc` or `std`** only.

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/ser/impls.rs.html#54-56)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/stable_deref_trait/1.2.1/x86_64-unknown-linux-gnu/src/stable_deref_trait/lib.rs.html#163)

### impl [StableDeref](https://docs.rs/stable_deref_trait/1.2.1/x86_64-unknown-linux-gnu/stable_deref_trait/trait.StableDeref.html "trait stable_deref_trait::StableDeref") for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/base64/0.13.1/x86_64-unknown-linux-gnu/src/base64/write/encoder_string_writer.rs.html#106)

### impl [StrConsumer](https://docs.rs/base64/0.13.1/x86_64-unknown-linux-gnu/base64/write/encoder_string_writer/trait.StrConsumer.html "trait base64::write::encoder_string_writer::StrConsumer") for [String](struct.String.html "struct bevy::prelude::String")

Pushes the str onto the end of the String

[Source](https://docs.rs/base64/0.13.1/x86_64-unknown-linux-gnu/src/base64/write/encoder_string_writer.rs.html#107)

#### fn [consume](https://docs.rs/base64/0.13.1/x86_64-unknown-linux-gnu/base64/write/encoder_string_writer/trait.StrConsumer.html#tymethod.consume)(&mut self, buf: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Consume the base64 encoded data in `buf`

[Source](https://docs.rs/toml_parser/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_parser/decoder/mod.rs.html#105)

### impl<'s> [StringBuilder](https://docs.rs/toml_parser/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/toml_parser/decoder/trait.StringBuilder.html "trait toml_parser::decoder::StringBuilder")<'s> for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/toml_parser/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_parser/decoder/mod.rs.html#106)

#### fn [clear](https://docs.rs/toml_parser/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/toml_parser/decoder/trait.StringBuilder.html#tymethod.clear)(&mut self)

[Source](https://docs.rs/toml_parser/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_parser/decoder/mod.rs.html#109)

#### fn [push\_str](https://docs.rs/toml_parser/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/toml_parser/decoder/trait.StringBuilder.html#tymethod.push_str)(&mut self, append: &'s [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/toml_parser/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_parser/decoder/mod.rs.html#113)

#### fn [push\_char](https://docs.rs/toml_parser/1.1.2+spec-1.1.0/x86_64-unknown-linux-gnu/toml_parser/decoder/trait.StringBuilder.html#tymethod.push_char)(&mut self, append: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#350)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [String](struct.String.html "struct bevy::prelude::String")

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/std/net/socket_addr.rs.html#250)

### impl [ToSocketAddrs](https://doc.rust-lang.org/nightly/std/net/socket_addr/trait.ToSocketAddrs.html "trait std::net::socket_addr::ToSocketAddrs") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/std/net/socket_addr.rs.html#251)

#### type [Iter](https://doc.rust-lang.org/nightly/std/net/socket_addr/trait.ToSocketAddrs.html#associatedtype.Iter) = [IntoIter](vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<[SocketAddr](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")\>

Returned iterator over socket addresses which this type may correspond to.

[Source](https://doc.rust-lang.org/nightly/src/std/net/socket_addr.rs.html#252)

#### fn [to\_socket\_addrs](https://doc.rust-lang.org/nightly/std/net/socket_addr/trait.ToSocketAddrs.html#tymethod.to_socket_addrs)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IntoIter](vec/struct.IntoIter.html "struct bevy::prelude::vec::IntoIter")<[SocketAddr](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")\>, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Converts this object to an iterator of resolved [`SocketAddr`](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")s. [Read more](https://doc.rust-lang.org/nightly/std/net/socket_addr/trait.ToSocketAddrs.html#tymethod.to_socket_addrs)

[Source](https://doc.rust-lang.org/nightly/src/proc_macro/to_tokens.rs.html#292)

### impl [ToTokens](https://doc.rust-lang.org/nightly/proc_macro/to_tokens/trait.ToTokens.html "trait proc_macro::to_tokens::ToTokens") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/proc_macro/to_tokens.rs.html#293)

#### fn [to\_tokens](https://doc.rust-lang.org/nightly/proc_macro/to_tokens/trait.ToTokens.html#tymethod.to_tokens)(&self, tokens: &mut [TokenStream](https://doc.rust-lang.org/nightly/proc_macro/struct.TokenStream.html "struct proc_macro::TokenStream"))

🔬This is a nightly-only experimental API. (`proc_macro_totokens`)

Write `self` to the given `TokenStream`. [Read more](https://doc.rust-lang.org/nightly/proc_macro/to_tokens/trait.ToTokens.html#tymethod.to_tokens)

[Source](https://doc.rust-lang.org/nightly/src/proc_macro/to_tokens.rs.html#57)

#### fn [to\_token\_stream](https://doc.rust-lang.org/nightly/proc_macro/to_tokens/trait.ToTokens.html#method.to_token_stream)(&self) -> [TokenStream](https://doc.rust-lang.org/nightly/proc_macro/struct.TokenStream.html "struct proc_macro::TokenStream")

🔬This is a nightly-only experimental API. (`proc_macro_totokens`)

Convert `self` directly into a `TokenStream` object. [Read more](https://doc.rust-lang.org/nightly/proc_macro/to_tokens/trait.ToTokens.html#method.to_token_stream)

[Source](https://doc.rust-lang.org/nightly/src/proc_macro/to_tokens.rs.html#67-69)

#### fn [into\_token\_stream](https://doc.rust-lang.org/nightly/proc_macro/to_tokens/trait.ToTokens.html#method.into_token_stream)(self) -> [TokenStream](https://doc.rust-lang.org/nightly/proc_macro/struct.TokenStream.html "struct proc_macro::TokenStream")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`proc_macro_totokens`)

Convert `self` directly into a `TokenStream` object. [Read more](https://doc.rust-lang.org/nightly/proc_macro/to_tokens/trait.ToTokens.html#method.into_token_stream)

[Source](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/src/quote/to_tokens.rs.html#128)

### impl [ToTokens](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/to_tokens/trait.ToTokens.html "trait quote::to_tokens::ToTokens") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/src/quote/to_tokens.rs.html#129)

#### fn [to\_tokens](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/to_tokens/trait.ToTokens.html#tymethod.to_tokens)(&self, tokens: &mut [TokenStream](https://docs.rs/proc-macro2/1.0.106/x86_64-unknown-linux-gnu/proc_macro2/struct.TokenStream.html "struct proc_macro2::TokenStream"))

Write `self` to the given `TokenStream`. [Read more](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/to_tokens/trait.ToTokens.html#tymethod.to_tokens)

[Source](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/src/quote/to_tokens.rs.html#60)

#### fn [to\_token\_stream](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/to_tokens/trait.ToTokens.html#method.to_token_stream)(&self) -> [TokenStream](https://docs.rs/proc-macro2/1.0.106/x86_64-unknown-linux-gnu/proc_macro2/struct.TokenStream.html "struct proc_macro2::TokenStream")

Convert `self` directly into a `TokenStream` object. [Read more](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/to_tokens/trait.ToTokens.html#method.to_token_stream)

[Source](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/src/quote/to_tokens.rs.html#70-72)

#### fn [into\_token\_stream](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/to_tokens/trait.ToTokens.html#method.into_token_stream)(self) -> [TokenStream](https://docs.rs/proc-macro2/1.0.106/x86_64-unknown-linux-gnu/proc_macro2/struct.TokenStream.html "struct proc_macro2::TokenStream")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Convert `self` directly into a `TokenStream` object. [Read more](https://docs.rs/quote/1.0.45/x86_64-unknown-linux-gnu/quote/to_tokens/trait.ToTokens.html#method.into_token_stream)

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#671)

### impl<'a> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&'a [ByteStr](https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html "struct core::bstr::ByteStr")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#672)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Utf8Error](https://doc.rust-lang.org/nightly/core/str/error/struct.Utf8Error.html "struct core::str::error::Utf8Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#675)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( s: &'a [ByteStr](https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html "struct core::bstr::ByteStr"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), <[String](struct.String.html "struct bevy::prelude::String") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&'a [ByteStr](https://doc.rust-lang.org/nightly/core/bstr/struct.ByteStr.html "struct core::bstr::ByteStr")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/from_value.rs.html#127)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&[Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'\_>> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/from_value.rs.html#128)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/error/enum.Error.html "enum zvariant::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/from_value.rs.html#130)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: &[Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), <[String](struct.String.html "struct bevy::prelude::String") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&[Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'\_>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#567)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ByteString](https://doc.rust-lang.org/nightly/alloc/bstr/struct.ByteString.html "struct alloc::bstr::ByteString")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#568)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [FromUtf8Error](https://doc.rust-lang.org/nightly/alloc/string/struct.FromUtf8Error.html "struct alloc::string::FromUtf8Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/alloc/bstr.rs.html#571)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( s: [ByteString](https://doc.rust-lang.org/nightly/alloc/bstr/struct.ByteString.html "struct alloc::bstr::ByteString"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), <[String](struct.String.html "struct bevy::prelude::String") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ByteString](https://doc.rust-lang.org/nightly/alloc/bstr/struct.ByteString.html "struct alloc::bstr::ByteString")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

1.85.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/ffi/c_str.rs.html#845)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[CString](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html "struct alloc::ffi::c_str::CString")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/ffi/c_str.rs.html#852)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [CString](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html "struct alloc::ffi::c_str::CString"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), <[String](struct.String.html "struct bevy::prelude::String") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[CString](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html "struct alloc::ffi::c_str::CString")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Converts a [`CString`](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html "struct alloc::ffi::c_str::CString") into a [`String`](struct.String.html "struct bevy::prelude::String") if it contains valid UTF-8 data.

This method is equivalent to [`CString::into_string`](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.CString.html#method.into_string "method alloc::ffi::c_str::CString::into_string").

[Source](https://doc.rust-lang.org/nightly/src/alloc/ffi/c_str.rs.html#846)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [IntoStringError](https://doc.rust-lang.org/nightly/alloc/ffi/c_str/struct.IntoStringError.html "struct alloc::ffi::c_str::IntoStringError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/owned_value.rs.html#70)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[OwnedValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/owned_value/struct.OwnedValue.html "struct zvariant::owned_value::OwnedValue")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/owned_value.rs.html#70)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/error/enum.Error.html "enum zvariant::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/owned_value.rs.html#70)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [OwnedValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/owned_value/struct.OwnedValue.html "struct zvariant::owned_value::OwnedValue"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), <[String](struct.String.html "struct bevy::prelude::String") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[OwnedValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/owned_value/struct.OwnedValue.html "struct zvariant::owned_value::OwnedValue")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/parser.rs.html#42)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [Uuid](../asset/uuid/struct.Uuid.html "struct bevy::asset::uuid::Uuid")

Available on **crate feature `std`** only.

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/parser.rs.html#43)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](../asset/uuid/struct.Error.html "struct bevy::asset::uuid::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/parser.rs.html#45)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(uuid\_str: [String](struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Uuid](../asset/uuid/struct.Uuid.html "struct bevy::asset::uuid::Uuid"), <[Uuid](../asset/uuid/struct.Uuid.html "struct bevy::asset::uuid::Uuid") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/object_path.rs.html#151)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [ObjectPath](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/object_path/struct.ObjectPath.html "struct zvariant::object_path::ObjectPath")<'\_>

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/object_path.rs.html#152)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/error/enum.Error.html "enum zvariant::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/object_path.rs.html#154)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ObjectPath](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/object_path/struct.ObjectPath.html "struct zvariant::object_path::ObjectPath")<'\_>, [Error](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/error/enum.Error.html "enum zvariant::error::Error")\>

Performs the conversion.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/bus_name.rs.html#260)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [BusName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/bus_name/enum.BusName.html "enum zbus_names::bus_name::BusName")<'\_>

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/bus_name.rs.html#261)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/bus_name.rs.html#263)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[BusName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/bus_name/enum.BusName.html "enum zbus_names::bus_name::BusName")<'\_>, [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")\>

Performs the conversion.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/unique_name.rs.html#37-41)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [UniqueName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/unique_name/struct.UniqueName.html "struct zbus_names::unique_name::UniqueName")<'\_>

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/unique_name.rs.html#37-41)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/unique_name.rs.html#37-41)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UniqueName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/unique_name/struct.UniqueName.html "struct zbus_names::unique_name::UniqueName")<'\_>, [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")\>

Performs the conversion.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/well_known_name.rs.html#38-42)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [WellKnownName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/well_known_name/struct.WellKnownName.html "struct zbus_names::well_known_name::WellKnownName")<'\_>

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/well_known_name.rs.html#38-42)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/well_known_name.rs.html#38-42)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[WellKnownName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/well_known_name/struct.WellKnownName.html "struct zbus_names::well_known_name::WellKnownName")<'\_>, [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")\>

Performs the conversion.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/interface_name.rs.html#40-44)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [InterfaceName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/interface_name/struct.InterfaceName.html "struct zbus_names::interface_name::InterfaceName")<'\_>

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/interface_name.rs.html#40-44)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/interface_name.rs.html#40-44)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[InterfaceName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/interface_name/struct.InterfaceName.html "struct zbus_names::interface_name::InterfaceName")<'\_>, [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")\>

Performs the conversion.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/member_name.rs.html#38-42)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [MemberName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/member_name/struct.MemberName.html "struct zbus_names::member_name::MemberName")<'\_>

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/member_name.rs.html#38-42)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/member_name.rs.html#38-42)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[MemberName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/member_name/struct.MemberName.html "struct zbus_names::member_name::MemberName")<'\_>, [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")\>

Performs the conversion.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/property_name.rs.html#33-37)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [PropertyName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/property_name/struct.PropertyName.html "struct zbus_names::property_name::PropertyName")<'\_>

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/property_name.rs.html#33-37)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/property_name.rs.html#33-37)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[PropertyName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/property_name/struct.PropertyName.html "struct zbus_names::property_name::PropertyName")<'\_>, [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")\>

Performs the conversion.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/error_name.rs.html#42-46)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [ErrorName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error_name/struct.ErrorName.html "struct zbus_names::error_name::ErrorName")<'\_>

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/error_name.rs.html#42-46)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/src/zbus_names/error_name.rs.html#42-46)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [String](struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ErrorName](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error_name/struct.ErrorName.html "struct zbus_names::error_name::ErrorName")<'\_>, [Error](https://docs.rs/zbus_names/4.3.1/x86_64-unknown-linux-gnu/zbus_names/error/enum.Error.html "enum zbus_names::error::Error")\>

Performs the conversion.

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/guid.rs.html#86)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\> for [Guid](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/guid/struct.Guid.html "struct zbus::guid::Guid")<'\_>

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/guid.rs.html#87)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/error/enum.Error.html "enum zbus::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/src/zbus/guid.rs.html#89)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [String](struct.String.html "struct bevy::prelude::String"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Guid](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/guid/struct.Guid.html "struct zbus::guid::Guid")<'\_>, <[Guid](https://docs.rs/zbus/5.13.2/x86_64-unknown-linux-gnu/zbus/guid/struct.Guid.html "struct zbus::guid::Guid")<'\_> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[String](struct.String.html "struct bevy::prelude::String")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/from_value.rs.html#82)

### impl<'a> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'a>> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/from_value.rs.html#82)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Error](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/error/enum.Error.html "enum zvariant::error::Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/from_value.rs.html#82)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'a>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), <[String](struct.String.html "struct bevy::prelude::String") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/value/enum.Value.html "enum zvariant::value::Value")<'a>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

1.87.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3332)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3344)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( bytes: [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[String](struct.String.html "struct bevy::prelude::String"), <[String](struct.String.html "struct bevy::prelude::String") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec](struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Converts the given [`Vec<u8>`](struct.Vec.html "struct bevy::prelude::Vec") into a [`String`](struct.String.html "struct bevy::prelude::String") if it contains valid UTF-8 data.

##### Examples

```rust
let s1 = b"hello world".to_vec();
let v1 = String::try_from(s1).unwrap();
assert_eq!(v1, "hello world");
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3333)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [FromUtf8Error](https://doc.rust-lang.org/nightly/alloc/string/struct.FromUtf8Error.html "struct alloc::string::FromUtf8Error")

The type returned in the event of a conversion error.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/basic.rs.html#185)

### impl [Type](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html "trait zvariant::type::Type") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/basic.rs.html#185)

#### const [SIGNATURE](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html#associatedconstant.SIGNATURE): &'static [Signature](https://docs.rs/zvariant_utils/3.3.0/x86_64-unknown-linux-gnu/zvariant_utils/signature/enum.Signature.html "enum zvariant_utils::signature::Signature")

The signature for the implementing type, in parsed format. [Read more](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html#associatedconstant.SIGNATURE)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [type\_path](trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [short\_type\_path](trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [type\_ident](trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [crate\_name](trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [module\_path](trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/src/gltf_json/validation.rs.html#237)

### impl [Validate](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/gltf_json/validation/trait.Validate.html "trait gltf_json::validation::Validate") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/src/gltf_json/validation.rs.html#10-13)

#### fn [validate](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/gltf_json/validation/trait.Validate.html#method.validate)<P, R>(&self, \_root: &[Root](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/gltf_json/root/struct.Root.html "struct gltf_json::root::Root"), \_path: P, \_report: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where P: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")() -> [Path](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/gltf_json/path/struct.Path.html "struct gltf_json::path::Path"), R: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")() -> [Path](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/gltf_json/path/struct.Path.html "struct gltf_json::path::Path"), [Error](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/gltf_json/validation/enum.Error.html "enum gltf_json::validation::Error")),

Validates the invariants required for the library to function safely.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#674)

### impl [Value](../log/tracing/trait.Value.html "trait bevy::log::tracing::Value") for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#675)

#### fn [record](../log/tracing/trait.Value.html#tymethod.record)(&self, key: &[Field](../log/tracing/field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](../log/tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

Visits this value with the given `Visitor`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3351)

### impl [Write](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html "trait core::fmt::Write") for [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3353)

#### fn [write\_str](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#tymethod.write_str)(&mut self, s: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Writes a string slice into this writer, returning whether the write succeeded. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#tymethod.write_str)

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#3359)

#### fn [write\_char](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_char)(&mut self, c: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Writes a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") into this writer, returning whether the write succeeded. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_char)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/fmt/mod.rs.html#214)

#### fn [write\_fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_fmt)(&mut self, args: [Arguments](https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html "struct core::fmt::Arguments")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Glue for usage of the [`write!`](https://doc.rust-lang.org/nightly/core/macro.write.html "macro core::write") macro with implementors of this trait. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_fmt)

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/key.rs.html#38)

### impl [WriteTomlKey](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/key/trait.WriteTomlKey.html "trait toml_writer::key::WriteTomlKey") for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/key.rs.html#39)

#### fn [write\_toml\_key](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/key/trait.WriteTomlKey.html#tymethod.write_toml_key)<W>(&self, writer: [&mut W](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

where W: [TomlWrite](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html "trait toml_writer::write::TomlWrite") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/value.rs.html#151)

### impl [WriteTomlValue](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/value/trait.WriteTomlValue.html "trait toml_writer::value::WriteTomlValue") for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/value.rs.html#152)

#### fn [write\_toml\_value](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/value/trait.WriteTomlValue.html#tymethod.write_toml_value)<W>(&self, writer: [&mut W](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

where W: [TomlWrite](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html "trait toml_writer::write::TomlWrite") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/src/writeable/impls.rs.html#116)

### impl [Writeable](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html "trait writeable::Writeable") for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/src/writeable/impls.rs.html#118)

#### fn [write\_to](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.write_to)<W>(&self, sink: [&mut W](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

where W: [Write](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html "trait core::fmt::Write") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Writes a string to the given sink. Errors from the sink are bubbled up. The default implementation delegates to `write_to_parts`, and discards any `Part` annotations.

[Source](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/src/writeable/impls.rs.html#123)

#### fn [writeable\_length\_hint](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.writeable_length_hint)(&self) -> [LengthHint](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/struct.LengthHint.html "struct writeable::LengthHint")

Returns a hint for the number of UTF-8 bytes that will be written to the sink. [Read more](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.writeable_length_hint)

[Source](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/src/writeable/impls.rs.html#128)

#### fn [writeable\_borrow](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.writeable_borrow)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns a `&str` that matches the output of `write_to`, if possible. [Read more](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.writeable_borrow)

[Source](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/src/writeable/lib.rs.html#280)

#### fn [write\_to\_parts](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.write_to_parts)<S>(&self, sink: [&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

where S: [PartsWrite](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.PartsWrite.html "trait writeable::PartsWrite") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Write bytes and `Part` annotations to the given sink. Errors from the sink are bubbled up. The default implementation delegates to `write_to`, and doesn’t produce any `Part` annotations.

[Source](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/src/writeable/lib.rs.html#353)

#### fn [write\_to\_string](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.write_to_string)(&self) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Creates a new string with the data from this `Writeable`. [Read more](https://docs.rs/writeable/0.6.3/x86_64-unknown-linux-gnu/writeable/trait.Writeable.html#method.write_to_string)

[Source](https://docs.rs/zerofrom/0.1.8/x86_64-unknown-linux-gnu/src/zerofrom/zero_from.rs.html#76)

### impl<'zf> [ZeroFrom](https://docs.rs/zerofrom/0.1.8/x86_64-unknown-linux-gnu/zerofrom/zero_from/trait.ZeroFrom.html "trait zerofrom::zero_from::ZeroFrom")<'zf, [String](struct.String.html "struct bevy::prelude::String")\> for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'zf, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerofrom/0.1.8/x86_64-unknown-linux-gnu/src/zerofrom/zero_from.rs.html#78)

#### fn [zero\_from](https://docs.rs/zerofrom/0.1.8/x86_64-unknown-linux-gnu/zerofrom/zero_from/trait.ZeroFrom.html#tymethod.zero_from)(other: &'zf [String](struct.String.html "struct bevy::prelude::String")) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'zf, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Clone the other `C` into a struct that may retain references into `C`.

[Source](https://docs.rs/zerofrom/0.1.8/x86_64-unknown-linux-gnu/src/zerofrom/zero_from.rs.html#91)

### impl<'zf> [ZeroFrom](https://docs.rs/zerofrom/0.1.8/x86_64-unknown-linux-gnu/zerofrom/zero_from/trait.ZeroFrom.html "trait zerofrom::zero_from::ZeroFrom")<'zf, [String](struct.String.html "struct bevy::prelude::String")\> for &'zf [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zerofrom/0.1.8/x86_64-unknown-linux-gnu/src/zerofrom/zero_from.rs.html#93)

#### fn [zero\_from](https://docs.rs/zerofrom/0.1.8/x86_64-unknown-linux-gnu/zerofrom/zero_from/trait.ZeroFrom.html#tymethod.zero_from)(other: &'zf [String](struct.String.html "struct bevy::prelude::String")) -> &'zf [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Clone the other `C` into a struct that may retain references into `C`.

[Source](https://docs.rs/zeroize/1.8.2/x86_64-unknown-linux-gnu/src/zeroize/lib.rs.html#589)

### impl [Zeroize](https://docs.rs/zeroize/1.8.2/x86_64-unknown-linux-gnu/zeroize/trait.Zeroize.html "trait zeroize::Zeroize") for [String](struct.String.html "struct bevy::prelude::String")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/zeroize/1.8.2/x86_64-unknown-linux-gnu/src/zeroize/lib.rs.html#590)

#### fn [zeroize](https://docs.rs/zeroize/1.8.2/x86_64-unknown-linux-gnu/zeroize/trait.Zeroize.html#tymethod.zeroize)(&mut self)

Zero out this object from memory using Rust intrinsics which ensure the zeroization operation is not “optimized away” by the compiler.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [String](struct.String.html "struct bevy::prelude::String")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [String](struct.String.html "struct bevy::prelude::String")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [String](struct.String.html "struct bevy::prelude::String")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [String](struct.String.html "struct bevy::prelude::String")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [String](struct.String.html "struct bevy::prelude::String")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [String](struct.String.html "struct bevy::prelude::String")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [String](struct.String.html "struct bevy::prelude::String")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/src/bitvec/view.rs.html#226-229)

### impl<A, T> [AsBits](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/view/trait.AsBits.html "trait bitvec::view::AsBits")<T> for A

where A: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>, T: [BitStore](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/store/trait.BitStore.html "trait bitvec::store::BitStore"),

[Source](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/src/bitvec/view.rs.html#232-233)

#### fn [as\_bits](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/view/trait.AsBits.html#tymethod.as_bits)<O>(&self) -> &[BitSlice](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/slice/struct.BitSlice.html "struct bitvec::slice::BitSlice")<T, O> [ⓘ](#)

where O: [BitOrder](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/order/trait.BitOrder.html "trait bitvec::order::BitOrder"),

Views `self` as an immutable bit-slice region with the `O` ordering.

[Source](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/src/bitvec/view.rs.html#238-239)

#### fn [try\_as\_bits](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/view/trait.AsBits.html#tymethod.try_as_bits)<O>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&[BitSlice](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/slice/struct.BitSlice.html "struct bitvec::slice::BitSlice")<T, O>, [BitSpanError](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/ptr/span/enum.BitSpanError.html "enum bitvec::ptr::span::BitSpanError")<T>>

where O: [BitOrder](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/order/trait.BitOrder.html "trait bitvec::order::BitOrder"),

Attempts to view `self` as an immutable bit-slice region with the `O` ordering. [Read more](https://docs.rs/bitvec/1.0.1/x86_64-unknown-linux-gnu/bitvec/view/trait.AsBits.html#tymethod.try_as_bits)

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/lib.rs.html#147)

### impl<T> [BodyExt](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html "trait http_body_util::BodyExt") for T

where T: [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/lib.rs.html#40-42)

#### fn [frame](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.frame)(&mut self) -> [Frame](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/frame/struct.Frame.html "struct http_body_util::combinators::frame::Frame")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Returns a future that resolves to the next [`Frame`](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/frame/struct.Frame.html "struct http_body_util::combinators::frame::Frame"), if any.

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/lib.rs.html#48-52)

#### fn [map\_frame](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.map_frame)<F, B>(self, f: F) -> [MapFrame](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/map_frame/struct.MapFrame.html "struct http_body_util::combinators::map_frame::MapFrame")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([Frame](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/frame/struct.Frame.html "struct http_body::frame::Frame")<Self::[Data](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data "type http_body::Body::Data")\>) -> [Frame](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/frame/struct.Frame.html "struct http_body::frame::Frame")<B>, B: [Buf](https://docs.rs/bytes/1.11.1/x86_64-unknown-linux-gnu/bytes/buf/buf_impl/trait.Buf.html "trait bytes::buf::buf_impl::Buf"),

Maps this body’s frame to a different kind.

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/lib.rs.html#58-61)

#### fn [map\_err](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.map_err)<F, E>(self, f: F) -> [MapErr](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/map_err/struct.MapErr.html "struct http_body_util::combinators::map_err::MapErr")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error")) -> E,

Maps this body’s error value to a different value.

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/lib.rs.html#67-69)

#### fn [boxed](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.boxed)(self) -> [BoxBody](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/box_body/struct.BoxBody.html "struct http_body_util::combinators::box_body::BoxBody")<Self::[Data](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data "type http_body::Body::Data"), Self::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Turn this body into a boxed trait object.

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/lib.rs.html#75-77)

#### fn [boxed\_unsync](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.boxed_unsync)(self) -> [UnsyncBoxBody](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/box_body/struct.UnsyncBoxBody.html "struct http_body_util::combinators::box_body::UnsyncBoxBody")<Self::[Data](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data "type http_body::Body::Data"), Self::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Turn this body into a boxed trait object that is !Sync.

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/lib.rs.html#84-86)

#### fn [collect](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.collect)(self) -> [Collect](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/collect/struct.Collect.html "struct http_body_util::combinators::collect::Collect")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Turn this body into [`Collected`](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/collected/struct.Collected.html "struct http_body_util::collected::Collected") body which will collect all the DATA frames and trailers.

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/lib.rs.html#130-133)

#### fn [with\_trailers](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.with_trailers)<F>(self, trailers: F) -> [WithTrailers](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/with_trailers/struct.WithTrailers.html "struct http_body_util::combinators::with_trailers::WithTrailers")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [Future](../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[HeaderMap](https://docs.rs/http/1.4.1/x86_64-unknown-linux-gnu/http/header/map/struct.HeaderMap.html "struct http::header::map::HeaderMap"), Self::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error")\>>>,

Add trailers to the body. [Read more](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.with_trailers)

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/lib.rs.html#139-141)

#### fn [into\_data\_stream](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/trait.BodyExt.html#method.into_data_stream)(self) -> [BodyDataStream](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.BodyDataStream.html "struct http_body_util::stream::BodyDataStream")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Turn this body into [`BodyDataStream`](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/stream/struct.BodyDataStream.html "struct http_body_util::stream::BodyDataStream").

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

[Source](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)

### impl<T> [Brush](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/style/brush/trait.Brush.html "trait parley::style::brush::Brush") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/taffy/0.10.1/x86_64-unknown-linux-gnu/src/taffy/style/mod.rs.html#62-63)

### impl<T> [CheapCloneStr](https://docs.rs/taffy/0.10.1/x86_64-unknown-linux-gnu/taffy/style/trait.CheapCloneStr.html "trait taffy::style::CheapCloneStr") for T

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + for<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + 'static,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#104-107)

### impl<Q, K> [Comparable](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html "trait equivalent::Comparable")<K> for Q

where Q: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#110)

#### fn [compare](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html#tymethod.compare)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

Compare self to `key` and return their ordering.

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#47-49)

### impl<T> [DynHash](../ecs/label/trait.DynHash.html "trait bevy::ecs::label::DynHash") for T

where T: [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#51)

#### fn [dyn\_hash](../ecs/label/trait.DynHash.html#tymethod.dyn_hash)(&self, state: &mut dyn [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"))

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher").

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#39-41)

### impl<'de, T> [DynamicDeserialize](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html "trait zvariant::type::dynamic::DynamicDeserialize")<'de> for T

where T: [Type](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html "trait zvariant::type::Type") + [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#43)

#### type [Deserializer](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html#associatedtype.Deserializer) = [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

A [DeserializeSeed](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeSeed.html "trait serde_core::de::DeserializeSeed") implementation for this type.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#45)

#### fn [deserializer\_for\_signature](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html#tymethod.deserializer_for_signature)( signature: &[Signature](https://docs.rs/zvariant_utils/3.3.0/x86_64-unknown-linux-gnu/zvariant_utils/signature/enum.Signature.html "enum zvariant_utils::signature::Signature"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [DynamicDeserialize](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html "trait zvariant::type::dynamic::DynamicDeserialize")<'de>>::[Deserializer](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html#associatedtype.Deserializer "type zvariant::type::dynamic::DynamicDeserialize::Deserializer"), [Error](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/error/enum.Error.html "enum zvariant::error::Error")\>

Get a deserializer compatible with this parsed signature.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#30-32)

### impl<T> [DynamicType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicType.html "trait zvariant::type::dynamic::DynamicType") for T

where T: [Type](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html "trait zvariant::type::Type") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#34)

#### fn [signature](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicType.html#tymethod.signature)(&self) -> [Signature](https://docs.rs/zvariant_utils/3.3.0/x86_64-unknown-linux-gnu/zvariant_utils/signature/enum.Signature.html "enum zvariant_utils::signature::Signature")

The type signature for `self`. [Read more](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicType.html#tymethod.signature)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Compare self to `key` and return `true` if they are equal.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#151-154)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#156)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

### impl<T> [Instrument](../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

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

### impl<T> [IntoResult](../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/slice.rs.html#102)

### impl<S, T> [ParallelSlice](../tasks/trait.ParallelSlice.html "trait bevy::tasks::ParallelSlice")<T> for S

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), S: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/slice.rs.html#37-40)

#### fn [par\_chunk\_map](../tasks/trait.ParallelSlice.html#method.par_chunk_map)<F, R>( &self, task\_pool: &[TaskPool](../tasks/struct.TaskPool.html "struct bevy::tasks::TaskPool"), chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), f: F, ) -> [Vec](struct.Vec.html "struct bevy::prelude::Vec")<R>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> R + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), R: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Splits the slice in chunks of size `chunks_size` or less and maps the chunks in parallel across the provided `task_pool`. One task is spawned in the task pool for every chunk. [Read more](../tasks/trait.ParallelSlice.html#method.par_chunk_map)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/slice.rs.html#84-87)

#### fn [par\_splat\_map](../tasks/trait.ParallelSlice.html#method.par_splat_map)<F, R>( &self, task\_pool: &[TaskPool](../tasks/struct.TaskPool.html "struct bevy::tasks::TaskPool"), max\_tasks: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, f: F, ) -> [Vec](struct.Vec.html "struct bevy::prelude::Vec")<R>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> R + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), R: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Splits the slice into a maximum of `max_tasks` chunks, and maps the chunks in parallel across the provided `task_pool`. One task is spawned in the task pool for every chunk. [Read more](../tasks/trait.ParallelSlice.html#method.par_splat_map)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#203-206)

### impl<T> [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source") for T

where T: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#208)

#### type [Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice)<'a> = <<T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target") as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'a> where T: 'a

A type this `Source` can be sliced into.

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#213)

#### fn [len](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Length of the source

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#217-219)

#### fn [read](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)<'a, Chunk>(&'a self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Chunk>

where Chunk: [Chunk](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Chunk.html "trait logos::source::Chunk")<'a>,

Read a chunk of bytes into an array. Returns `None` when reading out of bounds would occur. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#224)

#### fn [slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)(&self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>>

Get a slice of the source at given range. This is analogous to `slice::get(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#229)

#### unsafe fn [slice\_unchecked](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)( &self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> <T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>

Available on **non-crate feature `forbid_unsafe`** only.

Get a slice of the source at given range. This is analogous to `slice::get_unchecked(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#233)

#### fn [is\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Check if `index` is valid for this `Source`, that is: [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#237)

#### fn [find\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

For `&str` sources attempts to find the closest `char` boundary at which source can be sliced, starting from `index`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)

[Source](https://docs.rs/syn/2.0.117/x86_64-unknown-linux-gnu/src/syn/spanned.rs.html#104)

### impl<T> [Spanned](https://docs.rs/syn/2.0.117/x86_64-unknown-linux-gnu/syn/spanned/trait.Spanned.html "trait syn::spanned::Spanned") for T

where T: Spanned + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/syn/2.0.117/x86_64-unknown-linux-gnu/src/syn/spanned.rs.html#105)

#### fn [span](https://docs.rs/syn/2.0.117/x86_64-unknown-linux-gnu/syn/spanned/trait.Spanned.html#tymethod.span)(&self) -> [Span](https://docs.rs/proc-macro2/1.0.106/x86_64-unknown-linux-gnu/proc_macro2/struct.Span.html "struct proc_macro2::Span")

Returns a `Span` covering the complete contents of this syntax tree node, or [`Span::call_site()`](https://docs.rs/proc-macro2/1.0.106/x86_64-unknown-linux-gnu/proc_macro2/struct.Span.html#method.call_site "associated function proc_macro2::Span::call_site") if this node is empty.

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/src/hex/lib.rs.html#137)

### impl<T> [ToHex](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/hex/trait.ToHex.html "trait hex::ToHex") for T

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>,

[Source](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/src/hex/lib.rs.html#138)

#### fn [encode\_hex](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/hex/trait.ToHex.html#tymethod.encode_hex)<U>(&self) -> U

where U: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>,

Encode the hex strict representing `self` into the result. Lower case letters are used (e.g. `f9b4ca`)

[Source](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/src/hex/lib.rs.html#142)

#### fn [encode\_hex\_upper](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/hex/trait.ToHex.html#tymethod.encode_hex_upper)<U>(&self) -> U

where U: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>,

Encode the hex strict representing `self` into the result. Upper case letters are used (e.g. `F9B4CA`)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](trait.ToOwned.html#method.clone_into)

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

### impl<T> [ToString](trait.ToString.html "trait bevy::prelude::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2902)

#### fn [to\_string](trait.ToString.html#tymethod.to_string)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Converts the given value to a `String`. [Read more](trait.ToString.html#tymethod.to_string)

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/key.rs.html#14-16)

### impl<T> [ToTomlKey](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/key/trait.ToTomlKey.html "trait toml_writer::key::ToTomlKey") for T

where T: [WriteTomlKey](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/key/trait.WriteTomlKey.html "trait toml_writer::key::WriteTomlKey") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/key.rs.html#18)

#### fn [to\_toml\_key](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/key/trait.ToTomlKey.html#tymethod.to_toml_key)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/value.rs.html#17-19)

### impl<T> [ToTomlValue](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/value/trait.ToTomlValue.html "trait toml_writer::value::ToTomlValue") for T

where T: [WriteTomlValue](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/value/trait.WriteTomlValue.html "trait toml_writer::value::WriteTomlValue") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/value.rs.html#21)

#### fn [to\_toml\_value](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/value/trait.ToTomlValue.html#tymethod.to_toml_value)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#88)

### impl<W> [TomlWrite](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html "trait toml_writer::write::TomlWrite") for W

where W: [Write](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html "trait core::fmt::Write"),

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#2)

#### fn [open\_table\_header](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.open_table_header)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#5)

#### fn [close\_table\_header](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.close_table_header)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#9)

#### fn [open\_array\_of\_tables\_header](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.open_array_of_tables_header)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#12)

#### fn [close\_array\_of\_tables\_header](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.close_array_of_tables_header)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#16)

#### fn [open\_inline\_table](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.open_inline_table)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#19)

#### fn [close\_inline\_table](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.close_inline_table)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#23)

#### fn [open\_array](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.open_array)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#26)

#### fn [close\_array](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.close_array)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#30)

#### fn [key\_sep](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.key_sep)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#34)

#### fn [keyval\_sep](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.keyval_sep)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#41)

#### fn [key](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.key)(&mut self, value: impl [WriteTomlKey](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/key/trait.WriteTomlKey.html "trait toml_writer::key::WriteTomlKey")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Write an encoded TOML key [Read more](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.key)

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#67)

#### fn [value](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.value)(&mut self, value: impl [WriteTomlValue](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/value/trait.WriteTomlValue.html "trait toml_writer::value::WriteTomlValue")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Write an encoded TOML scalar value [Read more](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.value)

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#71)

#### fn [val\_sep](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.val_sep)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#75)

#### fn [space](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.space)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#79)

#### fn [open\_comment](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.open_comment)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/src/toml_writer/write.rs.html#83)

#### fn [newline](https://docs.rs/toml_writer/1.1.1+spec-1.1.0/x86_64-unknown-linux-gnu/toml_writer/write/trait.TomlWrite.html#method.newline)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

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

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"&BitSlice<T, O>":"<h3>Notable traits for <code>&amp;<a class=\\"struct\\" href=\\"https://docs.rs/bitvec/1.0.1/x86\_64-unknown-linux-gnu/bitvec/slice/struct.BitSlice.html\\" title=\\"struct bitvec::slice::BitSlice\\">BitSlice</a>&lt;T, O&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, O&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;<a class=\\"struct\\" href=\\"https://docs.rs/bitvec/1.0.1/x86\_64-unknown-linux-gnu/bitvec/slice/struct.BitSlice.html\\" title=\\"struct bitvec::slice::BitSlice\\">BitSlice</a>&lt;T, O&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://docs.rs/bitvec/1.0.1/x86\_64-unknown-linux-gnu/bitvec/store/trait.BitStore.html\\" title=\\"trait bitvec::store::BitStore\\">BitStore</a>,\\n O: <a class=\\"trait\\" href=\\"https://docs.rs/bitvec/1.0.1/x86\_64-unknown-linux-gnu/bitvec/order/trait.BitOrder.html\\" title=\\"trait bitvec::order::BitOrder\\">BitOrder</a>,\\n <a class=\\"struct\\" href=\\"https://docs.rs/bitvec/1.0.1/x86\_64-unknown-linux-gnu/bitvec/slice/struct.BitSlice.html\\" title=\\"struct bitvec::slice::BitSlice\\">BitSlice</a>&lt;T, O&gt;: <a class=\\"trait\\" href=\\"https://docs.rs/bitvec/1.0.1/x86\_64-unknown-linux-gnu/bitvec/field/trait.BitField.html\\" title=\\"trait bitvec::field::BitField\\">BitField</a>,</div></div>","&\[u8\]":"<h3>Notable traits for <code>&amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","&mut Vec<u8>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div>","&mut \[u8\]":"<h3>Notable traits for <code>&amp;mut \[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for &amp;mut \[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Arc<str>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Bytes<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html\\" title=\\"struct core::str::iter::Bytes\\">Bytes</a>&lt;'\_&gt;</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html\\" title=\\"struct core::str::iter::Bytes\\">Bytes</a>&lt;'\_&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>;</div>","CharIndices<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html\\" title=\\"struct core::str::iter::CharIndices\\">CharIndices</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html\\" title=\\"struct core::str::iter::CharIndices\\">CharIndices</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>);</div>","Chars<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html\\" title=\\"struct core::str::iter::Chars\\">Chars</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html\\" title=\\"struct core::str::iter::Chars\\">Chars</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","Collect<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/http-body-util/0.1.3/x86\_64-unknown-linux-gnu/http\_body\_util/combinators/collect/struct.Collect.html\\" title=\\"struct http\_body\_util::combinators::collect::Collect\\">Collect</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/http-body-util/0.1.3/x86\_64-unknown-linux-gnu/http\_body\_util/combinators/collect/struct.Collect.html\\" title=\\"struct http\_body\_util::combinators::collect::Collect\\">Collect</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html\\" title=\\"trait http\_body::Body\\">Body</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/http-body-util/0.1.3/x86\_64-unknown-linux-gnu/http\_body\_util/collected/struct.Collected.html\\" title=\\"struct http\_body\_util::collected::Collected\\">Collected</a>&lt;&lt;T as <a class=\\"trait\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html\\" title=\\"trait http\_body::Body\\">Body</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html#associatedtype.Data\\" title=\\"type http\_body::Body::Data\\">Data</a>&gt;, &lt;T as <a class=\\"trait\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html\\" title=\\"trait http\_body::Body\\">Body</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html#associatedtype.Error\\" title=\\"type http\_body::Body::Error\\">Error</a>&gt;;</div>","Drain<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/string/struct.Drain.html\\" title=\\"struct alloc::string::Drain\\">Drain</a>&lt;'\_&gt;</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/string/struct.Drain.html\\" title=\\"struct alloc::string::Drain\\">Drain</a>&lt;'\_&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","EncodeUtf16<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html\\" title=\\"struct core::str::iter::EncodeUtf16\\">EncodeUtf16</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html\\" title=\\"struct core::str::iter::EncodeUtf16\\">EncodeUtf16</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\\">u16</a>;</div>","EscapeDebug<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html\\" title=\\"struct core::str::iter::EscapeDebug\\">EscapeDebug</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html\\" title=\\"struct core::str::iter::EscapeDebug\\">EscapeDebug</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","EscapeDefault<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html\\" title=\\"struct core::str::iter::EscapeDefault\\">EscapeDefault</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html\\" title=\\"struct core::str::iter::EscapeDefault\\">EscapeDefault</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","EscapeUnicode<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html\\" title=\\"struct core::str::iter::EscapeUnicode\\">EscapeUnicode</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html\\" title=\\"struct core::str::iter::EscapeUnicode\\">EscapeUnicode</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","Frame<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/http-body-util/0.1.3/x86\_64-unknown-linux-gnu/http\_body\_util/combinators/frame/struct.Frame.html\\" title=\\"struct http\_body\_util::combinators::frame::Frame\\">Frame</a>&lt;'\_, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"https://docs.rs/http-body-util/0.1.3/x86\_64-unknown-linux-gnu/http\_body\_util/combinators/frame/struct.Frame.html\\" title=\\"struct http\_body\_util::combinators::frame::Frame\\">Frame</a>&lt;'\_, T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html\\" title=\\"trait http\_body::Body\\">Body</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;<a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"struct\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/frame/struct.Frame.html\\" title=\\"struct http\_body::frame::Frame\\">Frame</a>&lt;&lt;T as <a class=\\"trait\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html\\" title=\\"trait http\_body::Body\\">Body</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html#associatedtype.Data\\" title=\\"type http\_body::Body::Data\\">Data</a>&gt;, &lt;T as <a class=\\"trait\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html\\" title=\\"trait http\_body::Body\\">Body</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://docs.rs/http-body/1.0.1/x86\_64-unknown-linux-gnu/http\_body/trait.Body.html#associatedtype.Error\\" title=\\"type http\_body::Body::Error\\">Error</a>&gt;&gt;;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","IntoChars":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/string/struct.IntoChars.html\\" title=\\"struct alloc::string::IntoChars\\">IntoChars</a></code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/string/struct.IntoChars.html\\" title=\\"struct alloc::string::IntoChars\\">IntoChars</a></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","Lines<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html\\" title=\\"struct core::str::iter::Lines\\">Lines</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html\\" title=\\"struct core::str::iter::Lines\\">Lines</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","LinesAny<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html\\" title=\\"struct core::str::iter::LinesAny\\">LinesAny</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html\\" title=\\"struct core::str::iter::LinesAny\\">LinesAny</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","MatchIndices<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html\\" title=\\"struct core::str::iter::MatchIndices\\">MatchIndices</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html\\" title=\\"struct core::str::iter::MatchIndices\\">MatchIndices</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>);</div>","Matches<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html\\" title=\\"struct core::str::iter::Matches\\">Matches</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html\\" title=\\"struct core::str::iter::Matches\\">Matches</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RMatchIndices<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html\\" title=\\"struct core::str::iter::RMatchIndices\\">RMatchIndices</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html\\" title=\\"struct core::str::iter::RMatchIndices\\">RMatchIndices</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>);</div>","RMatches<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html\\" title=\\"struct core::str::iter::RMatches\\">RMatches</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html\\" title=\\"struct core::str::iter::RMatches\\">RMatches</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RSplit<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html\\" title=\\"struct core::str::iter::RSplit\\">RSplit</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html\\" title=\\"struct core::str::iter::RSplit\\">RSplit</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RSplitN<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html\\" title=\\"struct core::str::iter::RSplitN\\">RSplitN</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html\\" title=\\"struct core::str::iter::RSplitN\\">RSplitN</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RSplitTerminator<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html\\" title=\\"struct core::str::iter::RSplitTerminator\\">RSplitTerminator</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html\\" title=\\"struct core::str::iter::RSplitTerminator\\">RSplitTerminator</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","Split<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html\\" title=\\"struct core::str::iter::Split\\">Split</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html\\" title=\\"struct core::str::iter::Split\\">Split</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitAsciiWhitespace<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html\\" title=\\"struct core::str::iter::SplitAsciiWhitespace\\">SplitAsciiWhitespace</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html\\" title=\\"struct core::str::iter::SplitAsciiWhitespace\\">SplitAsciiWhitespace</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitInclusive<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html\\" title=\\"struct core::str::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html\\" title=\\"struct core::str::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitN<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html\\" title=\\"struct core::str::iter::SplitN\\">SplitN</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html\\" title=\\"struct core::str::iter::SplitN\\">SplitN</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitTerminator<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html\\" title=\\"struct core::str::iter::SplitTerminator\\">SplitTerminator</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html\\" title=\\"struct core::str::iter::SplitTerminator\\">SplitTerminator</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitWhitespace<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html\\" title=\\"struct core::str::iter::SplitWhitespace\\">SplitWhitespace</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html\\" title=\\"struct core::str::iter::SplitWhitespace\\">SplitWhitespace</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","Vec<u8>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}