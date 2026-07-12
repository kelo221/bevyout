[bevy](../../index.html)::[platform](../index.html)::[prelude](index.html)

# Macro vec 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/macros.rs.html#42)

```rust
macro_rules! vec {
    () => { ... };
    ($elem:expr; $n:expr) => { ... };
    ($($x:expr),+ $(,)?) => { ... };
}
```

Available on **non-`no_global_oom_handling`** only.

Creates a [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") containing the arguments.

`vec!` allows `Vec`s to be defined with the same syntax as array expressions. There are two forms of this macro:

*   Create a [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") containing a given list of elements:

```rust
let v = vec![1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
```

*   Create a [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") from a given element and size:

```rust
let v = vec![1; 3];
assert_eq!(v, [1, 1, 1]);
```

Note that unlike array expressions this syntax supports all elements which implement [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") and the number of elements doesn’t have to be a constant.

This will use `clone` to duplicate an expression, so one should be careful using this with types having a nonstandard `Clone` implementation. For example, `vec![Rc::new(1); 5]` will create a vector of five references to the same boxed integer value, not five references pointing to independently boxed integers.

Also, note that `vec![expr; 0]` is allowed, and produces an empty vector. This will still evaluate `expr`, however, and immediately drop the resulting value, so be mindful of side effects.