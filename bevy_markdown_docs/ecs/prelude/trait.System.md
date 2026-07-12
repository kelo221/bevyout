[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Trait System 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#48)

```rust
pub trait System:
    Send
    + Sync
    + 'static {
    type In: SystemInput;
    type Out;

    // Required methods
    fn name(&self) -> DebugName;
    fn flags(&self) -> SystemStateFlags;
    unsafe fn run_unsafe(
        &mut self,
        input: <Self::In as SystemInput>::Inner<'_>,
        world: UnsafeWorldCell<'_>,
    ) -> Result<Self::Out, RunSystemError>;
    fn refresh_hotpatch(&mut self);
    fn apply_deferred(&mut self, world: &mut World);
    fn queue_deferred(&mut self, world: DeferredWorld<'_>);
    fn initialize(&mut self, _world: &mut World) -> FilteredAccessSet;
    fn check_change_tick(&mut self, check: CheckChangeTicks);
    fn get_last_run(&self) -> Tick;
    fn set_last_run(&mut self, last_run: Tick);

    // Provided methods
    fn system_type(&self) -> TypeId { ... }
    fn type_id(&self) -> TypeId { ... }
    fn is_send(&self) -> bool { ... }
    fn is_exclusive(&self) -> bool { ... }
    fn has_deferred(&self) -> bool { ... }
    fn run(
        &mut self,
        input: <Self::In as SystemInput>::Inner<'_>,
        world: &mut World,
    ) -> Result<Self::Out, RunSystemError> { ... }
    fn run_without_applying_deferred(
        &mut self,
        input: <Self::In as SystemInput>::Inner<'_>,
        world: &mut World,
    ) -> Result<Self::Out, RunSystemError> { ... }
    fn default_system_sets(&self) -> Vec<Interned<dyn SystemSet>> { ... }
}
```

An ECS system that can be added to a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")

Systems are functions with all arguments implementing [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

Systems are added to an application using `App::add_systems(Update, my_system)` or similar methods, and will generally run once per pass of the main loop.

Systems are executed in parallel, in opportunistic order; data access is managed automatically. It’s possible to specify explicit execution order between specific systems, see [`IntoScheduleConfigs`](../../prelude/trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs").

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#50)

#### type [In](#associatedtype.In): [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")

The system’s input.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#52)

#### type [Out](#associatedtype.Out)

The system’s output.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#55)

#### fn [name](#tymethod.name)(&self) -> [DebugName](../../prelude/struct.DebugName.html "struct bevy::prelude::DebugName")

Returns the system’s name.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#75)

#### fn [flags](#tymethod.flags)(&self) -> [SystemStateFlags](../system/struct.SystemStateFlags.html "struct bevy::ecs::system::SystemStateFlags")

Returns the [`SystemStateFlags`](../system/struct.SystemStateFlags.html "struct bevy::ecs::system::SystemStateFlags") of the system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#109-113)

#### unsafe fn [run\_unsafe](#tymethod.run_unsafe)( &mut self, input: <Self::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out"), [RunSystemError](../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Runs the system with the given input in the world. Unlike [`System::run`](../../prelude/trait.System.html#method.run "method bevy::prelude::System::run"), this function can be called in parallel with other systems and may break Rust’s aliasing rules if used incorrectly, making it unsafe to call.

Unlike [`System::run`](../../prelude/trait.System.html#method.run "method bevy::prelude::System::run"), this will not apply deferred parameters, which must be independently applied by calling [`System::apply_deferred`](../../prelude/trait.System.html#tymethod.apply_deferred "method bevy::prelude::System::apply_deferred") at later point in time.

##### Safety

*   The caller must ensure that [`world`](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") has permission to access any world data registered in the access returned from [`System::initialize`](../../prelude/trait.System.html#tymethod.initialize "method bevy::prelude::System::initialize"). There must be no conflicting simultaneous accesses while the system is running.
*   If [`System::is_exclusive`](../../prelude/trait.System.html#method.is_exclusive "method bevy::prelude::System::is_exclusive") returns `true`, then it must be valid to call [`UnsafeWorldCell::world_mut`](../world/unsafe_world_cell/struct.UnsafeWorldCell.html#method.world_mut "method bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell::world_mut") on `world`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#117)

#### fn [refresh\_hotpatch](#tymethod.refresh_hotpatch)(&mut self)

Available on **crate feature `hotpatching`** only.

Refresh the inner pointer based on the latest hot patch jump table

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#153)

#### fn [apply\_deferred](#tymethod.apply_deferred)(&mut self, world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"))

Applies any [`Deferred`](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred") system parameters (or other system buffers) of this system to the world.

This is where [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") get applied.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#157)

#### fn [queue\_deferred](#tymethod.queue_deferred)(&mut self, world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>)

Enqueues any [`Deferred`](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred") system parameters (or other system buffers) of this system into the world’s command buffer.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#162)

#### fn [initialize](#tymethod.initialize)(&mut self, \_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet")

Initialize the system.

Returns a [`FilteredAccessSet`](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet") with the access required to run the system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#168)

#### fn [check\_change\_tick](#tymethod.check_change_tick)(&mut self, check: [CheckChangeTicks](../change_detection/struct.CheckChangeTicks.html "struct bevy::ecs::change_detection::CheckChangeTicks"))

Checks any [`Tick`](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")s stored on this system and wraps their value if they get too old.

This method must be called periodically to ensure that change detection behaves correctly. When using bevy’s default configuration, this will be called for you as needed.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#178)

#### fn [get\_last\_run](#tymethod.get_last_run)(&self) -> [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Gets the tick indicating the last time this system ran.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#186)

#### fn [set\_last\_run](#tymethod.set_last_run)(&mut self, last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"))

Overwrites the tick indicating the last time this system ran.

##### Warning

This is a complex and error-prone operation, that can have unexpected consequences on any system relying on this code. However, it can be an essential escape hatch when, for example, you are trying to synchronize representations using change detection and need to avoid infinite recursion.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#58)

#### fn [system\_type](#method.system_type)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Returns the [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") of the underlying system type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#70)

#### fn [type\_id](#method.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

👎Deprecated since 0.19.0:

Use `system_type` instead. This method shadows `Any::type_id` and will be removed in a future release.

Returns the [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") of the underlying system type.

Use [`system_type`](../../prelude/trait.System.html#method.system_type "method bevy::prelude::System::system_type") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#79)

#### fn [is\_send](#method.is_send)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the system is [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#85)

#### fn [is\_exclusive](#method.is_exclusive)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the system must be run exclusively.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#91)

#### fn [has\_deferred](#method.has_deferred)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if system has deferred buffers.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#126-130)

#### fn [run](#method.run)( &mut self, input: <Self::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out"), [RunSystemError](../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Runs the system with the given input in the world.

For [read-only](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem") systems, see [`run_readonly`](../../prelude/trait.ReadOnlySystem.html#method.run_readonly "method bevy::prelude::ReadOnlySystem::run_readonly"), which can be called using `&World`.

Unlike [`System::run_unsafe`](../../prelude/trait.System.html#tymethod.run_unsafe "method bevy::prelude::System::run_unsafe"), this will apply deferred parameters _immediately_.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/custom\_executor.rs ([line 27](../../../src/custom_executor/custom_executor.rs.html#27))

```rust
17    fn run(
18        &mut self,
19        schedule: &mut SystemSchedule,
20        world: &mut World,
21        _skip_systems: Option<&FixedBitSet>,
22        _error_handler: fn(BevyError, ErrorContext),
23    ) {
24        #[expect(unsafe_code, reason = "CustomExecutor's require unsafe")]
25        // SAFETY: `run` is a trait method on `System`
26        for entry in unsafe { schedule.systems_mut().iter_mut() } {
27            let _ = entry.run((), world);
28        }
29    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#139-143)

#### fn [run\_without\_applying\_deferred](#method.run_without_applying_deferred)( &mut self, input: <Self::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out"), [RunSystemError](../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Runs the system with the given input in the world.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#173)

#### fn [default\_system\_sets](#method.default_system_sets)(&self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Interned](../intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")\>>

Returns the system’s default [system sets](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet").

Each system will create a default system set that contains the system.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#244-247)

### impl<In, Out> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = Out, In = In>

where In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, Out: 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#249)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#571)

### impl [IntoScheduleConfigs](../../prelude/trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#572)

#### fn [into\_configs](../../prelude/trait.IntoScheduleConfigs.html#tymethod.into_configs)(self) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>>

Convert into a [`ScheduleConfigs`](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#322)

#### fn [in\_set](../../prelude/trait.IntoScheduleConfigs.html#method.in_set)(self, set: impl [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Add these systems to the provided `set`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#334)

#### fn [before](../../prelude/trait.IntoScheduleConfigs.html#method.before)<M>(self, set: impl [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Runs before all systems in `set`. If `self` has any systems that produce [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") or other [`Deferred`](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred") operations, all systems in `set` will see their effect. [Read more](../../prelude/trait.IntoScheduleConfigs.html#method.before)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#361)

#### fn [after](../../prelude/trait.IntoScheduleConfigs.html#method.after)<M>(self, set: impl [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Run after all systems in `set`. If `set` has any systems that produce [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") or other [`Deferred`](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred") operations, all systems in `self` will see their effect. [Read more](../../prelude/trait.IntoScheduleConfigs.html#method.after)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#369)

#### fn [before\_ignore\_deferred](../../prelude/trait.IntoScheduleConfigs.html#method.before_ignore_deferred)<M>( self, set: impl [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>, ) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Run before all systems in `set`. [Read more](../../prelude/trait.IntoScheduleConfigs.html#method.before_ignore_deferred)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#377)

#### fn [after\_ignore\_deferred](../../prelude/trait.IntoScheduleConfigs.html#method.after_ignore_deferred)<M>( self, set: impl [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>, ) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Run after all systems in `set`. [Read more](../../prelude/trait.IntoScheduleConfigs.html#method.after_ignore_deferred)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#411-414)

#### fn [distributive\_run\_if](../../prelude/trait.IntoScheduleConfigs.html#method.distributive_run_if)<M>( self, condition: impl [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), ) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Add a run condition to each contained system. [Read more](../../prelude/trait.IntoScheduleConfigs.html#method.distributive_run_if)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#448)

#### fn [run\_if](../../prelude/trait.IntoScheduleConfigs.html#method.run_if)<M>(self, condition: impl [SystemCondition](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M>) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Run the systems only if the [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") is `true`. [Read more](../../prelude/trait.IntoScheduleConfigs.html#method.run_if)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#454)

#### fn [ambiguous\_with](../../prelude/trait.IntoScheduleConfigs.html#method.ambiguous_with)<M>(self, set: impl [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Suppress warnings and errors that would result from these systems having ambiguities (conflicting access but indeterminate order) with systems in `set`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#460)

#### fn [ambiguous\_with\_all](../../prelude/trait.IntoScheduleConfigs.html#method.ambiguous_with_all)(self) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Suppress warnings and errors that would result from these systems having ambiguities (conflicting access but indeterminate order) with any other system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#471)

#### fn [chain](../../prelude/trait.IntoScheduleConfigs.html#method.chain)(self) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Treat this collection as a sequence of systems. [Read more](../../prelude/trait.IntoScheduleConfigs.html#method.chain)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#480)

#### fn [chain\_ignore\_deferred](../../prelude/trait.IntoScheduleConfigs.html#method.chain_ignore_deferred)(self) -> [ScheduleConfigs](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Treat this collection as a sequence of systems. [Read more](../../prelude/trait.IntoScheduleConfigs.html#method.chain_ignore_deferred)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#51)

### impl [Schedulable](../schedule/trait.Schedulable.html "trait bevy::ecs::schedule::Schedulable") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#52)

#### type [Metadata](../schedule/trait.Schedulable.html#associatedtype.Metadata) = [GraphInfo](../schedule/struct.GraphInfo.html "struct bevy::ecs::schedule::GraphInfo")

Additional data used to configure independent scheduling. Stored in [`ScheduleConfig`](../schedule/struct.ScheduleConfig.html "struct bevy::ecs::schedule::ScheduleConfig").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#53)

#### type [GroupMetadata](../schedule/trait.Schedulable.html#associatedtype.GroupMetadata) = [Chain](../schedule/enum.Chain.html "enum bevy::ecs::schedule::Chain")

Additional data used to configure a schedulable group. Stored in [`ScheduleConfigs`](../schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#55)

#### fn [into\_config](../schedule/trait.Schedulable.html#tymethod.into_config)(self) -> [ScheduleConfig](../schedule/struct.ScheduleConfig.html "struct bevy::ecs::schedule::ScheduleConfig")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>>

Initializes a configuration from this node.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#167)

### impl [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [ApplyDeferred](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#168)

#### type [In](#associatedtype.In) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#169)

#### type [Out](#associatedtype.Out) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#151)

### impl [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [ConditionWithAccess](../schedule/struct.ConditionWithAccess.html "struct bevy::ecs::schedule::ConditionWithAccess")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#152)

#### type [In](#associatedtype.In) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#153)

#### type [Out](#associatedtype.Out) = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#60)

### impl [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [SystemWithAccess](../schedule/struct.SystemWithAccess.html "struct bevy::ecs::schedule::SystemWithAccess")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#61)

#### type [In](#associatedtype.In) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#62)

#### type [Out](#associatedtype.Out) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#136-140)

### impl<A, B, Func> [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<Func, A, B>

where Func: [Combine](../system/trait.Combine.html "trait bevy::ecs::system::Combine")<A, B> + 'static, A: [System](../../prelude/trait.System.html "trait bevy::prelude::System"), B: [System](../../prelude/trait.System.html "trait bevy::prelude::System"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#142)

#### type [In](#associatedtype.In) = <Func as [Combine](../system/trait.Combine.html "trait bevy::ecs::system::Combine")<A, B>>::[In](../system/trait.Combine.html#associatedtype.In "type bevy::ecs::system::Combine::In")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#143)

#### type [Out](#associatedtype.Out) = <Func as [Combine](../system/trait.Combine.html "trait bevy::ecs::system::Combine")<A, B>>::[Out](../system/trait.Combine.html#associatedtype.Out "type bevy::ecs::system::Combine::Out")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#378-382)

### impl<A, B> [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [PipeSystem](../system/struct.PipeSystem.html "struct bevy::ecs::system::PipeSystem")<A, B>

where A: [System](../../prelude/trait.System.html "trait bevy::prelude::System"), B: [System](../../prelude/trait.System.html "trait bevy::prelude::System"), <B as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In"): for<'a> [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")<Inner<'a> = <A as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#384)

#### type [In](#associatedtype.In) = <A as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#385)

#### type [Out](#associatedtype.Out) = <B as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#120-123)

### impl<Func, S> [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [AdapterSystem](../system/struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem")<Func, S>

where Func: [Adapt](../system/trait.Adapt.html "trait bevy::ecs::system::Adapt")<S>, S: [System](../../prelude/trait.System.html "trait bevy::prelude::System"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#125)

#### type [In](#associatedtype.In) = <Func as [Adapt](../system/trait.Adapt.html "trait bevy::ecs::system::Adapt")<S>>::[In](../system/trait.Adapt.html#associatedtype.In "type bevy::ecs::system::Adapt::In")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#126)

#### type [Out](#associatedtype.Out) = <Func as [Adapt](../system/trait.Adapt.html "trait bevy::ecs::system::Adapt")<S>>::[Out](../system/trait.Adapt.html#associatedtype.Out "type bevy::ecs::system::Adapt::Out")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#641-646)

### impl<Marker, In, Out, F> [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [FunctionSystem](../system/struct.FunctionSystem.html "struct bevy::ecs::system::FunctionSystem")<Marker, In, Out, F>

where Marker: 'static, In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, Out: 'static, F: [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>, <F as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[In](../../prelude/trait.SystemParamFunction.html#associatedtype.In "type bevy::prelude::SystemParamFunction::In"): [FromInput](../system/trait.FromInput.html "trait bevy::ecs::system::FromInput")<In>, <F as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Out](../../prelude/trait.SystemParamFunction.html#associatedtype.Out "type bevy::prelude::SystemParamFunction::Out"): [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<Out>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#648)

#### type [In](#associatedtype.In) = In

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#649)

#### type [Out](#associatedtype.Out) = Out

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#349-355)

### impl<Marker, In, Out, Func, Builder> [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [BuilderSystem](../system/struct.BuilderSystem.html "struct bevy::ecs::system::BuilderSystem")<Marker, In, Out, Func, Builder>

where Marker: 'static, In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, Out: 'static, Func: [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>, <Func as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[In](../../prelude/trait.SystemParamFunction.html#associatedtype.In "type bevy::prelude::SystemParamFunction::In"): [FromInput](../system/trait.FromInput.html "trait bevy::ecs::system::FromInput")<In>, <Func as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Out](../../prelude/trait.SystemParamFunction.html#associatedtype.Out "type bevy::prelude::SystemParamFunction::Out"): [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<Out>, Builder: [SystemParamBuilder](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<<Func as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Param](../../prelude/trait.SystemParamFunction.html#associatedtype.Param "type bevy::prelude::SystemParamFunction::Param")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#357)

#### type [In](#associatedtype.In) = In

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#359)

#### type [Out](#associatedtype.Out) = Out

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_function_system.rs.html#81-86)

### impl<Marker, Out, F> [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [ExclusiveFunctionSystem](../system/struct.ExclusiveFunctionSystem.html "struct bevy::ecs::system::ExclusiveFunctionSystem")<Marker, Out, F>

where Marker: 'static, Out: 'static, <F as [ExclusiveSystemParamFunction](../system/trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction")<Marker>>::[Out](../system/trait.ExclusiveSystemParamFunction.html#associatedtype.Out "type bevy::ecs::system::ExclusiveSystemParamFunction::Out"): [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<Out>, F: [ExclusiveSystemParamFunction](../system/trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction")<Marker>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_function_system.rs.html#88)

#### type [In](#associatedtype.In) = <F as [ExclusiveSystemParamFunction](../system/trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction")<Marker>>::[In](../system/trait.ExclusiveSystemParamFunction.html#associatedtype.In "type bevy::ecs::system::ExclusiveSystemParamFunction::In")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_function_system.rs.html#89)

#### type [Out](#associatedtype.Out) = Out

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/schedule_system.rs.html#134-137)

### impl<S, T> [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [WithInputFromWrapper](../system/struct.WithInputFromWrapper.html "struct bevy::ecs::system::WithInputFromWrapper")<S, T>

where S: for<'i> [System](../../prelude/trait.System.html "trait bevy::prelude::System"), <S as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In"): for<'i> [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")<Inner<'i> = [&'i mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/schedule_system.rs.html#139)

#### type [In](#associatedtype.In) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/schedule_system.rs.html#140)

#### type [Out](#associatedtype.Out) = <S as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/schedule_system.rs.html#47-50)

### impl<S, T> [System](../../prelude/trait.System.html "trait bevy::prelude::System") for [WithInputWrapper](../system/struct.WithInputWrapper.html "struct bevy::ecs::system::WithInputWrapper")<S, T>

where S: for<'i> [System](../../prelude/trait.System.html "trait bevy::prelude::System"), <S as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In"): for<'i> [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")<Inner<'i> = [&'i mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/schedule_system.rs.html#52)

#### type [In](#associatedtype.In) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/schedule_system.rs.html#53)

#### type [Out](#associatedtype.Out) = <S as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out")