[bevy](../index.html)::[prelude](index.html)

# Struct Name 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#50)

```rust
pub struct Name(pub HashedStr);
```

Component used to identify an entity. Stores a hash for faster comparisons.

The hash is eagerly re-computed upon each update to the name.

[`Name`](struct.Name.html "struct bevy::prelude::Name") should not be treated as a globally unique identifier for entities, as multiple entities can have the same name. [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") should be used instead as the default unique identifier.

## Tuple Fields

`0: [HashedStr](../ecs/name/struct.HashedStr.html "struct bevy::ecs::name::HashedStr")`

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#75)

### impl [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#79)

#### pub fn [new](#method.new)(name: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>) -> [Name](struct.Name.html "struct bevy::prelude::Name")

Creates a new [`Name`](struct.Name.html "struct bevy::prelude::Name") from any string-like type.

The internal hash will be computed immediately.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/movement/physics\_in\_fixed\_timestep.rs ([line 178](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#178))

```rust
175fn spawn_player(mut commands: Commands) {
176    commands.spawn((Camera3d::default(), CameraSensitivity::default()));
177    commands.spawn((
178        Name::new("Player"),
179        Transform::from_scale(Vec3::splat(0.3)),
180        AccumulatedInput::default(),
181        Velocity::default(),
182        PhysicalTranslation::default(),
183        PreviousPhysicalTranslation::default(),
184    ));
185}
186
187/// Spawn a field of floating spheres to fly around in
188fn spawn_environment(
189    mut commands: Commands,
190    mut meshes: ResMut<Assets<Mesh>>,
191    mut materials: ResMut<Assets<StandardMaterial>>,
192) {
193    let sphere_material = materials.add(Color::from(tailwind::SKY_200));
194    let sphere_mesh = meshes.add(Sphere::new(0.3));
195    let spheres_in_x = 6;
196    let spheres_in_y = 4;
197    let spheres_in_z = 10;
198    let distance = 3.0;
199    for x in 0..spheres_in_x {
200        for y in 0..spheres_in_y {
201            for z in 0..spheres_in_z {
202                let translation = Vec3::new(
203                    x as f32 * distance - (spheres_in_x as f32 - 1.0) * distance / 2.0,
204                    y as f32 * distance - (spheres_in_y as f32 - 1.0) * distance / 2.0,
205                    z as f32 * distance - (spheres_in_z as f32 - 1.0) * distance / 2.0,
206                );
207                commands.spawn((
208                    Name::new("Sphere"),
209                    Transform::from_translation(translation),
210                    Mesh3d(sphere_mesh.clone()),
211                    MeshMaterial3d(sphere_material.clone()),
212                ));
213            }
214        }
215    }
216
217    commands.spawn((
218        DirectionalLight::default(),
219        Transform::default().looking_to(Vec3::new(-1.0, -3.0, 0.5), Vec3::Y),
220    ));
221}
```

Hide additional examples

examples/ecs/observer\_propagation.rs ([line 29](../../src/observer_propagation/observer_propagation.rs.html#29))

```rust
27fn setup(mut commands: Commands) {
28    commands
29        .spawn((Name::new("Goblin"), HitPoints(50)))
30        .observe(take_damage)
31        .with_children(|parent| {
32            parent
33                .spawn((Name::new("Helmet"), Armor(5)))
34                .observe(block_attack);
35            parent
36                .spawn((Name::new("Socks"), Armor(10)))
37                .observe(block_attack);
38            parent
39                .spawn((Name::new("Shirt"), Armor(15)))
40                .observe(block_attack);
41        });
42}
```

examples/ui/text/text\_input.rs ([line 126](../../src/text_input/text_input.rs.html#126))

```rust
118fn build_input_text(commands: &mut Commands, is_left: bool, font_size: f32) -> Entity {
119    commands
120        .spawn((
121            Node {
122                border: px(2).all(),
123                ..Default::default()
124            },
125            BorderColor::from(Color::from(SLATE_300)),
126            Name::new(if is_left { "Left" } else { "Right" }),
127            EditableText {
128                visible_width: Some(10.),
129                allow_newlines: false,
130                ..Default::default()
131            },
132            TextLayout::no_wrap(),
133            TextFont {
134                font_size: FontSize::Px(font_size),
135                ..default()
136            },
137            TextCursorStyle::default(),
138            TabIndex(if is_left { 0 } else { 1 }),
139            BackgroundColor(DARK_GREY.into()),
140        ))
141        .id()
142}
```

examples/camera/camera\_orbit.rs ([line 52](../../src/camera_orbit/camera_orbit.rs.html#52))

```rust
46fn setup(
47    mut commands: Commands,
48    mut meshes: ResMut<Assets<Mesh>>,
49    mut materials: ResMut<Assets<StandardMaterial>>,
50) {
51    commands.spawn((
52        Name::new("Camera"),
53        Camera3d::default(),
54        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
55    ));
56
57    commands.spawn((
58        Name::new("Plane"),
59        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
60        MeshMaterial3d(materials.add(StandardMaterial {
61            base_color: Color::srgb(0.3, 0.5, 0.3),
62            // Turning off culling keeps the plane visible when viewed from beneath.
63            cull_mode: None,
64            ..default()
65        })),
66    ));
67
68    commands.spawn((
69        Name::new("Cube"),
70        Mesh3d(meshes.add(Cuboid::default())),
71        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
72        Transform::from_xyz(1.5, 0.51, 1.5),
73    ));
74
75    commands.spawn((
76        Name::new("Light"),
77        PointLight::default(),
78        Transform::from_xyz(3.0, 8.0, 5.0),
79    ));
80}
81
82fn instructions(mut commands: Commands) {
83    commands.spawn((
84        Name::new("Instructions"),
85        Text::new(
86            "Mouse up or down: pitch\n\
87            Mouse left or right: yaw\n\
88            Mouse buttons: roll",
89        ),
90        Node {
91            position_type: PositionType::Absolute,
92            top: px(12),
93            left: px(12),
94            ..default()
95        },
96    ));
97}
```

examples/ecs/entity\_disabling.rs ([line 112](../../src/entity_disabling/entity_disabling.rs.html#112))

```rust
104fn setup_scene(
105    mut commands: Commands,
106    mut meshes: ResMut<Assets<Mesh>>,
107    mut materials: ResMut<Assets<ColorMaterial>>,
108) {
109    commands.spawn(Camera2d);
110
111    let named_shapes = [
112        (Name::new("Annulus"), meshes.add(Annulus::new(25.0, 50.0))),
113        (
114            Name::new("Bestagon"),
115            meshes.add(RegularPolygon::new(50.0, 6)),
116        ),
117        (Name::new("Rhombus"), meshes.add(Rhombus::new(75.0, 100.0))),
118    ];
119    let num_shapes = named_shapes.len();
120
121    for (i, (name, shape)) in named_shapes.into_iter().enumerate() {
122        // Distribute colors evenly across the rainbow.
123        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
124
125        commands.spawn((
126            name,
127            DisableOnClick,
128            Mesh2d(shape),
129            MeshMaterial2d(materials.add(color)),
130            Transform::from_xyz(
131                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
132                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
133                0.0,
134                0.0,
135            ),
136        ));
137    }
138}
```

examples/usage/cooldown.rs ([line 94](../../src/cooldown/cooldown.rs.html#94))

```rust
84fn build_ability(
85    food: FoodItem,
86    texture: Handle<Image>,
87    layout: Handle<TextureAtlasLayout>,
88) -> impl Bundle {
89    let FoodItem {
90        name,
91        cooldown,
92        index,
93    } = food;
94    let name = Name::new(name);
95
96    // Every food item is a button with a child node.
97    // The child node's height will be animated to be at 100% at the beginning
98    // of a cooldown, effectively graying out the whole button, and then getting smaller over time.
99    (
100        Node {
101            width: px(80),
102            height: px(80),
103            flex_direction: FlexDirection::ColumnReverse,
104            ..default()
105        },
106        BackgroundColor(tailwind::SLATE_400.into()),
107        Button,
108        ImageNode::from_atlas_image(texture, TextureAtlas { layout, index }),
109        Cooldown(Timer::from_seconds(cooldown, TimerMode::Once)),
110        name,
111        children![(
112            Node {
113                width: percent(100),
114                height: percent(0),
115                ..default()
116            },
117            BackgroundColor(tailwind::SLATE_50.with_alpha(0.5).into()),
118        )],
119    )
120}
```

Additional examples can be found in:  

*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#593)
*   [examples/usage/context\_menu.rs](../../src/context_menu/context_menu.rs.html#90)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#64)
*   [examples/ecs/relationships.rs](../../src/relationships/relationships.rs.html#52)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#75)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#51)
*   [examples/scene/world\_serialization.rs](../../src/world_serialization/world_serialization.rs.html#183)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#228)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#394)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#29)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#97)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#108)
*   [examples/animation/animated\_ui.rs](../../src/animated_ui/animated_ui.rs.html#42)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#239)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#46)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#87)

#### pub fn [set](#method.set)(&mut self, name: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

Sets the entity’s name.

The internal hash will be re-computed.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#96)

#### pub fn [mutate](#method.mutate)(&mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [String](struct.String.html "struct bevy::prelude::String")))

Updates the name of the entity in place.

This will allocate a new string if the name was previously created from a borrow.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#111)

#### pub fn [as\_str](#method.as_str)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Gets the name of the entity as a `&str`.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/ecs/relationships.rs ([line 96](../../src/relationships/relationships.rs.html#96))

```rust
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
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#116)

#### pub fn [pre\_hash](#method.pre_hash)(&self) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

Get the precomputed hash of this names string, useful for raw entry operations on [`PreHashMap`](../utils/type.PreHashMap.html "type bevy::utils::PreHashMap")

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#154)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the length of `self`.

This length is in bytes, not [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s or graphemes. In other words, it might not be what a human considers the length of the string.

##### Examples

```rust
let len = "foo".len();
assert_eq!(3, len);

assert_eq!("ƒoo".len(), 4); // fancy f!
assert_eq!("ƒoo".chars().count(), 3);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#174)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if `self` has a length of zero bytes.

##### Examples

```rust
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

1.9.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#374)

#### pub fn [is\_char\_boundary](#method.is_char_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks that `index`\-th byte is the first byte in a UTF-8 code point sequence or the end of the string.

The start and end of the string (when `index == self.len()`) are considered to be boundaries.

Returns `false` if `index` is greater than `self.len()`.

##### Examples

```rust
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0));
// start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));

// second byte of `ö`
assert!(!s.is_char_boundary(2));

// third byte of `老`
assert!(!s.is_char_boundary(8));
```

1.91.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#423)

#### pub fn [floor\_char\_boundary](#method.floor_char_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Finds the closest `x` not exceeding `index` where [`is_char_boundary(x)`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary "method str::is_char_boundary") is `true`.

This method can help you truncate a string so that it’s still valid UTF-8, but doesn’t exceed a given number of bytes. Note that this is done purely at the character level and can still visually split graphemes, even though the underlying characters aren’t split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only includes 🧑 (person) instead.

##### Examples

```rust
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.floor_char_boundary(13);
assert_eq!(closest, 10);
assert_eq!(&s[..closest], "❤️🧡");
```

1.91.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#480)

#### pub fn [ceil\_char\_boundary](#method.ceil_char_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Finds the closest `x` not below `index` where [`is_char_boundary(x)`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary "method str::is_char_boundary") is `true`.

If `index` is greater than the length of the string, this returns the length of the string.

This method is the natural complement to [`floor_char_boundary`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.floor_char_boundary "method str::floor_char_boundary"). See that method for more details.

##### Examples

```rust
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.ceil_char_boundary(13);
assert_eq!(closest, 14);
assert_eq!(&s[..closest], "❤️🧡💛");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#513)

#### pub fn [as\_bytes](#method.as_bytes)(&self) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

Converts a string slice to a byte slice. To convert the byte slice back into a string slice, use the [`from_utf8`](https://doc.rust-lang.org/nightly/core/str/converts/fn.from_utf8.html "fn core::str::converts::from_utf8") function.

##### Examples

```rust
let bytes = "bors".as_bytes();
assert_eq!(b"bors", bytes);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#589)

#### pub fn [as\_ptr](#method.as_ptr)(&self) -> [\*const](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

Converts a string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8"). This pointer will be pointing to the first byte of the string slice.

The caller must ensure that the returned pointer is never written to. If you need to mutate the contents of the string slice, use [`as_mut_ptr`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.as_mut_ptr "method str::as_mut_ptr").

##### Examples

```rust
let s = "Hello";
let ptr = s.as_ptr();
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#634)

#### pub fn [get](#method.get)<I>(&self, i: I) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")\>

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns a subslice of `str`.

This is the non-panicking alternative to indexing the `str`. Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") whenever equivalent indexing operation would panic.

##### Examples

```rust
let v = String::from("🗻∈🌏");

assert_eq!(Some("🗻"), v.get(0..4));

// indices not on UTF-8 sequence boundaries
assert!(v.get(1..).is_none());
assert!(v.get(..8).is_none());

// out of bounds
assert!(v.get(..42).is_none());
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#699)

#### pub unsafe fn [get\_unchecked](#method.get_unchecked)<I>(&self, i: I) -> &<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns an unchecked subslice of `str`.

This is the unchecked alternative to indexing the `str`.

##### Safety

Callers of this function are responsible that these preconditions are satisfied:

*   The starting index must not exceed the ending index;
*   Indexes must be within bounds of the original slice;
*   Indexes must lie on UTF-8 sequence boundaries.

Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the `str` type.

##### Examples

```rust
let v = "🗻∈🌏";
unsafe {
    assert_eq!("🗻", v.get_unchecked(0..4));
    assert_eq!("∈", v.get_unchecked(4..7));
    assert_eq!("🌏", v.get_unchecked(7..11));
}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#785)

#### pub unsafe fn [slice\_unchecked](#method.slice_unchecked)(&self, begin: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), end: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.29.0:

use `get_unchecked(begin..end)` instead

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "primitive str") and [`Index`](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index").

This new slice goes from `begin` to `end`, including `begin` but excluding `end`.

To get a mutable string slice instead, see the [`slice_mut_unchecked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.slice_mut_unchecked "method str::slice_mut_unchecked") method.

##### Safety

Callers of this function are responsible that three preconditions are satisfied:

*   `begin` must not exceed `end`.
*   `begin` and `end` must be byte positions within the string slice.
*   `begin` and `end` must lie on UTF-8 sequence boundaries.

##### Examples

```rust
let s = "Löwe 老虎 Léopard";

unsafe {
    assert_eq!("Löwe 老虎 Léopard", s.slice_unchecked(0, 21));
}

let s = "Hello, world!";

unsafe {
    assert_eq!("world", s.slice_unchecked(7, 12));
}
```

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#859)

#### pub fn [split\_at](#method.split_at)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Divides one string slice into two at an index.

The argument, `mid`, should be a byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut "method str::split_at_mut") method.

##### Panics

Panics if `mid` is not on a UTF-8 code point boundary, or if it is past the end of the last code point of the string slice. For a non-panicking alternative see [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_checked "method str::split_at_checked").

##### Examples

```rust
let s = "Per Martin-Löf";

let (first, last) = s.split_at(3);

assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#940)

#### pub fn [split\_at\_checked](#method.split_at_checked)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

Divides one string slice into two at an index.

The argument, `mid`, should be a valid byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point. The method returns `None` if that’s not the case.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut_checked "method str::split_at_mut_checked") method.

##### Examples

```rust
let s = "Per Martin-Löf";

let (first, last) = s.split_at_checked(3).unwrap();
assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);

assert_eq!(None, s.split_at_checked(13));  // Inside “ö”
assert_eq!(None, s.split_at_checked(16));  // Beyond the string length
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1078)

#### pub fn [chars](#method.chars)(&self) -> [Chars](https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html "struct core::str::iter::Chars")<'\_> [ⓘ](#)

Returns an iterator over the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s of a string slice.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"). This method returns such an iterator.

It’s important to remember that [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") represents a Unicode Scalar Value, and might not match your idea of what a ‘character’ is. Iteration over grapheme clusters may be what you actually want. This functionality is not provided by Rust’s standard library, check crates.io instead.

##### Examples

Basic usage:

```rust
let word = "goodbye";

let count = word.chars().count();
assert_eq!(7, count);

let mut chars = word.chars();

assert_eq!(Some('g'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('d'), chars.next());
assert_eq!(Some('b'), chars.next());
assert_eq!(Some('y'), chars.next());
assert_eq!(Some('e'), chars.next());

assert_eq!(None, chars.next());
```

Remember, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s might not match your intuition about characters:

```rust
let y = "y̆";

let mut chars = y.chars();

assert_eq!(Some('y'), chars.next()); // not 'y̆'
assert_eq!(Some('\u{0306}'), chars.next());

assert_eq!(None, chars.next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1135)

#### pub fn [char\_indices](#method.char_indices)(&self) -> [CharIndices](https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html "struct core::str::iter::CharIndices")<'\_> [ⓘ](#)

Returns an iterator over the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s of a string slice, and their positions.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"). This method returns an iterator of both these [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, as well as their byte positions.

The iterator yields tuples. The position is first, the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") is second.

##### Examples

Basic usage:

```rust
let word = "goodbye";

let count = word.char_indices().count();
assert_eq!(7, count);

let mut char_indices = word.char_indices();

assert_eq!(Some((0, 'g')), char_indices.next());
assert_eq!(Some((1, 'o')), char_indices.next());
assert_eq!(Some((2, 'o')), char_indices.next());
assert_eq!(Some((3, 'd')), char_indices.next());
assert_eq!(Some((4, 'b')), char_indices.next());
assert_eq!(Some((5, 'y')), char_indices.next());
assert_eq!(Some((6, 'e')), char_indices.next());

assert_eq!(None, char_indices.next());
```

Remember, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s might not match your intuition about characters:

```rust
let yes = "y̆es";

let mut char_indices = yes.char_indices();

assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
assert_eq!(Some((1, '\u{0306}')), char_indices.next());

// note the 3 here - the previous character took up two bytes
assert_eq!(Some((3, 'e')), char_indices.next());
assert_eq!(Some((4, 's')), char_indices.next());

assert_eq!(None, char_indices.next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1158)

#### pub fn [bytes](#method.bytes)(&self) -> [Bytes](https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html "struct core::str::iter::Bytes")<'\_> [ⓘ](#)

Returns an iterator over the bytes of a string slice.

As a string slice consists of a sequence of bytes, we can iterate through a string slice by byte. This method returns such an iterator.

##### Examples

```rust
let mut bytes = "bors".bytes();

assert_eq!(Some(b'b'), bytes.next());
assert_eq!(Some(b'o'), bytes.next());
assert_eq!(Some(b'r'), bytes.next());
assert_eq!(Some(b's'), bytes.next());

assert_eq!(None, bytes.next());
```

1.1.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1210)

#### pub fn [split\_whitespace](#method.split_whitespace)(&self) -> [SplitWhitespace](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html "struct core::str::iter::SplitWhitespace")<'\_> [ⓘ](#)

Splits a string slice by whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of whitespace.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`. If you only want to split on ASCII whitespace instead, use [`split_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_ascii_whitespace "method str::split_ascii_whitespace").

##### Examples

Basic usage:

```rust
let mut iter = "A few words".split_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

All kinds of whitespace are considered:

```rust
let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all whitespace, the iterator yields no string slices:

```rust
assert_eq!("".split_whitespace().next(), None);
assert_eq!("   ".split_whitespace().next(), None);
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1264)

#### pub fn [split\_ascii\_whitespace](#method.split_ascii_whitespace)(&self) -> [SplitAsciiWhitespace](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html "struct core::str::iter::SplitAsciiWhitespace")<'\_> [ⓘ](#)

Splits a string slice by ASCII whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of ASCII whitespace.

This uses the same definition as [`char::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.is_ascii_whitespace "method char::is_ascii_whitespace"). To split by Unicode `Whitespace` instead, use [`split_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_whitespace "method str::split_whitespace"). Note that because of this difference in definition, even if `s.is_ascii()` is `true`, `s.split_ascii_whitespace()` behavior will differ from `s.split_whitespace()` if `s` contains U+000B VERTICAL TAB.

##### Examples

Basic usage:

```rust
let mut iter = "A few words".split_ascii_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

Various kinds of ASCII whitespace are considered (see [`char::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.is_ascii_whitespace "method char::is_ascii_whitespace")):

```rust
let mut iter = " Mary   had\ta little  \n\t lamb".split_ascii_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all ASCII whitespace, the iterator yields no string slices:

```rust
assert_eq!("".split_ascii_whitespace().next(), None);
assert_eq!("   ".split_ascii_whitespace().next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1328)

#### pub fn [lines](#method.lines)(&self) -> [Lines](https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html "struct core::str::iter::Lines")<'\_> [ⓘ](#)

Returns an iterator over the lines of a string, as string slices.

Lines are split at line endings that are either newlines (`\n`) or sequences of a carriage return followed by a line feed (`\r\n`).

Line terminators are not included in the lines returned by the iterator.

Note that any carriage return (`\r`) not immediately followed by a line feed (`\n`) does not split a line. These carriage returns are thereby included in the produced lines.

The final line ending is optional. A string that ends with a final line ending will return the same lines as an otherwise identical string without a final line ending.

An empty string returns an empty iterator.

##### Examples

Basic usage:

```rust
let text = "foo\r\nbar\n\nbaz\r";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
// Trailing carriage return is included in the last line
assert_eq!(Some("baz\r"), lines.next());

assert_eq!(None, lines.next());
```

The final line does not require any ending:

```rust
let text = "foo\nbar\n\r\nbaz";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
assert_eq!(Some("baz"), lines.next());

assert_eq!(None, lines.next());
```

An empty string returns an empty iterator:

```rust
let text = "";
let mut lines = text.lines();

assert_eq!(lines.next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1337)

#### pub fn [lines\_any](#method.lines_any)(&self) -> [LinesAny](https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html "struct core::str::iter::LinesAny")<'\_> [ⓘ](#)

👎Deprecated since 1.4.0:

use lines() instead now

Returns an iterator over the lines of a string.

1.8.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1357)

#### pub fn [encode\_utf16](#method.encode_utf16)(&self) -> [EncodeUtf16](https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html "struct core::str::iter::EncodeUtf16")<'\_> [ⓘ](#)

Returns an iterator of `u16` over the string encoded as native endian UTF-16 (without byte-order mark).

##### Examples

```rust
let text = "Zażółć gęślą jaźń";

let utf8_len = text.len();
let utf16_len = text.encode_utf16().count();

assert!(utf16_len <= utf8_len);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1382)

#### pub fn [contains](#method.contains)<P>(&self, pat: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns `true` if the given pattern matches a sub-slice of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
let bananas = "bananas";

assert!(bananas.contains("nana"));
assert!(!bananas.contains("apples"));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1420)

#### pub fn [starts\_with](#method.starts_with)<P>(&self, pat: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns `true` if the given pattern matches a prefix of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, in which case this function will return true if the `&str` is a prefix of this string slice.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can also be a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches. These will only be checked against the first character of this string slice. Look at the second example below regarding behavior for slices of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s.

##### Examples

```rust
let bananas = "bananas";

assert!(bananas.starts_with("bana"));
assert!(!bananas.starts_with("nana"));
```

```rust
let bananas = "bananas";

// Note that both of these assert successfully.
assert!(bananas.starts_with(&['b', 'a', 'n', 'a']));
assert!(bananas.starts_with(&['a', 'b', 'c', 'd']));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1445-1447)

#### pub fn [ends\_with](#method.ends_with)<P>(&self, pat: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns `true` if the given pattern matches a suffix of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
let bananas = "bananas";

assert!(bananas.ends_with("anas"));
assert!(!bananas.ends_with("nana"));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1496)

#### pub fn [find](#method.find)<P>(&self, pat: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns the byte index of the first character of this string slice that matches the pattern.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the pattern doesn’t match.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

Simple patterns:

```rust
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.find('L'), Some(0));
assert_eq!(s.find('é'), Some(14));
assert_eq!(s.find("pard"), Some(17));
```

More complex patterns using point-free style and closures:

```rust
let s = "Löwe 老虎 Léopard";

assert_eq!(s.find(char::is_whitespace), Some(5));
assert_eq!(s.find(char::is_lowercase), Some(1));
assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
```

Not finding the pattern:

```rust
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.find(x), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1542-1544)

#### pub fn [rfind](#method.rfind)<P>(&self, pat: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns the byte index for the first character of the last match of the pattern in this string slice.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the pattern doesn’t match.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

Simple patterns:

```rust
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.rfind('L'), Some(13));
assert_eq!(s.rfind('é'), Some(14));
assert_eq!(s.rfind("pard"), Some(24));
```

More complex patterns with closures:

```rust
let s = "Löwe 老虎 Léopard";

assert_eq!(s.rfind(char::is_whitespace), Some(12));
assert_eq!(s.rfind(char::is_lowercase), Some(20));
```

Not finding the pattern:

```rust
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.rfind(x), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1670)

#### pub fn [split](#method.split)<P>(&self, pat: P) -> [Split](https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html "struct core::str::iter::Split")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

If there are no matches the full string slice is returned as the only item in the iterator.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit "method str::rsplit") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

let v: Vec<&str> = "".split('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
assert_eq!(v, ["lion", "", "tiger", "leopard"]);

let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);

let v: Vec<&str> = "AABBCC".split("DD").collect();
assert_eq!(v, ["AABBCC"]);

let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
assert_eq!(v, ["abc", "def", "ghi"]);

let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);
```

If the pattern is a slice of chars, split on each occurrence of any of the characters:

```rust
let v: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
assert_eq!(v, ["2020", "11", "03", "23", "59"]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "def", "ghi"]);
```

If a string contains multiple contiguous separators, you will end up with empty strings in the output:

```rust
let x = "||||a||b|c".to_string();
let d: Vec<_> = x.split('|').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

Contiguous separators are separated by the empty string.

```rust
let x = "(///)".to_string();
let d: Vec<_> = x.split('/').collect();

assert_eq!(d, &["(", "", "", ")"]);
```

Separators at the start or end of a string are neighbored by empty strings.

```rust
let d: Vec<_> = "010".split("0").collect();
assert_eq!(d, &["", "1", ""]);
```

When the empty string is used as a separator, it separates every character in the string, along with the beginning and end of the string.

```rust
let f: Vec<_> = "rust".split("").collect();
assert_eq!(f, &["", "r", "u", "s", "t", ""]);
```

Contiguous separators can lead to possibly surprising behavior when whitespace is used as the separator. This code is correct:

```rust
let x = "    a  b c".to_string();
let d: Vec<_> = x.split(' ').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

It does _not_ give you:

[ⓘ](# "This example is not tested")

```rust
assert_eq!(d, &["a", "b", "c"]);
```

Use [`split_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_whitespace "method str::split_whitespace") for this behavior.

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1711)

#### pub fn [split\_inclusive](#method.split_inclusive)<P>(&self, pat: P) -> [SplitInclusive](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html "struct core::str::iter::SplitInclusive")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

Differs from the iterator produced by `split` in that `split_inclusive` leaves the matched part as the terminator of the substring.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
```

If the last element of the string is matched, that element will be considered the terminator of the preceding substring. That substring will be the last item returned by the iterator.

```rust
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb.\n"
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb.\n"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1766-1768)

#### pub fn [rsplit](#method.rsplit)<P>(&self, pat: P) -> [RSplit](https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html "struct core::str::iter::RSplit")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern and yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lamb".rsplit(' ').collect();
assert_eq!(v, ["lamb", "little", "a", "had", "Mary"]);

let v: Vec<&str> = "".rsplit('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplit('X').collect();
assert_eq!(v, ["leopard", "tiger", "", "lion"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplit("::").collect();
assert_eq!(v, ["leopard", "tiger", "lion"]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".rsplit(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "def", "abc"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1815)

#### pub fn [split\_terminator](#method.split_terminator)<P>(&self, pat: P) -> [SplitTerminator](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html "struct core::str::iter::SplitTerminator")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

Equivalent to [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split"), except that the trailing substring is skipped if empty.

This method can be used for string data that is _terminated_, rather than _separated_ by a pattern.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit_terminator`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit_terminator "method str::rsplit_terminator") method can be used.

##### Examples

```rust
let v: Vec<&str> = "A.B.".split_terminator('.').collect();
assert_eq!(v, ["A", "B"]);

let v: Vec<&str> = "A..B..".split_terminator(".").collect();
assert_eq!(v, ["A", "", "B", ""]);

let v: Vec<&str> = "A.B:C.D".split_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["A", "B", "C", "D"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1861-1863)

#### pub fn [rsplit\_terminator](#method.rsplit_terminator)<P>(&self, pat: P) -> [RSplitTerminator](https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html "struct core::str::iter::RSplitTerminator")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over substrings of `self`, separated by characters matched by a pattern and yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

Equivalent to [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split"), except that the trailing substring is skipped if empty.

This method can be used for string data that is _terminated_, rather than _separated_ by a pattern.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be double ended if a forward/reverse search yields the same elements.

For iterating from the front, the [`split_terminator`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_terminator "method str::split_terminator") method can be used.

##### Examples

```rust
let v: Vec<&str> = "A.B.".rsplit_terminator('.').collect();
assert_eq!(v, ["B", "A"]);

let v: Vec<&str> = "A..B..".rsplit_terminator(".").collect();
assert_eq!(v, ["", "B", "", "A"]);

let v: Vec<&str> = "A.B:C.D".rsplit_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["D", "C", "B", "A"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1916)

#### pub fn [splitn](#method.splitn)<P>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pat: P) -> [SplitN](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html "struct core::str::iter::SplitN")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of the given string slice, separated by a pattern, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

If the pattern allows a reverse search, the [`rsplitn`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplitn "method str::rsplitn") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
assert_eq!(v, ["Mary", "had", "a little lambda"]);

let v: Vec<&str> = "lionXXtigerXleopard".splitn(3, "X").collect();
assert_eq!(v, ["lion", "", "tigerXleopard"]);

let v: Vec<&str> = "abcXdef".splitn(1, 'X').collect();
assert_eq!(v, ["abcXdef"]);

let v: Vec<&str> = "".splitn(1, 'X').collect();
assert_eq!(v, [""]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".splitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "defXghi"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1965-1967)

#### pub fn [rsplitn](#method.rsplitn)<P>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pat: P) -> [RSplitN](https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html "struct core::str::iter::RSplitN")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over substrings of this string slice, separated by a pattern, starting from the end of the string, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

For splitting from the front, the [`splitn`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.splitn "method str::splitn") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lamb".rsplitn(3, ' ').collect();
assert_eq!(v, ["lamb", "little", "Mary had a"]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplitn(3, 'X').collect();
assert_eq!(v, ["leopard", "tiger", "lionX"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplitn(2, "::").collect();
assert_eq!(v, ["leopard", "lion::tiger"]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".rsplitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "abc1def"]);
```

1.52.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1985)

#### pub fn [split\_once](#method.split_once)<P>(&self, delimiter: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Splits the string on the first occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### Examples

```rust
assert_eq!("cfg".split_once('='), None);
assert_eq!("cfg=".split_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".split_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".split_once('='), Some(("cfg", "foo=bar")));
```

1.52.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2004-2006)

#### pub fn [rsplit\_once](#method.rsplit_once)<P>(&self, delimiter: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Splits the string on the last occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### Examples

```rust
assert_eq!("cfg".rsplit_once('='), None);
assert_eq!("cfg=".rsplit_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".rsplit_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".rsplit_once('='), Some(("cfg=foo", "bar")));
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2044)

#### pub fn [matches](#method.matches)<P>(&self, pat: P) -> [Matches](https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html "struct core::str::iter::Matches")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over the disjoint matches of a pattern within the given string slice.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatches "method str::rmatches") method can be used.

##### Examples

```rust
let v: Vec<&str> = "abcXXXabcYYYabc".matches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
assert_eq!(v, ["1", "2", "3"]);
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2078-2080)

#### pub fn [rmatches](#method.rmatches)<P>(&self, pat: P) -> [RMatches](https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html "struct core::str::iter::RMatches")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over the disjoint matches of a pattern within this string slice, yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.matches "method str::matches") method can be used.

##### Examples

```rust
let v: Vec<&str> = "abcXXXabcYYYabc".rmatches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".rmatches(char::is_numeric).collect();
assert_eq!(v, ["3", "2", "1"]);
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2122)

#### pub fn [match\_indices](#method.match_indices)<P>(&self, pat: P) -> [MatchIndices](https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html "struct core::str::iter::MatchIndices")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over the disjoint matches of a pattern within this string slice as well as the index that the match starts at.

For matches of `pat` within `self` that overlap, only the indices corresponding to the first match are returned.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatch_indices`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatch_indices "method str::rmatch_indices") method can be used.

##### Examples

```rust
let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);

let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
assert_eq!(v, [(1, "abc"), (4, "abc")]);

let v: Vec<_> = "ababa".match_indices("aba").collect();
assert_eq!(v, [(0, "aba")]); // only the first `aba`
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2162-2164)

#### pub fn [rmatch\_indices](#method.rmatch_indices)<P>(&self, pat: P) -> [RMatchIndices](https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html "struct core::str::iter::RMatchIndices")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over the disjoint matches of a pattern within `self`, yielded in reverse order along with the index of the match.

For matches of `pat` within `self` that overlap, only the indices corresponding to the last match are returned.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`match_indices`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.match_indices "method str::match_indices") method can be used.

##### Examples

```rust
let v: Vec<_> = "abcXXXabcYYYabc".rmatch_indices("abc").collect();
assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);

let v: Vec<_> = "1abcabc2".rmatch_indices("abc").collect();
assert_eq!(v, [(4, "abc"), (1, "abc")]);

let v: Vec<_> = "ababa".rmatch_indices("aba").collect();
assert_eq!(v, [(2, "aba")]); // only the last `aba`
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2186)

#### pub fn [trim](#method.trim)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading and trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### Examples

```rust
let s = "\n Hello\tworld\t\n";

assert_eq!("Hello\tworld", s.trim());
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2225)

#### pub fn [trim\_start](#method.trim_start)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### Examples

Basic usage:

```rust
let s = "\n Hello\tworld\t\n";
assert_eq!("Hello\tworld\t\n", s.trim_start());
```

Directionality:

```rust
let s = "  English  ";
assert!(Some('E') == s.trim_start().chars().next());

let s = "  עברית  ";
assert!(Some('ע') == s.trim_start().chars().next());
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2264)

#### pub fn [trim\_end](#method.trim_end)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### Examples

Basic usage:

```rust
let s = "\n Hello\tworld\t\n";
assert_eq!("\n Hello\tworld", s.trim_end());
```

Directionality:

```rust
let s = "  English  ";
assert!(Some('h') == s.trim_end().chars().rev().next());

let s = "  עברית  ";
assert!(Some('ת') == s.trim_end().chars().rev().next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2304)

#### pub fn [trim\_left](#method.trim_left)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.33.0:

superseded by `trim_start`

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _right_ side, not the left.

##### Examples

Basic usage:

```rust
let s = " Hello\tworld\t";

assert_eq!("Hello\tworld\t", s.trim_left());
```

Directionality:

```rust
let s = "  English";
assert!(Some('E') == s.trim_left().chars().next());

let s = "  עברית";
assert!(Some('ע') == s.trim_left().chars().next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2344)

#### pub fn [trim\_right](#method.trim_right)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.33.0:

superseded by `trim_end`

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _left_ side, not the right.

##### Examples

Basic usage:

```rust
let s = " Hello\tworld\t";

assert_eq!(" Hello\tworld", s.trim_right());
```

Directionality:

```rust
let s = "English  ";
assert!(Some('h') == s.trim_right().chars().rev().next());

let s = "עברית  ";
assert!(Some('ת') == s.trim_right().chars().rev().next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2377-2379)

#### pub fn [trim\_matches](#method.trim_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [DoubleEndedSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.DoubleEndedSearcher.html "trait core::str::pattern::DoubleEndedSearcher")<'a>,

Returns a string slice with all prefixes and suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

Simple patterns:

```rust
assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
```

A more complex pattern, using a closure:

```rust
assert_eq!("1foo1barXX".trim_matches(|c| c == '1' || c == 'X'), "foo1bar");
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2424)

#### pub fn [trim\_start\_matches](#method.trim_start_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### Examples

```rust
assert_eq!("11foo1bar11".trim_start_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_start_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
```

1.45.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2458)

#### pub fn [strip\_prefix](#method.strip_prefix)<P>(&self, prefix: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns a string slice with the prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix, wrapped in `Some`. Unlike [`trim_start_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start_matches "method str::trim_start_matches"), this method removes the prefix exactly once.

If the string does not start with `prefix`, returns `None`.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
assert_eq!("foo:bar".strip_prefix("foo:"), Some("bar"));
assert_eq!("foo:bar".strip_prefix("bar"), None);
assert_eq!("foofoo".strip_prefix("foo"), Some("foo"));
```

1.45.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2486-2488)

#### pub fn [strip\_suffix](#method.strip_suffix)<P>(&self, suffix: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns a string slice with the suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix, wrapped in `Some`. Unlike [`trim_end_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end_matches "method str::trim_end_matches"), this method removes the suffix exactly once.

If the string does not end with `suffix`, returns `None`.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
assert_eq!("bar:foo".strip_suffix(":foo"), Some("bar"));
assert_eq!("bar:foo".strip_suffix("bar"), None);
assert_eq!("foofoo".strip_suffix("foo"), Some("foo"));
```

1.98.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2520-2522)

#### pub fn [strip\_circumfix](#method.strip_circumfix)<P, S>(&self, prefix: P, suffix: S) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), S: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <S as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns a string slice with the prefix and suffix removed.

If the string starts with the pattern `prefix` and ends with the pattern `suffix`, returns the substring after the prefix and before the suffix, wrapped in `Some`. Unlike [`trim_start_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start_matches "method str::trim_start_matches") and [`trim_end_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end_matches "method str::trim_end_matches"), this method removes both the prefix and suffix exactly once.

If the string does not start with `prefix` or does not end with `suffix`, returns `None`.

Each [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
assert_eq!("bar:hello:foo".strip_circumfix("bar:", ":foo"), Some("hello"));
assert_eq!("bar:foo".strip_circumfix("foo", "foo"), None);
assert_eq!("foo:bar;".strip_circumfix("foo:", ';'), Some("bar"));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2560)

#### pub fn [trim\_prefix](#method.trim_prefix)<P>(&self, prefix: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix. Unlike [`strip_prefix`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_prefix "method str::strip_prefix"), this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

If the string does not start with `prefix`, returns the original string unchanged.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
#![feature(trim_prefix_suffix)]

// Prefix present - removes it
assert_eq!("foo:bar".trim_prefix("foo:"), "bar");
assert_eq!("foofoo".trim_prefix("foo"), "foo");

// Prefix absent - returns original string
assert_eq!("foo:bar".trim_prefix("bar"), "foo:bar");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2597-2599)

#### pub fn [trim\_suffix](#method.trim_suffix)<P>(&self, suffix: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix. Unlike [`strip_suffix`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_suffix "method str::strip_suffix"), this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

If the string does not end with `suffix`, returns the original string unchanged.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
#![feature(trim_prefix_suffix)]

// Suffix present - removes it
assert_eq!("bar:foo".trim_suffix(":foo"), "bar");
assert_eq!("foofoo".trim_suffix("foo"), "foo");

// Suffix absent - returns original string
assert_eq!("bar:foo".trim_suffix("bar"), "bar:foo");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2640-2642)

#### pub fn [trim\_end\_matches](#method.trim_end_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### Examples

Simple patterns:

```rust
assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_end_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

```rust
assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2684)

#### pub fn [trim\_left\_matches](#method.trim_left_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

👎Deprecated since 1.33.0:

superseded by `trim_start_matches`

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _right_ side, not the left.

##### Examples

```rust
assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_left_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2727-2729)

#### pub fn [trim\_right\_matches](#method.trim_right_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

👎Deprecated since 1.33.0:

superseded by `trim_end_matches`

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _left_ side, not the right.

##### Examples

Simple patterns:

```rust
assert_eq!("11foo1bar11".trim_right_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_right_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_right_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

```rust
assert_eq!("1fooX".trim_right_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2778)

#### pub fn [parse](#method.parse)<F>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<F, <F as [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr")\>::[Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "type core::str::traits::FromStr::Err")\>

where F: [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr"),

Parses this string slice into another type.

Because `parse` is so general, it can cause problems with type inference. As such, `parse` is one of the few times you’ll see the syntax affectionately known as the ‘turbofish’: `::<>`. This helps the inference algorithm understand specifically which type you’re trying to parse into.

`parse` can parse into any type that implements the [`FromStr`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") trait.

##### Errors

Will return [`Err`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "associated type core::str::traits::FromStr::Err") if it’s not possible to parse this string slice into the desired type.

##### Examples

Basic usage:

```rust
let four: u32 = "4".parse().unwrap();

assert_eq!(4, four);
```

Using the ‘turbofish’ instead of annotating `four`:

```rust
let four = "4".parse::<u32>();

assert_eq!(Ok(4), four);
```

Failing to parse:

```rust
let nope = "j".parse::<u32>();

assert!(nope.is_err());
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2799)

#### pub fn [is\_ascii](#method.is_ascii)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if all characters in this string are within the ASCII range.

An empty string returns `true`.

##### Examples

```rust
let ascii = "hello!\n";
let non_ascii = "Grüße, Jürgen ❤";

assert!(ascii.is_ascii());
assert!(!non_ascii.is_ascii());
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2811)

#### pub fn [as\_ascii](#method.as_ascii)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&\[[AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\]>

🔬This is a nightly-only experimental API. (`ascii_char`)

If this string slice [`is_ascii`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_ascii "method str::is_ascii"), returns it as a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), otherwise returns `None`.

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2825)

#### pub unsafe fn [as\_ascii\_unchecked](#method.as_ascii_unchecked)(&self) -> &\[[AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\]

🔬This is a nightly-only experimental API. (`ascii_char`)

Converts this string slice into a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), without checking whether they are valid.

##### Safety

Every character in this string must be ASCII, or else this is UB.

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2856)

#### pub fn [eq\_ignore\_ascii\_case](#method.eq_ignore_ascii_case)(&self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks that two strings are an ASCII case-insensitive match.

Same as `to_ascii_lowercase(a) == to_ascii_lowercase(b)`, but without allocating and copying temporaries.

For Unicode-aware case-insensitive matching, consider [`str::eq_ignore_case_unnormalized`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.eq_ignore_case_unnormalized "method str::eq_ignore_case_unnormalized").

##### Examples

```rust
assert!("Ferris".eq_ignore_ascii_case("FERRIS"));
assert!("Ferrös".eq_ignore_ascii_case("FERRöS"));
assert!(!"Ferrös".eq_ignore_ascii_case("FERRÖS"));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2907)

#### pub fn [eq\_ignore\_case\_unnormalized](#method.eq_ignore_case_unnormalized)(&self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

🔬This is a nightly-only experimental API. (`casefold`)

Checks that two strings are a caseless match, according to [Definition 144](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G53513) in Chapter 3 of the Unicode Standard.

Same as `a.to_casefold_unnormalized() == b.to_casefold_unnormalized()`, but without allocating. See that method’s documentation, as well as [`char::to_casefold_unnormalized()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_casefold_unnormalized "method char::to_casefold_unnormalized"), for more information about case folding.

No [normalization](https://www.unicode.org/faq/normalization.html) (e.g. NFC) is performed, so visually and semantically identical strings might still compare unequal. For example, `"Å"` (U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE) is considered distinct from `"Å"` (A followed by U+030A COMBINING RING ABOVE), even though Unicode considers them canonically equivalent.

In addition, this method is independent of language/locale, so the special behavior of I/ı/İ/i in Turkish and Azeri is not handled.

##### Examples

```rust
#![feature(casefold)]
assert!("Ferris".eq_ignore_case_unnormalized("FERRIS"));
assert!("Ferrös".eq_ignore_case_unnormalized("FERRÖS"));
assert!("ẞ".eq_ignore_case_unnormalized("ss"));
```

No NFC [normalization](https://www.unicode.org/faq/normalization.html) is performed:

```rust
#![feature(casefold)]
// These two strings are visually and semantically identical...
let comp = "Å";
let decomp = "Å";

// ... but not codepoint-for-codepoint equal.
assert_eq!(comp, "\u{C5}");
assert_eq!(decomp, "A\u{030A}");

// Their case-foldings are likewise unequal:
assert!(!comp.eq_ignore_case_unnormalized(decomp));
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2991)

#### pub fn [trim\_ascii\_start](#method.trim_ascii_start)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace"). Importantly, this definition excludes the U+000B code point even though it has the Unicode [`White_Space`](https://www.unicode.org/reports/tr44/#White_Space) property and is removed by [`str::trim_start`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start "method str::trim_start").

##### Examples

```rust
assert_eq!(" \t \u{3000}hello world\n".trim_ascii_start(), "\u{3000}hello world\n");
assert_eq!("  ".trim_ascii_start(), "");
assert_eq!("".trim_ascii_start(), "");
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3019)

#### pub fn [trim\_ascii\_end](#method.trim_ascii_end)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace"). Importantly, this definition excludes the U+000B code point even though it has the Unicode [`White_Space`](https://www.unicode.org/reports/tr44/#White_Space) property and is removed by [`str::trim_end`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end "method str::trim_end").

##### Examples

```rust
assert_eq!("\r hello world\u{3000}\n ".trim_ascii_end(), "\r hello world\u{3000}");
assert_eq!("  ".trim_ascii_end(), "");
assert_eq!("".trim_ascii_end(), "");
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3048)

#### pub fn [trim\_ascii](#method.trim_ascii)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading and trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace"). Importantly, this definition excludes the U+000B code point even though it has the Unicode [`White_Space`](https://www.unicode.org/reports/tr44/#White_Space) property and is removed by [`str::trim`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim "method str::trim").

##### Examples

```rust
assert_eq!("\r hello world\n ".trim_ascii(), "hello world");
assert_eq!("  ".trim_ascii(), "");
assert_eq!("".trim_ascii(), "");
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3091)

#### pub fn [escape\_debug](#method.escape_debug)(&self) -> [EscapeDebug](https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html "struct core::str::iter::EscapeDebug")<'\_> [ⓘ](#)

Returns an iterator that escapes each char in `self` with [`char::escape_debug`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_debug "method char::escape_debug").

Note: only extended grapheme codepoints that begin the string will be escaped.

##### Examples

As an iterator:

```rust
for c in "❤\n!".escape_debug() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

```rust
println!("{}", "❤\n!".escape_debug());
```

Both are equivalent to:

```rust
println!("❤\\n!");
```

Using `to_string`:

```rust
assert_eq!("❤\n!".escape_debug().to_string(), "❤\\n!");
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3137)

#### pub fn [escape\_default](#method.escape_default)(&self) -> [EscapeDefault](https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html "struct core::str::iter::EscapeDefault")<'\_> [ⓘ](#)

Returns an iterator that escapes each char in `self` with [`char::escape_default`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_default "method char::escape_default").

##### Examples

As an iterator:

```rust
for c in "❤\n!".escape_default() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

```rust
println!("{}", "❤\n!".escape_default());
```

Both are equivalent to:

```rust
println!("\\u{{2764}}\\n!");
```

Using `to_string`:

```rust
assert_eq!("❤\n!".escape_default().to_string(), "\\u{2764}\\n!");
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3175)

#### pub fn [escape\_unicode](#method.escape_unicode)(&self) -> [EscapeUnicode](https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html "struct core::str::iter::EscapeUnicode")<'\_> [ⓘ](#)

Returns an iterator that escapes each char in `self` with [`char::escape_unicode`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_unicode "method char::escape_unicode").

##### Examples

As an iterator:

```rust
for c in "❤\n!".escape_unicode() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

```rust
println!("{}", "❤\n!".escape_unicode());
```

Both are equivalent to:

```rust
println!("\\u{{2764}}\\u{{a}}\\u{{21}}");
```

Using `to_string`:

```rust
assert_eq!("❤\n!".escape_unicode().to_string(), "\\u{2764}\\u{a}\\u{21}");
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3209)

#### pub fn [substr\_range](#method.substr_range)(&self, substr: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Range](https://doc.rust-lang.org/nightly/core/range/struct.Range.html "struct core::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the range that a substring points to.

Returns `None` if `substr` does not point within `self`.

Unlike [`str::find`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.find "method str::find"), **this does not search through the string**. Instead, it uses pointer arithmetic to find where in the string `substr` is derived from.

This is useful for extending [`str::split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split") and similar methods.

Note that this method may return false positives (typically either `Some(0..0)` or `Some(self.len()..self.len())`) if `substr` is a zero-length `str` that points at the beginning or end of another, independent, `str`.

##### Examples

```rust
#![feature(substr_range)]
use core::range::Range;

let data = "a, b, b, a";
let mut iter = data.split(", ").map(|s| data.substr_range(s).unwrap());

assert_eq!(iter.next(), Some(Range { start: 0, end: 1 }));
assert_eq!(iter.next(), Some(Range { start: 3, end: 4 }));
assert_eq!(iter.next(), Some(Range { start: 6, end: 7 }));
assert_eq!(iter.next(), Some(Range { start: 9, end: 10 }));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3220)

#### pub fn [as\_str](#method.as_str-1)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

🔬This is a nightly-only experimental API. (`str_as_str`)

Returns the same string as a string slice `&str`.

This method is redundant when used directly on `&str`, but it helps dereferencing other string-like types to string slices, for example references to `Box<str>` or `Arc<str>`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#308)

#### pub fn [replace](#method.replace)<P>(&self, from: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [String](struct.String.html "struct bevy::prelude::String")

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Available on **non-`no_global_oom_handling`** only.

Replaces all matches of a pattern with another string.

`replace` creates a new [`String`](struct.String.html "struct bevy::prelude::String"), and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice.

##### Examples

```rust
let s = "this is old";

assert_eq!("this is new", s.replace("old", "new"));
assert_eq!("than an old", s.replace("is", "an"));
```

When the pattern doesn’t match, it returns this string slice as [`String`](struct.String.html "struct bevy::prelude::String"):

```rust
let s = "this is old";
assert_eq!(s, s.replace("cookie monster", "little lamb"));
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#366)

#### pub fn [replacen](#method.replacen)<P>(&self, pat: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), count: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](struct.String.html "struct bevy::prelude::String")

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Available on **non-`no_global_oom_handling`** only.

Replaces first N matches of a pattern with another string.

`replacen` creates a new [`String`](struct.String.html "struct bevy::prelude::String"), and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice at most `count` times.

##### Examples

```rust
let s = "foo foo 123 foo";
assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
```

When the pattern doesn’t match, it returns this string slice as [`String`](struct.String.html "struct bevy::prelude::String"):

```rust
let s = "this is old";
assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#433)

#### pub fn [to\_lowercase](#method.to_lowercase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns the lowercase equivalent of this string slice, as a new [`String`](struct.String.html "struct bevy::prelude::String").

‘Lowercase’ is defined according to the terms of [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G34432) of the Unicode standard.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

Unlike [`char::to_lowercase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_lowercase "method char::to_lowercase"), this method fully handles the context-dependent casing of Greek sigma. However, like that method, it does not handle locale-specific casing, like Turkish and Azeri I/ı/İ/i. See its documentation for more information.

##### Examples

Basic usage:

```rust
let s = "HELLO WORLD";

assert_eq!("hello world", s.to_lowercase());
```

Tricky examples, with sigma:

```rust
let sigma = "Σ";

assert_eq!("σ", sigma.to_lowercase());

// but at the end of a word, it's ς, not σ:
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());

let odysseus_king_of_ithaca = "Ο ΟΔΥΣΣΈΑΣ ΒΑΣΙΛΙΆΣ ΤΗΣ ΙΘΆΚΗΣ";

assert_eq!("ο οδυσσέας βασιλιάς της ιθάκης", odysseus_king_of_ithaca.to_lowercase());
```

Languages without case are not changed:

```rust
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_lowercase());
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#554)

#### pub fn [word\_to\_titlecase](#method.word_to_titlecase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

🔬This is a nightly-only experimental API. (`titlecase`)

Available on **non-`no_global_oom_handling`** only.

Returns the titlecase equivalent of this string slice, which is assumed to represent a single word, as a new [`String`](struct.String.html "struct bevy::prelude::String").

Essentially, this consists of uppercasing the first cased letter (with [`char::to_titlecase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_titlecase "method char::to_titlecase")), and lowercasing everything that follows.

‘Titlecase’ is defined according to the terms of [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G34082) of the Unicode standard.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

Unlike [`char::to_lowercase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_lowercase "method char::to_lowercase"), this method fully handles the context-dependent casing of Greek sigma. However, like that method, it does not handle locale-specific casing, like Turkish and Azeri I/ı/İ/i. See its documentation for more information.

This method does not perform any kind of word segmentation.

##### Examples

Basic usage:

```rust
#![feature(titlecase)]
let s = "HELLO WORLD";

assert_eq!("Hello world", s.word_to_titlecase());
```

The first _cased_ letter is uppercased:

```rust
#![feature(titlecase)]
let the_night_before_christmas = "'twas";

assert_eq!("'Twas", the_night_before_christmas.word_to_titlecase());
```

Languages without case are not changed:

```rust
#![feature(titlecase)]
let new_year = "农历新年";

assert_eq!(new_year, new_year.word_to_titlecase());
```

Georgian uppercase (“Mtavruli”) letters are not used in titlecase:

```rust
#![feature(titlecase)]
let georgian = "ერთობაშია";

assert_eq!(georgian, georgian.word_to_titlecase());
```

No word segmentation is performed, so only the first cased letter in the whole string gets uppercased:

```rust
#![feature(titlecase)]
let blazingly_fast = "ferris and I";

assert_eq!("Ferris and i", blazingly_fast.word_to_titlecase());
```

Tricky examples, with sigma:

```rust
#![feature(titlecase)]
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("Ὀδυσσεύς", odysseus.word_to_titlecase());

let odysseus_king_of_ithaca = "Ο ΟΔΥΣΣΈΑΣ ΒΑΣΙΛΙΆΣ ΤΗΣ ΙΘΆΚΗΣ";

assert_eq!("Ο οδυσσέας βασιλιάς της ιθάκης", odysseus_king_of_ithaca.word_to_titlecase());
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#640)

#### pub fn [to\_uppercase](#method.to_uppercase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns the uppercase equivalent of this string slice, as a new [`String`](struct.String.html "struct bevy::prelude::String").

‘Uppercase’ is defined according to the terms of [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G34431) of the Unicode standard.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

Like [`char::to_uppercase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_uppercase "method char::to_uppercase") this method does not handle language-specific casing, like Turkish and Azeri I/ı/İ/i. See that method’s documentation for more information.

##### Examples

Basic usage:

```rust
let s = "hello world";

assert_eq!("HELLO WORLD", s.to_uppercase());
```

Scripts without case are not changed:

```rust
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_uppercase());
```

One character can become multiple:

```rust
let s = "tschüß";

assert_eq!("TSCHÜSS", s.to_uppercase());
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#742)

#### pub fn [to\_casefold\_unnormalized](#method.to_casefold_unnormalized)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

🔬This is a nightly-only experimental API. (`casefold`)

Available on **non-`no_global_oom_handling`** only.

Returns the case-folded equivalent of this string slice, as a new [`String`](struct.String.html "struct bevy::prelude::String").

Case folding is a transformation, mostly matching lowercase, that is meant to be used for case-insensitive string comparisons. Case-folded strings should not usually be exposed directly to users.

For the precise specification of case folding, see [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G63737) of the Unicode standard.

Since some characters can expand into multiple characters when case folding, this function returns a [`String`](struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

No [normalization](https://www.unicode.org/faq/normalization.html) (e.g. NFC) is performed, so visually and semantically identical strings might still casefold differently. For example, `"Å"` (U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE) is considered distinct from `"Å"` (A followed by U+030A COMBINING RING ABOVE), even though Unicode considers them canonically equivalent.

Like [`char::to_casefold_unnormalized()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_casefold_unnormalized "method char::to_casefold_unnormalized") this method does not handle language-specific casing, like Turkish and Azeri I/ı/İ/i. See that method’s documentation for more information.

##### Examples

Basic usage:

```rust
#![feature(casefold)]
let s0 = "HELLO";
let s1 = "Hello";

assert_eq!(s0.to_casefold_unnormalized(), s1.to_casefold_unnormalized());
assert_eq!(s0.to_casefold_unnormalized(), "hello")
```

Scripts without case are not changed:

```rust
#![feature(casefold)]
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_casefold_unnormalized());
```

One character can become multiple:

```rust
#![feature(casefold)]
let s0 = "TSCHÜẞ";
let s1 = "TSCHÜSS";
let s2 = "tschüß";

assert_eq!(s0.to_casefold_unnormalized(), s1.to_casefold_unnormalized());
assert_eq!(s0.to_casefold_unnormalized(), s2.to_casefold_unnormalized());
assert_eq!(s0.to_casefold_unnormalized(), "tschüss");
```

No NFC [normalization](https://www.unicode.org/faq/normalization.html) is performed:

```rust
#![feature(casefold)]
// These two strings are visually and semantically identical...
let comp = "Å";
let decomp = "Å";

// ... but not codepoint-for-codepoint equal.
assert_eq!(comp, "\u{C5}");
assert_eq!(decomp, "A\u{030A}");

// Their case-foldings are likewise unequal:
assert_eq!(comp.to_casefold_unnormalized(), "\u{E5}");
assert_eq!(decomp.to_casefold_unnormalized(), "a\u{030A}");
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#808)

#### pub fn [repeat](#method.repeat)(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Creates a new [`String`](struct.String.html "struct bevy::prelude::String") by repeating a string `n` times.

##### Panics

This function will panic if the capacity would overflow.

##### Examples

Basic usage:

```rust
assert_eq!("abc".repeat(4), String::from("abcabcabcabc"));
```

A panic upon overflow:

[ⓘ](# "This example panics")

```rust
// this will panic at runtime
let huge = "0123456789abcdef".repeat(usize::MAX);
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#838)

#### pub fn [to\_ascii\_uppercase](#method.to_ascii_uppercase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns a copy of this string where each character is mapped to its ASCII upper case equivalent.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To uppercase the value in-place, use [`make_ascii_uppercase`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_uppercase "method str::make_ascii_uppercase").

To uppercase ASCII characters in addition to non-ASCII characters, use [`to_uppercase`](#method.to_uppercase).

##### Examples

```rust
let s = "Grüße, Jürgen ❤";

assert_eq!("GRüßE, JüRGEN ❤", s.to_ascii_uppercase());
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#870)

#### pub fn [to\_ascii\_lowercase](#method.to_ascii_lowercase)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns a copy of this string where each character is mapped to its ASCII lower case equivalent.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To lowercase the value in-place, use [`make_ascii_lowercase`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_lowercase "method str::make_ascii_lowercase").

To lowercase ASCII characters in addition to non-ASCII characters, use [`to_lowercase`](#method.to_lowercase).

##### Examples

```rust
let s = "Grüße, Jürgen ❤";

assert_eq!("grüße, jürgen ❤", s.to_ascii_lowercase());
```

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#192)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#194)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#40)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#40)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Name](struct.Name.html "struct bevy::prelude::Name")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#40)

### impl [Component](trait.Component.html "trait bevy::prelude::Component") for [Name](struct.Name.html "struct bevy::prelude::Name")

where [Name](struct.Name.html "struct bevy::prelude::Name"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#40)

#### const [STORAGE\_TYPE](trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::Table

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#40)

#### type [Mutability](trait.Component.html#associatedtype.Mutability) = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](trait.Component.html "trait bevy::prelude::Component"). [Read more](trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#40)

#### fn [register\_required\_components](trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#40)

#### fn [clone\_behavior](trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#40)

#### fn [relationship\_accessor](trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[Name](struct.Name.html "struct bevy::prelude::Name")\>>

Returns [`ComponentRelationshipAccessor`](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor") required for working with relationships in dynamic contexts. [Read more](trait.Component.html#method.relationship_accessor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#524)

#### fn [on\_add](trait.Component.html#method.on_add)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_add` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#529)

#### fn [on\_insert](trait.Component.html#method.on_insert)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_insert` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#534)

#### fn [on\_discard](trait.Component.html#method.on_discard)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_discard` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#539)

#### fn [on\_remove](trait.Component.html#method.on_remove)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_remove` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#544)

#### fn [on\_despawn](trait.Component.html#method.on_despawn)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_despawn` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#649)

#### fn [map\_entities](trait.Component.html#method.map_entities)<E>(\_this: &mut Self, \_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Maps the entities on this component using the given [`EntityMapper`](trait.EntityMapper.html "trait bevy::prelude::EntityMapper"). This is used to remap entities in contexts like scenes and entity cloning. When deriving [`Component`](trait.Component.html "trait bevy::prelude::Component"), this is populated by annotating fields containing entities with `#[entities]` [Read more](trait.Component.html#method.map_entities)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#128)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#130)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#52)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#53)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Name](struct.Name.html "struct bevy::prelude::Name")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#244)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#245)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#247)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[Name](struct.Name.html "struct bevy::prelude::Name") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#260)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Name](struct.Name.html "struct bevy::prelude::Name")

Available on **crate feature `serialize`** only.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#261)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Name](struct.Name.html "struct bevy::prelude::Name"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#121)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#123)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#230)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#199)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[Name](struct.Name.html "struct bevy::prelude::Name")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#201)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(val: &[Name](struct.Name.html "struct bevy::prelude::Name")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#1345)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[Name](struct.Name.html "struct bevy::prelude::Name")\> for [AnimationTargetId](../animation/struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#1346)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(name: &[Name](struct.Name.html "struct bevy::prelude::Name")) -> [AnimationTargetId](../animation/struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#176)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#178)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Name](struct.Name.html "struct bevy::prelude::Name")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#206)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Name](struct.Name.html "struct bevy::prelude::Name")\> for [String](struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#208)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(val: [Name](struct.Name.html "struct bevy::prelude::Name")) -> [String](struct.String.html "struct bevy::prelude::String")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#183)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#185)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(name: [String](struct.String.html "struct bevy::prelude::String")) -> [Name](struct.Name.html "struct bevy::prelude::Name")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Name](struct.Name.html "struct bevy::prelude::Name")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Name](struct.Name.html "struct bevy::prelude::Name") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [from\_reflect](trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Name](struct.Name.html "struct bevy::prelude::Name")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#213)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#214)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Name](struct.Name.html "struct bevy::prelude::Name"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#238)

### impl [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#239)

#### fn [cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)(&self, other: &[Name](struct.Name.html "struct bevy::prelude::Name")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#219)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#220)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Name](struct.Name.html "struct bevy::prelude::Name")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#232)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#233)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[Name](struct.Name.html "struct bevy::prelude::Name")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [get\_represented\_type\_info](trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [try\_apply](trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [reflect\_kind](trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [reflect\_ref](trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [reflect\_mut](trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [reflect\_owned](trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Name](struct.Name.html "struct bevy::prelude::Name")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [try\_into\_reflect](trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Name](struct.Name.html "struct bevy::prelude::Name")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [try\_as\_reflect](trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [try\_as\_reflect\_mut](trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [into\_partial\_reflect](trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Name](struct.Name.html "struct bevy::prelude::Name")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [as\_partial\_reflect](trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [as\_partial\_reflect\_mut](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#44)

#### fn [reflect\_hash](trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#44)

#### fn [reflect\_partial\_eq](trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [reflect\_partial\_cmp](trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#44)

#### fn [debug](trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#44)

#### fn [reflect\_clone](trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [into\_any](trait.Reflect.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Name](struct.Name.html "struct bevy::prelude::Name")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [as\_any](trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [as\_any\_mut](trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [into\_reflect](trait.Reflect.html#tymethod.into_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Name](struct.Name.html "struct bevy::prelude::Name")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [as\_reflect](trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [as\_reflect\_mut](trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [set](trait.Reflect.html#tymethod.set)(&mut self, value: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#253)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Name](struct.Name.html "struct bevy::prelude::Name")

Available on **crate feature `serialize`** only.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#254)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [field](trait.TupleStruct.html#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field with index `index` as a `&dyn Reflect`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [field\_mut](trait.TupleStruct.html#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field with index `index` as a `&mut dyn Reflect`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [field\_len](trait.TupleStruct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the tuple struct.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [iter\_fields](trait.TupleStruct.html#tymethod.iter_fields)(&self) -> [TupleStructFieldIter](../reflect/tuple_struct/struct.TupleStructFieldIter.html "struct bevy::reflect::tuple_struct::TupleStructFieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the tuple struct’s fields.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [to\_dynamic\_tuple\_struct](trait.TupleStruct.html#method.to_dynamic_tuple_struct)(&self) -> [DynamicTupleStruct](../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct")

Creates a new [`DynamicTupleStruct`](../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct") from this tuple struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#71)

#### fn [get\_represented\_tuple\_struct\_info](trait.TupleStruct.html#method.get_represented_tuple_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TupleStructInfo](../reflect/tuple_struct/struct.TupleStructInfo.html "struct bevy::reflect::tuple_struct::TupleStructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [type\_path](trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [short\_type\_path](trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [type\_ident](trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [crate\_name](trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [module\_path](trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Name](struct.Name.html "struct bevy::prelude::Name")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Name](struct.Name.html "struct bevy::prelude::Name")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Name](struct.Name.html "struct bevy::prelude::Name")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Name](struct.Name.html "struct bevy::prelude::Name")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Name](struct.Name.html "struct bevy::prelude::Name")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Name](struct.Name.html "struct bevy::prelude::Name")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Name](struct.Name.html "struct bevy::prelude::Name")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

[Source](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)

### impl<T> [Brush](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/style/brush/trait.Brush.html "trait parley::style::brush::Brush") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#16)

### impl<C> [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#17-19)

#### fn [component\_ids](trait.Bundle.html#tymethod.component_ids)( components: &mut [ComponentsRegistrator](../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\> + use<C>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#23)

#### fn [get\_component\_ids](trait.Bundle.html#tymethod.get_component_ids)( components: &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#31-35)

#### unsafe fn [from\_components](../ecs/bundle/trait.BundleFromComponents.html#tymethod.from_components)<T, F>(ctx: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), func: [&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> C

where F: for<'a> [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'a>, C: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/taffy/0.10.1/x86_64-unknown-linux-gnu/src/taffy/style/mod.rs.html#62-63)

### impl<T> [CheapCloneStr](https://docs.rs/taffy/0.10.1/x86_64-unknown-linux-gnu/taffy/style/trait.CheapCloneStr.html "trait taffy::style::CheapCloneStr") for T

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + for<'a> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](struct.String.html "struct bevy::prelude::String")\> + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + 'static,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#104-107)

### impl<Q, K> [Comparable](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html "trait equivalent::Comparable")<K> for Q

where Q: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#110)

#### fn [compare](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html#tymethod.compare)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

Compare self to `key` and return their ordering.

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#47-49)

### impl<T> [DynHash](../ecs/label/trait.DynHash.html "trait bevy::ecs::label::DynHash") for T

where T: [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#51)

#### fn [dyn\_hash](../ecs/label/trait.DynHash.html#tymethod.dyn_hash)(&self, state: &mut dyn [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"))

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#43)

### impl<C> [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for C

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

An operation on the entity that happens _after_ inserting this bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#46-49)

#### unsafe fn [get\_components](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)( ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, C>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), ) -> <C as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect")

Moves the components out of the bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#54)

#### unsafe fn [apply\_effect](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)( \_ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<C>>, \_entity: &mut [EntityWorldMut](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Applies the after-effects of spawning this bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

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

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#722)

### impl<T> [ErasedBundleTemplate](../scene/trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate") for T

where T: [Template](trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#723)

#### unsafe fn [apply](../scene/trait.ErasedBundleTemplate.html#tymethod.apply)( &self, context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`. [Read more](../scene/trait.ErasedBundleTemplate.html#tymethod.apply)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#729)

#### fn [clone\_template](../scene/trait.ErasedBundleTemplate.html#tymethod.clone_template)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedBundleTemplate](../scene/trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate")\>

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#686)

### impl<T> [ErasedComponentTemplate](../scene/trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate") for T

where T: [Template](trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#687-691)

#### unsafe fn [apply](../scene/trait.ErasedComponentTemplate.html#tymethod.apply)( &self, context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, bundle\_writer: &mut [BundleWriter](../ecs/bundle/struct.BundleWriter.html "struct bevy::ecs::bundle::BundleWriter")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`. [Read more](../scene/trait.ErasedComponentTemplate.html#tymethod.apply)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#701)

#### fn [clone\_template](../scene/trait.ErasedComponentTemplate.html#tymethod.clone_template)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedComponentTemplate](../scene/trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate")\>

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#207)

### impl<S> [GetTupleStructField](trait.GetTupleStructField.html "trait bevy::prelude::GetTupleStructField") for S

where S: [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#208)

#### fn [get\_field](trait.GetTupleStructField.html#tymethod.get_field)<T>(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a reference to the value of the field with index `index`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#213)

#### fn [get\_field\_mut](trait.GetTupleStructField.html#tymethod.get_field_mut)<T>(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a mutable reference to the value of the field with index `index`, downcast to `T`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

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

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#203-206)

### impl<T> [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source") for T

where T: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#208)

#### type [Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice)<'a> = <<T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target") as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'a> where T: 'a

A type this `Source` can be sliced into.

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#213)

#### fn [len](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Length of the source

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#217-219)

#### fn [read](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)<'a, Chunk>(&'a self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Chunk>

where Chunk: [Chunk](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Chunk.html "trait logos::source::Chunk")<'a>,

Read a chunk of bytes into an array. Returns `None` when reading out of bounds would occur. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#224)

#### fn [slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)(&self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>>

Get a slice of the source at given range. This is analogous to `slice::get(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#229)

#### unsafe fn [slice\_unchecked](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)( &self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> <T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>

Available on **non-crate feature `forbid_unsafe`** only.

Get a slice of the source at given range. This is analogous to `slice::get_unchecked(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#233)

#### fn [is\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Check if `index` is valid for this `Source`, that is: [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#237)

#### fn [find\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

For `&str` sources attempts to find the closest `char` boundary at which source can be sliced, starting from `index`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](trait.ToOwned.html#method.clone_into)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)

### impl<T> [ToSmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/trait.ToSmolStr.html "trait smol_str::ToSmolStr") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)

#### fn [to\_smolstr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/trait.ToSmolStr.html#tymethod.to_smolstr)(&self) -> [SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2900)

### impl<T> [ToString](trait.ToString.html "trait bevy::prelude::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2902)

#### fn [to\_string](trait.ToString.html#tymethod.to_string)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Converts the given value to a `String`. [Read more](trait.ToString.html#tymethod.to_string)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

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

{"&\[u8\]":"<h3>Notable traits for <code>&amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Bytes<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html\\" title=\\"struct core::str::iter::Bytes\\">Bytes</a>&lt;'\_&gt;</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html\\" title=\\"struct core::str::iter::Bytes\\">Bytes</a>&lt;'\_&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>;</div>","CharIndices<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html\\" title=\\"struct core::str::iter::CharIndices\\">CharIndices</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html\\" title=\\"struct core::str::iter::CharIndices\\">CharIndices</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>);</div>","Chars<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html\\" title=\\"struct core::str::iter::Chars\\">Chars</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html\\" title=\\"struct core::str::iter::Chars\\">Chars</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","EncodeUtf16<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html\\" title=\\"struct core::str::iter::EncodeUtf16\\">EncodeUtf16</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html\\" title=\\"struct core::str::iter::EncodeUtf16\\">EncodeUtf16</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\\">u16</a>;</div>","EscapeDebug<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html\\" title=\\"struct core::str::iter::EscapeDebug\\">EscapeDebug</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html\\" title=\\"struct core::str::iter::EscapeDebug\\">EscapeDebug</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","EscapeDefault<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html\\" title=\\"struct core::str::iter::EscapeDefault\\">EscapeDefault</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html\\" title=\\"struct core::str::iter::EscapeDefault\\">EscapeDefault</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","EscapeUnicode<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html\\" title=\\"struct core::str::iter::EscapeUnicode\\">EscapeUnicode</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html\\" title=\\"struct core::str::iter::EscapeUnicode\\">EscapeUnicode</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Lines<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html\\" title=\\"struct core::str::iter::Lines\\">Lines</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html\\" title=\\"struct core::str::iter::Lines\\">Lines</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","LinesAny<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html\\" title=\\"struct core::str::iter::LinesAny\\">LinesAny</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html\\" title=\\"struct core::str::iter::LinesAny\\">LinesAny</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","MatchIndices<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html\\" title=\\"struct core::str::iter::MatchIndices\\">MatchIndices</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html\\" title=\\"struct core::str::iter::MatchIndices\\">MatchIndices</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>);</div>","Matches<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html\\" title=\\"struct core::str::iter::Matches\\">Matches</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html\\" title=\\"struct core::str::iter::Matches\\">Matches</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RMatchIndices<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html\\" title=\\"struct core::str::iter::RMatchIndices\\">RMatchIndices</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html\\" title=\\"struct core::str::iter::RMatchIndices\\">RMatchIndices</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>);</div>","RMatches<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html\\" title=\\"struct core::str::iter::RMatches\\">RMatches</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html\\" title=\\"struct core::str::iter::RMatches\\">RMatches</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RSplit<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html\\" title=\\"struct core::str::iter::RSplit\\">RSplit</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html\\" title=\\"struct core::str::iter::RSplit\\">RSplit</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RSplitN<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html\\" title=\\"struct core::str::iter::RSplitN\\">RSplitN</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html\\" title=\\"struct core::str::iter::RSplitN\\">RSplitN</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RSplitTerminator<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html\\" title=\\"struct core::str::iter::RSplitTerminator\\">RSplitTerminator</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html\\" title=\\"struct core::str::iter::RSplitTerminator\\">RSplitTerminator</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","Split<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html\\" title=\\"struct core::str::iter::Split\\">Split</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html\\" title=\\"struct core::str::iter::Split\\">Split</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitAsciiWhitespace<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html\\" title=\\"struct core::str::iter::SplitAsciiWhitespace\\">SplitAsciiWhitespace</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html\\" title=\\"struct core::str::iter::SplitAsciiWhitespace\\">SplitAsciiWhitespace</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitInclusive<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html\\" title=\\"struct core::str::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html\\" title=\\"struct core::str::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitN<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html\\" title=\\"struct core::str::iter::SplitN\\">SplitN</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html\\" title=\\"struct core::str::iter::SplitN\\">SplitN</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitTerminator<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html\\" title=\\"struct core::str::iter::SplitTerminator\\">SplitTerminator</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html\\" title=\\"struct core::str::iter::SplitTerminator\\">SplitTerminator</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitWhitespace<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html\\" title=\\"struct core::str::iter::SplitWhitespace\\">SplitWhitespace</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html\\" title=\\"struct core::str::iter::SplitWhitespace\\">SplitWhitespace</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","TupleStructFieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a (dyn <a class=\\"trait\\" href=\\"trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static);</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}