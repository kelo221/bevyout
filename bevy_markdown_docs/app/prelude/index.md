[bevy](../../index.html)::[app](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/lib.rs.html#59)

The app prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[App](struct.App.html "struct bevy::app::prelude::App")

[`App`](../../prelude/struct.App.html "struct bevy::prelude::App") is the primary API for writing user applications. It automates the setup of a [standard lifecycle](../../prelude/struct.Main.html "struct bevy::prelude::Main") and provides interface glue for [plugins](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin").

[First](struct.First.html "struct bevy::app::prelude::First")

Runs first in the schedule.

[FixedFirst](struct.FixedFirst.html "struct bevy::app::prelude::FixedFirst")

Runs first in the [`FixedMain`](../struct.FixedMain.html "struct bevy::app::FixedMain") schedule.

[FixedLast](struct.FixedLast.html "struct bevy::app::prelude::FixedLast")

The schedule that runs last in [`FixedMain`](../struct.FixedMain.html "struct bevy::app::FixedMain")

[FixedPostUpdate](struct.FixedPostUpdate.html "struct bevy::app::prelude::FixedPostUpdate")

The schedule that runs after the [`FixedUpdate`](../../prelude/struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate") schedule, for reacting to changes made in the main update logic.

[FixedPreUpdate](struct.FixedPreUpdate.html "struct bevy::app::prelude::FixedPreUpdate")

The schedule that contains logic that must run before [`FixedUpdate`](../../prelude/struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate").

[FixedUpdate](struct.FixedUpdate.html "struct bevy::app::prelude::FixedUpdate")

The schedule that contains most gameplay logic, which runs at a fixed rate rather than every render frame. For logic that should run once per render frame, use the [`Update`](../../prelude/struct.Update.html "struct bevy::prelude::Update") schedule instead.

[Last](struct.Last.html "struct bevy::app::prelude::Last")

Runs last in the schedule.

[Main](struct.Main.html "struct bevy::app::prelude::Main")

The schedule that contains the app logic that is evaluated each tick of [`App::update()`](../../prelude/struct.App.html#method.update "method bevy::prelude::App::update").

[PostStartup](struct.PostStartup.html "struct bevy::app::prelude::PostStartup")

The schedule that runs once after [`Startup`](../../prelude/struct.Startup.html "struct bevy::prelude::Startup").

[PostUpdate](struct.PostUpdate.html "struct bevy::app::prelude::PostUpdate")

The schedule that contains logic that must run after [`Update`](../../prelude/struct.Update.html "struct bevy::prelude::Update"). For example, synchronizing “local transforms” in a hierarchy to “global” absolute transforms. This enables the [`PostUpdate`](../../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate") transform-sync system to react to “local transform” changes in [`Update`](../../prelude/struct.Update.html "struct bevy::prelude::Update") without the [`Update`](../../prelude/struct.Update.html "struct bevy::prelude::Update") systems needing to know about (or add scheduler dependencies for) the “global transform sync system”.

[PreStartup](struct.PreStartup.html "struct bevy::app::prelude::PreStartup")

The schedule that runs before [`Startup`](../../prelude/struct.Startup.html "struct bevy::prelude::Startup").

[PreUpdate](struct.PreUpdate.html "struct bevy::app::prelude::PreUpdate")

The schedule that contains logic that must run before [`Update`](../../prelude/struct.Update.html "struct bevy::prelude::Update"). For example, a system that reads raw keyboard input OS events into a `Messages` resource. This enables systems in [`Update`](../../prelude/struct.Update.html "struct bevy::prelude::Update") to consume the messages from the `Messages` resource without actually knowing about (or taking a direct scheduler dependency on) the “os-level keyboard event system”.

[RunFixedMainLoop](struct.RunFixedMainLoop.html "struct bevy::app::prelude::RunFixedMainLoop")

Runs the [`FixedMain`](../struct.FixedMain.html "struct bevy::app::FixedMain") schedule in a loop according until all relevant elapsed time has been “consumed”.

[SpawnScene](struct.SpawnScene.html "struct bevy::app::prelude::SpawnScene")

The schedule that contains scene spawning.

[Startup](struct.Startup.html "struct bevy::app::prelude::Startup")

The schedule that runs once when the app starts.

[SubApp](struct.SubApp.html "struct bevy::app::prelude::SubApp")

A secondary application with its own [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). These can run independently of each other.

[TaskPoolOptions](struct.TaskPoolOptions.html "struct bevy::app::prelude::TaskPoolOptions")

Helper for configuring and creating the default task pools. For end-users who want full control, set up [`TaskPoolPlugin`](../../prelude/struct.TaskPoolPlugin.html "struct bevy::prelude::TaskPoolPlugin")

[TaskPoolPlugin](struct.TaskPoolPlugin.html "struct bevy::app::prelude::TaskPoolPlugin")

Setup of default task pools: [`AsyncComputeTaskPool`](../../tasks/struct.AsyncComputeTaskPool.html "struct bevy::tasks::AsyncComputeTaskPool"), [`ComputeTaskPool`](../../tasks/struct.ComputeTaskPool.html "struct bevy::tasks::ComputeTaskPool"), [`IoTaskPool`](../../tasks/struct.IoTaskPool.html "struct bevy::tasks::IoTaskPool").

[Update](struct.Update.html "struct bevy::app::prelude::Update")

The schedule that contains any app logic that must run once per render frame. For most gameplay logic, consider using [`FixedUpdate`](../../prelude/struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate") instead.

## Enums

[AppExit](enum.AppExit.html "enum bevy::app::prelude::AppExit")

A [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") that indicates the [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") should exit. If one or more of these are present at the end of an update, the [runner](../../prelude/struct.App.html#method.set_runner "method bevy::prelude::App::set_runner") will end and ([maybe](../../prelude/struct.App.html#method.run "method bevy::prelude::App::run")) return control to the caller.

[RunFixedMainLoopSystems](enum.RunFixedMainLoopSystems.html "enum bevy::app::prelude::RunFixedMainLoopSystems")

Set enum for the systems that want to run inside [`RunFixedMainLoop`](../../prelude/struct.RunFixedMainLoop.html "struct bevy::prelude::RunFixedMainLoop"), but before or after the fixed update logic. Systems in this set will run exactly once per frame, regardless of the number of fixed updates. They will also run under a variable timestep.

## Traits

[Plugin](trait.Plugin.html "trait bevy::app::prelude::Plugin")

A collection of Bevy app logic and configuration.

[PluginGroup](trait.PluginGroup.html "trait bevy::app::prelude::PluginGroup")

Combines multiple [`Plugin`](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")s into a single unit.