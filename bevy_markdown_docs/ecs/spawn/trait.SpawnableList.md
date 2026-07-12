[bevy](../../index.html)::[ecs](../index.html)::[spawn](index.html)

# Trait SpawnableList 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#45)

```rust
pub trait SpawnableList<R>: Sized {
    // Required methods
    fn spawn(this: MovingPtr<'_, Self>, world: &mut World, entity: Entity);
    fn size_hint(&self) -> usize;
}
```

A spawn-able list of changes to a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and relative to a given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"). This is generally used for spawning “related” entities, such as children.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#51)

#### fn [spawn](#tymethod.spawn)(this: [MovingPtr](../ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, Self>, world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

Spawn this list of changes in a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") and relative to a given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"). This is generally used for spawning “related” entities, such as children.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#55)

#### fn [size\_hint](#tymethod.size_hint)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns a size hint, which is used to reserve space for this list in a [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"). This should be less than or equal to the actual size of the list. When in doubt, just use 0.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#278-285)

### impl<R, P> [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R> for [(P₁, P₂, …, Pₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), P: [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R>,

This trait is implemented for tuples up to 13 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#278-285)

#### fn [spawn](#tymethod.spawn)(\_this: [MovingPtr](../ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, \_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), \_entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

where [(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#278-285)

#### fn [size\_hint](#tymethod.size_hint)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#278-285)

### impl<R> [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R> for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#278-285)

#### fn [spawn](#tymethod.spawn)(\_this: [MovingPtr](../ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>, \_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), \_entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

where [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#278-285)

#### fn [size\_hint](#tymethod.size_hint)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#69)

### impl<R, B> [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R> for [Spawn](../../prelude/struct.Spawn.html "struct bevy::prelude::Spawn")<B>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#58)

### impl<R, B> [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R> for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<B>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <B as [DynamicBundle](../bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#157-158)

### impl<R, F> [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R> for [SpawnWith](../../prelude/struct.SpawnWith.html "struct bevy::prelude::SpawnWith")<F>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [RelatedSpawner](../relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner")<'\_, R>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#121-122)

### impl<R, I, B> [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R> for [SpawnIter](../../prelude/struct.SpawnIter.html "struct bevy::prelude::SpawnIter")<I>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = B> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#204)

### impl<R, I> [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R> for [WithRelated](../../prelude/struct.WithRelated.html "struct bevy::prelude::WithRelated")<I>

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/spawn.rs.html#241)

### impl<R> [SpawnableList](trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList")<R> for [WithOneRelated](../../prelude/struct.WithOneRelated.html "struct bevy::prelude::WithOneRelated")

where R: [Relationship](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),