[bevy](../../index.html)::[ecs](../index.html)::[world](index.html)

# Struct FilteredEntityMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#365)

```rust
pub struct FilteredEntityMut<'w, 's> { /* private fields */ }
```

Provides mutable access to a single entity and some of its components defined by the contained [`Access`](../query/struct.Access.html "struct bevy::ecs::query::Access").

To define the access when used as a [`QueryData`](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), use a [`QueryBuilder`](../../prelude/struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder") or [`QueryParamBuilder`](../system/struct.QueryParamBuilder.html "struct bevy::ecs::system::QueryParamBuilder"). The `FilteredEntityMut` must be the entire `QueryData`, and not nested inside a tuple with other data.

```rust
// This gives the `FilteredEntityMut` access to `&mut A`.
let mut query = QueryBuilder::<FilteredEntityMut>::new(&mut world)
    .data::<&mut A>()
    .build();

let mut filtered_entity: FilteredEntityMut = query.single_mut(&mut world).unwrap();
let component: Mut<A> = filtered_entity.get_mut().unwrap();
```

Also see [`UnsafeFilteredEntityMut`](struct.UnsafeFilteredEntityMut.html "struct bevy::ecs::world::UnsafeFilteredEntityMut") for a way to bypass borrow-checker restrictions.

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#370)

### impl<'w, 's> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#386)

#### pub fn [reborrow](#method.reborrow)(&mut self) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, 's>

Returns a new instance with a shorter lifetime. This is useful if you have `&mut FilteredEntityMut`, but you need `FilteredEntityMut`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#396)

#### pub fn [into\_readonly](#method.into_readonly)(self) -> [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'w, 's>

Consumes `self` and returns read-only access to all of the entity’s components, with the world `'w` lifetime.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#405)

#### pub fn [as\_readonly](#method.as_readonly)(&self) -> [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'\_, 's>

Gets read-only access to all of the entity’s components.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#421)

#### pub fn [try\_into\_all](#method.try_into_all)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>, [TryFromFilteredError](enum.TryFromFilteredError.html "enum bevy::ecs::world::TryFromFilteredError")\>

Consumes self and returns mutable access to the entity and _all_ of its components, with the world `'w` lifetime. Returns an error if the access does not include read and write access to all components.

##### Errors

*   [`TryFromFilteredError::MissingReadAllAccess`](enum.TryFromFilteredError.html#variant.MissingReadAllAccess "variant bevy::ecs::world::TryFromFilteredError::MissingReadAllAccess") - if the access does not include read access to all components.
*   [`TryFromFilteredError::MissingWriteAllAccess`](enum.TryFromFilteredError.html#variant.MissingWriteAllAccess "variant bevy::ecs::world::TryFromFilteredError::MissingWriteAllAccess") - if the access does not include write access to all components.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#434)

#### pub fn [as\_unsafe\_entity\_cell](#method.as_unsafe_entity_cell)(&mut self) -> [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>

Get access to the underlying [`UnsafeEntityCell`](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#441)

#### pub fn [id](#method.id)(&self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Returns the [ID](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") of the current entity.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/dynamic.rs ([line 212](../../../src/dynamic/dynamic.rs.html#212))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#447)

#### pub fn [location](#method.location)(&self) -> [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

Gets metadata indicating the location where the current entity is stored.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#453)

#### pub fn [archetype](#method.archetype)(&self) -> &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

Returns the archetype that the current entity belongs to.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#459)

#### pub fn [access](#method.access)(&self) -> &[Access](../query/struct.Access.html "struct bevy::ecs::query::Access")

Returns a reference to the underlying [`Access`](../query/struct.Access.html "struct bevy::ecs::query::Access").

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/ecs/dynamic.rs ([line 181](../../../src/dynamic/dynamic.rs.html#181))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#471)

#### pub fn [contains](#method.contains)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Returns `true` if the current entity has a component of type `T`. Otherwise, this returns `false`.

###### Notes

If you do not know the concrete type of a component, consider using [`Self::contains_id`](struct.FilteredEntityMut.html#method.contains_id "method bevy::ecs::world::FilteredEntityMut::contains_id") or [`Self::contains_type_id`](struct.FilteredEntityMut.html#method.contains_type_id "method bevy::ecs::world::FilteredEntityMut::contains_type_id").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#484)

#### pub fn [contains\_id](#method.contains_id)(&self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the current entity has a component identified by `component_id`. Otherwise, this returns false.

###### Notes

*   If you know the concrete type of the component, you should prefer [`Self::contains`](struct.FilteredEntityMut.html#method.contains "method bevy::ecs::world::FilteredEntityMut::contains").
*   If you know the component’s [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") but not its [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), consider using [`Self::contains_type_id`](struct.FilteredEntityMut.html#method.contains_type_id "method bevy::ecs::world::FilteredEntityMut::contains_type_id").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#496)

#### pub fn [contains\_type\_id](#method.contains_type_id)(&self, type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the current entity has a component with the type identified by `type_id`. Otherwise, this returns false.

###### Notes

*   If you know the concrete type of the component, you should prefer [`Self::contains`](struct.FilteredEntityMut.html#method.contains "method bevy::ecs::world::FilteredEntityMut::contains").
*   If you have a [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") instead of a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), consider using [`Self::contains_id`](struct.FilteredEntityMut.html#method.contains_id "method bevy::ecs::world::FilteredEntityMut::contains_id").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#503)

#### pub fn [get](#method.get)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Gets access to the component of type `T` for the current entity. Returns `None` if the entity does not have a component of type `T`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#512)

#### pub fn [get\_ref](#method.get_ref)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Gets access to the component of type `T` for the current entity, including change detection information as a [`Ref`](../../prelude/struct.Ref.html "struct bevy::prelude::Ref").

Returns `None` if the entity does not have a component of type `T`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#520)

#### pub fn [get\_mut](#method.get_mut)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets mutable access to the component of type `T` for the current entity. Returns `None` if the entity does not have a component of type `T` or if the access does not include write access to `T`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#569-571)

#### pub unsafe fn [get\_mut\_unchecked](#method.get_mut_unchecked)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Gets mutable access to the component of type `T` for the current entity. Returns `None` if the entity does not have a component of type `T` or if the access does not include write access to `T`.

This only requires `&self`, and so may be used to get mutable access to multiple components.

##### Example

```rust
#[derive(Component)]
struct X(usize);
#[derive(Component)]
struct Y(usize);

let mut entity = world.spawn((X(0), Y(0))).into_mutable();

// This gives the `FilteredEntityMut` access to `&mut X` and `&mut Y`.
let mut query = QueryBuilder::<FilteredEntityMut>::new(&mut world)
    .data::<(&mut X, &mut Y)>()
    .build();

let mut filtered_entity: FilteredEntityMut = query.single_mut(&mut world).unwrap();

// Get mutable access to two components at once
// SAFETY: We don't take any other references to `X` from this entity
let mut x = unsafe { filtered_entity.get_mut_unchecked::<X>() }.unwrap();
// SAFETY: We don't take any other references to `Y` from this entity
let mut y = unsafe { filtered_entity.get_mut_unchecked::<Y>() }.unwrap();
*x = X(1);
*y = Y(1);
```

##### Safety

No other references to the same component may exist at the same time as the returned reference.

##### See also

*   [`get_mut`](struct.FilteredEntityMut.html#method.get_mut "method bevy::ecs::world::FilteredEntityMut::get_mut") for the safe version.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#589)

#### pub fn [into\_mut](#method.into_mut)<T>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

Consumes self and gets mutable access to the component of type `T` with the world `'w` lifetime for the current entity. Returns `None` if the entity does not have a component of type `T`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#604)

#### pub unsafe fn [into\_mut\_assume\_mutable](#method.into_mut_assume_mutable)<T>(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Consumes self and gets mutable access to the component of type `T` with the world `'w` lifetime for the current entity. Returns `None` if the entity does not have a component of type `T`.

##### Safety

*   `T` must be a mutable component

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#622)

#### pub fn [get\_change\_ticks](#method.get_change_ticks)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentTicks](../change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")\>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

Retrieves the change ticks for the given component. This can be useful for implementing change detection in custom runtimes.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#633)

#### pub fn [get\_change\_ticks\_by\_id](#method.get_change_ticks_by_id)( &self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentTicks](../change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")\>

Retrieves the change ticks for the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"). This can be useful for implementing change detection in custom runtimes.

**You should prefer to use the typed API [`Self::get_change_ticks`](struct.FilteredEntityMut.html#method.get_change_ticks "method bevy::ecs::world::FilteredEntityMut::get_change_ticks") where possible and only use this in cases where the actual component types are not known at compile time.**

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#646)

#### pub fn [get\_by\_id](#method.get_by_id)(&self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'\_>>

Gets the component of the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") from the entity.

**You should prefer to use the typed API [`Self::get`](struct.FilteredEntityMut.html#method.get "method bevy::ecs::world::FilteredEntityMut::get") where possible and only use this in cases where the actual component types are not known at compile time.**

Unlike [`FilteredEntityMut::get`](struct.FilteredEntityMut.html#method.get "method bevy::ecs::world::FilteredEntityMut::get"), this returns a raw pointer to the component, which is only valid while the [`FilteredEntityMut`](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut") is alive.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_components.rs ([line 50](../../../src/many_components/many_components.rs.html#50))

```rust
38fn base_system(access_components: In<Vec<ComponentId>>, mut query: Query<FilteredEntityMut>) {
39    #[cfg(feature = "trace")]
40    let _span = tracing::info_span!("base_system", components = ?access_components.0, count = query.iter().len()).entered();
41
42    for mut filtered_entity in &mut query {
43        // We calculate Faulhaber's formula mod 256 with n = value and p = exponent.
44        // See https://en.wikipedia.org/wiki/Faulhaber%27s_formula
45        // The time is takes to compute this depends on the number of entities and the values in
46        // each entity. This is to ensure that each system takes a different amount of time.
47        let mut total: Wrapping<u8> = Wrapping(0);
48        for (exponent, component_id) in (1_u32..).zip(access_components.0.iter()) {
49            // find the value of the component
50            let ptr = filtered_entity.get_by_id(*component_id).unwrap();
51
52            // SAFETY: All components have a u8 layout
53            let value: u8 = unsafe { *ptr.deref::<u8>() };
54
55            for i in 0..=value {
56                let mut product = Wrapping(1);
57                for _ in 1..=exponent {
58                    product *= Wrapping(i);
59                }
60                total += product;
61            }
62        }
63
64        // we assign this value to all the components we can write to
65        for component_id in &access_components.0 {
66            if let Some(ptr) = filtered_entity.get_mut_by_id(*component_id) {
67                // SAFETY: All components have a u8 layout
68                unsafe {
69                    let mut value = ptr.with_type::<u8>();
70                    *value = total.0;
71                }
72            }
73        }
74    }
75}
```

Hide additional examples

examples/ecs/dynamic.rs ([line 186](../../../src/dynamic/dynamic.rs.html#186))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#659)

#### pub fn [get\_mut\_by\_id](#method.get_mut_by_id)( &mut self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'\_>>

Gets a [`MutUntyped`](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped") of the component of the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") from the entity.

**You should prefer to use the typed API [`Self::get_mut`](struct.FilteredEntityMut.html#method.get_mut "method bevy::ecs::world::FilteredEntityMut::get_mut") where possible and only use this in cases where the actual component types are not known at compile time.**

Unlike [`FilteredEntityMut::get_mut`](struct.FilteredEntityMut.html#method.get_mut "method bevy::ecs::world::FilteredEntityMut::get_mut"), this returns a raw pointer to the component, which is only valid while the [`FilteredEntityMut`](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut") is alive.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_components.rs ([line 66](../../../src/many_components/many_components.rs.html#66))

```rust
38fn base_system(access_components: In<Vec<ComponentId>>, mut query: Query<FilteredEntityMut>) {
39    #[cfg(feature = "trace")]
40    let _span = tracing::info_span!("base_system", components = ?access_components.0, count = query.iter().len()).entered();
41
42    for mut filtered_entity in &mut query {
43        // We calculate Faulhaber's formula mod 256 with n = value and p = exponent.
44        // See https://en.wikipedia.org/wiki/Faulhaber%27s_formula
45        // The time is takes to compute this depends on the number of entities and the values in
46        // each entity. This is to ensure that each system takes a different amount of time.
47        let mut total: Wrapping<u8> = Wrapping(0);
48        for (exponent, component_id) in (1_u32..).zip(access_components.0.iter()) {
49            // find the value of the component
50            let ptr = filtered_entity.get_by_id(*component_id).unwrap();
51
52            // SAFETY: All components have a u8 layout
53            let value: u8 = unsafe { *ptr.deref::<u8>() };
54
55            for i in 0..=value {
56                let mut product = Wrapping(1);
57                for _ in 1..=exponent {
58                    product *= Wrapping(i);
59                }
60                total += product;
61            }
62        }
63
64        // we assign this value to all the components we can write to
65        for component_id in &access_components.0 {
66            if let Some(ptr) = filtered_entity.get_mut_by_id(*component_id) {
67                // SAFETY: All components have a u8 layout
68                unsafe {
69                    let mut value = ptr.with_type::<u8>();
70                    *value = total.0;
71                }
72            }
73        }
74    }
75}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#684-687)

#### pub unsafe fn [get\_mut\_by\_id\_unchecked](#method.get_mut_by_id_unchecked)( &self, component\_id: [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'\_>>

Gets a [`MutUntyped`](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped") of the component of the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") from the entity.

**You should prefer to use the typed API [`Self::get_mut`](struct.FilteredEntityMut.html#method.get_mut "method bevy::ecs::world::FilteredEntityMut::get_mut") where possible and only use this in cases where the actual component types are not known at compile time.**

Unlike [`FilteredEntityMut::get_mut`](struct.FilteredEntityMut.html#method.get_mut "method bevy::ecs::world::FilteredEntityMut::get_mut"), this returns a raw pointer to the component, which is only valid while the [`FilteredEntityMut`](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut") is alive.

This only requires `&self`, and so may be used to get mutable access to multiple components.

##### Safety

No other references to the same component may exist at the same time as the returned reference.

##### See also

*   [`get_mut_by_id`](struct.FilteredEntityMut.html#method.get_mut_by_id "method bevy::ecs::world::FilteredEntityMut::get_mut_by_id") for the safe version.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#697)

#### pub fn [spawned\_by](#method.spawned_by)(&self) -> [MaybeLocation](../change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")

Returns the source code location from which this entity has last been spawned.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#702)

#### pub fn [spawn\_tick](#method.spawn_tick)(&self) -> [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Returns the [`Tick`](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick") at which this entity has been spawned.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1402)

### impl [ArchetypeQueryData](../query/trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#791)

### impl [ContainsEntity](../../prelude/trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#792)

#### fn [entity](../../prelude/trait.ContainsEntity.html#tymethod.entity)(&self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

Returns the contained entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#798)

### impl [EntityEquivalent](../entity/trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#769)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/entity_mut.rs.html#829)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a mut [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>> for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/entity_mut.rs.html#831)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: &'a mut [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2340)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>> for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2342)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: &'a mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#756)

### impl<'w, 's> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'w [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, 's>> for [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#758)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: &'w [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, 's>) -> [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'w, 's>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/except.rs.html#480)

### impl<'w, 's, B> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'w mut [EntityMutExcept](struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, 's, B>> for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/except.rs.html#482)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: &'w mut [EntityMutExcept](struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, 's, B>) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#742)

### impl<'w, 's> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'w mut [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, 's>> for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#744)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: &'w mut [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, 's>) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/entity_mut.rs.html#822)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>> for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/entity_mut.rs.html#824)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/except.rs.html#473)

### impl<'w, 's, B> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[EntityMutExcept](struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'w, 's, B>> for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/except.rs.html#475)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: [EntityMutExcept](struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'w, 's, B>) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2333)

### impl<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'a>> for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/world_mut.rs.html#2335)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'a>) -> [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'static>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#749)

### impl<'w, 's> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>> for [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#751)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(entity: [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>) -> [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'w, 's>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#785)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#786)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1397)

### impl [IterQueryData](../query/trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#779)

### impl [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#780)

#### fn [cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)(&self, other: &[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1034-1036)

#### fn [max](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)(self, other: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1073-1075)

#### fn [min](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)(self, other: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1099-1101)

#### fn [clamp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)(self, min: Self, max: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#763)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#764)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#771)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#774)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

[`FilteredEntityMut`](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")’s comparison trait implementations match the underlying [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), and cannot discern between different worlds.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1410)

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1428)

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1446)

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1464)

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1344)

### impl<'a, 'b> [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'b>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1345)

#### const [IS\_READ\_ONLY](../query/trait.QueryData.html#associatedconstant.IS_READ_ONLY): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

True if this query is read-only and may not perform mutable access.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1346)

#### const [IS\_ARCHETYPAL](../query/trait.QueryData.html#associatedconstant.IS_ARCHETYPAL): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

Returns true if (and only if) this query data relies strictly on archetypes to limit which entities are accessed by the Query. [Read more](../query/trait.QueryData.html#associatedconstant.IS_ARCHETYPAL)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1347)

#### type [ReadOnly](../query/trait.QueryData.html#associatedtype.ReadOnly) = [FilteredEntityRef](struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'a, 'b>

The read-only variant of this [`QueryData`](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), which satisfies the [`ReadOnlyQueryData`](../query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") trait.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1348)

#### type [Item](../query/trait.QueryData.html#associatedtype.Item)<'w, 's> = [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

The item returned by this [`WorldQuery`](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") This will be the data retrieved by the query, and is visible to the end user when calling e.g. `Query<Self>::get`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1350-1352)

#### fn [shrink](../query/trait.QueryData.html#tymethod.shrink)<'wlong, 'wshort, 's>( item: <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'b> as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wlong, 's>, ) -> <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'b> as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'wshort, 's>

where 'wlong: 'wshort,

This function manually implements subtyping for the query items.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1357-1361)

#### fn [provide\_extra\_access](../query/trait.QueryData.html#method.provide_extra_access)( state: &mut <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'b> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), access: &mut [Access](../query/struct.Access.html "struct bevy::ecs::query::Access"), available\_access: &[Access](../query/struct.Access.html "struct bevy::ecs::query::Access"), )

Offers additional access above what we requested in `update_component_access`. Implementations may add additional access that is a subset of `available_access` and does not conflict with anything in `access`, and must update `access` to include that access. [Read more](../query/trait.QueryData.html#method.provide_extra_access)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1374-1379)

#### unsafe fn [fetch](../query/trait.QueryData.html#tymethod.fetch)<'w, 's>( access: &'s <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'b> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), fetch: &mut <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'b> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](../query/trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), \_table\_row: [TableRow](../storage/struct.TableRow.html "struct bevy::ecs::storage::TableRow"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'b> as [QueryData](../query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 's>>

Fetch [`Self::Item`](../query/trait.QueryData.html#associatedtype.Item "associated type bevy::ecs::query::QueryData::Item") for either the given `entity` in the current [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), or for the given `entity` in the current [`Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"). This must always be called after [`WorldQuery::set_table`](../query/trait.WorldQuery.html#tymethod.set_table "associated function bevy::ecs::query::WorldQuery::set_table") with a `table_row` in the range of the current [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table") or after [`WorldQuery::set_archetype`](../query/trait.WorldQuery.html#tymethod.set_archetype "associated function bevy::ecs::query::WorldQuery::set_archetype") with an `entity` in the current archetype. Accesses components registered in [`WorldQuery::update_component_access`](../query/trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access"). [Read more](../query/trait.QueryData.html#tymethod.fetch)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1391)

#### fn [iter\_access](../query/trait.QueryData.html#tymethod.iter_access)( state: &<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, 'b> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [EcsAccessType](../query/enum.EcsAccessType.html "enum bevy::ecs::query::EcsAccessType")<'\_>>

Returns an iterator over the access needed by [`QueryData::fetch`](../query/trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch"). Access conflicts are usually checked in [`WorldQuery::update_component_access`](../query/trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access"), but in certain cases this method can be useful to implement a way of checking for access conflicts in a non-allocating way.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1400)

### impl [SingleEntityQueryData](../query/trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#716)

### impl<'a> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&'a [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>> for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#717)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromFilteredError](enum.TryFromFilteredError.html "enum bevy::ecs::world::TryFromFilteredError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#720)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( entity: &'a [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>, <[EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&'a [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#733)

### impl<'a> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&'a mut [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>> for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#734)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromFilteredError](enum.TryFromFilteredError.html "enum bevy::ecs::world::TryFromFilteredError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#737)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( entity: &'a mut [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>, <[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<&'a mut [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#707)

### impl<'a> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, '\_>> for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#708)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromFilteredError](enum.TryFromFilteredError.html "enum bevy::ecs::world::TryFromFilteredError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#711)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( entity: [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a>, <[EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'a> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, '\_>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#725)

### impl<'a> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, '\_>> for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#726)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromFilteredError](enum.TryFromFilteredError.html "enum bevy::ecs::world::TryFromFilteredError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#728)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( entity: [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a>, <[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'a> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'a, '\_>>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1279)

### impl [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1287)

#### const [IS\_DENSE](../query/trait.WorldQuery.html#associatedconstant.IS_DENSE): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

Returns true if (and only if) every table of every archetype matched by this fetch contains all of the matched components. [Read more](../query/trait.WorldQuery.html#associatedconstant.IS_DENSE)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1280)

#### type [Fetch](../query/trait.WorldQuery.html#associatedtype.Fetch)<'w> = EntityFetch<'w>

Per archetype/table state retrieved by this [`WorldQuery`](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") to compute [`Self::Item`](../query/trait.QueryData.html#associatedtype.Item "associated type bevy::ecs::query::QueryData::Item") for each entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1281)

#### type [State](../query/trait.WorldQuery.html#associatedtype.State) = [Access](../query/struct.Access.html "struct bevy::ecs::query::Access")

State used to construct a [`Self::Fetch`](../query/trait.WorldQuery.html#associatedtype.Fetch "associated type bevy::ecs::query::WorldQuery::Fetch"). This will be cached inside [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState"), so it is best to move as much data / computation here as possible to reduce the cost of constructing [`Self::Fetch`](../query/trait.WorldQuery.html#associatedtype.Fetch "associated type bevy::ecs::query::WorldQuery::Fetch").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1283)

#### fn [shrink\_fetch](../query/trait.WorldQuery.html#tymethod.shrink_fetch)<'wlong, 'wshort>( fetch: <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](../query/trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wlong>, ) -> <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](../query/trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'wshort>

where 'wlong: 'wshort,

This function manually implements subtyping for the query fetches.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1289-1294)

#### unsafe fn [init\_fetch](../query/trait.WorldQuery.html#tymethod.init_fetch)<'w, 's>( world: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, \_state: &'s <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), last\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), this\_run: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](../query/trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>

Creates a new instance of [`Self::Fetch`](../query/trait.WorldQuery.html#associatedtype.Fetch "associated type bevy::ecs::query::WorldQuery::Fetch"), by combining data from the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") with the cached [`Self::State`](../query/trait.WorldQuery.html#associatedtype.State "associated type bevy::ecs::query::WorldQuery::State"). Readonly accesses resources registered in [`WorldQuery::update_component_access`](../query/trait.WorldQuery.html#tymethod.update_component_access "associated function bevy::ecs::query::WorldQuery::update_component_access"). [Read more](../query/trait.WorldQuery.html#tymethod.init_fetch)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1303-1308)

#### unsafe fn [set\_archetype](../query/trait.WorldQuery.html#tymethod.set_archetype)<'w, 's>( \_fetch: &mut <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](../query/trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, \_state: &'s <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_: &'w [Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"), \_table: &[Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

Adjusts internal state to account for the next [`Archetype`](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype"). This will always be called on archetypes that match this [`WorldQuery`](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery"). [Read more](../query/trait.WorldQuery.html#tymethod.set_archetype)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1312-1316)

#### unsafe fn [set\_table](../query/trait.WorldQuery.html#tymethod.set_table)<'w, 's>( \_fetch: &mut <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[Fetch](../query/trait.WorldQuery.html#associatedtype.Fetch "type bevy::ecs::query::WorldQuery::Fetch")<'w>, \_state: &'s <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_: &'w [Table](../storage/struct.Table.html "struct bevy::ecs::storage::Table"), )

Adjusts internal state to account for the next [`Table`](../storage/struct.Table.html "struct bevy::ecs::storage::Table"). This will always be called on tables that match this [`WorldQuery`](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery"). [Read more](../query/trait.WorldQuery.html#tymethod.set_table)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1319)

#### fn [update\_component\_access](../query/trait.WorldQuery.html#tymethod.update_component_access)( state: &<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), filtered\_access: &mut [FilteredAccess](../query/struct.FilteredAccess.html "struct bevy::ecs::query::FilteredAccess"), )

Adds any component accesses to the current entity used by this [`WorldQuery`](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") to `access`. [Read more](../query/trait.WorldQuery.html#tymethod.update_component_access)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1327)

#### fn [init\_state](../query/trait.WorldQuery.html#tymethod.init_state)( \_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> <[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")

Creates and initializes a [`State`](../query/trait.WorldQuery.html#associatedtype.State "associated type bevy::ecs::query::WorldQuery::State") for this [`WorldQuery`](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1331)

#### fn [get\_state](../query/trait.WorldQuery.html#tymethod.get_state)( \_components: &[Components](../component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State")\>

Attempts to initialize a [`State`](../query/trait.WorldQuery.html#associatedtype.State "associated type bevy::ecs::query::WorldQuery::State") for this [`WorldQuery`](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") type using read-only access to [`Components`](../component/struct.Components.html "struct bevy::ecs::component::Components").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1335-1338)

#### fn [matches\_component\_set](../query/trait.WorldQuery.html#tymethod.matches_component_set)( \_state: &<[FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_> as [WorldQuery](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery")\>::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_set\_contains\_id: &impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this query matches a set of components. Otherwise, returns `false`. [Read more](../query/trait.WorldQuery.html#tymethod.matches_component_set)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#131-136)

#### fn [init\_nested\_access](../query/trait.WorldQuery.html#method.init_nested_access)( \_state: &Self::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_system\_name: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, \_component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), \_world: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, )

Adds any component accesses to other entities used by this [`WorldQuery`](../query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery"). [Read more](../query/trait.WorldQuery.html#method.init_nested_access)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/world_query.rs.html#158)

#### fn [update\_archetypes](../query/trait.WorldQuery.html#method.update_archetypes)(\_state: &mut Self::[State](../query/trait.WorldQuery.html#associatedtype.State "type bevy::ecs::query::WorldQuery::State"), \_world: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>)

Called when the query state is updating its archetype cache. This can be used by nested queries to update their internal archetype caches.

## Auto Trait Implementations

### impl<'w, 's> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

### impl<'w, 's> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

### impl<'w, 's> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

### impl<'w, 's> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

### impl<'w, 's> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

### impl<'w, 's> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

### impl<'w, 's> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [FilteredEntityMut](struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'w, 's>

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

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#104-107)

### impl<Q, K> [Comparable](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html "trait equivalent::Comparable")<K> for Q

where Q: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#110)

#### fn [compare](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html#tymethod.compare)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

Compare self to `key` and return their ordering.

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#47-49)

### impl<T> [DynHash](../label/trait.DynHash.html "trait bevy::ecs::label::DynHash") for T

where T: [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#51)

#### fn [dyn\_hash](../label/trait.DynHash.html#tymethod.dyn_hash)(&self, state: &mut dyn [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"))

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher").

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Compare self to `key` and return `true` if they are equal.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#151-154)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#156)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

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