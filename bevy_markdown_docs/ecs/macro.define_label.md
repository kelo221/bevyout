[bevy](../index.html)::[ecs](index.html)

# Macro define\_label 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#86)

```rust
macro_rules! define_label {
    (
        $(#[$label_attr:meta])*
        $label_trait_name:ident,
        $interner_name:ident
    ) => { ... };
    (
        $(#[$label_attr:meta])*
        $label_trait_name:ident,
        $interner_name:ident,
        extra_methods: { $($trait_extra_methods:tt)* },
        extra_methods_impl: { $($interned_extra_methods_impl:tt)* }
    ) => { ... };
}
```

Macro to define a new label trait

## Example

```rust
define_label!(
    /// Documentation of label trait
    MyNewLabelTrait,
    MY_NEW_LABEL_TRAIT_INTERNER
);

define_label!(
    /// Documentation of another label trait
    MyNewExtendedLabelTrait,
    MY_NEW_EXTENDED_LABEL_TRAIT_INTERNER,
    extra_methods: {
        // Extra methods for the trait can be defined here
        fn additional_method(&self) -> i32;
    },
    extra_methods_impl: {
        // Implementation of the extra methods for Interned<dyn MyNewExtendedLabelTrait>
        fn additional_method(&self) -> i32 {
            0
        }
    }
);
```