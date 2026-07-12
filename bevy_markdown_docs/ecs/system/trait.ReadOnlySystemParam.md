[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait ReadOnlySystemParam 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#291)

```rust
pub unsafe trait ReadOnlySystemParam: SystemParam { }
```

A [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that only reads a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Safety

This must only be implemented for [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") impls that exclusively read the World passed in to [`SystemParam::get_param`](trait.SystemParam.html#tymethod.get_param "associated function bevy::ecs::system::SystemParam::get_param")

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

### impl [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2087-2093)

### impl<P> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [(P₁, P₂, …, Pₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where P: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

This trait is implemented for tuples up to 17 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1660)

### impl<T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2267)

### impl<T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1702)

### impl<T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

where T: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1282)

### impl [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ExclusiveMarker](struct.ExclusiveMarker.html "struct bevy::ecs::system::ExclusiveMarker")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2612)

### impl [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [FilteredResources](../../prelude/struct.FilteredResources.html "struct bevy::prelude::FilteredResources")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1316)

### impl [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [NonSendMarker](struct.NonSendMarker.html "struct bevy::ecs::system::NonSendMarker")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1589)

### impl [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [SystemChangeTick](struct.SystemChangeTick.html "struct bevy::ecs::system::SystemChangeTick")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_name.rs.html#77)

### impl [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [SystemName](struct.SystemName.html "struct bevy::ecs::system::SystemName")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/identifier.rs.html#52)

### impl [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [WorldId](../world/struct.WorldId.html "struct bevy::ecs::world::WorldId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#387-388)

### impl<'a, 'b, D, F> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Single](../../prelude/struct.Single.html "struct bevy::prelude::Single")<'a, 'b, D, F>

where D: [ReadOnlyQueryData](../query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") + 'static, F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#668)

### impl<'a, T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'a, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1415)

### impl<'a> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for &'a [Archetypes](../archetype/struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1531)

### impl<'a> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for &'a [Bundles](../bundle/struct.Bundles.html "struct bevy::ecs::bundle::Bundles")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1444)

### impl<'a> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for &'a [Components](../component/struct.Components.html "struct bevy::ecs::component::Components")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1473)

### impl<'a> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for &'a [Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1502)

### impl<'a> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for &'a [EntityAllocator](../entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#619)

### impl<'a> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for &'a [RemovedComponentMessages](../lifecycle/struct.RemovedComponentMessages.html "struct bevy::ecs::lifecycle::RemovedComponentMessages")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#976)

### impl<'s, T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, T>

where T: [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#274-279)

### impl<'w, 's, Config, Clear> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Gizmos](../../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos")<'w, 's, Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Deferred](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred")<'s, [GizmoBuffer](../../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [GizmoConfigStore](../../prelude/struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#433-434)

### impl<'w, 's, D, F> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Populated](../../prelude/struct.Populated.html "struct bevy::prelude::Populated")<'w, 's, D, F>

where D: [ReadOnlyQueryData](../query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") + 'static, F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#297-298)

### impl<'w, 's, D, F> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>

where D: [ReadOnlyQueryData](../query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") + 'static, F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#298-299)

### impl<'w, 's, D, F> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ViewQuery](../../render/renderer/struct.ViewQuery.html "struct bevy::render::renderer::ViewQuery")<'w, 's, D, F>

where D: [ReadOnlyQueryData](../query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") + 'static, F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_mutator.rs.html#56)

### impl<'w, 's, M> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [MessageMutator](../../prelude/struct.MessageMutator.html "struct bevy::prelude::MessageMutator")<'w, 's, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"), [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, [MessageCursor](../message/struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor")<M>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, [Messages](../../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_reader.rs.html#33)

### impl<'w, 's, M> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [MessageReader](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader")<'w, 's, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"), [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, [MessageCursor](../message/struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor")<M>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [Messages](../../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_writer.rs.html#61)

### impl<'w, 's, M> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"), [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, [Messages](../../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'w, 's, P0, P1, P2, P3, P4, P5, P6, P7> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4, P5, P6, P7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P1: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P2: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P3: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P4: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P5: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P6: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P7: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'w, 's, P0, P1, P2, P3, P4, P5, P6> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4, P5, P6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P1: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P2: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P3: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P4: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P5: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P6: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'w, 's, P0, P1, P2, P3, P4, P5> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4, P5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P1: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P2: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P3: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P4: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P5: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'w, 's, P0, P1, P2, P3, P4> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P1: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P2: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P3: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P4: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'w, 's, P0, P1, P2, P3> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P1: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P2: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P3: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'w, 's, P0, P1, P2> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P1: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P2: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'w, 's, P0, P1> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), P1: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#665)

### impl<'w, 's, P0> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ParamSet](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2197-2198)

### impl<'w, 's, P> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [StaticSystemParam](struct.StaticSystemParam.html "struct bevy::ecs::system::StaticSystemParam")<'w, 's, P>

where P: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#44)

### impl<'w, 's, R> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [TextReader](../../text/struct.TextReader.html "struct bevy::text::TextReader")<'w, 's, R>

where R: [TextSection](../../text/trait.TextSection.html "trait bevy::text::TextSection"), [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, [TextIterScratch](../../text/struct.TextIterScratch.html "struct bevy::text::TextIterScratch")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, ([&'static R](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &'static [TextFont](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont"), &'static [TextColor](../../prelude/struct.TextColor.html "struct bevy::prelude::TextColor"), &'static [LineHeight](../../text/enum.LineHeight.html "enum bevy::text::LineHeight"), &'static [LetterSpacing](../../text/enum.LetterSpacing.html "enum bevy::text::LetterSpacing"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Children](../../prelude/struct.Children.html "struct bevy::prelude::Children")\>)>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, (&'static [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan"), &'static [TextFont](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont"), &'static [TextColor](../../prelude/struct.TextColor.html "struct bevy::prelude::TextColor"), &'static [LineHeight](../../text/enum.LineHeight.html "enum bevy::text::LineHeight"), &'static [LetterSpacing](../../text/enum.LetterSpacing.html "enum bevy::text::LetterSpacing"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Children](../../prelude/struct.Children.html "struct bevy::prelude::Children")\>)>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#287)

### impl<'w, 's, R> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [TextWriter](../../text/struct.TextWriter.html "struct bevy::text::TextWriter")<'w, 's, R>

where R: [TextSection](../../text/trait.TextSection.html "trait bevy::text::TextSection"), [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, [TextIterScratch](../../text/struct.TextIterScratch.html "struct bevy::text::TextIterScratch")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, ([&'static mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &'static mut [TextFont](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont"), &'static mut [TextColor](../../prelude/struct.TextColor.html "struct bevy::prelude::TextColor"), &'static mut [LineHeight](../../text/enum.LineHeight.html "enum bevy::text::LineHeight"), &'static mut [LetterSpacing](../../text/enum.LetterSpacing.html "enum bevy::text::LetterSpacing")), [Without](../../prelude/struct.Without.html "struct bevy::prelude::Without")<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, (&'static mut [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan"), &'static mut [TextFont](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont"), &'static mut [TextColor](../../prelude/struct.TextColor.html "struct bevy::prelude::TextColor"), &'static mut [LineHeight](../../text/enum.LineHeight.html "enum bevy::text::LineHeight"), &'static mut [LetterSpacing](../../text/enum.LetterSpacing.html "enum bevy::text::LetterSpacing")), [Without](../../prelude/struct.Without.html "struct bevy::prelude::Without")<R>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, &'static [Children](../../prelude/struct.Children.html "struct bevy::prelude::Children")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#749)

### impl<'w, 's, T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ComponentIdFor](../component/struct.ComponentIdFor.html "struct bevy::ecs::component::ComponentIdFor")<'s, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"), [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, InitComponentId<T>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#509)

### impl<'w, 's, T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [RemovedComponents](../../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents")<'w, 's, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"), [ComponentIdFor](../component/struct.ComponentIdFor.html "struct bevy::ecs::component::ComponentIdFor")<'s, T>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, [RemovedComponentReader](../lifecycle/struct.RemovedComponentReader.html "struct bevy::ecs::lifecycle::RemovedComponentReader")<T>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), &'w [RemovedComponentMessages](../lifecycle/struct.RemovedComponentMessages.html "struct bevy::ecs::lifecycle::RemovedComponentMessages"): [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/auto_directional_navigation.rs.html#118)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [AutoDirectionalNavigator](../../ui/auto_directional_navigation/struct.AutoDirectionalNavigator.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigator")<'w, 's>

where [DirectionalNavigation](../../input_focus/directional_navigation/struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation")<'w>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [AutoNavigationConfig](../../input_focus/directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"), &'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")), [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[AutoDirectionalNavigation](../../ui/auto_directional_navigation/struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")), [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[AutoDirectionalNavigation](../../ui/auto_directional_navigation/struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#205-208)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

where [Deferred](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred")<'s, [CommandQueue](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), &'w [Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities"): [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2982)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [DefaultUiCamera](../../prelude/struct.DefaultUiCamera.html "struct bevy::prelude::DefaultUiCamera")<'w, 's>

where [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), &'static [Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera"), &'static [RenderTarget](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget"))>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ([With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>, [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[IsDefaultUiCamera](../../prelude/struct.IsDefaultUiCamera.html "struct bevy::prelude::IsDefaultUiCamera")\>)>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[PrimaryWindow](../../window/struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#347)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Diagnostics](../../diagnostic/struct.Diagnostics.html "struct bevy::diagnostic::Diagnostics")<'w, 's>

where [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [DiagnosticsStore](../../diagnostic/struct.DiagnosticsStore.html "struct bevy::diagnostic::DiagnosticsStore")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Deferred](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred")<'s, DiagnosticsBuffer>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#395)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [DirectionalNavigation](../../input_focus/directional_navigation/struct.DirectionalNavigation.html "struct bevy::input_focus::directional_navigation::DirectionalNavigation")<'w>

where [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, [InputFocus](../../input_focus/struct.InputFocus.html "struct bevy::input_focus::InputFocus")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [DirectionalNavigationMap](../../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#250)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [FallbackImageMsaa](../../render/texture/struct.FallbackImageMsaa.html "struct bevy::render::texture::FallbackImageMsaa")<'w>

where [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, [FallbackImageFormatMsaaCache](../../render/texture/struct.FallbackImageFormatMsaaCache.html "struct bevy::render::texture::FallbackImageFormatMsaaCache")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [RenderDevice](../../render/renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [RenderQueue](../../render/renderer/struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [DefaultImageSampler](../../render/render_resource/struct.DefaultImageSampler.html "struct bevy::render::render_resource::DefaultImageSampler")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#187)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [FlushCommands](../../render/renderer/struct.FlushCommands.html "struct bevy::render::renderer::FlushCommands")<'w>

where [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, [PendingCommandBuffers](../../render/renderer/struct.PendingCommandBuffers.html "struct bevy::render::renderer::PendingCommandBuffers")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [RenderQueue](../../render/renderer/struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#415)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [IsFocusedHelper](../../input_focus/struct.IsFocusedHelper.html "struct bevy::input_focus::IsFocusedHelper")<'w, 's>

where [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, &'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [InputFocus](../../input_focus/struct.InputFocus.html "struct bevy::input_focus::InputFocus")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [InputFocusVisible](../../input_focus/struct.InputFocusVisible.html "struct bevy::input_focus::InputFocusVisible")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#172)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [MeshRayCast](../../prelude/struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast")<'w, 's>

where [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<[Mesh](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<([FloatOrd](../../math/struct.FloatOrd.html "struct bevy::math::FloatOrd"), ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [RayMeshHit](../../picking/mesh_picking/ray_cast/struct.RayMeshHit.html "struct bevy::picking::mesh_picking::ray_cast::RayMeshHit")))>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [RayMeshHit](../../picking/mesh_picking/ray_cast/struct.RayMeshHit.html "struct bevy::picking::mesh_picking::ray_cast::RayMeshHit"))>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Local](../../prelude/struct.Local.html "struct bevy::prelude::Local")<'s, [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<([FloatOrd](../../math/struct.FloatOrd.html "struct bevy::math::FloatOrd"), [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, (&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility"), &'static [ViewVisibility](../../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility"), &'static [Aabb](../../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb"), &'static [GlobalTransform](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform"), [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")), [Or](../../prelude/struct.Or.html "struct bevy::prelude::Or")<([With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Mesh3d](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d")\>, [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Mesh2d](../../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d")\>, [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[SimplifiedMesh](../../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")\>)>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, ([Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Mesh2d](../../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d")\>, [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Mesh3d](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d")\>, [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [SimplifiedMesh](../../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")\>, [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<[RayCastBackfaces](../../prelude/struct.RayCastBackfaces.html "struct bevy::prelude::RayCastBackfaces")\>, &'static [GlobalTransform](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")), [Or](../../prelude/struct.Or.html "struct bevy::prelude::Or")<([With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Mesh3d](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d")\>, [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Mesh2d](../../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d")\>, [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[SimplifiedMesh](../../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")\>)>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/parallel_scope.rs.html#52)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [ParallelCommands](../../prelude/struct.ParallelCommands.html "struct bevy::prelude::ParallelCommands")<'w, 's>

where [Deferred](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred")<'s, ParallelCommandQueue>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), &'w [EntityAllocator](../entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator"): [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), &'w [Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities"): [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#596)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [PickingMessageWriters](../../prelude/struct.PickingMessageWriters.html "struct bevy::prelude::PickingMessageWriters")<'w>

where [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Cancel](../../prelude/struct.Cancel.html "struct bevy::prelude::Cancel")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Click](../../prelude/struct.Click.html "struct bevy::prelude::Click")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Press](../../prelude/struct.Press.html "struct bevy::prelude::Press")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[DragDrop](../../prelude/struct.DragDrop.html "struct bevy::prelude::DragDrop")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[DragEnd](../../prelude/struct.DragEnd.html "struct bevy::prelude::DragEnd")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[DragEnter](../../prelude/struct.DragEnter.html "struct bevy::prelude::DragEnter")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Drag](../../prelude/struct.Drag.html "struct bevy::prelude::Drag")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[DragLeave](../../prelude/struct.DragLeave.html "struct bevy::prelude::DragLeave")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[DragOver](../../prelude/struct.DragOver.html "struct bevy::prelude::DragOver")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[DragStart](../../prelude/struct.DragStart.html "struct bevy::prelude::DragStart")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Scroll](../../prelude/struct.Scroll.html "struct bevy::prelude::Scroll")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Move](../../prelude/struct.Move.html "struct bevy::prelude::Move")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Out](../../prelude/struct.Out.html "struct bevy::prelude::Out")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Over](../../prelude/struct.Over.html "struct bevy::prelude::Over")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Leave](../../prelude/struct.Leave.html "struct bevy::prelude::Leave")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Enter](../../prelude/struct.Enter.html "struct bevy::prelude::Enter")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [MessageWriter](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")<'w, [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<[Release](../../prelude/struct.Release.html "struct bevy::prelude::Release")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#131)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [RenderContext](../../render/renderer/struct.RenderContext.html "struct bevy::render::renderer::RenderContext")<'w, 's>

where [Deferred](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred")<'s, [RenderContextState](../../render/renderer/struct.RenderContextState.html "struct bevy::render::renderer::RenderContextState")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [RenderDevice](../../render/renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, [DiagnosticsRecorder](../../render/diagnostic/struct.DiagnosticsRecorder.html "struct bevy::render::diagnostic::DiagnosticsRecorder")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/helper.rs.html#20)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [TransformHelper](../../prelude/struct.TransformHelper.html "struct bevy::prelude::TransformHelper")<'w, 's>

where [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, &'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, &'static [Transform](../../prelude/struct.Transform.html "struct bevy::prelude::Transform")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#280)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [UiCameraMap](../../ui_render/struct.UiCameraMap.html "struct bevy::ui_render::UiCameraMap")<'w, 's>

where [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#56)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [UiChildren](../../ui/experimental/struct.UiChildren.html "struct bevy::ui::experimental::UiChildren")<'w, 's>

where [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, ([Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Children](../../prelude/struct.Children.html "struct bevy::prelude::Children")\>, [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<[GhostNode](../../ui/experimental/struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")\>), [Or](../../prelude/struct.Or.html "struct bevy::prelude::Or")<([With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Node](../../prelude/struct.Node.html "struct bevy::prelude::Node")\>, [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[GhostNode](../../ui/experimental/struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")\>)>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [Changed](../../prelude/struct.Changed.html "struct bevy::prelude::Changed")<[Children](../../prelude/struct.Children.html "struct bevy::prelude::Children")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, &'static [Children](../../prelude/struct.Children.html "struct bevy::prelude::Children")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[GhostNode](../../ui/experimental/struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, &'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#31)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [UiRootNodes](../../ui/experimental/struct.UiRootNodes.html "struct bevy::ui::experimental::UiRootNodes")<'w, 's>

where [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ([With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Node](../../prelude/struct.Node.html "struct bevy::prelude::Node")\>, [Without](../../prelude/struct.Without.html "struct bevy::prelude::Without")<[ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>)>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ([With](../../prelude/struct.With.html "struct bevy::prelude::With")<[GhostNode](../../ui/experimental/struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")\>, [Without](../../prelude/struct.Without.html "struct bevy::prelude::Without")<[ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>)>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Node](../../prelude/struct.Node.html "struct bevy::prelude::Node")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), [UiChildren](../../ui/experimental/struct.UiChildren.html "struct bevy::ui::experimental::UiChildren")<'w, 's>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#308)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [VisibilityExtractionSystemParam](../../render/view/struct.VisibilityExtractionSystemParam.html "struct bevy::render::view::VisibilityExtractionSystemParam")<'w, 's>

where [Extract](../../render/struct.Extract.html "struct bevy::render::Extract")<'w, 's, [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'static, 'static, &'static [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")\>>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1319)

### impl<'w, T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [NonSend](../../prelude/struct.NonSend.html "struct bevy::prelude::NonSend")<'w, T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#785)

### impl<'w> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_param.rs.html#64)

### impl<P> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Extract](../../render/struct.Extract.html "struct bevy::render::Extract")<'\_, '\_, P>

where P: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1210)

### impl<T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Deferred](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred")<'\_, T>

where T: [SystemBuffer](trait.SystemBuffer.html "trait bevy::ecs::system::SystemBuffer"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1802)

### impl<T> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [If](../../prelude/struct.If.html "struct bevy::prelude::If")<T>

where T: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),