[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait IntoSystem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#185)

```rust
pub trait IntoSystem<In, Out, Marker>: Sizedwhere
    In: SystemInput,{
    type System: System<In = In, Out = Out>;

    // Required method
    fn into_system(this: Self) -> Self::System;

    // Provided methods
    fn pipe<B, BIn, BOut, MarkerB>(self, system: B) -> IntoPipeSystem<Self, B>
       where Out: 'static,
             B: IntoSystem<BIn, BOut, MarkerB>,
             BIn: for<'a> SystemInput<Inner<'a> = Out> { ... }
    fn map<T, F>(self, f: F) -> IntoAdapterSystem<F, Self>
       where F: Send + Sync + 'static + FnMut(Out) -> T { ... }
    fn with_input<T>(self, value: T) -> WithInputWrapper<Self::System, T>
       where In: for<'i> SystemInput<Inner<'i> = &'i mut T>,
             T: Send + Sync + 'static { ... }
    fn with_input_from<T>(self) -> WithInputFromWrapper<Self::System, T>
       where In: for<'i> SystemInput<Inner<'i> = &'i mut T>,
             T: FromWorld + Send + Sync + 'static { ... }
    fn system_type_id(&self) -> TypeId { ... }
}
```

Conversion trait to turn something into a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

Use this to get a system from a function. Also note that every system implements this trait as well.

## Usage notes

This trait should only be used as a bound for trait implementations or as an argument to a function. If a system needs to be returned from a function or stored somewhere, use [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") instead of this trait.

## Examples

```rust
use bevy_ecs::prelude::*;

fn my_system_function(a_usize_local: Local<usize>) {}

let system = IntoSystem::into_system(my_system_function);
```

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#187)

#### type [System](#associatedtype.System): [System](../../prelude/trait.System.html "trait bevy::prelude::System")<In = In, Out = Out>

The type of [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") that this instance converts into.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#190)

#### fn [into\_system](#tymethod.into_system)(this: Self) -> Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")

Turns this value into its corresponding [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#196-200)

#### fn [pipe](#method.pipe)<B, BIn, BOut, MarkerB>(self, system: B) -> [IntoPipeSystem](struct.IntoPipeSystem.html "struct bevy::ecs::system::IntoPipeSystem")<Self, B>

where Out: 'static, B: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<BIn, BOut, MarkerB>, BIn: for<'a> [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")<Inner<'a> = Out>,

Pass the output of this system `A` into a second system `B`, creating a new compound system.

The second system must have [`In<T>`](../../prelude/struct.In.html "struct bevy::prelude::In") as its first parameter, where `T` is the return type of the first system.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/system\_piping.rs ([line 21](../../../src/system_piping/system_piping.rs.html#21))

```rust
9fn main() {
10    App::new()
11        .insert_resource(Message("42".to_string()))
12        .insert_resource(OptionalWarning(Err("Got to rusty?".to_string())))
13        .add_plugins(LogPlugin {
14            level: Level::TRACE,
15            filter: "".to_string(),
16            ..default()
17        })
18        .add_systems(
19            Update,
20            (
21                parse_message_system.pipe(handler_system),
22                data_pipe_system.map(|out| info!("{out}")),
23                parse_message_system.map(|out| debug!("{out:?}")),
24                warning_pipe_system.map(|out| {
25                    if let Err(err) = out {
26                        error!("{err}");
27                    }
28                }),
29                parse_error_message_system.map(|out| {
30                    if let Err(err) = out {
31                        error!("{err}");
32                    }
33                }),
34                parse_message_system.map(drop),
35            ),
36        )
37        .run();
38}
```

Hide additional examples

examples/state/custom\_transitions.rs ([line 71](../../../src/custom_transitions/custom_transitions.rs.html#71))

```rust
64        fn build(&self, app: &mut App) {
65            app.add_systems(
66                StateTransition,
67                // The internals can generate at most one transition event of specific type per frame.
68                // We take the latest one and clear the queue.
69                last_transition::<S>
70                    // We insert the optional event into our schedule runner.
71                    .pipe(run_reenter::<S>)
72                    // State transitions are handled in three ordered steps, exposed as system sets.
73                    // We can add our systems to them, which will run the corresponding schedules when they're evaluated.
74                    // These are:
75                    // - [`ExitSchedules`] - Ran from leaf-states to root-states,
76                    // - [`TransitionSchedules`] - Ran in arbitrary order,
77                    // - [`EnterSchedules`] - Ran from root-states to leaf-states.
78                    .in_set(EnterSchedules::<S>::default()),
79            )
80            .add_systems(
81                StateTransition,
82                last_transition::<S>
83                    .pipe(run_reexit::<S>)
84                    .in_set(ExitSchedules::<S>::default()),
85            );
86        }
```

examples/ecs/error\_handling.rs ([lines 41-43](../../../src/error_handling/error_handling.rs.html#41-43))

```rust
12fn main() {
13    let mut app = App::new();
14    // By default, fallible systems that return an error will respond according to the `Severity`` in the error.
15    // These will typically panic, unless `with_severity` is used to change the severity of the error.
16    //
17    // We can change this by configuring the fallback error handler, which applies to the entire app
18    // (you can also set it for specific `World`s).
19    // Here we are using one of the built-in error handlers.
20    // Bevy provides built-in handlers for `panic`, `error`, `warn`, `info`,
21    // `debug`, `trace` and `ignore`.
22    app.set_error_handler(warn);
23
24    app.add_plugins(DefaultPlugins);
25
26    #[cfg(feature = "mesh_picking")]
27    app.add_plugins(MeshPickingPlugin);
28
29    // Fallible systems can be used the same way as regular systems. The only difference is they
30    // return a `Result<(), BevyError>` instead of a `()` (unit) type. Bevy will handle both
31    // types of systems the same way, except for the error handling.
32    app.add_systems(Startup, setup);
33
34    // Commands can also return `Result`s, which are automatically handled by the global error handler
35    // if not explicitly handled by the user.
36    app.add_systems(Startup, failing_commands);
37
38    // Individual systems can also be handled by piping the output result:
39    app.add_systems(
40        PostStartup,
41        failing_system.pipe(|result: In<Result>| {
42            let _ = result.0.inspect_err(|err| info!("captured error: {err}"));
43        }),
44    );
45
46    // Fallible observers are also supported.
47    app.add_observer(fallible_observer);
48
49    // If we run the app, we'll see the following output at startup:
50    //
51    //  WARN Encountered an error in system `fallible_systems::failing_system`: Resource not initialized
52    // ERROR fallible_systems::failing_system failed: Resource not initialized
53    //  INFO captured error: Resource not initialized
54    app.run();
55}
```

examples/stress\_tests/many\_components.rs ([line 124](../../../src/many_components/many_components.rs.html#124))

```rust
78fn stress_test(num_entities: u32, num_components: u32, num_systems: u32) {
79    let mut rng = ChaCha8Rng::seed_from_u64(42);
80    let mut app = App::default();
81    let world = app.world_mut();
82
83    // register a bunch of components
84    let component_ids: Vec<ComponentId> = (1..=num_components)
85        .map(|i| {
86            world.register_component_with_descriptor(
87                // SAFETY:
88                // * We don't implement a drop function
89                // * u8 is Sync and Send
90                unsafe {
91                    ComponentDescriptor::new_with_layout(
92                        format!("Component{i}").to_string(),
93                        StorageType::Table,
94                        Layout::new::<u8>(),
95                        None,
96                        true, // is mutable
97                        ComponentCloneBehavior::Default,
98                        None,
99                    )
100                },
101            )
102        })
103        .collect();
104
105    // fill the schedule with systems
106    let mut schedule = Schedule::new(Update);
107    for _ in 1..=num_systems {
108        let num_access_components = rng.random_range(1..10);
109        let access_components: Vec<ComponentId> = component_ids
110            .sample(&mut rng, num_access_components)
111            .copied()
112            .collect();
113        let system = (QueryParamBuilder::new(|builder| {
114            for &access_component in &access_components {
115                if rand::random::<bool>() {
116                    builder.mut_id(access_component);
117                } else {
118                    builder.ref_id(access_component);
119                }
120            }
121        }),)
122            .build_state(world)
123            .build_any_system(base_system);
124        schedule.add_systems((move || access_components.clone()).pipe(system));
125    }
126
127    // spawn a bunch of entities
128    for _ in 1..=num_entities {
129        let num_components = rng.random_range(1..10);
130        let components: Vec<ComponentId> = component_ids
131            .sample(&mut rng, num_components)
132            .copied()
133            .collect();
134
135        let mut entity = world.spawn_empty();
136        // We use `ManuallyDrop` here as we need to avoid dropping the u8's when `values` is dropped
137        // since ownership of the values is passed to the world in `insert_by_ids`.
138        // But we do want to deallocate the memory when values is dropped.
139        let mut values: Vec<ManuallyDrop<u8>> = components
140            .iter()
141            .map(|_id| ManuallyDrop::new(rng.random_range(0..255)))
142            .collect();
143        let ptrs: Vec<OwningPtr> = values
144            .iter_mut()
145            .map(|value| {
146                // SAFETY:
147                // * We don't read/write `values` binding after this and values are `ManuallyDrop`,
148                // so we have the right to drop/move the values
149                unsafe { PtrMut::from(value).promote() }
150            })
151            .collect();
152        // SAFETY:
153        // * component_id's are from the same world
154        // * `values` was initialized above, so references are valid
155        unsafe {
156            entity.insert_by_ids(&components, ptrs.into_iter());
157        }
158    }
159
160    // overwrite Update schedule in the app
161    app.add_schedule(schedule);
162    app.add_plugins(MinimalPlugins)
163        .add_plugins(DiagnosticsPlugin)
164        .add_plugins(LogPlugin::default())
165        .add_plugins(FrameTimeDiagnosticsPlugin::default())
166        .add_plugins(LogDiagnosticsPlugin::filtered(HashSet::from_iter([
167            DiagnosticPath::new("fps"),
168        ])));
169    app.run();
170}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#224-226)

#### fn [map](#method.map)<T, F>(self, f: F) -> [IntoAdapterSystem](struct.IntoAdapterSystem.html "struct bevy::ecs::system::IntoAdapterSystem")<F, Self>

where F: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Out) -> T,

Pass the output of this system into the passed function `f`, creating a new system that outputs the value returned from the function.

```rust
// Ignores the output of a system that may fail.
schedule.add_systems(my_system.map(drop));

fn my_system(res: Res<T>) -> Result<(), Err> {
    // ...
}
```

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/ecs/system\_piping.rs ([line 22](../../../src/system_piping/system_piping.rs.html#22))

```rust
9fn main() {
10    App::new()
11        .insert_resource(Message("42".to_string()))
12        .insert_resource(OptionalWarning(Err("Got to rusty?".to_string())))
13        .add_plugins(LogPlugin {
14            level: Level::TRACE,
15            filter: "".to_string(),
16            ..default()
17        })
18        .add_systems(
19            Update,
20            (
21                parse_message_system.pipe(handler_system),
22                data_pipe_system.map(|out| info!("{out}")),
23                parse_message_system.map(|out| debug!("{out:?}")),
24                warning_pipe_system.map(|out| {
25                    if let Err(err) = out {
26                        error!("{err}");
27                    }
28                }),
29                parse_error_message_system.map(|out| {
30                    if let Err(err) = out {
31                        error!("{err}");
32                    }
33                }),
34                parse_message_system.map(drop),
35            ),
36        )
37        .run();
38}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#253-256)

#### fn [with\_input](#method.with_input)<T>(self, value: T) -> [WithInputWrapper](struct.WithInputWrapper.html "struct bevy::ecs::system::WithInputWrapper")<Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), T>

where In: for<'i> [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")<Inner<'i> = [&'i mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Passes a mutable reference to `value` as input to the system each run, turning it into a system that takes no input.

`Self` can have any [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") type that takes a mutable reference to `T`, such as [`InMut`](../../prelude/struct.InMut.html "struct bevy::prelude::InMut").

##### Example

```rust
fn my_system(InMut(value): InMut<usize>) {
    *value += 1;
    if *value > 10 {
       println!("Value is greater than 10!");
    }
}

schedule.add_systems(my_system.with_input(0));
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#294-297)

#### fn [with\_input\_from](#method.with_input_from)<T>(self) -> [WithInputFromWrapper](struct.WithInputFromWrapper.html "struct bevy::ecs::system::WithInputFromWrapper")<Self::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), T>

where In: for<'i> [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")<Inner<'i> = [&'i mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Passes a mutable reference to a value of type `T` created via [`FromWorld`](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") as input to the system each run, turning it into a system that takes no input.

`Self` can have any [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") type that takes a mutable reference to `T`, such as [`InMut`](../../prelude/struct.InMut.html "struct bevy::prelude::InMut").

##### Example

```rust
struct MyData {
    value: usize,
}

impl FromWorld for MyData {
    fn from_world(world: &mut World) -> Self {
        // Fetch from the world the data needed to create `MyData`
    }
}

fn my_system(InMut(data): InMut<MyData>) {
    data.value += 1;
    if data.value > 10 {
        println!("Value is greater than 10!");
    }
}
schedule.add_systems(my_system.with_input_from::<MyData>());
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#304)

#### fn [system\_type\_id](#method.system_type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Get the [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") of the [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") produced after calling [`into_system`](../../prelude/trait.IntoSystem.html#tymethod.into_system "associated function bevy::prelude::IntoSystem::into_system").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#303-309)

### impl<A, B, IA, OA, IB, OB, MA, MB> [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<IA, OB, (IsPipeSystemMarker, OA, IB, MA, MB)> for [IntoPipeSystem](struct.IntoPipeSystem.html "struct bevy::ecs::system::IntoPipeSystem")<A, B>

where IA: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput"), A: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<IA, OA, MA>, B: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<IB, OB, MB>, IB: for<'a> [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")<Inner<'a> = OA>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#311)

#### type [System](#associatedtype.System) = [PipeSystem](struct.PipeSystem.html "struct bevy::ecs::system::PipeSystem")<<A as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<IA, OA, MA>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System"), <B as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<IB, OB, MB>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#84-89)

### impl<Func, S, I, O, M> [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<<Func as [Adapt](trait.Adapt.html "trait bevy::ecs::system::Adapt")<<S as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>>::[In](trait.Adapt.html#associatedtype.In "type bevy::ecs::system::Adapt::In"), <Func as [Adapt](trait.Adapt.html "trait bevy::ecs::system::Adapt")<<S as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>>::[Out](trait.Adapt.html#associatedtype.Out "type bevy::ecs::system::Adapt::Out"), (IsAdapterSystemMarker, I, O, M)> for [IntoAdapterSystem](struct.IntoAdapterSystem.html "struct bevy::ecs::system::IntoAdapterSystem")<Func, S>

where Func: [Adapt](trait.Adapt.html "trait bevy::ecs::system::Adapt")<<S as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>, I: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput"), S: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#91)

#### type [System](#associatedtype.System) = [AdapterSystem](struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem")<Func, <S as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#576-581)

### impl<Marker, In, Out, F> [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, Out, (IsFunctionSystem, Marker)> for F

where Marker: 'static, In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, Out: 'static, F: [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>, <F as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[In](../../prelude/trait.SystemParamFunction.html#associatedtype.In "type bevy::prelude::SystemParamFunction::In"): [FromInput](trait.FromInput.html "trait bevy::ecs::system::FromInput")<In>, <F as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Out](../../prelude/trait.SystemParamFunction.html#associatedtype.Out "type bevy::prelude::SystemParamFunction::Out"): [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<Out>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#583)

#### type [System](#associatedtype.System) = [FunctionSystem](struct.FunctionSystem.html "struct bevy::ecs::system::FunctionSystem")<Marker, In, Out, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#284-291)

### impl<Marker, In, Out, Func, Builder> [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, Out, (IsBuilderSystem, Marker)> for [IntoBuilderSystem](struct.IntoBuilderSystem.html "struct bevy::ecs::system::IntoBuilderSystem")<Marker, In, Out, Func, Builder>

where Marker: 'static, In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, Out: 'static, Func: [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>, <Func as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[In](../../prelude/trait.SystemParamFunction.html#associatedtype.In "type bevy::prelude::SystemParamFunction::In"): [FromInput](trait.FromInput.html "trait bevy::ecs::system::FromInput")<In>, <Func as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Out](../../prelude/trait.SystemParamFunction.html#associatedtype.Out "type bevy::prelude::SystemParamFunction::Out"): [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<Out>, Builder: [SystemParamBuilder](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<<Func as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Param](../../prelude/trait.SystemParamFunction.html#associatedtype.Param "type bevy::prelude::SystemParamFunction::Param")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#293)

#### type [System](#associatedtype.System) = [BuilderSystem](struct.BuilderSystem.html "struct bevy::ecs::system::BuilderSystem")<Marker, In, Out, Func, Builder>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_function_system.rs.html#56-61)

### impl<Out, Marker, F> [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<<F as [ExclusiveSystemParamFunction](trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction")<Marker>>::[In](trait.ExclusiveSystemParamFunction.html#associatedtype.In "type bevy::ecs::system::ExclusiveSystemParamFunction::In"), Out, (IsExclusiveFunctionSystem, Marker, Out)> for F

where Out: 'static, Marker: 'static, <F as [ExclusiveSystemParamFunction](trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction")<Marker>>::[Out](trait.ExclusiveSystemParamFunction.html#associatedtype.Out "type bevy::ecs::system::ExclusiveSystemParamFunction::Out"): [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<Out>, F: [ExclusiveSystemParamFunction](trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction")<Marker>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/exclusive_function_system.rs.html#63)

#### type [System](#associatedtype.System) = [ExclusiveFunctionSystem](struct.ExclusiveFunctionSystem.html "struct bevy::ecs::system::ExclusiveFunctionSystem")<Marker, Out, F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#310)

### impl<T> [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<<T as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In"), <T as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out"), [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for T

where T: [System](../../prelude/trait.System.html "trait bevy::prelude::System"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#311)

#### type [System](#associatedtype.System) = T