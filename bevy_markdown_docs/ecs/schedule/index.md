[bevy](../../index.html)::[ecs](../index.html)

# Module schedule 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#51)

Contains APIs for ordering systems and executing them on a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World")

## Modules

[common\_conditions](common_conditions/index.html "mod bevy::ecs::schedule::common_conditions")

A collection of [run conditions](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that may be useful in any bevy app.

[graph](graph/index.html "mod bevy::ecs::schedule::graph")

An implementation of a graph data structure.

[passes](passes/index.html "mod bevy::ecs::schedule::passes")

Included optional schedule build passes.

## Structs

[AmbiguousSystemConflictsWarning](struct.AmbiguousSystemConflictsWarning.html "struct bevy::ecs::schedule::AmbiguousSystemConflictsWarning")

Error returned when there are ambiguous system conflicts detected.

[AnonymousSet](struct.AnonymousSet.html "struct bevy::ecs::schedule::AnonymousSet")

A [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") implicitly created when using [`Schedule::add_systems`](../../prelude/struct.Schedule.html#method.add_systems "method bevy::prelude::Schedule::add_systems") or [`Schedule::configure_sets`](../../prelude/struct.Schedule.html#method.configure_sets "method bevy::prelude::Schedule::configure_sets").

[ApplyDeferred](struct.ApplyDeferred.html "struct bevy::ecs::schedule::ApplyDeferred")

A special [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") that instructs the executor to call [`System::apply_deferred`](../../prelude/trait.System.html#tymethod.apply_deferred "method bevy::prelude::System::apply_deferred") on the systems that have run but not applied their [`Deferred`](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred") system parameters (like [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")) or other system buffers.

[CompactNodeIdAndDirection](struct.CompactNodeIdAndDirection.html "struct bevy::ecs::schedule::CompactNodeIdAndDirection")

Compact storage of a [`NodeId`](enum.NodeId.html "enum bevy::ecs::schedule::NodeId") and a [`Direction`](graph/enum.Direction.html "enum bevy::ecs::schedule::graph::Direction").

[CompactNodeIdPair](struct.CompactNodeIdPair.html "struct bevy::ecs::schedule::CompactNodeIdPair")

Compact storage of a [`NodeId`](enum.NodeId.html "enum bevy::ecs::schedule::NodeId") pair.

[ConditionWithAccess](struct.ConditionWithAccess.html "struct bevy::ecs::schedule::ConditionWithAccess")

A [`BoxedCondition`](type.BoxedCondition.html "type bevy::ecs::schedule::BoxedCondition") stored alongside the access returned from [`System::initialize`](../../prelude/trait.System.html#tymethod.initialize "method bevy::prelude::System::initialize").

[ConflictingSystems](struct.ConflictingSystems.html "struct bevy::ecs::schedule::ConflictingSystems")

Pairs of systems that conflict with each other along with the components they conflict on, which prevents them from running in parallel. If the component list is empty, the systems conflict on [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") access in general (e.g. one of them is exclusive, or both systems have `Query<EntityMut>`).

[FixedBitSet](struct.FixedBitSet.html "struct bevy::ecs::schedule::FixedBitSet")

`FixedBitSet` is a simple fixed size set of bits that each can be enabled (1 / **true**) or disabled (0 / **false**).

[FlattenedDependencies](struct.FlattenedDependencies.html "struct bevy::ecs::schedule::FlattenedDependencies")

A wrapper around the directed, acyclic graph of system edges.

[GraphInfo](struct.GraphInfo.html "struct bevy::ecs::schedule::GraphInfo")

Metadata about how the node fits in the schedule graph

[MainThreadExecutor](struct.MainThreadExecutor.html "struct bevy::ecs::schedule::MainThreadExecutor")

New-typed [`ThreadExecutor`](../../tasks/struct.ThreadExecutor.html "struct bevy::tasks::ThreadExecutor") [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") that is used to run systems on the main thread

[MultiThreadedExecutor](struct.MultiThreadedExecutor.html "struct bevy::ecs::schedule::MultiThreadedExecutor")

Runs the schedule using a thread pool. Non-conflicting systems can run in parallel.

[Schedule](struct.Schedule.html "struct bevy::ecs::schedule::Schedule")

A collection of systems, and the metadata and executor needed to run them in a certain order under certain conditions.

[ScheduleBuildMetadata](struct.ScheduleBuildMetadata.html "struct bevy::ecs::schedule::ScheduleBuildMetadata")

Metadata about the schedule build process.

[ScheduleBuildSettings](struct.ScheduleBuildSettings.html "struct bevy::ecs::schedule::ScheduleBuildSettings")

Specifies miscellaneous settings for schedule construction.

[ScheduleBuilt](struct.ScheduleBuilt.html "struct bevy::ecs::schedule::ScheduleBuilt")

An event triggered when a schedule is successfully built.

[ScheduleConfig](struct.ScheduleConfig.html "struct bevy::ecs::schedule::ScheduleConfig")

Stores configuration for a single generic node (a system or a system set)

[ScheduleGraph](struct.ScheduleGraph.html "struct bevy::ecs::schedule::ScheduleGraph")

Metadata for a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule").

[ScheduleNotInitialized](struct.ScheduleNotInitialized.html "struct bevy::ecs::schedule::ScheduleNotInitialized")

Error to denote that [`Schedule::initialize`](../../prelude/struct.Schedule.html#method.initialize "method bevy::prelude::Schedule::initialize") or [`Schedule::run`](../../prelude/struct.Schedule.html#method.run "method bevy::prelude::Schedule::run") has not yet been called for this schedule.

[Schedules](struct.Schedules.html "struct bevy::ecs::schedule::Schedules")

Resource that stores [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")s mapped to [`ScheduleLabel`](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")s excluding the current running [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule").

[SingleThreadedExecutor](struct.SingleThreadedExecutor.html "struct bevy::ecs::schedule::SingleThreadedExecutor")

Runs the schedule using a single thread.

[Stepping](struct.Stepping.html "struct bevy::ecs::schedule::Stepping")

Resource for controlling system stepping behavior

[SystemKey](struct.SystemKey.html "struct bevy::ecs::schedule::SystemKey")

A unique identifier for a system in a [`ScheduleGraph`](struct.ScheduleGraph.html "struct bevy::ecs::schedule::ScheduleGraph").

[SystemSchedule](struct.SystemSchedule.html "struct bevy::ecs::schedule::SystemSchedule")

Holds systems and conditions of a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") sorted in topological order (along with dependency information for `multi_threaded` execution).

[SystemSetKey](struct.SystemSetKey.html "struct bevy::ecs::schedule::SystemSetKey")

A unique identifier for a system set in a [`ScheduleGraph`](struct.ScheduleGraph.html "struct bevy::ecs::schedule::ScheduleGraph").

[SystemSets](struct.SystemSets.html "struct bevy::ecs::schedule::SystemSets")

Container for system sets in a schedule.

[SystemTypeSet](struct.SystemTypeSet.html "struct bevy::ecs::schedule::SystemTypeSet")

A [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") grouping instances of the same function.

[SystemTypeSetAmbiguityError](struct.SystemTypeSetAmbiguityError.html "struct bevy::ecs::schedule::SystemTypeSetAmbiguityError")

Error returned when calling [`SystemSets::check_type_set_ambiguity`](struct.SystemSets.html#method.check_type_set_ambiguity "method bevy::ecs::schedule::SystemSets::check_type_set_ambiguity").

[SystemWithAccess](struct.SystemWithAccess.html "struct bevy::ecs::schedule::SystemWithAccess")

A [`ScheduleSystem`](../system/type.ScheduleSystem.html "type bevy::ecs::system::ScheduleSystem") stored alongside the access returned from [`System::initialize`](../../prelude/trait.System.html#tymethod.initialize "method bevy::prelude::System::initialize").

[Systems](struct.Systems.html "struct bevy::ecs::schedule::Systems")

Container for systems in a schedule.

## Enums

[Chain](enum.Chain.html "enum bevy::ecs::schedule::Chain")

Chain systems into dependencies

[LogLevel](enum.LogLevel.html "enum bevy::ecs::schedule::LogLevel")

Specifies how schedule construction should respond to detecting a certain kind of issue.

[NodeId](enum.NodeId.html "enum bevy::ecs::schedule::NodeId")

Unique identifier for a system or system set stored in a [`ScheduleGraph`](struct.ScheduleGraph.html "struct bevy::ecs::schedule::ScheduleGraph").

[ScheduleBuildError](enum.ScheduleBuildError.html "enum bevy::ecs::schedule::ScheduleBuildError")

Category of errors encountered during [`Schedule::initialize`](../../prelude/struct.Schedule.html#method.initialize "method bevy::prelude::Schedule::initialize").

[ScheduleBuildWarning](enum.ScheduleBuildWarning.html "enum bevy::ecs::schedule::ScheduleBuildWarning")

Category of warnings encountered during [`Schedule::initialize`](../../prelude/struct.Schedule.html#method.initialize "method bevy::prelude::Schedule::initialize").

[ScheduleCleanupPolicy](enum.ScheduleCleanupPolicy.html "enum bevy::ecs::schedule::ScheduleCleanupPolicy")

Policy to use when removing systems.

[ScheduleConfigs](enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")

Single or nested configurations for [`Schedulable`](trait.Schedulable.html "trait bevy::ecs::schedule::Schedulable")s.

[ScheduleError](enum.ScheduleError.html "enum bevy::ecs::schedule::ScheduleError")

Error returned from some `Schedule` methods

## Traits

[DynEq](trait.DynEq.html "trait bevy::ecs::schedule::DynEq")

An object safe version of [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"). This trait is automatically implemented for any `'static` type that implements `Eq`.

[IntoScheduleConfigs](trait.IntoScheduleConfigs.html "trait bevy::ecs::schedule::IntoScheduleConfigs")

Types that can convert into a [`ScheduleConfigs`](enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs").

[IntoSystemSet](trait.IntoSystemSet.html "trait bevy::ecs::schedule::IntoSystemSet")

Types that can be converted into a [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet").

[Schedulable](trait.Schedulable.html "trait bevy::ecs::schedule::Schedulable")

Stores data to differentiate different schedulable structs.

[ScheduleBuildPass](trait.ScheduleBuildPass.html "trait bevy::ecs::schedule::ScheduleBuildPass")

A pass for modular modification of the dependency graph.

[ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")

A strongly-typed class of labels used to identify a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule").

[SystemCondition](trait.SystemCondition.html "trait bevy::ecs::schedule::SystemCondition")

A system that determines if one or more scheduled systems should run.

[SystemExecutor](trait.SystemExecutor.html "trait bevy::ecs::schedule::SystemExecutor")

Types that can run a [`SystemSchedule`](struct.SystemSchedule.html "struct bevy::ecs::schedule::SystemSchedule") on a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[SystemSet](trait.SystemSet.html "trait bevy::ecs::schedule::SystemSet")

System sets are tag-like labels that can be used to group systems together.

## Functions

[default\_executor](fn.default_executor.html "fn bevy::ecs::schedule::default_executor")

Returns the default executor for the current platform.

## Type Aliases

[AndEager](type.AndEager.html "type bevy::ecs::schedule::AndEager")

Combines the outputs of two systems using the `&` operator (eagerly evaluated).

[AndThen](type.AndThen.html "type bevy::ecs::schedule::AndThen")

Combines the outputs of two systems using the `&&` operator (short-circuiting).

[BoxedCondition](type.BoxedCondition.html "type bevy::ecs::schedule::BoxedCondition")

A type-erased run condition stored in a [`Box`](../../prelude/struct.Box.html "struct bevy::prelude::Box").

[InternedScheduleLabel](type.InternedScheduleLabel.html "type bevy::ecs::schedule::InternedScheduleLabel")

A shorthand for `Interned<dyn ScheduleLabel>`.

[InternedSystemSet](type.InternedSystemSet.html "type bevy::ecs::schedule::InternedSystemSet")

A shorthand for `Interned<dyn SystemSet>`.

[NandEager](type.NandEager.html "type bevy::ecs::schedule::NandEager")

Combines and inverts the outputs of two systems using the `&` and `!` operators (eagerly evaluated).

[NandThen](type.NandThen.html "type bevy::ecs::schedule::NandThen")

Combines and inverts the outputs of two systems using the `&&` and `!` operators (short-circuiting).

[NorEager](type.NorEager.html "type bevy::ecs::schedule::NorEager")

Combines and inverts the outputs of two systems using the `|` and `!` operators (eagerly evaluated).

[NorElse](type.NorElse.html "type bevy::ecs::schedule::NorElse")

Combines and inverts the outputs of two systems using the `||` and `!` operators (short-circuiting).

[NotSystem](type.NotSystem.html "type bevy::ecs::schedule::NotSystem")

Invokes [`Not`](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") with the output of another system.

[OrEager](type.OrEager.html "type bevy::ecs::schedule::OrEager")

Combines the outputs of two systems using the `|` operator (short-circuiting).

[OrElse](type.OrElse.html "type bevy::ecs::schedule::OrElse")

Combines the outputs of two systems using the `||` operator (short-circuiting).

[Xnor](type.Xnor.html "type bevy::ecs::schedule::Xnor")

Combines and inverts the outputs of two systems using the `^` and `!` operators (eagerly evaluated).

[Xor](type.Xor.html "type bevy::ecs::schedule::Xor")

Combines the outputs of two systems using the `^` operator (eagerly evaluated).

## Derive Macros

[ScheduleLabel](derive.ScheduleLabel.html "derive bevy::ecs::schedule::ScheduleLabel")

Derive macro generating an impl of the trait `ScheduleLabel`.

[SystemSet](derive.SystemSet.html "derive bevy::ecs::schedule::SystemSet")

Derive macro generating an impl of the trait `SystemSet`.