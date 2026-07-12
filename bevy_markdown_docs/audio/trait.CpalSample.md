[bevy](../index.html)::[audio](index.html)

# Trait CpalSample 

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#39)

```rust
pub trait CpalSample:
    Copy
    + Clone
    + PartialOrd
    + PartialEq {
    type Signed: SignedSample + Duplex<Self>;
    type Float: FloatSample + Duplex<Self>;

    const EQUILIBRIUM: Self;
    const IDENTITY: Self::Float = <Self::Float as FloatSample>::IDENTITY;

    // Provided methods
    fn to_sample<S>(self) -> S
       where Self: ToSample<S> { ... }
    fn from_sample<S>(s: S) -> Self
       where Self: FromSample<S> { ... }
    fn to_signed_sample(self) -> Self::Signed { ... }
    fn to_float_sample(self) -> Self::Float { ... }
    fn add_amp(self, amp: Self::Signed) -> Self { ... }
    fn mul_amp(self, amp: Self::Float) -> Self { ... }
}
```

A trait for working generically across different **Sample** format types.

Provides methods for converting to and from any type that implements the [`FromSample`](./trait.FromSample.html) trait and provides methods for performing signal amplitude addition and multiplication.

## Example

```rust
use dasp_sample::{I24, Sample};

fn main() {
    assert_eq!((-1.0).to_sample::<u8>(), 0);
    assert_eq!(0.0.to_sample::<u8>(), 128);
    assert_eq!(0i32.to_sample::<u32>(), 2_147_483_648);
    assert_eq!(I24::new(0).unwrap(), Sample::from_sample(0.0));
    assert_eq!(0.0, Sample::EQUILIBRIUM);
}
```

## Required Associated Constants

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#86)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): Self

The equilibrium value for the wave that this `Sample` type represents. This is normally the value that is equal distance from both the min and max ranges of the sample.

##### Example

```rust
use dasp_sample::Sample;

fn main() {
    assert_eq!(0.0, f32::EQUILIBRIUM);
    assert_eq!(0, i32::EQUILIBRIUM);
    assert_eq!(128, u8::EQUILIBRIUM);
    assert_eq!(32_768_u16, Sample::EQUILIBRIUM);
}
```

**Note:** This will likely be changed to an “associated const” if the feature lands.

## Provided Associated Constants

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#107)

#### const [IDENTITY](#associatedconstant.IDENTITY): Self::[Float](trait.CpalSample.html#associatedtype.Float "type bevy::audio::CpalSample::Float") = <Self::Float as FloatSample>::IDENTITY

The multiplicative identity of the signal.

In other words: A value which when used to scale/multiply the amplitude or frequency of a signal, returns the same signal.

This is useful as a default, non-affecting amplitude or frequency multiplier.

##### Example

```rust
use dasp_sample::{Sample, U48};

fn main() {
    assert_eq!(1.0, f32::IDENTITY);
    assert_eq!(1.0, i8::IDENTITY);
    assert_eq!(1.0, u8::IDENTITY);
    assert_eq!(1.0, U48::IDENTITY);
}
```

## Required Associated Types

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#52)

#### type [Signed](#associatedtype.Signed): [SignedSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/trait.SignedSample.html "trait dasp_sample::SignedSample") + [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<Self>

When summing two samples of a signal together, it is necessary for both samples to be represented in some signed format. This associated `Addition` type represents the format to which `Self` should be converted for optimal `Addition` performance.

For example, u32’s optimal `Addition` type would be i32, u8’s would be i8, f32’s would be f32, etc.

Specifying this as an associated type allows us to automatically determine the optimal, lossless Addition format type for summing any two unique `Sample` types together.

As a user of the `sample` crate, you will never need to be concerned with this type unless you are defining your own unique `Sample` type(s).

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#67)

#### type [Float](#associatedtype.Float): [FloatSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/trait.FloatSample.html "trait dasp_sample::FloatSample") + [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<Self>

When multiplying two samples of a signal together, it is necessary for both samples to be represented in some signed, floating-point format. This associated `Multiplication` type represents the format to which `Self` should be converted for optimal `Multiplication` performance.

For example, u32’s optimal `Multiplication` type would be f32, u64’s would be f64, i8’s would be f32, etc.

Specifying this as an associated type allows us to automatically determine the optimal, lossless Multiplication format type for multiplying any two unique `Sample` types together.

As a user of the `sample` crate, you will never need to be concerned with this type unless you are defining your own unique `Sample` type(s).

## Provided Methods

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#125-127)

#### fn [to\_sample](#method.to_sample)<S>(self) -> S

where Self: [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

Convert `self` to any type that implements `FromSample<Self>`.

Find more details on type-specific conversion ranges and caveats in the `conv` module.

##### Example

```rust
use dasp_sample::Sample;

fn main() {
    assert_eq!(0.0.to_sample::<i32>(), 0);
    assert_eq!(0.0.to_sample::<u8>(), 128);
    assert_eq!((-1.0).to_sample::<u8>(), 0);
}
```

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#149-151)

#### fn [from\_sample](#method.from_sample)<S>(s: S) -> Self

where Self: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S>,

Create a `Self` from any type that implements `ToSample<Self>`.

Find more details on type-specific conversion ranges and caveats in the `conv` module.

##### Example

```rust
use dasp_sample::{Sample, I24};

fn main() {
    assert_eq!(f32::from_sample(128_u8), 0.0);
    assert_eq!(i8::from_sample(-1.0), -128);
    assert_eq!(I24::from_sample(0.0), I24::new(0).unwrap());
}
```

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#170)

#### fn [to\_signed\_sample](#method.to_signed_sample)(self) -> Self::[Signed](trait.CpalSample.html#associatedtype.Signed "type bevy::audio::CpalSample::Signed")

Converts `self` to the equivalent `Sample` in the associated `Signed` format.

This is a simple wrapper around `Sample::to_sample` which may provide extra convenience in some cases, particularly for assisting type inference.

##### Example

```rust
use dasp_sample::Sample;

fn main() {
    assert_eq!(128_u8.to_signed_sample(), 0i8);
}
```

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#188)

#### fn [to\_float\_sample](#method.to_float_sample)(self) -> Self::[Float](trait.CpalSample.html#associatedtype.Float "type bevy::audio::CpalSample::Float")

Converts `self` to the equivalent `Sample` in the associated `Float` format.

This is a simple wrapper around `Sample::to_sample` which may provide extra convenience in some cases, particularly for assisting type inference.

##### Example

```rust
use dasp_sample::Sample;

fn main() {
    assert_eq!(128_u8.to_float_sample(), 0.0);
}
```

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#209)

#### fn [add\_amp](#method.add_amp)(self, amp: Self::[Signed](trait.CpalSample.html#associatedtype.Signed "type bevy::audio::CpalSample::Signed")) -> Self

Adds (or “offsets”) the amplitude of the `Sample` by the given signed amplitude.

`Self` will be converted to `Self::Signed`, the addition will occur and then the result will be converted back to `Self`. These conversions allow us to correctly handle the addition of unsigned signal formats.

##### Example

```rust
use dasp_sample::Sample;

fn main() {
    assert_eq!(0.25.add_amp(0.5), 0.75);
    assert_eq!(192u8.add_amp(-128), 64);
}
```

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#237)

#### fn [mul\_amp](#method.mul_amp)(self, amp: Self::[Float](trait.CpalSample.html#associatedtype.Float "type bevy::audio::CpalSample::Float")) -> Self

Multiplies (or “scales”) the amplitude of the `Sample` by the given float amplitude.

*   `amp` > 1.0 amplifies the sample.
*   `amp` < 1.0 attenuates the sample.
*   `amp` == 1.0 yields the same sample.
*   `amp` == 0.0 yields the `Sample::EQUILIBRIUM`.

`Self` will be converted to `Self::Float`, the multiplication will occur and then the result will be converted back to `Self`. These conversions allow us to correctly handle the multiplication of integral signal formats.

##### Example

```rust
use dasp_sample::Sample;

fn main() {
    assert_eq!(64_i8.mul_amp(0.5), 32);
    assert_eq!(0.5.mul_amp(-2.0), -1.0);
    assert_eq!(64_u8.mul_amp(0.0), 128);
}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) = 0.0

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html) = 0.0

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html) = 0

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html) = 0

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html) = 0

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html) = 0

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html) = 128

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html) = 32\_768

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html) = 2\_147\_483\_648

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html) = 9\_223\_372\_036\_854\_775\_808

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

## Implementors

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [I24](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/i24/struct.I24.html "struct dasp_sample::types::i24::I24")

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [I24](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/i24/struct.I24.html "struct dasp_sample::types::i24::I24") = types::i24::EQUILIBRIUM

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [I24](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/i24/struct.I24.html "struct dasp_sample::types::i24::I24")

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [I48](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/i48/struct.I48.html "struct dasp_sample::types::i48::I48")

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [I48](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/i48/struct.I48.html "struct dasp_sample::types::i48::I48") = types::i48::EQUILIBRIUM

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [I48](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/i48/struct.I48.html "struct dasp_sample::types::i48::I48")

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [U24](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/u24/struct.U24.html "struct dasp_sample::types::u24::U24")

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [U24](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/u24/struct.U24.html "struct dasp_sample::types::u24::U24") = types::u24::EQUILIBRIUM

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

### impl [Sample](trait.CpalSample.html "trait bevy::audio::CpalSample") for [U48](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/u48/struct.U48.html "struct dasp_sample::types::u48::U48")

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### const [EQUILIBRIUM](#associatedconstant.EQUILIBRIUM): [U48](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/types/u48/struct.U48.html "struct dasp_sample::types::u48::U48") = types::u48::EQUILIBRIUM

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Signed](#associatedtype.Signed) = [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/lib.rs.html#261-276)

#### type [Float](#associatedtype.Float) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)