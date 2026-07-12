[bevy](../index.html)::[reflect](index.html)

# Attribute Macro reflect\_trait 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#551)

```rust
#[reflect_trait]
```

A macro that automatically generates type data for traits, which their implementors can then register.

The output of this macro is a struct that takes reflected instances of the implementor’s type and returns the value as a trait object. Because of this, **it can only be used on [object-safe](https://doc.rust-lang.org/reference/items/traits.html#object-safety) traits.**

For a trait named `MyTrait`, this will generate the struct `ReflectMyTrait`. The generated struct can be created using `FromType` with any type that implements the trait. The creation and registration of this generated struct as type data can be automatically handled by [`#[derive(Reflect)]`](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect").

## Example

[ⓘ](# "This example is not tested")

```rust
#[reflect_trait] // Generates `ReflectMyTrait`
trait MyTrait {
  fn print(&self) -> &str;
}

#[derive(Reflect)]
#[reflect(MyTrait)] // Automatically registers `ReflectMyTrait`
struct SomeStruct;

impl MyTrait for SomeStruct {
  fn print(&self) -> &str {
    "Hello, World!"
  }
}

// We can create the type data manually if we wanted:
let my_trait: ReflectMyTrait = FromType::<SomeStruct>::from_type();

// Or we can simply get it from the registry:
let mut registry = TypeRegistry::default();
registry.register::<SomeStruct>();
let my_trait = registry
  .get_type_data::<ReflectMyTrait>(TypeId::of::<SomeStruct>())
  .unwrap();

// Then use it on reflected data
let reflected: Box<dyn Reflect> = Box::new(SomeStruct);
let reflected_my_trait: &dyn MyTrait = my_trait.get(&*reflected).unwrap();
assert_eq!("Hello, World!", reflected_my_trait.print());
```