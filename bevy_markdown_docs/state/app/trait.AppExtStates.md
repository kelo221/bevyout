[bevy](../../index.html)::[state](../index.html)::[app](index.html)

# Trait AppExtStates 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#24)

```rust
pub trait AppExtStates {
    // Required methods
    fn init_state<S>(&mut self) -> &mut Self
       where S: FreelyMutableState + FromWorld;
    fn insert_state<S>(&mut self, state: S) -> &mut Self
       where S: FreelyMutableState;
    fn add_computed_state<S>(&mut self) -> &mut Self
       where S: ComputedStates;
    fn add_sub_state<S>(&mut self) -> &mut Self
       where S: SubStates;
    fn register_type_state<S>(&mut self) -> &mut Self
       where S: States + FromReflect + GetTypeRegistration + Typed;
    fn register_type_mutable_state<S>(&mut self) -> &mut Self
       where S: FreelyMutableState + FromReflect + GetTypeRegistration + Typed;
}
```

Available on **crate feature `bevy_app`** only.

State installation methods for [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") and [`SubApp`](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp").

## Required Methods

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#40)

#### fn [init\_state](#tymethod.init_state)<S>(&mut self) -> &mut Self

where S: [FreelyMutableState](../state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Initializes a [`State`](../../prelude/struct.State.html "struct bevy::prelude::State") with standard starting values.

This method is idempotent: it has no effect when called again using the same generic type.

Adds [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State") and [`NextState<S>`](../../prelude/enum.NextState.html "enum bevy::prelude::NextState") resources, and enables use of the [`OnEnter`](../../prelude/struct.OnEnter.html "struct bevy::prelude::OnEnter"), [`OnTransition`](../../prelude/struct.OnTransition.html "struct bevy::prelude::OnTransition") and [`OnExit`](../../prelude/struct.OnExit.html "struct bevy::prelude::OnExit") schedules. These schedules are triggered before [`Update`](../../prelude/struct.Update.html "struct bevy::prelude::Update") and at startup.

If you would like to control how other systems run based on the current state, you can emulate this behavior using the [`in_state`](../../prelude/fn.in_state.html "fn bevy::prelude::in_state") [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition").

Note that you can also apply state transitions at other points in the schedule by triggering the [`StateTransition`](../../prelude/struct.StateTransition.html "struct bevy::prelude::StateTransition") schedule manually.

The use of any states requires the presence of [`StatesPlugin`](struct.StatesPlugin.html "struct bevy::state::app::StatesPlugin") (which is included in `DefaultPlugins`).

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#54)

#### fn [insert\_state](#tymethod.insert_state)<S>(&mut self, state: S) -> &mut Self

where S: [FreelyMutableState](../state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState"),

Inserts a specific [`State`](../../prelude/struct.State.html "struct bevy::prelude::State") to the current [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") and overrides any [`State`](../../prelude/struct.State.html "struct bevy::prelude::State") previously added of the same type.

Adds [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State") and [`NextState<S>`](../../prelude/enum.NextState.html "enum bevy::prelude::NextState") resources, and enables use of the [`OnEnter`](../../prelude/struct.OnEnter.html "struct bevy::prelude::OnEnter"), [`OnTransition`](../../prelude/struct.OnTransition.html "struct bevy::prelude::OnTransition") and [`OnExit`](../../prelude/struct.OnExit.html "struct bevy::prelude::OnExit") schedules. These schedules are triggered before [`Update`](../../prelude/struct.Update.html "struct bevy::prelude::Update") and at startup.

If you would like to control how other systems run based on the current state, you can emulate this behavior using the [`in_state`](../../prelude/fn.in_state.html "fn bevy::prelude::in_state") [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition").

Note that you can also apply state transitions at other points in the schedule by triggering the [`StateTransition`](../../prelude/struct.StateTransition.html "struct bevy::prelude::StateTransition") schedule manually.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#59)

#### fn [add\_computed\_state](#tymethod.add_computed_state)<S>(&mut self) -> &mut Self

where S: [ComputedStates](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates"),

Sets up a type implementing [`ComputedStates`](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates").

This method is idempotent: it has no effect when called again using the same generic type.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#64)

#### fn [add\_sub\_state](#tymethod.add_sub_state)<S>(&mut self) -> &mut Self

where S: [SubStates](../../prelude/trait.SubStates.html "trait bevy::prelude::SubStates"),

Sets up a type implementing [`SubStates`](../../prelude/trait.SubStates.html "trait bevy::prelude::SubStates").

This method is idempotent: it has no effect when called again using the same generic type.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#71-73)

#### fn [register\_type\_state](#tymethod.register_type_state)<S>(&mut self) -> &mut Self

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

Available on **crate feature `bevy_reflect`** only.

Registers the state type `T` using [`App::register_type`](../../prelude/struct.App.html#method.register_type "method bevy::prelude::App::register_type"), and adds [`ReflectState`](../../prelude/struct.ReflectState.html "struct bevy::prelude::ReflectState") type data to `T` in the type registry.

This enables reflection code to access the state. For detailed information, see the docs on [`crate::reflect::ReflectState`](../../prelude/struct.ReflectState.html "struct bevy::prelude::ReflectState") .

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#81-83)

#### fn [register\_type\_mutable\_state](#tymethod.register_type_mutable_state)<S>(&mut self) -> &mut Self

where S: [FreelyMutableState](../state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

Available on **crate feature `bevy_reflect`** only.

Registers the state type `T` using [`App::register_type`](../../prelude/struct.App.html#method.register_type "method bevy::prelude::App::register_type"), and adds [`crate::reflect::ReflectState`](../../prelude/struct.ReflectState.html "struct bevy::prelude::ReflectState") and [`crate::reflect::ReflectFreelyMutableState`](../../prelude/struct.ReflectFreelyMutableState.html "struct bevy::prelude::ReflectFreelyMutableState") type data to `T` in the type registry.

This enables reflection code to access and modify the state. For detailed information, see the docs on [`crate::reflect::ReflectState`](../../prelude/struct.ReflectState.html "struct bevy::prelude::ReflectState") and [`crate::reflect::ReflectFreelyMutableState`](../../prelude/struct.ReflectFreelyMutableState.html "struct bevy::prelude::ReflectFreelyMutableState").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#288)

### impl [AppExtStates](../../prelude/trait.AppExtStates.html "trait bevy::prelude::AppExtStates") for [App](../../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#95)

### impl [AppExtStates](../../prelude/trait.AppExtStates.html "trait bevy::prelude::AppExtStates") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")