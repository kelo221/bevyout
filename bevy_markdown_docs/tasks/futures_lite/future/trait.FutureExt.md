[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Trait FutureExt 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#686)

```rust
pub trait FutureExt: Future {
    // Provided methods
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Self::Output>
       where Self: Unpin { ... }
    fn or<F>(self, other: F) -> Or<Self, F> ⓘ
       where Self: Sized,
             F: Future<Output = Self::Output> { ... }
    fn race<F>(self, other: F) -> Race<Self, F> ⓘ
       where Self: Sized,
             F: Future<Output = Self::Output> { ... }
    fn catch_unwind(self) -> CatchUnwind<Self> ⓘ
       where Self: Sized + UnwindSafe { ... }
    fn boxed<'a>(
        self,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>
       where Self: Sized + Send + 'a { ... }
    fn boxed_local<'a>(self) -> Pin<Box<dyn Future<Output = Self::Output> + 'a>>
       where Self: Sized + 'a { ... }
}
```

Extension trait for [`Future`](../trait.Future.html "trait bevy::tasks::futures_lite::Future").

## Provided Methods

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#688-690)

#### fn [poll](#method.poll)(&mut self, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

A convenience for calling [`Future::poll()`](../trait.Future.html#tymethod.poll "method bevy::tasks::futures_lite::Future::poll") on `!`[`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") types.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#713-716)

#### fn [or](#method.or)<F>(self, other: F) -> [Or](struct.Or.html "struct bevy::tasks::futures_lite::future::Or")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>,

Returns the result of `self` or `other` future, preferring `self` if both are ready.

If you need to treat the two futures fairly without a preference for either, use the [`race()`](fn.race.html "fn bevy::tasks::futures_lite::future::race") function or the [`FutureExt::race()`](../trait.FutureExt.html#method.race "method bevy::tasks::futures_lite::FutureExt::race") method.

##### Examples

```rust
use futures_lite::future::{pending, ready, FutureExt};

assert_eq!(ready(1).or(pending()).await, 1);
assert_eq!(pending().or(ready(2)).await, 2);

// The first future wins.
assert_eq!(ready(1).or(ready(2)).await, 1);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#746-749)

#### fn [race](#method.race)<F>(self, other: F) -> [Race](struct.Race.html "struct bevy::tasks::futures_lite::future::Race")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>,

Available on **crate features `race` and `std`** only.

Returns the result of `self` or `other` future, with no preference if both are ready.

Each time [`Race`](struct.Race.html "struct bevy::tasks::futures_lite::future::Race") is polled, the two inner futures are polled in random order. Therefore, no future takes precedence over the other if both can complete at the same time.

If you have preference for one of the futures, use the [`or()`](fn.or.html "fn bevy::tasks::futures_lite::future::or") function or the [`FutureExt::or()`](../trait.FutureExt.html#method.or "method bevy::tasks::futures_lite::FutureExt::or") method.

##### Examples

```rust
use futures_lite::future::{pending, ready, FutureExt};

assert_eq!(ready(1).race(pending()).await, 1);
assert_eq!(pending().race(ready(2)).await, 2);

// One of the two futures is randomly chosen as the winner.
let res = ready(1).race(ready(2)).await;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#774-776)

#### fn [catch\_unwind](#method.catch_unwind)(self) -> [CatchUnwind](struct.CatchUnwind.html "struct bevy::tasks::futures_lite::future::CatchUnwind")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

Available on **crate feature `std`** only.

Catches panics while polling the future.

##### Examples

```rust
use futures_lite::future::FutureExt;

let fut1 = async {}.catch_unwind();
let fut2 = async { panic!() }.catch_unwind();

assert!(fut1.await.is_ok());
assert!(fut2.await.is_err());
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#798-800)

#### fn [boxed](#method.boxed)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a,

Available on **crate feature `alloc`** only.

Boxes the future and changes its type to `dyn Future + Send + 'a`.

##### Examples

```rust
use futures_lite::future::{self, FutureExt};

let a = future::ready('a');
let b = future::pending();

// Futures of different types can be stored in
// the same collection when they are boxed:
let futures = vec![a.boxed(), b.boxed()];
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#822-824)

#### fn [boxed\_local](#method.boxed_local)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\> + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + 'a,

Available on **crate feature `alloc`** only.

Boxes the future and changes its type to `dyn Future + 'a`.

##### Examples

```rust
use futures_lite::future::{self, FutureExt};

let a = future::ready('a');
let b = future::pending();

// Futures of different types can be stored in
// the same collection when they are boxed:
let futures = vec![a.boxed_local(), b.boxed_local()];
```

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#830)

### impl<F> [FutureExt](../trait.FutureExt.html "trait bevy::tasks::futures_lite::FutureExt") for F

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

{"CatchUnwind<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.CatchUnwind.html\\" title=\\"struct bevy::tasks::futures\_lite::future::CatchUnwind\\">CatchUnwind</a>&lt;F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.CatchUnwind.html\\" title=\\"struct bevy::tasks::futures\_lite::future::CatchUnwind\\">CatchUnwind</a>&lt;F&gt;<div class=\\"where\\">where\\n F: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/panic/unwind\_safe/trait.UnwindSafe.html\\" title=\\"trait core::panic::unwind\_safe::UnwindSafe\\">UnwindSafe</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;&lt;F as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>, <a class=\\"struct\\" href=\\"../../../prelude/struct.Box.html\\" title=\\"struct bevy::prelude::Box\\">Box</a>&lt;dyn <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/any/trait.Any.html\\" title=\\"trait core::any::Any\\">Any</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\\" title=\\"trait core::marker::Send\\">Send</a>&gt;&gt;;</div>","Or<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Or.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Or\\">Or</a>&lt;F1, F2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F1, F2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Or.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Or\\">Or</a>&lt;F1, F2&gt;<div class=\\"where\\">where\\n F1: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,\\n F2: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>","Race<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Race.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Race\\">Race</a>&lt;F1, F2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F1, F2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Race.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Race\\">Race</a>&lt;F1, F2&gt;<div class=\\"where\\">where\\n F1: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,\\n F2: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>"}