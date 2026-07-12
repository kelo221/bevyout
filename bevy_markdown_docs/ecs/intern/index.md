[bevy](../../index.html)::[ecs](../index.html)

# Module intern 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#39)

Provides types used to statically intern immutable values.

Interning is a pattern used to save memory by deduplicating identical values, speed up code by shrinking the stack size of large types, and make comparisons for any type as fast as integers.

## Structs

[Interned](struct.Interned.html "struct bevy::ecs::intern::Interned")

An interned value. Will stay valid until the end of the program and will not drop.

[Interner](struct.Interner.html "struct bevy::ecs::intern::Interner")

A thread-safe interner which can be used to create [`Interned<T>`](struct.Interned.html "struct bevy::ecs::intern::Interned") from `&T`

## Traits

[Internable](trait.Internable.html "trait bevy::ecs::intern::Internable")

A trait for internable values.