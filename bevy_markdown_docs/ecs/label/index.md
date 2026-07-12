[bevy](../../index.html)::[ecs](../index.html)

# Module label 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#40)

Traits used by label implementations

## Structs

[Box](struct.Box.html "struct bevy::ecs::label::Box")

A pointer type that uniquely owns a heap allocation of type `T`.

## Traits

[DynEq](trait.DynEq.html "trait bevy::ecs::label::DynEq")

An object safe version of [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"). This trait is automatically implemented for any `'static` type that implements `Eq`.

[DynHash](trait.DynHash.html "trait bevy::ecs::label::DynHash")

An object safe version of [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"). This trait is automatically implemented for any `'static` type that implements `Hash`.