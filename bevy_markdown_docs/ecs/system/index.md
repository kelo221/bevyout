[bevy](../../index.html)::[ecs](../index.html)

# Module system 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#54)

Tools for controlling behavior in an ECS application.

Systems define how an ECS based application behaves. Systems are added to a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"), which is then run. A system is usually written as a normal function, which is automatically converted into a system.

System functions can have parameters, through which one can query and mutate Bevy ECS state. Only types that implement [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") can be used, automatically fetching data from the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

System functions often look like this:

```rust
fn update_score_system(
    mut query: Query<(&Player, &mut Score)>,
    mut round: ResMut<Round>,
) {
    for (player, mut score) in &mut query {
        if player.alive {
            score.0 += round.0;
        }
    }
    round.0 += 1;
}
```

## System ordering

By default, the execution of systems is parallel and not deterministic. Not all systems can run together: if a system mutably accesses data, no other system that reads or writes that data can be run at the same time. These systems are said to be **incompatible**.

The relative order in which incompatible systems are run matters. When this is not specified, a **system order ambiguity** exists in your schedule. You can **explicitly order** systems:

*   by calling the `.before(this_system)` or `.after(that_system)` methods when adding them to your schedule
*   by adding them to a [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet"), and then using `.configure_sets(ThisSet.before(ThatSet))` syntax to configure many systems at once
*   through the use of `.add_systems((system_a, system_b, system_c).chain())`

### Example

```rust
// Configure these systems to run in order using `chain()`.
schedule.add_systems((print_first, print_last).chain());
// Prints "HelloWorld!"
schedule.run(&mut world);

// Configure this system to run in between the other two systems
// using explicit dependencies.
schedule.add_systems(print_mid.after(print_first).before(print_last));
// Prints "Hello, World!"
schedule.run(&mut world);

fn print_first() {
    print!("Hello");
}
fn print_mid() {
    print!(", ");
}
fn print_last() {
    println!("World!");
}
```

## System return type

Systems added to a schedule through [`add_systems`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") may either return empty `()` or a [`Result`](../../prelude/type.Result.html "type bevy::prelude::Result"). Other contexts (like one shot systems) allow systems to return arbitrary values.

## System parameter list

Following is the complete list of accepted types as system parameters:

*   [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query")
*   [`Res`](../../prelude/struct.Res.html "struct bevy::prelude::Res") and `Option<Res>`
*   [`ResMut`](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut") and `Option<ResMut>`
*   [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")
*   [`Local`](../../prelude/struct.Local.html "struct bevy::prelude::Local")
*   [`MessageReader`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader")
*   [`MessageWriter`](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter")
*   [`NonSend`](../../prelude/struct.NonSend.html "struct bevy::prelude::NonSend") and `Option<NonSend>`
*   [`NonSendMut`](../../prelude/struct.NonSendMut.html "struct bevy::prelude::NonSendMut") and `Option<NonSendMut>`
*   [`RemovedComponents`](../../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents")
*   [`SystemName`](struct.SystemName.html "struct bevy::ecs::system::SystemName")
*   [`SystemChangeTick`](struct.SystemChangeTick.html "struct bevy::ecs::system::SystemChangeTick")
*   [`Archetypes`](../archetype/struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes") (Provides Archetype metadata)
*   [`Bundles`](../bundle/struct.Bundles.html "struct bevy::ecs::bundle::Bundles") (Provides Bundles metadata)
*   [`Components`](../component/struct.Components.html "struct bevy::ecs::component::Components") (Provides Components metadata)
*   [`Entities`](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities") (Provides Entities metadata)
*   All tuples between 1 to 16 elements where each element implements [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")
*   [`ParamSet`](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet")
*   [`()` (unit primitive type)](https://doc.rust-lang.org/stable/std/primitive.unit.html)

In addition, the following parameters can be used when constructing a dynamic system with [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder"), but will only provide an empty value when used with an ordinary system:

*   [`FilteredResources`](../../prelude/struct.FilteredResources.html "struct bevy::prelude::FilteredResources")
*   [`FilteredResourcesMut`](../../prelude/struct.FilteredResourcesMut.html "struct bevy::prelude::FilteredResourcesMut")
*   [`DynSystemParam`](struct.DynSystemParam.html "struct bevy::ecs::system::DynSystemParam")
*   [`Vec<P>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") and [`SmallVec<[P, N]>`](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec") where `P: SystemParam`
*   [`ParamSet<Vec<P>>`](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet") where `P: SystemParam`

## Modules

[command](command/index.html "mod bevy::ecs::system::command")

Contains the definition of the [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") trait, as well as the blanket implementation of the trait for closures.

[entity\_command](entity_command/index.html "mod bevy::ecs::system::entity_command")

Contains the definition of the [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") trait, as well as the blanket implementation of the trait for closures.

[lifetimeless](lifetimeless/index.html "mod bevy::ecs::system::lifetimeless")

Contains type aliases for built-in [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")s with `'static` lifetimes. This makes it more convenient to refer to these types in contexts where explicit lifetime annotations are required.

## Structs

[AdapterSystem](struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem")

A [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") that takes the output of `S` and transforms it by applying `Func` to it.

[BuilderSystem](struct.BuilderSystem.html "struct bevy::ecs::system::BuilderSystem")

A [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") created from a [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") whose state is not initialized until the first run.

[CachedSystemId](struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId")

A cached [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") distinguished by the unique function type of its system.

[CombinatorSystem](struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")

A [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") defined by combining two other systems. The behavior of this combinator is specified by implementing the [`Combine`](trait.Combine.html "trait bevy::ecs::system::Combine") trait. For a full usage example, see the docs for [`Combine`](trait.Combine.html "trait bevy::ecs::system::Combine").

[Commands](struct.Commands.html "struct bevy::ecs::system::Commands")

A [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") queue to perform structural changes to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Deferred](struct.Deferred.html "struct bevy::ecs::system::Deferred")

A [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that stores a buffer which gets applied to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") during [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred"). This is used internally by [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") to defer `World` mutations.

[DynParamBuilder](struct.DynParamBuilder.html "struct bevy::ecs::system::DynParamBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for a [`DynSystemParam`](struct.DynSystemParam.html "struct bevy::ecs::system::DynSystemParam"). See the [`DynSystemParam`](struct.DynSystemParam.html "struct bevy::ecs::system::DynSystemParam") docs for examples.

[DynSystemParam](struct.DynSystemParam.html "struct bevy::ecs::system::DynSystemParam")

A [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") with a type that can be configured at runtime.

[DynSystemParamState](struct.DynSystemParamState.html "struct bevy::ecs::system::DynSystemParamState")

The [`SystemParam::State`](trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State") for a [`DynSystemParam`](struct.DynSystemParam.html "struct bevy::ecs::system::DynSystemParam").

[EntityCommands](struct.EntityCommands.html "struct bevy::ecs::system::EntityCommands")

A list of commands that will be run to modify an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[EntityEntryCommands](struct.EntityEntryCommands.html "struct bevy::ecs::system::EntityEntryCommands")

A wrapper around [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") with convenience methods for working with a specified component type.

[ExclusiveFunctionSystem](struct.ExclusiveFunctionSystem.html "struct bevy::ecs::system::ExclusiveFunctionSystem")

A function system that runs with exclusive [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") access.

[ExclusiveMarker](struct.ExclusiveMarker.html "struct bevy::ecs::system::ExclusiveMarker")

A dummy type to tell the executor to run the system exclusively.

[FilteredResourcesMutParamBuilder](struct.FilteredResourcesMutParamBuilder.html "struct bevy::ecs::system::FilteredResourcesMutParamBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for a [`FilteredResourcesMut`](../../prelude/struct.FilteredResourcesMut.html "struct bevy::prelude::FilteredResourcesMut"). See the [`FilteredResourcesMut`](../../prelude/struct.FilteredResourcesMut.html "struct bevy::prelude::FilteredResourcesMut") docs for examples.

[FilteredResourcesParamBuilder](struct.FilteredResourcesParamBuilder.html "struct bevy::ecs::system::FilteredResourcesParamBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for a [`FilteredResources`](../../prelude/struct.FilteredResources.html "struct bevy::prelude::FilteredResources"). See the [`FilteredResources`](../../prelude/struct.FilteredResources.html "struct bevy::prelude::FilteredResources") docs for examples.

[FunctionSystem](struct.FunctionSystem.html "struct bevy::ecs::system::FunctionSystem")

The [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") counter part of an ordinary function.

[If](struct.If.html "struct bevy::ecs::system::If")

A [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that wraps another parameter and causes its system to skip instead of failing when the parameter is invalid.

[IfBuilder](struct.IfBuilder.html "struct bevy::ecs::system::IfBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for a [`If`](../../prelude/struct.If.html "struct bevy::prelude::If").

[In](struct.In.html "struct bevy::ecs::system::In")

A [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") type which denotes that a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") receives an input value of type `T` from its caller.

[InMut](struct.InMut.html "struct bevy::ecs::system::InMut")

A [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") type which denotes that a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") receives a mutable reference to a value of type `T` from its caller.

[InRef](struct.InRef.html "struct bevy::ecs::system::InRef")

A [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") type which denotes that a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") receives a read-only reference to a value of type `T` from its caller.

[IntoAdapterSystem](struct.IntoAdapterSystem.html "struct bevy::ecs::system::IntoAdapterSystem")

An [`IntoSystem`](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem") creating an instance of [`AdapterSystem`](struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem").

[IntoBuilderSystem](struct.IntoBuilderSystem.html "struct bevy::ecs::system::IntoBuilderSystem")

An [`IntoSystem`](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem") creating an instance of [`BuilderSystem`](struct.BuilderSystem.html "struct bevy::ecs::system::BuilderSystem")

[IntoPipeSystem](struct.IntoPipeSystem.html "struct bevy::ecs::system::IntoPipeSystem")

An [`IntoSystem`](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem") creating an instance of [`PipeSystem`](struct.PipeSystem.html "struct bevy::ecs::system::PipeSystem").

[Local](struct.Local.html "struct bevy::ecs::system::Local")

A [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides a system-private value of `T` that persists across system calls.

[LocalBuilder](struct.LocalBuilder.html "struct bevy::ecs::system::LocalBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for a [`Local`](../../prelude/struct.Local.html "struct bevy::prelude::Local"). The provided value will be used as the initial value of the `Local`.

[NonSend](struct.NonSend.html "struct bevy::ecs::system::NonSend")

Shared borrow of a non-[`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource.

[NonSendMarker](struct.NonSendMarker.html "struct bevy::ecs::system::NonSendMarker")

A dummy type that is [`!Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"), to force systems to run on the main thread.

[NonSendMut](struct.NonSendMut.html "struct bevy::ecs::system::NonSendMut")

Unique borrow of a non-[`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource.

[OptionBuilder](struct.OptionBuilder.html "struct bevy::ecs::system::OptionBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

[ParallelCommands](struct.ParallelCommands.html "struct bevy::ecs::system::ParallelCommands")

An alternative to [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") that can be used in parallel contexts, such as those in [`Query::par_iter`](../../prelude/struct.Query.html#method.par_iter "method bevy::prelude::Query::par_iter").

[ParamBuilder](struct.ParamBuilder.html "struct bevy::ecs::system::ParamBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for any [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that uses its default initialization.

[ParamSet](struct.ParamSet.html "struct bevy::ecs::system::ParamSet")

A collection of potentially conflicting [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")s allowed by disjoint access.

[ParamSetBuilder](struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for a [`ParamSet`](../../prelude/struct.ParamSet.html "struct bevy::prelude::ParamSet").

[PipeSystem](struct.PipeSystem.html "struct bevy::ecs::system::PipeSystem")

A [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") created by piping the output of the first system into the input of the second.

[Populated](struct.Populated.html "struct bevy::ecs::system::Populated")

[System parameter](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that works very much like [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") except it always contains at least one matching entity.

[Query](struct.Query.html "struct bevy::ecs::system::Query")

A [system parameter](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides selective access to the [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") data stored in a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[QueryLens](struct.QueryLens.html "struct bevy::ecs::system::QueryLens")

Type returned from [`Query::transmute_lens`](../../prelude/struct.Query.html#method.transmute_lens "method bevy::prelude::Query::transmute_lens") containing the new [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

[QueryParamBuilder](struct.QueryParamBuilder.html "struct bevy::ecs::system::QueryParamBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query"). This takes a closure accepting an `&mut` [`QueryBuilder`](../../prelude/struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder") and uses the builder to construct the query’s state. This can be used to add additional filters, or to configure the components available to [`FilteredEntityRef`](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef") or [`FilteredEntityMut`](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut").

[RegisteredSystemDespawner](struct.RegisteredSystemDespawner.html "struct bevy::ecs::system::RegisteredSystemDespawner")

A resource that stores the channel for despawning unused registered system entities.

[RemovedSystem](struct.RemovedSystem.html "struct bevy::ecs::system::RemovedSystem")

A system that has been removed from the registry. It contains the system and whether or not it has been initialized.

[Res](struct.Res.html "struct bevy::ecs::system::Res")

Shared borrow of a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource").

[ResMut](struct.ResMut.html "struct bevy::ecs::system::ResMut")

Unique mutable borrow of a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource").

[ResultBuilder](struct.ResultBuilder.html "struct bevy::ecs::system::ResultBuilder")

A [`SystemParamBuilder`](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") for a [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") of [`SystemParamValidationError`](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError").

[Single](struct.Single.html "struct bevy::ecs::system::Single")

[System parameter](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides access to single entity’s components, much like [`Query::single`](../../prelude/struct.Query.html#method.single "method bevy::prelude::Query::single")/[`Query::single_mut`](../../prelude/struct.Query.html#method.single_mut "method bevy::prelude::Query::single_mut").

[StaticSystemInput](struct.StaticSystemInput.html "struct bevy::ecs::system::StaticSystemInput")

A helper for using [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")s in generic contexts.

[StaticSystemParam](struct.StaticSystemParam.html "struct bevy::ecs::system::StaticSystemParam")

A helper for using system parameters in generic contexts

[StrongSystemHandle](struct.StrongSystemHandle.html "struct bevy::ecs::system::StrongSystemHandle")

A strong handle for a registered system that despawns the entity when dropped.

[SystemChangeTick](struct.SystemChangeTick.html "struct bevy::ecs::system::SystemChangeTick")

A [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that reads the previous and current change ticks of the system.

[SystemHandleValue](struct.SystemHandleValue.html "struct bevy::ecs::system::SystemHandleValue")

Stores an [`Arc<Mutex<SystemHandleOrValue<I, O>>>`](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc").

[SystemId](struct.SystemId.html "struct bevy::ecs::system::SystemId")

An identifier for a registered system.

[SystemIdMarker](struct.SystemIdMarker.html "struct bevy::ecs::system::SystemIdMarker")

Marker [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") for identifying [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s.

[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta")

The metadata of a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

[SystemName](struct.SystemName.html "struct bevy::ecs::system::SystemName")

[`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that returns the name of the system which it is used in.

[SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")

An error that occurs when a system parameter is not valid, used by system executors to determine what to do with a system.

[SystemState](struct.SystemState.html "struct bevy::ecs::system::SystemState")

Holds on to persistent state required to drive [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

[SystemStateFlags](struct.SystemStateFlags.html "struct bevy::ecs::system::SystemStateFlags")

Bitflags representing system states and requirements.

[WithInputFromWrapper](struct.WithInputFromWrapper.html "struct bevy::ecs::system::WithInputFromWrapper")

Constructed in [`IntoSystem::with_input_from`](../../prelude/trait.IntoSystem.html#method.with_input_from "method bevy::prelude::IntoSystem::with_input_from").

[WithInputWrapper](struct.WithInputWrapper.html "struct bevy::ecs::system::WithInputWrapper")

See [`IntoSystem::with_input`](../../prelude/trait.IntoSystem.html#method.with_input "method bevy::prelude::IntoSystem::with_input") for details.

## Enums

[RegisteredSystemError](enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError")

An operation with stored systems failed.

[RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")

Running system failed.

[SystemHandle](enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle")

A maybe-strong handle to an entity acting as a registered system. Strong handles are created by [`World::register_tracked_system`](../../prelude/struct.World.html#method.register_tracked_system "method bevy::prelude::World::register_tracked_system") or [`World::register_tracked_boxed_system`](../../prelude/struct.World.html#method.register_tracked_boxed_system "method bevy::prelude::World::register_tracked_boxed_system").

[SystemHandleTemplate](enum.SystemHandleTemplate.html "enum bevy::ecs::system::SystemHandleTemplate")

A [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") that produces a [`SystemHandle`](enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle").

## Traits

[Adapt](trait.Adapt.html "trait bevy::ecs::system::Adapt")

Customizes the behavior of an [`AdapterSystem`](struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem")

[Combine](trait.Combine.html "trait bevy::ecs::system::Combine")

Customizes the behavior of a [`CombinatorSystem`](struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem").

[Command](trait.Command.html "trait bevy::ecs::system::Command")

A [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") mutation.

[EntityCommand](trait.EntityCommand.html "trait bevy::ecs::system::EntityCommand")

A command which gets executed for a given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[ExclusiveSystemParam](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam")

A parameter that can be used in an exclusive system (a system with an `&mut World` parameter). Any parameters implementing this trait must come after the `&mut World` parameter.

[ExclusiveSystemParamFunction](trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction")

A trait implemented for all exclusive system functions that can be used as [`System`](../../prelude/trait.System.html "trait bevy::prelude::System")s.

[FromInput](trait.FromInput.html "trait bevy::ecs::system::FromInput")

A type that may be constructed from the input of a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System"). This is used to allow systems whose first parameter is a `StaticSystemInput<In>` to take an `In` as input, and can be implemented for user types to allow similar conversions.

[IntoObserverSystem](trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem")

Implemented for systems that convert into [`ObserverSystem`](trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem").

[IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")

A type that may be converted to the output of a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System"). This is used to allow systems to return either a plain value or a [`Result`](../../prelude/type.Result.html "type bevy::prelude::Result").

[IntoSystem](trait.IntoSystem.html "trait bevy::ecs::system::IntoSystem")

Conversion trait to turn something into a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

[ObserverSystem](trait.ObserverSystem.html "trait bevy::ecs::system::ObserverSystem")

Implemented for [`System`](../../prelude/trait.System.html "trait bevy::prelude::System")s that have [`On`](../../prelude/struct.On.html "struct bevy::prelude::On") as the first argument.

[ReadOnlySystem](trait.ReadOnlySystem.html "trait bevy::ecs::system::ReadOnlySystem")

[`System`](../../prelude/trait.System.html "trait bevy::prelude::System") types that do not modify the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") when run. This is implemented for any systems whose parameters all implement [`ReadOnlySystemParam`](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam").

[ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam")

A [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that only reads a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[RunSystemOnce](trait.RunSystemOnce.html "trait bevy::ecs::system::RunSystemOnce")

Trait used to run a system immediately on a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[System](trait.System.html "trait bevy::ecs::system::System")

An ECS system that can be added to a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")

[SystemBuffer](trait.SystemBuffer.html "trait bevy::ecs::system::SystemBuffer")

Types that can be used with [`Deferred<T>`](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred") in systems. This allows storing system-local data which is used to defer [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") mutations.

[SystemInput](trait.SystemInput.html "trait bevy::ecs::system::SystemInput")

Trait for types that can be used as input to [`System`](../../prelude/trait.System.html "trait bevy::prelude::System")s.

[SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")

A parameter that can be used in a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

[SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::ecs::system::SystemParamBuilder")

A builder that can create a [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

[SystemParamFunction](trait.SystemParamFunction.html "trait bevy::ecs::system::SystemParamFunction")

A trait implemented for all functions that can be used as [`System`](../../prelude/trait.System.html "trait bevy::prelude::System")s.

## Functions

[assert\_is\_read\_only\_system](fn.assert_is_read_only_system.html "fn bevy::ecs::system::assert_is_read_only_system")

Ensure that a given function is a [read-only system](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem").

[assert\_is\_system](fn.assert_is_system.html "fn bevy::ecs::system::assert_is_system")

Ensure that a given function is a [system](../../prelude/trait.System.html "trait bevy::prelude::System").

[assert\_system\_does\_not\_conflict](fn.assert_system_does_not_conflict.html "fn bevy::ecs::system::assert_system_does_not_conflict")

Ensures that the provided system doesn’t conflict with itself.

[despawn\_unused\_registered\_systems](fn.despawn_unused_registered_systems.html "fn bevy::ecs::system::despawn_unused_registered_systems")

A system that despawns any registered system entities whose [`SystemHandle`](enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle") reference count has reached zero.

[system\_value](fn.system_value.html "fn bevy::ecs::system::system_value")

This will create a new [`SystemHandleTemplate`](enum.SystemHandleTemplate.html "enum bevy::ecs::system::SystemHandleTemplate") for the given `system` value. This makes it possible to define systems “inline” in templates / scenes that produce a [`SystemHandle`](enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle").

## Type Aliases

[BoxedReadOnlySystem](type.BoxedReadOnlySystem.html "type bevy::ecs::system::BoxedReadOnlySystem")

A convenience type alias for a boxed [`ReadOnlySystem`](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem") trait object.

[BoxedSystem](type.BoxedSystem.html "type bevy::ecs::system::BoxedSystem")

A convenience type alias for a boxed [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") trait object.

[ExclusiveSystemParamItem](type.ExclusiveSystemParamItem.html "type bevy::ecs::system::ExclusiveSystemParamItem")

Shorthand way of accessing the associated type [`ExclusiveSystemParam::Item`](trait.ExclusiveSystemParam.html#associatedtype.Item "associated type bevy::ecs::system::ExclusiveSystemParam::Item") for a given [`ExclusiveSystemParam`](trait.ExclusiveSystemParam.html "trait bevy::ecs::system::ExclusiveSystemParam").

[ScheduleSystem](type.ScheduleSystem.html "type bevy::ecs::system::ScheduleSystem")

Type alias for a `BoxedSystem` that a `Schedule` can store.

[SystemIn](type.SystemIn.html "type bevy::ecs::system::SystemIn")

Shorthand way to get the [`System::In`](../../prelude/trait.System.html#associatedtype.In "associated type bevy::prelude::System::In") for a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") as a [`SystemInput::Inner`](../../prelude/trait.SystemInput.html#associatedtype.Inner "associated type bevy::prelude::SystemInput::Inner").

[SystemParamItem](type.SystemParamItem.html "type bevy::ecs::system::SystemParamItem")

Shorthand way of accessing the associated type [`SystemParam::Item`](trait.SystemParam.html#associatedtype.Item "associated type bevy::ecs::system::SystemParam::Item") for a given [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

## Derive Macros

[SystemParam](derive.SystemParam.html "derive bevy::ecs::system::SystemParam")

Implement `SystemParam` to use a struct as a parameter in a system