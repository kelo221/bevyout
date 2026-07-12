[bevy](../../../index.html)::[ecs](../../index.html)::[schedule](../index.html)

# Module passes 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/mod.rs.html#22)

Included optional schedule build passes.

## Structs

[AutoInsertApplyDeferredPass](struct.AutoInsertApplyDeferredPass.html "struct bevy::ecs::schedule::passes::AutoInsertApplyDeferredPass")

A [`ScheduleBuildPass`](../trait.ScheduleBuildPass.html "trait bevy::ecs::schedule::ScheduleBuildPass") that inserts [`ApplyDeferred`](../../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred") systems into the schedule graph when there are [`Deferred`](../../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred") in one system and there are ordering dependencies on that system. [`Commands`](../../../prelude/struct.Commands.html "struct bevy::prelude::Commands") is one such deferred buffer.

[IgnoreDeferred](struct.IgnoreDeferred.html "struct bevy::ecs::schedule::passes::IgnoreDeferred")

If added to a dependency edge, the edge will not be considered for auto sync point insertions.