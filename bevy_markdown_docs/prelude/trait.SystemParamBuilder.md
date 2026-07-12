[bevy](../index.html)::[prelude](index.html)

# Trait SystemParamBuilder 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#132)

```rust
pub unsafe trait SystemParamBuilder<P>: Sizedwhere
    P: SystemParam,{
    // Required method
    fn build(self, world: &mut World) -> <P as SystemParam>::State;

    // Provided methods
    fn build_state(self, world: &mut World) -> SystemState<P> { ... }
    fn build_system<Marker, In, Out, Func>(
        self,
        func: Func,
    ) -> IntoBuilderSystem<Marker, In, Out, Func, Self>
       where Self: 'static,
             Func: SystemParamFunction<Marker, Param = P> { ... }
}
```

A builder that can create a [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

```rust
fn some_system(param: MyParam) {}

fn build_system(builder: impl SystemParamBuilder<MyParam> + 'static) {
    // To build a system, create a tuple of `SystemParamBuilder`s
    // with a builder for each parameter.
    // Note that the builder for a system must be a tuple,
    // even if there is only one parameter.
    (builder,)
        .build_system(some_system);
}

fn build_system_direct(builder: impl SystemParamBuilder<MyParam>) {
    let mut world = World::new();
    // You can also construct a system in two steps, first by
    // constructing a [`SystemState`] with `build_state` and
    // second by constructing the final system with `build_system`.
    // This can be useful in cases that require type inference
    // for function parameters (like closures!), since normal
    // `build_system` requires explicitly specifying all parameter
    // types. See `build_closure_system_infer/explicit` below for more
    // info.
    (builder,)
        .build_state(&mut world)
        .build_system(some_system);
}

fn build_closure_system_infer(builder: impl SystemParamBuilder<MyParam>) {
    let mut world = World::new();
    // Closures can be used in addition to named functions.
    // If a closure is used, the parameter types must all be inferred
    // from the builders, so you cannot use plain `ParamBuilder`.
    (builder, ParamBuilder::resource())
        .build_state(&mut world)
        .build_system(|param, res| {
            let param: MyParam = param;
            let res: Res<R> = res;
        });
}

fn build_closure_system_explicit(builder: impl SystemParamBuilder<MyParam>) {
    let mut world = World::new();
    // Alternately, you can provide all types in the closure
    // parameter list and call `build_system()` normally.
    (builder, ParamBuilder::resource())
        .build_state(&mut world) // this line can be optionally omitted, since all the parameter types are explicit!
        .build_system(|param: MyParam, res: Res<R>| {});
}
```

See the documentation for individual builders for more examples.

## List of Builders

[`ParamBuilder`](../ecs/system/struct.ParamBuilder.html "struct bevy::ecs::system::ParamBuilder") can be used for parameters that don’t require any special building. Using a `ParamBuilder` will build the system parameter the same way it would be initialized in an ordinary system.

`ParamBuilder` also provides factory methods that return a `ParamBuilder` typed as `impl SystemParamBuilder<P>` for common system parameters that can be used to guide closure parameter inference.

[`QueryParamBuilder`](../ecs/system/struct.QueryParamBuilder.html "struct bevy::ecs::system::QueryParamBuilder") can build a [`Query`](struct.Query.html "struct bevy::prelude::Query") to add additional filters, or to configure the components available to [`FilteredEntityRef`](../ecs/world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef") or [`FilteredEntityMut`](../ecs/world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut"). You can also use a [`QueryState`](struct.QueryState.html "struct bevy::prelude::QueryState") to build a [`Query`](struct.Query.html "struct bevy::prelude::Query").

[`LocalBuilder`](../ecs/system/struct.LocalBuilder.html "struct bevy::ecs::system::LocalBuilder") can build a [`Local`](struct.Local.html "struct bevy::prelude::Local") to supply the initial value for the `Local`.

[`FilteredResourcesParamBuilder`](../ecs/system/struct.FilteredResourcesParamBuilder.html "struct bevy::ecs::system::FilteredResourcesParamBuilder") can build a [`FilteredResources`](struct.FilteredResources.html "struct bevy::prelude::FilteredResources"), and [`FilteredResourcesMutParamBuilder`](../ecs/system/struct.FilteredResourcesMutParamBuilder.html "struct bevy::ecs::system::FilteredResourcesMutParamBuilder") can build a [`FilteredResourcesMut`](struct.FilteredResourcesMut.html "struct bevy::prelude::FilteredResourcesMut"), to configure the resources that can be accessed.

[`DynParamBuilder`](../ecs/system/struct.DynParamBuilder.html "struct bevy::ecs::system::DynParamBuilder") can build a [`DynSystemParam`](../ecs/system/struct.DynSystemParam.html "struct bevy::ecs::system::DynSystemParam") to determine the type of the inner parameter, and to supply any `SystemParamBuilder` it needs.

Tuples of builders can build tuples of parameters, one builder for each element. Note that since systems require a tuple as a parameter, the outer builder for a system will always be a tuple.

A [`Vec`](struct.Vec.html "struct bevy::prelude::Vec") of builders can build a `Vec` of parameters, one builder for each element.

A [`ParamSetBuilder`](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder") can build a [`ParamSet`](struct.ParamSet.html "struct bevy::prelude::ParamSet"). This can wrap either a tuple or a `Vec`, one builder for each element.

A custom system param created with `#[derive(SystemParam)]` can be buildable if it includes a `#[system_param(builder)]` attribute. See [the documentation for `SystemParam` derives](../ecs/system/trait.SystemParam.html#builders "trait bevy::ecs::system::SystemParam").

## Safety

The implementor must ensure that the state returned from [`SystemParamBuilder::build`](trait.SystemParamBuilder.html#tymethod.build "method bevy::prelude::SystemParamBuilder::build") is valid for `P`. Note that the exact safety requirements depend on the implementation of [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), so if `Self` is not a local type then you must call [`SystemParam::init_state`](../ecs/system/trait.SystemParam.html#tymethod.init_state "associated function bevy::ecs::system::SystemParam::init_state") or another [`SystemParamBuilder::build`](trait.SystemParamBuilder.html#tymethod.build "method bevy::prelude::SystemParamBuilder::build").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#135)

#### fn [build](#tymethod.build)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> <P as [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../ecs/system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

Registers any [`World`](struct.World.html "struct bevy::prelude::World") access used by this [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") and creates a new instance of this param’s [`State`](../ecs/system/trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State").

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#139)

#### fn [build\_state](#method.build_state)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> [SystemState](../ecs/system/struct.SystemState.html "struct bevy::ecs::system::SystemState")<P>

Create a [`SystemState`](../ecs/system/struct.SystemState.html "struct bevy::ecs::system::SystemState") from a [`SystemParamBuilder`](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder"). To create a system, call [`SystemState::build_system`](../ecs/system/struct.SystemState.html#method.build_system "method bevy::ecs::system::SystemState::build_system") on the result.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_components.rs ([line 122](../../src/many_components/many_components.rs.html#122))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#158-164)

#### fn [build\_system](#method.build_system)<Marker, In, Out, Func>( self, func: Func, ) -> [IntoBuilderSystem](../ecs/system/struct.IntoBuilderSystem.html "struct bevy::ecs::system::IntoBuilderSystem")<Marker, In, Out, Func, Self>

where Self: 'static, Func: [SystemParamFunction](trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker, Param = P>,

Create a [`System`](trait.System.html "trait bevy::prelude::System") from a [`SystemParamBuilder`](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder") directly.

This method is useful in cases where type inference for closure parameters isn’t necessary, or where it’s not possible to call [`SystemState::build_system`](../ecs/system/struct.SystemState.html#method.build_system "method bevy::ecs::system::SystemState::build_system") by passing in an `&mut World`. Rather than constructing the system’s state immediately, this function returns a wrapper that initializes the system state during the first run.

Caveats:

*   doesn’t support parameter type inference.
*   only works for ’static system param builder types.

In cases where either of these are required, call [`SystemParamBuilder::build_state`](trait.SystemParamBuilder.html#method.build_state "method bevy::prelude::SystemParamBuilder::build_state") instead.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#607-614)

### impl [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#607-614)

#### fn [build](#tymethod.build)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../ecs/system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#626-627)

### impl<P, B, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[P; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>> for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[B; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

where P: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#629)

#### fn [build](#tymethod.build)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> <[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[P; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\> as [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../ecs/system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#607-614)

### impl<P, B> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[(P₁, P₂, …, Pₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\> for [(B₁, B₂, …, Bₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where P: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P>,

This trait is implemented for tuples up to 17 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#607-614)

#### fn [build](#tymethod.build)(self, world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> <[(P,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../ecs/system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#765)

### impl<'a, 'w, 's> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[DynSystemParam](../ecs/system/struct.DynSystemParam.html "struct bevy::ecs::system::DynSystemParam")<'w, 's>> for [DynParamBuilder](../ecs/system/struct.DynParamBuilder.html "struct bevy::ecs::system::DynParamBuilder")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#794-795)

### impl<'s, T> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[Local](struct.Local.html "struct bevy::prelude::Local")<'s, T>> for [LocalBuilder](../ecs/system/struct.LocalBuilder.html "struct bevy::ecs::system::LocalBuilder")<T>

where T: [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#563-569)

### impl<'w, 's, D, F, T> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>> for [QueryParamBuilder](../ecs/system/struct.QueryParamBuilder.html "struct bevy::ecs::system::QueryParamBuilder")<T>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static, T: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [QueryBuilder](struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder")<'\_, D, F>),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#489-490)

### impl<'w, 's, D, F> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[Query](struct.Query.html "struct bevy::prelude::Query")<'w, 's, D, F>> for [QueryState](struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") + 'static, F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#734)

### impl<'w, 's, P0, B0> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>> for [ParamSetBuilder](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")<[(B0,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B0: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P0>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#734)

### impl<'w, 's, P0, P1, B0, B1> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>> for [ParamSetBuilder](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")<[(B0, B1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B0: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P0>, B1: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P1>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#734)

### impl<'w, 's, P0, P1, P2, B0, B1, B2> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>> for [ParamSetBuilder](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")<[(B0, B1, B2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B0: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P0>, B1: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P1>, B2: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P2>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#734)

### impl<'w, 's, P0, P1, P2, P3, B0, B1, B2, B3> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>> for [ParamSetBuilder](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")<[(B0, B1, B2, B3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B0: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P0>, B1: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P1>, B2: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P2>, B3: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P3>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#734)

### impl<'w, 's, P0, P1, P2, P3, P4, B0, B1, B2, B3, B4> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>> for [ParamSetBuilder](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")<[(B0, B1, B2, B3, B4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P4: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B0: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P0>, B1: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P1>, B2: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P2>, B3: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P3>, B4: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P4>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#734)

### impl<'w, 's, P0, P1, P2, P3, P4, P5, B0, B1, B2, B3, B4, B5> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4, P5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>> for [ParamSetBuilder](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")<[(B0, B1, B2, B3, B4, B5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P4: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P5: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B0: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P0>, B1: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P1>, B2: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P2>, B3: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P3>, B4: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P4>, B5: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P5>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#734)

### impl<'w, 's, P0, P1, P2, P3, P4, P5, P6, B0, B1, B2, B3, B4, B5, B6> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4, P5, P6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>> for [ParamSetBuilder](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")<[(B0, B1, B2, B3, B4, B5, B6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P4: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P5: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P6: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B0: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P0>, B1: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P1>, B2: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P2>, B3: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P3>, B4: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P4>, B5: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P5>, B6: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P6>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#734)

### impl<'w, 's, P0, P1, P2, P3, P4, P5, P6, P7, B0, B1, B2, B3, B4, B5, B6, B7> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [(P0, P1, P2, P3, P4, P5, P6, P7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>> for [ParamSetBuilder](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")<[(B0, B1, B2, B3, B4, B5, B6, B7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where P0: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P1: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P2: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P3: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P4: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P5: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P6: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), P7: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B0: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P0>, B1: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P1>, B2: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P2>, B3: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P3>, B4: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P4>, B5: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P5>, B6: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P6>, B7: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P7>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#737-738)

### impl<'w, 's, P, B> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")<'w, 's, [Vec](struct.Vec.html "struct bevy::prelude::Vec")<P>>> for [ParamSetBuilder](../ecs/system/struct.ParamSetBuilder.html "struct bevy::ecs::system::ParamSetBuilder")<[Vec](struct.Vec.html "struct bevy::prelude::Vec")<B>>

where P: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#826-827)

### impl<'w, 's, T> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[FilteredResources](struct.FilteredResources.html "struct bevy::prelude::FilteredResources")<'w, 's>> for [FilteredResourcesParamBuilder](../ecs/system/struct.FilteredResourcesParamBuilder.html "struct bevy::ecs::system::FilteredResourcesParamBuilder")<T>

where T: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [FilteredResourcesBuilder](../ecs/world/struct.FilteredResourcesBuilder.html "struct bevy::ecs::world::FilteredResourcesBuilder")<'\_>),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#860-861)

### impl<'w, 's, T> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[FilteredResourcesMut](struct.FilteredResourcesMut.html "struct bevy::prelude::FilteredResourcesMut")<'w, 's>> for [FilteredResourcesMutParamBuilder](../ecs/system/struct.FilteredResourcesMutParamBuilder.html "struct bevy::ecs::system::FilteredResourcesMutParamBuilder")<T>

where T: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [FilteredResourcesMutBuilder](../ecs/world/struct.FilteredResourcesMutBuilder.html "struct bevy::ecs::world::FilteredResourcesMutBuilder")<'\_>),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#904)

### impl<P, B> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[If](struct.If.html "struct bevy::prelude::If")<P>> for [IfBuilder](../ecs/system/struct.IfBuilder.html "struct bevy::ecs::system::IfBuilder")<B>

where P: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#875-876)

### impl<P, B> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<P>> for [OptionBuilder](../ecs/system/struct.OptionBuilder.html "struct bevy::ecs::system::OptionBuilder")<B>

where P: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#888-889)

### impl<P, B> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<P, [SystemParamValidationError](../ecs/system/struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>> for [ResultBuilder](../ecs/system/struct.ResultBuilder.html "struct bevy::ecs::system::ResultBuilder")<B>

where P: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#617)

### impl<P, B> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<[Vec](struct.Vec.html "struct bevy::prelude::Vec")<P>> for [Vec](struct.Vec.html "struct bevy::prelude::Vec")<B>

where P: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"), B: [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#213)

### impl<P> [SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<P> for [ParamBuilder](../ecs/system/struct.ParamBuilder.html "struct bevy::ecs::system::ParamBuilder")

where P: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),