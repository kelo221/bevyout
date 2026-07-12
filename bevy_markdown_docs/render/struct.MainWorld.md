[bevy](../index.html)::[render](index.html)

# Struct MainWorld 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#106)

```rust
pub struct MainWorld(/* private fields */);
```

The simulation [`World`](../prelude/struct.World.html "struct bevy::prelude::World") of the application, stored as a resource.

This resource is only available during [`ExtractSchedule`](../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule") and not during command application of that schedule. See [`Extract`](struct.Extract.html "struct bevy::render::Extract") for more details.

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [World](../prelude/struct.World.html "struct bevy::prelude::World")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/mod.rs.html#355)

#### pub fn [register\_event\_key](#method.register_event_key)<E>(&mut self) -> [EventKey](../ecs/event/struct.EventKey.html "struct bevy::ecs::event::EventKey")

where E: [Event](../prelude/trait.Event.html "trait bevy::prelude::Event"),

Generates the [`EventKey`](../ecs/event/struct.EventKey.html "struct bevy::ecs::event::EventKey") for this event type.

If this type has already been registered, this will return the existing [`EventKey`](../ecs/event/struct.EventKey.html "struct bevy::ecs::event::EventKey").

This is used by various dynamically typed observer APIs, such as [`DeferredWorld::trigger_raw`](../ecs/world/struct.DeferredWorld.html#method.trigger_raw "method bevy::ecs::world::DeferredWorld::trigger_raw").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/mod.rs.html#364)

#### pub fn [event\_key](#method.event_key)<E>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[EventKey](../ecs/event/struct.EventKey.html "struct bevy::ecs::event::EventKey")\>

where E: [Event](../prelude/trait.Event.html "trait bevy::prelude::Event"),

Fetches the [`EventKey`](../ecs/event/struct.EventKey.html "struct bevy::ecs::event::EventKey") for this event type, if it has already been generated.

This is used by various dynamically typed observer APIs, such as [`DeferredWorld::trigger_raw`](../ecs/world/struct.DeferredWorld.html#method.trigger_raw "method bevy::ecs::world::DeferredWorld::trigger_raw").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/mod.rs.html#55)

#### pub fn [add\_observer](#method.add_observer)<M>( &mut self, observer: impl [IntoObserver](../ecs/observer/trait.IntoObserver.html "trait bevy::ecs::observer::IntoObserver")<M>, ) -> [EntityWorldMut](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>

Spawns a “global” [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer") which will watch for the given event. Returns its [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") as a [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

`system` can be any system whose first parameter is [`On`](../prelude/struct.On.html "struct bevy::prelude::On").

##### Example

```rust
#[derive(Component)]
struct A;

world.add_observer(|_: On<Add, A>| {
    // ...
});
world.add_observer(|_: On<Remove, A>| {
    // ...
});
```

**Calling [`observe`](../prelude/struct.EntityWorldMut.html#method.observe "method bevy::prelude::EntityWorldMut::observe") on the returned [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") will observe the observer itself, which you very likely do not want.**

##### Panics

Panics if the given system is an exclusive system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/mod.rs.html#63)

#### pub fn [trigger](#method.trigger)<'a, E>(&mut self, event: E)

where E: [Event](../prelude/trait.Event.html "trait bevy::prelude::Event"), <E as [Event](../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'a>: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Triggers the given [`Event`](../prelude/trait.Event.html "trait bevy::prelude::Event"), which will run any [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.

For a variant that borrows the `event` rather than consuming it, use [`World::trigger_ref`](../prelude/struct.World.html#method.trigger_ref "method bevy::prelude::World::trigger_ref") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/mod.rs.html#75)

#### pub fn [trigger\_with](#method.trigger_with)<'a, E>( &mut self, event: E, trigger: <E as [Event](../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'a>, )

where E: [Event](../prelude/trait.Event.html "trait bevy::prelude::Event"),

Triggers the given [`Event`](../prelude/trait.Event.html "trait bevy::prelude::Event") using the given [`Trigger`](../ecs/event/trait.Trigger.html "trait bevy::ecs::event::Trigger"), which will run any [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.

For a variant that borrows the `event` rather than consuming it, use [`World::trigger_ref`](../prelude/struct.World.html#method.trigger_ref "method bevy::prelude::World::trigger_ref") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/mod.rs.html#84)

#### pub fn [trigger\_ref](#method.trigger_ref)<'a, E>(&mut self, event: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [Event](../prelude/trait.Event.html "trait bevy::prelude::Event"), <E as [Event](../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'a>: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Triggers the given mutable [`Event`](../prelude/trait.Event.html "trait bevy::prelude::Event") reference, which will run any [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.

Compared to [`World::trigger`](../prelude/struct.World.html#method.trigger "method bevy::prelude::World::trigger"), this method is most useful when it’s necessary to check or use the event after it has been modified by observers.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/mod.rs.html#97)

#### pub fn [trigger\_ref\_with](#method.trigger_ref_with)<'a, E>( &mut self, event: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html), trigger: &mut <E as [Event](../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'a>, )

where E: [Event](../prelude/trait.Event.html "trait bevy::prelude::Event"),

Triggers the given mutable [`Event`](../prelude/trait.Event.html "trait bevy::prelude::Event") reference using the given mutable [`Trigger`](../ecs/event/trait.Trigger.html "trait bevy::ecs::event::Trigger") reference, which will run any [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.

Compared to [`World::trigger`](../prelude/struct.World.html#method.trigger "method bevy::prelude::World::trigger"), this method is most useful when it’s necessary to check or use the event after it has been modified by observers.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/mod.rs.html#149-154)

#### pub unsafe fn [trigger\_dynamic](#method.trigger_dynamic)( &mut self, event\_key: [EventKey](../ecs/event/struct.EventKey.html "struct bevy::ecs::event::EventKey"), event\_data: [PtrMut](../ecs/ptr/struct.PtrMut.html "struct bevy::ecs::ptr::PtrMut")<'\_>, trigger\_data: [PtrMut](../ecs/ptr/struct.PtrMut.html "struct bevy::ecs::ptr::PtrMut")<'\_>, )

Triggers global [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer")s for `event_key` with untyped event and trigger data.

Dynamic equivalent of [`World::trigger`](../prelude/struct.World.html#method.trigger "method bevy::prelude::World::trigger"). Only fires global observers, not entity- or component-scoped ones.

Use [`World::trigger_dynamic_targets`](../prelude/struct.World.html#method.trigger_dynamic_targets "method bevy::prelude::World::trigger_dynamic_targets") to also fire entity-scoped observers.

##### Safety

*   `event_data` must point to a valid, aligned value whose layout matches what observers registered for this `event_key` expect.
*   `trigger_data` must point to a valid, aligned value whose layout matches what observers registered for this `event_key` expect.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/ecs/dynamic.rs ([lines 274-278](../../src/dynamic/dynamic.rs.html#274-278))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/mod.rs.html#200-206)

#### pub unsafe fn [trigger\_dynamic\_targets](#method.trigger_dynamic_targets)( &mut self, event\_key: [EventKey](../ecs/event/struct.EventKey.html "struct bevy::ecs::event::EventKey"), entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), event\_data: [PtrMut](../ecs/ptr/struct.PtrMut.html "struct bevy::ecs::ptr::PtrMut")<'\_>, trigger\_data: [PtrMut](../ecs/ptr/struct.PtrMut.html "struct bevy::ecs::ptr::PtrMut")<'\_>, )

Triggers [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer")s for `event_key` targeting `entity`, with untyped event and trigger data.

Fires global and entity-scoped observers. Dynamic equivalent of [`EntityWorldMut::trigger`](../prelude/struct.EntityWorldMut.html#method.trigger "method bevy::prelude::EntityWorldMut::trigger").

##### Safety

*   `event_data` must point to a valid, aligned value whose layout matches what observers registered for this `event_key` expect.
*   `trigger_data` must point to a valid, aligned value whose layout matches what observers registered for this `event_key` expect.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/mod.rs.html#249-256)

#### pub unsafe fn [trigger\_dynamic\_targets\_components](#method.trigger_dynamic_targets_components)( &mut self, event\_key: [EventKey](../ecs/event/struct.EventKey.html "struct bevy::ecs::event::EventKey"), entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), components: &\[[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\], event\_data: [PtrMut](../ecs/ptr/struct.PtrMut.html "struct bevy::ecs::ptr::PtrMut")<'\_>, trigger\_data: [PtrMut](../ecs/ptr/struct.PtrMut.html "struct bevy::ecs::ptr::PtrMut")<'\_>, )

Triggers [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer")s for `event_key` targeting `entity` and `components`, with untyped event and trigger data.

Fires global, entity-scoped, and component-scoped observers. Dynamic equivalent of [`EntityComponentsTrigger`](../ecs/event/struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger").

##### Safety

*   `event_data` must point to a valid, aligned value whose layout matches what observers registered for this `event_key` expect.
*   `trigger_data` must point to a valid, aligned value whose layout matches what observers registered for this `event_key` expect.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#455-461)

#### pub fn [register\_system](#method.register_system)<I, O, M>( &mut self, system: impl [IntoSystem](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static, ) -> [SystemId](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId")<I, O>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

Registers a system and returns a [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") so it can later be called by [`World::run_system`](../prelude/struct.World.html#method.run_system "method bevy::prelude::World::run_system").

It’s possible to register multiple copies of the same system by calling this function multiple times. If that’s not what you want, consider using [`World::register_system_cached`](../prelude/struct.World.html#method.register_system_cached "method bevy::prelude::World::register_system_cached") instead.

This is different from adding systems to a [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"), because the [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") that is returned can be used anywhere in the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") to run the associated system. This allows for running systems in a pushed-based fashion. Using a [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") is still preferred for most cases due to its better performance and ability to run non-conflicting systems simultaneously.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/ecs/one\_shot\_systems.rs ([line 51](../../src/one_shot_systems/one_shot_systems.rs.html#51))

```rust
47fn setup_with_world(world: &mut World) {
48    // We can run it once manually
49    world.run_system_once(system_b).unwrap();
50    // Or with a Callback
51    let system_id = world.register_system(system_b);
52    world.spawn((Callback(system_id), B));
53}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#470-473)

#### pub fn [register\_boxed\_system](#method.register_boxed_system)<I, O>( &mut self, system: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../prelude/trait.System.html "trait bevy::prelude::System")<Out = O, In = I>>, ) -> [SystemId](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId")<I, O>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

Similar to [`Self::register_system`](../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system"), but allows passing in a [`BoxedSystem`](../ecs/system/type.BoxedSystem.html "type bevy::ecs::system::BoxedSystem").

This is useful if the [`IntoSystem`](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem") implementor has already been turned into a [`System`](../prelude/trait.System.html "trait bevy::prelude::System") trait object and put in a [`Box`](../prelude/struct.Box.html "struct bevy::prelude::Box").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#491-497)

#### pub fn [register\_tracked\_system](#method.register_tracked_system)<I, O, M>( &mut self, system: impl [IntoSystem](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static, ) -> [SystemHandle](../ecs/system/enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle")<I, O>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

Registers a system and returns a tracked [`SystemHandle`](../ecs/system/enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle") so it can later be called by [`World::run_system`](../prelude/struct.World.html#method.run_system "method bevy::prelude::World::run_system"). The system entity will be automatically queued for despawn when the last clone of the returned handle is dropped.

By default, unused tracked system entities are despawned by the [`despawn_unused_registered_systems`](../ecs/system/fn.despawn_unused_registered_systems.html "fn bevy::ecs::system::despawn_unused_registered_systems") system in the `Last` schedule of the default app. Otherwise, it needs to be run manually to ensure proper cleanup of registered systems.

It’s possible to register multiple copies of the same system by calling this function multiple times. If that’s not what you want, consider using [`World::register_system_cached`](../prelude/struct.World.html#method.register_system_cached "method bevy::prelude::World::register_system_cached") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#507-513)

#### pub fn [register\_tracked\_boxed\_system](#method.register_tracked_boxed_system)<I, O>( &mut self, system: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../prelude/trait.System.html "trait bevy::prelude::System")<Out = O, In = I>>, ) -> [SystemHandle](../ecs/system/enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle")<I, O>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

Similar to [`Self::register_tracked_system`](../prelude/struct.World.html#method.register_tracked_system "method bevy::prelude::World::register_tracked_system"), but allows passing in a [`BoxedSystem`](../ecs/system/type.BoxedSystem.html "type bevy::ecs::system::BoxedSystem").

This is useful if the [`IntoSystem`](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem") implementor has already been turned into a [`System`](../prelude/trait.System.html "trait bevy::prelude::System") trait object and put in a [`Box`](../prelude/struct.Box.html "struct bevy::prelude::Box").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#530-536)

#### pub fn [unregister\_system](#method.unregister_system)<I, O>( &mut self, id: [SystemId](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId")<I, O>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[RemovedSystem](../ecs/system/struct.RemovedSystem.html "struct bevy::ecs::system::RemovedSystem")<I, O>, [RegisteredSystemError](../ecs/system/enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError")<I, O>>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

Removes a registered system and returns the system, if it exists. After removing a system, the [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") becomes invalid and attempting to use it afterwards will result in errors. Re-adding the removed system will register it on a new [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId").

If no system corresponds to the given [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId"), this method returns an error. Systems are also not allowed to remove themselves, this returns an error too.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#640-643)

#### pub fn [run\_system](#method.run_system)<O>( &mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[SystemId](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), O>>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<O, [RegisteredSystemError](../ecs/system/enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), O>>

where O: 'static,

Run stored systems by their [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId"). Before running a system, it must first be registered. The method [`World::register_system`](../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system") stores a given system and returns a [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId"). This is different from [`RunSystemOnce::run_system_once`](../ecs/system/trait.RunSystemOnce.html#method.run_system_once "method bevy::ecs::system::RunSystemOnce::run_system_once"), because it keeps local state between calls and change detection works correctly.

Also runs any queued-up commands.

In order to run a chained system with an input, use [`World::run_system_with`](../prelude/struct.World.html#method.run_system_with "method bevy::prelude::World::run_system_with") instead.

##### Examples

###### Running a system

```rust
fn increment(mut counter: Local<u8>) {
   *counter += 1;
   println!("{}", *counter);
}

let mut world = World::default();
let counter_one = world.register_system(increment);
let counter_two = world.register_system(increment);
world.run_system(counter_one); // -> 1
world.run_system(counter_one); // -> 2
world.run_system(counter_two); // -> 1
```

###### Change detection

```rust
#[derive(Resource, Default)]
struct ChangeDetector;

let mut world = World::default();
world.init_resource::<ChangeDetector>();
let detector = world.register_system(|change_detector: ResMut<ChangeDetector>| {
    if change_detector.is_changed() {
        println!("Something happened!");
    } else {
        println!("Nothing happened.");
    }
});

// Resources are changed when they are first added
let _ = world.run_system(detector); // -> Something happened!
let _ = world.run_system(detector); // -> Nothing happened.
world.resource_mut::<ChangeDetector>().set_changed();
let _ = world.run_system(detector); // -> Something happened!
```

###### Getting system output

```rust
#[derive(Resource)]
struct PlayerScore(i32);

#[derive(Resource)]
struct OpponentScore(i32);

fn get_player_score(player_score: Res<PlayerScore>) -> i32 {
  player_score.0
}

fn get_opponent_score(opponent_score: Res<OpponentScore>) -> i32 {
  opponent_score.0
}

let mut world = World::default();
world.insert_resource(PlayerScore(3));
world.insert_resource(OpponentScore(2));

let scoring_systems = [
  ("player", world.register_system(get_player_score)),
  ("opponent", world.register_system(get_opponent_score)),
];

for (label, scoring_system) in scoring_systems {
  println!("{label} has score {}", world.run_system(scoring_system).expect("system succeeded"));
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#672-679)

#### pub fn [run\_system\_with](#method.run_system_with)<I, O>( &mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[SystemId](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId")<I, O>>, input: <I as [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<O, [RegisteredSystemError](../ecs/system/enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError")<I, O>>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

Run a stored chained system by its [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId"), providing an input value. Before running a system, it must first be registered. The method [`World::register_system`](../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system") stores a given system and returns a [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId").

To use the supplied input, the system should have a [`SystemInput`](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") as the first parameter. Also runs any queued-up commands.

##### Examples

```rust
fn increment(In(increment_by): In<u8>, mut counter: Local<u8>) -> u8 {
  *counter += increment_by;
  *counter
}

let mut world = World::default();
let counter_one = world.register_system(increment);
let counter_two = world.register_system(increment);
assert_eq!(world.run_system_with(counter_one, 1).unwrap(), 1);
assert_eq!(world.run_system_with(counter_one, 20).unwrap(), 21);
assert_eq!(world.run_system_with(counter_two, 30).unwrap(), 30);
```

See [`World::run_system`](../prelude/struct.World.html#method.run_system "method bevy::prelude::World::run_system") for more examples.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#759-763)

#### pub fn [register\_system\_cached](#method.register_system_cached)<I, O, M, S>( &mut self, system: S, ) -> [SystemId](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId")<I, O>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static, S: [IntoSystem](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static,

Registers a system or returns its cached [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId").

If you want to run the system immediately and you don’t need its `SystemId`, see [`World::run_system_cached`](../prelude/struct.World.html#method.run_system_cached "method bevy::prelude::World::run_system_cached").

The first time this function is called for a particular system, it will register it and store its [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") in a [`CachedSystemId`](../ecs/system/struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId") resource for later. If you would rather manage the `SystemId` yourself, or register multiple copies of the same system, use [`World::register_system`](../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system") instead.

##### Limitations

This function only accepts ZST (zero-sized) systems to guarantee that any two systems of the same type must be equal. This means that closures that capture the environment, and function pointers, are not accepted.

If you want to access values from the environment within a system, consider passing them in as inputs via [`World::run_system_cached_with`](../prelude/struct.World.html#method.run_system_cached_with "method bevy::prelude::World::run_system_cached_with"). If that’s not an option, consider [`World::register_system`](../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#795-802)

#### pub fn [unregister\_system\_cached](#method.unregister_system_cached)<I, O, M, S>( &mut self, \_system: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[RemovedSystem](../ecs/system/struct.RemovedSystem.html "struct bevy::ecs::system::RemovedSystem")<I, O>, [RegisteredSystemError](../ecs/system/enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError")<I, O>>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static, S: [IntoSystem](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static,

Removes a cached system and its [`CachedSystemId`](../ecs/system/struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId") resource.

See [`World::register_system_cached`](../prelude/struct.World.html#method.register_system_cached "method bevy::prelude::World::register_system_cached") for more information.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#813-816)

#### pub fn [run\_system\_cached](#method.run_system_cached)<O, M, S>( &mut self, system: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<O, [RegisteredSystemError](../ecs/system/enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), O>>

where O: 'static, S: [IntoSystem](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), O, M> + 'static,

Runs a cached system, registering it if necessary.

See [`World::register_system_cached`](../prelude/struct.World.html#method.register_system_cached "method bevy::prelude::World::register_system_cached") for more information.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#824-832)

#### pub fn [run\_system\_cached\_with](#method.run_system_cached_with)<I, O, M, S>( &mut self, system: S, input: <I as [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<O, [RegisteredSystemError](../ecs/system/enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError")<I, O>>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static, S: [IntoSystem](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static,

Runs a cached system with an input, registering it if necessary.

To use the supplied input, the system should have a [`SystemInput`](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") as the first parameter. See [`World::register_system_cached`](../prelude/struct.World.html#method.register_system_cached "method bevy::prelude::World::register_system_cached") for more information.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/reflect.rs.html#69-73)

#### pub fn [get\_reflect](#method.get_reflect)( &self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static), [GetComponentReflectError](../ecs/world/reflect/enum.GetComponentReflectError.html "enum bevy::ecs::world::reflect::GetComponentReflectError")\>

Retrieves a reference to the given `entity`’s [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given `type_id` using reflection.

Requires implementing [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for the [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") (e.g., using [`#[derive(Reflect)`](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect")) and `app.register_type::<TheComponent>()` to have been called[1](#fn1).

If you want to call this with a [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), see [`World::components`](../prelude/struct.World.html#method.components "method bevy::prelude::World::components") and [`Components::get_id`](../ecs/component/struct.Components.html#method.get_id "method bevy::ecs::component::Components::get_id") to get the corresponding [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId").

Also see the crate documentation for [`bevy_reflect`](../reflect/index.html "mod bevy::reflect") for more information on [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and bevy’s reflection capabilities.

##### Errors

See [`GetComponentReflectError`](../ecs/world/reflect/enum.GetComponentReflectError.html "enum bevy::ecs::world::reflect::GetComponentReflectError") for the possible errors and their descriptions.

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

1.  More specifically: Requires [`TypeData`](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for [`ReflectFromPtr`](../reflect/struct.ReflectFromPtr.html "struct bevy::reflect::ReflectFromPtr") to be registered for the given `type_id`, which is automatically handled when deriving [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and calling [`App::register_type`](../../bevy_app/struct.App.html#method.register_type). [↩](#fnref1)
    

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/reflect.rs.html#141-145)

#### pub fn [get\_reflect\_mut](#method.get_reflect_mut)( &mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [GetComponentReflectError](../ecs/world/reflect/enum.GetComponentReflectError.html "enum bevy::ecs::world::reflect::GetComponentReflectError")\>

Retrieves a mutable reference to the given `entity`’s [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given `type_id` using reflection.

Requires implementing [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for the [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") (e.g., using [`#[derive(Reflect)`](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect")) and `app.register_type::<TheComponent>()` to have been called.

This is the mutable version of [`World::get_reflect`](../prelude/struct.World.html#method.get_reflect "method bevy::prelude::World::get_reflect"), see its docs for more information and an example.

Just calling this method does not trigger [change detection](../ecs/change_detection/index.html "mod bevy::ecs::change_detection").

##### Errors

See [`GetComponentReflectError`](../ecs/world/reflect/enum.GetComponentReflectError.html "enum bevy::ecs::world::reflect::GetComponentReflectError") for the possible errors and their descriptions.

##### Example

See the documentation for [`World::get_reflect`](../prelude/struct.World.html#method.get_reflect "method bevy::prelude::World::get_reflect").

##### Note

Requires the feature `bevy_reflect` (included in the default features).

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/reflect.rs.html#197-201)

#### pub fn [insert\_reflect\_resource](#method.insert_reflect_resource)( &mut self, resource\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), reflected\_resource: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, )

Inserts a reflected resource into the world. If the resource already exists, it is overwritten.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#198)

#### pub fn [id](#method.id)(&self) -> [WorldId](../ecs/world/struct.WorldId.html "struct bevy::ecs::world::WorldId")

Retrieves this [`World`](../prelude/struct.World.html "struct bevy::prelude::World")’s unique ID

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#204)

#### pub fn [as\_unsafe\_world\_cell](#method.as_unsafe_world_cell)(&mut self) -> [UnsafeWorldCell](../ecs/world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>

Creates a new [`UnsafeWorldCell`](../ecs/world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") view with complete read+write access.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#210)

#### pub fn [as\_unsafe\_world\_cell\_readonly](#method.as_unsafe_world_cell_readonly)(&self) -> [UnsafeWorldCell](../ecs/world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>

Creates a new [`UnsafeWorldCell`](../ecs/world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") view with only read access to everything.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#216)

#### pub fn [entities](#method.entities)(&self) -> &[Entities](../ecs/entity/struct.Entities.html "struct bevy::ecs::entity::Entities")

Retrieves this world’s [`Entities`](../ecs/entity/struct.Entities.html "struct bevy::ecs::entity::Entities") collection.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/ecs/callbacks.rs ([line 37](../../src/callbacks/callbacks.rs.html#37))

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

#### pub fn [entity\_allocator](#method.entity_allocator)(&self) -> &[EntityAllocator](../ecs/entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator")

Retrieves this world’s [`EntityAllocator`](../ecs/entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator") collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#228)

#### pub fn [entity\_allocator\_mut](#method.entity_allocator_mut)(&mut self) -> &mut [EntityAllocator](../ecs/entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator")

Retrieves this world’s [`EntityAllocator`](../ecs/entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator") collection mutably.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#238)

#### pub unsafe fn [entities\_mut](#method.entities_mut)(&mut self) -> &mut [Entities](../ecs/entity/struct.Entities.html "struct bevy::ecs::entity::Entities")

Retrieves this world’s [`Entities`](../ecs/entity/struct.Entities.html "struct bevy::ecs::entity::Entities") collection mutably.

##### Safety

Mutable reference must not be used to put the [`Entities`](../ecs/entity/struct.Entities.html "struct bevy::ecs::entity::Entities") data in an invalid state for this [`World`](../prelude/struct.World.html "struct bevy::prelude::World")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#246)

#### pub fn [entity\_count](#method.entity_count)(&self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Retrieves the number of [`Entities`](../ecs/entity/struct.Entities.html "struct bevy::ecs::entity::Entities") in the world.

This is helpful as a diagnostic, but it can also be used effectively in tests.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#252)

#### pub fn [archetypes](#method.archetypes)(&self) -> &[Archetypes](../ecs/archetype/struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes")

Retrieves this world’s [`Archetypes`](../ecs/archetype/struct.Archetypes.html "struct bevy::ecs::archetype::Archetypes") collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#258)

#### pub fn [components](#method.components)(&self) -> &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components")

Retrieves this world’s [`Components`](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components") collection.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/ecs/dynamic.rs ([line 124](../../src/dynamic/dynamic.rs.html#124))

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

#### pub fn [resource\_entities](#method.resource_entities)(&self) -> &[ResourceEntities](../ecs/resource/struct.ResourceEntities.html "struct bevy::ecs::resource::ResourceEntities")

Retrieves this world’s [`ResourceEntities`](../ecs/resource/struct.ResourceEntities.html "struct bevy::ecs::resource::ResourceEntities").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#272)

#### pub fn [components\_queue](#method.components_queue)(&self) -> [ComponentsQueuedRegistrator](../ecs/component/struct.ComponentsQueuedRegistrator.html "struct bevy::ecs::component::ComponentsQueuedRegistrator")<'\_>

Prepares a [`ComponentsQueuedRegistrator`](../ecs/component/struct.ComponentsQueuedRegistrator.html "struct bevy::ecs::component::ComponentsQueuedRegistrator") for the world. **NOTE:** [`ComponentsQueuedRegistrator`](../ecs/component/struct.ComponentsQueuedRegistrator.html "struct bevy::ecs::component::ComponentsQueuedRegistrator") is easily misused. See its docs for important notes on when and how it should be used.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#279)

#### pub fn [components\_registrator](#method.components_registrator)(&mut self) -> [ComponentsRegistrator](../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>

Prepares a [`ComponentsRegistrator`](../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator") for the world.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#286)

#### pub fn [storages](#method.storages)(&self) -> &[Storages](../ecs/storage/struct.Storages.html "struct bevy::ecs::storage::Storages")

Retrieves this world’s [`Storages`](../ecs/storage/struct.Storages.html "struct bevy::ecs::storage::Storages") collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#292)

#### pub fn [bundles](#method.bundles)(&self) -> &[Bundles](../ecs/bundle/struct.Bundles.html "struct bevy::ecs::bundle::Bundles")

Retrieves this world’s [`Bundles`](../ecs/bundle/struct.Bundles.html "struct bevy::ecs::bundle::Bundles") collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#298)

#### pub fn [removed\_components](#method.removed_components)(&self) -> &[RemovedComponentMessages](../ecs/lifecycle/struct.RemovedComponentMessages.html "struct bevy::ecs::lifecycle::RemovedComponentMessages")

Retrieves this world’s [`RemovedComponentMessages`](../ecs/lifecycle/struct.RemovedComponentMessages.html "struct bevy::ecs::lifecycle::RemovedComponentMessages") collection

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#304)

#### pub fn [observers](#method.observers)(&self) -> &[Observers](../ecs/observer/struct.Observers.html "struct bevy::ecs::observer::Observers")

Retrieves this world’s [`Observers`](../ecs/observer/struct.Observers.html "struct bevy::ecs::observer::Observers") list

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#311)

#### pub fn [commands](#method.commands)(&mut self) -> [Commands](../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>

Creates a new [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands") instance that writes to the world’s command queue Use [`World::flush`](../prelude/struct.World.html#method.flush "method bevy::prelude::World::flush") to apply all queued commands

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#328)

#### pub fn [register\_component](#method.register_component)<T>(&mut self) -> [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Registers a new [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") type and returns the [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") created for it.

##### Usage Notes

In most cases, you don’t need to call this method directly since component registration happens automatically during system initialization.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#334)

#### pub fn [register\_disabling\_component](#method.register_disabling_component)<C>(&mut self)

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Registers a component type as “disabling”, using [default query filters](../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters") to exclude entities with the component from queries.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#344)

#### pub fn [register\_component\_hooks](#method.register_component_hooks)<T>(&mut self) -> &mut [ComponentHooks](../ecs/lifecycle/struct.ComponentHooks.html "struct bevy::ecs::lifecycle::ComponentHooks")

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Returns a mutable reference to the [`ComponentHooks`](../ecs/lifecycle/struct.ComponentHooks.html "struct bevy::ecs::lifecycle::ComponentHooks") for a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") type.

Will panic if `T` exists in any archetypes.

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/ecs/component\_hooks.rs ([line 67](../../src/component_hooks/component_hooks.rs.html#67))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#354-357)

#### pub fn [register\_component\_hooks\_by\_id](#method.register_component_hooks_by_id)( &mut self, id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [ComponentHooks](../ecs/lifecycle/struct.ComponentHooks.html "struct bevy::ecs::lifecycle::ComponentHooks")\>

Returns a mutable reference to the [`ComponentHooks`](../ecs/lifecycle/struct.ComponentHooks.html "struct bevy::ecs::lifecycle::ComponentHooks") for a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") with the given id if it exists.

Will panic if `id` exists in any archetypes.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#406)

#### pub fn [register\_required\_components](#method.register_required_components)<T, R>(&mut self)

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"), R: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Registers the given component `R` as a [required component](../prelude/trait.Component.html#required-components "trait bevy::prelude::Component") for `T`.

When `T` is added to an entity, `R` and its own required components will also be added if `R` was not already provided. The [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") `constructor` will be used for the creation of `R`. If a custom constructor is desired, use [`World::register_required_components_with`](../prelude/struct.World.html#method.register_required_components_with "method bevy::prelude::World::register_required_components_with") instead.

For the non-panicking version, see [`World::try_register_required_components`](../prelude/struct.World.html#method.try_register_required_components "method bevy::prelude::World::try_register_required_components").

Note that requirements must currently be registered before `T` is inserted into the world for the first time. This limitation may be fixed in the future.

##### Panics

Panics if `R` is already a directly required component for `T`, or if `T` has ever been added on an entity before the registration.

Indirect requirements through other components are allowed. In those cases, any existing requirements will only be overwritten if the new requirement is more specific.

##### Example

```rust
#[derive(Component)]
struct A;

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct B(usize);

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct C(u32);

// Register B as required by A and C as required by B.
world.register_required_components::<A, B>();
world.register_required_components::<B, C>();

// This will implicitly also insert B and C with their Default constructors.
let id = world.spawn(A).id();
assert_eq!(&B(0), world.entity(id).get::<B>().unwrap());
assert_eq!(&C(0), world.entity(id).get::<C>().unwrap());
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#457-460)

#### pub fn [register\_required\_components\_with](#method.register_required_components_with)<T, R>( &mut self, constructor: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> R, )

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"), R: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Registers the given component `R` as a [required component](../prelude/trait.Component.html#required-components "trait bevy::prelude::Component") for `T`.

When `T` is added to an entity, `R` and its own required components will also be added if `R` was not already provided. The given `constructor` will be used for the creation of `R`. If a [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") constructor is desired, use [`World::register_required_components`](../prelude/struct.World.html#method.register_required_components "method bevy::prelude::World::register_required_components") instead.

For the non-panicking version, see [`World::try_register_required_components_with`](../prelude/struct.World.html#method.try_register_required_components_with "method bevy::prelude::World::try_register_required_components_with").

Note that requirements must currently be registered before `T` is inserted into the world for the first time. This limitation may be fixed in the future.

##### Panics

Panics if `R` is already a directly required component for `T`, or if `T` has ever been added on an entity before the registration.

Indirect requirements through other components are allowed. In those cases, any existing requirements will only be overwritten if the new requirement is more specific.

##### Example

```rust
#[derive(Component)]
struct A;

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct B(usize);

#[derive(Component, PartialEq, Eq, Debug)]
struct C(u32);

// Register B and C as required by A and C as required by B.
// A requiring C directly will overwrite the indirect requirement through B.
world.register_required_components::<A, B>();
world.register_required_components_with::<B, C>(|| C(1));
world.register_required_components_with::<A, C>(|| C(2));

// This will implicitly also insert B with its Default constructor and C
// with the custom constructor defined by A.
let id = world.spawn(A).id();
assert_eq!(&B(0), world.entity(id).get::<B>().unwrap());
assert_eq!(&C(2), world.entity(id).get::<C>().unwrap());
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#512-514)

#### pub fn [try\_register\_required\_components](#method.try_register_required_components)<T, R>( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [RequiredComponentsError](../ecs/component/enum.RequiredComponentsError.html "enum bevy::ecs::component::RequiredComponentsError")\>

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"), R: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Tries to register the given component `R` as a [required component](../prelude/trait.Component.html#required-components "trait bevy::prelude::Component") for `T`.

When `T` is added to an entity, `R` and its own required components will also be added if `R` was not already provided. The [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") `constructor` will be used for the creation of `R`. If a custom constructor is desired, use [`World::register_required_components_with`](../prelude/struct.World.html#method.register_required_components_with "method bevy::prelude::World::register_required_components_with") instead.

For the panicking version, see [`World::register_required_components`](../prelude/struct.World.html#method.register_required_components "method bevy::prelude::World::register_required_components").

Note that requirements must currently be registered before `T` is inserted into the world for the first time. This limitation may be fixed in the future.

##### Errors

Returns a [`RequiredComponentsError`](../ecs/component/enum.RequiredComponentsError.html "enum bevy::ecs::component::RequiredComponentsError") if `R` is already a directly required component for `T`, or if `T` has ever been added on an entity before the registration.

Indirect requirements through other components are allowed. In those cases, any existing requirements will only be overwritten if the new requirement is more specific.

##### Example

```rust
#[derive(Component)]
struct A;

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct B(usize);

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct C(u32);

// Register B as required by A and C as required by B.
world.register_required_components::<A, B>();
world.register_required_components::<B, C>();

// Duplicate registration! This will fail.
assert!(world.try_register_required_components::<A, B>().is_err());

// This will implicitly also insert B and C with their Default constructors.
let id = world.spawn(A).id();
assert_eq!(&B(0), world.entity(id).get::<B>().unwrap());
assert_eq!(&C(0), world.entity(id).get::<C>().unwrap());
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#568-571)

#### pub fn [try\_register\_required\_components\_with](#method.try_register_required_components_with)<T, R>( &mut self, constructor: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> R, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [RequiredComponentsError](../ecs/component/enum.RequiredComponentsError.html "enum bevy::ecs::component::RequiredComponentsError")\>

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"), R: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Tries to register the given component `R` as a [required component](../prelude/trait.Component.html#required-components "trait bevy::prelude::Component") for `T`.

When `T` is added to an entity, `R` and its own required components will also be added if `R` was not already provided. The given `constructor` will be used for the creation of `R`. If a [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") constructor is desired, use [`World::register_required_components`](../prelude/struct.World.html#method.register_required_components "method bevy::prelude::World::register_required_components") instead.

For the panicking version, see [`World::register_required_components_with`](../prelude/struct.World.html#method.register_required_components_with "method bevy::prelude::World::register_required_components_with").

Note that requirements must currently be registered before `T` is inserted into the world for the first time. This limitation may be fixed in the future.

##### Errors

Returns a [`RequiredComponentsError`](../ecs/component/enum.RequiredComponentsError.html "enum bevy::ecs::component::RequiredComponentsError") if `R` is already a directly required component for `T`, or if `T` has ever been added on an entity before the registration.

Indirect requirements through other components are allowed. In those cases, any existing requirements will only be overwritten if the new requirement is more specific.

##### Example

```rust
#[derive(Component)]
struct A;

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct B(usize);

#[derive(Component, PartialEq, Eq, Debug)]
struct C(u32);

// Register B and C as required by A and C as required by B.
// A requiring C directly will overwrite the indirect requirement through B.
world.register_required_components::<A, B>();
world.register_required_components_with::<B, C>(|| C(1));
world.register_required_components_with::<A, C>(|| C(2));

// Duplicate registration! Even if the constructors were different, this would fail.
assert!(world.try_register_required_components_with::<B, C>(|| C(1)).is_err());

// This will implicitly also insert B with its Default constructor and C
// with the custom constructor defined by A.
let id = world.spawn(A).id();
assert_eq!(&B(0), world.entity(id).get::<B>().unwrap());
assert_eq!(&C(2), world.entity(id).get::<C>().unwrap());
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#589)

#### pub fn [get\_required\_components](#method.get_required_components)<C>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[RequiredComponents](../ecs/component/struct.RequiredComponents.html "struct bevy::ecs::component::RequiredComponents")\>

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Retrieves the [required components](../ecs/component/struct.RequiredComponents.html "struct bevy::ecs::component::RequiredComponents") for the given component type, if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#596)

#### pub fn [get\_required\_components\_by\_id](#method.get_required_components_by_id)( &self, id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[RequiredComponents](../ecs/component/struct.RequiredComponents.html "struct bevy::ecs::component::RequiredComponents")\>

Retrieves the [required components](../ecs/component/struct.RequiredComponents.html "struct bevy::ecs::component::RequiredComponents") for the component of the given [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#610-613)

#### pub fn [register\_component\_with\_descriptor](#method.register_component_with_descriptor)( &mut self, descriptor: [ComponentDescriptor](../ecs/component/struct.ComponentDescriptor.html "struct bevy::ecs::component::ComponentDescriptor"), ) -> [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

Registers a new [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") type and returns the [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") created for it.

This method differs from [`World::register_component`](../prelude/struct.World.html#method.register_component "method bevy::prelude::World::register_component") in that it uses a [`ComponentDescriptor`](../ecs/component/struct.ComponentDescriptor.html "struct bevy::ecs::component::ComponentDescriptor") to register the new component type instead of statically available type information. This enables the dynamic registration of new component definitions at runtime for advanced use cases.

While the option to register a component from a descriptor is useful in type-erased contexts, the standard [`World::register_component`](../prelude/struct.World.html#method.register_component "method bevy::prelude::World::register_component") function should always be used instead when type information is available at compile time.

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 164](../../src/immutable_components/immutable_components.rs.html#164))

```rust
135fn demo_3(world: &mut World) {
136    // This is a list of dynamic components we will create.
137    // The first item is the name of the component, and the second is the size
138    // in bytes.
139    let my_dynamic_components = [("Foo", 1), ("Bar", 2), ("Baz", 4)];
140
141    // This pipeline takes our component descriptions, registers them, and gets
142    // their ComponentId's.
143    let my_registered_components = my_dynamic_components
144        .into_iter()
145        .map(|(name, size)| {
146            // SAFETY:
147            // - No drop command is required
148            // - The component will store [u8; size], which is Send + Sync
149            let descriptor = unsafe {
150                ComponentDescriptor::new_with_layout(
151                    name.to_string(),
152                    StorageType::Table,
153                    Layout::array::<u8>(size).unwrap(),
154                    None,
155                    false,
156                    ComponentCloneBehavior::Default,
157                    None,
158                )
159            };
160
161            (name, size, descriptor)
162        })
163        .map(|(name, size, descriptor)| {
164            let component_id = world.register_component_with_descriptor(descriptor);
165
166            (name, size, component_id)
167        })
168        .collect::<Vec<(&str, usize, ComponentId)>>();
169
170    // Now that our components are registered, let's add them to an entity
171    let mut entity = world.spawn_empty();
172
173    for (_name, size, component_id) in &my_registered_components {
174        // We're just storing some zeroes for the sake of demonstration.
175        let data = core::iter::repeat_n(0, *size).collect::<Vec<u8>>();
176
177        OwningPtr::make(data, |ptr| {
178            // SAFETY:
179            // - ComponentId has been taken from the same world
180            // - Array is created to the layout specified in the world
181            unsafe {
182                entity.insert_by_id(*component_id, ptr);
183            }
184        });
185    }
186
187    for (_name, _size, component_id) in &my_registered_components {
188        // With immutable components, we can read the values...
189        assert!(entity.get_by_id(*component_id).is_ok());
190
191        // ...but we cannot gain a mutable reference.
192        assert!(entity.get_mut_by_id(*component_id).is_err());
193
194        // Instead, you must either remove or replace the value.
195    }
196}
```

Hide additional examples

examples/stress\_tests/many\_components.rs ([lines 86-101](../../src/many_components/many_components.rs.html#86-101))

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

examples/ecs/dynamic.rs ([lines 113-123](../../src/dynamic/dynamic.rs.html#113-123))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#645)

#### pub fn [component\_id](#method.component_id)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Returns the [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") of the given [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") type `T`.

The returned `ComponentId` is specific to the `World` instance it was retrieved from and should not be used with another `World` instance.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the `Component` type has not yet been initialized within the `World` using [`World::register_component`](../prelude/struct.World.html#method.register_component "method bevy::prelude::World::register_component").

```rust
use bevy_ecs::prelude::*;

let mut world = World::new();

#[derive(Component)]
struct ComponentA;

let component_a_id = world.register_component::<ComponentA>();

assert_eq!(component_a_id, world.component_id::<ComponentA>().unwrap())
```

##### See also

*   [`ComponentIdFor`](../ecs/component/struct.ComponentIdFor.html "struct bevy::ecs::component::ComponentIdFor")
*   [`Components::component_id()`](../ecs/component/struct.Components.html#method.component_id "method bevy::ecs::component::Components::component_id")
*   [`Components::get_id()`](../ecs/component/struct.Components.html#method.get_id "method bevy::ecs::component::Components::get_id")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#655)

#### pub fn [register\_resource](#method.register_resource)<R>(&mut self) -> [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

👎Deprecated since 0.19.0:

Use register\_component::<R>() instead.

Registers a new [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") type and returns the [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") created for it.

The [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") doesn’t have a value in the [`World`](../prelude/struct.World.html "struct bevy::prelude::World"), it’s only registered. If you want to insert the [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") in the [`World`](../prelude/struct.World.html "struct bevy::prelude::World"), use [`World::init_resource`](../prelude/struct.World.html#method.init_resource "method bevy::prelude::World::init_resource") or [`World::insert_resource`](../prelude/struct.World.html#method.insert_resource "method bevy::prelude::World::insert_resource") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#667)

#### pub fn [resource\_id](#method.resource_id)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>

where T: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

👎Deprecated since 0.19.0:

use component\_id

Returns the [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") of the given [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") type `T`.

The returned [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") is specific to the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") instance it was retrieved from and should not be used with another [`World`](../prelude/struct.World.html "struct bevy::prelude::World") instance.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") type has not yet been initialized within the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") using [`World::register_resource`](../prelude/struct.World.html#method.register_resource "method bevy::prelude::World::register_resource"), [`World::init_resource`](../prelude/struct.World.html#method.init_resource "method bevy::prelude::World::init_resource") or [`World::insert_resource`](../prelude/struct.World.html#method.insert_resource "method bevy::prelude::World::insert_resource").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#770)

#### pub fn [entity](#method.entity)<F>(&self, entities: F) -> <F as [WorldEntityFetch](../ecs/world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Ref](../ecs/world/trait.WorldEntityFetch.html#associatedtype.Ref "type bevy::ecs::world::WorldEntityFetch::Ref")<'\_>

where F: [WorldEntityFetch](../ecs/world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch"),

Returns [`EntityRef`](../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")s that expose read-only operations for the given `entities`. This will panic if any of the given entities do not exist. Use [`World::get_entity`](../prelude/struct.World.html#method.get_entity "method bevy::prelude::World::get_entity") if you want to check for entity existence instead of implicitly panicking.

This function supports fetching a single entity or multiple entities:

*   Pass an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") to receive a single [`EntityRef`](../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef").
*   Pass a slice of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive a [`Vec<EntityRef>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec").
*   Pass an array of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive an equally-sized array of [`EntityRef`](../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")s.

##### Panics

If any of the given `entities` do not exist in the world.

##### Examples

###### Single [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")

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

###### Array of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s

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

###### Slice of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s

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

###### [`EntityHashSet`](../ecs/entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

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

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 78](../../src/immutable_components/immutable_components.rs.html#78))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#896)

#### pub fn [entity\_mut](#method.entity_mut)<F>(&mut self, entities: F) -> <F as [WorldEntityFetch](../ecs/world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Mut](../ecs/world/trait.WorldEntityFetch.html#associatedtype.Mut "type bevy::ecs::world::WorldEntityFetch::Mut")<'\_>

where F: [WorldEntityFetch](../ecs/world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch"),

Returns [`EntityMut`](../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s that expose read and write operations for the given `entities`. This will panic if any of the given entities do not exist. Use [`World::get_entity_mut`](../prelude/struct.World.html#method.get_entity_mut "method bevy::prelude::World::get_entity_mut") if you want to check for entity existence instead of implicitly panicking.

This function supports fetching a single entity or multiple entities:

*   Pass an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") to receive a single [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").
    *   This reference type allows for structural changes to the entity, such as adding or removing components, or despawning the entity.
*   Pass a slice of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive a [`Vec<EntityMut>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec").
*   Pass an array of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive an equally-sized array of [`EntityMut`](../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s.
*   Pass a reference to a [`EntityHashSet`](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") to receive an [`EntityHashMap<EntityMut>`](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap").

In order to perform structural changes on the returned entity reference, such as adding or removing components, or despawning the entity, only a single [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") can be passed to this function. Allowing multiple entities at the same time with structural access would lead to undefined behavior, so [`EntityMut`](../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut") is returned when requesting multiple entities.

##### Panics

If any of the given `entities` do not exist in the world.

##### Examples

###### Single [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
let entity = world.spawn(Position { x: 0.0, y: 0.0 }).id();

let mut entity_mut = world.entity_mut(entity);
let mut position = entity_mut.get_mut::<Position>().unwrap();
position.y = 1.0;
assert_eq!(position.x, 0.0);
entity_mut.despawn();
```

###### Array of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s

```rust
#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
let e1 = world.spawn(Position { x: 0.0, y: 0.0 }).id();
let e2 = world.spawn(Position { x: 1.0, y: 1.0 }).id();

let [mut e1_ref, mut e2_ref] = world.entity_mut([e1, e2]);
let mut e1_position = e1_ref.get_mut::<Position>().unwrap();
e1_position.x = 1.0;
assert_eq!(e1_position.x, 1.0);
let mut e2_position = e2_ref.get_mut::<Position>().unwrap();
e2_position.x = 2.0;
assert_eq!(e2_position.x, 2.0);
```

###### Slice of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s

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
for mut eref in world.entity_mut(&ids[..]) {
    let mut pos = eref.get_mut::<Position>().unwrap();
    pos.y = 2.0;
    assert_eq!(pos.y, 2.0);
}
```

###### [`EntityHashSet`](../ecs/entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

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
for (_id, mut eref) in world.entity_mut(&ids) {
    let mut pos = eref.get_mut::<Position>().unwrap();
    pos.y = 2.0;
    assert_eq!(pos.y, 2.0);
}
```

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 118](../../src/immutable_components/immutable_components.rs.html#118))

```rust
103fn demo_2(world: &mut World) {
104    // Setup our name index
105    world.init_resource::<NameIndex>();
106
107    // Spawn some entities!
108    let alyssa = world.spawn(Name("Alyssa")).id();
109    let javier = world.spawn(Name("Javier")).id();
110
111    // Check our index
112    let index = world.resource::<NameIndex>();
113
114    assert_eq!(index.get_entity("Alyssa"), Some(alyssa));
115    assert_eq!(index.get_entity("Javier"), Some(javier));
116
117    // Changing the name of an entity is also fully capture by our index
118    world.entity_mut(javier).insert(Name("Steven"));
119
120    // Javier changed their name to Steven
121    let steven = javier;
122
123    // Check our index
124    let index = world.resource::<NameIndex>();
125
126    assert_eq!(index.get_entity("Javier"), None);
127    assert_eq!(index.get_entity("Steven"), Some(steven));
128}
```

Hide additional examples

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 278](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#278))

```rust
257    fn on_scene_completed(
258        &mut self,
259        load_context: &mut LoadContext<'_>,
260        _scene: &gltf::Scene,
261        _world_root_id: Entity,
262        world: &mut World,
263    ) {
264        // Create an AnimationGraph from the desired clip
265        let (graph, index) = AnimationGraph::from_clip(self.clip.clone().unwrap());
266        // Store the animation graph as an asset with an arbitrary label
267        // We only have one graph, so this label will be unique
268        let graph_handle =
269            load_context.add_labeled_asset("MyAnimationGraphLabel".to_string(), graph);
270
271        // Create a component that stores a reference to our animation
272        let animation_to_play = AnimationToPlay {
273            graph_handle,
274            index,
275        };
276
277        // Insert the `AnimationToPlay` component on the first animation root
278        let mut entity = world.entity_mut(*self.animation_root_entities.iter().next().unwrap());
279        entity.insert(animation_to_play);
280    }
```

examples/async\_tasks/async\_compute.rs ([line 100](../../src/async_compute/async_compute.rs.html#100))

```rust
66fn spawn_tasks(mut commands: Commands) {
67    let thread_pool = AsyncComputeTaskPool::get();
68    for x in 0..NUM_CUBES {
69        for y in 0..NUM_CUBES {
70            for z in 0..NUM_CUBES {
71                // Spawn new task on the AsyncComputeTaskPool; the task will be
72                // executed in the background, and the Task future returned by
73                // spawn() can be used to poll for the result
74                let entity = commands.spawn_empty().id();
75                let task = thread_pool.spawn(async move {
76                    let duration = Duration::from_secs_f32(rand::rng().random_range(0.05..5.0));
77
78                    // Pretend this is a time-intensive function. :)
79                    Delay::new(duration).await;
80
81                    // Such hard work, all done!
82                    let transform = Transform::from_xyz(x as f32, y as f32, z as f32);
83                    let mut command_queue = CommandQueue::default();
84
85                    // we use a raw command queue to pass a FnOnce(&mut World) back to be
86                    // applied in a deferred manner.
87                    command_queue.push(move |world: &mut World| {
88                        let (box_mesh_handle, box_material_handle) = {
89                            let mut system_state = SystemState::<(
90                                Res<BoxMeshHandle>,
91                                Res<BoxMaterialHandle>,
92                            )>::new(world);
93                            let (box_mesh_handle, box_material_handle) =
94                                system_state.get_mut(world).unwrap();
95
96                            (box_mesh_handle.clone(), box_material_handle.clone())
97                        };
98
99                        world
100                            .entity_mut(entity)
101                            // Add our new `Mesh3d` and `MeshMaterial3d` to our tagged entity
102                            .insert((
103                                Mesh3d(box_mesh_handle),
104                                MeshMaterial3d(box_material_handle),
105                                transform,
106                            ));
107                    });
108
109                    command_queue
110                });
111
112                // Add our new task as a component
113                commands.entity(entity).insert(ComputeTransform(task));
114            }
115        }
116    }
117}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#912-915)

#### pub fn [inspect\_entity](#method.inspect_entity)( &self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &[ComponentInfo](../ecs/component/struct.ComponentInfo.html "struct bevy::ecs::component::ComponentInfo")\>, [EntityNotSpawnedError](../ecs/entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

Returns the components of an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") through [`ComponentInfo`](../ecs/component/struct.ComponentInfo.html "struct bevy::ecs::component::ComponentInfo").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#951-954)

#### pub fn [get\_entity](#method.get_entity)<F>( &self, entities: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [WorldEntityFetch](../ecs/world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Ref](../ecs/world/trait.WorldEntityFetch.html#associatedtype.Ref "type bevy::ecs::world::WorldEntityFetch::Ref")<'\_>, [EntityNotSpawnedError](../ecs/entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

where F: [WorldEntityFetch](../ecs/world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch"),

Returns [`EntityRef`](../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")s that expose read-only operations for the given `entities`, returning [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") if any of the given entities do not exist. Instead of immediately unwrapping the value returned from this function, prefer [`World::entity`](../prelude/struct.World.html#method.entity "method bevy::prelude::World::entity").

This function supports fetching a single entity or multiple entities:

*   Pass an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") to receive a single [`EntityRef`](../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef").
*   Pass a slice of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive a [`Vec<EntityRef>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec").
*   Pass an array of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive an equally-sized array of [`EntityRef`](../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")s.
*   Pass a reference to a [`EntityHashSet`](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") to receive an [`EntityHashMap<EntityRef>`](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap").

##### Errors

If any of the given `entities` do not exist in the world, the first [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") found to be missing will return an [`EntityNotSpawnedError`](../ecs/entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError").

##### Examples

For examples, see [`World::entity`](../prelude/struct.World.html#method.entity "method bevy::prelude::World::entity").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#992-995)

#### pub fn [get\_entity\_mut](#method.get_entity_mut)<F>( &mut self, entities: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [WorldEntityFetch](../ecs/world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Mut](../ecs/world/trait.WorldEntityFetch.html#associatedtype.Mut "type bevy::ecs::world::WorldEntityFetch::Mut")<'\_>, [EntityMutableFetchError](../ecs/world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

where F: [WorldEntityFetch](../ecs/world/trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch"),

Returns [`EntityMut`](../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s that expose read and write operations for the given `entities`, returning [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") if any of the given entities do not exist. Instead of immediately unwrapping the value returned from this function, prefer [`World::entity_mut`](../prelude/struct.World.html#method.entity_mut "method bevy::prelude::World::entity_mut").

This function supports fetching a single entity or multiple entities:

*   Pass an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") to receive a single [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").
    *   This reference type allows for structural changes to the entity, such as adding or removing components, or despawning the entity.
*   Pass a slice of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive a [`Vec<EntityMut>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec").
*   Pass an array of [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity")s to receive an equally-sized array of [`EntityMut`](../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")s.
*   Pass a reference to a [`EntityHashSet`](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") to receive an [`EntityHashMap<EntityMut>`](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap").

In order to perform structural changes on the returned entity reference, such as adding or removing components, or despawning the entity, only a single [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") can be passed to this function. Allowing multiple entities at the same time with structural access would lead to undefined behavior, so [`EntityMut`](../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut") is returned when requesting multiple entities.

##### Errors

*   Returns [`EntityMutableFetchError::NotSpawned`](../ecs/world/error/enum.EntityMutableFetchError.html#variant.NotSpawned "variant bevy::ecs::world::error::EntityMutableFetchError::NotSpawned") if any of the given `entities` do not exist in the world.
    *   Only the first entity found to be missing will be returned.
*   Returns [`EntityMutableFetchError::AliasedMutability`](../ecs/world/error/enum.EntityMutableFetchError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityMutableFetchError::AliasedMutability") if the same entity is requested multiple times.

##### Examples

For examples, see [`World::entity_mut`](../prelude/struct.World.html#method.entity_mut "method bevy::prelude::World::entity_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1010)

#### pub fn [iter\_entities](#method.iter_entities)(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EntityRef](../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>>

Returns an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") iterator of current entities.

This is useful in contexts where you only have immutable access to the [`World`](../prelude/struct.World.html "struct bevy::prelude::World"). If you have mutable access to the [`World`](../prelude/struct.World.html "struct bevy::prelude::World"), use [`query()::<EntityRef>().iter(&world)`](../prelude/struct.World.html#method.query "method bevy::prelude::World::query") instead.

Note that this does iterate through _all_ entities, including resource entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1061)

#### pub fn [entities\_and\_commands](#method.entities_and_commands)(&mut self) -> ([EntityFetcher](../ecs/world/struct.EntityFetcher.html "struct bevy::ecs::world::EntityFetcher")<'\_>, [Commands](../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>)

Simultaneously provides access to entity data and a command queue, which will be applied when the world is next flushed.

This allows using borrowed entity data to construct commands where the borrow checker would otherwise prevent it.

See [`DeferredWorld::entities_and_commands`](../ecs/world/struct.DeferredWorld.html#method.entities_and_commands "method bevy::ecs::world::DeferredWorld::entities_and_commands") for the deferred version.

##### Example

```rust
#[derive(Component)]
struct Targets(Vec<Entity>);
#[derive(Component)]
struct TargetedBy(Entity);

let mut world: World = // ...
let (entities, mut commands) = world.entities_and_commands();

let entity = entities.get(eid).unwrap();
for &target in entity.get::<Targets>().unwrap().0.iter() {
    commands.entity(target).insert(TargetedBy(eid));
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1099-1103)

#### pub fn [spawn\_at](#method.spawn_at)<B>( &mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), bundle: B, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityWorldMut](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, [SpawnError](../ecs/entity/enum.SpawnError.html "enum bevy::ecs::entity::SpawnError")\>

where B: [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Spawns the bundle on the valid but not spawned entity. If the entity can not be spawned for any reason, returns an error.

If it succeeds, this declares the entity to have this bundle.

In general, you should prefer [`spawn`](../prelude/struct.World.html#method.spawn "method bevy::prelude::World::spawn"). Spawn internally calls this method, but it takes care of finding a suitable [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") for you. This is made available for advanced use, which you can see at [`EntityAllocator::alloc`](../ecs/entity/struct.EntityAllocator.html#method.alloc "method bevy::ecs::entity::EntityAllocator::alloc").

##### Risk

It is possible to spawn an `entity` that has not been allocated yet; however, doing so is currently a bad idea as the allocator may hand out this entity index in the future, assuming it to be not spawned. This would cause a panic.

Manual spawning is a powerful tool, but must be used carefully.

##### Example

Currently, this is primarily used to spawn entities that come from [`EntityAllocator::alloc`](../ecs/entity/struct.EntityAllocator.html#method.alloc "method bevy::ecs::entity::EntityAllocator::alloc"). See that for an example.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1162)

#### pub fn [spawn\_empty\_at](#method.spawn_empty_at)( &mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityWorldMut](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, [SpawnError](../ecs/entity/enum.SpawnError.html "enum bevy::ecs::entity::SpawnError")\>

A faster version of [`spawn_at`](../prelude/struct.World.html#method.spawn_at "method bevy::prelude::World::spawn_at") for the empty bundle.

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/ecs/error\_handling.rs ([line 164](../../src/error_handling/error_handling.rs.html#164))

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
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1267)

#### pub fn [spawn](#method.spawn)<B>(&mut self, bundle: B) -> [EntityWorldMut](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>

where B: [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Spawns a new [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") with a given [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") of [components](../prelude/trait.Component.html "trait bevy::prelude::Component") and returns a corresponding [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut"), which can be used to add components to the entity or retrieve its id. In case large batches of entities need to be spawned, consider using [`World::spawn_batch`](../prelude/struct.World.html#method.spawn_batch "method bevy::prelude::World::spawn_batch") instead.

```rust
use bevy_ecs::{bundle::Bundle, component::Component, world::World};

#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
};

#[derive(Component)]
struct Name(&'static str);

#[derive(Bundle)]
struct PhysicsBundle {
    position: Position,
    velocity: Velocity,
}

let mut world = World::new();

// `spawn` can accept a single component:
world.spawn(Position { x: 0.0, y: 0.0 });

// It can also accept a tuple of components:
world.spawn((
    Position { x: 0.0, y: 0.0 },
    Velocity { x: 1.0, y: 1.0 },
));

// Or it can accept a pre-defined Bundle of components:
world.spawn(PhysicsBundle {
    position: Position { x: 2.0, y: 2.0 },
    velocity: Velocity { x: 0.0, y: 4.0 },
});

let entity = world
    // Tuples can also mix Bundles and Components
    .spawn((
        PhysicsBundle {
            position: Position { x: 2.0, y: 2.0 },
            velocity: Velocity { x: 0.0, y: 4.0 },
        },
        Name("Elaina Proctor"),
    ))
    // Calling id() will return the unique identifier for the spawned entity
    .id();
let position = world.entity(entity).get::<Position>().unwrap();
assert_eq!(position.x, 2.0);
```

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/app/externally\_driven\_headless\_renderer.rs ([line 100](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#100))

```rust
96    fn spawn_camera(&mut self, target: RenderTarget) -> Entity {
97        self.0
98            .main
99            .world_mut()
100            .spawn((Camera3d::default(), target, Transform::IDENTITY))
101            .id()
102    }
103
104    // Run one world update and wait for rendering to finish.
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
119
120    // Schedules a screenshot to be captured on the next update.
121    fn screenshot(&mut self, target: RenderTarget, i: u32) {
122        self.0
123            .main
124            .world_mut()
125            .spawn(Screenshot::image(target.as_image().unwrap().clone()))
126            .observe(save_to_disk(format!("test_images/screenshot{i}.png")));
127    }
```

Hide additional examples

examples/ecs/one\_shot\_systems.rs ([line 52](../../src/one_shot_systems/one_shot_systems.rs.html#52))

```rust
47fn setup_with_world(world: &mut World) {
48    // We can run it once manually
49    world.run_system_once(system_b).unwrap();
50    // Or with a Callback
51    let system_id = world.register_system(system_b);
52    world.spawn((Callback(system_id), B));
53}
```

examples/ecs/ecs\_guide.rs ([lines 255-261](../../src/ecs_guide/ecs_guide.rs.html#255-261))

```rust
244fn exclusive_player_system(world: &mut World) {
245    // this does the same thing as "new_player_system"
246    let total_players = world.resource_mut::<GameState>().total_players;
247    let should_add_player = {
248        let game_rules = world.resource::<GameRules>();
249        let add_new_player = random::<bool>();
250        add_new_player && total_players < game_rules.max_players
251    };
252    // Randomly add a new player
253    if should_add_player {
254        println!("Player {} has joined the game!", total_players + 1);
255        world.spawn((
256            Player {
257                name: format!("Player {}", total_players + 1),
258            },
259            Score { value: 0 },
260            PlayerStreak::None,
261        ));
262
263        let mut game_state = world.resource_mut::<GameState>();
264        game_state.total_players += 1;
265    }
266}
```

examples/ecs/immutable\_components.rs ([line 32](../../src/immutable_components/immutable_components.rs.html#32))

```rust
30fn demo_1(world: &mut World) {
31    // Immutable components can be inserted just like mutable components.
32    let mut entity = world.spawn((MyMutableComponent(false), MyImmutableComponent(false)));
33
34    // But where mutable components can be mutated...
35    let mut my_mutable_component = entity.get_mut::<MyMutableComponent>().unwrap();
36    my_mutable_component.0 = true;
37
38    // ...immutable ones cannot. The below fails to compile as `MyImmutableComponent`
39    // is declared as immutable.
40    // let mut my_immutable_component = entity.get_mut::<MyImmutableComponent>().unwrap();
41
42    // Instead, you could take or replace the immutable component to update its value.
43    let mut my_immutable_component = entity.take::<MyImmutableComponent>().unwrap();
44    my_immutable_component.0 = true;
45    entity.insert(my_immutable_component);
46}
47
48/// This is an example of a component like [`Name`](bevy::prelude::Name), but immutable.
49#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component, Reflect)]
50#[reflect(Hash, Component)]
51#[component(
52    immutable,
53    // Since this component is immutable, we can fully capture all mutations through
54    // these component hooks. This allows for keeping other parts of the ECS synced
55    // to a component's value at all times.
56    on_insert = on_insert_name,
57    on_discard = on_discard_name,
58)]
59pub struct Name(pub &'static str);
60
61/// This index allows for O(1) lookups of an [`Entity`] by its [`Name`].
62#[derive(Resource, Default)]
63struct NameIndex {
64    name_to_entity: HashMap<Name, Entity>,
65}
66
67impl NameIndex {
68    fn get_entity(&self, name: &'static str) -> Option<Entity> {
69        self.name_to_entity.get(&Name(name)).copied()
70    }
71}
72
73/// When a [`Name`] is inserted, we will add it to our [`NameIndex`].
74///
75/// Since all mutations to [`Name`] are captured by hooks, we know it is not currently
76/// inserted in the index, and its value will not change without triggering a hook.
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
102
103fn demo_2(world: &mut World) {
104    // Setup our name index
105    world.init_resource::<NameIndex>();
106
107    // Spawn some entities!
108    let alyssa = world.spawn(Name("Alyssa")).id();
109    let javier = world.spawn(Name("Javier")).id();
110
111    // Check our index
112    let index = world.resource::<NameIndex>();
113
114    assert_eq!(index.get_entity("Alyssa"), Some(alyssa));
115    assert_eq!(index.get_entity("Javier"), Some(javier));
116
117    // Changing the name of an entity is also fully capture by our index
118    world.entity_mut(javier).insert(Name("Steven"));
119
120    // Javier changed their name to Steven
121    let steven = javier;
122
123    // Check our index
124    let index = world.resource::<NameIndex>();
125
126    assert_eq!(index.get_entity("Javier"), None);
127    assert_eq!(index.get_entity("Steven"), Some(steven));
128}
```

examples/scene/world\_serialization.rs ([lines 179-185](../../src/world_serialization/world_serialization.rs.html#179-185))

```rust
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

examples/ecs/dynamic.rs ([line 250](../../src/dynamic/dynamic.rs.html#250))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1308)

#### pub fn [spawn\_empty](#method.spawn_empty)(&mut self) -> [EntityWorldMut](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>

Spawns a new [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") and returns a corresponding [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut"), which can be used to add components to the entity or retrieve its id.

```rust
use bevy_ecs::{component::Component, world::World};

#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}
#[derive(Component)]
struct Label(&'static str);
#[derive(Component)]
struct Num(u32);

let mut world = World::new();
let entity = world.spawn_empty()
    .insert(Position { x: 0.0, y: 0.0 }) // add a single component
    .insert((Num(1), Label("hello"))) // add a bundle of components
    .id();

let position = world.entity(entity).get::<Position>().unwrap();
assert_eq!(position.x, 0.0);
```

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 171](../../src/immutable_components/immutable_components.rs.html#171))

```rust
135fn demo_3(world: &mut World) {
136    // This is a list of dynamic components we will create.
137    // The first item is the name of the component, and the second is the size
138    // in bytes.
139    let my_dynamic_components = [("Foo", 1), ("Bar", 2), ("Baz", 4)];
140
141    // This pipeline takes our component descriptions, registers them, and gets
142    // their ComponentId's.
143    let my_registered_components = my_dynamic_components
144        .into_iter()
145        .map(|(name, size)| {
146            // SAFETY:
147            // - No drop command is required
148            // - The component will store [u8; size], which is Send + Sync
149            let descriptor = unsafe {
150                ComponentDescriptor::new_with_layout(
151                    name.to_string(),
152                    StorageType::Table,
153                    Layout::array::<u8>(size).unwrap(),
154                    None,
155                    false,
156                    ComponentCloneBehavior::Default,
157                    None,
158                )
159            };
160
161            (name, size, descriptor)
162        })
163        .map(|(name, size, descriptor)| {
164            let component_id = world.register_component_with_descriptor(descriptor);
165
166            (name, size, component_id)
167        })
168        .collect::<Vec<(&str, usize, ComponentId)>>();
169
170    // Now that our components are registered, let's add them to an entity
171    let mut entity = world.spawn_empty();
172
173    for (_name, size, component_id) in &my_registered_components {
174        // We're just storing some zeroes for the sake of demonstration.
175        let data = core::iter::repeat_n(0, *size).collect::<Vec<u8>>();
176
177        OwningPtr::make(data, |ptr| {
178            // SAFETY:
179            // - ComponentId has been taken from the same world
180            // - Array is created to the layout specified in the world
181            unsafe {
182                entity.insert_by_id(*component_id, ptr);
183            }
184        });
185    }
186
187    for (_name, _size, component_id) in &my_registered_components {
188        // With immutable components, we can read the values...
189        assert!(entity.get_by_id(*component_id).is_ok());
190
191        // ...but we cannot gain a mutable reference.
192        assert!(entity.get_mut_by_id(*component_id).is_err());
193
194        // Instead, you must either remove or replace the value.
195    }
196}
```

Hide additional examples

examples/stress\_tests/many\_components.rs ([line 135](../../src/many_components/many_components.rs.html#135))

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

examples/ecs/dynamic.rs ([line 161](../../src/dynamic/dynamic.rs.html#161))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1341-1344)

#### pub fn [spawn\_batch](#method.spawn_batch)<I>( &mut self, iter: I, ) -> [SpawnBatchIter](../ecs/world/struct.SpawnBatchIter.html "struct bevy::ecs::world::SpawnBatchIter")<'\_, <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")\> [ⓘ](#)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item") as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../ecs/bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

Spawns a batch of entities with the same component [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") type. Takes a given [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") iterator and returns a corresponding [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") iterator. This is more efficient than spawning entities and adding components to them individually using [`World::spawn`](../prelude/struct.World.html#method.spawn "method bevy::prelude::World::spawn"), but it is limited to spawning entities with the same [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") type, whereas spawning individually is more flexible.

```rust
use bevy_ecs::{component::Component, entity::Entity, world::World};

#[derive(Component)]
struct Str(&'static str);
#[derive(Component)]
struct Num(u32);

let mut world = World::new();
let entities = world.spawn_batch(vec![
  (Str("a"), Num(0)), // the first entity
  (Str("b"), Num(1)), // the second entity
]).collect::<Vec<Entity>>();

assert_eq!(entities.len(), 2);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1366)

#### pub fn [get](#method.get)<T>(&self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Retrieves a reference to the given `entity`’s [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type. Returns `None` if the `entity` does not have a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type.

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

##### [Examples found in repository](#scraped-examples-11)[?](../../scrape-examples-help.html)

examples/ecs/component\_hooks.rs ([line 86](../../src/component_hooks/component_hooks.rs.html#86))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1387-1390)

#### pub fn [get\_mut](#method.get_mut)<T>(&mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>>

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Retrieves a mutable reference to the given `entity`’s [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type. Returns `None` if the `entity` does not have a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type.

```rust
use bevy_ecs::{component::Component, world::World};

#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
let entity = world.spawn(Position { x: 0.0, y: 0.0 }).id();
let mut position = world.get_mut::<Position>(entity).unwrap();
position.x = 1.0;
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1427-1431)

#### pub fn [modify\_component](#method.modify_component)<T, R>( &mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<R>, [EntityMutableFetchError](../ecs/world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Temporarily removes a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") `T` from the provided [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") and runs the provided closure on it, returning the result if `T` was available. This will trigger the `Remove` and `Discard` component hooks without causing an archetype move.

This is most useful with immutable components, where removal and reinsertion is the only way to modify a value.

If you do not need to ensure the above hooks are triggered, and your component is mutable, prefer using [`get_mut`](../prelude/struct.World.html#method.get_mut "method bevy::prelude::World::get_mut").

##### Examples

```rust
#[derive(Component, PartialEq, Eq, Debug)]
#[component(immutable)]
struct Foo(bool);

world.modify_component(entity, |foo: &mut Foo| {
    foo.0 = true;
});
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1460-1465)

#### pub fn [modify\_component\_by\_id](#method.modify_component_by_id)<R>( &mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), f: impl for<'a> [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([MutUntyped](../ecs/change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'a>) -> R, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<R>, [EntityMutableFetchError](../ecs/world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

Temporarily removes a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") identified by the provided [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") from the provided [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") and runs the provided closure on it, returning the result if the component was available. This will trigger the `Remove` and `Discard` component hooks without causing an archetype move.

This is most useful with immutable components, where removal and reinsertion is the only way to modify a value.

If you do not need to ensure the above hooks are triggered, and your component is mutable, prefer using [`get_mut_by_id`](../prelude/struct.World.html#method.get_mut_by_id "method bevy::prelude::World::get_mut_by_id").

You should prefer the typed [`modify_component`](../prelude/struct.World.html#method.modify_component "method bevy::prelude::World::modify_component") whenever possible.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1510-1513)

#### pub fn [modify\_resource](#method.modify_resource)<R, S>( &mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<S>, [EntityMutableFetchError](../ecs/world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Temporarily removes a [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") `R` and runs the provided closure on it, returning the result if `R` was available. This will trigger the `Remove` and `Discard` component hooks without causing an archetype move.

This is most useful with immutable resources, where removal and reinsertion is the only way to modify a value.

If you do not need to ensure the above hooks are triggered, and your resource is mutable, prefer using [`get_resource_mut`](../prelude/struct.World.html#method.get_resource_mut "method bevy::prelude::World::get_resource_mut").

##### Examples

```rust
#[derive(Resource, PartialEq, Eq, Debug)]
#[component(immutable)]
struct Bar(bool);

world.modify_resource(|bar: &mut Bar| {
    bar.0 = true;
});
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1546-1550)

#### pub fn [modify\_resource\_by\_id](#method.modify_resource_by_id)<S>( &mut self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), f: impl for<'a> [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([MutUntyped](../ecs/change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'a>) -> S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<S>, [EntityMutableFetchError](../ecs/world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

Temporarily removes a [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") identified by the provided [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") and runs the provided closure on it, returning the result if the component was available. This will trigger the `Remove` and `Discard` component hooks without causing an archetype move.

This is most useful with immutable resources, where removal and reinsertion is the only way to modify a value.

If you do not need to ensure the above hooks are triggered, and your resource is mutable, prefer using [`get_resource_mut_by_id`](../prelude/struct.World.html#method.get_resource_mut_by_id "method bevy::prelude::World::get_resource_mut_by_id").

You should prefer the typed [`modify_resource`](../prelude/struct.World.html#method.modify_resource "method bevy::prelude::World::modify_resource") whenever possible.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1598)

#### pub fn [despawn](#method.despawn)(&mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Despawns the given [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), if it exists. This will also remove all of the entity’s [`Components`](../prelude/trait.Component.html "trait bevy::prelude::Component").

Returns `true` if the entity is successfully despawned and `false` if the entity does not exist. This counts despawning a not constructed entity as a success, and frees it to the allocator. See [entity](../ecs/entity/index.html "mod bevy::ecs::entity") module docs for more about construction.

##### Note

This will also despawn the entities in any [`RelationshipTarget`](../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") that is configured to despawn descendants. For example, this will recursively despawn [`Children`](../prelude/struct.Children.html "struct bevy::prelude::Children").

```rust
use bevy_ecs::{component::Component, world::World};

#[derive(Component)]
struct Position {
  x: f32,
  y: f32,
}

let mut world = World::new();
let entity = world.spawn(Position { x: 0.0, y: 0.0 }).id();
assert!(world.despawn(entity));
assert!(world.get_entity(entity).is_err());
assert!(world.get::<Position>(entity).is_none());
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1618)

#### pub fn [try\_despawn](#method.try_despawn)(&mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [EntityDespawnError](../ecs/world/error/struct.EntityDespawnError.html "struct bevy::ecs::world::error::EntityDespawnError")\>

Despawns the given `entity`, if it exists. This will also remove all of the entity’s [`Components`](../prelude/trait.Component.html "trait bevy::prelude::Component").

Returns an [`EntityDespawnError`](../ecs/world/error/struct.EntityDespawnError.html "struct bevy::ecs::world::error::EntityDespawnError") if the entity is not spawned to be despawned.

##### Note

This will also despawn the entities in any [`RelationshipTarget`](../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") that is configured to despawn descendants. For example, this will recursively despawn [`Children`](../prelude/struct.Children.html "struct bevy::prelude::Children").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1643)

#### pub fn [despawn\_no\_free](#method.despawn_no_free)(&mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

Performs [`try_despawn_no_free`](../prelude/struct.World.html#method.try_despawn_no_free "method bevy::prelude::World::try_despawn_no_free"), warning on errors. See that method for more information.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1677)

#### pub fn [try\_despawn\_no\_free](#method.try_despawn_no_free)( &mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityDespawnError](../ecs/world/error/struct.EntityDespawnError.html "struct bevy::ecs::world::error::EntityDespawnError")\>

Despawns the given `entity`, if it exists. This will also remove all of the entity’s [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")s.

The _only_ difference between this and [despawning](../prelude/struct.World.html#method.despawn "method bevy::prelude::World::despawn") an entity is that this does not release the `entity` to be reused. It is up to the caller to either re-spawn or free the `entity`; otherwise, the [`EntityIndex`](../ecs/entity/struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex") will not be able to be reused. In general, [`despawn`](../prelude/struct.World.html#method.despawn "method bevy::prelude::World::despawn") should be used instead, which automatically allows the row to be reused.

Returns the new [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") if of the despawned [`EntityIndex`](../ecs/entity/struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex"), which should eventually either be re-spawned or freed to the allocator. Returns an [`EntityDespawnError`](../ecs/world/error/struct.EntityDespawnError.html "struct bevy::ecs::world::error::EntityDespawnError") if the entity is not spawned.

##### Note

This will also _despawn_ the entities in any [`RelationshipTarget`](../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") that is configured to despawn descendants. For example, this will recursively despawn [`Children`](../prelude/struct.Children.html "struct bevy::prelude::Children").

##### Example

There is no simple example in which this would be practical, but one use for this is a custom entity allocator. Despawning internally calls this and frees the entity id to Bevy’s default entity allocator. The same principal can be used to create custom allocators with additional properties. For example, this could be used to make an allocator that yields groups of consecutive [`EntityIndex`](../ecs/entity/struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex")s, etc. See [`EntityAllocator::alloc`](../ecs/entity/struct.EntityAllocator.html#method.alloc "method bevy::ecs::entity::EntityAllocator::alloc") for more on this.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1735)

#### pub fn [clear\_trackers](#method.clear_trackers)(&mut self)

Clears the internal component tracker state.

The world maintains some internal state about changed and removed components. This state is used by [`RemovedComponents`](../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents") to provide access to the entities that had a specific type of component removed since last tick.

The state is also used for change detection when accessing components and resources outside of a system, for example via [`World::get_mut()`](../prelude/struct.World.html#method.get_mut "method bevy::prelude::World::get_mut") or [`World::get_resource_mut()`](../prelude/struct.World.html#method.get_resource_mut "method bevy::prelude::World::get_resource_mut").

By clearing this internal state, the world “forgets” about those changes, allowing a new round of detection to be recorded.

When using `bevy_ecs` as part of the full Bevy engine, this method is called automatically by `bevy_app::App::update` and `bevy_app::SubApp::update`, so you don’t need to call it manually. When using `bevy_ecs` as a separate standalone crate however, you do need to call this manually.

```rust
// a whole new world
let mut world = World::new();

// you changed it
let entity = world.spawn(Transform::default()).id();

// change is detected
let transform = world.get_mut::<Transform>(entity).unwrap();
assert!(transform.is_changed());

// update the last change tick
world.clear_trackers();

// change is no longer detected
let transform = world.get_mut::<Transform>(entity).unwrap();
assert!(!transform.is_changed());
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1803)

#### pub fn [query](#method.query)<D>(&mut self) -> [QueryState](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"),

Returns [`QueryState`](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") for the given [`QueryData`](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), which is used to efficiently run queries on the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") by storing and reusing the [`QueryState`](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

```rust
use bevy_ecs::{component::Component, entity::Entity, world::World};

#[derive(Component, Debug, PartialEq)]
struct Position {
  x: f32,
  y: f32,
}

#[derive(Component)]
struct Velocity {
  x: f32,
  y: f32,
}

let mut world = World::new();
let entities = world.spawn_batch(vec![
    (Position { x: 0.0, y: 0.0}, Velocity { x: 1.0, y: 0.0 }),
    (Position { x: 0.0, y: 0.0}, Velocity { x: 0.0, y: 1.0 }),
]).collect::<Vec<Entity>>();

let mut query = world.query::<(&mut Position, &Velocity)>();
for (mut position, velocity) in query.iter_mut(&mut world) {
   position.x += velocity.x;
   position.y += velocity.y;
}

assert_eq!(world.get::<Position>(entities[0]).unwrap(), &Position { x: 1.0, y: 0.0 });
assert_eq!(world.get::<Position>(entities[1]).unwrap(), &Position { x: 0.0, y: 1.0 });
```

To iterate over entities in a deterministic order, sort the results of the query using the desired component as a key. Note that this requires fetching the whole result set from the query and allocation of a [`Vec`](../prelude/struct.Vec.html "struct bevy::prelude::Vec") to store it.

```rust
use bevy_ecs::{component::Component, entity::Entity, world::World};

#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Order(i32);
#[derive(Component, PartialEq, Debug)]
struct Label(&'static str);

let mut world = World::new();
let a = world.spawn((Order(2), Label("second"))).id();
let b = world.spawn((Order(3), Label("third"))).id();
let c = world.spawn((Order(1), Label("first"))).id();
let mut entities = world.query::<(Entity, &Order, &Label)>()
    .iter(&world)
    .collect::<Vec<_>>();
// Sort the query results by their `Order` component before comparing
// to expected results. Query iteration order should not be relied on.
entities.sort_by_key(|e| e.1);
assert_eq!(entities, vec![
    (c, &Order(1), &Label("first")),
    (a, &Order(2), &Label("second")),
    (b, &Order(3), &Label("third")),
]);
```

##### [Examples found in repository](#scraped-examples-12)[?](../../scrape-examples-help.html)

examples/window/persisting\_window\_settings.rs ([line 56](../../src/persisting_window_settings/persisting_window_settings.rs.html#56))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1827)

#### pub fn [query\_filtered](#method.query_filtered)<D, F>(&mut self) -> [QueryState](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Returns [`QueryState`](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") for the given filtered [`QueryData`](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), which is used to efficiently run queries on the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") by storing and reusing the [`QueryState`](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

```rust
use bevy_ecs::{component::Component, entity::Entity, world::World, query::With};

#[derive(Component)]
struct A;
#[derive(Component)]
struct B;

let mut world = World::new();
let e1 = world.spawn(A).id();
let e2 = world.spawn((A, B)).id();

let mut query = world.query_filtered::<Entity, With<B>>();
let matching_entities = query.iter(&world).collect::<Vec<Entity>>();

assert_eq!(matching_entities, vec![e2]);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1878)

#### pub fn [try\_query](#method.try_query)<D>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[QueryState](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D>>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"),

Returns [`QueryState`](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") for the given [`QueryData`](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), which is used to efficiently run queries on the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") by storing and reusing the [`QueryState`](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

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

#### pub fn [try\_query\_filtered](#method.try_query_filtered)<D, F>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[QueryState](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

Returns [`QueryState`](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState") for the given filtered [`QueryData`](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), which is used to efficiently run queries on the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") by storing and reusing the [`QueryState`](../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

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

#### pub fn [removed](#method.removed)<T>(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Returns an iterator of entities that had components of type `T` removed since the last call to [`World::clear_trackers`](../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1921)

#### pub fn [removed\_with\_id](#method.removed_with_id)( &self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

Returns an iterator of entities that had components with the given `component_id` removed since the last call to [`World::clear_trackers`](../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1939-1942)

#### pub fn [register\_non\_send\_with\_descriptor](#method.register_non_send_with_descriptor)( &mut self, descriptor: [ComponentDescriptor](../ecs/component/struct.ComponentDescriptor.html "struct bevy::ecs::component::ComponentDescriptor"), ) -> [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

Registers a new non-send resource type and returns the [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") created for it.

This enables the dynamic registration of new non-send resources definitions at runtime for advanced use cases.

##### Note

Registering a non-send resource does not insert it into [`World`](../prelude/struct.World.html "struct bevy::prelude::World"). For insertion, you could use [`World::insert_non_send_by_id`](../prelude/struct.World.html#method.insert_non_send_by_id "method bevy::prelude::World::insert_non_send_by_id").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1984)

#### pub fn [init\_resource](#method.init_resource)<R>(&mut self) -> [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Initializes a new resource and returns the [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") created for it.

If the resource already exists, nothing happens.

The value given by the [`FromWorld::from_world`](../prelude/trait.FromWorld.html#tymethod.from_world "associated function bevy::prelude::FromWorld::from_world") method will be used. Note that any resource with the [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") trait automatically implements [`FromWorld`](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"), and those default values will be here instead.

##### [Examples found in repository](#scraped-examples-13)[?](../../scrape-examples-help.html)

examples/ecs/custom\_executor.rs ([line 47](../../src/custom_executor/custom_executor.rs.html#47))

```rust
45fn main() {
46    let mut world = World::new();
47    world.init_resource::<Counter>();
48
49    let mut schedule = Schedule::default();
50    schedule.set_executor(CustomExecutor);
51    schedule.add_systems((increment, print_counter).chain());
52
53    for _ in 0..5 {
54        schedule.run(&mut world);
55    }
56}
```

Hide additional examples

examples/ecs/immutable\_components.rs ([line 105](../../src/immutable_components/immutable_components.rs.html#105))

```rust
103fn demo_2(world: &mut World) {
104    // Setup our name index
105    world.init_resource::<NameIndex>();
106
107    // Spawn some entities!
108    let alyssa = world.spawn(Name("Alyssa")).id();
109    let javier = world.spawn(Name("Javier")).id();
110
111    // Check our index
112    let index = world.resource::<NameIndex>();
113
114    assert_eq!(index.get_entity("Alyssa"), Some(alyssa));
115    assert_eq!(index.get_entity("Javier"), Some(javier));
116
117    // Changing the name of an entity is also fully capture by our index
118    world.entity_mut(javier).insert(Name("Steven"));
119
120    // Javier changed their name to Steven
121    let steven = javier;
122
123    // Check our index
124    let index = world.resource::<NameIndex>();
125
126    assert_eq!(index.get_entity("Javier"), None);
127    assert_eq!(index.get_entity("Steven"), Some(steven));
128}
```

examples/ecs/dynamic.rs ([line 259](../../src/dynamic/dynamic.rs.html#259))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#1997)

#### pub fn [insert\_resource](#method.insert_resource)<R>(&mut self, value: R)

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Inserts a new resource with the given `value`.

Resources are “unique” data of a given type. If you insert a resource of a type that already exists, you will overwrite any existing data.

##### [Examples found in repository](#scraped-examples-14)[?](../../scrape-examples-help.html)

examples/scene/world\_serialization.rs ([line 187](../../src/world_serialization/world_serialization.rs.html#187))

```rust
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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2020)

#### pub fn [init\_non\_send\_resource](#method.init_non_send_resource)<R>(&mut self) -> [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

where R: 'static + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

👎Deprecated since 0.19.0:

use World::init\_non\_send

Initializes a new non-send resource and returns the [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") created for it.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2037)

#### pub fn [init\_non\_send](#method.init_non_send)<R>(&mut self) -> [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

where R: 'static + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Initializes new non-send data and returns the [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") created for it.

If the data already exists, nothing happens.

The value given by the [`FromWorld::from_world`](../prelude/trait.FromWorld.html#tymethod.from_world "associated function bevy::prelude::FromWorld::from_world") method will be used. Note that any non-send data with the `Default` trait automatically implements `FromWorld`, and those default values will be here instead.

##### Panics

Panics if called from a thread other than the main thread.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2059)

#### pub fn [insert\_non\_send\_resource](#method.insert_non_send_resource)<R>(&mut self, value: R)

where R: 'static,

👎Deprecated since 0.19.0:

use World::insert\_non\_send

Inserts a new non-send resource with the given `value`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2074)

#### pub fn [insert\_non\_send](#method.insert_non_send)<R>(&mut self, value: R)

where R: 'static,

Inserts new non-send data with the given `value`.

`NonSend` data cannot be sent across threads, and do not need the `Send + Sync` bounds. Systems with `NonSend` resources are always scheduled on the main thread.

##### Panics

If a value is already present, this function will panic if called from a different thread than where the original value was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2087)

#### pub fn [remove\_resource](#method.remove_resource)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<R>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Removes the resource of a given type and returns it, if it exists. Otherwise returns `None`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2099)

#### pub fn [remove\_non\_send\_resource](#method.remove_non_send_resource)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<R>

where R: 'static,

👎Deprecated since 0.19.0:

use World::remove\_non\_send

Removes a `!Send` resource from the world and returns it, if present.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2115)

#### pub fn [remove\_non\_send](#method.remove_non_send)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<R>

where R: 'static,

Removes `!Send` data from the world and returns it, if present.

`NonSend` resources cannot be sent across threads, and do not need the `Send + Sync` bounds. Systems with `NonSend` data are always scheduled on the main thread.

Returns `None` if a value was not previously present.

##### Panics

If a value is present, this function will panic if called from a different thread than where the value was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2124)

#### pub fn [contains\_resource](#method.contains_resource)<R>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Returns `true` if a resource of type `R` exists. Otherwise returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2132)

#### pub fn [contains\_resource\_by\_id](#method.contains_resource_by_id)(&self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if a resource with provided `component_id` exists. Otherwise returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2143)

#### pub fn [contains\_non\_send](#method.contains_non_send)<R>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where R: 'static,

Returns `true` if `!Send` data of type `R` exists. Otherwise returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2152)

#### pub fn [contains\_non\_send\_by\_id](#method.contains_non_send_by_id)(&self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if `!Send` data with `component_id` exists. Otherwise returns `false`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2166)

#### pub fn [is\_resource\_added](#method.is_resource_added)<R>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Returns `true` if a resource of type `R` exists and was added since the world’s [`last_change_tick`](../prelude/struct.World.html#method.last_change_tick "method bevy::prelude::World::last_change_tick"). Otherwise, this returns `false`.

This means that:

*   When called from an exclusive system, this will check for additions since the system last ran.
*   When called elsewhere, this will check for additions since the last time that [`World::clear_trackers`](../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2179)

#### pub fn [is\_resource\_added\_by\_id](#method.is_resource_added_by_id)(&self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if a resource with id `component_id` exists and was added since the world’s [`last_change_tick`](../prelude/struct.World.html#method.last_change_tick "method bevy::prelude::World::last_change_tick"). Otherwise, this returns `false`.

This means that:

*   When called from an exclusive system, this will check for additions since the system last ran.
*   When called elsewhere, this will check for additions since the last time that [`World::clear_trackers`](../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2191)

#### pub fn [is\_resource\_changed](#method.is_resource_changed)<R>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Returns `true` if a resource of type `R` exists and was modified since the world’s [`last_change_tick`](../prelude/struct.World.html#method.last_change_tick "method bevy::prelude::World::last_change_tick"). Otherwise, this returns `false`.

This means that:

*   When called from an exclusive system, this will check for changes since the system last ran.
*   When called elsewhere, this will check for changes since the last time that [`World::clear_trackers`](../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2204)

#### pub fn [is\_resource\_changed\_by\_id](#method.is_resource_changed_by_id)(&self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if a resource with id `component_id` exists and was modified since the world’s [`last_change_tick`](../prelude/struct.World.html#method.last_change_tick "method bevy::prelude::World::last_change_tick"). Otherwise, this returns `false`.

This means that:

*   When called from an exclusive system, this will check for changes since the system last ran.
*   When called elsewhere, this will check for changes since the last time that [`World::clear_trackers`](../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2210)

#### pub fn [get\_resource\_change\_ticks](#method.get_resource_change_ticks)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentTicks](../ecs/change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")\>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Retrieves the change ticks for the given resource.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2219-2222)

#### pub fn [get\_resource\_change\_ticks\_by\_id](#method.get_resource_change_ticks_by_id)( &self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentTicks](../ecs/change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")\>

Retrieves the change ticks for the given [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId").

**You should prefer to use the typed API [`World::get_resource_change_ticks`](../prelude/struct.World.html#method.get_resource_change_ticks "method bevy::prelude::World::get_resource_change_ticks") where possible.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2239)

#### pub fn [resource](#method.resource)<R>(&self) -> [&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference to the resource of the given type

##### Panics

Panics if the resource does not exist. Use [`get_resource`](../prelude/struct.World.html#method.get_resource "method bevy::prelude::World::get_resource") instead if you want to handle this case.

If you want to instead insert a value if the resource does not exist, use [`get_resource_or_insert_with`](../prelude/struct.World.html#method.get_resource_or_insert_with "method bevy::prelude::World::get_resource_or_insert_with").

##### [Examples found in repository](#scraped-examples-15)[?](../../scrape-examples-help.html)

examples/3d/specular\_tint.rs ([line 33](../../src/specular_tint/specular_tint.rs.html#33))

```rust
32    fn from_world(world: &mut World) -> Self {
33        let asset_server = world.resource::<AssetServer>();
34        Self {
35            noise_texture: asset_server.load("textures/AlphaNoise.png"),
36        }
37    }
```

Hide additional examples

examples/audio/play\_sound\_effect.rs ([line 14](../../src/play_sound_effect/play_sound_effect.rs.html#14))

```rust
13    fn from_world(world: &mut World) -> Self {
14        let asset_server = world.resource::<AssetServer>();
15        SoundEffect {
16            handle: asset_server.load("sounds/breakout_collision.ogg"),
17        }
18    }
```

examples/scene/world\_serialization.rs ([line 89](../../src/world_serialization/world_serialization.rs.html#89))

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

examples/app/externally\_driven\_headless\_renderer.rs ([line 111](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#111))

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

examples/2d/dynamic\_mip\_generation.rs ([line 295](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#295))

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

tests/ecs/ambiguity\_detection.rs ([line 94](../../src/ambiguity_detection/ambiguity_detection.rs.html#94))

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

*   [examples/ecs/immutable\_components.rs](../../src/immutable_components/immutable_components.rs.html#112)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#248)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#50)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#323)
*   [examples/ecs/component\_hooks.rs](../../src/component_hooks/component_hooks.rs.html#104)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2263)

#### pub fn [resource\_ref](#method.resource_ref)<R>(&self) -> [Ref](../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_, R>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference to the resource of the given type

##### Panics

Panics if the resource does not exist. Use [`get_resource_ref`](../prelude/struct.World.html#method.get_resource_ref "method bevy::prelude::World::get_resource_ref") instead if you want to handle this case.

If you want to instead insert a value if the resource does not exist, use [`get_resource_or_insert_with`](../prelude/struct.World.html#method.get_resource_or_insert_with "method bevy::prelude::World::get_resource_or_insert_with").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2287)

#### pub fn [resource\_mut](#method.resource_mut)<R>(&mut self) -> [Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets a mutable reference to the resource of the given type

##### Panics

Panics if the resource does not exist. Use [`get_resource_mut`](../prelude/struct.World.html#method.get_resource_mut "method bevy::prelude::World::get_resource_mut") instead if you want to handle this case.

If you want to instead insert a value if the resource does not exist, use [`get_resource_or_insert_with`](../prelude/struct.World.html#method.get_resource_or_insert_with "method bevy::prelude::World::get_resource_or_insert_with").

##### [Examples found in repository](#scraped-examples-16)[?](../../scrape-examples-help.html)

examples/shader/shader\_material\_wesl.rs ([line 44](../../src/shader_material_wesl/shader_material_wesl.rs.html#44))

```rust
41    fn build(&self, app: &mut App) {
42        let handle = app
43            .world_mut()
44            .resource_mut::<AssetServer>()
45            .load::<Shader>("shaders/util.wesl");
46        app.insert_resource(UtilityShader(handle));
47    }
```

Hide additional examples

examples/2d/dynamic\_mip\_generation.rs ([line 292](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#292))

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

examples/shader\_advanced/compute\_mesh.rs ([line 68](../../src/compute_mesh/compute_mesh.rs.html#68))

```rust
62    fn finish(&self, app: &mut App) {
63        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
64            return;
65        };
66        render_app
67            .world_mut()
68            .resource_mut::<MeshAllocatorSettings>()
69            // This allows using the mesh allocator slabs as
70            // storage buffers directly in the compute shader.
71            // Which means that we can write from our compute
72            // shader directly to the allocated mesh slabs.
73            .extra_buffer_usages = BufferUsages::STORAGE;
74    }
```

examples/app/custom\_loop.rs ([line 19](../../src/custom_loop/custom_loop.rs.html#19))

```rust
10fn my_runner(mut app: App) -> AppExit {
11    // Finalize plugin building, including running any necessary clean-up.
12    // This is normally completed by the default runner.
13    app.finish();
14    app.cleanup();
15
16    println!("Type stuff into the console");
17    for line in io::stdin().lines() {
18        {
19            let mut input = app.world_mut().resource_mut::<Input>();
20            input.0 = line.unwrap();
21        }
22        app.update();
23
24        if let Some(exit) = app.should_exit() {
25            return exit;
26        }
27    }
28
29    AppExit::Success
30}
```

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 142](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#142))

```rust
130    fn build(&self, app: &mut App) {
131        #[cfg(target_family = "wasm")]
132        bevy::tasks::block_on(async {
133            app.world_mut()
134                .resource_mut::<GltfExtensionHandlers>()
135                .0
136                .write()
137                .await
138                .push(Box::new(GltfExtensionHandlerAnimation::default()))
139        });
140        #[cfg(not(target_family = "wasm"))]
141        app.world_mut()
142            .resource_mut::<GltfExtensionHandlers>()
143            .0
144            .write_blocking()
145            .push(Box::new(GltfExtensionHandlerAnimation::default()));
146    }
```

examples/gltf/gltf\_extension\_mesh\_2d.rs ([line 82](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#82))

```rust
70    fn build(&self, app: &mut App) {
71        #[cfg(target_family = "wasm")]
72        bevy::tasks::block_on(async {
73            app.world_mut()
74                .resource_mut::<GltfExtensionHandlers>()
75                .0
76                .write()
77                .await
78                .push(Box::new(GltfExtensionHandlerToMesh2d))
79        });
80        #[cfg(not(target_family = "wasm"))]
81        app.world_mut()
82            .resource_mut::<GltfExtensionHandlers>()
83            .0
84            .write_blocking()
85            .push(Box::new(GltfExtensionHandlerToMesh2d));
86
87        app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
88    }
```

Additional examples can be found in:  

*   [examples/app/externally\_driven\_headless\_renderer.rs](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#91)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#246)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#308)
*   [tests/ecs/ambiguity\_detection.rs](../../src/ambiguity_detection/ambiguity_detection.rs.html#71)
*   [examples/showcase/stepping.rs](../../src/breakout/stepping.rs.html#44)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#302)
*   [examples/time/time.rs](../../src/time/time.rs.html#51)
*   [examples/ecs/custom\_schedule.rs](../../src/custom_schedule/custom_schedule.rs.html#38)
*   [examples/ecs/system\_stepping.rs](../../src/system_stepping/system_stepping.rs.html#59)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2302)

#### pub fn [get\_resource](#method.get_resource)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference to the resource of the given type if it exists

##### [Examples found in repository](#scraped-examples-17)[?](../../scrape-examples-help.html)

examples/window/persisting\_window\_settings.rs ([line 51](../../src/persisting_window_settings/persisting_window_settings.rs.html#51))

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

examples/ecs/error\_handling.rs ([line 155](../../src/error_handling/error_handling.rs.html#155))

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

examples/ecs/dynamic.rs ([line 282](../../src/dynamic/dynamic.rs.html#282))

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

#### pub fn [get\_resource\_ref](#method.get_resource_ref)<R>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ref](../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_, R>>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Gets a reference including change detection to the resource of the given type if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2320)

#### pub fn [get\_resource\_mut](#method.get_resource_mut)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets a mutable reference to the resource of the given type if it exists

##### [Examples found in repository](#scraped-examples-18)[?](../../scrape-examples-help.html)

examples/showcase/loading\_screen.rs ([line 301](../../src/loading_screen/loading_screen.rs.html#301))

```rust
300    fn update_pipelines_ready(mut main_world: ResMut<MainWorld>, pipelines: Res<PipelineCache>) {
301        if let Some(mut pipelines_ready) = main_world.get_resource_mut::<PipelinesReady>() {
302            pipelines_ready.0 = pipelines.waiting_pipelines().count() == 0;
303        }
304    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2344-2347)

#### pub fn [get\_resource\_or\_insert\_with](#method.get_resource_or_insert_with)<R>( &mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> R, ) -> [Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets a mutable reference to the resource of type `T` if it exists, otherwise inserts the resource using the result of calling `func`.

##### Example

```rust
#[derive(Resource)]
struct MyResource(i32);

let my_res = world.get_resource_or_insert_with(|| MyResource(10));
assert_eq!(my_res.0, 10);
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2391-2393)

#### pub fn [get\_resource\_or\_init](#method.get_resource_or_init)<R>(&mut self) -> [Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\> + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Gets a mutable reference to the resource of type `T` if it exists, otherwise initializes the resource by calling its [`FromWorld`](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") implementation.

##### Example

```rust
#[derive(Resource)]
struct Foo(i32);

impl Default for Foo {
    fn default() -> Self {
        Self(15)
    }
}

#[derive(Resource)]
struct MyResource(i32);

impl FromWorld for MyResource {
    fn from_world(world: &mut World) -> Self {
        let foo = world.get_resource_or_init::<Foo>();
        Self(foo.0 * 2)
    }
}

let my_res = world.get_resource_or_init::<MyResource>();
assert_eq!(my_res.0, 30);
```

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

Panics if the data does not exist. Use [`get_non_send`](../prelude/struct.World.html#method.get_non_send "method bevy::prelude::World::get_non_send") instead if you want to handle this case.

This function will panic if it isn’t called from the same thread that the resource was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2434)

#### pub fn [non\_send\_resource\_mut](#method.non_send_resource_mut)<R>(&mut self) -> [Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>

where R: 'static,

👎Deprecated since 0.19.0:

use World::non\_send\_mut

Gets a mutable reference to a non-send resource of the given type, if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2448)

#### pub fn [non\_send\_mut](#method.non_send_mut)<R>(&mut self) -> [Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>

where R: 'static,

Gets a mutable reference to the non-send data of the given type, if it exists.

##### Panics

Panics if the data does not exist. Use [`get_non_send_mut`](../prelude/struct.World.html#method.get_non_send_mut "method bevy::prelude::World::get_non_send_mut") instead if you want to handle this case.

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2483)

#### pub fn [get\_non\_send\_resource\_mut](#method.get_non_send_resource_mut)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>>

where R: 'static,

👎Deprecated since 0.19.0:

use World::get\_non\_send\_mut

Gets a mutable reference to a non-send resource of the given type, if it exists. Otherwise returns `None`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2493)

#### pub fn [get\_non\_send\_mut](#method.get_non_send_mut)<R>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>>

where R: 'static,

Gets a mutable reference to the non-send data of the given type, if it exists. Otherwise returns `None`.

##### Panics

This function will panic if it isn’t called from the same thread that the resource was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2516-2520)

#### pub fn [insert\_batch](#method.insert_batch)<I, B>(&mut self, batch: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"): [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = ([Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), B)>, B: [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <B as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../ecs/bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

For a given batch of ([`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")) pairs, adds the `Bundle` of components to each `Entity`. This is faster than doing equivalent operations one-by-one.

A batch can be any type that implements [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") containing `(Entity, Bundle)` tuples, such as a [`Vec<(Entity, Bundle)>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec") or an array `[(Entity, Bundle); N]`.

This will overwrite any previous values of components shared by the `Bundle`. See [`World::insert_batch_if_new`](../prelude/struct.World.html#method.insert_batch_if_new "method bevy::prelude::World::insert_batch_if_new") to keep the old values instead.

##### Panics

This function will panic if any of the associated entities do not exist.

For the fallible version, see [`World::try_insert_batch`](../prelude/struct.World.html#method.try_insert_batch "method bevy::prelude::World::try_insert_batch").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2541-2545)

#### pub fn [insert\_batch\_if\_new](#method.insert_batch_if_new)<I, B>(&mut self, batch: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"): [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = ([Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), B)>, B: [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <B as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../ecs/bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

For a given batch of ([`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")) pairs, adds the `Bundle` of components to each `Entity` without overwriting. This is faster than doing equivalent operations one-by-one.

A batch can be any type that implements [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") containing `(Entity, Bundle)` tuples, such as a [`Vec<(Entity, Bundle)>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec") or an array `[(Entity, Bundle); N]`.

This is the same as [`World::insert_batch`](../prelude/struct.World.html#method.insert_batch "method bevy::prelude::World::insert_batch"), but in case of duplicate components it will leave the old values instead of replacing them with new ones.

##### Panics

This function will panic if any of the associated entities do not exist.

For the fallible version, see [`World::try_insert_batch_if_new`](../prelude/struct.World.html#method.try_insert_batch_if_new "method bevy::prelude::World::try_insert_batch_if_new").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2661-2665)

#### pub fn [try\_insert\_batch](#method.try_insert_batch)<I, B>( &mut self, batch: I, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryInsertBatchError](../ecs/world/error/struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError")\>

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"): [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = ([Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), B)>, B: [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <B as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../ecs/bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

For a given batch of ([`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")) pairs, adds the `Bundle` of components to each `Entity`. This is faster than doing equivalent operations one-by-one.

A batch can be any type that implements [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") containing `(Entity, Bundle)` tuples, such as a [`Vec<(Entity, Bundle)>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec") or an array `[(Entity, Bundle); N]`.

This will overwrite any previous values of components shared by the `Bundle`. See [`World::try_insert_batch_if_new`](../prelude/struct.World.html#method.try_insert_batch_if_new "method bevy::prelude::World::try_insert_batch_if_new") to keep the old values instead.

Returns a [`TryInsertBatchError`](../ecs/world/error/struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError") if any of the provided entities do not exist.

For the panicking version, see [`World::insert_batch`](../prelude/struct.World.html#method.insert_batch "method bevy::prelude::World::insert_batch").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2683-2687)

#### pub fn [try\_insert\_batch\_if\_new](#method.try_insert_batch_if_new)<I, B>( &mut self, batch: I, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryInsertBatchError](../ecs/world/error/struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError")\>

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"): [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = ([Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), B)>, B: [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <B as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../ecs/bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

For a given batch of ([`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")) pairs, adds the `Bundle` of components to each `Entity` without overwriting. This is faster than doing equivalent operations one-by-one.

A batch can be any type that implements [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") containing `(Entity, Bundle)` tuples, such as a [`Vec<(Entity, Bundle)>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec") or an array `[(Entity, Bundle); N]`.

This is the same as [`World::try_insert_batch`](../prelude/struct.World.html#method.try_insert_batch "method bevy::prelude::World::try_insert_batch"), but in case of duplicate components it will leave the old values instead of replacing them with new ones.

Returns a [`TryInsertBatchError`](../ecs/world/error/struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError") if any of the provided entities do not exist.

For the panicking version, see [`World::insert_batch_if_new`](../prelude/struct.World.html#method.insert_batch_if_new "method bevy::prelude::World::insert_batch_if_new").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2851)

#### pub fn [resource\_scope](#method.resource_scope)<R, U>( &mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [World](../prelude/struct.World.html "struct bevy::prelude::World"), [Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>) -> U, ) -> U

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Temporarily removes the requested resource from this [`World`](../prelude/struct.World.html "struct bevy::prelude::World"), runs custom user code, then re-adds the resource before returning.

This enables safe simultaneous mutable access to both a resource and the rest of the [`World`](../prelude/struct.World.html "struct bevy::prelude::World"). For more complex access patterns, consider using [`SystemState`](../ecs/system/struct.SystemState.html "struct bevy::ecs::system::SystemState").

##### Panics

Panics if the resource does not exist. Use [`try_resource_scope`](../prelude/struct.World.html#method.try_resource_scope "method bevy::prelude::World::try_resource_scope") instead if you want to handle this case.

##### Example

```rust
use bevy_ecs::prelude::*;
#[derive(Resource)]
struct A(u32);
#[derive(Component)]
struct B(u32);
let mut world = World::new();
world.insert_resource(A(1));
let entity = world.spawn(B(1)).id();

world.resource_scope(|world, mut a: Mut<A>| {
    let b = world.get_mut::<B>(entity).unwrap();
    a.0 += b.0;
});
assert_eq!(world.get_resource::<A>().unwrap().0, 2);
```

##### Note

If the world’s resource metadata is cleared within the scope, such as by calling [`World::clear_resources`](../prelude/struct.World.html#method.clear_resources "method bevy::prelude::World::clear_resources") or [`World::clear_all`](../prelude/struct.World.html#method.clear_all "method bevy::prelude::World::clear_all"), the resource will _not_ be re-inserted at the end of the scope.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#2869-2872)

#### pub fn [try\_resource\_scope](#method.try_resource_scope)<R, U>( &mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [World](../prelude/struct.World.html "struct bevy::prelude::World"), [Mut](../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, R>) -> U, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<U>

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Temporarily removes the requested resource from this [`World`](../prelude/struct.World.html "struct bevy::prelude::World") if it exists, runs custom user code, then re-adds the resource before returning. Returns `None` if the resource does not exist in this [`World`](../prelude/struct.World.html "struct bevy::prelude::World").

This enables safe simultaneous mutable access to both a resource and the rest of the [`World`](../prelude/struct.World.html "struct bevy::prelude::World"). For more complex access patterns, consider using [`SystemState`](../ecs/system/struct.SystemState.html "struct bevy::ecs::system::SystemState").

See also [`resource_scope`](../prelude/struct.World.html#method.resource_scope "method bevy::prelude::World::resource_scope").

##### Note

If the world’s resource metadata is cleared within the scope, such as by calling [`World::clear_resources`](../prelude/struct.World.html#method.clear_resources "method bevy::prelude::World::clear_resources") or [`World::clear_all`](../prelude/struct.World.html#method.clear_all "method bevy::prelude::World::clear_all"), the resource will _not_ be re-inserted at the end of the scope.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3015)

#### pub fn [write\_message](#method.write_message)<M>(&mut self, message: M) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MessageId](../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>>

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message"),

Writes a [`Message`](../prelude/trait.Message.html "trait bevy::prelude::Message"). This method returns the [`MessageId`](../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId") of the written `message`, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the `message` could not be written.

##### [Examples found in repository](#scraped-examples-19)[?](../../scrape-examples-help.html)

examples/app/render\_recovery.rs ([line 133](../../src/render_recovery/render_recovery.rs.html#133))

```rust
106fn input(
107    input: Res<ButtonInput<Key>>,
108    mut error: ResMut<RenderError>,
109    mut handler: ResMut<RenderErrorHandler>,
110) {
111    *error = RenderError::None;
112    if input.just_pressed(Key::Character("o".into())) {
113        *error = RenderError::OutOfMemory;
114    }
115    if input.just_pressed(Key::Character("v".into())) {
116        *error = RenderError::Validation;
117    }
118    if input.just_pressed(Key::Character("d".into())) {
119        *error = RenderError::DeviceLost;
120    }
121    if input.just_pressed(Key::Character("l".into())) {
122        *error = RenderError::Loop;
123    }
124
125    if input.just_pressed(Key::Character("1".into())) {
126        *handler = RenderErrorHandler(|_, _, _| RenderErrorPolicy::Ignore);
127    }
128    if input.just_pressed(Key::Character("2".into())) {
129        *handler = RenderErrorHandler(|error, _, _| panic!("Rendering error {error:?}"));
130    }
131    if input.just_pressed(Key::Character("3".into())) {
132        *handler = RenderErrorHandler(|_, main_world, _| {
133            main_world.write_message(AppExit::error());
134            RenderErrorPolicy::StopRendering
135        });
136    }
137    if input.just_pressed(Key::Character("4".into())) {
138        *handler = RenderErrorHandler(|_, _, _| RenderErrorPolicy::StopRendering);
139    }
140    if input.just_pressed(Key::Character("5".into())) {
141        *handler = RenderErrorHandler(|_, _, _| RenderErrorPolicy::Recover(default()));
142    }
143}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3023)

#### pub fn [write\_message\_default](#method.write_message_default)<M>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MessageId](../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>>

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Writes the default value of the [`Message`](../prelude/trait.Message.html "trait bevy::prelude::Message") of type `M`. This method returns the [`MessageId`](../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId") of the written message, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the `event` could not be written.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3031-3034)

#### pub fn [write\_message\_batch](#method.write_message_batch)<M>( &mut self, messages: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = M>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[WriteBatchIds](../ecs/message/struct.WriteBatchIds.html "struct bevy::ecs::message::WriteBatchIds")<M>>

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message"),

Writes a batch of [`Message`](../prelude/trait.Message.html "trait bevy::prelude::Message")s from an iterator. This method returns the [IDs](../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId") of the written `messages`, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the `events` could not be written.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3054-3059)

#### pub unsafe fn [insert\_resource\_by\_id](#method.insert_resource_by_id)( &mut self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), value: [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>, caller: [MaybeLocation](../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation"), )

Inserts a new resource with the given `value`. Will replace the value if it already existed.

**You should prefer to use the typed API [`World::insert_resource`](../prelude/struct.World.html#method.insert_resource "method bevy::prelude::World::insert_resource") where possible and only use this in cases where the actual types are not known at compile time.**

##### Safety

The value referenced by `value` must be valid for the given [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") of this world.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3090-3095)

#### pub unsafe fn [insert\_non\_send\_by\_id](#method.insert_non_send_by_id)( &mut self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), value: [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>, caller: [MaybeLocation](../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation"), )

Inserts new `!Send` data with the given `value`. Will replace the value if it already existed.

**You should prefer to use the typed API [`World::insert_non_send`](../prelude/struct.World.html#method.insert_non_send "method bevy::prelude::World::insert_non_send") where possible and only use this in cases where the actual types are not known at compile time.**

##### Panics

If a value is already present, this function will panic if not called from the same thread that the original value was inserted from.

##### Safety

The value referenced by `value` must be valid for the given [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") of this world.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3150)

#### pub fn [flush](#method.flush)(&mut self)

Flushes queued entities and commands.

Queued entities will be spawned, and then commands will be applied.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3162)

#### pub fn [increment\_change\_tick](#method.increment_change_tick)(&mut self) -> [Tick](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Increments the world’s current change tick and returns the old value.

If you need to call this method, but do not have `&mut` access to the world, consider using [`as_unsafe_world_cell_readonly`](../prelude/struct.World.html#method.as_unsafe_world_cell_readonly "method bevy::prelude::World::as_unsafe_world_cell_readonly") to obtain an [`UnsafeWorldCell`](../ecs/world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") and calling [`increment_change_tick`](../ecs/world/unsafe_world_cell/struct.UnsafeWorldCell.html#method.increment_change_tick "method bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell::increment_change_tick") on that. Note that this _can_ be done in safe code, despite the name of the type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3174)

#### pub fn [read\_change\_tick](#method.read_change_tick)(&self) -> [Tick](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Reads the current change tick of this world.

If you have exclusive (`&mut`) access to the world, consider using [`change_tick()`](../prelude/struct.World.html#method.change_tick "method bevy::prelude::World::change_tick"), which is more efficient since it does not require atomic synchronization.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3184)

#### pub fn [change\_tick](#method.change_tick)(&mut self) -> [Tick](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Reads the current change tick of this world.

This does the same thing as [`read_change_tick()`](../prelude/struct.World.html#method.read_change_tick "method bevy::prelude::World::read_change_tick"), only this method is more efficient since it does not require atomic synchronization.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3196)

#### pub fn [last\_change\_tick](#method.last_change_tick)(&self) -> [Tick](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

When called from within an exclusive system (a [`System`](../prelude/trait.System.html "trait bevy::prelude::System") that takes `&mut World` as its first parameter), this method returns the [`Tick`](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick") indicating the last time the exclusive system was run.

Otherwise, this returns the `Tick` indicating the last time that [`World::clear_trackers`](../prelude/struct.World.html#method.clear_trackers "method bevy::prelude::World::clear_trackers") was called.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3290-3294)

#### pub fn [last\_change\_tick\_scope](#method.last_change_tick_scope)<T>( &mut self, last\_change\_tick: [Tick](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [World](../prelude/struct.World.html "struct bevy::prelude::World")) -> T, ) -> T

Sets [`World::last_change_tick()`](../prelude/struct.World.html#method.last_change_tick "method bevy::prelude::World::last_change_tick") to the specified value during a scope. When the scope terminates, it will return to its old value.

This is useful if you need a region of code to be able to react to earlier changes made in the same system.

##### Examples

```rust
// This function runs an update loop repeatedly, allowing each iteration of the loop
// to react to changes made in the previous loop iteration.
fn update_loop(
    world: &mut World,
    mut update_fn: impl FnMut(&mut World) -> std::ops::ControlFlow<()>,
) {
    let mut last_change_tick = world.last_change_tick();

    // Repeatedly run the update function until it requests a break.
    loop {
        let control_flow = world.last_change_tick_scope(last_change_tick, |world| {
            // Increment the change tick so we can detect changes from the previous update.
            last_change_tick = world.change_tick();
            world.increment_change_tick();

            // Update once.
            update_fn(world)
        });

        // End the loop when the closure returns `ControlFlow::Break`.
        if control_flow.is_break() {
            break;
        }
    }
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3326)

#### pub fn [check\_change\_ticks](#method.check_change_ticks)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[CheckChangeTicks](../ecs/change_detection/struct.CheckChangeTicks.html "struct bevy::ecs::change_detection::CheckChangeTicks")\>

Iterates all component change ticks and clamps any older than [`MAX_CHANGE_AGE`](../ecs/change_detection/constant.MAX_CHANGE_AGE.html "constant bevy::ecs::change_detection::MAX_CHANGE_AGE"). This also triggers [`CheckChangeTicks`](../ecs/change_detection/struct.CheckChangeTicks.html "struct bevy::ecs::change_detection::CheckChangeTicks") observers and returns the same event here.

Calling this method prevents [`Tick`](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")s overflowing and thus prevents false positives when comparing them.

**Note:** Does nothing and returns `None` if the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") counter has not been incremented at least [`CHECK_TICK_THRESHOLD`](../ecs/change_detection/constant.CHECK_TICK_THRESHOLD.html "constant bevy::ecs::change_detection::CHECK_TICK_THRESHOLD") times since the previous pass.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3362)

#### pub fn [clear\_all](#method.clear_all)(&mut self)

Clears all entities, resources, and non-send data. This invalidates all [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") and resource fetches such as [`Res`](../prelude/struct.Res.html "struct bevy::prelude::Res"), [`ResMut`](../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3375)

#### pub fn [clear\_entities](#method.clear_entities)(&mut self)

Despawns all entities in this [`World`](../prelude/struct.World.html "struct bevy::prelude::World").

**Note:** This includes all resources, as they are stored as components. Any resource fetch to this [`World`](../prelude/struct.World.html "struct bevy::prelude::World") will fail unless they are re-initialized, including engine-internal resources that are only initialized on app/world construction.

This can easily cause systems expecting certain resources to immediately start panicking. Use with caution.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3390)

#### pub fn [clear\_resources](#method.clear_resources)(&mut self)

Clears all resources in this [`World`](../prelude/struct.World.html "struct bevy::prelude::World").

**Note:** Any resource fetch to this [`World`](../prelude/struct.World.html "struct bevy::prelude::World") will fail unless they are re-initialized, including engine-internal resources that are only initialized on app/world construction.

This can easily cause systems expecting certain resources to immediately start panicking. Use with caution.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3398)

#### pub fn [clear\_non\_send](#method.clear_non_send)(&mut self)

Clears all non-send data in this [`World`](../prelude/struct.World.html "struct bevy::prelude::World").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3408)

#### pub fn [register\_bundle](#method.register_bundle)<B>(&mut self) -> &[BundleInfo](../ecs/bundle/struct.BundleInfo.html "struct bevy::ecs::bundle::BundleInfo")

where B: [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Registers all of the components in the given [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") and returns both the component ids and the bundle id.

This is largely equivalent to calling [`register_component`](../prelude/struct.World.html#method.register_component "method bevy::prelude::World::register_component") on each component in the bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3450)

#### pub fn [register\_dynamic\_bundle](#method.register_dynamic_bundle)( &mut self, component\_ids: &\[[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\], ) -> &[BundleInfo](../ecs/bundle/struct.BundleInfo.html "struct bevy::ecs::bundle::BundleInfo")

Registers the given [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s as a dynamic bundle and returns both the required component ids and the bundle id.

Note that the components need to be registered first, this function only creates a bundle combining them. Components can be registered with [`World::register_component`](../prelude/struct.World.html#method.register_component "method bevy::prelude::World::register_component")/[`_with_descriptor`](../prelude/struct.World.html#method.register_component_with_descriptor "method bevy::prelude::World::register_component_with_descriptor").

**You should prefer to use the typed API [`World::register_bundle`](../prelude/struct.World.html#method.register_bundle "method bevy::prelude::World::register_bundle") where possible and only use this in cases where not all of the actual types are known at compile time.**

##### Panics

This function will panic if any of the provided component ids do not belong to a component known to this [`World`](../prelude/struct.World.html "struct bevy::prelude::World").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3461)

#### pub fn [fallback\_error\_handler](#method.fallback_error_handler)(&self) -> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError"), [ErrorContext](../ecs/error/enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext"))

Convenience method for accessing the world’s fallback error handler, which can be overwritten with [`FallbackErrorHandler`](../ecs/error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3477)

#### pub fn [get\_resource\_by\_id](#method.get_resource_by_id)(&self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ptr](../ecs/ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>>

Gets a pointer to the resource with the id [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") if it exists. The returned pointer must not be used to modify the resource, and must not be dereferenced after the immutable borrow of the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") ends.

**You should prefer to use the typed API [`World::get_resource`](../prelude/struct.World.html#method.get_resource "method bevy::prelude::World::get_resource") where possible and only use this in cases where the actual types are not known at compile time.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3494)

#### pub fn [get\_resource\_mut\_by\_id](#method.get_resource_mut_by_id)( &mut self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MutUntyped](../ecs/change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'\_>>

Gets a pointer to the resource with the id [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") if it exists and is mutable. The returned pointer may be used to modify the resource, as long as the mutable borrow of the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") is still valid.

**You should prefer to use the typed API [`World::get_resource_mut`](../prelude/struct.World.html#method.get_resource_mut "method bevy::prelude::World::get_resource_mut") where possible and only use this in cases where the actual types are not known at compile time.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3591)

#### pub fn [iter\_resources](#method.iter_resources)(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = (&[ComponentInfo](../ecs/component/struct.ComponentInfo.html "struct bevy::ecs::component::ComponentInfo"), [Ptr](../ecs/ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>)>

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3667)

#### pub fn [iter\_resources\_mut](#method.iter_resources_mut)( &mut self, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = (&[ComponentInfo](../ecs/component/struct.ComponentInfo.html "struct bevy::ecs::component::ComponentInfo"), [MutUntyped](../ecs/change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'\_>)>

Mutably iterates over all resources in the world.

The returned iterator provides lifetimed, but type-unsafe pointers. Actually reading from or writing to the contents of each resource will require the use of unsafe code.

##### Example

```rust
// In this example, `A` and `B` are resources. We deliberately do not use the
// `bevy_reflect` crate here to showcase the low-level `MutUntyped` usage. You should
// probably use something like `ReflectFromPtr` in a real-world scenario.

// Create the hash map that will store the mutator closures for each resource type
let mut mutators: HashMap<TypeId, Box<dyn Fn(&mut MutUntyped<'_>)>> = HashMap::default();

// Add mutator closure for `A`
mutators.insert(TypeId::of::<A>(), Box::new(|mut_untyped| {
    // Note: `MutUntyped::as_mut()` automatically marks the resource as changed
    // for ECS change detection, and gives us a `PtrMut` we can use to mutate the resource.
    // SAFETY: We assert ptr is the same type of A with TypeId of A
    let a = unsafe { &mut mut_untyped.as_mut().deref_mut::<A>() };
    // ... mutate `a` here
}));

// Add mutator closure for `B`
mutators.insert(TypeId::of::<B>(), Box::new(|mut_untyped| {
    // SAFETY: We assert ptr is the same type of B with TypeId of B
    let b = unsafe { &mut mut_untyped.as_mut().deref_mut::<B>() };
    // ... mutate `b` here
}));

// Iterate all resources, in order to run the mutator closures for each matching resource type
for (info, mut mut_untyped) in world.iter_resources_mut() {
    let Some(type_id) = info.type_id() else {
       // It's possible for resources to not have a `TypeId` (e.g. non-Rust resources
       // dynamically inserted via a scripting language) in which case we can't match them.
       continue;
    };

    let Some(mutator) = mutators.get(&type_id) else {
       // No mutator closure for this resource type, skip it.
       continue;
    };

    // Run the mutator closure for the resource
    mutator(&mut mut_untyped);
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3704)

#### pub fn [get\_non\_send\_by\_id](#method.get_non_send_by_id)(&self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ptr](../ecs/ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>>

Gets a pointer to `!Send` data with the id [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") if it exists. The returned pointer must not be used to modify the resource, and must not be dereferenced after the immutable borrow of the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") ends.

**You should prefer to use the typed API [`World::get_non_send`](../prelude/struct.World.html#method.get_non_send "method bevy::prelude::World::get_non_send") where possible and only use this in cases where the actual types are not known at compile time.**

##### Panics

This function will panic if it isn’t called from the same thread that the data was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3724)

#### pub fn [get\_non\_send\_mut\_by\_id](#method.get_non_send_mut_by_id)( &mut self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MutUntyped](../ecs/change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'\_>>

Gets mutable access to `!Send` data with the id [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") if it exists. The returned pointer may be used to modify the data, as long as the mutable borrow of the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") is still valid.

**You should prefer to use the typed API [`World::get_non_send_mut`](../prelude/struct.World.html#method.get_non_send_mut "method bevy::prelude::World::get_non_send_mut") where possible and only use this in cases where the actual types are not known at compile time.**

##### Panics

This function will panic if it isn’t called from the same thread that the data was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3740)

#### pub fn [remove\_resource\_by\_id](#method.remove_resource_by_id)(&mut self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Removes the resource of a given type, if it exists. Returns `true` if the resource is successfully removed and `false` if the entity does not exist.

**You should prefer to use the typed API [`World::remove_resource`](../prelude/struct.World.html#method.remove_resource "method bevy::prelude::World::remove_resource") where possible and only use this in cases where the actual types are not known at compile time.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3759)

#### pub fn [remove\_non\_send\_by\_id](#method.remove_non_send_by_id)(&mut self, component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

Removes the non-send data of a given type, if it exists. Otherwise returns `None`.

**You should prefer to use the typed API [`World::remove_non_send`](../prelude/struct.World.html#method.remove_non_send "method bevy::prelude::World::remove_non_send") where possible and only use this in cases where the actual types are not known at compile time.**

##### Panics

This function will panic if it isn’t called from the same thread that the data was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3776)

#### pub fn [get\_by\_id](#method.get_by_id)( &self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ptr](../ecs/ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>>

Retrieves an immutable untyped reference to the given `entity`’s [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"). Returns `None` if the `entity` does not have a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type.

**You should prefer to use the typed API [`World::get_mut`](../prelude/struct.World.html#method.get_mut "method bevy::prelude::World::get_mut") where possible and only use this in cases where the actual types are not known at compile time.**

##### Panics

This function will panic if it isn’t called from the same thread that the resource was inserted from.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3786-3790)

#### pub fn [get\_mut\_by\_id](#method.get_mut_by_id)( &mut self, entity: [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), component\_id: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MutUntyped](../ecs/change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'\_>>

Retrieves a mutable untyped reference to the given `entity`’s [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given [`ComponentId`](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"). Returns `None` if the `entity` does not have a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") of the given type.

**You should prefer to use the typed API [`World::get_mut`](../prelude/struct.World.html#method.get_mut "method bevy::prelude::World::get_mut") where possible and only use this in cases where the actual types are not known at compile time.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3812)

#### pub fn [add\_schedule](#method.add_schedule)(&mut self, schedule: [Schedule](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"))

Adds the specified [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") to the world. If a schedule already exists with the same [label](../prelude/struct.Schedule.html#method.label "method bevy::prelude::Schedule::label"), it will be replaced.

The schedule can later be run by calling [`.run_schedule(label)`](../prelude/struct.World.html#method.run_schedule "method bevy::prelude::World::run_schedule") or by directly accessing the [`Schedules`](../prelude/struct.Schedules.html "struct bevy::prelude::Schedules") resource.

The `Schedules` resource will be initialized if it does not already exist.

An alternative to this is to call [`Schedules::add_systems()`](../prelude/struct.Schedules.html#method.add_systems "method bevy::prelude::Schedules::add_systems") with some [`ScheduleLabel`](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") and let the schedule for that label be created if it does not already exist.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3828-3832)

#### pub fn [try\_schedule\_scope](#method.try_schedule_scope)<R>( &mut self, label: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [World](../prelude/struct.World.html "struct bevy::prelude::World"), &mut [Schedule](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")) -> R, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<R, [TryRunScheduleError](../ecs/world/error/struct.TryRunScheduleError.html "struct bevy::ecs::world::error::TryRunScheduleError")\>

Temporarily removes the schedule associated with `label` from the world, runs user code, and finally re-adds the schedule. This returns a [`TryRunScheduleError`](../ecs/world/error/struct.TryRunScheduleError.html "struct bevy::ecs::world::error::TryRunScheduleError") if there is no schedule associated with `label`.

The [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") is fetched from the [`Schedules`](../prelude/struct.Schedules.html "struct bevy::prelude::Schedules") resource of the world by its label, and system state is cached.

For simple cases where you just need to call the schedule once, consider using [`World::try_run_schedule`](../prelude/struct.World.html#method.try_run_schedule "method bevy::prelude::World::try_run_schedule") instead. For other use cases, see the example on [`World::schedule_scope`](../prelude/struct.World.html#method.schedule_scope "method bevy::prelude::World::schedule_scope").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3888-3892)

#### pub fn [schedule\_scope](#method.schedule_scope)<R>( &mut self, label: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [World](../prelude/struct.World.html "struct bevy::prelude::World"), &mut [Schedule](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")) -> R, ) -> R

Temporarily removes the schedule associated with `label` from the world, runs user code, and finally re-adds the schedule.

The [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") is fetched from the [`Schedules`](../prelude/struct.Schedules.html "struct bevy::prelude::Schedules") resource of the world by its label, and system state is cached.

##### Examples

```rust
// Run the schedule five times.
world.schedule_scope(MySchedule, |world, schedule| {
    for _ in 0..5 {
        schedule.run(world);
    }
});
```

For simple cases where you just need to call the schedule once, consider using [`World::run_schedule`](../prelude/struct.World.html#method.run_schedule "method bevy::prelude::World::run_schedule") instead.

##### Panics

If the requested schedule does not exist.

##### [Examples found in repository](#scraped-examples-20)[?](../../scrape-examples-help.html)

tests/ecs/ambiguity\_detection.rs ([lines 103-106](../../src/ambiguity_detection/ambiguity_detection.rs.html#103-106))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3904-3907)

#### pub fn [try\_run\_schedule](#method.try_run_schedule)( &mut self, label: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryRunScheduleError](../ecs/world/error/struct.TryRunScheduleError.html "struct bevy::ecs::world::error::TryRunScheduleError")\>

Attempts to run the [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") associated with the `label` a single time, and returns a [`TryRunScheduleError`](../ecs/world/error/struct.TryRunScheduleError.html "struct bevy::ecs::world::error::TryRunScheduleError") if the schedule does not exist.

The [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") is fetched from the [`Schedules`](../prelude/struct.Schedules.html "struct bevy::prelude::Schedules") resource of the world by its label, and system state is cached.

For simple testing use cases, call [`Schedule::run(&mut world)`](../prelude/struct.Schedule.html#method.run "method bevy::prelude::Schedule::run") instead.

##### [Examples found in repository](#scraped-examples-21)[?](../../scrape-examples-help.html)

examples/state/custom\_transitions.rs ([line 113](../../src/custom_transitions/custom_transitions.rs.html#113))

```rust
95    fn run_reenter<S: States>(transition: In<Option<StateTransitionEvent<S>>>, world: &mut World) {
96        // We return early if no transition event happened.
97        let Some(transition) = transition.0 else {
98            return;
99        };
100
101        // If we wanted to ignore identity transitions,
102        // we'd compare `exited` and `entered` here,
103        // and return if they were the same.
104
105        // We check if we actually entered a state.
106        // A [`None`] would indicate that the state was removed from the world.
107        // This only happens in the case of [`SubStates`] and [`ComputedStates`].
108        let Some(entered) = transition.entered else {
109            return;
110        };
111
112        // If all conditions are valid, we run our custom schedule.
113        let _ = world.try_run_schedule(OnReenter(entered));
114
115        // If you want to overwrite the default `OnEnter` behavior to act like re-enter,
116        // you can do so by running the `OnEnter` schedule here. Note that you don't want
117        // to run `OnEnter` when the default behavior does so.
118        // ```
119        // if transition.entered != transition.exited {
120        //     return;
121        // }
122        // let _ = world.try_run_schedule(OnReenter(entered));
123        // ```
124    }
125
126    /// Custom schedule that will behave like [`OnExit`], but run on identity transitions.
127    #[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
128    pub struct OnReexit<S: States>(pub S);
129
130    fn run_reexit<S: States>(transition: In<Option<StateTransitionEvent<S>>>, world: &mut World) {
131        let Some(transition) = transition.0 else {
132            return;
133        };
134        let Some(exited) = transition.exited else {
135            return;
136        };
137
138        let _ = world.try_run_schedule(OnReexit(exited));
139    }
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3922)

#### pub fn [run\_schedule](#method.run_schedule)(&mut self, label: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"))

Runs the [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") associated with the `label` a single time.

The [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") is fetched from the [`Schedules`](../prelude/struct.Schedules.html "struct bevy::prelude::Schedules") resource of the world by its label, and system state is cached.

For simple testing use cases, call [`Schedule::run(&mut world)`](../prelude/struct.Schedule.html#method.run "method bevy::prelude::Schedule::run") instead. This avoids the need to create a unique [`ScheduleLabel`](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel").

##### Panics

If the requested schedule does not exist.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3927)

#### pub fn [allow\_ambiguous\_component](#method.allow_ambiguous_component)<T>(&mut self)

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Ignore system order ambiguities caused by conflicts on [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")s of type `T`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3934)

#### pub fn [allow\_ambiguous\_resource](#method.allow_ambiguous_resource)<T>(&mut self)

where T: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Ignore system order ambiguities caused by conflicts on [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource")s of type `T`.

## Trait Implementations

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

### impl [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

where [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

#### const [STORAGE\_TYPE](../prelude/trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::SparseSet

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

#### type [Mutability](../prelude/trait.Component.html#associatedtype.Mutability) = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](../prelude/trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](../prelude/trait.Component.html "trait bevy::prelude::Component"). [Read more](../prelude/trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

#### fn [register\_required\_components](../prelude/trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](../prelude/trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

#### fn [clone\_behavior](../prelude/trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](../prelude/trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

#### fn [relationship\_accessor](../prelude/trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")\>>

Returns [`ComponentRelationshipAccessor`](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor") required for working with relationships in dynamic contexts. [Read more](../prelude/trait.Component.html#method.relationship_accessor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#524)

#### fn [on\_add](../prelude/trait.Component.html#method.on_add)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_add` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#529)

#### fn [on\_insert](../prelude/trait.Component.html#method.on_insert)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_insert` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#534)

#### fn [on\_discard](../prelude/trait.Component.html#method.on_discard)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_discard` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#539)

#### fn [on\_remove](../prelude/trait.Component.html#method.on_remove)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_remove` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#544)

#### fn [on\_despawn](../prelude/trait.Component.html#method.on_despawn)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_despawn` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#649)

#### fn [map\_entities](../prelude/trait.Component.html#method.map_entities)<E>(\_this: &mut Self, \_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Maps the entities on this component using the given [`EntityMapper`](../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"). This is used to remap entities in contexts like scenes and entity cloning. When deriving [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component"), this is populated by annotating fields containing entities with `#[entities]` [Read more](../prelude/trait.Component.html#method.map_entities)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [World](../prelude/struct.World.html "struct bevy::prelude::World")

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

### impl [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

where [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

## Auto Trait Implementations

### impl ![Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#16)

### impl<C> [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#17-19)

#### fn [component\_ids](../prelude/trait.Bundle.html#tymethod.component_ids)( components: &mut [ComponentsRegistrator](../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\> + use<C>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#23)

#### fn [get\_component\_ids](../prelude/trait.Bundle.html#tymethod.get_component_ids)( components: &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#31-35)

#### unsafe fn [from\_components](../ecs/bundle/trait.BundleFromComponents.html#tymethod.from_components)<T, F>(ctx: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), func: [&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> C

where F: for<'a> [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'a>, C: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#43)

### impl<C> [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for C

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

An operation on the entity that happens _after_ inserting this bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#46-49)

#### unsafe fn [get\_components](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)( ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, C>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), ) -> <C as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect")

Moves the components out of the bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#54)

#### unsafe fn [apply\_effect](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)( \_ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<C>>, \_entity: &mut [EntityWorldMut](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Applies the after-effects of spawning this bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

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

### impl<T> [Instrument](../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

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

### impl<T> [IntoResult](../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

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

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

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

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","SpawnBatchIter<'\_, <I as IntoIterator>::IntoIter>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../ecs/world/struct.SpawnBatchIter.html\\" title=\\"struct bevy::ecs::world::SpawnBatchIter\\">SpawnBatchIter</a>&lt;'\_, I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../ecs/world/struct.SpawnBatchIter.html\\" title=\\"struct bevy::ecs::world::SpawnBatchIter\\">SpawnBatchIter</a>&lt;'\_, I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>: <a class=\\"trait\\" href=\\"../prelude/trait.Bundle.html\\" title=\\"trait bevy::prelude::Bundle\\">Bundle</a>,\\n &lt;&lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a> as <a class=\\"trait\\" href=\\"../ecs/bundle/trait.DynamicBundle.html\\" title=\\"trait bevy::ecs::bundle::DynamicBundle\\">DynamicBundle</a>&gt;::<a class=\\"associatedtype\\" href=\\"../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect\\" title=\\"type bevy::ecs::bundle::DynamicBundle::Effect\\">Effect</a>: <a class=\\"trait\\" href=\\"../ecs/bundle/trait.NoBundleEffect.html\\" title=\\"trait bevy::ecs::bundle::NoBundleEffect\\">NoBundleEffect</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"struct\\" href=\\"../prelude/struct.Entity.html\\" title=\\"struct bevy::prelude::Entity\\">Entity</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}