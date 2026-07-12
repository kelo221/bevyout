[bevy](../../index.html)::[ecs](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#67)

The ECS prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Macros

[children](macro.children.html "macro bevy::ecs::prelude::children")

Returns a [`SpawnRelatedBundle`](../spawn/struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle") that will insert the [`Children`](../../prelude/struct.Children.html "struct bevy::prelude::Children") component, spawn a [`SpawnableList`](../spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") of entities with given bundles that relate to the [`Children`](../../prelude/struct.Children.html "struct bevy::prelude::Children") entity via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") component, and reserve space in the [`Children`](../../prelude/struct.Children.html "struct bevy::prelude::Children") for each spawned entity.

[related](macro.related.html "macro bevy::ecs::prelude::related")

Returns a [`SpawnRelatedBundle`](../spawn/struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle") that will insert the given [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"), spawn a [`SpawnableList`](../spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") of entities with given bundles that relate to the [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") entity via the [`RelationshipTarget::Relationship`](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship") component, and reserve space in the [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") for each spawned entity.

## Structs

[Add](struct.Add.html "struct bevy::ecs::prelude::Add")

Trigger emitted when a component is inserted onto an entity that does not already have that component. Runs before `Insert`. See [`ComponentHooks::on_add`](../lifecycle/struct.ComponentHooks.html#method.on_add "method bevy::ecs::lifecycle::ComponentHooks::on_add") for more information.

[Added](struct.Added.html "struct bevy::ecs::prelude::Added")

A filter on a component that only retains results the first time after they have been added.

[Allow](struct.Allow.html "struct bevy::ecs::prelude::Allow")

Allows a query to contain entities with the component `T`, bypassing [`DefaultQueryFilters`](../entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters").

[AnyOf](struct.AnyOf.html "struct bevy::ecs::prelude::AnyOf")

The `AnyOf` query parameter fetches entities with any of the component types included in T.

[AppFunctionRegistry](struct.AppFunctionRegistry.html "struct bevy::ecs::prelude::AppFunctionRegistry")`reflect_functions`

A [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") storing [`FunctionRegistry`](../../reflect/func/struct.FunctionRegistry.html "struct bevy::reflect::func::FunctionRegistry") for function registrations relevant to a whole app.

[AppTypeRegistry](struct.AppTypeRegistry.html "struct bevy::ecs::prelude::AppTypeRegistry")

A [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") storing [`TypeRegistry`](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry") for type registrations relevant to a whole app.

[ApplyDeferred](struct.ApplyDeferred.html "struct bevy::ecs::prelude::ApplyDeferred")

A special [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") that instructs the executor to call [`System::apply_deferred`](../../prelude/trait.System.html#tymethod.apply_deferred "method bevy::prelude::System::apply_deferred") on the systems that have run but not applied their [`Deferred`](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred") system parameters (like [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")) or other system buffers.

[BevyError](struct.BevyError.html "struct bevy::ecs::prelude::BevyError")

The built in “universal” Bevy error type. This has a blanket [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") impl for any type that implements Rust’s [`Error`](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error"), meaning it can be used as a “catch all” error.

[Changed](struct.Changed.html "struct bevy::ecs::prelude::Changed")

A filter on a component that only retains results the first time after they have been added or mutably dereferenced.

[ChildOf](struct.ChildOf.html "struct bevy::ecs::prelude::ChildOf")

Stores the parent entity of this child entity with this component.

[Children](struct.Children.html "struct bevy::ecs::prelude::Children")

Tracks which entities are children of this parent entity.

[Commands](struct.Commands.html "struct bevy::ecs::prelude::Commands")

A [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") queue to perform structural changes to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[ContiguousMut](struct.ContiguousMut.html "struct bevy::ecs::prelude::ContiguousMut")

Data type returned by [`ContiguousQueryData::fetch_contiguous`](../query/trait.ContiguousQueryData.html#tymethod.fetch_contiguous "associated function bevy::ecs::query::ContiguousQueryData::fetch_contiguous") for [`Mut<T>`](../../prelude/struct.Mut.html "struct bevy::prelude::Mut") and `&mut T`

[ContiguousRef](struct.ContiguousRef.html "struct bevy::ecs::prelude::ContiguousRef")

Contiguous equivalent of [`Ref<T>`](../../prelude/struct.Ref.html "struct bevy::prelude::Ref").

[Deferred](struct.Deferred.html "struct bevy::ecs::prelude::Deferred")

A [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that stores a buffer which gets applied to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") during [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred"). This is used internally by [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") to defer `World` mutations.

[Despawn](struct.Despawn.html "struct bevy::ecs::prelude::Despawn")

[`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") emitted for each component on an entity when it is despawned. See [`ComponentHooks::on_despawn`](../lifecycle/struct.ComponentHooks.html#method.on_despawn "method bevy::ecs::lifecycle::ComponentHooks::on_despawn") for more information.

[Discard](struct.Discard.html "struct bevy::ecs::prelude::Discard")

Trigger emitted when a component is removed from an entity, regardless of whether or not it is later replaced.

[Entity](struct.Entity.html "struct bevy::ecs::prelude::Entity")

Unique identifier for an entity in a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). Note that this is just an id, not the entity itself. Further, the entity this id refers to may no longer exist in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). For more information about entities, their ids, and how to use them, see the module [docs](../entity/index.html "mod bevy::ecs::entity").

[EntityCommands](struct.EntityCommands.html "struct bevy::ecs::prelude::EntityCommands")

A list of commands that will be run to modify an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[EntityMut](struct.EntityMut.html "struct bevy::ecs::prelude::EntityMut")

Provides mutable access to a single entity and all of its components.

[EntityRef](struct.EntityRef.html "struct bevy::ecs::prelude::EntityRef")

A read-only reference to a particular [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") and all of its components.

[EntityWorldMut](struct.EntityWorldMut.html "struct bevy::ecs::prelude::EntityWorldMut")

A mutable reference to a particular [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), and the entire world.

[FilteredResources](struct.FilteredResources.html "struct bevy::ecs::prelude::FilteredResources")

Provides read-only access to a set of [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")s defined by the contained [`Access`](../query/struct.Access.html "struct bevy::ecs::query::Access").

[FilteredResourcesMut](struct.FilteredResourcesMut.html "struct bevy::ecs::prelude::FilteredResourcesMut")

Provides mutable access to a set of [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")s defined by the contained [`Access`](../query/struct.Access.html "struct bevy::ecs::query::Access").

[Has](struct.Has.html "struct bevy::ecs::prelude::Has")

Returns a bool that describes if an entity has the component `T`.

[If](struct.If.html "struct bevy::ecs::prelude::If")

A [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that wraps another parameter and causes its system to skip instead of failing when the parameter is invalid.

[In](struct.In.html "struct bevy::ecs::prelude::In")

A [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") type which denotes that a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") receives an input value of type `T` from its caller.

[InMut](struct.InMut.html "struct bevy::ecs::prelude::InMut")

A [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") type which denotes that a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") receives a mutable reference to a value of type `T` from its caller.

[InRef](struct.InRef.html "struct bevy::ecs::prelude::InRef")

A [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") type which denotes that a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") receives a read-only reference to a value of type `T` from its caller.

[Insert](struct.Insert.html "struct bevy::ecs::prelude::Insert")

Trigger emitted when a component is inserted, regardless of whether or not the entity already had that component. Runs after `Add`, if it ran. See [`ComponentHooks::on_insert`](../lifecycle/struct.ComponentHooks.html#method.on_insert "method bevy::ecs::lifecycle::ComponentHooks::on_insert") for more information.

[Local](struct.Local.html "struct bevy::ecs::prelude::Local")

A [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides a system-private value of `T` that persists across system calls.

[MessageMutator](struct.MessageMutator.html "struct bevy::ecs::prelude::MessageMutator")

Reads and writes [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s of type `T`, keeping track of which messages have already been read.

[MessageReader](struct.MessageReader.html "struct bevy::ecs::prelude::MessageReader")

Reads [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s of type `T` in order and tracks which messages have already been read.

[MessageWriter](struct.MessageWriter.html "struct bevy::ecs::prelude::MessageWriter")

Writes [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s of type `T`.

[Messages](struct.Messages.html "struct bevy::ecs::prelude::Messages")

A message collection that represents the messages that occurred within the last two [`Messages::update`](../../prelude/struct.Messages.html#method.update "method bevy::prelude::Messages::update") calls. Messages can be written to using a [`MessageWriter`](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter") and are typically cheaply read using a [`MessageReader`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader").

[Mut](struct.Mut.html "struct bevy::ecs::prelude::Mut")

Unique mutable borrow of an entity’s component or of a resource.

[Name](struct.Name.html "struct bevy::ecs::prelude::Name")

Component used to identify an entity. Stores a hash for faster comparisons.

[NameOrEntity](struct.NameOrEntity.html "struct bevy::ecs::prelude::NameOrEntity")

Convenient query for giving a human friendly name to an entity.

[NonSend](struct.NonSend.html "struct bevy::ecs::prelude::NonSend")

Shared borrow of a non-[`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource.

[NonSendMut](struct.NonSendMut.html "struct bevy::ecs::prelude::NonSendMut")

Unique borrow of a non-[`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource.

[Observer](struct.Observer.html "struct bevy::ecs::prelude::Observer")

An [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") system. Add this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to turn it into an “observer”.

[On](struct.On.html "struct bevy::ecs::prelude::On")

A [system parameter](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") used by an observer to process events. See [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") and [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") for examples.

[Or](struct.Or.html "struct bevy::ecs::prelude::Or")

A filter that tests if any of the given filters apply.

[ParallelCommands](struct.ParallelCommands.html "struct bevy::ecs::prelude::ParallelCommands")

An alternative to [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") that can be used in parallel contexts, such as those in [`Query::par_iter`](../../prelude/struct.Query.html#method.par_iter "method bevy::prelude::Query::par_iter").

[ParamSet](struct.ParamSet.html "struct bevy::ecs::prelude::ParamSet")

A collection of potentially conflicting [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")s allowed by disjoint access.

[Populated](struct.Populated.html "struct bevy::ecs::prelude::Populated")

[System parameter](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that works very much like [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") except it always contains at least one matching entity.

[PopulatedMessageReader](struct.PopulatedMessageReader.html "struct bevy::ecs::prelude::PopulatedMessageReader")

Reads [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s of type `T` in order and tracks which messages have already been read. Skips the system if there no messages.

[Query](struct.Query.html "struct bevy::ecs::prelude::Query")

A [system parameter](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides selective access to the [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") data stored in a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[QueryBuilder](struct.QueryBuilder.html "struct bevy::ecs::prelude::QueryBuilder")

Builder struct to create [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") instances at runtime.

[QueryState](struct.QueryState.html "struct bevy::ecs::prelude::QueryState")

Provides scoped access to a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") state according to a given [`QueryData`](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData") and [`QueryFilter`](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter").

[Ref](struct.Ref.html "struct bevy::ecs::prelude::Ref")

Shared borrow of an entity’s component with access to change detection. Similar to [`Mut`](../../prelude/struct.Mut.html "struct bevy::prelude::Mut") but is immutable and so doesn’t require unique access.

[ReflectComponent](struct.ReflectComponent.html "struct bevy::ecs::prelude::ReflectComponent")

A struct used to operate on reflected [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") trait of a type.

[ReflectEvent](struct.ReflectEvent.html "struct bevy::ecs::prelude::ReflectEvent")

A struct used to operate on reflected [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") trait of a type.

[ReflectFromWorld](struct.ReflectFromWorld.html "struct bevy::ecs::prelude::ReflectFromWorld")

A struct used to operate on the reflected [`FromWorld`](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") trait of a type.

[ReflectMessage](struct.ReflectMessage.html "struct bevy::ecs::prelude::ReflectMessage")

A struct used to operate on reflected [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") trait of a type.

[ReflectResource](struct.ReflectResource.html "struct bevy::ecs::prelude::ReflectResource")

A struct that marks a reflected [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") of a type.

[Remove](struct.Remove.html "struct bevy::ecs::prelude::Remove")

Trigger emitted when a component is removed from an entity, and runs before the component is removed, so you can still access the component data. See [`ComponentHooks::on_remove`](../lifecycle/struct.ComponentHooks.html#method.on_remove "method bevy::ecs::lifecycle::ComponentHooks::on_remove") for more information.

[RemovedComponents](struct.RemovedComponents.html "struct bevy::ecs::prelude::RemovedComponents")

A [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that yields entities that had their `T` [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") removed or have been despawned with it.

[Res](struct.Res.html "struct bevy::ecs::prelude::Res")

Shared borrow of a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource").

[ResMut](struct.ResMut.html "struct bevy::ecs::prelude::ResMut")

Unique mutable borrow of a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource").

[Schedule](struct.Schedule.html "struct bevy::ecs::prelude::Schedule")

A collection of systems, and the metadata and executor needed to run them in a certain order under certain conditions.

[Schedules](struct.Schedules.html "struct bevy::ecs::prelude::Schedules")

Resource that stores [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")s mapped to [`ScheduleLabel`](../schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")s excluding the current running [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule").

[Single](struct.Single.html "struct bevy::ecs::prelude::Single")

[System parameter](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides access to single entity’s components, much like [`Query::single`](../../prelude/struct.Query.html#method.single "method bevy::prelude::Query::single")/[`Query::single_mut`](../../prelude/struct.Query.html#method.single_mut "method bevy::prelude::Query::single_mut").

[Spawn](struct.Spawn.html "struct bevy::ecs::prelude::Spawn")

A wrapper over a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") indicating that an entity should be spawned with that [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"). This is intended to be used for hierarchical spawning via traits like [`SpawnableList`](../spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") and [`SpawnRelated`](../../prelude/trait.SpawnRelated.html "trait bevy::prelude::SpawnRelated").

[SpawnIter](struct.SpawnIter.html "struct bevy::ecs::prelude::SpawnIter")

A [`SpawnableList`](../spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") that spawns entities using an iterator of a given [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"):

[SpawnWith](struct.SpawnWith.html "struct bevy::ecs::prelude::SpawnWith")

A [`SpawnableList`](../spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") that spawns entities using a [`FnOnce`](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce") with a [`RelatedSpawner`](../relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner") as an argument:

[With](struct.With.html "struct bevy::ecs::prelude::With")

Filter that selects entities with a component `T`.

[WithOneRelated](struct.WithOneRelated.html "struct bevy::ecs::prelude::WithOneRelated")

A wrapper over an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") indicating that an entity should be added. This is intended to be used for hierarchical spawning via traits like [`SpawnableList`](../spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") and [`SpawnRelated`](../../prelude/trait.SpawnRelated.html "trait bevy::prelude::SpawnRelated").

[WithRelated](struct.WithRelated.html "struct bevy::ecs::prelude::WithRelated")

A [`SpawnableList`](../spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") that links already spawned entities to the root entity via relations of type `I`.

[Without](struct.Without.html "struct bevy::ecs::prelude::Without")

Filter that selects entities without a component `T`.

[World](struct.World.html "struct bevy::ecs::prelude::World")

Stores and exposes operations on [entities](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [components](../../prelude/trait.Component.html "trait bevy::prelude::Component"), resources, and their associated metadata.

## Enums

[Severity](enum.Severity.html "enum bevy::ecs::prelude::Severity")

Indicates how severe a [`BevyError`](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError") is.

## Traits

[Bundle](trait.Bundle.html "trait bevy::ecs::prelude::Bundle")

The `Bundle` trait enables insertion and removal of [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component")s from an entity.

[Command](trait.Command.html "trait bevy::ecs::prelude::Command")

A [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") mutation.

[Component](trait.Component.html "trait bevy::ecs::prelude::Component")

A data type that can be used to store data for an [entity](../entity/index.html "mod bevy::ecs::entity").

[ContainsEntity](trait.ContainsEntity.html "trait bevy::ecs::prelude::ContainsEntity")

A trait for types that contain an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[DetectChanges](trait.DetectChanges.html "trait bevy::ecs::prelude::DetectChanges")

Types that can read change detection information. This change detection is controlled by [`DetectChangesMut`](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut") types such as [`ResMut`](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut").

[DetectChangesMut](trait.DetectChangesMut.html "trait bevy::ecs::prelude::DetectChangesMut")

Types that implement reliable change detection.

[EntityCommand](trait.EntityCommand.html "trait bevy::ecs::prelude::EntityCommand")

A command which gets executed for a given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[EntityEvent](trait.EntityEvent.html "trait bevy::ecs::prelude::EntityEvent")

An [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") is an [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") that is triggered for a specific [`EntityEvent::event_target`](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") entity:

[EntityMapper](trait.EntityMapper.html "trait bevy::ecs::prelude::EntityMapper")

An implementor of this trait knows how to map an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") into another [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[Event](trait.Event.html "trait bevy::ecs::prelude::Event")

An [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") is something that “happens” at a given moment.

[FromTemplate](trait.FromTemplate.html "trait bevy::ecs::prelude::FromTemplate")

[`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") is implemented for types that can be produced by a specific, canonical [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"). This creates a way to correlate to the [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") using the desired template output type. This is used by Bevy’s scene system.

[FromWorld](trait.FromWorld.html "trait bevy::ecs::prelude::FromWorld")

Creates an instance of the type this trait is implemented for using data from the supplied [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[IntoScheduleConfigs](trait.IntoScheduleConfigs.html "trait bevy::ecs::prelude::IntoScheduleConfigs")

Types that can convert into a [`ScheduleConfigs`](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs").

[IntoSystem](trait.IntoSystem.html "trait bevy::ecs::prelude::IntoSystem")

Conversion trait to turn something into a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

[IntoSystemSet](trait.IntoSystemSet.html "trait bevy::ecs::prelude::IntoSystemSet")

Types that can be converted into a [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet").

[Message](trait.Message.html "trait bevy::ecs::prelude::Message")

A buffered message for pull-based event handling.

[ObserverSystemExt](trait.ObserverSystemExt.html "trait bevy::ecs::prelude::ObserverSystemExt")

Extension trait for adding run conditions to observer systems.

[ReadOnlySystem](trait.ReadOnlySystem.html "trait bevy::ecs::prelude::ReadOnlySystem")

[`System`](../../prelude/trait.System.html "trait bevy::prelude::System") types that do not modify the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") when run. This is implemented for any systems whose parameters all implement [`ReadOnlySystemParam`](../system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam").

[RelationshipTarget](trait.RelationshipTarget.html "trait bevy::ecs::prelude::RelationshipTarget")

A [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") containing the collection of entities that relate to this [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") via the associated `Relationship` type. See the [`Relationship`](../relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship") documentation for more information.

[Resource](trait.Resource.html "trait bevy::ecs::prelude::Resource")

A type that can be inserted into a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") as a singleton.

[ResultSeverityExt](trait.ResultSeverityExt.html "trait bevy::ecs::prelude::ResultSeverityExt")

Extension methods for annotating errors with a [`Severity`](../../prelude/enum.Severity.html "enum bevy::prelude::Severity").

[SpawnRelated](trait.SpawnRelated.html "trait bevy::ecs::prelude::SpawnRelated")

[`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") methods that create a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") with a [`DynamicBundle::Effect`](../bundle/trait.DynamicBundle.html#associatedtype.Effect "associated type bevy::ecs::bundle::DynamicBundle::Effect") that:

[System](trait.System.html "trait bevy::ecs::prelude::System")

An ECS system that can be added to a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")

[SystemCondition](trait.SystemCondition.html "trait bevy::ecs::prelude::SystemCondition")

A system that determines if one or more scheduled systems should run.

[SystemInput](trait.SystemInput.html "trait bevy::ecs::prelude::SystemInput")

Trait for types that can be used as input to [`System`](../../prelude/trait.System.html "trait bevy::prelude::System")s.

[SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::ecs::prelude::SystemParamBuilder")

A builder that can create a [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

[SystemParamFunction](trait.SystemParamFunction.html "trait bevy::ecs::prelude::SystemParamFunction")

A trait implemented for all functions that can be used as [`System`](../../prelude/trait.System.html "trait bevy::prelude::System")s.

[SystemSet](trait.SystemSet.html "trait bevy::ecs::prelude::SystemSet")

System sets are tag-like labels that can be used to group systems together.

[Template](trait.Template.html "trait bevy::ecs::prelude::Template")

A [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") is something that, given a spawn context (target [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), etc), can produce a [`Template::Output`](../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

## Functions

[any\_component\_removed](fn.any_component_removed.html "fn bevy::ecs::prelude::any_component_removed")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entity with a component of the given type removed.

[any\_match\_filter](fn.any_match_filter.html "fn bevy::ecs::prelude::any_match_filter")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entities that match the given [`QueryFilter`](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter").

[any\_with\_component](fn.any_with_component.html "fn bevy::ecs::prelude::any_with_component")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entities with the given component type.

[condition\_changed](fn.condition_changed.html "fn bevy::ecs::prelude::condition_changed")

Generates a [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that returns true when the passed one changes.

[condition\_changed\_to](fn.condition_changed_to.html "fn bevy::ecs::prelude::condition_changed_to")

Generates a [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that returns true when the result of the passed one went from false to true since the last time this was called.

[not](fn.not.html "fn bevy::ecs::prelude::not")

Generates a [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that inverses the result of passed one.

[on\_message](fn.on_message.html "fn bevy::ecs::prelude::on_message")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any new messages of the given type since it was last called.

[resource\_added](fn.resource_added.html "fn bevy::ecs::prelude::resource_added")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added since the condition was last checked.

[resource\_changed](fn.resource_changed.html "fn bevy::ecs::prelude::resource_changed")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added or mutably dereferenced since the condition was last checked.

[resource\_changed\_or\_removed](fn.resource_changed_or_removed.html "fn bevy::ecs::prelude::resource_changed_or_removed")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added, removed or mutably dereferenced since the condition was last checked.

[resource\_equals](fn.resource_equals.html "fn bevy::ecs::prelude::resource_equals")

Generates a [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the resource is equal to `value`.

[resource\_exists](fn.resource_exists.html "fn bevy::ecs::prelude::resource_exists")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource exists.

[resource\_exists\_and\_changed](fn.resource_exists_and_changed.html "fn bevy::ecs::prelude::resource_exists_and_changed")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added or mutably dereferenced since the condition was last checked.

[resource\_exists\_and\_equals](fn.resource_exists_and_equals.html "fn bevy::ecs::prelude::resource_exists_and_equals")

Generates a [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the resource exists and is equal to `value`.

[resource\_removed](fn.resource_removed.html "fn bevy::ecs::prelude::resource_removed")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been removed since the condition was last checked.

[run\_once](fn.run_once.html "fn bevy::ecs::prelude::run_once")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` on the first time the condition is run and false every time after.

[template](fn.template.html "fn bevy::ecs::prelude::template")

Returns a “free floating” template for a given `func`. This prevents the need to define a custom type for one-off templates.

## Type Aliases

[ChildSpawner](type.ChildSpawner.html "type bevy::ecs::prelude::ChildSpawner")

A type alias over [`RelatedSpawner`](../relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner") used to spawn child entities containing a [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship.

[ChildSpawnerCommands](type.ChildSpawnerCommands.html "type bevy::ecs::prelude::ChildSpawnerCommands")

A type alias over [`RelatedSpawnerCommands`](../relationship/struct.RelatedSpawnerCommands.html "struct bevy::ecs::relationship::RelatedSpawnerCommands") used to spawn child entities containing a [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship.

[Result](type.Result.html "type bevy::ecs::prelude::Result")

A result type for use in fallible systems, commands and observers.

[SystemIn](type.SystemIn.html "type bevy::ecs::prelude::SystemIn")

Shorthand way to get the [`System::In`](../../prelude/trait.System.html#associatedtype.In "associated type bevy::prelude::System::In") for a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") as a [`SystemInput::Inner`](../../prelude/trait.SystemInput.html#associatedtype.Inner "associated type bevy::prelude::SystemInput::Inner").

## Derive Macros

[Bundle](derive.Bundle.html "derive bevy::ecs::prelude::Bundle")

Implement the `Bundle` trait.

[Component](derive.Component.html "derive bevy::ecs::prelude::Component")

Cheat sheet for derive syntax, see full explanation and examples on the `Component` trait doc.

[EntityEvent](derive.EntityEvent.html "derive bevy::ecs::prelude::EntityEvent")

Cheat sheet for derive syntax, see full explanation on `EntityEvent` trait docs.

[Event](derive.Event.html "derive bevy::ecs::prelude::Event")

Implement the `Event` trait.

[FromTemplate](derive.FromTemplate.html "derive bevy::ecs::prelude::FromTemplate")

Derives `FromTemplate`.

[FromWorld](derive.FromWorld.html "derive bevy::ecs::prelude::FromWorld")

Implement the `FromWorld` trait.

[Message](derive.Message.html "derive bevy::ecs::prelude::Message")

Implement the `Message` trait.

[Resource](derive.Resource.html "derive bevy::ecs::prelude::Resource")

Implement the `Resource` trait.

[SystemSet](derive.SystemSet.html "derive bevy::ecs::prelude::SystemSet")

Derive macro generating an impl of the trait `SystemSet`.