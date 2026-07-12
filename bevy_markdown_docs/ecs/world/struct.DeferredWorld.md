[bevy](../../index.html)::[ecs](../index.html)::[world](index.html)

# Struct DeferredWorld 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#28)

```rust
pub struct DeferredWorld<'w> { /* private fields */ }
```

A [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") reference that disallows structural ECS changes. This includes initializing resources, registering components or spawning entities.

This means that in order to add entities, for example, you will need to use commands instead of the world directly.

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#62)

### impl<'w> [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#65)

#### pub fn [reborrow](#method.reborrow)(&mut self) -> [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>

Reborrow self as a new instance of [`DeferredWorld`](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#71)

#### pub fn [commands](#method.commands)(&mut self) -> [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>

Creates a [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") instance that pushes to the world’s command queue

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/component\_hooks.rs ([line 131](../../../src/component_hooks/component_hooks.rs.html#131))

```rust
61fn setup(world: &mut World) {
62    // In order to register component hooks the component must:
63    // - not be currently in use by any entities in the world
64    // - not already have a hook of that kind registered
65    // This is to prevent overriding hooks defined in plugins and other crates as well as keeping things fast
66    world
67        .register_component_hooks::<MyComponent>()
68        // There are 4 component lifecycle hooks: `on_add`, `on_insert`, `on_discard` and `on_remove`
69        // A hook has 2 arguments:
70        // - a `DeferredWorld`, this allows access to resource and component data as well as `Commands`
71        // - a `HookContext`, this provides access to the following contextual information:
72        //   - the entity that triggered the hook
73        //   - the component id of the triggering component, this is mostly used for dynamic components
74        //   - the location of the code that caused the hook to trigger
75        //
76        // `on_add` will trigger when a component is inserted onto an entity without it
77        .on_add(
78            |mut world,
79             HookContext {
80                 entity,
81                 component_id,
82                 caller,
83                 ..
84             }| {
85                // You can access component data from within the hook
86                let value = world.get::<MyComponent>(entity).unwrap().0;
87                println!(
88                    "{component_id:?} added to {entity} with value {value:?}{}",
89                    caller
90                        .map(|location| format!("due to {location}"))
91                        .unwrap_or_default()
92                );
93                // Or access resources
94                world
95                    .resource_mut::<MyComponentIndex>()
96                    .insert(value, entity);
97                // Or send messages
98                world.write_message(MyMessage);
99            },
100        )
101        // `on_insert` will trigger when a component is inserted onto an entity,
102        // regardless of whether or not it already had it and after `on_add` if it ran
103        .on_insert(|world, _| {
104            println!("Current Index: {:?}", world.resource::<MyComponentIndex>());
105        })
106        // `on_discard` will trigger when a component is inserted onto an entity that already had it,
107        // and runs before the value is replaced.
108        // Also triggers when a component is removed from an entity, and runs before `on_remove`
109        .on_discard(|mut world, context| {
110            let value = world.get::<MyComponent>(context.entity).unwrap().0;
111            world.resource_mut::<MyComponentIndex>().remove(&value);
112        })
113        // `on_remove` will trigger when a component is removed from an entity,
114        // since it runs before the component is removed you can still access the component data
115        .on_remove(
116            |mut world,
117             HookContext {
118                 entity,
119                 component_id,
120                 caller,
121                 ..
122             }| {
123                let value = world.get::<MyComponent>(entity).unwrap().0;
124                println!(
125                    "{component_id:?} removed from {entity} with value {value:?}{}",
126                    caller
127                        .map(|location| format!("due to {location}"))
128                        .unwrap_or_default()
129                );
130                // You can also issue commands through `.commands()`
131                world.commands().entity(entity).despawn();
132            },
133        );
134}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#87-90)

#### pub fn [get\_mut](#method.get_mut)<T>(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Retrieves a mutable reference to the given `entity`’s [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type. Returns `None` if the `entity` does not have a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/ecs/error\_handling.rs ([line 132](../../../src/error_handling/error_handling.rs.html#132))

```rust
126fn fallible_observer(
127    pointer_move: On<Pointer<Move>>,
128    mut world: DeferredWorld,
129    mut step: Local<f32>,
130) -> Result {
131    let mut transform = world
132        .get_mut::<Transform>(pointer_move.entity)
133        .ok_or("No transform found.")?;
134
135    *step = if transform.translation.x > 3. {
136        -0.1
137    } else if transform.translation.x < -3. || *step == 0. {
138        0.1
139    } else {
140        *step
141    };
142
143    transform.translation.x += *step;
144
145    Ok(())
146}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#268-271)

#### pub fn [get\_entity\_mut](#method.get_entity_mut)<F>( &mut self, entities: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[DeferredMut](trait.WorldEntityFetch.html#associatedtype.DeferredMut "type bevy::ecs::world::WorldEntityFetch::DeferredMut")<'\_>, [EntityMutableFetchError](error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

where F: [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch"),

Returns [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s that expose read and write operations for the given `entities`, returning [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") if any of the given entities do not exist. Instead of immediately unwrapping the value returned from this function, prefer [`World::entity_mut`](../../prelude/struct.World.html#method.entity_mut "method bevy::prelude::World::entity_mut").

This function supports fetching a single entity or multiple entities:

*   Pass an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to receive a single [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut").
*   Pass a slice of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive a [`Vec<EntityMut>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec").
*   Pass an array of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive an equally-sized array of [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s.
*   Pass an [`&EntityHashSet`](../entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") to receive an [`EntityHashMap<EntityMut>`](../entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap").

**As [`DeferredWorld`](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld") does not allow structural changes, all returned references are [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s, which do not allow structural changes (i.e. adding/removing components or despawning the entity).**

##### Errors

*   Returns [`EntityMutableFetchError::NotSpawned`](error/enum.EntityMutableFetchError.html#variant.NotSpawned "variant bevy::ecs::world::error::EntityMutableFetchError::NotSpawned") if any of the given `entities` do not exist in the world.
    *   Only the first entity found to be missing will be returned.
*   Returns [`EntityMutableFetchError::AliasedMutability`](error/enum.EntityMutableFetchError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityMutableFetchError::AliasedMutability") if the same entity is requested multiple times.

##### Examples

For examples, see [`DeferredWorld::entity_mut`](struct.DeferredWorld.html#method.entity_mut "method bevy::ecs::world::DeferredWorld::entity_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#400)

#### pub fn [entity\_mut](#method.entity_mut)<F>( &mut self, entities: F, ) -> <F as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[DeferredMut](trait.WorldEntityFetch.html#associatedtype.DeferredMut "type bevy::ecs::world::WorldEntityFetch::DeferredMut")<'\_>

where F: [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch"),

Returns [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s that expose read and write operations for the given `entities`. This will panic if any of the given entities do not exist. Use [`DeferredWorld::get_entity_mut`](struct.DeferredWorld.html#method.get_entity_mut "method bevy::ecs::world::DeferredWorld::get_entity_mut") if you want to check for entity existence instead of implicitly panicking.

This function supports fetching a single entity or multiple entities:

*   Pass an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to receive a single [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut").
*   Pass a slice of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive a [`Vec<EntityMut>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec").
*   Pass an array of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive an equally-sized array of [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s.
*   Pass an [`&EntityHashSet`](../entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet") to receive an [`EntityHashMap<EntityMut>`](../entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap").

**As [`DeferredWorld`](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld") does not allow structural changes, all returned references are [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s, which do not allow structural changes (i.e. adding/removing components or despawning the entity).**

##### Panics

If any of the given `entities` do not exist in the world.

##### Examples

###### Single [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world: DeferredWorld = // ...

let mut entity_mut = world.entity_mut(entity);
let mut position = entity_mut.get_mut::<Position>().unwrap();
position.y = 1.0;
assert_eq!(position.x, 0.0);
```

###### Array of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world: DeferredWorld = // ...

let [mut e1_ref, mut e2_ref] = world.entity_mut([e1, e2]);
let mut e1_position = e1_ref.get_mut::<Position>().unwrap();
e1_position.x = 1.0;
assert_eq!(e1_position.x, 1.0);
let mut e2_position = e2_ref.get_mut::<Position>().unwrap();
e2_position.x = 2.0;
assert_eq!(e2_position.x, 2.0);
```

###### Slice of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world: DeferredWorld = // ...

let ids = vec![e1, e2, e3];
for mut eref in world.entity_mut(&ids[..]) {
    let mut pos = eref.get_mut::<Position>().unwrap();
    pos.y = 2.0;
    assert_eq!(pos.y, 2.0);
}
```

###### [`&EntityHashSet`](../entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world: DeferredWorld = // ...

let ids = EntityHashSet::from_iter([e1, e2, e3]);
for (_id, mut eref) in world.entity_mut(&ids) {
    let mut pos = eref.get_mut::<Position>().unwrap();
    pos.y = 2.0;
    assert_eq!(pos.y, 2.0);
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#437)

#### pub fn [entities\_and\_commands](#method.entities_and_commands)(&mut self) -> ([EntityFetcher](struct.EntityFetcher.html "struct bevy::ecs::world::EntityFetcher")<'\_>, [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>)

Simultaneously provides access to entity data and a command queue, which will be applied when the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") is next flushed.

This allows using borrowed entity data to construct commands where the borrow checker would otherwise prevent it.

See [`World::entities_and_commands`](../../prelude/struct.World.html#method.entities_and_commands "method bevy::prelude::World::entities_and_commands") for the non-deferred version.

##### Example

```rust
#[derive(Component)]
struct Targets(Vec<Entity>);
#[derive(Component)]
struct TargetedBy(Entity);

let mut world: DeferredWorld = // ...
let (entities, mut commands) = world.entities_and_commands();

let entity = entities.get(eid).unwrap();
for &target in entity.get::<Targets>().unwrap().0.iter() {
    commands.entity(target).insert(TargetedBy(eid));
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#459-462)

#### pub fn [query](#method.query)<'s, D, F>( &mut self, state: &'s mut [QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>, ) -> [Query](../../prelude/struct.Query.html "struct bevy::prelude::Query")<'\_, 's, D, F>

where D: [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Returns [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") for the given [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState"), which is used to efficiently run queries on the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") by storing and reusing the [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

##### Panics

If state is from a different world then self

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#475)

#### pub fn [resource\_mut](#method.resource_mut)<R>(&mut self) -> [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets a mutable reference to the resource of the given type

##### Panics

Panics if the resource does not exist. Use [`get_resource_mut`](struct.DeferredWorld.html#method.get_resource_mut "method bevy::ecs::world::DeferredWorld::get_resource_mut") instead if you want to handle this case.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/ecs/component\_hooks.rs ([line 95](../../../src/component_hooks/component_hooks.rs.html#95))

```rust
61fn setup(world: &mut World) {
62    // In order to register component hooks the component must:
63    // - not be currently in use by any entities in the world
64    // - not already have a hook of that kind registered
65    // This is to prevent overriding hooks defined in plugins and other crates as well as keeping things fast
66    world
67        .register_component_hooks::<MyComponent>()
68        // There are 4 component lifecycle hooks: `on_add`, `on_insert`, `on_discard` and `on_remove`
69        // A hook has 2 arguments:
70        // - a `DeferredWorld`, this allows access to resource and component data as well as `Commands`
71        // - a `HookContext`, this provides access to the following contextual information:
72        //   - the entity that triggered the hook
73        //   - the component id of the triggering component, this is mostly used for dynamic components
74        //   - the location of the code that caused the hook to trigger
75        //
76        // `on_add` will trigger when a component is inserted onto an entity without it
77        .on_add(
78            |mut world,
79             HookContext {
80                 entity,
81                 component_id,
82                 caller,
83                 ..
84             }| {
85                // You can access component data from within the hook
86                let value = world.get::<MyComponent>(entity).unwrap().0;
87                println!(
88                    "{component_id:?} added to {entity} with value {value:?}{}",
89                    caller
90                        .map(|location| format!("due to {location}"))
91                        .unwrap_or_default()
92                );
93                // Or access resources
94                world
95                    .resource_mut::<MyComponentIndex>()
96                    .insert(value, entity);
97                // Or send messages
98                world.write_message(MyMessage);
99            },
100        )
101        // `on_insert` will trigger when a component is inserted onto an entity,
102        // regardless of whether or not it already had it and after `on_add` if it ran
103        .on_insert(|world, _| {
104            println!("Current Index: {:?}", world.resource::<MyComponentIndex>());
105        })
106        // `on_discard` will trigger when a component is inserted onto an entity that already had it,
107        // and runs before the value is replaced.
108        // Also triggers when a component is removed from an entity, and runs before `on_remove`
109        .on_discard(|mut world, context| {
110            let value = world.get::<MyComponent>(context.entity).unwrap().0;
111            world.resource_mut::<MyComponentIndex>().remove(&value);
112        })
113        // `on_remove` will trigger when a component is removed from an entity,
114        // since it runs before the component is removed you can still access the component data
115        .on_remove(
116            |mut world,
117             HookContext {
118                 entity,
119                 component_id,
120                 caller,
121                 ..
122             }| {
123                let value = world.get::<MyComponent>(entity).unwrap().0;
124                println!(
125                    "{component_id:?} removed from {entity} with value {value:?}{}",
126                    caller
127                        .map(|location| format!("due to {location}"))
128                        .unwrap_or_default()
129                );
130                // You can also issue commands through `.commands()`
131                world.commands().entity(entity).despawn();
132            },
133        );
134}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#490)

#### pub fn [get\_resource\_mut](#method.get_resource_mut)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets a mutable reference to the resource of the given type if it exists

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 81](../../../src/immutable_components/immutable_components.rs.html#81))

```rust
77fn on_insert_name(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
78    let Some(&name) = world.entity(entity).get::<Name>() else {
79        unreachable!("Insert hook guarantees `Name` is available on entity")
80    };
81    let Some(mut index) = world.get_resource_mut::<NameIndex>() else {
82        return;
83    };
84
85    index.name_to_entity.insert(name, entity);
86}
87
88/// When a [`Name`] is removed or replaced, remove it from our [`NameIndex`].
89///
90/// Since all mutations to [`Name`] are captured by hooks, we know it is currently
91/// inserted in the index.
92fn on_discard_name(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
93    let Some(&name) = world.entity(entity).get::<Name>() else {
94        unreachable!("Discard hook guarantees `Name` is available on entity")
95    };
96    let Some(mut index) = world.get_resource_mut::<NameIndex>() else {
97        return;
98    };
99
100    index.name_to_entity.remove(&name);
101}
```

Hide additional examples

examples/ecs/dynamic.rs ([line 242](../../../src/dynamic/dynamic.rs.html#242))

```rust
69fn main() {
70    let mut world = World::new();
71    let mut lines = std::io::stdin().lines();
72    let mut component_names = HashMap::<String, ComponentId>::new();
73    let mut component_info = HashMap::<ComponentId, ComponentInfo>::new();
74    let mut event_names = HashMap::<String, EventKey>::new();
75
76    println!("{PROMPT}");
77    loop {
78        print!("\n> ");
79        let _ = std::io::stdout().flush();
80        let Some(Ok(line)) = lines.next() else {
81            return;
82        };
83
84        if line.is_empty() {
85            return;
86        };
87
88        let Some((first, rest)) = line.trim().split_once(|c: char| c.is_whitespace()) else {
89            match &line.chars().next() {
90                Some('c') => println!("{COMPONENT_PROMPT}"),
91                Some('s') => println!("{ENTITY_PROMPT}"),
92                Some('q') => println!("{QUERY_PROMPT}"),
93                Some('e') => println!("{EVENT_PROMPT}"),
94                Some('t') => println!("{EMIT_PROMPT}"),
95                _ => println!("{PROMPT}"),
96            }
97            continue;
98        };
99
100        match &first[0..1] {
101            "c" => {
102                rest.split(',').for_each(|component| {
103                    let mut component = component.split_whitespace();
104                    let Some(name) = component.next() else {
105                        return;
106                    };
107                    let size = match component.next().map(str::parse) {
108                        Some(Ok(size)) => size,
109                        _ => 0,
110                    };
111                    // Register our new component to the world with a layout specified by it's size
112                    // SAFETY: [u64] is Send + Sync
113                    let id = world.register_component_with_descriptor(unsafe {
114                        ComponentDescriptor::new_with_layout(
115                            name.to_string(),
116                            StorageType::Table,
117                            Layout::array::<u64>(size).unwrap(),
118                            None,
119                            true,
120                            ComponentCloneBehavior::Default,
121                            None,
122                        )
123                    });
124                    let Some(info) = world.components().get_info(id) else {
125                        return;
126                    };
127                    component_names.insert(name.to_string(), id);
128                    component_info.insert(id, info.clone());
129                    println!("Component {} created with id: {}", name, id.index());
130                });
131            }
132            "s" => {
133                let mut to_insert_ids = Vec::new();
134                let mut to_insert_data = Vec::new();
135                rest.split(',').for_each(|component| {
136                    let mut component = component.split_whitespace();
137                    let Some(name) = component.next() else {
138                        return;
139                    };
140
141                    // Get the id for the component with the given name
142                    let Some(&id) = component_names.get(name) else {
143                        println!("Component {name} does not exist");
144                        return;
145                    };
146
147                    // Calculate the length for the array based on the layout created for this component id
148                    let info = world.components().get_info(id).unwrap();
149                    let len = info.layout().size() / size_of::<u64>();
150                    let mut values: Vec<u64> = component
151                        .take(len)
152                        .filter_map(|value| value.parse::<u64>().ok())
153                        .collect();
154                    values.resize(len, 0);
155
156                    // Collect the id and array to be inserted onto our entity
157                    to_insert_ids.push(id);
158                    to_insert_data.push(values);
159                });
160
161                let mut entity = world.spawn_empty();
162
163                // Construct an `OwningPtr` for each component in `to_insert_data`
164                let to_insert_ptr = to_owning_ptrs(&mut to_insert_data);
165
166                // SAFETY:
167                // - Component ids have been taken from the same world
168                // - Each array is created to the layout specified in the world
169                unsafe {
170                    entity.insert_by_ids(&to_insert_ids, to_insert_ptr.into_iter());
171                }
172
173                println!("Entity spawned with id: {}", entity.id());
174            }
175            "q" => {
176                let mut builder = QueryBuilder::<FilteredEntityMut>::new(&mut world);
177                parse_query(rest, &mut builder, &component_names);
178                let mut query = builder.build();
179                query.iter_mut(&mut world).for_each(|filtered_entity| {
180                    let terms = filtered_entity
181                        .access()
182                        .try_iter_access()
183                        .unwrap()
184                        .map(|component_access| {
185                            let id = *component_access.index();
186                            let ptr = filtered_entity.get_by_id(id).unwrap();
187                            let info = component_info.get(&id).unwrap();
188                            let len = info.layout().size() / size_of::<u64>();
189
190                            // SAFETY:
191                            // - All components are created with layout [u64]
192                            // - len is calculated from the component descriptor
193                            let data = unsafe {
194                                std::slice::from_raw_parts_mut(
195                                    ptr.assert_unique().as_ptr().cast::<u64>(),
196                                    len,
197                                )
198                            };
199
200                            // If we have write access, increment each value once
201                            if matches!(component_access, ComponentAccessKind::Exclusive(_)) {
202                                data.iter_mut().for_each(|data| {
203                                    *data += 1;
204                                });
205                            }
206
207                            format!("{}: {:?}", info.name(), data[0..len].to_vec())
208                        })
209                        .collect::<Vec<_>>()
210                        .join(", ");
211
212                    println!("{}: {}", filtered_entity.id(), terms);
213                });
214            }
215            "e" => {
216                rest.split(',').for_each(|event| {
217                    let name = event.trim();
218                    if name.is_empty() {
219                        return;
220                    }
221
222                    // Register a ComponentId for this event, no Rust type needed.
223                    // SAFETY: ZST with no drop
224                    let event_component_id = world.register_component_with_descriptor(unsafe {
225                        ComponentDescriptor::new_with_layout(
226                            format!("event:{name}"),
227                            StorageType::Table,
228                            Layout::new::<()>(),
229                            None,
230                            false,
231                            ComponentCloneBehavior::Ignore,
232                            None,
233                        )
234                    });
235                    // SAFETY: event_component_id was just registered for this event
236                    let event_key = unsafe { EventKey::new(event_component_id) };
237                    event_names.insert(name.to_string(), event_key);
238
239                    // Build a dynamic observer that prints when the event fires.
240                    let runner: ObserverRunner = |mut world, _observer, ctx, _event, _trigger| {
241                        println!("  Observer fired!");
242                        if let Some(mut counts) = world.get_resource_mut::<EventFireCount>() {
243                            *counts.0.entry(ctx.event_key).or_insert(0) += 1;
244                        }
245                    };
246
247                    // SAFETY: event_key was just registered, runner ignores pointers
248                    let observer =
249                        unsafe { Observer::with_dynamic_runner(runner).with_event_key(event_key) };
250                    world.spawn(observer);
251
252                    println!(
253                        "Event '{name}' registered (key: {}) with a dynamic observer",
254                        event_component_id.index()
255                    );
256                });
257
258                // Ensure the counter resource exists.
259                world.init_resource::<EventFireCount>();
260            }
261            "t" => {
262                let name = rest.trim();
263                let Some(&event_key) = event_names.get(name) else {
264                    println!(
265                        "Event '{name}' does not exist. Register it first with 'event {name}'"
266                    );
267                    continue;
268                };
269
270                let mut event_data = ();
271                let mut trigger_data = ();
272                // SAFETY: event_key was registered in this world, both pointers are valid ZSTs
273                unsafe {
274                    world.trigger_dynamic(
275                        event_key,
276                        PtrMut::from(&mut event_data),
277                        PtrMut::from(&mut trigger_data),
278                    );
279                }
280
281                let count = world
282                    .get_resource::<EventFireCount>()
283                    .map_or(0, |c| c.0.get(&event_key).copied().unwrap_or(0));
284                println!("Event '{name}' triggered ({count} fires)");
285            }
286            _ => continue,
287        }
288    }
289}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#497)

#### pub fn [non\_send\_resource\_mut](#method.non_send_resource_mut)<R>(&mut self) -> [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>

where R: 'static,

👎Deprecated since 0.19.0:

use DeferredWorld::non\_send\_mut

Gets a mutable reference to a non-send resource of the given type, if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#511)

#### pub fn [non\_send\_mut](#method.non_send_mut)<R>(&mut self) -> [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>

where R: 'static,

Gets a mutable reference to the non-send data of the given type, if it exists.

##### Panics

Panics if the data does not exist. Use [`get_non_send_mut`](../../prelude/struct.World.html#method.get_non_send_mut "method bevy::prelude::World::get_non_send_mut") instead if you want to handle this case.

This function will panic if it isn’t called from the same thread that the data was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#526)

#### pub fn [get\_non\_send\_resource\_mut](#method.get_non_send_resource_mut)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>>

where R: 'static,

👎Deprecated since 0.19.0:

use DeferredWorld::get\_non\_send\_mut

Gets a mutable reference to a non-send resource of the given type, if it exists. Otherwise returns `None`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#536)

#### pub fn [get\_non\_send\_mut](#method.get_non_send_mut)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>>

where R: 'static,

Gets a mutable reference to non-send data of the given type, if it exists. Otherwise returns `None`.

##### Panics

This function will panic if it isn’t called from the same thread that the data was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#545)

#### pub fn [write\_message](#method.write_message)<M>(&mut self, message: M) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MessageId](../message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

Writes a [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message"). This method returns the [`MessageId`](../message/struct.MessageId.html "struct bevy::ecs::message::MessageId") of the written `message`, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the `message` could not be written.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/ecs/component\_hooks.rs ([line 98](../../../src/component_hooks/component_hooks.rs.html#98))

```rust
61fn setup(world: &mut World) {
62    // In order to register component hooks the component must:
63    // - not be currently in use by any entities in the world
64    // - not already have a hook of that kind registered
65    // This is to prevent overriding hooks defined in plugins and other crates as well as keeping things fast
66    world
67        .register_component_hooks::<MyComponent>()
68        // There are 4 component lifecycle hooks: `on_add`, `on_insert`, `on_discard` and `on_remove`
69        // A hook has 2 arguments:
70        // - a `DeferredWorld`, this allows access to resource and component data as well as `Commands`
71        // - a `HookContext`, this provides access to the following contextual information:
72        //   - the entity that triggered the hook
73        //   - the component id of the triggering component, this is mostly used for dynamic components
74        //   - the location of the code that caused the hook to trigger
75        //
76        // `on_add` will trigger when a component is inserted onto an entity without it
77        .on_add(
78            |mut world,
79             HookContext {
80                 entity,
81                 component_id,
82                 caller,
83                 ..
84             }| {
85                // You can access component data from within the hook
86                let value = world.get::<MyComponent>(entity).unwrap().0;
87                println!(
88                    "{component_id:?} added to {entity} with value {value:?}{}",
89                    caller
90                        .map(|location| format!("due to {location}"))
91                        .unwrap_or_default()
92                );
93                // Or access resources
94                world
95                    .resource_mut::<MyComponentIndex>()
96                    .insert(value, entity);
97                // Or send messages
98                world.write_message(MyMessage);
99            },
100        )
101        // `on_insert` will trigger when a component is inserted onto an entity,
102        // regardless of whether or not it already had it and after `on_add` if it ran
103        .on_insert(|world, _| {
104            println!("Current Index: {:?}", world.resource::<MyComponentIndex>());
105        })
106        // `on_discard` will trigger when a component is inserted onto an entity that already had it,
107        // and runs before the value is replaced.
108        // Also triggers when a component is removed from an entity, and runs before `on_remove`
109        .on_discard(|mut world, context| {
110            let value = world.get::<MyComponent>(context.entity).unwrap().0;
111            world.resource_mut::<MyComponentIndex>().remove(&value);
112        })
113        // `on_remove` will trigger when a component is removed from an entity,
114        // since it runs before the component is removed you can still access the component data
115        .on_remove(
116            |mut world,
117             HookContext {
118                 entity,
119                 component_id,
120                 caller,
121                 ..
122             }| {
123                let value = world.get::<MyComponent>(entity).unwrap().0;
124                println!(
125                    "{component_id:?} removed from {entity} with value {value:?}{}",
126                    caller
127                        .map(|location| format!("due to {location}"))
128                        .unwrap_or_default()
129                );
130                // You can also issue commands through `.commands()`
131                world.commands().entity(entity).despawn();
132            },
133        );
134}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#553)

#### pub fn [write\_message\_default](#method.write_message_default)<E>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MessageId](../message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<E>>

where E: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Writes the default value of the [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") of type `E`. This method returns the [`MessageId`](../message/struct.MessageId.html "struct bevy::ecs::message::MessageId") of the written `event`, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the `event` could not be written.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#561-564)

#### pub fn [write\_message\_batch](#method.write_message_batch)<E>( &mut self, events: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = E>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[WriteBatchIds](../message/struct.WriteBatchIds.html "struct bevy::ecs::message::WriteBatchIds")<E>>

where E: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

Writes a batch of [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s from an iterator. This method returns the [IDs](../message/struct.MessageId.html "struct bevy::ecs::message::MessageId") of the written `events`, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the `event` could not be written.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#582)

#### pub fn [get\_resource\_mut\_by\_id](#method.get_resource_mut_by_id)( &mut self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'\_>>

Gets a pointer to the resource with the id [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") if it exists. The returned pointer may be used to modify the resource, as long as the mutable borrow of the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") is still valid.

**You should prefer to use the typed API [`World::get_resource_mut`](../../prelude/struct.World.html#method.get_resource_mut "method bevy::prelude::World::get_resource_mut") where possible and only use this in cases where the actual types are not known at compile time.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#597)

#### pub fn [get\_non\_send\_mut\_by\_id](#method.get_non_send_mut_by_id)( &mut self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'\_>>

Gets mutable access to `!Send` data with the id [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") if it exists. The returned pointer may be used to modify the data, as long as the mutable borrow of the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") is still valid.

**You should prefer to use the typed API [`DeferredWorld::get_non_send_mut`](struct.DeferredWorld.html#method.get_non_send_mut "method bevy::ecs::world::DeferredWorld::get_non_send_mut") where possible and only use this in cases where the actual types are not known at compile time.**

##### Panics

This function will panic if it isn’t called from the same thread that the data was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#608-612)

#### pub fn [get\_mut\_by\_id](#method.get_mut_by_id)( &mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'\_>>

Retrieves a mutable untyped reference to the given `entity`’s [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") of the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"). Returns `None` if the `entity` does not have a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type.

**You should prefer to use the typed API [`World::get_mut`](../../prelude/struct.World.html#method.get_mut "method bevy::prelude::World::get_mut") where possible and only use this in cases where the actual types are not known at compile time.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#781-787)

#### pub unsafe fn [trigger\_raw](#method.trigger_raw)<'a, E>( &mut self, event\_key: [EventKey](../event/struct.EventKey.html "struct bevy::ecs::event::EventKey"), event: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html), trigger: &mut <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'a>, caller: [MaybeLocation](../change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation"), )

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"),

Triggers all `event` observers for the given `targets`

##### Safety

*   Caller must ensure `E` is accessible as the type represented by `event_key`

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#814)

#### pub fn [trigger](#method.trigger)<'a>(&mut self, event: impl Event : Default>)

where <impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'a>: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Sends a global [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") without any targets.

This will run any [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") of the given [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") that isn’t scoped to specific targets.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#823)

#### pub fn [as\_unsafe\_world\_cell](#method.as_unsafe_world_cell)(&mut self) -> [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>

Gets an [`UnsafeWorldCell`](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") containing the underlying world.

##### Safety

*   must only be used to make non-structural ECS changes

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#829)

#### pub fn [change\_tick](#method.change_tick)(&mut self) -> [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Gets the current change tick of [`DeferredWorld`](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld").

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [World](../../prelude/struct.World.html "struct bevy::prelude::World")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/mod.rs.html#364)

#### pub fn [event\_key](#method.event_key)<E>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[EventKey](../event/struct.EventKey.html "struct bevy::ecs::event::EventKey")\>

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"),

Fetches the [`EventKey`](../event/struct.EventKey.html "struct bevy::ecs::event::EventKey") for this event type, if it has already been generated.

This is used by various dynamically typed observer APIs, such as [`DeferredWorld::trigger_raw`](struct.DeferredWorld.html#method.trigger_raw "method bevy::ecs::world::DeferredWorld::trigger_raw").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/reflect.rs.html#69-73)

#### pub fn [get\_reflect](#method.get_reflect)( &self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static), [GetComponentReflectError](reflect/enum.GetComponentReflectError.html "enum bevy::ecs::world::reflect::GetComponentReflectError")\>

Retrieves a reference to the given `entity`’s [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") of the given `type_id` using reflection.

Requires implementing [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for the [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") (e.g., using [`#[derive(Reflect)`](../../prelude/derive.Reflect.html "derive bevy::prelude::Reflect")) and `app.register_type::<TheComponent>()` to have been called[1](#fn1).

If you want to call this with a [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), see [`World::components`](../../prelude/struct.World.html#method.components "method bevy::prelude::World::components") and [`Components::get_id`](../component/struct.Components.html#method.get_id "method bevy::ecs::component::Components::get_id") to get the corresponding [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId").

Also see the crate documentation for [`bevy_reflect`](../../reflect/index.html "mod bevy::reflect") for more information on [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and bevy’s reflection capabilities.

##### Errors

See [`GetComponentReflectError`](reflect/enum.GetComponentReflectError.html "enum bevy::ecs::world::reflect::GetComponentReflectError") for the possible errors and their descriptions.

##### Example

```rust
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use std::any::TypeId;

// define a `Component` and derive `Reflect` for it
#[derive(Component, Reflect)]
struct MyComponent;

// create a `World` for this example
let mut world = World::new();

// Note: This is usually handled by `App::register_type()`, but this example cannot use `App`.
world.init_resource::<AppTypeRegistry>();
world.get_resource_mut::<AppTypeRegistry>().unwrap().write().register::<MyComponent>();

// spawn an entity with a `MyComponent`
let entity = world.spawn(MyComponent).id();

// retrieve a reflected reference to the entity's `MyComponent`
let comp_reflected: &dyn Reflect = world.get_reflect(entity, TypeId::of::<MyComponent>()).unwrap();

// make sure we got the expected type
assert!(comp_reflected.is::<MyComponent>());
```

##### Note

Requires the `bevy_reflect` feature (included in the default features).

* * *

1.  More specifically: Requires [`TypeData`](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for [`ReflectFromPtr`](../../reflect/struct.ReflectFromPtr.html "struct bevy::reflect::ReflectFromPtr") to be registered for the given `type_id`, which is automatically handled when deriving [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and calling [`App::register_type`](../../bevy_app/struct.App.html#method.register_type). [↩](#fnref1)
    

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#198)

#### pub fn [id](#method.id)(&self) -> [WorldId](struct.WorldId.html "struct bevy::ecs::world::WorldId")

Retrieves this [`World`](../../prelude/struct.World.html "struct bevy::prelude::World")’s unique ID

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#210)

#### pub fn [as\_unsafe\_world\_cell\_readonly](#method.as_unsafe_world_cell_readonly)(&self) -> [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>

Creates a new [`UnsafeWorldCell`](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") view with only read access to everything.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#216)

#### pub fn [entities](#method.entities)(&self) -> &[Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities")

Retrieves this world’s [`Entities`](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities") collection.

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/ecs/callbacks.rs ([line 37](../../../src/callbacks/callbacks.rs.html#37))

```rust
21fn setup_callbacks(mut commands: Commands) {
22    let trivial_callback = Callback {
23        system_id: commands.register_system(|| {
24            println!("This is the trivial callback system");
25        }),
26    };
27
28    let ordinary_system_callback = Callback {
29        system_id: commands.register_system(|query: Query<&Callback>| {
30            let n_callbacks = query.iter().len();
31            println!("This is the ordinary callback system. There are currently {n_callbacks} callbacks in the world.");
32        }),
33    };
34
35    let exclusive_callback = Callback {
36        system_id: commands.register_system(|world: &mut World| {
37            let n_entities = world.entities().len();
38            println!("This is the exclusive callback system. There are currently {n_entities} entities in the world.");
39        }),
40    };
41
42    commands.spawn(trivial_callback);
43    commands.spawn(ordinary_system_callback);
44    commands.spawn(exclusive_callback);
45}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#222)

#### pub fn [entity\_allocator](#method.entity_allocator)(&self) -> &[EntityAllocator](../entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator")

Retrieves this world’s [`EntityAllocator`](../entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator") collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#246)

#### pub fn [entity\_count](#method.entity_count)(&self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Retrieves the number of [`Entities`](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities") in the world.

This is helpful as a diagnostic, but it can also be used effectively in tests.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#252)

#### pub fn [archetypes](#method.archetypes)(&self) -> &[Archetypes](../archetype/struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes")

Retrieves this world’s [`Archetypes`](../archetype/struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes") collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#258)

#### pub fn [components](#method.components)(&self) -> &[Components](../component/struct.Components.html "struct bevy::ecs::component::Components")

Retrieves this world’s [`Components`](../component/struct.Components.html "struct bevy::ecs::component::Components") collection.

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/ecs/dynamic.rs ([line 124](../../../src/dynamic/dynamic.rs.html#124))

```rust
69fn main() {
70    let mut world = World::new();
71    let mut lines = std::io::stdin().lines();
72    let mut component_names = HashMap::<String, ComponentId>::new();
73    let mut component_info = HashMap::<ComponentId, ComponentInfo>::new();
74    let mut event_names = HashMap::<String, EventKey>::new();
75
76    println!("{PROMPT}");
77    loop {
78        print!("\n> ");
79        let _ = std::io::stdout().flush();
80        let Some(Ok(line)) = lines.next() else {
81            return;
82        };
83
84        if line.is_empty() {
85            return;
86        };
87
88        let Some((first, rest)) = line.trim().split_once(|c: char| c.is_whitespace()) else {
89            match &line.chars().next() {
90                Some('c') => println!("{COMPONENT_PROMPT}"),
91                Some('s') => println!("{ENTITY_PROMPT}"),
92                Some('q') => println!("{QUERY_PROMPT}"),
93                Some('e') => println!("{EVENT_PROMPT}"),
94                Some('t') => println!("{EMIT_PROMPT}"),
95                _ => println!("{PROMPT}"),
96            }
97            continue;
98        };
99
100        match &first[0..1] {
101            "c" => {
102                rest.split(',').for_each(|component| {
103                    let mut component = component.split_whitespace();
104                    let Some(name) = component.next() else {
105                        return;
106                    };
107                    let size = match component.next().map(str::parse) {
108                        Some(Ok(size)) => size,
109                        _ => 0,
110                    };
111                    // Register our new component to the world with a layout specified by it's size
112                    // SAFETY: [u64] is Send + Sync
113                    let id = world.register_component_with_descriptor(unsafe {
114                        ComponentDescriptor::new_with_layout(
115                            name.to_string(),
116                            StorageType::Table,
117                            Layout::array::<u64>(size).unwrap(),
118                            None,
119                            true,
120                            ComponentCloneBehavior::Default,
121                            None,
122                        )
123                    });
124                    let Some(info) = world.components().get_info(id) else {
125                        return;
126                    };
127                    component_names.insert(name.to_string(), id);
128                    component_info.insert(id, info.clone());
129                    println!("Component {} created with id: {}", name, id.index());
130                });
131            }
132            "s" => {
133                let mut to_insert_ids = Vec::new();
134                let mut to_insert_data = Vec::new();
135                rest.split(',').for_each(|component| {
136                    let mut component = component.split_whitespace();
137                    let Some(name) = component.next() else {
138                        return;
139                    };
140
141                    // Get the id for the component with the given name
142                    let Some(&id) = component_names.get(name) else {
143                        println!("Component {name} does not exist");
144                        return;
145                    };
146
147                    // Calculate the length for the array based on the layout created for this component id
148                    let info = world.components().get_info(id).unwrap();
149                    let len = info.layout().size() / size_of::<u64>();
150                    let mut values: Vec<u64> = component
151                        .take(len)
152                        .filter_map(|value| value.parse::<u64>().ok())
153                        .collect();
154                    values.resize(len, 0);
155
156                    // Collect the id and array to be inserted onto our entity
157                    to_insert_ids.push(id);
158                    to_insert_data.push(values);
159                });
160
161                let mut entity = world.spawn_empty();
162
163                // Construct an `OwningPtr` for each component in `to_insert_data`
164                let to_insert_ptr = to_owning_ptrs(&mut to_insert_data);
165
166                // SAFETY:
167                // - Component ids have been taken from the same world
168                // - Each array is created to the layout specified in the world
169                unsafe {
170                    entity.insert_by_ids(&to_insert_ids, to_insert_ptr.into_iter());
171                }
172
173                println!("Entity spawned with id: {}", entity.id());
174            }
175            "q" => {
176                let mut builder = QueryBuilder::<FilteredEntityMut>::new(&mut world);
177                parse_query(rest, &mut builder, &component_names);
178                let mut query = builder.build();
179                query.iter_mut(&mut world).for_each(|filtered_entity| {
180                    let terms = filtered_entity
181                        .access()
182                        .try_iter_access()
183                        .unwrap()
184                        .map(|component_access| {
185                            let id = *component_access.index();
186                            let ptr = filtered_entity.get_by_id(id).unwrap();
187                            let info = component_info.get(&id).unwrap();
188                            let len = info.layout().size() / size_of::<u64>();
189
190                            // SAFETY:
191                            // - All components are created with layout [u64]
192                            // - len is calculated from the component descriptor
193                            let data = unsafe {
194                                std::slice::from_raw_parts_mut(
195                                    ptr.assert_unique().as_ptr().cast::<u64>(),
196                                    len,
197                                )
198                            };
199
200                            // If we have write access, increment each value once
201                            if matches!(component_access, ComponentAccessKind::Exclusive(_)) {
202                                data.iter_mut().for_each(|data| {
203                                    *data += 1;
204                                });
205                            }
206
207                            format!("{}: {:?}", info.name(), data[0..len].to_vec())
208                        })
209                        .collect::<Vec<_>>()
210                        .join(", ");
211
212                    println!("{}: {}", filtered_entity.id(), terms);
213                });
214            }
215            "e" => {
216                rest.split(',').for_each(|event| {
217                    let name = event.trim();
218                    if name.is_empty() {
219                        return;
220                    }
221
222                    // Register a ComponentId for this event, no Rust type needed.
223                    // SAFETY: ZST with no drop
224                    let event_component_id = world.register_component_with_descriptor(unsafe {
225                        ComponentDescriptor::new_with_layout(
226                            format!("event:{name}"),
227                            StorageType::Table,
228                            Layout::new::<()>(),
229                            None,
230                            false,
231                            ComponentCloneBehavior::Ignore,
232                            None,
233                        )
234                    });
235                    // SAFETY: event_component_id was just registered for this event
236                    let event_key = unsafe { EventKey::new(event_component_id) };
237                    event_names.insert(name.to_string(), event_key);
238
239                    // Build a dynamic observer that prints when the event fires.
240                    let runner: ObserverRunner = |mut world, _observer, ctx, _event, _trigger| {
241                        println!("  Observer fired!");
242                        if let Some(mut counts) = world.get_resource_mut::<EventFireCount>() {
243                            *counts.0.entry(ctx.event_key).or_insert(0) += 1;
244                        }
245                    };
246
247                    // SAFETY: event_key was just registered, runner ignores pointers
248                    let observer =
249                        unsafe { Observer::with_dynamic_runner(runner).with_event_key(event_key) };
250                    world.spawn(observer);
251
252                    println!(
253                        "Event '{name}' registered (key: {}) with a dynamic observer",
254                        event_component_id.index()
255                    );
256                });
257
258                // Ensure the counter resource exists.
259                world.init_resource::<EventFireCount>();
260            }
261            "t" => {
262                let name = rest.trim();
263                let Some(&event_key) = event_names.get(name) else {
264                    println!(
265                        "Event '{name}' does not exist. Register it first with 'event {name}'"
266                    );
267                    continue;
268                };
269
270                let mut event_data = ();
271                let mut trigger_data = ();
272                // SAFETY: event_key was registered in this world, both pointers are valid ZSTs
273                unsafe {
274                    world.trigger_dynamic(
275                        event_key,
276                        PtrMut::from(&mut event_data),
277                        PtrMut::from(&mut trigger_data),
278                    );
279                }
280
281                let count = world
282                    .get_resource::<EventFireCount>()
283                    .map_or(0, |c| c.0.get(&event_key).copied().unwrap_or(0));
284                println!("Event '{name}' triggered ({count} fires)");
285            }
286            _ => continue,
287        }
288    }
289}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#264)

#### pub fn [resource\_entities](#method.resource_entities)(&self) -> &[ResourceEntities](../resource/struct.ResourceEntities.html "struct bevy::ecs::resource::ResourceEntities")

Retrieves this world’s [`ResourceEntities`](../resource/struct.ResourceEntities.html "struct bevy::ecs::resource::ResourceEntities").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#272)

#### pub fn [components\_queue](#method.components_queue)(&self) -> [ComponentsQueuedRegistrator](../component/struct.ComponentsQueuedRegistrator.html "struct bevy::ecs::component::ComponentsQueuedRegistrator")<'\_>

Prepares a [`ComponentsQueuedRegistrator`](../component/struct.ComponentsQueuedRegistrator.html "struct bevy::ecs::component::ComponentsQueuedRegistrator") for the world. **NOTE:** [`ComponentsQueuedRegistrator`](../component/struct.ComponentsQueuedRegistrator.html "struct bevy::ecs::component::ComponentsQueuedRegistrator") is easily misused. See its docs for important notes on when and how it should be used.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#286)

#### pub fn [storages](#method.storages)(&self) -> &[Storages](../storage/struct.Storages.html "struct bevy::ecs::storage::Storages")

Retrieves this world’s [`Storages`](../storage/struct.Storages.html "struct bevy::ecs::storage::Storages") collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#292)

#### pub fn [bundles](#method.bundles)(&self) -> &[Bundles](../bundle/struct.Bundles.html "struct bevy::ecs::bundle::Bundles")

Retrieves this world’s [`Bundles`](../bundle/struct.Bundles.html "struct bevy::ecs::bundle::Bundles") collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#298)

#### pub fn [removed\_components](#method.removed_components)(&self) -> &[RemovedComponentMessages](../lifecycle/struct.RemovedComponentMessages.html "struct bevy::ecs::lifecycle::RemovedComponentMessages")

Retrieves this world’s [`RemovedComponentMessages`](../lifecycle/struct.RemovedComponentMessages.html "struct bevy::ecs::lifecycle::RemovedComponentMessages") collection

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#304)

#### pub fn [observers](#method.observers)(&self) -> &[Observers](../observer/struct.Observers.html "struct bevy::ecs::observer::Observers")

Retrieves this world’s [`Observers`](../observer/struct.Observers.html "struct bevy::ecs::observer::Observers") list

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#589)

#### pub fn [get\_required\_components](#method.get_required_components)<C>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[RequiredComponents](../component/struct.RequiredComponents.html "struct bevy::ecs::component::RequiredComponents")\>

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Retrieves the [required components](../component/struct.RequiredComponents.html "struct bevy::ecs::component::RequiredComponents") for the given component type, if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#596)

#### pub fn [get\_required\_components\_by\_id](#method.get_required_components_by_id)( &self, id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[RequiredComponents](../component/struct.RequiredComponents.html "struct bevy::ecs::component::RequiredComponents")\>

Retrieves the [required components](../component/struct.RequiredComponents.html "struct bevy::ecs::component::RequiredComponents") for the component of the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#645)

#### pub fn [component\_id](#method.component_id)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Returns the [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") of the given [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") type `T`.

The returned `ComponentId` is specific to the `World` instance it was retrieved from and should not be used with another `World` instance.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the `Component` type has not yet been initialized within the `World` using [`World::register_component`](../../prelude/struct.World.html#method.register_component "method bevy::prelude::World::register_component").

```rust
use bevy_ecs::prelude::*;

let mut world = World::new();

#[derive(Component)]
struct ComponentA;

let component_a_id = world.register_component::<ComponentA>();

assert_eq!(component_a_id, world.component_id::<ComponentA>().unwrap())
```

##### See also

*   [`ComponentIdFor`](../component/struct.ComponentIdFor.html "struct bevy::ecs::component::ComponentIdFor")
*   [`Components::component_id()`](../component/struct.Components.html#method.component_id "method bevy::ecs::component::Components::component_id")
*   [`Components::get_id()`](../component/struct.Components.html#method.get_id "method bevy::ecs::component::Components::get_id")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#667)

#### pub fn [resource\_id](#method.resource_id)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

👎Deprecated since 0.19.0:

use component\_id

Returns the [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") of the given [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") type `T`.

The returned [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") is specific to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") instance it was retrieved from and should not be used with another [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") instance.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") type has not yet been initialized within the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") using [`World::register_resource`](../../prelude/struct.World.html#method.register_resource "method bevy::prelude::World::register_resource"), [`World::init_resource`](../../prelude/struct.World.html#method.init_resource "method bevy::prelude::World::init_resource") or [`World::insert_resource`](../../prelude/struct.World.html#method.insert_resource "method bevy::prelude::World::insert_resource").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#770)

#### pub fn [entity](#method.entity)<F>(&self, entities: F) -> <F as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Ref](trait.WorldEntityFetch.html#associatedtype.Ref "type bevy::ecs::world::WorldEntityFetch::Ref")<'\_>

where F: [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch"),

Returns [`EntityRef`](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")s that expose read-only operations for the given `entities`. This will panic if any of the given entities do not exist. Use [`World::get_entity`](../../prelude/struct.World.html#method.get_entity "method bevy::prelude::World::get_entity") if you want to check for entity existence instead of implicitly panicking.

This function supports fetching a single entity or multiple entities:

*   Pass an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to receive a single [`EntityRef`](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef").
*   Pass a slice of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive a [`Vec<EntityRef>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec").
*   Pass an array of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive an equally-sized array of [`EntityRef`](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")s.

##### Panics

If any of the given `entities` do not exist in the world.

##### Examples

###### Single [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
let entity = world.spawn(Position { x: 0.0, y: 0.0 }).id();

let position = world.entity(entity).get::<Position>().unwrap();
assert_eq!(position.x, 0.0);
```

###### Array of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
let e1 = world.spawn(Position { x: 0.0, y: 0.0 }).id();
let e2 = world.spawn(Position { x: 1.0, y: 1.0 }).id();

let [e1_ref, e2_ref] = world.entity([e1, e2]);
let e1_position = e1_ref.get::<Position>().unwrap();
assert_eq!(e1_position.x, 0.0);
let e2_position = e2_ref.get::<Position>().unwrap();
assert_eq!(e2_position.x, 1.0);
```

###### Slice of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
let e1 = world.spawn(Position { x: 0.0, y: 1.0 }).id();
let e2 = world.spawn(Position { x: 0.0, y: 1.0 }).id();
let e3 = world.spawn(Position { x: 0.0, y: 1.0 }).id();

let ids = vec![e1, e2, e3];
for eref in world.entity(&ids[..]) {
    assert_eq!(eref.get::<Position>().unwrap().y, 1.0);
}
```

###### [`EntityHashSet`](../entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
let e1 = world.spawn(Position { x: 0.0, y: 1.0 }).id();
let e2 = world.spawn(Position { x: 0.0, y: 1.0 }).id();
let e3 = world.spawn(Position { x: 0.0, y: 1.0 }).id();

let ids = EntityHashSet::from_iter([e1, e2, e3]);
for (_id, eref) in world.entity(&ids) {
    assert_eq!(eref.get::<Position>().unwrap().y, 1.0);
}
```

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 78](../../../src/immutable_components/immutable_components.rs.html#78))

```rust
77fn on_insert_name(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
78    let Some(&name) = world.entity(entity).get::<Name>() else {
79        unreachable!("Insert hook guarantees `Name` is available on entity")
80    };
81    let Some(mut index) = world.get_resource_mut::<NameIndex>() else {
82        return;
83    };
84
85    index.name_to_entity.insert(name, entity);
86}
87
88/// When a [`Name`] is removed or replaced, remove it from our [`NameIndex`].
89///
90/// Since all mutations to [`Name`] are captured by hooks, we know it is currently
91/// inserted in the index.
92fn on_discard_name(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
93    let Some(&name) = world.entity(entity).get::<Name>() else {
94        unreachable!("Discard hook guarantees `Name` is available on entity")
95    };
96    let Some(mut index) = world.get_resource_mut::<NameIndex>() else {
97        return;
98    };
99
100    index.name_to_entity.remove(&name);
101}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#912-915)

#### pub fn [inspect\_entity](#method.inspect_entity)( &self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &[ComponentInfo](../component/struct.ComponentInfo.html "struct bevy::ecs::component::ComponentInfo")\>, [EntityNotSpawnedError](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

Returns the components of an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") through [`ComponentInfo`](../component/struct.ComponentInfo.html "struct bevy::ecs::component::ComponentInfo").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#951-954)

#### pub fn [get\_entity](#method.get_entity)<F>( &self, entities: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Ref](trait.WorldEntityFetch.html#associatedtype.Ref "type bevy::ecs::world::WorldEntityFetch::Ref")<'\_>, [EntityNotSpawnedError](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

where F: [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch"),

Returns [`EntityRef`](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")s that expose read-only operations for the given `entities`, returning [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") if any of the given entities do not exist. Instead of immediately unwrapping the value returned from this function, prefer [`World::entity`](../../prelude/struct.World.html#method.entity "method bevy::prelude::World::entity").

This function supports fetching a single entity or multiple entities:

*   Pass an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to receive a single [`EntityRef`](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef").
*   Pass a slice of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive a [`Vec<EntityRef>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec").
*   Pass an array of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive an equally-sized array of [`EntityRef`](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")s.
*   Pass a reference to a [`EntityHashSet`](../entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") to receive an [`EntityHashMap<EntityRef>`](../entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap").

##### Errors

If any of the given `entities` do not exist in the world, the first [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") found to be missing will return an [`EntityNotSpawnedError`](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError").

##### Examples

For examples, see [`World::entity`](../../prelude/struct.World.html#method.entity "method bevy::prelude::World::entity").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1010)

#### pub fn [iter\_entities](#method.iter_entities)(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>>

Returns an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") iterator of current entities.

This is useful in contexts where you only have immutable access to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). If you have mutable access to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), use [`query()::<EntityRef>().iter(&world)`](../../prelude/struct.World.html#method.query "method bevy::prelude::World::query") instead.

Note that this does iterate through _all_ entities, including resource entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1366)

#### pub fn [get](#method.get)<T>(&self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Retrieves a reference to the given `entity`’s [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type. Returns `None` if the `entity` does not have a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type.

```rust
use bevy_ecs::{component::Component, world::World};

#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
let entity = world.spawn(Position { x: 0.0, y: 0.0 }).id();
let position = world.get::<Position>(entity).unwrap();
assert_eq!(position.x, 0.0);
```

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/ecs/component\_hooks.rs ([line 86](../../../src/component_hooks/component_hooks.rs.html#86))

```rust
61fn setup(world: &mut World) {
62    // In order to register component hooks the component must:
63    // - not be currently in use by any entities in the world
64    // - not already have a hook of that kind registered
65    // This is to prevent overriding hooks defined in plugins and other crates as well as keeping things fast
66    world
67        .register_component_hooks::<MyComponent>()
68        // There are 4 component lifecycle hooks: `on_add`, `on_insert`, `on_discard` and `on_remove`
69        // A hook has 2 arguments:
70        // - a `DeferredWorld`, this allows access to resource and component data as well as `Commands`
71        // - a `HookContext`, this provides access to the following contextual information:
72        //   - the entity that triggered the hook
73        //   - the component id of the triggering component, this is mostly used for dynamic components
74        //   - the location of the code that caused the hook to trigger
75        //
76        // `on_add` will trigger when a component is inserted onto an entity without it
77        .on_add(
78            |mut world,
79             HookContext {
80                 entity,
81                 component_id,
82                 caller,
83                 ..
84             }| {
85                // You can access component data from within the hook
86                let value = world.get::<MyComponent>(entity).unwrap().0;
87                println!(
88                    "{component_id:?} added to {entity} with value {value:?}{}",
89                    caller
90                        .map(|location| format!("due to {location}"))
91                        .unwrap_or_default()
92                );
93                // Or access resources
94                world
95                    .resource_mut::<MyComponentIndex>()
96                    .insert(value, entity);
97                // Or send messages
98                world.write_message(MyMessage);
99            },
100        )
101        // `on_insert` will trigger when a component is inserted onto an entity,
102        // regardless of whether or not it already had it and after `on_add` if it ran
103        .on_insert(|world, _| {
104            println!("Current Index: {:?}", world.resource::<MyComponentIndex>());
105        })
106        // `on_discard` will trigger when a component is inserted onto an entity that already had it,
107        // and runs before the value is replaced.
108        // Also triggers when a component is removed from an entity, and runs before `on_remove`
109        .on_discard(|mut world, context| {
110            let value = world.get::<MyComponent>(context.entity).unwrap().0;
111            world.resource_mut::<MyComponentIndex>().remove(&value);
112        })
113        // `on_remove` will trigger when a component is removed from an entity,
114        // since it runs before the component is removed you can still access the component data
115        .on_remove(
116            |mut world,
117             HookContext {
118                 entity,
119                 component_id,
120                 caller,
121                 ..
122             }| {
123                let value = world.get::<MyComponent>(entity).unwrap().0;
124                println!(
125                    "{component_id:?} removed from {entity} with value {value:?}{}",
126                    caller
127                        .map(|location| format!("due to {location}"))
128                        .unwrap_or_default()
129                );
130                // You can also issue commands through `.commands()`
131                world.commands().entity(entity).despawn();
132            },
133        );
134}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1878)

#### pub fn [try\_query](#method.try_query)<D>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D>>

where D: [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData"),

Returns [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") for the given [`QueryData`](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), which is used to efficiently run queries on the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") by storing and reusing the [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

```rust
use bevy_ecs::{component::Component, entity::Entity, world::World};

#[derive(Component, Debug, PartialEq)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
world.spawn_batch(vec![
    Position { x: 0.0, y: 0.0 },
    Position { x: 1.0, y: 1.0 },
]);

fn get_positions(world: &World) -> Vec<(Entity, &Position)> {
    let mut query = world.try_query::<(Entity, &Position)>().unwrap();
    query.iter(world).collect()
}

let positions = get_positions(&world);

assert_eq!(world.get::<Position>(positions[0].0).unwrap(), positions[0].1);
assert_eq!(world.get::<Position>(positions[1].0).unwrap(), positions[1].1);
```

Requires only an immutable world reference, but may fail if, for example, the components that make up this query have not been registered into the world.

```rust
use bevy_ecs::{component::Component, entity::Entity, world::World};

#[derive(Component)]
struct A;

let mut world = World::new();

let none_query = world.try_query::<&A>();
assert!(none_query.is_none());

world.register_component::<A>();

let some_query = world.try_query::<&A>();
assert!(some_query.is_some());
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1905)

#### pub fn [try\_query\_filtered](#method.try_query_filtered)<D, F>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[QueryState](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>>

where D: [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Returns [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") for the given filtered [`QueryData`](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), which is used to efficiently run queries on the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") by storing and reusing the [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

```rust
use bevy_ecs::{component::Component, entity::Entity, world::World, query::With};

#[derive(Component)]
struct A;
#[derive(Component)]
struct B;

let mut world = World::new();
let e1 = world.spawn(A).id();
let e2 = world.spawn((A, B)).id();

let mut query = world.try_query_filtered::<Entity, With<B>>().unwrap();
let matching_entities = query.iter(&world).collect::<Vec<Entity>>();

assert_eq!(matching_entities, vec![e2]);
```

Requires only an immutable world reference, but may fail if, for example, the components that make up this query have not been registered into the world.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1911)

#### pub fn [removed](#method.removed)<T>(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Returns an iterator of entities that had components of type `T` removed since the last call to [`World::clear_trackers`](../../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1921)

#### pub fn [removed\_with\_id](#method.removed_with_id)( &self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

Returns an iterator of entities that had components with the given `component_id` removed since the last call to [`World::clear_trackers`](../../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2124)

#### pub fn [contains\_resource](#method.contains_resource)<R>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Returns `true` if a resource of type `R` exists. Otherwise returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2132)

#### pub fn [contains\_resource\_by\_id](#method.contains_resource_by_id)(&self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if a resource with provided `component_id` exists. Otherwise returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2143)

#### pub fn [contains\_non\_send](#method.contains_non_send)<R>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where R: 'static,

Returns `true` if `!Send` data of type `R` exists. Otherwise returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2152)

#### pub fn [contains\_non\_send\_by\_id](#method.contains_non_send_by_id)(&self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if `!Send` data with `component_id` exists. Otherwise returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2166)

#### pub fn [is\_resource\_added](#method.is_resource_added)<R>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Returns `true` if a resource of type `R` exists and was added since the world’s [`last_change_tick`](../../prelude/struct.World.html#method.last_change_tick "method bevy::prelude::World::last_change_tick"). Otherwise, this returns `false`.

This means that:

*   When called from an exclusive system, this will check for additions since the system last ran.
*   When called elsewhere, this will check for additions since the last time that [`World::clear_trackers`](../../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2179)

#### pub fn [is\_resource\_added\_by\_id](#method.is_resource_added_by_id)(&self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if a resource with id `component_id` exists and was added since the world’s [`last_change_tick`](../../prelude/struct.World.html#method.last_change_tick "method bevy::prelude::World::last_change_tick"). Otherwise, this returns `false`.

This means that:

*   When called from an exclusive system, this will check for additions since the system last ran.
*   When called elsewhere, this will check for additions since the last time that [`World::clear_trackers`](../../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2191)

#### pub fn [is\_resource\_changed](#method.is_resource_changed)<R>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Returns `true` if a resource of type `R` exists and was modified since the world’s [`last_change_tick`](../../prelude/struct.World.html#method.last_change_tick "method bevy::prelude::World::last_change_tick"). Otherwise, this returns `false`.

This means that:

*   When called from an exclusive system, this will check for changes since the system last ran.
*   When called elsewhere, this will check for changes since the last time that [`World::clear_trackers`](../../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2204)

#### pub fn [is\_resource\_changed\_by\_id](#method.is_resource_changed_by_id)(&self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if a resource with id `component_id` exists and was modified since the world’s [`last_change_tick`](../../prelude/struct.World.html#method.last_change_tick "method bevy::prelude::World::last_change_tick"). Otherwise, this returns `false`.

This means that:

*   When called from an exclusive system, this will check for changes since the system last ran.
*   When called elsewhere, this will check for changes since the last time that [`World::clear_trackers`](../../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2210)

#### pub fn [get\_resource\_change\_ticks](#method.get_resource_change_ticks)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentTicks](../change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")\>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Retrieves the change ticks for the given resource.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2219-2222)

#### pub fn [get\_resource\_change\_ticks\_by\_id](#method.get_resource_change_ticks_by_id)( &self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentTicks](../change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")\>

Retrieves the change ticks for the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId").

**You should prefer to use the typed API [`World::get_resource_change_ticks`](../../prelude/struct.World.html#method.get_resource_change_ticks "method bevy::prelude::World::get_resource_change_ticks") where possible.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2239)

#### pub fn [resource](#method.resource)<R>(&self) -> [&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference to the resource of the given type

##### Panics

Panics if the resource does not exist. Use [`get_resource`](../../prelude/struct.World.html#method.get_resource "method bevy::prelude::World::get_resource") instead if you want to handle this case.

If you want to instead insert a value if the resource does not exist, use [`get_resource_or_insert_with`](../../prelude/struct.World.html#method.get_resource_or_insert_with "method bevy::prelude::World::get_resource_or_insert_with").

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/3d/specular\_tint.rs ([line 33](../../../src/specular_tint/specular_tint.rs.html#33))

```rust
32    fn from_world(world: &mut World) -> Self {
33        let asset_server = world.resource::<AssetServer>();
34        Self {
35            noise_texture: asset_server.load("textures/AlphaNoise.png"),
36        }
37    }
```

Hide additional examples

examples/audio/play\_sound\_effect.rs ([line 14](../../../src/play_sound_effect/play_sound_effect.rs.html#14))

```rust
13    fn from_world(world: &mut World) -> Self {
14        let asset_server = world.resource::<AssetServer>();
15        SoundEffect {
16            handle: asset_server.load("sounds/breakout_collision.ogg"),
17        }
18    }
```

examples/scene/world\_serialization.rs ([line 89](../../../src/world_serialization/world_serialization.rs.html#89))

```rust
88    fn from_world(world: &mut World) -> Self {
89        let time = world.resource::<Time>();
90        ComponentB {
91            _time_since_startup: time.elapsed(),
92            value: "Default Value".to_string(),
93        }
94    }
95}
96
97/// A simple resource that also derives `Reflect`, allowing it to be stored in world files.
98///
99/// Just like a component, you can skip serializing fields or implement `FromWorld` if needed.
100#[derive(Resource, Reflect, Default)]
101#[reflect(Resource)]
102struct ResourceA {
103    /// This resource tracks a `score` value.
104    pub score: u32,
105}
106
107/// # World File Paths
108///
109/// `WORLD_FILE_PATH` points to the original world file that we'll be loading.
110/// `NEW_WORLD_FILE_PATH` points to the new world file that we'll be creating
111/// (and demonstrating how to serialize to disk).
112///
113/// The initial world file will be loaded below and not change when the world is saved.
114const WORLD_FILE_PATH: &str = "serialized_worlds/load_scene_example.scn.ron";
115
116/// The new, updated world data will be saved here so that you can see the changes.
117const NEW_WORLD_FILE_PATH: &str = "serialized_worlds/load_scene_example-new.scn.ron";
118
119/// Loads a world from an asset file and spawns it in the current world.
120///
121/// Spawning a `DynamicWorldRoot` creates a new parent entity, which then spawns new
122/// instances of the world's entities as its children. If you modify the
123/// `WORLD_FILE_PATH` file, or if you enable file watching, you can see
124/// changes reflected immediately.
125fn load_world_system(mut commands: Commands, asset_server: Res<AssetServer>) {
126    commands.spawn(DynamicWorldRoot(asset_server.load(WORLD_FILE_PATH)));
127    commands.spawn((
128        Camera3d::default(),
129        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::new(0.0, 0.25, 0.0), Vec3::Y),
130    ));
131    commands.spawn((
132        DirectionalLight::default(),
133        Transform::default().looking_to(Vec3::new(0.0, -1.0, -1.0), Vec3::Y),
134    ));
135}
136
137/// Logs changes made to `ComponentA` entities, and also checks whether `ResourceA`
138/// has been recently added.
139///
140/// Any time a `ComponentA` is modified, that change will appear here. This system
141/// demonstrates how you might detect and handle world updates at runtime.
142fn log_system(
143    query: Query<(Entity, &ComponentA), Changed<ComponentA>>,
144    res: Option<Res<ResourceA>>,
145) {
146    for (entity, component_a) in &query {
147        info!("  Entity({})", entity.index());
148        info!(
149            "    ComponentA: {{ x: {} y: {} }}\n",
150            component_a.x, component_a.y
151        );
152    }
153    if let Some(res) = res
154        && res.is_added()
155    {
156        info!("  New ResourceA: {{ score: {} }}\n", res.score);
157    }
158}
159
160/// Demonstrates how to create a new world from scratch, populate it with data,
161/// and then serialize it to a file. The new file is written to `NEW_WORLD_FILE_PATH`.
162///
163/// This system creates a fresh world, duplicates the type registry so that our
164/// custom component types are recognized, spawns some sample entities and resources,
165/// and then serializes the resulting dynamic world.
166fn save_world_system(world: &mut World) {
167    let asset_server = world.resource::<AssetServer>().clone();
168    // The `TypeRegistry` resource contains information about all registered types (including components).
169    // This is used to construct worlds, so we'll want to ensure that we use the registry from the
170    // main world. To do this, we can simply clone the `AppTypeRegistry` resource.
171    let type_registry = world.resource::<AppTypeRegistry>().clone();
172
173    // Any ECS World can be serialized.
174    // For demonstration purposes, we'll create a new one.
175    let mut scene_world = World::new();
176
177    let mut component_b = ComponentB::from_world(world);
178    component_b.value = "hello".to_string();
179    scene_world.spawn((
180        component_b,
181        ComponentA { x: 1.0, y: 2.0 },
182        Transform::IDENTITY,
183        Name::new("joe"),
184        WorldAssetRoot(asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Scene0")),
185    ));
186    scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
187    scene_world.insert_resource(ResourceA { score: 1 });
188
189    // With our sample world ready to go, we can now create a DynamicWorld from it.
190    // For simplicity, we will create our scene using DynamicWorld directly, but if
191    // you need more control, you can use DynamicWorldBuilder.
192    let dynamic_world = DynamicWorld::from_world_with(&scene_world, &type_registry.read());
193
194    // Dynamic Worlds can be serialized like this:
195    let type_registry = world.resource::<AppTypeRegistry>();
196    let type_registry = type_registry.read();
197    let serialized_world = dynamic_world.serialize(&type_registry).unwrap();
198
199    // Shows the serialized world in the console
200    info!("{}", serialized_world);
201
202    // Writing the world to a new file. Using a task to avoid calling the filesystem APIs in a system
203    // as they are blocking.
204    //
205    // This can't work in Wasm as there is no filesystem access.
206    #[cfg(not(target_arch = "wasm32"))]
207    IoTaskPool::get()
208        .spawn(async move {
209            // Write the world RON data to file
210            File::create(format!("assets/{NEW_WORLD_FILE_PATH}"))
211                .and_then(|mut file| file.write(serialized_world.as_bytes()))
212                .expect("Error while writing world to file");
213        })
214        .detach();
215}
```

examples/app/externally\_driven\_headless\_renderer.rs ([line 111](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#111))

```rust
105    fn update(&mut self) {
106        self.0.update();
107        // Wait for frame to finish rendering by wait polling the device
108        self.0
109            .main
110            .world()
111            .resource::<RenderDevice>()
112            .wgpu_device()
113            .poll(PollType::Wait {
114                submission_index: None,
115                timeout: None,
116            })
117            .unwrap();
118    }
```

examples/2d/dynamic\_mip\_generation.rs ([line 295](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#295))

```rust
291    fn from_world(world: &mut World) -> Self {
292        let mut meshes = world.resource_mut::<Assets<Mesh>>();
293        let rectangle = meshes.add(Rectangle::default());
294
295        let asset_server = world.resource::<AssetServer>();
296        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
297        let text_font = TextFont {
298            font: font.into(),
299            font_size: FONT_SIZE,
300            ..default()
301        };
302
303        AppAssets {
304            rectangle,
305            text_font,
306        }
307    }
```

tests/ecs/ambiguity\_detection.rs ([line 94](../../../src/ambiguity_detection/ambiguity_detection.rs.html#94))

```rust
91fn count_ambiguities(sub_app: &mut SubApp) -> AmbiguitiesCount {
92    let schedule_labels = sub_app
93        .world()
94        .resource::<Schedules>()
95        .iter()
96        .map(|(_, schedule)| schedule.label())
97        .collect::<Vec<_>>();
98    let mut ambiguities = <HashMap<_, _>>::default();
99    for label in schedule_labels {
100        let ambiguities_in_schedule =
101            sub_app
102                .world_mut()
103                .schedule_scope(label, |world, schedule| {
104                    schedule.initialize(world).unwrap().unwrap();
105                    schedule.graph().conflicting_systems().len()
106                });
107        ambiguities.insert(label, ambiguities_in_schedule);
108    }
109    AmbiguitiesCount(ambiguities)
110}
```

Additional examples can be found in:  

*   [examples/ecs/immutable\_components.rs](../../../src/immutable_components/immutable_components.rs.html#112)
*   [examples/ecs/ecs\_guide.rs](../../../src/ecs_guide/ecs_guide.rs.html#248)
*   [examples/3d/clustered\_decal\_maps.rs](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#50)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../../src/custom_phase_item/custom_phase_item.rs.html#323)
*   [examples/ecs/component\_hooks.rs](../../../src/component_hooks/component_hooks.rs.html#104)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2263)

#### pub fn [resource\_ref](#method.resource_ref)<R>(&self) -> [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_, R>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference to the resource of the given type

##### Panics

Panics if the resource does not exist. Use [`get_resource_ref`](../../prelude/struct.World.html#method.get_resource_ref "method bevy::prelude::World::get_resource_ref") instead if you want to handle this case.

If you want to instead insert a value if the resource does not exist, use [`get_resource_or_insert_with`](../../prelude/struct.World.html#method.get_resource_or_insert_with "method bevy::prelude::World::get_resource_or_insert_with").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2302)

#### pub fn [get\_resource](#method.get_resource)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference to the resource of the given type if it exists

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/window/persisting\_window\_settings.rs ([line 51](../../../src/persisting_window_settings/persisting_window_settings.rs.html#51))

```rust
49fn init_window_pos(app: &mut App) {
50    let world = app.world_mut();
51    let Some(window_settings) = world.get_resource::<WindowSettings>() else {
52        return;
53    };
54    let window_settings = window_settings.clone();
55
56    let Ok(mut window) = world.query::<&mut Window>().single_mut(world) else {
57        warn!("window not found");
58        return;
59    };
60
61    if let Some(position) = window_settings.position {
62        window.position = WindowPosition::new(position);
63    }
64
65    if let Some(size) = window_settings.size {
66        window.resolution = WindowResolution::new(size.x, size.y);
67    }
68
69    window.mode = if window_settings.fullscreen {
70        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
71    } else {
72        WindowMode::Windowed
73    };
74}
```

Hide additional examples

examples/ecs/error\_handling.rs ([line 155](../../../src/error_handling/error_handling.rs.html#155))

```rust
151fn failing_system(world: &mut World) -> Result {
152    world
153        // `get_resource` returns an `Option<T>`, so we use `ok_or` to convert it to a `Result` on
154        // which we can call `?` to propagate the error.
155        .get_resource::<UninitializedResource>()
156        // We can provide a `str` here because `BevyError` implements `From<&str>`.
157        .ok_or("Resource not initialized")
158        // The default error severity is Severity::Panic.
159        // We can add a Severity level to any Result locally to downgrade it appropriately.
160        .with_severity(Severity::Warning)?;
161
162    world
163        // This entity doesn't exist!
164        .spawn_empty_at(Entity::from_raw_u32(12345678).unwrap())
165        .map_severity(|e| match e {
166            // Not that concerning, we just need to make sure to find a different entity
167            SpawnError::AlreadySpawned => Severity::Debug,
168            // Oh no
169            SpawnError::Invalid(_) => Severity::Error,
170        })?;
171
172    Ok(())
173}
174
175fn failing_commands(mut commands: Commands) {
176    commands
177        // This entity doesn't exist!
178        .entity(Entity::from_raw_u32(12345678).unwrap())
179        // Normally, this failed command would panic,
180        // but since we've set the global error handler to `warn`
181        // it will log a warning instead.
182        .insert(Transform::default());
183
184    // The error handlers for commands can be set individually as well,
185    // by using the queue_handled method.
186    commands.queue_handled(
187        |world: &mut World| -> Result {
188            world
189                .get_resource::<UninitializedResource>()
190                .ok_or("Resource not initialized when accessed in a command")?;
191
192            Ok(())
193        },
194        |error, context| {
195            error!("{error}, {context}");
196        },
197    );
198}
```

examples/ecs/dynamic.rs ([line 282](../../../src/dynamic/dynamic.rs.html#282))

```rust
69fn main() {
70    let mut world = World::new();
71    let mut lines = std::io::stdin().lines();
72    let mut component_names = HashMap::<String, ComponentId>::new();
73    let mut component_info = HashMap::<ComponentId, ComponentInfo>::new();
74    let mut event_names = HashMap::<String, EventKey>::new();
75
76    println!("{PROMPT}");
77    loop {
78        print!("\n> ");
79        let _ = std::io::stdout().flush();
80        let Some(Ok(line)) = lines.next() else {
81            return;
82        };
83
84        if line.is_empty() {
85            return;
86        };
87
88        let Some((first, rest)) = line.trim().split_once(|c: char| c.is_whitespace()) else {
89            match &line.chars().next() {
90                Some('c') => println!("{COMPONENT_PROMPT}"),
91                Some('s') => println!("{ENTITY_PROMPT}"),
92                Some('q') => println!("{QUERY_PROMPT}"),
93                Some('e') => println!("{EVENT_PROMPT}"),
94                Some('t') => println!("{EMIT_PROMPT}"),
95                _ => println!("{PROMPT}"),
96            }
97            continue;
98        };
99
100        match &first[0..1] {
101            "c" => {
102                rest.split(',').for_each(|component| {
103                    let mut component = component.split_whitespace();
104                    let Some(name) = component.next() else {
105                        return;
106                    };
107                    let size = match component.next().map(str::parse) {
108                        Some(Ok(size)) => size,
109                        _ => 0,
110                    };
111                    // Register our new component to the world with a layout specified by it's size
112                    // SAFETY: [u64] is Send + Sync
113                    let id = world.register_component_with_descriptor(unsafe {
114                        ComponentDescriptor::new_with_layout(
115                            name.to_string(),
116                            StorageType::Table,
117                            Layout::array::<u64>(size).unwrap(),
118                            None,
119                            true,
120                            ComponentCloneBehavior::Default,
121                            None,
122                        )
123                    });
124                    let Some(info) = world.components().get_info(id) else {
125                        return;
126                    };
127                    component_names.insert(name.to_string(), id);
128                    component_info.insert(id, info.clone());
129                    println!("Component {} created with id: {}", name, id.index());
130                });
131            }
132            "s" => {
133                let mut to_insert_ids = Vec::new();
134                let mut to_insert_data = Vec::new();
135                rest.split(',').for_each(|component| {
136                    let mut component = component.split_whitespace();
137                    let Some(name) = component.next() else {
138                        return;
139                    };
140
141                    // Get the id for the component with the given name
142                    let Some(&id) = component_names.get(name) else {
143                        println!("Component {name} does not exist");
144                        return;
145                    };
146
147                    // Calculate the length for the array based on the layout created for this component id
148                    let info = world.components().get_info(id).unwrap();
149                    let len = info.layout().size() / size_of::<u64>();
150                    let mut values: Vec<u64> = component
151                        .take(len)
152                        .filter_map(|value| value.parse::<u64>().ok())
153                        .collect();
154                    values.resize(len, 0);
155
156                    // Collect the id and array to be inserted onto our entity
157                    to_insert_ids.push(id);
158                    to_insert_data.push(values);
159                });
160
161                let mut entity = world.spawn_empty();
162
163                // Construct an `OwningPtr` for each component in `to_insert_data`
164                let to_insert_ptr = to_owning_ptrs(&mut to_insert_data);
165
166                // SAFETY:
167                // - Component ids have been taken from the same world
168                // - Each array is created to the layout specified in the world
169                unsafe {
170                    entity.insert_by_ids(&to_insert_ids, to_insert_ptr.into_iter());
171                }
172
173                println!("Entity spawned with id: {}", entity.id());
174            }
175            "q" => {
176                let mut builder = QueryBuilder::<FilteredEntityMut>::new(&mut world);
177                parse_query(rest, &mut builder, &component_names);
178                let mut query = builder.build();
179                query.iter_mut(&mut world).for_each(|filtered_entity| {
180                    let terms = filtered_entity
181                        .access()
182                        .try_iter_access()
183                        .unwrap()
184                        .map(|component_access| {
185                            let id = *component_access.index();
186                            let ptr = filtered_entity.get_by_id(id).unwrap();
187                            let info = component_info.get(&id).unwrap();
188                            let len = info.layout().size() / size_of::<u64>();
189
190                            // SAFETY:
191                            // - All components are created with layout [u64]
192                            // - len is calculated from the component descriptor
193                            let data = unsafe {
194                                std::slice::from_raw_parts_mut(
195                                    ptr.assert_unique().as_ptr().cast::<u64>(),
196                                    len,
197                                )
198                            };
199
200                            // If we have write access, increment each value once
201                            if matches!(component_access, ComponentAccessKind::Exclusive(_)) {
202                                data.iter_mut().for_each(|data| {
203                                    *data += 1;
204                                });
205                            }
206
207                            format!("{}: {:?}", info.name(), data[0..len].to_vec())
208                        })
209                        .collect::<Vec<_>>()
210                        .join(", ");
211
212                    println!("{}: {}", filtered_entity.id(), terms);
213                });
214            }
215            "e" => {
216                rest.split(',').for_each(|event| {
217                    let name = event.trim();
218                    if name.is_empty() {
219                        return;
220                    }
221
222                    // Register a ComponentId for this event, no Rust type needed.
223                    // SAFETY: ZST with no drop
224                    let event_component_id = world.register_component_with_descriptor(unsafe {
225                        ComponentDescriptor::new_with_layout(
226                            format!("event:{name}"),
227                            StorageType::Table,
228                            Layout::new::<()>(),
229                            None,
230                            false,
231                            ComponentCloneBehavior::Ignore,
232                            None,
233                        )
234                    });
235                    // SAFETY: event_component_id was just registered for this event
236                    let event_key = unsafe { EventKey::new(event_component_id) };
237                    event_names.insert(name.to_string(), event_key);
238
239                    // Build a dynamic observer that prints when the event fires.
240                    let runner: ObserverRunner = |mut world, _observer, ctx, _event, _trigger| {
241                        println!("  Observer fired!");
242                        if let Some(mut counts) = world.get_resource_mut::<EventFireCount>() {
243                            *counts.0.entry(ctx.event_key).or_insert(0) += 1;
244                        }
245                    };
246
247                    // SAFETY: event_key was just registered, runner ignores pointers
248                    let observer =
249                        unsafe { Observer::with_dynamic_runner(runner).with_event_key(event_key) };
250                    world.spawn(observer);
251
252                    println!(
253                        "Event '{name}' registered (key: {}) with a dynamic observer",
254                        event_component_id.index()
255                    );
256                });
257
258                // Ensure the counter resource exists.
259                world.init_resource::<EventFireCount>();
260            }
261            "t" => {
262                let name = rest.trim();
263                let Some(&event_key) = event_names.get(name) else {
264                    println!(
265                        "Event '{name}' does not exist. Register it first with 'event {name}'"
266                    );
267                    continue;
268                };
269
270                let mut event_data = ();
271                let mut trigger_data = ();
272                // SAFETY: event_key was registered in this world, both pointers are valid ZSTs
273                unsafe {
274                    world.trigger_dynamic(
275                        event_key,
276                        PtrMut::from(&mut event_data),
277                        PtrMut::from(&mut trigger_data),
278                    );
279                }
280
281                let count = world
282                    .get_resource::<EventFireCount>()
283                    .map_or(0, |c| c.0.get(&event_key).copied().unwrap_or(0));
284                println!("Event '{name}' triggered ({count} fires)");
285            }
286            _ => continue,
287        }
288    }
289}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2311)

#### pub fn [get\_resource\_ref](#method.get_resource_ref)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_, R>>

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference including change detection to the resource of the given type if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2406)

#### pub fn [non\_send\_resource](#method.non_send_resource)<R>(&self) -> [&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where R: 'static,

👎Deprecated since 0.19.0:

use World::non\_send

Gets an immutable reference to a non-send resource of the given type, if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2420)

#### pub fn [non\_send](#method.non_send)<R>(&self) -> [&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where R: 'static,

Gets an immutable reference to the non-send data of the given type, if it exists.

##### Panics

Panics if the data does not exist. Use [`get_non_send`](../../prelude/struct.World.html#method.get_non_send "method bevy::prelude::World::get_non_send") instead if you want to handle this case.

This function will panic if it isn’t called from the same thread that the resource was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2463)

#### pub fn [get\_non\_send\_resource](#method.get_non_send_resource)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where R: 'static,

👎Deprecated since 0.19.0:

use World::get\_non\_send

Gets a reference to a non-send resource of the given type, if it exists. Otherwise returns `None`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2473)

#### pub fn [get\_non\_send](#method.get_non_send)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where R: 'static,

Gets a reference to the non-send data of the given type, if it exists. Otherwise returns `None`.

##### Panics

This function will panic if it isn’t called from the same thread that the resource was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3174)

#### pub fn [read\_change\_tick](#method.read_change_tick)(&self) -> [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Reads the current change tick of this world.

If you have exclusive (`&mut`) access to the world, consider using [`change_tick()`](../../prelude/struct.World.html#method.change_tick "method bevy::prelude::World::change_tick"), which is more efficient since it does not require atomic synchronization.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3196)

#### pub fn [last\_change\_tick](#method.last_change_tick)(&self) -> [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

When called from within an exclusive system (a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") that takes `&mut World` as its first parameter), this method returns the [`Tick`](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick") indicating the last time the exclusive system was run.

Otherwise, this returns the `Tick` indicating the last time that [`World::clear_trackers`](../../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3461)

#### pub fn [fallback\_error\_handler](#method.fallback_error_handler)(&self) -> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError"), [ErrorContext](../error/enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext"))

Convenience method for accessing the world’s fallback error handler, which can be overwritten with [`FallbackErrorHandler`](../error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3477)

#### pub fn [get\_resource\_by\_id](#method.get_resource_by_id)(&self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>>

Gets a pointer to the resource with the id [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") if it exists. The returned pointer must not be used to modify the resource, and must not be dereferenced after the immutable borrow of the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") ends.

**You should prefer to use the typed API [`World::get_resource`](../../prelude/struct.World.html#method.get_resource "method bevy::prelude::World::get_resource") where possible and only use this in cases where the actual types are not known at compile time.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3591)

#### pub fn [iter\_resources](#method.iter_resources)(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = (&[ComponentInfo](../component/struct.ComponentInfo.html "struct bevy::ecs::component::ComponentInfo"), [Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>)>

Iterates over all resources in the world.

The returned iterator provides lifetimed, but type-unsafe pointers. Actually reading the contents of each resource will require the use of unsafe code.

##### Examples

###### Printing the size of all resources

```rust
let mut total = 0;
for (info, _) in world.iter_resources() {
   println!("Resource: {}", info.name());
   println!("Size: {} bytes", info.layout().size());
   total += info.layout().size();
}
println!("Total size: {} bytes", total);
```

###### Dynamically running closures for resources matching specific `TypeId`s

```rust
// In this example, `A` and `B` are resources. We deliberately do not use the
// `bevy_reflect` crate here to showcase the low-level [`Ptr`] usage. You should
// probably use something like `ReflectFromPtr` in a real-world scenario.

// Create the hash map that will store the closures for each resource type
let mut closures: HashMap<TypeId, Box<dyn Fn(&Ptr<'_>)>> = HashMap::default();

// Add closure for `A`
closures.insert(TypeId::of::<A>(), Box::new(|ptr| {
    // SAFETY: We assert ptr is the same type of A with TypeId of A
    let a = unsafe { &ptr.deref::<A>() };
    // ... do something with `a` here
}));

// Add closure for `B`
closures.insert(TypeId::of::<B>(), Box::new(|ptr| {
    // SAFETY: We assert ptr is the same type of B with TypeId of B
    let b = unsafe { &ptr.deref::<B>() };
    // ... do something with `b` here
}));

// Iterate all resources, in order to run the closures for each matching resource type
for (info, ptr) in world.iter_resources() {
    let Some(type_id) = info.type_id() else {
       // It's possible for resources to not have a `TypeId` (e.g. non-Rust resources
       // dynamically inserted via a scripting language) in which case we can't match them.
       continue;
    };

    let Some(closure) = closures.get(&type_id) else {
       // No closure for this resource type, skip it.
       continue;
    };

    // Run the closure for the resource
    closure(&ptr);
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3704)

#### pub fn [get\_non\_send\_by\_id](#method.get_non_send_by_id)(&self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>>

Gets a pointer to `!Send` data with the id [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") if it exists. The returned pointer must not be used to modify the resource, and must not be dereferenced after the immutable borrow of the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") ends.

**You should prefer to use the typed API [`World::get_non_send`](../../prelude/struct.World.html#method.get_non_send "method bevy::prelude::World::get_non_send") where possible and only use this in cases where the actual types are not known at compile time.**

##### Panics

This function will panic if it isn’t called from the same thread that the data was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3776)

#### pub fn [get\_by\_id](#method.get_by_id)( &self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>>

Retrieves an immutable untyped reference to the given `entity`’s [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") of the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"). Returns `None` if the `entity` does not have a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type.

**You should prefer to use the typed API [`World::get_mut`](../../prelude/struct.World.html#method.get_mut "method bevy::prelude::World::get_mut") where possible and only use this in cases where the actual types are not known at compile time.**

##### Panics

This function will panic if it isn’t called from the same thread that the resource was inserted from.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#33)

### impl<'w> [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#34)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [World](../../prelude/struct.World.html "struct bevy::prelude::World")

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#36)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#54)

### impl<'w> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")\> for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/deferred_world.rs.html#55)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(world: &'w mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#825)

### impl<'w> [SystemParam](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#826)

#### type [State](../system/trait.SystemParam.html#associatedtype.State) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

Used to store data which persists across invocations of a system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#827)

#### type [Item](../system/trait.SystemParam.html#associatedtype.Item)<'world, 'state> = [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'world>

The item type returned when constructing this system param. The value of this associated type should be `Self`, instantiated with new lifetimes. [Read more](../system/trait.SystemParam.html#associatedtype.Item)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#829)

#### fn [init\_state](../system/trait.SystemParam.html#tymethod.init_state)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w> as [SystemParam](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

Creates a new instance of this param’s [`State`](../system/trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#831-836)

#### fn [init\_access](../system/trait.SystemParam.html#tymethod.init_access)( \_state: &<[DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w> as [SystemParam](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &mut [SystemMeta](../system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), \_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

Registers any [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") access used by this [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"). [Read more](../system/trait.SystemParam.html#tymethod.init_access)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#845-850)

#### unsafe fn [get\_param](../system/trait.SystemParam.html#tymethod.get_param)<'world, 'state>( \_state: &'state mut <[DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w> as [SystemParam](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](../system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), \_system\_meta: &[SystemMeta](../system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'world>, \_change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w> as [SystemParam](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'world, 'state>, [SystemParamValidationError](../system/struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

Creates a parameter to be passed into a [`SystemParamFunction`](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction"). [Read more](../system/trait.SystemParam.html#tymethod.get_param)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#250)

#### fn [apply](../system/trait.SystemParam.html#method.apply)(state: &mut Self::[State](../system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](../system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"))

Applies any deferred mutations stored in this [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")’s state. This is used to apply [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") during [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#258)

#### fn [queue](../system/trait.SystemParam.html#method.queue)( state: &mut Self::[State](../system/trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](../system/struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

Queues any deferred mutations to be applied at the next [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

## Auto Trait Implementations

### impl<'w> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

### impl<'w> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

### impl<'w> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

### impl<'w> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

### impl<'w> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

### impl<'w> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

### impl<'w> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [DeferredWorld](struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}