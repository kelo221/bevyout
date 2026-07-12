[bevy](../index.html)::[reflect](index.html)

# Macro load\_type\_registrations 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#872)

```rust
load_type_registrations!() { /* proc-macro */ }
```

Collects and loads type registrations when using `auto_register_static` feature.

Correctly using this macro requires following:

1.  This macro must be called **last** during compilation. This can be achieved by putting your main function in a separate crate or restructuring your project to be separated into `bin` and `lib`, and putting this macro in `bin`. Any automatic type registrations using `#[derive(Reflect)]` within the same crate as this macro are not guaranteed to run.
2.  Your project must be compiled with `auto_register_static` feature **and** `BEVY_REFLECT_AUTO_REGISTER_STATIC=1` env variable. Enabling the feature generates registration functions while setting the variable enables export and caching of registration function names.
3.  Must be called before creating `App` or using `TypeRegistry::register_derived_types`.

If you’re experiencing linking issues try running `cargo clean` before rebuilding.