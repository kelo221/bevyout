[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait RunSystemOnce 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#353)

```rust
pub trait RunSystemOnce: Sized {
    // Required method
    fn run_system_once_with<T, In, Out, Marker>(
        self,
        system: T,
        input: <<<T as IntoSystem<In, Out, Marker>>::System as System>::In as SystemInput>::Inner<'_>,
    ) -> Result<Out, RunSystemError>
       where T: IntoSystem<In, Out, Marker>,
             In: SystemInput;

    // Provided method
    fn run_system_once<T, Out, Marker>(
        self,
        system: T,
    ) -> Result<Out, RunSystemError>
       where T: IntoSystem<(), Out, Marker> { ... }
}
```

Trait used to run a system immediately on a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Warning

This function is not an efficient method of running systems and it’s meant to be used as a utility for testing and/or diagnostics.

Systems called through [`run_system_once`](trait.RunSystemOnce.html#method.run_system_once "method bevy::ecs::system::RunSystemOnce::run_system_once") do not hold onto any state, as they are created and destroyed every time [`run_system_once`](trait.RunSystemOnce.html#method.run_system_once "method bevy::ecs::system::RunSystemOnce::run_system_once") is called. Practically, this means that [`Local`](../../prelude/struct.Local.html "struct bevy::prelude::Local") variables are reset on every run and change detection does not work.

```rust
#[derive(Resource, Default)]
struct Counter(u8);

fn increment(mut counter: Local<Counter>) {
   counter.0 += 1;
   println!("{}", counter.0);
}

let mut world = World::default();
world.run_system_once(increment); // prints 1
world.run_system_once(increment); // still prints 1
```

If you do need systems to hold onto state between runs, use [`World::run_system_cached`](../../prelude/struct.World.html#method.run_system_cached "method bevy::prelude::World::run_system_cached") or [`World::run_system`](../../prelude/struct.World.html#method.run_system "method bevy::prelude::World::run_system").

## Usage

Typically, to test a system, or to extract specific diagnostics information from a world, you’d need a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") to run the system. This can create redundant boilerplate code when writing tests or trying to quickly iterate on debug specific systems.

For these situations, this function can be useful because it allows you to execute a system immediately with some custom input and retrieve its output without requiring the necessary boilerplate.

## Examples

### Immediate Command Execution

This usage is helpful when trying to test systems or functions that operate on [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands"):

```rust
let mut world = World::default();
let entity = world.run_system_once(|mut commands: Commands| {
    commands.spawn_empty().id()
}).unwrap();
```

### Immediate Queries

This usage is helpful when trying to run an arbitrary query on a world for testing or debugging purposes:

```rust
#[derive(Component)]
struct T(usize);

let mut world = World::default();
world.spawn(T(0));
world.spawn(T(1));
world.spawn(T(1));
let count = world.run_system_once(|query: Query<&T>| {
    query.iter().filter(|t| t.0 == 1).count()
}).unwrap();
```

Note that instead of closures you can also pass in regular functions as systems:

```rust
#[derive(Component)]
struct T(usize);

fn count(query: Query<&T>) -> usize {
    query.iter().filter(|t| t.0 == 1).count()
}

let mut world = World::default();
world.spawn(T(0));
world.spawn(T(1));
world.spawn(T(1));
let count = world.run_system_once(count).unwrap();
```

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#363-370)

#### fn [run\_system\_once\_with](#tymethod.run_system_once_with)<T, In, Out, Marker>( self, system: T, input: <<<T as [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, Out, Marker>>::[System](../../prelude/trait.IntoSystem.html#associatedtype.System "type bevy::prelude::IntoSystem::System") as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

where T: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<In, Out, Marker>, In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput"),

Tries to run a system with given input and apply deferred parameters.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#355-357)

#### fn [run\_system\_once](#method.run_system_once)<T, Out, Marker>( self, system: T, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

where T: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Out, Marker>,

Tries to run a system and apply its deferred parameters.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/one\_shot\_systems.rs ([line 49](../../../src/one_shot_systems/one_shot_systems.rs.html#49))

```rust
47fn setup_with_world(world: &mut World) {
48    // We can run it once manually
49    world.run_system_once(system_b).unwrap();
50    // Or with a Callback
51    let system_id = world.register_system(system_b);
52    world.spawn((Callback(system_id), B));
53}
```

Hide additional examples

examples/ecs/relationships.rs ([line 75](../../../src/relationships/relationships.rs.html#75))

```rust
36fn main() {
37    // Operating on a raw `World` and running systems one at a time
38    // is great for writing tests and teaching abstract concepts!
39    let mut world = World::new();
40
41    // We're going to spawn a few entities and relate them to each other in a complex way.
42    // To start, Bob will target Alice, Charlie will target Bob,
43    // and Alice will target Charlie. This creates a loop in the relationship graph.
44    //
45    // Then, we'll spawn Devon, who will target Charlie,
46    // creating a more complex graph with a branching structure.
47    fn spawning_entities_with_relationships(mut commands: Commands) {
48        // Calling .id() after spawning an entity will return the `Entity` identifier of the spawned entity,
49        // even though the entity itself is not yet instantiated in the world.
50        // This works because Commands will reserve the entity ID before actually spawning the entity,
51        // through the use of atomic counters.
52        let alice = commands.spawn(Name::new("Alice")).id();
53        // Relations are just components, so we can add them into the bundle that we're spawning.
54        let bob = commands.spawn((Name::new("Bob"), Targeting(alice))).id();
55
56        // The `with_related` and `with_related_entities` helper methods on `EntityCommands` can be used to add relations in a more ergonomic way.
57        let charlie = commands
58            .spawn((Name::new("Charlie"), Targeting(bob)))
59            // The `with_related` method will spawn a bundle with `Targeting` relationship
60            .with_related::<Targeting>(Name::new("James"))
61            // The `with_related_entities` method will automatically add the `Targeting` component to any entities spawned within the closure,
62            // targeting the entity that we're calling `with_related` on.
63            .with_related_entities::<Targeting>(|related_spawner_commands| {
64                // We could spawn multiple entities here, and they would all target `charlie`.
65                related_spawner_commands.spawn(Name::new("Devon"));
66            })
67            .id();
68
69        // Simply inserting the `Targeting` component will automatically create and update the `TargetedBy` component on the target entity.
70        // We can do this at any point; not just when the entity is spawned.
71        commands.entity(alice).insert(Targeting(charlie));
72    }
73
74    world
75        .run_system_once(spawning_entities_with_relationships)
76        .unwrap();
77
78    fn debug_relationships(
79        // Not all of our entities are targeted by something, so we use `Option` in our query to handle this case.
80        relations_query: Query<(&Name, &Targeting, Option<&TargetedBy>)>,
81        name_query: Query<&Name>,
82    ) {
83        let mut relationships = String::new();
84
85        for (name, targeting, maybe_targeted_by) in relations_query.iter() {
86            let targeting_name = name_query.get(targeting.0).unwrap();
87            let targeted_by_string = if let Some(targeted_by) = maybe_targeted_by {
88                let mut vec_of_names = Vec::<&Name>::new();
89
90                for entity in targeted_by.iter() {
91                    let name = name_query.get(entity).unwrap();
92                    vec_of_names.push(name);
93                }
94
95                // Convert this to a nice string for printing.
96                let vec_of_str: Vec<&str> = vec_of_names.iter().map(|name| name.as_str()).collect();
97                vec_of_str.join(", ")
98            } else {
99                "nobody".to_string()
100            };
101
102            relationships.push_str(&format!(
103                "{name} is targeting {targeting_name}, and is targeted by {targeted_by_string}\n",
104            ));
105        }
106
107        println!("{relationships}");
108    }
109
110    world.run_system_once(debug_relationships).unwrap();
111
112    // Demonstrates how to correctly mutate relationships.
113    // Relationship components are immutable! We can't query for the `Targeting` component mutably and modify it directly,
114    // but we can insert a new `Targeting` component to replace the old one.
115    // This allows the hooks on the `Targeting` component to update the `TargetedBy` component correctly.
116    // The `TargetedBy` component will be updated automatically!
117    fn mutate_relationships(name_query: Query<(Entity, &Name)>, mut commands: Commands) {
118        // Let's find Devon by doing a linear scan of the entity names.
119        let devon = name_query
120            .iter()
121            .find(|(_entity, name)| name.as_str() == "Devon")
122            .unwrap()
123            .0;
124
125        let alice = name_query
126            .iter()
127            .find(|(_entity, name)| name.as_str() == "Alice")
128            .unwrap()
129            .0;
130
131        println!("Making Devon target Alice.\n");
132        commands.entity(devon).insert(Targeting(alice));
133    }
134
135    world.run_system_once(mutate_relationships).unwrap();
136    world.run_system_once(debug_relationships).unwrap();
137
138    // Systems can return errors,
139    // which can be used to signal that something went wrong during the system's execution.
140    #[derive(Debug)]
141    #[expect(
142        dead_code,
143        reason = "Rust considers types that are only used by their debug trait as dead code."
144    )]
145    struct TargetingCycle {
146        initial_entity: Entity,
147        visited: EntityHashSet,
148    }
149
150    /// Bevy's relationships come with all sorts of useful methods for traversal.
151    /// Here, we're going to look for cycles using a depth-first search.
152    fn check_for_cycles(
153        // We want to check every entity for cycles
154        query_to_check: Query<Entity, With<Targeting>>,
155        // Fetch the names for easier debugging.
156        name_query: Query<&Name>,
157        // The targeting_query allows us to traverse the relationship graph.
158        targeting_query: Query<&Targeting>,
159    ) -> Result<(), TargetingCycle> {
160        for initial_entity in query_to_check.iter() {
161            let mut visited = EntityHashSet::new();
162            let mut targeting_name = name_query.get(initial_entity).unwrap().clone();
163            println!("Checking for cycles starting at {targeting_name}",);
164
165            // There's all sorts of methods like this; check the `Query` docs for more!
166            // This would also be easy to do by just manually checking the `Targeting` component,
167            // and calling `query.get(targeted_entity)` on the entity that it targets in a loop.
168            for targeting in targeting_query.iter_ancestors(initial_entity) {
169                let target_name = name_query.get(targeting).unwrap();
170                println!("{targeting_name} is targeting {target_name}",);
171                targeting_name = target_name.clone();
172
173                if !visited.insert(targeting) {
174                    return Err(TargetingCycle {
175                        initial_entity,
176                        visited,
177                    });
178                }
179            }
180        }
181
182        // If we've checked all the entities and haven't found a cycle, we're good!
183        Ok(())
184    }
185
186    // Calling `world.run_system_once` on systems which return Results gives us two layers of errors:
187    // the first checks if running the system failed, and the second checks if the system itself returned an error.
188    // We're unwrapping the first, but checking the output of the system itself.
189    let cycle_result = world.run_system_once(check_for_cycles).unwrap();
190    println!("{cycle_result:?} \n");
191    // We deliberately introduced a cycle during spawning!
192    assert!(cycle_result.is_err());
193
194    // Now, let's demonstrate removing relationships and break the cycle.
195    fn untarget(mut commands: Commands, name_query: Query<(Entity, &Name)>) {
196        // Let's find Charlie by doing a linear scan of the entity names.
197        let charlie = name_query
198            .iter()
199            .find(|(_entity, name)| name.as_str() == "Charlie")
200            .unwrap()
201            .0;
202
203        // We can remove the `Targeting` component to remove the relationship
204        // and break the cycle we saw earlier.
205        println!("Removing Charlie's targeting relationship.\n");
206        commands.entity(charlie).remove::<Targeting>();
207    }
208
209    world.run_system_once(untarget).unwrap();
210    world.run_system_once(debug_relationships).unwrap();
211    // Cycle free!
212    let cycle_result = world.run_system_once(check_for_cycles).unwrap();
213    println!("{cycle_result:?} \n");
214    assert!(cycle_result.is_ok());
215}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#373)

### impl [RunSystemOnce](trait.RunSystemOnce.html "trait bevy::ecs::system::RunSystemOnce") for &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")