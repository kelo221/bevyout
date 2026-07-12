[bevy](../index.html)

# Crate app 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/lib.rs.html#1-71)

This crate is about everything concerning the highest-level, application layer of a Bevy app.

## Modules

[ctrlc](ctrlc/index.html "mod bevy::app::ctrlc")

Cross platform handling of Ctrl-C signals.

[hotpatch](hotpatch/index.html "mod bevy::app::hotpatch")`hotpatching`

Utilities for hotpatching code.

[prelude](prelude/index.html "mod bevy::app::prelude")

The app prelude.

## Macros

[plugin\_group](macro.plugin_group.html "macro bevy::app::plugin_group")

A macro for generating a well-documented [`PluginGroup`](../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") from a list of [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") paths.

## Structs

[AnimationSystems](struct.AnimationSystems.html "struct bevy::app::AnimationSystems")

Animation system set. This exists in [`PostUpdate`](../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate").

[App](struct.App.html "struct bevy::app::App")

[`App`](../prelude/struct.App.html "struct bevy::prelude::App") is the primary API for writing user applications. It automates the setup of a [standard lifecycle](../prelude/struct.Main.html "struct bevy::prelude::Main") and provides interface glue for [plugins](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin").

[First](struct.First.html "struct bevy::app::First")

Runs first in the schedule.

[FixedFirst](struct.FixedFirst.html "struct bevy::app::FixedFirst")

Runs first in the [`FixedMain`](struct.FixedMain.html "struct bevy::app::FixedMain") schedule.

[FixedLast](struct.FixedLast.html "struct bevy::app::FixedLast")

The schedule that runs last in [`FixedMain`](struct.FixedMain.html "struct bevy::app::FixedMain")

[FixedMain](struct.FixedMain.html "struct bevy::app::FixedMain")

The schedule that contains systems which only run after a fixed period of time has elapsed.

[FixedMainScheduleOrder](struct.FixedMainScheduleOrder.html "struct bevy::app::FixedMainScheduleOrder")

Defines the schedules to be run for the [`FixedMain`](struct.FixedMain.html "struct bevy::app::FixedMain") schedule, including their order.

[FixedPostUpdate](struct.FixedPostUpdate.html "struct bevy::app::FixedPostUpdate")

The schedule that runs after the [`FixedUpdate`](../prelude/struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate") schedule, for reacting to changes made in the main update logic.

[FixedPreUpdate](struct.FixedPreUpdate.html "struct bevy::app::FixedPreUpdate")

The schedule that contains logic that must run before [`FixedUpdate`](../prelude/struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate").

[FixedUpdate](struct.FixedUpdate.html "struct bevy::app::FixedUpdate")

The schedule that contains most gameplay logic, which runs at a fixed rate rather than every render frame. For logic that should run once per render frame, use the [`Update`](../prelude/struct.Update.html "struct bevy::prelude::Update") schedule instead.

[HierarchyPropagatePlugin](struct.HierarchyPropagatePlugin.html "struct bevy::app::HierarchyPropagatePlugin")

Plugin to automatically propagate a component value to all direct and transient relationship targets (e.g. [`bevy_ecs::hierarchy::Children`](../prelude/struct.Children.html "struct bevy::prelude::Children")) of entities with a [`Propagate`](struct.Propagate.html "struct bevy::app::Propagate") component.

[Inherited](struct.Inherited.html "struct bevy::app::Inherited")

Internal struct for managing propagation

[Last](struct.Last.html "struct bevy::app::Last")

Runs last in the schedule.

[Main](struct.Main.html "struct bevy::app::Main")

The schedule that contains the app logic that is evaluated each tick of [`App::update()`](../prelude/struct.App.html#method.update "method bevy::prelude::App::update").

[MainScheduleOrder](struct.MainScheduleOrder.html "struct bevy::app::MainScheduleOrder")

Defines the schedules to be run for the [`Main`](../prelude/struct.Main.html "struct bevy::prelude::Main") schedule, including their order.

[MainSchedulePlugin](struct.MainSchedulePlugin.html "struct bevy::app::MainSchedulePlugin")

Initializes the [`Main`](../prelude/struct.Main.html "struct bevy::prelude::Main") schedule, sub schedules, and resources for a given [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

[PanicHandlerPlugin](struct.PanicHandlerPlugin.html "struct bevy::app::PanicHandlerPlugin")

Adds sensible panic handlers to Apps. This plugin is part of the `DefaultPlugins`. Adding this plugin will setup a panic hook appropriate to your target platform:

[PluginGroupBuilder](struct.PluginGroupBuilder.html "struct bevy::app::PluginGroupBuilder")

Facilitates the creation and configuration of a [`PluginGroup`](../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup").

[PostStartup](struct.PostStartup.html "struct bevy::app::PostStartup")

The schedule that runs once after [`Startup`](../prelude/struct.Startup.html "struct bevy::prelude::Startup").

[PostUpdate](struct.PostUpdate.html "struct bevy::app::PostUpdate")

The schedule that contains logic that must run after [`Update`](../prelude/struct.Update.html "struct bevy::prelude::Update"). For example, synchronizing “local transforms” in a hierarchy to “global” absolute transforms. This enables the [`PostUpdate`](../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate") transform-sync system to react to “local transform” changes in [`Update`](../prelude/struct.Update.html "struct bevy::prelude::Update") without the [`Update`](../prelude/struct.Update.html "struct bevy::prelude::Update") systems needing to know about (or add scheduler dependencies for) the “global transform sync system”.

[PreStartup](struct.PreStartup.html "struct bevy::app::PreStartup")

The schedule that runs before [`Startup`](../prelude/struct.Startup.html "struct bevy::prelude::Startup").

[PreUpdate](struct.PreUpdate.html "struct bevy::app::PreUpdate")

The schedule that contains logic that must run before [`Update`](../prelude/struct.Update.html "struct bevy::prelude::Update"). For example, a system that reads raw keyboard input OS events into a `Messages` resource. This enables systems in [`Update`](../prelude/struct.Update.html "struct bevy::prelude::Update") to consume the messages from the `Messages` resource without actually knowing about (or taking a direct scheduler dependency on) the “os-level keyboard event system”.

[Propagate](struct.Propagate.html "struct bevy::app::Propagate")

Causes the inner component to be added to this entity and all direct and transient relationship targets. A target with a [`Propagate<C>`](struct.Propagate.html "struct bevy::app::Propagate") component of its own will override propagation from that point in the tree.

[PropagateOver](struct.PropagateOver.html "struct bevy::app::PropagateOver")

Stops the output component being added to this entity. Relationship targets will still inherit the component from this entity or its parents.

[PropagateSet](struct.PropagateSet.html "struct bevy::app::PropagateSet")

The set in which propagation systems are added. You can schedule your logic relative to this set.

[PropagateStop](struct.PropagateStop.html "struct bevy::app::PropagateStop")

Stops the propagation at this entity. Children will not inherit the component.

[RunFixedMainLoop](struct.RunFixedMainLoop.html "struct bevy::app::RunFixedMainLoop")

Runs the [`FixedMain`](struct.FixedMain.html "struct bevy::app::FixedMain") schedule in a loop according until all relevant elapsed time has been “consumed”.

[ScheduleRunnerPlugin](struct.ScheduleRunnerPlugin.html "struct bevy::app::ScheduleRunnerPlugin")

Configures an [`App`](../prelude/struct.App.html "struct bevy::prelude::App") to run its [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") according to a given [`RunMode`](enum.RunMode.html "enum bevy::app::RunMode").

[SpawnScene](struct.SpawnScene.html "struct bevy::app::SpawnScene")

The schedule that contains scene spawning.

[Startup](struct.Startup.html "struct bevy::app::Startup")

The schedule that runs once when the app starts.

[SubApp](struct.SubApp.html "struct bevy::app::SubApp")

A secondary application with its own [`World`](../prelude/struct.World.html "struct bevy::prelude::World"). These can run independently of each other.

[SubApps](struct.SubApps.html "struct bevy::app::SubApps")

The collection of sub-apps that belong to an [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

[TaskPoolOptions](struct.TaskPoolOptions.html "struct bevy::app::TaskPoolOptions")

Helper for configuring and creating the default task pools. For end-users who want full control, set up [`TaskPoolPlugin`](../prelude/struct.TaskPoolPlugin.html "struct bevy::prelude::TaskPoolPlugin")

[TaskPoolPlugin](struct.TaskPoolPlugin.html "struct bevy::app::TaskPoolPlugin")

Setup of default task pools: [`AsyncComputeTaskPool`](../tasks/struct.AsyncComputeTaskPool.html "struct bevy::tasks::AsyncComputeTaskPool"), [`ComputeTaskPool`](../tasks/struct.ComputeTaskPool.html "struct bevy::tasks::ComputeTaskPool"), [`IoTaskPool`](../tasks/struct.IoTaskPool.html "struct bevy::tasks::IoTaskPool").

[TaskPoolThreadAssignmentPolicy](struct.TaskPoolThreadAssignmentPolicy.html "struct bevy::app::TaskPoolThreadAssignmentPolicy")

Defines a simple way to determine how many threads to use given the number of remaining cores and number of total cores

[TerminalCtrlCHandlerPlugin](struct.TerminalCtrlCHandlerPlugin.html "struct bevy::app::TerminalCtrlCHandlerPlugin")

Gracefully handles `Ctrl+C` by emitting a [`AppExit`](../prelude/enum.AppExit.html "enum bevy::prelude::AppExit") event. This plugin is part of the `DefaultPlugins`.

[Update](struct.Update.html "struct bevy::app::Update")

The schedule that contains any app logic that must run once per render frame. For most gameplay logic, consider using [`FixedUpdate`](../prelude/struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate") instead.

[ValidateParentHasComponentPlugin](struct.ValidateParentHasComponentPlugin.html "struct bevy::app::ValidateParentHasComponentPlugin")

A plugin that verifies that [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") `C` has parents that also have that component.

[ValidateParentHasComponentSystems](struct.ValidateParentHasComponentSystems.html "struct bevy::app::ValidateParentHasComponentSystems")

System set for systems added by [`ValidateParentHasComponentPlugin`](struct.ValidateParentHasComponentPlugin.html "struct bevy::app::ValidateParentHasComponentPlugin").

## Enums

[AppExit](enum.AppExit.html "enum bevy::app::AppExit")

A [`Message`](../prelude/trait.Message.html "trait bevy::prelude::Message") that indicates the [`App`](../prelude/struct.App.html "struct bevy::prelude::App") should exit. If one or more of these are present at the end of an update, the [runner](../prelude/struct.App.html#method.set_runner "method bevy::prelude::App::set_runner") will end and ([maybe](../prelude/struct.App.html#method.run "method bevy::prelude::App::run")) return control to the caller.

[PluginsState](enum.PluginsState.html "enum bevy::app::PluginsState")

Plugins state in the application

[RunFixedMainLoopSystems](enum.RunFixedMainLoopSystems.html "enum bevy::app::RunFixedMainLoopSystems")

Set enum for the systems that want to run inside [`RunFixedMainLoop`](../prelude/struct.RunFixedMainLoop.html "struct bevy::prelude::RunFixedMainLoop"), but before or after the fixed update logic. Systems in this set will run exactly once per frame, regardless of the number of fixed updates. They will also run under a variable timestep.

[RunMode](enum.RunMode.html "enum bevy::app::RunMode")

Determines the method used to run an [`App`](../prelude/struct.App.html "struct bevy::prelude::App")’s [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule").

[SceneSpawnerSystems](enum.SceneSpawnerSystems.html "enum bevy::app::SceneSpawnerSystems")

Set enum for the systems relating to scene spawning.

## Traits

[AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")

A strongly-typed class of labels used to identify an [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

[DynEq](trait.DynEq.html "trait bevy::app::DynEq")

An object safe version of [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"). This trait is automatically implemented for any `'static` type that implements `Eq`.

[Plugin](trait.Plugin.html "trait bevy::app::Plugin")

A collection of Bevy app logic and configuration.

[PluginGroup](trait.PluginGroup.html "trait bevy::app::PluginGroup")

Combines multiple [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")s into a single unit.

[Plugins](trait.Plugins.html "trait bevy::app::Plugins")

Types that represent a set of [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")s.

## Functions

[on\_r\_inserted](fn.on_r_inserted.html "fn bevy::app::on_r_inserted")

Add/remove [`Inherited::<C>`](struct.Inherited.html "struct bevy::app::Inherited") when an entity gains or changes its `R` relationship

[on\_r\_removed](fn.on_r_removed.html "fn bevy::app::on_r_removed")

Remove [`Inherited::<C>`](struct.Inherited.html "struct bevy::app::Inherited") when an entity loses its `R` relationship

[propagate\_inherited](fn.propagate_inherited.html "fn bevy::app::propagate_inherited")

add/remove `Inherited::<C>` for targets of entities with modified `Inherited::<C>`

[propagate\_output](fn.propagate_output.html "fn bevy::app::propagate_output")

add/remove `C` on entities with `Inherited::<C>`

[update\_removed\_limit](fn.update_removed_limit.html "fn bevy::app::update_removed_limit")

When `PropagateOver` or `PropagateStop` is removed, update the `Inherited::<C>` to trigger propagation

[update\_source](fn.update_source.html "fn bevy::app::update_source")

add/remove `Inherited::<C>` for entities with a direct `Propagate::<C>`

## Type Aliases

[InternedAppLabel](type.InternedAppLabel.html "type bevy::app::InternedAppLabel")

A shorthand for `Interned<dyn AppLabel>`.

## Derive Macros

[AppLabel](derive.AppLabel.html "derive bevy::app::AppLabel")

Generates an impl of the `AppLabel` trait.