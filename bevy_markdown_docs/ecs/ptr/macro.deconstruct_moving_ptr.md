[bevy](../../index.html)::[ecs](../index.html)::[ptr](index.html)

# Macro deconstruct\_moving\_ptr 

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1454)

```rust
macro_rules! deconstruct_moving_ptr {
    ({ let tuple { $($field_index:tt: $pattern:pat),* $(,)? } = $ptr:expr ;}) => { ... };
    ({ let MaybeUninit::<tuple> { $($field_index:tt: $pattern:pat),* $(,)? } = $ptr:expr ;}) => { ... };
    ({ let $struct_name:ident { $($field_index:tt$(: $pattern:pat)?),* $(,)? } = $ptr:expr ;}) => { ... };
    ({ let MaybeUninit::<$struct_name:ident> { $($field_index:tt$(: $pattern:pat)?),* $(,)? } = $ptr:expr ;}) => { ... };
}
```

Deconstructs a [`MovingPtr`](struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr") into its individual fields.

This consumes the [`MovingPtr`](struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr") and hands out [`MovingPtr`](struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr") wrappers around pointers to each of its fields. The value will _not_ be dropped.

The macro should wrap a `let` expression with a struct pattern. It does not support matching tuples by position, so for tuple structs you should use `0: pat` syntax.

For tuples themselves, pass the identifier `tuple` instead of the struct name, like `let tuple { 0: pat0, 1: pat1 } = value`.

This can also project into `MaybeUninit`. Wrap the type name or `tuple` with `MaybeUninit::<_>`, and the macro will deconstruct a `MovingPtr<MaybeUninit<ParentType>>` into `MovingPtr<MaybeUninit<FieldType>>` values.

## Examples

### Structs

```rust
use core::mem::{offset_of, MaybeUninit};
use bevy_ptr::{MovingPtr, move_as_ptr};


let parent = Parent {
  field_a: FieldAType(11),
  field_b: FieldBType(22),
  field_c: FieldCType(33),
};

let mut target_a = FieldAType(101);
let mut target_b = FieldBType(102);
let mut target_c = FieldCType(103);

// Converts `parent` into a `MovingPtr`
move_as_ptr!(parent);

// The field names must match the name used in the type definition.
// Each one will be a `MovingPtr` of the field's type.
bevy_ptr::deconstruct_moving_ptr!({
  let Parent { field_a, field_b, field_c } = parent;
});

field_a.assign_to(&mut target_a);
field_b.assign_to(&mut target_b);
field_c.assign_to(&mut target_c);

assert_eq!(target_a.0, 11);
assert_eq!(target_b.0, 22);
assert_eq!(target_c.0, 33);
```

### Tuples

```rust
use core::mem::{offset_of, MaybeUninit};
use bevy_ptr::{MovingPtr, move_as_ptr};


let parent = (
  FieldAType(11),
  FieldBType(22),
  FieldCType(33),
);

let mut target_a = FieldAType(101);
let mut target_b = FieldBType(102);
let mut target_c = FieldCType(103);

// Converts `parent` into a `MovingPtr`
move_as_ptr!(parent);

// The field names must match the name used in the type definition.
// Each one will be a `MovingPtr` of the field's type.
bevy_ptr::deconstruct_moving_ptr!({
  let tuple { 0: field_a, 1: field_b, 2: field_c } = parent;
});

field_a.assign_to(&mut target_a);
field_b.assign_to(&mut target_b);
field_c.assign_to(&mut target_c);

assert_eq!(target_a.0, 11);
assert_eq!(target_b.0, 22);
assert_eq!(target_c.0, 33);
```

### `MaybeUninit`

```rust
use core::mem::{offset_of, MaybeUninit};
use bevy_ptr::{MovingPtr, move_as_ptr};


let parent = MaybeUninit::new(Parent {
  field_a: FieldAType(11),
  field_b: FieldBType(22),
  field_c: FieldCType(33),
});

let mut target_a = MaybeUninit::new(FieldAType(101));
let mut target_b = MaybeUninit::new(FieldBType(102));
let mut target_c = MaybeUninit::new(FieldCType(103));

// Converts `parent` into a `MovingPtr`
move_as_ptr!(parent);

// The field names must match the name used in the type definition.
// Each one will be a `MovingPtr` of the field's type.
bevy_ptr::deconstruct_moving_ptr!({
  let MaybeUninit::<Parent> { field_a, field_b, field_c } = parent;
});

field_a.assign_to(&mut target_a);
field_b.assign_to(&mut target_b);
field_c.assign_to(&mut target_c);

unsafe {
  assert_eq!(target_a.assume_init().0, 11);
  assert_eq!(target_b.assume_init().0, 22);
  assert_eq!(target_c.assume_init().0, 33);
}
```