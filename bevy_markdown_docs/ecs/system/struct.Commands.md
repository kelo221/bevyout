[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Struct Commands 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#105)

```rust
pub struct Commands<'w, 's> { /* private fields */ }
```

A [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") queue to perform structural changes to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

Since each command requires exclusive access to the `World`, all queued commands are automatically applied in sequence when the `ApplyDeferred` system runs (see [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred") documentation for more details).

Each command can be used to modify the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") in arbitrary ways:

*   spawning or despawning entities
*   inserting components on new or existing entities
*   inserting resources
*   etc.

For a version of [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") that works in parallel contexts (such as within [`Query::par_iter`](../../prelude/struct.Query.html#method.par_iter "method bevy::prelude::Query::par_iter")) see [`ParallelCommands`](../../prelude/struct.ParallelCommands.html "struct bevy::prelude::ParallelCommands")

## Usage

Add `mut commands: Commands` as a function argument to your system to get a copy of this struct that will be applied the next time a copy of [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred") runs. Commands are almost always used as a [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

```rust
fn my_system(mut commands: Commands) {
   // ...
}
```

## Implementing

Each built-in command is implemented as a separate method, e.g. [`Commands::spawn`](../../prelude/struct.Commands.html#method.spawn "method bevy::prelude::Commands::spawn"). In addition to the pre-defined command methods, you can add commands with any arbitrary behavior using [`Commands::queue`](../../prelude/struct.Commands.html#method.queue "method bevy::prelude::Commands::queue"), which accepts any type implementing [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command").

Since closures and other functions implement this trait automatically, this allows one-shot, anonymous custom commands.

```rust
// NOTE: type inference fails here, so annotations are required on the closure.
commands.queue(|w: &mut World| {
    // Mutate the world however you want...
});
```

## Error handling

A [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") can return a [`Result`](../../prelude/type.Result.html "type bevy::prelude::Result"), which will be passed to an [error handler](../error/index.html "mod bevy::ecs::error") if the `Result` is an error.

The fallback error handler panics. It can be configured via the [`FallbackErrorHandler`](../error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler") resource.

Alternatively, you can customize the error handler for a specific command by calling [`Commands::queue_handled`](../../prelude/struct.Commands.html#method.queue_handled "method bevy::prelude::Commands::queue_handled").

The [`error`](../error/index.html "mod bevy::ecs::error") module provides some simple error handlers for convenience.

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#218)

### impl<'w, 's> [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#220)

#### pub fn [new](#method.new)(queue: &'s mut [CommandQueue](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue"), world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

Returns a new `Commands` instance from a [`CommandQueue`](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue") and a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#225-229)

#### pub fn [new\_from\_entities](#method.new_from_entities)( queue: &'s mut [CommandQueue](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue"), allocator: &'w [EntityAllocator](../entity/struct.EntityAllocator.html "struct bevy::ecs::entity::EntityAllocator"), entities: &'w [Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities"), ) -> [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

Returns a new `Commands` instance from a [`CommandQueue`](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue") and an [`Entities`](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities") reference.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#266)

#### pub fn [rebound\_to](#method.rebound_to)<'q>(&self, queue: &'q mut [CommandQueue](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue")) -> [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 'q>

Returns a new [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") that writes commands to the provided [`CommandQueue`](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue") instead of the one from `self`.

This is useful if you have a `Commands` that writes to one queue and you want one that writes to another.

Note that you’re responsible for ensuring the queue eventually writes its commands to the world. One way to do this is calling [`Commands::append`](../../prelude/struct.Commands.html#method.append "method bevy::prelude::Commands::append") on a `Commands` that writes to the world queue. Failure to write a queue may result in entities being allocated but never spawned, which means those entity IDs are never freed for reuse.

The original `Commands` isn’t mutated or borrowed after this returns, so you can keep using it.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#289)

#### pub fn [reborrow](#method.reborrow)(&mut self) -> [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, '\_>

Returns a [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") with a smaller lifetime.

This is useful if you have `&mut Commands` but need `Commands`.

##### Example

```rust
fn my_system(mut commands: Commands) {
    // We do our initialization in a separate function,
    // which expects an owned `Commands`.
    do_initialization(commands.reborrow());

    // Since we only reborrowed the commands instead of moving them, we can still use them.
    commands.spawn_empty();
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#303)

#### pub fn [append](#method.append)(&mut self, other: &mut [CommandQueue](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue"))

Take all commands from `other` and append them to `self`, leaving `other` empty.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/async\_tasks/async\_compute.rs ([line 136](../../../src/async_compute/async_compute.rs.html#136))

```rust
128fn handle_tasks(
129    mut commands: Commands,
130    mut transform_tasks: Query<(Entity, &mut ComputeTransform)>,
131) {
132    for (entity, mut task) in &mut transform_tasks {
133        // Use `check_ready` to efficiently poll the task without blocking the main thread.
134        if let Some(mut commands_queue) = check_ready(&mut task.0) {
135            // Append the returned command queue to execute it later.
136            commands.append(&mut commands_queue);
137            // Task is complete, so remove the task component from the entity.
138            commands.entity(entity).remove::<ComputeTransform>();
139        }
140    }
141}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#346)

#### pub fn [spawn\_empty](#method.spawn_empty)(&mut self) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Spawns a new empty [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") and returns its corresponding [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands").

##### Example

```rust
#[derive(Component)]
struct Label(&'static str);
#[derive(Component)]
struct Strength(u32);
#[derive(Component)]
struct Agility(u32);

fn example_system(mut commands: Commands) {
    // Create a new empty entity.
    commands.spawn_empty();

    // Create another empty entity.
    commands.spawn_empty()
        // Add a new component bundle to the entity.
        .insert((Strength(1), Agility(2)))
        // Add a single component to the entity.
        .insert(Label("hello world"));
}
```

##### See also

*   [`spawn`](../../prelude/struct.Commands.html#method.spawn "method bevy::prelude::Commands::spawn") to spawn an entity with components.
*   [`spawn_batch`](../../prelude/struct.Commands.html#method.spawn_batch "method bevy::prelude::Commands::spawn_batch") to spawn many entities with the same combination of components.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/async\_tasks/async\_compute.rs ([line 74](../../../src/async_compute/async_compute.rs.html#74))

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

Hide additional examples

examples/stress\_tests/transform\_hierarchy.rs ([line 405](../../../src/transform_hierarchy/transform_hierarchy.rs.html#405))

```rust
354fn spawn_tree(
355    parent_map: &[usize],
356    commands: &mut Commands,
357    update_filter: &UpdateFilter,
358    root_transform: Transform,
359) -> InsertResult {
360    // total count (# of nodes + root)
361    let count = parent_map.len() + 1;
362
363    #[derive(Default, Clone, Copy)]
364    struct NodeInfo {
365        child_count: u32,
366        depth: u32,
367    }
368
369    // node index -> entity lookup list
370    let mut ents: Vec<Entity> = Vec::with_capacity(count);
371    let mut node_info: Vec<NodeInfo> = vec![default(); count];
372    for (i, &parent_idx) in parent_map.iter().enumerate() {
373        // assert spawn order (parent must be processed before child)
374        assert!(parent_idx <= i, "invalid spawn order");
375        node_info[parent_idx].child_count += 1;
376    }
377
378    // insert root
379    ents.push(commands.spawn(root_transform).id());
380
381    let mut result = InsertResult::default();
382    let mut rng = rand::rng();
383    // used to count through the number of children (used only for visual layout)
384    let mut child_idx: Vec<u16> = vec![0; count];
385
386    // insert children
387    for (current_idx, &parent_idx) in parent_map.iter().enumerate() {
388        let current_idx = current_idx + 1;
389
390        // separation factor to visually separate children (0..1)
391        let sep = child_idx[parent_idx] as f32 / node_info[parent_idx].child_count as f32;
392        child_idx[parent_idx] += 1;
393
394        // calculate and set depth
395        // this works because it's guaranteed that we have already iterated over the parent
396        let depth = node_info[parent_idx].depth + 1;
397        let info = &mut node_info[current_idx];
398        info.depth = depth;
399
400        // update max depth of tree
401        result.maximum_depth = result.maximum_depth.max(depth.try_into().unwrap());
402
403        // insert child
404        let child_entity = {
405            let mut cmd = commands.spawn_empty();
406
407            // check whether or not to update this node
408            let update = (rng.random::<f32>() <= update_filter.probability)
409                && (depth >= update_filter.min_depth && depth <= update_filter.max_depth);
410
411            if update {
412                cmd.insert(UpdateValue(sep));
413                result.active_nodes += 1;
414            }
415
416            let transform = {
417                let mut translation = Vec3::ZERO;
418                // use the same placement fn as the `update` system
419                // this way the entities won't be all at (0, 0, 0) when they don't have an `Update` component
420                set_translation(&mut translation, sep);
421                Transform::from_translation(translation)
422            };
423
424            // only insert the components necessary for the transform propagation
425            cmd.insert(transform);
426
427            cmd.id()
428        };
429
430        commands.entity(ents[parent_idx]).add_child(child_entity);
431
432        ents.push(child_entity);
433    }
434
435    result.inserted_nodes = ents.len();
436    result
437}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#398)

#### pub fn [spawn](#method.spawn)<T>(&mut self, bundle: T) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where T: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

Spawns a new [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") with the given components and returns the entity’s corresponding [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands").

To spawn many entities with the same combination of components, [`spawn_batch`](../../prelude/struct.Commands.html#method.spawn_batch "method bevy::prelude::Commands::spawn_batch") can be used for better performance.

##### Example

```rust
#[derive(Component)]
struct ComponentA(u32);
#[derive(Component)]
struct ComponentB(u32);

#[derive(Bundle)]
struct ExampleBundle {
    a: ComponentA,
    b: ComponentB,
}

fn example_system(mut commands: Commands) {
    // Create a new entity with a single component.
    commands.spawn(ComponentA(1));

    // Create a new entity with two components using a "tuple bundle".
    commands.spawn((ComponentA(2), ComponentB(1)));

    // Create a new entity with a component bundle.
    commands.spawn(ExampleBundle {
        a: ComponentA(3),
        b: ComponentB(2),
    });
}
```

##### See also

*   [`spawn_empty`](../../prelude/struct.Commands.html#method.spawn_empty "method bevy::prelude::Commands::spawn_empty") to spawn an entity without any components.
*   [`spawn_batch`](../../prelude/struct.Commands.html#method.spawn_batch "method bevy::prelude::Commands::spawn_batch") to spawn many entities with the same combination of components.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/showcase/game\_menu.rs ([line 48](../../../src/game_menu/game_menu.rs.html#48))

```rust
47fn setup(mut commands: Commands) {
48    commands.spawn(Camera2d);
49}
50
51mod splash {
52    use bevy::prelude::*;
53
54    use super::GameState;
55
56    // This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
57    pub fn splash_plugin(app: &mut App) {
58        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
59        app
60            // When entering the state, spawn everything needed for this screen
61            .add_systems(OnEnter(GameState::Splash), splash_setup)
62            // While in this state, run the `countdown` system
63            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
64    }
65
66    // Tag component used to tag entities added on the splash screen
67    #[derive(Component)]
68    struct OnSplashScreen;
69
70    // Newtype to use a `Timer` for this screen as a resource
71    #[derive(Resource, Deref, DerefMut)]
72    struct SplashTimer(Timer);
73
74    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
75        let icon = asset_server.load("branding/icon.png");
76        // Display the logo
77        commands.spawn((
78            // This entity will be despawned when exiting the state
79            DespawnOnExit(GameState::Splash),
80            Node {
81                align_items: AlignItems::Center,
82                justify_content: JustifyContent::Center,
83                width: percent(100),
84                height: percent(100),
85                ..default()
86            },
87            OnSplashScreen,
88            children![(
89                ImageNode::new(icon),
90                Node {
91                    // This will set the logo to be 200px wide, and auto adjust its height
92                    width: px(200),
93                    ..default()
94                },
95            )],
96        ));
97        // Insert the timer as a resource
98        commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
99    }
100
101    // Tick the timer, and change state when finished
102    fn countdown(
103        mut game_state: ResMut<NextState<GameState>>,
104        time: Res<Time>,
105        mut timer: ResMut<SplashTimer>,
106    ) {
107        if timer.tick(time.delta()).is_finished() {
108            game_state.set(GameState::Menu);
109        }
110    }
111}
112
113mod game {
114    use bevy::{
115        color::palettes::basic::{BLUE, LIME},
116        prelude::*,
117    };
118
119    use super::{DisplayQuality, GameState, Volume, TEXT_COLOR};
120
121    // This plugin will contain the game. In this case, it's just be a screen that will
122    // display the current settings for 5 seconds before returning to the menu
123    pub fn game_plugin(app: &mut App) {
124        app.add_systems(OnEnter(GameState::Game), game_setup)
125            .add_systems(Update, game.run_if(in_state(GameState::Game)));
126    }
127
128    // Tag component used to tag entities added on the game screen
129    #[derive(Component)]
130    struct OnGameScreen;
131
132    #[derive(Resource, Deref, DerefMut)]
133    struct GameTimer(Timer);
134
135    fn game_setup(
136        mut commands: Commands,
137        display_quality: Res<DisplayQuality>,
138        volume: Res<Volume>,
139    ) {
140        commands.spawn((
141            DespawnOnExit(GameState::Game),
142            Node {
143                width: percent(100),
144                height: percent(100),
145                // center children
146                align_items: AlignItems::Center,
147                justify_content: JustifyContent::Center,
148                ..default()
149            },
150            OnGameScreen,
151            children![(
152                Node {
153                    // This will display its children in a column, from top to bottom
154                    flex_direction: FlexDirection::Column,
155                    // `align_items` will align children on the cross axis. Here the main axis is
156                    // vertical (column), so the cross axis is horizontal. This will center the
157                    // children
158                    align_items: AlignItems::Center,
159                    ..default()
160                },
161                BackgroundColor(Color::BLACK),
162                children![
163                    (
164                        Text::new("Will be back to the menu shortly..."),
165                        TextFont {
166                            font_size: FontSize::Px(67.0),
167                            ..default()
168                        },
169                        TextColor(TEXT_COLOR),
170                        Node {
171                            margin: UiRect::all(px(50)),
172                            ..default()
173                        },
174                    ),
175                    (
176                        Text::default(),
177                        Node {
178                            margin: UiRect::all(px(50)),
179                            ..default()
180                        },
181                        children![
182                            (
183                                TextSpan(format!("quality: {:?}", *display_quality)),
184                                TextFont {
185                                    font_size: FontSize::Px(50.0),
186                                    ..default()
187                                },
188                                TextColor(BLUE.into()),
189                            ),
190                            (
191                                TextSpan::new(" - "),
192                                TextFont {
193                                    font_size: FontSize::Px(50.0),
194                                    ..default()
195                                },
196                                TextColor(TEXT_COLOR),
197                            ),
198                            (
199                                TextSpan(format!("volume: {:?}", *volume)),
200                                TextFont {
201                                    font_size: FontSize::Px(50.0),
202                                    ..default()
203                                },
204                                TextColor(LIME.into()),
205                            ),
206                        ]
207                    ),
208                ]
209            )],
210        ));
211        // Spawn a 5 seconds timer to trigger going back to the menu
212        commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
213    }
214
215    // Tick the timer, and change state when finished
216    fn game(
217        time: Res<Time>,
218        mut game_state: ResMut<NextState<GameState>>,
219        mut timer: ResMut<GameTimer>,
220    ) {
221        if timer.tick(time.delta()).is_finished() {
222            game_state.set(GameState::Menu);
223        }
224    }
225}
226
227mod menu {
228    use bevy::{
229        app::AppExit,
230        color::palettes::css::CRIMSON,
231        ecs::component::Mutable,
232        ecs::spawn::{SpawnIter, SpawnWith},
233        prelude::*,
234    };
235
236    use super::{DisplayQuality, GameState, Setting, Volume, TEXT_COLOR};
237
238    // This plugin manages the menu, with 5 different screens:
239    // - a main menu with "New Game", "Settings", "Quit"
240    // - a settings menu with two submenus and a back button
241    // - two settings screen with a setting that can be set and a back button
242    pub fn menu_plugin(app: &mut App) {
243        app
244            // At start, the menu is not enabled. This will be changed in `menu_setup` when
245            // entering the `GameState::Menu` state.
246            // Current screen in the menu is handled by an independent state from `GameState`
247            .init_state::<MenuState>()
248            .add_systems(OnEnter(GameState::Menu), menu_setup)
249            // Systems to handle the main menu screen
250            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
251            // Systems to handle the settings menu screen
252            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
253            // Systems to handle the display settings screen
254            .add_systems(
255                OnEnter(MenuState::SettingsDisplay),
256                display_settings_menu_setup,
257            )
258            .add_systems(
259                Update,
260                (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
261            )
262            // Systems to handle the sound settings screen
263            .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
264            .add_systems(
265                Update,
266                setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
267            )
268            // Common systems to all screens that handles buttons behavior
269            .add_systems(
270                Update,
271                (menu_action, button_system).run_if(in_state(GameState::Menu)),
272            );
273    }
274
275    // State used for the current menu screen
276    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
277    enum MenuState {
278        Main,
279        Settings,
280        SettingsDisplay,
281        SettingsSound,
282        #[default]
283        Disabled,
284    }
285
286    // Tag component used to tag entities added on the main menu screen
287    #[derive(Component)]
288    struct OnMainMenuScreen;
289
290    // Tag component used to tag entities added on the settings menu screen
291    #[derive(Component)]
292    struct OnSettingsMenuScreen;
293
294    // Tag component used to tag entities added on the display settings menu screen
295    #[derive(Component)]
296    struct OnDisplaySettingsMenuScreen;
297
298    // Tag component used to tag entities added on the sound settings menu screen
299    #[derive(Component)]
300    struct OnSoundSettingsMenuScreen;
301
302    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
303    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
304    const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
305    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
306
307    // Tag component used to mark which setting is currently selected
308    #[derive(Component)]
309    struct SelectedOption;
310
311    // All actions that can be triggered from a button click
312    #[derive(Component)]
313    enum MenuButtonAction {
314        Play,
315        Settings,
316        SettingsDisplay,
317        SettingsSound,
318        BackToMainMenu,
319        BackToSettings,
320        Quit,
321    }
322
323    // This system handles changing all buttons color based on mouse interaction
324    fn button_system(
325        mut interaction_query: Query<
326            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
327            (Changed<Interaction>, With<Button>),
328        >,
329    ) {
330        for (interaction, mut background_color, selected) in &mut interaction_query {
331            *background_color = match (*interaction, selected) {
332                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
333                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
334                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
335                (Interaction::None, None) => NORMAL_BUTTON.into(),
336            }
337        }
338    }
339
340    // This system updates the settings when a new value for a setting is selected, and marks
341    // the button as the one currently selected
342    fn setting_button<T: Resource<Mutability = Mutable> + Component + PartialEq + Copy>(
343        interaction_query: Query<
344            (&Interaction, &Setting<T>, Entity),
345            (Changed<Interaction>, With<Button>),
346        >,
347        selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
348        mut commands: Commands,
349        mut setting: ResMut<T>,
350    ) {
351        let (previous_button, mut previous_button_color) = selected_query.into_inner();
352        for (interaction, button_setting, entity) in &interaction_query {
353            if *interaction == Interaction::Pressed && *setting != button_setting.0 {
354                *previous_button_color = NORMAL_BUTTON.into();
355                commands.entity(previous_button).remove::<SelectedOption>();
356                commands.entity(entity).insert(SelectedOption);
357                *setting = button_setting.0;
358            }
359        }
360    }
361
362    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
363        menu_state.set(MenuState::Main);
364    }
365
366    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
367        // Common style for all buttons on the screen
368        let button_node = Node {
369            width: px(300),
370            height: px(65),
371            margin: UiRect::all(px(20)),
372            justify_content: JustifyContent::Center,
373            align_items: AlignItems::Center,
374            ..default()
375        };
376        let button_icon_node = Node {
377            width: px(30),
378            // This takes the icons out of the flexbox flow, to be positioned exactly
379            position_type: PositionType::Absolute,
380            // The icon will be close to the left border of the button
381            left: px(10),
382            ..default()
383        };
384        let button_text_font = TextFont {
385            font_size: FontSize::Px(33.0),
386            ..default()
387        };
388
389        let right_icon = asset_server.load("textures/Game Icons/right.png");
390        let wrench_icon = asset_server.load("textures/Game Icons/wrench.png");
391        let exit_icon = asset_server.load("textures/Game Icons/exitRight.png");
392
393        commands.spawn((
394            DespawnOnExit(MenuState::Main),
395            Node {
396                width: percent(100),
397                height: percent(100),
398                align_items: AlignItems::Center,
399                justify_content: JustifyContent::Center,
400                ..default()
401            },
402            OnMainMenuScreen,
403            children![(
404                Node {
405                    flex_direction: FlexDirection::Column,
406                    align_items: AlignItems::Center,
407                    ..default()
408                },
409                BackgroundColor(CRIMSON.into()),
410                children![
411                    // Display the game name
412                    (
413                        Text::new("Bevy Game Menu UI"),
414                        TextFont {
415                            font_size: FontSize::Px(67.0),
416                            ..default()
417                        },
418                        TextColor(TEXT_COLOR),
419                        Node {
420                            margin: UiRect::all(px(50)),
421                            ..default()
422                        },
423                    ),
424                    // Display three buttons for each action available from the main menu:
425                    // - new game
426                    // - settings
427                    // - quit
428                    (
429                        Button,
430                        button_node.clone(),
431                        BackgroundColor(NORMAL_BUTTON),
432                        MenuButtonAction::Play,
433                        children![
434                            (ImageNode::new(right_icon), button_icon_node.clone()),
435                            (
436                                Text::new("New Game"),
437                                button_text_font.clone(),
438                                TextColor(TEXT_COLOR),
439                            ),
440                        ]
441                    ),
442                    (
443                        Button,
444                        button_node.clone(),
445                        BackgroundColor(NORMAL_BUTTON),
446                        MenuButtonAction::Settings,
447                        children![
448                            (ImageNode::new(wrench_icon), button_icon_node.clone()),
449                            (
450                                Text::new("Settings"),
451                                button_text_font.clone(),
452                                TextColor(TEXT_COLOR),
453                            ),
454                        ]
455                    ),
456                    (
457                        Button,
458                        button_node,
459                        BackgroundColor(NORMAL_BUTTON),
460                        MenuButtonAction::Quit,
461                        children![
462                            (ImageNode::new(exit_icon), button_icon_node),
463                            (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
464                        ]
465                    ),
466                ]
467            )],
468        ));
469    }
470
471    fn settings_menu_setup(mut commands: Commands) {
472        let button_node = Node {
473            width: px(200),
474            height: px(65),
475            margin: UiRect::all(px(20)),
476            justify_content: JustifyContent::Center,
477            align_items: AlignItems::Center,
478            ..default()
479        };
480
481        let button_text_style = (
482            TextFont {
483                font_size: FontSize::Px(33.0),
484                ..default()
485            },
486            TextColor(TEXT_COLOR),
487        );
488
489        commands.spawn((
490            DespawnOnExit(MenuState::Settings),
491            Node {
492                width: percent(100),
493                height: percent(100),
494                align_items: AlignItems::Center,
495                justify_content: JustifyContent::Center,
496                ..default()
497            },
498            OnSettingsMenuScreen,
499            children![(
500                Node {
501                    flex_direction: FlexDirection::Column,
502                    align_items: AlignItems::Center,
503                    ..default()
504                },
505                BackgroundColor(CRIMSON.into()),
506                Children::spawn(SpawnIter(
507                    [
508                        (MenuButtonAction::SettingsDisplay, "Display"),
509                        (MenuButtonAction::SettingsSound, "Sound"),
510                        (MenuButtonAction::BackToMainMenu, "Back"),
511                    ]
512                    .into_iter()
513                    .map(move |(action, text)| {
514                        (
515                            Button,
516                            button_node.clone(),
517                            BackgroundColor(NORMAL_BUTTON),
518                            action,
519                            children![(Text::new(text), button_text_style.clone())],
520                        )
521                    })
522                ))
523            )],
524        ));
525    }
526
527    fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
528        fn button_node() -> Node {
529            Node {
530                width: px(200),
531                height: px(65),
532                margin: UiRect::all(px(20)),
533                justify_content: JustifyContent::Center,
534                align_items: AlignItems::Center,
535                ..default()
536            }
537        }
538        fn button_text_style() -> impl Bundle {
539            (
540                TextFont {
541                    font_size: FontSize::Px(33.0),
542                    ..default()
543                },
544                TextColor(TEXT_COLOR),
545            )
546        }
547
548        let display_quality = *display_quality;
549        commands.spawn((
550            DespawnOnExit(MenuState::SettingsDisplay),
551            Node {
552                width: percent(100),
553                height: percent(100),
554                align_items: AlignItems::Center,
555                justify_content: JustifyContent::Center,
556                ..default()
557            },
558            OnDisplaySettingsMenuScreen,
559            children![(
560                Node {
561                    flex_direction: FlexDirection::Column,
562                    align_items: AlignItems::Center,
563                    ..default()
564                },
565                BackgroundColor(CRIMSON.into()),
566                children![
567                    // Create a new `Node`, this time not setting its `flex_direction`. It will
568                    // use the default value, `FlexDirection::Row`, from left to right.
569                    (
570                        Node {
571                            align_items: AlignItems::Center,
572                            ..default()
573                        },
574                        BackgroundColor(CRIMSON.into()),
575                        Children::spawn((
576                            // Display a label for the current setting
577                            Spawn((Text::new("Display Quality"), button_text_style())),
578                            SpawnWith(move |parent: &mut ChildSpawner| {
579                                for quality_setting in [
580                                    DisplayQuality::Low,
581                                    DisplayQuality::Medium,
582                                    DisplayQuality::High,
583                                ] {
584                                    let mut entity = parent.spawn((
585                                        Button,
586                                        Node {
587                                            width: px(150),
588                                            height: px(65),
589                                            ..button_node()
590                                        },
591                                        BackgroundColor(NORMAL_BUTTON),
592                                        Setting(quality_setting),
593                                        children![(
594                                            Text::new(format!("{quality_setting:?}")),
595                                            button_text_style(),
596                                        )],
597                                    ));
598                                    if display_quality == quality_setting {
599                                        entity.insert(SelectedOption);
600                                    }
601                                }
602                            })
603                        ))
604                    ),
605                    // Display the back button to return to the settings screen
606                    (
607                        Button,
608                        button_node(),
609                        BackgroundColor(NORMAL_BUTTON),
610                        MenuButtonAction::BackToSettings,
611                        children![(Text::new("Back"), button_text_style())]
612                    )
613                ]
614            )],
615        ));
616    }
617
618    fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
619        let button_node = Node {
620            width: px(200),
621            height: px(65),
622            margin: UiRect::all(px(20)),
623            justify_content: JustifyContent::Center,
624            align_items: AlignItems::Center,
625            ..default()
626        };
627        let button_text_style = (
628            TextFont {
629                font_size: FontSize::Px(33.0),
630                ..default()
631            },
632            TextColor(TEXT_COLOR),
633        );
634
635        let volume = *volume;
636        let button_node_clone = button_node.clone();
637        commands.spawn((
638            DespawnOnExit(MenuState::SettingsSound),
639            Node {
640                width: percent(100),
641                height: percent(100),
642                align_items: AlignItems::Center,
643                justify_content: JustifyContent::Center,
644                ..default()
645            },
646            OnSoundSettingsMenuScreen,
647            children![(
648                Node {
649                    flex_direction: FlexDirection::Column,
650                    align_items: AlignItems::Center,
651                    ..default()
652                },
653                BackgroundColor(CRIMSON.into()),
654                children![
655                    (
656                        Node {
657                            align_items: AlignItems::Center,
658                            ..default()
659                        },
660                        BackgroundColor(CRIMSON.into()),
661                        Children::spawn((
662                            Spawn((Text::new("Volume"), button_text_style.clone())),
663                            SpawnWith(move |parent: &mut ChildSpawner| {
664                                for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
665                                    let mut entity = parent.spawn((
666                                        Button,
667                                        Node {
668                                            width: px(30),
669                                            height: px(65),
670                                            ..button_node_clone.clone()
671                                        },
672                                        BackgroundColor(NORMAL_BUTTON),
673                                        Setting(Volume(volume_setting)),
674                                    ));
675                                    if volume == Volume(volume_setting) {
676                                        entity.insert(SelectedOption);
677                                    }
678                                }
679                            })
680                        ))
681                    ),
682                    (
683                        Button,
684                        button_node,
685                        BackgroundColor(NORMAL_BUTTON),
686                        MenuButtonAction::BackToSettings,
687                        children![(Text::new("Back"), button_text_style)]
688                    )
689                ]
690            )],
691        ));
692    }
```

Hide additional examples

examples/state/custom\_transitions.rs ([line 222](../../../src/custom_transitions/custom_transitions.rs.html#222))

```rust
221fn setup(mut commands: Commands) {
222    commands.spawn(Camera2d);
223}
224
225fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
226    commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
227    info!("Setup game");
228}
229
230fn teardown_game(mut commands: Commands, player: Single<Entity, With<Sprite>>) {
231    commands.entity(*player).despawn();
232    info!("Teardown game");
233}
234
235#[derive(Resource)]
236struct MenuData {
237    pub button_entity: Entity,
238}
239
240const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
241const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
242const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
243
244fn setup_menu(mut commands: Commands) {
245    let button_entity = commands
246        .spawn((
247            Node {
248                // center button
249                width: percent(100),
250                height: percent(100),
251                justify_content: JustifyContent::Center,
252                align_items: AlignItems::Center,
253                ..default()
254            },
255            children![(
256                Button,
257                Node {
258                    width: px(150),
259                    height: px(65),
260                    // horizontally center child text
261                    justify_content: JustifyContent::Center,
262                    // vertically center child text
263                    align_items: AlignItems::Center,
264                    ..default()
265                },
266                BackgroundColor(NORMAL_BUTTON),
267                children![(
268                    Text::new("Play"),
269                    TextFont {
270                        font_size: FontSize::Px(33.0),
271                        ..default()
272                    },
273                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
274                )]
275            )],
276        ))
277        .id();
278    commands.insert_resource(MenuData { button_entity });
279}
```

examples/state/states.rs ([line 54](../../../src/states/states.rs.html#54))

```rust
53fn setup(mut commands: Commands) {
54    commands.spawn(Camera2d);
55}
56
57fn setup_menu(mut commands: Commands) {
58    let button_entity = commands
59        .spawn((
60            Node {
61                // center button
62                width: percent(100),
63                height: percent(100),
64                justify_content: JustifyContent::Center,
65                align_items: AlignItems::Center,
66                ..default()
67            },
68            children![(
69                Button,
70                Node {
71                    width: px(150),
72                    height: px(65),
73                    // horizontally center child text
74                    justify_content: JustifyContent::Center,
75                    // vertically center child text
76                    align_items: AlignItems::Center,
77                    ..default()
78                },
79                BackgroundColor(NORMAL_BUTTON),
80                children![(
81                    Text::new("Play"),
82                    TextFont {
83                        font_size: FontSize::Px(33.0),
84                        ..default()
85                    },
86                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
87                )],
88            )],
89        ))
90        .id();
91    commands.insert_resource(MenuData { button_entity });
92}
93
94fn menu(
95    mut next_state: ResMut<NextState<AppState>>,
96    mut interaction_query: Query<
97        (&Interaction, &mut BackgroundColor),
98        (Changed<Interaction>, With<Button>),
99    >,
100) {
101    for (interaction, mut color) in &mut interaction_query {
102        match *interaction {
103            Interaction::Pressed => {
104                *color = PRESSED_BUTTON.into();
105                next_state.set(AppState::InGame);
106            }
107            Interaction::Hovered => {
108                *color = HOVERED_BUTTON.into();
109            }
110            Interaction::None => {
111                *color = NORMAL_BUTTON.into();
112            }
113        }
114    }
115}
116
117fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
118    commands.entity(menu_data.button_entity).despawn();
119}
120
121fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
122    commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
123}
```

examples/window/clear\_color.rs ([line 17](../../../src/clear_color/clear_color.rs.html#17))

```rust
16fn setup(mut commands: Commands) {
17    commands.spawn(Camera2d);
18}
```

examples/2d/sprite\_scale.rs ([line 14](../../../src/sprite_scale/sprite_scale.rs.html#14))

```rust
13fn setup_camera(mut commands: Commands) {
14    commands.spawn(Camera2d);
15}
16
17fn setup_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
18    let square = asset_server.load("textures/slice_square_2.png");
19    let banner = asset_server.load("branding/banner.png");
20
21    let rects = [
22        Rect {
23            size: Vec2::new(100., 225.),
24            text: "Stretched".to_string(),
25            transform: Transform::from_translation(Vec3::new(-570., 230., 0.)),
26            texture: square.clone(),
27            image_mode: SpriteImageMode::Auto,
28        },
29        Rect {
30            size: Vec2::new(100., 225.),
31            text: "Fill Center".to_string(),
32            transform: Transform::from_translation(Vec3::new(-450., 230., 0.)),
33            texture: square.clone(),
34            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillCenter),
35        },
36        Rect {
37            size: Vec2::new(100., 225.),
38            text: "Fill Start".to_string(),
39            transform: Transform::from_translation(Vec3::new(-330., 230., 0.)),
40            texture: square.clone(),
41            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillStart),
42        },
43        Rect {
44            size: Vec2::new(100., 225.),
45            text: "Fill End".to_string(),
46            transform: Transform::from_translation(Vec3::new(-210., 230., 0.)),
47            texture: square.clone(),
48            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillEnd),
49        },
50        Rect {
51            size: Vec2::new(300., 100.),
52            text: "Fill Start Horizontal".to_string(),
53            transform: Transform::from_translation(Vec3::new(10., 290., 0.)),
54            texture: square.clone(),
55            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillStart),
56        },
57        Rect {
58            size: Vec2::new(300., 100.),
59            text: "Fill End Horizontal".to_string(),
60            transform: Transform::from_translation(Vec3::new(10., 155., 0.)),
61            texture: square.clone(),
62            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillEnd),
63        },
64        Rect {
65            size: Vec2::new(200., 200.),
66            text: "Fill Center".to_string(),
67            transform: Transform::from_translation(Vec3::new(280., 230., 0.)),
68            texture: banner.clone(),
69            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillCenter),
70        },
71        Rect {
72            size: Vec2::new(200., 100.),
73            text: "Fill Center".to_string(),
74            transform: Transform::from_translation(Vec3::new(500., 230., 0.)),
75            texture: square.clone(),
76            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillCenter),
77        },
78        Rect {
79            size: Vec2::new(100., 100.),
80            text: "Stretched".to_string(),
81            transform: Transform::from_translation(Vec3::new(-570., -40., 0.)),
82            texture: banner.clone(),
83            image_mode: SpriteImageMode::Auto,
84        },
85        Rect {
86            size: Vec2::new(200., 200.),
87            text: "Fit Center".to_string(),
88            transform: Transform::from_translation(Vec3::new(-400., -40., 0.)),
89            texture: banner.clone(),
90            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
91        },
92        Rect {
93            size: Vec2::new(200., 200.),
94            text: "Fit Start".to_string(),
95            transform: Transform::from_translation(Vec3::new(-180., -40., 0.)),
96            texture: banner.clone(),
97            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitStart),
98        },
99        Rect {
100            size: Vec2::new(200., 200.),
101            text: "Fit End".to_string(),
102            transform: Transform::from_translation(Vec3::new(40., -40., 0.)),
103            texture: banner.clone(),
104            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitEnd),
105        },
106        Rect {
107            size: Vec2::new(100., 200.),
108            text: "Fit Center".to_string(),
109            transform: Transform::from_translation(Vec3::new(210., -40., 0.)),
110            texture: banner.clone(),
111            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
112        },
113    ];
114
115    for rect in rects {
116        commands.spawn((
117            Sprite {
118                image: rect.texture,
119                custom_size: Some(rect.size),
120                image_mode: rect.image_mode,
121                ..default()
122            },
123            rect.transform,
124            children![(
125                Text2d::new(rect.text),
126                TextLayout::justify(Justify::Center),
127                TextFont::from_font_size(15.),
128                Transform::from_xyz(0., -0.5 * rect.size.y - 10., 0.),
129                bevy::sprite::Anchor::TOP_CENTER,
130            )],
131        ));
132    }
133}
134
135fn setup_texture_atlas(
136    mut commands: Commands,
137    asset_server: Res<AssetServer>,
138    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
139) {
140    let gabe = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
141    let animation_indices_gabe = AnimationIndices { first: 0, last: 6 };
142    let gabe_atlas = TextureAtlas {
143        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
144            UVec2::splat(24),
145            7,
146            1,
147            None,
148            None,
149        )),
150        index: animation_indices_gabe.first,
151    };
152
153    let sprite_sheets = [
154        SpriteSheet {
155            size: Vec2::new(120., 50.),
156            text: "Stretched".to_string(),
157            transform: Transform::from_translation(Vec3::new(-570., -200., 0.)),
158            texture: gabe.clone(),
159            image_mode: SpriteImageMode::Auto,
160            atlas: gabe_atlas.clone(),
161            indices: animation_indices_gabe.clone(),
162            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
163        },
164        SpriteSheet {
165            size: Vec2::new(120., 50.),
166            text: "Fill Center".to_string(),
167            transform: Transform::from_translation(Vec3::new(-570., -300., 0.)),
168            texture: gabe.clone(),
169            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillCenter),
170            atlas: gabe_atlas.clone(),
171            indices: animation_indices_gabe.clone(),
172            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
173        },
174        SpriteSheet {
175            size: Vec2::new(120., 50.),
176            text: "Fill Start".to_string(),
177            transform: Transform::from_translation(Vec3::new(-430., -200., 0.)),
178            texture: gabe.clone(),
179            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillStart),
180            atlas: gabe_atlas.clone(),
181            indices: animation_indices_gabe.clone(),
182            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
183        },
184        SpriteSheet {
185            size: Vec2::new(120., 50.),
186            text: "Fill End".to_string(),
187            transform: Transform::from_translation(Vec3::new(-430., -300., 0.)),
188            texture: gabe.clone(),
189            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillEnd),
190            atlas: gabe_atlas.clone(),
191            indices: animation_indices_gabe.clone(),
192            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
193        },
194        SpriteSheet {
195            size: Vec2::new(50., 120.),
196            text: "Fill Center".to_string(),
197            transform: Transform::from_translation(Vec3::new(-300., -250., 0.)),
198            texture: gabe.clone(),
199            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillCenter),
200            atlas: gabe_atlas.clone(),
201            indices: animation_indices_gabe.clone(),
202            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
203        },
204        SpriteSheet {
205            size: Vec2::new(50., 120.),
206            text: "Fill Start".to_string(),
207            transform: Transform::from_translation(Vec3::new(-190., -250., 0.)),
208            texture: gabe.clone(),
209            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillStart),
210            atlas: gabe_atlas.clone(),
211            indices: animation_indices_gabe.clone(),
212            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
213        },
214        SpriteSheet {
215            size: Vec2::new(50., 120.),
216            text: "Fill End".to_string(),
217            transform: Transform::from_translation(Vec3::new(-90., -250., 0.)),
218            texture: gabe.clone(),
219            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FillEnd),
220            atlas: gabe_atlas.clone(),
221            indices: animation_indices_gabe.clone(),
222            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
223        },
224        SpriteSheet {
225            size: Vec2::new(120., 50.),
226            text: "Fit Center".to_string(),
227            transform: Transform::from_translation(Vec3::new(20., -200., 0.)),
228            texture: gabe.clone(),
229            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
230            atlas: gabe_atlas.clone(),
231            indices: animation_indices_gabe.clone(),
232            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
233        },
234        SpriteSheet {
235            size: Vec2::new(120., 50.),
236            text: "Fit Start".to_string(),
237            transform: Transform::from_translation(Vec3::new(20., -300., 0.)),
238            texture: gabe.clone(),
239            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitStart),
240            atlas: gabe_atlas.clone(),
241            indices: animation_indices_gabe.clone(),
242            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
243        },
244        SpriteSheet {
245            size: Vec2::new(120., 50.),
246            text: "Fit End".to_string(),
247            transform: Transform::from_translation(Vec3::new(160., -200., 0.)),
248            texture: gabe.clone(),
249            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitEnd),
250            atlas: gabe_atlas.clone(),
251            indices: animation_indices_gabe.clone(),
252            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
253        },
254    ];
255
256    for sprite_sheet in sprite_sheets {
257        commands.spawn((
258            Sprite {
259                image_mode: sprite_sheet.image_mode,
260                custom_size: Some(sprite_sheet.size),
261                ..Sprite::from_atlas_image(sprite_sheet.texture.clone(), sprite_sheet.atlas.clone())
262            },
263            sprite_sheet.indices,
264            sprite_sheet.timer,
265            sprite_sheet.transform,
266            children![(
267                Text2d::new(sprite_sheet.text),
268                TextLayout::justify(Justify::Center),
269                TextFont::from_font_size(15.),
270                Transform::from_xyz(0., -0.5 * sprite_sheet.size.y - 10., 0.),
271                bevy::sprite::Anchor::TOP_CENTER,
272            )],
273        ));
274    }
275}
```

examples/gizmos/anchored\_text\_gizmos.rs ([line 18](../../../src/anchored_text_gizmos/anchored_text_gizmos.rs.html#18))

```rust
17fn setup_camera(mut commands: Commands) {
18    commands.spawn(Camera2d);
19}
```

Additional examples can be found in:  

*   [examples/gizmos/text\_gizmos\_font.rs](../../../src/text_gizmos_font/text_gizmos_font.rs.html#25)
*   [examples/window/window\_resizing.rs](../../../src/window_resizing/window_resizing.rs.html#31)
*   [examples/state/computed\_states.rs](../../../src/computed_states/computed_states.rs.html#333)
*   [examples/state/sub\_states.rs](../../../src/sub_states/sub_states.rs.html#154)
*   [examples/app/without\_winit.rs](../../../src/without_winit/without_winit.rs.html#13)
*   [examples/window/custom\_cursor\_image.rs](../../../src/custom_cursor_image/custom_cursor_image.rs.html#67)
*   [examples/ecs/custom\_query\_param.rs](../../../src/custom_query_param/custom_query_param.rs.html#129)
*   [examples/ecs/system\_param.rs](../../../src/system_param/system_param.rs.html#37)
*   [examples/3d/irradiance\_volumes.rs](../../../src/irradiance_volumes/irradiance_volumes.rs.html#231)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#91)
*   [examples/ui/widgets/button.rs](../../../src/button/button.rs.html#73)
*   [examples/ui/widgets/standard\_widgets.rs](../../../src/standard_widgets/standard_widgets.rs.html#137)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../../src/standard_widgets_observers/standard_widgets_observers.rs.html#81)
*   [examples/ecs/one\_shot\_systems.rs](../../../src/one_shot_systems/one_shot_systems.rs.html#44)
*   [examples/audio/audio.rs](../../../src/audio/audio.rs.html#14-16)
*   [examples/asset/custom\_asset\_reader.rs](../../../src/custom_asset_reader/custom_asset_reader.rs.html#63)
*   [examples/window/transparent\_window.rs](../../../src/transparent_window/transparent_window.rs.html#34)
*   [examples/ecs/extraction.rs](../../../src/extraction/extraction.rs.html#75)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#51-54)
*   [examples/3d/mixed\_lighting.rs](../../../src/mixed_lighting/mixed_lighting.rs.html#159)
*   [examples/time/timers.rs](../../../src/timers/timers.rs.html#40-43)
*   [examples/2d/sprite.rs](../../../src/sprite/sprite.rs.html#13)
*   [examples/gizmos/2d\_text\_gizmos.rs](../../../src/2d_text_gizmos/2d_text_gizmos.rs.html#37)
*   [examples/asset/asset\_decompression.rs](../../../src/asset_decompression/asset_decompression.rs.html#107)
*   [examples/ecs/removal\_detection.rs](../../../src/removal_detection/removal_detection.rs.html#30)
*   [examples/3d/reflection\_probes.rs](../../../src/reflection_probes/reflection_probes.rs.html#113-118)
*   [examples/2d/move\_sprite.rs](../../../src/move_sprite/move_sprite.rs.html#20)
*   [examples/2d/rotate\_to\_cursor.rs](../../../src/rotate_to_cursor/rotate_to_cursor.rs.html#31)
*   [examples/3d/light\_probe\_blending.rs](../../../src/light_probe_blending/light_probe_blending.rs.html#248-257)
*   [examples/app/logs.rs](../../../src/logs/logs.rs.html#21)
*   [examples/audio/decodable.rs](../../../src/decodable/decodable.rs.html#100)
*   [examples/app/log\_layers\_ecs.rs](../../../src/log_layers_ecs/log_layers_ecs.rs.html#128)
*   [examples/ecs/change\_detection.rs](../../../src/change_detection/change_detection.rs.html#31)
*   [examples/usage/context\_menu.rs](../../../src/context_menu/context_menu.rs.html#61)
*   [examples/camera/pan\_camera\_controller.rs](../../../src/pan_camera_controller/pan_camera_controller.rs.html#24-32)
*   [examples/3d/fog.rs](../../../src/fog/fog.rs.html#43-53)
*   [examples/asset/multi\_asset\_sync.rs](../../../src/multi_asset_sync/multi_asset_sync.rs.html#176-185)
*   [examples/gltf/gltf\_extension\_mesh\_2d.rs](../../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#57-63)
*   [examples/audio/play\_sound\_effect.rs](../../../src/play_sound_effect/play_sound_effect.rs.html#31)
*   [examples/ecs/delayed\_commands.rs](../../../src/delayed_commands/delayed_commands.rs.html#22)
*   [examples/gizmos/3d\_text\_gizmos.rs](../../../src/3d_text_gizmos/3d_text_gizmos.rs.html#15-18)
*   [examples/2d/mesh2d.rs](../../../src/mesh2d/mesh2d.rs.html#17)
*   [examples/2d/dynamic\_mip\_generation.rs](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#317)
*   [examples/asset/web\_asset.rs](../../../src/web_asset/web_asset.rs.html#18)
*   [examples/ecs/component\_hooks.rs](../../../src/component_hooks/component_hooks.rs.html#147)
*   [examples/2d/sprite\_flipping.rs](../../../src/sprite_flipping/sprite_flipping.rs.html#13)
*   [examples/window/persisting\_window\_settings.rs](../../../src/persisting_window_settings/persisting_window_settings.rs.html#77)
*   [examples/window/screenshot.rs](../../../src/screenshot/screenshot.rs.html#26)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#176)
*   [examples/camera/2d\_screen\_shake.rs](../../../src/2d_screen_shake/2d_screen_shake.rs.html#165-175)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../../src/async_channel_pattern/async_channel_pattern.rs.html#77-81)
*   [examples/math/render\_primitives.rs](../../../src/render_primitives/render_primitives.rs.html#301)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#56-58)
*   [examples/ecs/generic\_system.rs](../../../src/generic_system/generic_system.rs.html#53-57)
*   [examples/stress\_tests/many\_gizmos.rs](../../../src/many_gizmos/many_gizmos.rs.html#83-86)
*   [examples/scene/world\_serialization.rs](../../../src/world_serialization/world_serialization.rs.html#126)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#22-25)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#96-104)
*   [examples/3d/mirror.rs](../../../src/mirror/mirror.rs.html#199-203)
*   [examples/3d/pccm.rs](../../../src/pccm/pccm.rs.html#85)
*   [examples/shader/animate\_shader.rs](../../../src/animate_shader/animate_shader.rs.html#24-28)
*   [examples/async\_tasks/async\_compute.rs](../../../src/async_compute/async_compute.rs.html#153)
*   [examples/ecs/state\_scoped.rs](../../../src/state_scoped/state_scoped.rs.html#42-57)
*   [examples/audio/pitch.rs](../../../src/pitch/pitch.rs.html#33-36)
*   [examples/ecs/observer\_propagation.rs](../../../src/observer_propagation/observer_propagation.rs.html#29)
*   [examples/2d/sprite\_tile.rs](../../../src/sprite_tile/sprite_tile.rs.html#22)
*   [examples/gltf/query\_gltf\_primitives.rs](../../../src/query_gltf_primitives/query_gltf_primitives.rs.html#55-58)
*   [examples/3d/lightmaps.rs](../../../src/lightmaps/lightmaps.rs.html#43-45)
*   [examples/shader/shader\_material\_2d.rs](../../../src/shader_material_2d/shader_material_2d.rs.html#32)
*   [examples/shader/shader\_material\_wesl.rs](../../../src/shader_material_wesl/shader_material_wesl.rs.html#57-64)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#208-213)
*   [examples/ecs/fallible\_params.rs](../../../src/fallible_params/fallible_params.rs.html#65)
*   [examples/3d/ssr.rs](../../../src/ssr/ssr.rs.html#265-273)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#59-63)
*   [examples/ecs/ecs\_guide.rs](../../../src/ecs_guide/ecs_guide.rs.html#228-234)
*   [examples/camera/2d\_top\_down\_camera.rs](../../../src/2d_top_down_camera/2d_top_down_camera.rs.html#37-40)
*   [examples/2d/pixel\_grid\_snap.rs](../../../src/pixel_grid_snap/pixel_grid_snap.rs.html#53-58)
*   [examples/shader/fallback\_image.rs](../../../src/fallback_image/fallback_image.rs.html#30-40)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../../src/custom_phase_item/custom_phase_item.rs.html#188-197)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../../src/fullscreen_material/fullscreen_material.rs.html#31-35)
*   [examples/3d/motion\_blur.rs](../../../src/motion_blur/motion_blur.rs.html#21-33)
*   [examples/ecs/hotpatching\_systems.rs](../../../src/hotpatching_systems/hotpatching_systems.rs.html#58)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#48-52)
*   [examples/camera/free\_camera\_controller.rs](../../../src/free_camera_controller/free_camera_controller.rs.html#77-90)
*   [examples/dev\_tools/fps\_overlay.rs](../../../src/fps_overlay/fps_overlay.rs.html#53)
*   [examples/3d/clustered\_decals.rs](../../../src/clustered_decals/clustered_decals.rs.html#187-197)
*   [examples/shader/shader\_material.rs](../../../src/shader_material/shader_material.rs.html#25-33)
*   [examples/shader/shader\_material\_glsl.rs](../../../src/shader_material_glsl/shader_material_glsl.rs.html#26-34)
*   [examples/gizmos/2d\_gizmos.rs](../../../src/2d_gizmos/2d_gizmos.rs.html#21)
*   [examples/asset/extra\_source.rs](../../../src/extra_asset_source/extra_source.rs.html#31)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../../src/texture_binding_array/texture_binding_array.rs.html#59-62)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../../src/scrollbars/scrollbars.rs.html#21)
*   [examples/ui/window\_fallthrough.rs](../../../src/window_fallthrough/window_fallthrough.rs.html#27)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#135-139)
*   [examples/asset/hot\_asset\_reloading.rs](../../../src/hot_asset_reloading/hot_asset_reloading.rs.html#26)
*   [examples/ecs/parallel\_query.rs](../../../src/parallel_query/parallel_query.rs.html#11)
*   [examples/shader\_advanced/manual\_material.rs](../../../src/manual_material/manual_material.rs.html#220-226)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#179-187)
*   [examples/audio/audio\_control.rs](../../../src/audio_control/audio_control.rs.html#17-20)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../../src/external_source_external_thread/external_source_external_thread.rs.html#27)
*   [examples/2d/transparency\_2d.rs](../../../src/transparency_2d/transparency_2d.rs.html#14)
*   [examples/showcase/loading\_screen.rs](../../../src/loading_screen/loading_screen.rs.html#86-93)
*   [examples/transforms/3d\_rotation.rs](../../../src/3d_rotation/3d_rotation.rs.html#27-32)
*   [examples/transforms/scale.rs](../../../src/scale/scale.rs.html#43-48)
*   [examples/ecs/entity\_disabling.rs](../../../src/entity_disabling/entity_disabling.rs.html#74-83)
*   [examples/shader\_advanced/custom\_vertex\_attribute.rs](../../../src/custom_vertex_attribute/custom_vertex_attribute.rs.html#44-50)
*   [examples/ui/text/font\_atlas\_debug.rs](../../../src/font_atlas_debug/font_atlas_debug.rs.html#53-61)
*   [examples/animation/morph\_targets.rs](../../../src/morph_targets/morph_targets.rs.html#38-44)
*   [tests/window/minimizing.rs](../../../src/minimizing/minimizing.rs.html#36-39)
*   [tests/window/resizing.rs](../../../src/resizing/resizing.rs.html#112-115)
*   [examples/3d/atmospheric\_fog.rs](../../../src/atmospheric_fog/atmospheric_fog.rs.html#27-40)
*   [examples/transforms/translation.rs](../../../src/translation/translation.rs.html#41-46)
*   [examples/shader/shader\_defs.rs](../../../src/shader_defs/shader_defs.rs.html#31-38)
*   [examples/gltf/load\_gltf\_extras.rs](../../../src/load_gltf_extras/load_gltf_extras.rs.html#20-23)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#62-69)
*   [examples/asset/embedded\_asset.rs](../../../src/embedded_asset/embedded_asset.rs.html#36)
*   [examples/3d/color\_grading.rs](../../../src/color_grading/color_grading.rs.html#139-157)
*   [examples/window/scale\_factor\_override.rs](../../../src/scale_factor_override/scale_factor_override.rs.html#28)
*   [examples/2d/sprite\_sheet.rs](../../../src/sprite_sheet/sprite_sheet.rs.html#53)
*   [examples/window/window\_drag\_move.rs](../../../src/window_drag_move/window_drag_move.rs.html#60)
*   [examples/ecs/callbacks.rs](../../../src/callbacks/callbacks.rs.html#42)
*   [examples/3d/tonemapping.rs](../../../src/tonemapping/tonemapping.rs.html#68-86)
*   [examples/3d/parenting.rs](../../../src/parenting/parenting.rs.html#38-49)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#227-230)
*   [examples/3d/post\_processing.rs](../../../src/post_processing/post_processing.rs.html#80-104)
*   [examples/3d/rotate\_environment\_map.rs](../../../src/rotate_environment_map/rotate_environment_map.rs.html#68-89)
*   [examples/app/settings.rs](../../../src/settings/settings.rs.html#62)
*   [examples/ui/text/editable\_text\_filter.rs](../../../src/editable_text_filter/editable_text_filter.rs.html#17)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#28-31)
*   [examples/3d/depth\_of\_field.rs](../../../src/depth_of_field/depth_of_field.rs.html#74-79)
*   [examples/ui/images/image\_node.rs](../../../src/image_node/image_node.rs.html#15)
*   [examples/camera/camera\_orbit.rs](../../../src/camera_orbit/camera_orbit.rs.html#51-55)
*   [examples/shader/shader\_material\_bindless.rs](../../../src/shader_material_bindless/shader_material_bindless.rs.html#58-65)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#22-25)
*   [examples/remote/server.rs](../../../src/server/server.rs.html#32-36)
*   [examples/audio/soundtrack.rs](../../../src/soundtrack/soundtrack.rs.html#84-92)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#126-129)
*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#101-104)
*   [examples/3d/animated\_material.rs](../../../src/animated_material/animated_material.rs.html#19-28)
*   [examples/3d/pcss.rs](../../../src/pcss/pcss.rs.html#160-167)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../../src/custom_post_processing/custom_post_processing.rs.html#248-261)
*   [examples/3d/clearcoat.rs](../../../src/clearcoat/clearcoat.rs.html#101-122)
*   [examples/camera/custom\_projection.rs](../../../src/custom_projection/custom_projection.rs.html#55-64)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#59-75)
*   [examples/picking/custom\_hit\_data.rs](../../../src/custom_hit_data/custom_hit_data.rs.html#81-86)
*   [examples/gltf/load\_gltf.rs](../../../src/load_gltf/load_gltf.rs.html#19-28)
*   [examples/3d/two\_passes.rs](../../../src/two_passes/two_passes.rs.html#19-22)
*   [examples/stress\_tests/many\_materials.rs](../../../src/many_materials/many_materials.rs.html#60-64)
*   [examples/diagnostics/log\_diagnostics.rs](../../../src/log_diagnostics/log_diagnostics.rs.html#64-68)
*   [examples/animation/animated\_mesh.rs](../../../src/animated_mesh/animated_mesh.rs.html#63)
*   [examples/stress\_tests/many\_glyphs.rs](../../../src/many_glyphs/many_glyphs.rs.html#67)
*   [examples/shader/extended\_material\_bindless.rs](../../../src/extended_material_bindless/extended_material_bindless.rs.html#115-134)
*   [examples/3d/clustered\_decal\_maps.rs](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#216-224)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../../src/custom_render_phase/custom_render_phase.rs.html#81-85)
*   [examples/dev\_tools/infinite\_grid.rs](../../../src/infinite_grid/infinite_grid.rs.html#30-35)
*   [examples/gltf/update\_gltf\_scene.rs](../../../src/update_gltf_scene/update_gltf_scene.rs.html#19-25)
*   [examples/stress\_tests/text\_pipeline.rs](../../../src/text_pipeline/text_pipeline.rs.html#37)
*   [examples/3d/skybox.rs](../../../src/skybox/skybox.rs.html#63-69)
*   [examples/3d/fog\_volumes.rs](../../../src/fog_volumes/fog_volumes.rs.html#34-44)
*   [examples/3d/lines.rs](../../../src/lines/lines.rs.html#28-39)
*   [examples/shader/storage\_buffer.rs](../../../src/storage_buffer/storage_buffer.rs.html#50-55)
*   [examples/window/low\_power.rs](../../../src/low_power/low_power.rs.html#173-177)
*   [examples/2d/mesh2d\_vertex\_color\_texture.rs](../../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#35)
*   [examples/3d/order\_independent\_transparency.rs](../../../src/order_independent_transparency/order_independent_transparency.rs.html#28-38)
*   [examples/shader/array\_texture.rs](../../../src/array_texture/array_texture.rs.html#49-52)
*   [examples/ecs/hierarchy.rs](../../../src/hierarchy/hierarchy.rs.html#20)
*   [examples/3d/generate\_custom\_mesh.rs](../../../src/generate_custom_mesh/generate_custom_mesh.rs.html#38-45)
*   [examples/2d/wireframe\_2d.rs](../../../src/wireframe_2d/wireframe_2d.rs.html#57-66)
*   [examples/3d/mesh\_ray\_cast.rs](../../../src/mesh_ray_cast/mesh_ray_cast.rs.html#88)
*   [examples/stress\_tests/many\_sprites.rs](../../../src/many_sprites/many_sprites.rs.html#69)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../../src/many_sprite_meshes/many_sprite_meshes.rs.html#71)
*   [examples/animation/color\_animation.rs](../../../src/color_animation/color_animation.rs.html#40)
*   [examples/audio/spatial\_audio\_2d.rs](../../../src/spatial_audio_2d/spatial_audio_2d.rs.html#36-43)
*   [examples/picking/simple\_picking.rs](../../../src/simple_picking/simple_picking.rs.html#18-26)
*   [examples/movement/smooth\_follow.rs](../../../src/smooth_follow/smooth_follow.rs.html#48-52)
*   [examples/2d/tilemap\_chunk.rs](../../../src/tilemap_chunk/tilemap_chunk.rs.html#44-60)
*   [examples/ui/relative\_cursor\_position.rs](../../../src/relative_cursor_position/relative_cursor_position.rs.html#14-25)
*   [examples/shader/extended\_material.rs](../../../src/extended_material/extended_material.rs.html#32-49)
*   [examples/ui/ui\_material.rs](../../../src/ui_material/ui_material.rs.html#26)
*   [examples/3d/vertex\_colors.rs](../../../src/vertex_colors/vertex_colors.rs.html#19-22)
*   [examples/animation/animated\_mesh\_control.rs](../../../src/animated_mesh_control/animated_mesh_control.rs.html#52-55)
*   [examples/animation/animation\_events.rs](../../../src/animation_events/animation_events.rs.html#43-53)
*   [examples/asset/generated\_assets.rs](../../../src/generated_assets/generated_assets.rs.html#19)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#57-77)
*   [tests/window/desktop\_request\_redraw.rs](../../../src/desktop_request_redraw/desktop_request_redraw.rs.html#44-47)
*   [examples/ui/text/font\_weights.rs](../../../src/font_weights/font_weights.rs.html#15)
*   [examples/3d/orthographic.rs](../../../src/orthographic/orthographic.rs.html#19-29)
*   [examples/showcase/contributors.rs](../../../src/contributors/contributors.rs.html#112-130)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#117)
*   [examples/picking/debug\_picking.rs](../../../src/debug_picking/debug_picking.rs.html#39-47)
*   [examples/remote/app\_under\_test.rs](../../../src/app_under_test/app_under_test.rs.html#74)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#83-90)
*   [examples/window/multiple\_windows.rs](../../../src/multiple_windows/multiple_windows.rs.html#15-17)
*   [examples/ui/ui\_scaling.rs](../../../src/ui_scaling/ui_scaling.rs.html#26)
*   [examples/3d/spherical\_area\_lights.rs](../../../src/spherical_area_lights/spherical_area_lights.rs.html#22-25)
*   [examples/math/bounding\_2d.rs](../../../src/bounding_2d/bounding_2d.rs.html#203)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#135-145)
*   [examples/usage/cooldown.rs](../../../src/cooldown/cooldown.rs.html#27)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../../src/many_morph_targets/many_morph_targets.rs.html#199-202)
*   [examples/transforms/transform.rs](../../../src/transform/transform.rs.html#47-56)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../../src/ui_texture_atlas/ui_texture_atlas.rs.html#24)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../../src/many_animated_sprites/many_animated_sprites.rs.html#68)
*   [examples/2d/bloom\_2d.rs](../../../src/bloom_2d/bloom_2d.rs.html#23-32)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#70)
*   [examples/stress\_tests/many\_text2d.rs](../../../src/many_text2d/many_text2d.rs.html#119)
*   [examples/ui/text/font\_variations.rs](../../../src/font_variations/font_variations.rs.html#16)
*   [examples/camera/first\_person\_view\_model.rs](../../../src/first_person_view_model/first_person_view_model.rs.html#107-147)
*   [examples/ecs/observers.rs](../../../src/observers/observers.rs.html#99)
*   [examples/gizmos/axes.rs](../../../src/axes/axes.rs.html#50-56)
*   [examples/app/headless\_renderer.rs](../../../src/headless_renderer/headless_renderer.rs.html#174-178)
*   [examples/ecs/relationships.rs](../../../src/relationships/relationships.rs.html#52)
*   [examples/3d/occlusion\_culling.rs](../../../src/occlusion_culling/occlusion_culling.rs.html#275)
*   [examples/3d/ssao.rs](../../../src/ssao/ssao.rs.html#30-37)
*   [examples/animation/animated\_ui.rs](../../../src/animated_ui/animated_ui.rs.html#140)
*   [examples/ui/layout/ghost\_nodes.rs](../../../src/ghost_nodes/ghost_nodes.rs.html#28)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../../src/overflow_debug/overflow_debug.rs.html#79)
*   [examples/3d/atmosphere.rs](../../../src/atmosphere/atmosphere.rs.html#142)
*   [examples/app/render\_recovery.rs](../../../src/render_recovery/render_recovery.rs.html#40-44)
*   [examples/stress\_tests/many\_buttons.rs](../../../src/many_buttons/many_buttons.rs.html#92)
*   [examples/animation/eased\_motion.rs](../../../src/eased_motion/eased_motion.rs.html#40-47)
*   [examples/stress\_tests/many\_gradients.rs](../../../src/many_gradients/many_gradients.rs.html#80)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../../src/transform_hierarchy/transform_hierarchy.rs.html#274)
*   [examples/audio/spatial\_audio\_3d.rs](../../../src/spatial_audio_3d/spatial_audio_3d.rs.html#28-35)
*   [examples/shader/automatic\_instancing.rs](../../../src/automatic_instancing/automatic_instancing.rs.html#58-66)
*   [examples/asset/alter\_sprite.rs](../../../src/alter_sprite/alter_sprite.rs.html#51)
*   [examples/ui/text/text\_input.rs](../../../src/text_input/text_input.rs.html#48)
*   [examples/ecs/error\_handling.rs](../../../src/error_handling/error_handling.rs.html#68-72)
*   [examples/camera/projection\_zoom.rs](../../../src/projection_zoom/projection_zoom.rs.html#50-65)
*   [examples/2d/rotation.rs](../../../src/rotation/rotation.rs.html#57)
*   [examples/3d/rect\_light.rs](../../../src/rect_light/rect_light.rs.html#42-45)
*   [examples/ui/images/ui\_texture\_slice.rs](../../../src/ui_texture_slice/ui_texture_slice.rs.html#54)
*   [examples/3d/specular\_tint.rs](../../../src/specular_tint/specular_tint.rs.html#83-100)
*   [examples/ui/widgets/viewport\_node.rs](../../../src/viewport_node/viewport_node.rs.html#32)
*   [examples/camera/2d\_on\_ui.rs](../../../src/2d_on_ui/2d_on_ui.rs.html#15)
*   [examples/3d/visibility\_range.rs](../../../src/visibility_range/visibility_range.rs.html#120-123)
*   [examples/2d/sprite\_animation.rs](../../../src/sprite_animation/sprite_animation.rs.html#90)
*   [examples/math/cubic\_splines.rs](../../../src/cubic_splines/cubic_splines.rs.html#69)
*   [examples/ui/text/ime\_support.rs](../../../src/ime_support/ime_support.rs.html#31)
*   [examples/window/monitor\_info.rs](../../../src/monitor_info/monitor_info.rs.html#41-49)
*   [examples/3d/volumetric\_fog.rs](../../../src/volumetric_fog/volumetric_fog.rs.html#63-65)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#60-75)
*   [examples/2d/cpu\_draw.rs](../../../src/cpu_draw/cpu_draw.rs.html#39)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#39-47)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../../src/many_cameras_lights/many_cameras_lights.rs.html#41-45)
*   [examples/3d/texture.rs](../../../src/texture/texture.rs.html#56-60)
*   [examples/time/virtual\_time.rs](../../../src/virtual_time/virtual_time.rs.html#48)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#37)
*   [examples/3d/shadow\_caster\_receiver.rs](../../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#43-47)
*   [examples/3d/wireframe.rs](../../../src/wireframe/wireframe.rs.html#62-67)
*   [examples/3d/bloom\_3d.rs](../../../src/bloom_3d/bloom_3d.rs.html#27-36)
*   [examples/stress\_tests/many\_lights.rs](../../../src/many_lights/many_lights.rs.html#53-57)
*   [examples/ui/layout/size\_constraints.rs](../../../src/size_constraints/size_constraints.rs.html#43)
*   [examples/3d/anti\_aliasing.rs](../../../src/anti_aliasing/anti_aliasing.rs.html#420-423)
*   [examples/ui/text/text\_background\_colors.rs](../../../src/text_background_colors/text_background_colors.rs.html#26)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#154)
*   [examples/ui/text/generic\_font\_families.rs](../../../src/generic_font_families/generic_font_families.rs.html#35)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#40-50)
*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#34)
*   [examples/transforms/align.rs](../../../src/align/align.rs.html#56-59)
*   [examples/ui/styling/transparency\_ui.rs](../../../src/transparency_ui/transparency_ui.rs.html#15)
*   [examples/ui/layout/anchor\_layout.rs](../../../src/anchor_layout/anchor_layout.rs.html#21)
*   [examples/3d/decal.rs](../../../src/decal/decal.rs.html#28-41)
*   [examples/math/random\_sampling.rs](../../../src/random_sampling/random_sampling.rs.html#61-65)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#134-142)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#64)
*   [examples/picking/sprite\_picking.rs](../../../src/sprite_picking/sprite_picking.rs.html#32)
*   [examples/showcase/stepping.rs](../../../src/breakout/stepping.rs.html#170-183)
*   [examples/2d/mesh2d\_alpha\_mode.rs](../../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#23)
*   [examples/2d/mesh2d\_arcs.rs](../../../src/mesh2d_arcs/mesh2d_arcs.rs.html#42-48)
*   [examples/3d/scrolling\_fog.rs](../../../src/scrolling_fog/scrolling_fog.rs.html#48-59)
*   [examples/ecs/iter\_combinations.rs](../../../src/iter_combinations/iter_combinations.rs.html#65-89)
*   [examples/shader\_advanced/compute\_mesh.rs](../../../src/compute_mesh/compute_mesh.rs.html#120-128)
*   [examples/ui/text/text\_wrap\_debug.rs](../../../src/text_wrap_debug/text_wrap_debug.rs.html#45)
*   [examples/3d/render\_to\_texture.rs](../../../src/render_to_texture/render_to_texture.rs.html#52-58)
*   [examples/2d/mesh2d\_manual.rs](../../../src/mesh2d_manual/mesh2d_manual.rs.html#114-119)
*   [examples/2d/sprite\_slice.rs](../../../src/sprite_slice/sprite_slice.rs.html#86-101)
*   [examples/ui/styling/stacked\_gradients.rs](../../../src/stacked_gradients/stacked_gradients.rs.html#17)
*   [examples/gizmos/transform\_gizmo.rs](../../../src/transform_gizmo/transform_gizmo.rs.html#37-48)
*   [examples/3d/transparency\_3d.rs](../../../src/transparency_3d/transparency_3d.rs.html#21-24)
*   [examples/asset/repeated\_texture.rs](../../../src/repeated_texture/repeated_texture.rs.html#27-34)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#90-98)
*   [examples/asset/asset\_settings.rs](../../../src/asset_settings/asset_settings.rs.html#26-33)
*   [examples/shader/gpu\_readback.rs](../../../src/gpu_readback/gpu_readback.rs.html#102)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#51-62)
*   [examples/3d/spotlight.rs](../../../src/spotlight/spotlight.rs.html#44-48)
*   [examples/asset/alter\_mesh.rs](../../../src/alter_mesh/alter_mesh.rs.html#106-113)
*   [examples/3d/meshlet.rs](../../../src/meshlet/meshlet.rs.html#40-51)
*   [examples/ui/text/system\_fonts.rs](../../../src/system_fonts/system_fonts.rs.html#27)
*   [examples/3d/pbr.rs](../../../src/pbr/pbr.rs.html#28-38)
*   [examples/ui/ui\_target\_camera.rs](../../../src/ui_target_camera/ui_target_camera.rs.html#21-30)
*   [examples/math/custom\_primitives.rs](../../../src/custom_primitives/custom_primitives.rs.html#177)
*   [examples/3d/auto\_exposure.rs](../../../src/auto_exposure/auto_exposure.rs.html#41-53)
*   [examples/animation/easing\_functions.rs](../../../src/easing_functions/easing_functions.rs.html#22)
*   [examples/asset/asset\_loading.rs](../../../src/asset_loading/asset_loading.rs.html#76-80)
*   [examples/window/multi\_window\_text.rs](../../../src/multi_window_text/multi_window_text.rs.html#30)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../../src/overflow/overflow.rs.html#14)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../../src/overflow_clip_margin/overflow_clip_margin.rs.html#13)
*   [examples/gizmos/light\_gizmos.rs](../../../src/light_gizmos/light_gizmos.rs.html#46-50)
*   [examples/ui/layout/flex\_layout.rs](../../../src/flex_layout/flex_layout.rs.html#23)
*   [examples/stress\_tests/bevymark\_3d.rs](../../../src/bevymark_3d/bevymark_3d.rs.html#219-222)
*   [examples/shader/shader\_prepass.rs](../../../src/shader_prepass/shader_prepass.rs.html#47-59)
*   [examples/ui/widgets/vertical\_slider.rs](../../../src/vertical_slider/vertical_slider.rs.html#37)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#258)
*   [examples/ui/layout/display\_and\_visibility.rs](../../../src/display_and_visibility/display_and_visibility.rs.html#82)
*   [examples/3d/shadow\_biases.rs](../../../src/shadow_biases/shadow_biases.rs.html#48-65)
*   [examples/ui/widgets/tab\_navigation.rs](../../../src/tab_navigation/tab_navigation.rs.html#70)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../../src/drag_to_scroll/drag_to_scroll.rs.html#26)
*   [examples/3d/split\_screen.rs](../../../src/split_screen/split_screen.rs.html#25-28)
*   [examples/2d/2d\_shapes.rs](../../../src/2d_shapes/2d_shapes.rs.html#52)
*   [examples/showcase/breakout.rs](../../../src/breakout/breakout.rs.html#179)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../../src/scroll/scroll.rs.html#112)
*   [examples/picking/mesh\_picking.rs](../../../src/mesh_picking/mesh_picking.rs.html#81-91)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#86-94)
*   [examples/ui/layout/z\_index.rs](../../../src/z_index/z_index.rs.html#20)
*   [examples/ui/render\_ui\_to\_texture.rs](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#62)
*   [examples/ui/images/image\_node\_resizing.rs](../../../src/image_node_resizing/image_node_resizing.rs.html#69)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#16)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#106)
*   [examples/3d/parallax\_mapping.rs](../../../src/parallax_mapping/parallax_mapping.rs.html#219-223)
*   [examples/ui/navigation/directional\_navigation.rs](../../../src/directional_navigation/directional_navigation.rs.html#115)
*   [examples/2d/texture\_atlas.rs](../../../src/texture_atlas/texture_atlas.rs.html#96)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#162-167)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../../src/multiple_text_inputs/multiple_text_inputs.rs.html#44)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#17)
*   [examples/3d/deferred\_rendering.rs](../../../src/deferred_rendering/deferred_rendering.rs.html#35-58)
*   [examples/ui/styling/box\_shadow.rs](../../../src/box_shadow/box_shadow.rs.html#145)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#110-136)
*   [examples/ui/text/text.rs](../../../src/text/text.rs.html#31)
*   [examples/2d/text2d.rs](../../../src/text2d/text2d.rs.html#44)
*   [examples/3d/3d\_shapes.rs](../../../src/3d_shapes/3d_shapes.rs.html#135-146)
*   [examples/animation/custom\_skinned\_mesh.rs](../../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#46-49)
*   [examples/usage/debug\_frustum\_culling.rs](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#112-116)
*   [examples/3d/blend\_modes.rs](../../../src/blend_modes/blend_modes.rs.html#38-50)
*   [examples/animation/animated\_transform.rs](../../../src/animated_transform/animated_transform.rs.html#30-33)
*   [examples/3d/lighting.rs](../../../src/lighting/lighting.rs.html#50-57)
*   [examples/ui/styling/borders.rs](../../../src/borders/borders.rs.html#13)
*   [examples/ui/text/letter\_spacing.rs](../../../src/letter_spacing/letter_spacing.rs.html#31)
*   [examples/3d/camera\_sub\_view.rs](../../../src/camera_sub_view/camera_sub_view.rs.html#34-37)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#165)
*   [examples/ui/text/text\_debug.rs](../../../src/text_debug/text_debug.rs.html#36)
*   [examples/ui/layout/grid.rs](../../../src/grid/grid.rs.html#20)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#73-87)
*   [examples/ui/ui\_transform.rs](../../../src/ui_transform/ui_transform.rs.html#104)
*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#197-203)
*   [examples/ui/text/font\_query.rs](../../../src/font_query/font_query.rs.html#17)
*   [examples/ui/text/multiline\_text\_input.rs](../../../src/multiline_text_input/multiline_text_input.rs.html#29)
*   [examples/ui/styling/gradients.rs](../../../src/gradients/gradients.rs.html#28)
*   [examples/testbed/full\_ui.rs](../../../src/testbed_full_ui/full_ui.rs.html#33)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#439)

#### pub fn [entity](#method.entity)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Returns the [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") for the given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

This method does not guarantee that commands queued by the returned `EntityCommands` will be successful, since the entity could be despawned before they are executed.

##### Example

```rust
#[derive(Resource)]
struct PlayerEntity {
    entity: Entity
}

#[derive(Component)]
struct Label(&'static str);

fn example_system(mut commands: Commands, player: Res<PlayerEntity>) {
    // Get the entity and add a component.
    commands.entity(player.entity).insert(Label("hello world"));
}
```

##### See also

*   [`get_entity`](../../prelude/struct.Commands.html#method.get_entity "method bevy::prelude::Commands::get_entity") for the fallible version.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/state/custom\_transitions.rs ([line 166](../../../src/custom_transitions/custom_transitions.rs.html#166))

```rust
165fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
166    commands.entity(menu_data.button_entity).despawn();
167}
168
169const SPEED: f32 = 100.0;
170fn movement(
171    time: Res<Time>,
172    input: Res<ButtonInput<KeyCode>>,
173    mut query: Query<&mut Transform, With<Sprite>>,
174) {
175    for mut transform in &mut query {
176        let mut direction = Vec3::ZERO;
177        if input.pressed(KeyCode::ArrowLeft) {
178            direction.x -= 1.0;
179        }
180        if input.pressed(KeyCode::ArrowRight) {
181            direction.x += 1.0;
182        }
183        if input.pressed(KeyCode::ArrowUp) {
184            direction.y += 1.0;
185        }
186        if input.pressed(KeyCode::ArrowDown) {
187            direction.y -= 1.0;
188        }
189
190        if direction != Vec3::ZERO {
191            transform.translation += direction.normalize() * SPEED * time.delta_secs();
192        }
193    }
194}
195
196fn change_color(time: Res<Time>, mut query: Query<&mut Sprite>) {
197    for mut sprite in &mut query {
198        let new_color = LinearRgba {
199            blue: ops::sin(time.elapsed_secs() * 0.5) + 2.0,
200            ..LinearRgba::from(sprite.color)
201        };
202
203        sprite.color = new_color.into();
204    }
205}
206
207// We can restart the game by pressing "R".
208// This will trigger an [`AppState::InGame`] -> [`AppState::InGame`]
209// transition, which will run our custom schedules.
210fn trigger_game_restart(
211    input: Res<ButtonInput<KeyCode>>,
212    mut next_state: ResMut<NextState<AppState>>,
213) {
214    if input.just_pressed(KeyCode::KeyR) {
215        // Although we are already in this state setting it again will generate an identity transition.
216        // While default schedules ignore those kinds of transitions, our custom schedules will react to them.
217        next_state.set(AppState::InGame);
218    }
219}
220
221fn setup(mut commands: Commands) {
222    commands.spawn(Camera2d);
223}
224
225fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
226    commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
227    info!("Setup game");
228}
229
230fn teardown_game(mut commands: Commands, player: Single<Entity, With<Sprite>>) {
231    commands.entity(*player).despawn();
232    info!("Teardown game");
233}
```

Hide additional examples

examples/state/states.rs ([line 118](../../../src/states/states.rs.html#118))

```rust
117fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
118    commands.entity(menu_data.button_entity).despawn();
119}
```

examples/state/sub\_states.rs ([line 87](../../../src/sub_states/sub_states.rs.html#87))

```rust
86fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
87    commands.entity(menu_data.button_entity).despawn();
88}
```

examples/remote/server.rs ([line 86](../../../src/server/server.rs.html#86))

```rust
85fn remove(mut commands: Commands, cube_entity: Single<Entity, With<Cube>>) {
86    commands.entity(*cube_entity).remove::<Cube>();
87}
```

examples/state/computed\_states.rs ([line 406](../../../src/computed_states/computed_states.rs.html#406))

```rust
405    pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
406        commands.entity(menu_data.root_entity).despawn();
407    }
```

examples/stress\_tests/many\_buttons.rs ([line 322](../../../src/many_buttons/many_buttons.rs.html#322))

```rust
321fn despawn_ui(mut commands: Commands, root_node: Single<Entity, (With<Node>, Without<ChildOf>)>) {
322    commands.entity(*root_node).despawn();
323}
```

Additional examples can be found in:  

*   [examples/ecs/generic\_system.rs](../../../src/generic_system/generic_system.rs.html#87)
*   [examples/usage/context\_menu.rs](../../../src/context_menu/context_menu.rs.html#77)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#385)
*   [examples/showcase/loading\_screen.rs](../../../src/loading_screen/loading_screen.rs.html#128)
*   [examples/ecs/contiguous\_query.rs](../../../src/contiguous_query/contiguous_query.rs.html#43)
*   [examples/ecs/removal\_detection.rs](../../../src/removal_detection/removal_detection.rs.html#47)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../../src/standard_widgets_observers/standard_widgets_observers.rs.html#453)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../../src/external_source_external_thread/external_source_external_thread.rs.html#71)
*   [examples/3d/volumetric\_fog.rs](../../../src/volumetric_fog/volumetric_fog.rs.html#163)
*   [examples/asset/multi\_asset\_sync.rs](../../../src/multi_asset_sync/multi_asset_sync.rs.html#274)
*   [examples/ecs/one\_shot\_systems.rs](../../../src/one_shot_systems/one_shot_systems.rs.html#64)
*   [examples/gizmos/transform\_gizmo.rs](../../../src/transform_gizmo/transform_gizmo.rs.html#132)
*   [examples/ecs/extraction.rs](../../../src/extraction/extraction.rs.html#120)
*   [examples/window/screenshot.rs](../../../src/screenshot/screenshot.rs.html#38)
*   [examples/3d/reflection\_probes.rs](../../../src/reflection_probes/reflection_probes.rs.html#197)
*   [examples/3d/depth\_of\_field.rs](../../../src/depth_of_field/depth_of_field.rs.html#170)
*   [examples/ui/widgets/tab\_navigation.rs](../../../src/tab_navigation/tab_navigation.rs.html#56)
*   [examples/ecs/observer\_propagation.rs](../../../src/observer_propagation/observer_propagation.rs.html#117)
*   [examples/ecs/entity\_disabling.rs](../../../src/entity_disabling/entity_disabling.rs.html#50)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#225)
*   [examples/3d/pbr.rs](../../../src/pbr/pbr.rs.html#142)
*   [examples/asset/asset\_decompression.rs](../../../src/asset_decompression/asset_decompression.rs.html#129)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#290)
*   [examples/animation/morph\_targets.rs](../../../src/morph_targets/morph_targets.rs.html#71)
*   [examples/app/log\_layers\_ecs.rs](../../../src/log_layers_ecs/log_layers_ecs.rs.html#151)
*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#147)
*   [examples/animation/animated\_mesh\_control.rs](../../../src/animated_mesh_control/animated_mesh_control.rs.html#160)
*   [examples/ecs/error\_handling.rs](../../../src/error_handling/error_handling.rs.html#178)
*   [examples/window/window\_settings.rs](../../../src/window_settings/window_settings.rs.html#178)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#360)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#86)
*   [examples/ui/widgets/standard\_widgets.rs](../../../src/standard_widgets/standard_widgets.rs.html#117)
*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#169)
*   [examples/3d/clearcoat.rs](../../../src/clearcoat/clearcoat.rs.html#283)
*   [examples/3d/clustered\_decals.rs](../../../src/clustered_decals/clustered_decals.rs.html#512)
*   [examples/showcase/game\_menu.rs](../../../src/game_menu/game_menu.rs.html#355)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#83)
*   [examples/usage/cooldown.rs](../../../src/cooldown/cooldown.rs.html#147)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#113)
*   [examples/3d/order\_independent\_transparency.rs](../../../src/order_independent_transparency/order_independent_transparency.rs.html#83)
*   [examples/3d/tonemapping.rs](../../../src/tonemapping/tonemapping.rs.html#221)
*   [examples/3d/ssr.rs](../../../src/ssr/ssr.rs.html#319)
*   [examples/ecs/delayed\_commands.rs](../../../src/delayed_commands/delayed_commands.rs.html#53)
*   [examples/3d/pccm.rs](../../../src/pccm/pccm.rs.html#187)
*   [examples/3d/irradiance\_volumes.rs](../../../src/irradiance_volumes/irradiance_volumes.rs.html#431)
*   [examples/3d/shadow\_caster\_receiver.rs](../../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#145)
*   [examples/3d/mirror.rs](../../../src/mirror/mirror.rs.html#640)
*   [examples/audio/soundtrack.rs](../../../src/soundtrack/soundtrack.rs.html#72)
*   [examples/ecs/fallible\_params.rs](../../../src/fallible_params/fallible_params.rs.html#118)
*   [examples/showcase/stepping.rs](../../../src/breakout/stepping.rs.html#250)
*   [examples/3d/pcss.rs](../../../src/pcss/pcss.rs.html#296)
*   [examples/window/custom\_cursor\_image.rs](../../../src/custom_cursor_image/custom_cursor_image.rs.html#43)
*   [examples/ecs/hierarchy.rs](../../../src/hierarchy/hierarchy.rs.html#58)
*   [examples/2d/dynamic\_mip\_generation.rs](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#568)
*   [examples/3d/lightmaps.rs](../../../src/lightmaps/lightmaps.rs.html#76)
*   [tests/window/desktop\_request\_redraw.rs](../../../src/desktop_request_redraw/desktop_request_redraw.rs.html#87)
*   [examples/animation/animated\_mesh.rs](../../../src/animated_mesh/animated_mesh.rs.html#93)
*   [examples/3d/visibility\_range.rs](../../../src/visibility_range/visibility_range.rs.html#214)
*   [examples/math/bounding\_2d.rs](../../../src/bounding_2d/bounding_2d.rs.html#163)
*   [examples/ecs/relationships.rs](../../../src/relationships/relationships.rs.html#71)
*   [examples/3d/occlusion\_culling.rs](../../../src/occlusion_culling/occlusion_culling.rs.html#301)
*   [examples/animation/eased\_motion.rs](../../../src/eased_motion/eased_motion.rs.html#51)
*   [examples/3d/clustered\_decal\_maps.rs](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#373)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#262)
*   [examples/ui/text/text\_input.rs](../../../src/text_input/text_input.rs.html#110)
*   [examples/showcase/breakout.rs](../../../src/breakout/breakout.rs.html#357)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#95)
*   [examples/window/monitor\_info.rs](../../../src/monitor_info/monitor_info.rs.html#75)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#188)
*   [examples/async\_tasks/async\_compute.rs](../../../src/async_compute/async_compute.rs.html#113)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#335)
*   [examples/3d/blend\_modes.rs](../../../src/blend_modes/blend_modes.rs.html#304)
*   [examples/3d/light\_probe\_blending.rs](../../../src/light_probe_blending/light_probe_blending.rs.html#532)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../../src/many_morph_targets/many_morph_targets.rs.html#327)
*   [examples/math/random\_sampling.rs](../../../src/random_sampling/random_sampling.rs.html#150)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../../src/transform_hierarchy/transform_hierarchy.rs.html#430)
*   [examples/ui/text/text\_wrap\_debug.rs](../../../src/text_wrap_debug/text_wrap_debug.rs.html#116)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#385)
*   [examples/3d/deferred\_rendering.rs](../../../src/deferred_rendering/deferred_rendering.rs.html#302)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#393)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#828)
*   [examples/3d/mixed\_lighting.rs](../../../src/mixed_lighting/mixed_lighting.rs.html#266)
*   [examples/diagnostics/log\_diagnostics.rs](../../../src/log_diagnostics/log_diagnostics.rs.html#196)
*   [examples/ecs/component\_hooks.rs](../../../src/component_hooks/component_hooks.rs.html#131)
*   [examples/3d/ssao.rs](../../../src/ssao/ssao.rs.html#110)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../../src/scroll/scroll.rs.html#178)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#190)
*   [examples/3d/bloom\_3d.rs](../../../src/bloom_3d/bloom_3d.rs.html#140)
*   [examples/ui/navigation/directional\_navigation.rs](../../../src/directional_navigation/directional_navigation.rs.html#255)
*   [examples/2d/bloom\_2d.rs](../../../src/bloom_2d/bloom_2d.rs.html#115)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#179)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#199)
*   [examples/3d/anti\_aliasing.rs](../../../src/anti_aliasing/anti_aliasing.rs.html#98)
*   [examples/animation/custom\_skinned\_mesh.rs](../../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#165)
*   [examples/animation/animated\_transform.rs](../../../src/animated_transform/animated_transform.rs.html#154)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#464)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#198)
*   [examples/ui/text/text\_debug.rs](../../../src/text_debug/text_debug.rs.html#244)
*   [examples/ui/widgets/feathers\_gallery.rs](../../../src/feathers_gallery/feathers_gallery.rs.html#285)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#490)

#### pub fn [get\_entity](#method.get_entity)( &mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>, [InvalidEntityError](../entity/struct.InvalidEntityError.html "struct bevy::ecs::entity::InvalidEntityError")\>

Returns the [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") for the requested [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") if it is valid. This method does not guarantee that commands queued by the returned `EntityCommands` will be successful, since the entity could be despawned before they are executed. This also does not error when the entity has not been spawned. For that behavior, see [`get_spawned_entity`](../../prelude/struct.Commands.html#method.get_spawned_entity "method bevy::prelude::Commands::get_spawned_entity"), which should be preferred for accessing entities you expect to already be spawned, like those found from a query. For details on entity spawning vs validity, see [`entity`](../entity/index.html "mod bevy::ecs::entity") module docs.

##### Errors

Returns [`InvalidEntityError`](../entity/struct.InvalidEntityError.html "struct bevy::ecs::entity::InvalidEntityError") if the requested entity does not exist.

##### Example

```rust
#[derive(Resource)]
struct PlayerEntity {
    entity: Entity
}

#[derive(Component)]
struct Label(&'static str);

fn example_system(mut commands: Commands, player: Res<PlayerEntity>) -> Result {
    // Get the entity if it still exists and store the `EntityCommands`.
    // If it doesn't exist, the `?` operator will propagate the returned error
    // to the system, and the system will pass it to an error handler.
    let mut entity_commands = commands.get_entity(player.entity)?;

    // Add a component to the entity.
    entity_commands.insert(Label("hello world"));

    // Return from the system successfully.
    Ok(())
}
```

##### See also

*   [`entity`](../../prelude/struct.Commands.html#method.entity "method bevy::prelude::Commands::entity") for the infallible version.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/ecs/observers.rs ([line 165](../../../src/observers/observers.rs.html#165))

```rust
163fn explode_mine(explode: On<Explode>, query: Query<&Mine>, mut commands: Commands) {
164    // Explode is an EntityEvent. `explode.entity` is the entity that Explode was triggered for.
165    let Ok(mut entity) = commands.get_entity(explode.entity) else {
166        return;
167    };
168    info!("Boom! {} exploded.", explode.entity);
169    entity.despawn();
170    let mine = query.get(explode.entity).unwrap();
171    // Trigger another explosion cascade.
172    commands.trigger(ExplodeMines {
173        pos: mine.pos,
174        radius: mine.size,
175    });
176}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#544-547)

#### pub fn [get\_spawned\_entity](#method.get_spawned_entity)( &mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>, [EntityNotSpawnedError](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

Returns the [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") for the requested [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") if it spawned in the world _now_. Note that for entities that have not been spawned _yet_, like ones from [`spawn`](../../prelude/struct.Commands.html#method.spawn "method bevy::prelude::Commands::spawn"), this will error. If that is not desired, try [`get_entity`](../../prelude/struct.Commands.html#method.get_entity "method bevy::prelude::Commands::get_entity"). This should be used over [`get_entity`](../../prelude/struct.Commands.html#method.get_entity "method bevy::prelude::Commands::get_entity") when you expect the entity to already be spawned in the world. If the entity is valid but not yet spawned, this will error that information, where [`get_entity`](../../prelude/struct.Commands.html#method.get_entity "method bevy::prelude::Commands::get_entity") would succeed, leading to potentially surprising results. For details on entity spawning vs validity, see [`entity`](../entity/index.html "mod bevy::ecs::entity") module docs.

This method does not guarantee that commands queued by the returned `EntityCommands` will be successful, since the entity could be despawned before they are executed.

##### Errors

Returns [`EntityNotSpawnedError`](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError") if the requested entity does not exist.

##### Example

```rust
#[derive(Resource)]
struct PlayerEntity {
    entity: Entity
}

#[derive(Component)]
struct Label(&'static str);

fn example_system(mut commands: Commands, player: Res<PlayerEntity>) -> Result {
    // Get the entity if it still exists and store the `EntityCommands`.
    // If it doesn't exist, the `?` operator will propagate the returned error
    // to the system, and the system will pass it to an error handler.
    let mut entity_commands = commands.get_spawned_entity(player.entity)?;

    // Add a component to the entity.
    entity_commands.insert(Label("hello world"));

    // Return from the system successfully.
    Ok(())
}
```

##### See also

*   [`entity`](../../prelude/struct.Commands.html#method.entity "method bevy::prelude::Commands::entity") for the infallible version.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#587-590)

#### pub fn [spawn\_batch](#method.spawn_batch)<I>(&mut self, batch: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item"): [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <<I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item") as [DynamicBundle](../bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

Spawns multiple entities with the same combination of components, based on a batch of [`Bundles`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle").

A batch can be any type that implements [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") and contains bundles, such as a [`Vec<Bundle>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") or an array `[Bundle; N]`.

This method is equivalent to iterating the batch and calling [`spawn`](../../prelude/struct.Commands.html#method.spawn "method bevy::prelude::Commands::spawn") for each bundle, but is faster by pre-allocating memory and having exclusive [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") access.

##### Example

```rust
use bevy_ecs::prelude::*;

#[derive(Component)]
struct Score(u32);

fn example_system(mut commands: Commands) {
    commands.spawn_batch([
        (Name::new("Alice"), Score(0)),
        (Name::new("Bob"), Score(0)),
    ]);
}
```

##### See also

*   [`spawn`](../../prelude/struct.Commands.html#method.spawn "method bevy::prelude::Commands::spawn") to spawn an entity with components.
*   [`spawn_empty`](../../prelude/struct.Commands.html#method.spawn_empty "method bevy::prelude::Commands::spawn_empty") to spawn an entity without components.

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/ecs/contiguous\_query.rs ([lines 59-66](../../../src/contiguous_query/contiguous_query.rs.html#59-66))

```rust
57fn setup(mut commands: Commands) {
58    let mut i = 0;
59    commands.spawn_batch(std::iter::from_fn(move || {
60        i += 1;
61        if i == 10_000 {
62            None
63        } else {
64            Some((Health(i as f32 * 5.0), HealthDecay(0.9)))
65        }
66    }));
67}
```

Hide additional examples

examples/ecs/ecs\_guide.rs ([lines 193-208](../../../src/ecs_guide/ecs_guide.rs.html#193-208))

```rust
183fn startup_system(mut commands: Commands, mut game_state: ResMut<GameState>) {
184    // Create our game rules resource
185    commands.insert_resource(GameRules {
186        max_rounds: 10,
187        winning_score: 4,
188        max_players: 4,
189    });
190
191    // Add some players to our world. Players start with a score of 0 ... we want our game to be
192    // fair!
193    commands.spawn_batch(vec![
194        (
195            Player {
196                name: "Alice".to_string(),
197            },
198            Score { value: 0 },
199            PlayerStreak::None,
200        ),
201        (
202            Player {
203                name: "Bob".to_string(),
204            },
205            Score { value: 0 },
206            PlayerStreak::None,
207        ),
208    ]);
209
210    // set the total players to "2"
211    game_state.total_players = 2;
212}
```

examples/3d/order\_independent\_transparency.rs ([line 345](../../../src/order_independent_transparency/order_independent_transparency.rs.html#345))

```rust
317fn spawn_auto_instancing_test(
318    commands: &mut Commands,
319    meshes: &mut Assets<Mesh>,
320    materials: &mut Assets<StandardMaterial>,
321    asset_server: Res<AssetServer>,
322) {
323    let render_layers = RenderLayers::layer(1);
324
325    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
326    let material_handle = materials.add(StandardMaterial {
327        alpha_mode: AlphaMode::Blend,
328        base_color_texture: Some(asset_server.load("textures/slice_square.png")),
329        ..Default::default()
330    });
331    let mut bundles = Vec::with_capacity(3 * 3 * 3);
332
333    for z in -1..=1 {
334        for y in -1..=1 {
335            for x in -1..=1 {
336                bundles.push((
337                    Mesh3d(cube.clone()),
338                    MeshMaterial3d(material_handle.clone()),
339                    Transform::from_xyz(x as f32 * 2.0, y as f32 * 2.0, z as f32 * 2.0),
340                    render_layers.clone(),
341                ));
342            }
343        }
344    }
345    commands.spawn_batch(bundles);
346}
```

examples/stress\_tests/many\_sprites.rs ([line 99](../../../src/many_sprites/many_sprites.rs.html#99))

```rust
54fn setup(mut commands: Commands, assets: Res<AssetServer>, color_tint: Res<ColorTint>) {
55    warn!(include_str!("warning_string.txt"));
56
57    let mut rng = rand::rng();
58
59    let tile_size = Vec2::splat(64.0);
60    let map_size = Vec2::splat(320.0);
61
62    let half_x = (map_size.x / 2.0) as i32;
63    let half_y = (map_size.y / 2.0) as i32;
64
65    let sprite_handle = assets.load("branding/icon.png");
66
67    // Spawns the camera
68
69    commands.spawn(Camera2d);
70
71    // Builds and spawns the sprites
72    let mut sprites = vec![];
73    for y in -half_y..half_y {
74        for x in -half_x..half_x {
75            let position = Vec2::new(x as f32, y as f32);
76            let translation = (position * tile_size).extend(rng.random::<f32>());
77            let rotation = Quat::from_rotation_z(rng.random::<f32>());
78            let scale = Vec3::splat(rng.random::<f32>() * 2.0);
79
80            sprites.push((
81                Sprite {
82                    image: sprite_handle.clone(),
83                    custom_size: Some(tile_size),
84                    color: if color_tint.0 {
85                        COLORS[rng.random_range(0..3)]
86                    } else {
87                        Color::WHITE
88                    },
89                    ..default()
90                },
91                Transform {
92                    translation,
93                    rotation,
94                    scale,
95                },
96            ));
97        }
98    }
99    commands.spawn_batch(sprites);
100}
```

examples/stress\_tests/many\_sprite\_meshes.rs ([line 101](../../../src/many_sprite_meshes/many_sprite_meshes.rs.html#101))

```rust
56fn setup(mut commands: Commands, assets: Res<AssetServer>, color_tint: Res<ColorTint>) {
57    warn!(include_str!("warning_string.txt"));
58
59    let mut rng = rand::rng();
60
61    let tile_size = Vec2::splat(64.0);
62    let map_size = Vec2::splat(320.0);
63
64    let half_x = (map_size.x / 2.0) as i32;
65    let half_y = (map_size.y / 2.0) as i32;
66
67    let sprite_handle = assets.load("branding/icon.png");
68
69    // Spawns the camera
70
71    commands.spawn(Camera2d);
72
73    // Builds and spawns the sprites
74    let mut sprites = vec![];
75    for y in -half_y..half_y {
76        for x in -half_x..half_x {
77            let position = Vec2::new(x as f32, y as f32);
78            let translation = (position * tile_size).extend(rng.random::<f32>());
79            let rotation = Quat::from_rotation_z(rng.random::<f32>());
80            let scale = Vec3::splat(rng.random::<f32>() * 2.0);
81
82            sprites.push((
83                SpriteMesh {
84                    image: sprite_handle.clone(),
85                    custom_size: Some(tile_size),
86                    color: if color_tint.0 {
87                        COLORS[rng.random_range(0..3)]
88                    } else {
89                        Color::WHITE
90                    },
91                    ..default()
92                },
93                Transform {
94                    translation,
95                    rotation,
96                    scale,
97                },
98            ));
99        }
100    }
101    commands.spawn_batch(sprites);
102}
```

examples/stress\_tests/bevymark\_3d.rs ([line 397](../../../src/bevymark_3d/bevymark_3d.rs.html#397))

```rust
350fn spawn_cubes(
351    commands: &mut Commands,
352    args: &Args,
353    counter: &mut BevyCounter,
354    spawn_count: usize,
355    cube_resources: &mut CubeResources,
356    waves_to_simulate: Option<usize>,
357    wave: usize,
358) {
359    let batch_material = cube_resources.materials[wave % cube_resources.materials.len()].clone();
360
361    let spawn_y = VOLUME_SIZE.y / 2.0 - HALF_CUBE_SIZE;
362    let spawn_z = -VOLUME_SIZE.z / 2.0 + HALF_CUBE_SIZE;
363
364    let batch = (0..spawn_count)
365        .map(|_| {
366            let spawn_pos = Vec3::new(
367                (cube_resources.transform_rng.random::<f32>() - 0.5) * VOLUME_SIZE.x,
368                spawn_y,
369                spawn_z,
370            );
371
372            let (transform, velocity) = cube_velocity_transform(
373                spawn_pos,
374                &mut cube_resources.velocity_rng,
375                waves_to_simulate,
376                FIXED_DELTA_TIME,
377            );
378
379            let material = if args.vary_per_instance {
380                cube_resources
381                    .materials
382                    .choose(&mut cube_resources.material_rng)
383                    .unwrap()
384                    .clone()
385            } else {
386                batch_material.clone()
387            };
388
389            (
390                Mesh3d(cube_resources.cube_mesh.clone()),
391                MeshMaterial3d(material),
392                transform,
393                Cube { velocity },
394            )
395        })
396        .collect::<Vec<_>>();
397    commands.spawn_batch(batch);
398
399    counter.count += spawn_count;
400    counter.color = Color::linear_rgb(
401        cube_resources.color_rng.random(),
402        cube_resources.color_rng.random(),
403        cube_resources.color_rng.random(),
404    );
405}
```

Additional examples can be found in:  

*   [examples/stress\_tests/many\_text2d.rs](../../../src/many_text2d/many_text2d.rs.html#152)
*   [examples/stress\_tests/many\_lights.rs](../../../src/many_lights/many_lights.rs.html#71-86)
*   [examples/3d/spotlight.rs](../../../src/spotlight/spotlight.rs.html#58-72)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#450)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#641)

#### pub fn [queue](#method.queue)(&mut self, command: impl [Command](../../prelude/trait.Command.html "trait bevy::prelude::Command"))

Pushes a generic [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") to the command queue.

If the [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") returns a [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result"), it will be handled using the [fallback error handler](../error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler").

To use a custom error handler, see [`Commands::queue_handled`](../../prelude/struct.Commands.html#method.queue_handled "method bevy::prelude::Commands::queue_handled").

The command can be:

*   A custom struct that implements [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command").
*   A closure or function that matches one of the following signatures:
    *   [`(&mut World)`](../../prelude/struct.World.html "struct bevy::prelude::World")
*   A built-in command from the [`command`](command/index.html "mod bevy::ecs::system::command") module.

##### Example

```rust
#[derive(Resource, Default)]
struct Counter(u64);

struct AddToCounter(String);

impl Command for AddToCounter {
    type Out = Result;

    fn apply(self, world: &mut World) -> Result {
        let mut counter = world.get_resource_or_insert_with(Counter::default);
        let amount: u64 = self.0.parse()?;
        counter.0 += amount;
        Ok(())
    }
}

fn add_three_to_counter_system(mut commands: Commands) {
    commands.queue(AddToCounter("3".to_string()));
}

fn add_twenty_five_to_counter_system(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let mut counter = world.get_resource_or_insert_with(Counter::default);
        counter.0 += 25;
    });
}
```

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/app/settings.rs ([line 127](../../../src/settings/settings.rs.html#127))

```rust
111fn change_count(
112    mut counter: ResMut<Counter>,
113    keyboard: Res<ButtonInput<KeyCode>>,
114    mut commands: Commands,
115) {
116    let mut changed = false;
117    if keyboard.just_pressed(KeyCode::Space) {
118        counter.count += 1;
119        changed = true;
120    }
121    if keyboard.just_pressed(KeyCode::Backspace) || keyboard.just_pressed(KeyCode::Delete) {
122        counter.count -= 1;
123        changed = true;
124    }
125
126    if changed {
127        commands.queue(SaveSettingsDeferred(Duration::from_secs_f32(0.1)));
128    }
129}
130
131fn on_window_close(mut close: MessageReader<WindowCloseRequested>, mut commands: Commands) {
132    // Save settings immediately, then quit.
133    if let Some(_close_event) = close.read().next() {
134        commands.queue(SaveSettingsSync::IfChanged);
135        commands.write_message(AppExit::Success);
136    }
137}
```

Hide additional examples

examples/window/persisting\_window\_settings.rs ([line 111](../../../src/persisting_window_settings/persisting_window_settings.rs.html#111))

```rust
90fn update_window_settings(
91    mut move_events: MessageReader<WindowMoved>,
92    mut resize_events: MessageReader<WindowResized>,
93    windows: Query<&mut Window>,
94    window_settings: ResMut<WindowSettings>,
95    mut commands: Commands,
96) {
97    let Ok(window) = windows.single() else {
98        return;
99    };
100
101    let mut window_changed = false;
102    for _ in move_events.read() {
103        window_changed = true;
104    }
105
106    for _ in resize_events.read() {
107        window_changed = true;
108    }
109
110    if window_changed && store_window_settings(window_settings, window) {
111        commands.queue(SaveSettingsDeferred(Duration::from_secs_f32(0.5)));
112    }
113}
114
115fn store_window_settings(mut window_settings: ResMut<WindowSettings>, window: &Window) -> bool {
116    window_settings.set_if_neq(WindowSettings {
117        position: match window.position {
118            WindowPosition::At(pos) => Some(pos),
119            _ => None,
120        },
121        size: Some(UVec2::new(
122            window.resolution.width() as u32,
123            window.resolution.height() as u32,
124        )),
125        fullscreen: window.mode != WindowMode::Windowed,
126    })
127}
128
129fn on_window_close(mut close: MessageReader<WindowCloseRequested>, mut commands: Commands) {
130    // Save settings immediately, then quit.
131    if let Some(_close_event) = close.read().next() {
132        commands.queue(SaveSettingsSync::IfChanged);
133        commands.write_message(AppExit::Success);
134    }
135}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#694-698)

#### pub fn [queue\_handled](#method.queue_handled)( &mut self, command: impl [Command](../../prelude/trait.Command.html "trait bevy::prelude::Command"), error\_handler: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError"), [ErrorContext](../error/enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext")), )

Pushes a generic [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") to the command queue.

If the [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") returns a [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result"), the given `error_handler` will be used to handle error cases.

To implicitly use the fallback error handler, see [`Commands::queue`](../../prelude/struct.Commands.html#method.queue "method bevy::prelude::Commands::queue").

The command can be:

*   A custom struct that implements [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command").
*   A closure or function that matches one of the following signatures:
    *   [`(&mut World)`](../../prelude/struct.World.html "struct bevy::prelude::World")
    *   [`(&mut World)`](../../prelude/struct.World.html "struct bevy::prelude::World") `->` [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")
*   A built-in command from the [`command`](command/index.html "mod bevy::ecs::system::command") module.

##### Example

```rust
use bevy_ecs::error::warn;

#[derive(Resource, Default)]
struct Counter(u64);

struct AddToCounter(String);

impl Command for AddToCounter {
    type Out = Result;

    fn apply(self, world: &mut World) -> Result {
        let mut counter = world.get_resource_or_insert_with(Counter::default);
        let amount: u64 = self.0.parse()?;
        counter.0 += amount;
        Ok(())
    }
}

fn add_three_to_counter_system(mut commands: Commands) {
    commands.queue_handled(AddToCounter("3".to_string()), warn);
}

fn add_twenty_five_to_counter_system(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let mut counter = world.get_resource_or_insert_with(Counter::default);
        counter.0 += 25;
    });
}
```

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/ecs/error\_handling.rs ([lines 186-197](../../../src/error_handling/error_handling.rs.html#186-197))

```rust
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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#703)

#### pub fn [queue\_silenced](#method.queue_silenced)(&mut self, command: impl [Command](../../prelude/trait.Command.html "trait bevy::prelude::Command"))

Pushes a generic [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command") to the queue like [`Commands::queue_handled`](../../prelude/struct.Commands.html#method.queue_handled "method bevy::prelude::Commands::queue_handled"), but instead silently ignores any errors.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#744-747)

#### pub fn [insert\_batch](#method.insert_batch)<I, B>(&mut self, batch: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), B)> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <B as [DynamicBundle](../bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

Adds a series of [`Bundles`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to each [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") they are paired with, based on a batch of `(Entity, Bundle)` pairs.

A batch can be any type that implements [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") and contains `(Entity, Bundle)` tuples, such as a [`Vec<(Entity, Bundle)>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") or an array `[(Entity, Bundle); N]`.

This will overwrite any pre-existing components shared by the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") type. Use [`Commands::insert_batch_if_new`](../../prelude/struct.Commands.html#method.insert_batch_if_new "method bevy::prelude::Commands::insert_batch_if_new") to keep the pre-existing components instead.

This method is equivalent to iterating the batch and calling [`insert`](../../prelude/struct.EntityCommands.html#method.insert "method bevy::prelude::EntityCommands::insert") for each pair, but is faster by caching data that is shared between entities.

##### Fallible

This command will fail if any of the given entities do not exist.

It will internally return a [`TryInsertBatchError`](../world/error/struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError"), which will be handled by the [fallback error handler](../error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#775-778)

#### pub fn [insert\_batch\_if\_new](#method.insert_batch_if_new)<I, B>(&mut self, batch: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), B)> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <B as [DynamicBundle](../bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

Adds a series of [`Bundles`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to each [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") they are paired with, based on a batch of `(Entity, Bundle)` pairs.

A batch can be any type that implements [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") and contains `(Entity, Bundle)` tuples, such as a [`Vec<(Entity, Bundle)>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") or an array `[(Entity, Bundle); N]`.

This will keep any pre-existing components shared by the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") type and discard the new values. Use [`Commands::insert_batch`](../../prelude/struct.Commands.html#method.insert_batch "method bevy::prelude::Commands::insert_batch") to overwrite the pre-existing components instead.

This method is equivalent to iterating the batch and calling [`insert_if_new`](../../prelude/struct.EntityCommands.html#method.insert_if_new "method bevy::prelude::EntityCommands::insert_if_new") for each pair, but is faster by caching data that is shared between entities.

##### Fallible

This command will fail if any of the given entities do not exist.

It will internally return a [`TryInsertBatchError`](../world/error/struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError"), which will be handled by the [fallback error handler](../error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#805-808)

#### pub fn [try\_insert\_batch](#method.try_insert_batch)<I, B>(&mut self, batch: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), B)> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <B as [DynamicBundle](../bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

Adds a series of [`Bundles`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to each [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") they are paired with, based on a batch of `(Entity, Bundle)` pairs.

A batch can be any type that implements [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") and contains `(Entity, Bundle)` tuples, such as a [`Vec<(Entity, Bundle)>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") or an array `[(Entity, Bundle); N]`.

This will overwrite any pre-existing components shared by the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") type. Use [`Commands::try_insert_batch_if_new`](../../prelude/struct.Commands.html#method.try_insert_batch_if_new "method bevy::prelude::Commands::try_insert_batch_if_new") to keep the pre-existing components instead.

This method is equivalent to iterating the batch and calling [`insert`](../../prelude/struct.EntityCommands.html#method.insert "method bevy::prelude::EntityCommands::insert") for each pair, but is faster by caching data that is shared between entities.

##### Fallible

This command will fail if any of the given entities do not exist.

It will internally return a [`TryInsertBatchError`](../world/error/struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError"), which will be handled by [logging the error at the `warn` level](../error/fn.warn.html "fn bevy::ecs::error::warn").

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/2d/mesh2d\_manual.rs ([line 375](../../../src/mesh2d_manual/mesh2d_manual.rs.html#375))

```rust
332pub fn extract_colored_mesh2d(
333    mut commands: Commands,
334    mut previous_len: Local<usize>,
335    // When extracting, you must use `Extract` to mark the `SystemParam`s
336    // which should be taken from the main world.
337    query: Extract<
338        Query<
339            (
340                Entity,
341                RenderEntity,
342                &ViewVisibility,
343                &GlobalTransform,
344                &Mesh2d,
345            ),
346            With<ColoredMesh2d>,
347        >,
348    >,
349    mut render_mesh_instances: ResMut<RenderColoredMesh2dInstances>,
350) {
351    let mut values = Vec::with_capacity(*previous_len);
352    for (entity, render_entity, view_visibility, transform, handle) in &query {
353        if !view_visibility.get() {
354            continue;
355        }
356
357        let transforms = Mesh2dTransforms {
358            world_from_local: transform.affine().into(),
359            flags: MeshFlags::empty().bits(),
360        };
361
362        values.push((render_entity, ColoredMesh2d));
363        render_mesh_instances.insert(
364            entity.into(),
365            RenderMesh2dInstance {
366                mesh_asset_id: handle.0.id(),
367                transforms,
368                material_bind_group_id: Material2dBindGroupId::default(),
369                automatic_batching: false,
370                tag: 0,
371            },
372        );
373    }
374    *previous_len = values.len();
375    commands.try_insert_batch(values);
376}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#836-839)

#### pub fn [try\_insert\_batch\_if\_new](#method.try_insert_batch_if_new)<I, B>(&mut self, batch: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), B)> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <B as [DynamicBundle](../bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

Adds a series of [`Bundles`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to each [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") they are paired with, based on a batch of `(Entity, Bundle)` pairs.

A batch can be any type that implements [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") and contains `(Entity, Bundle)` tuples, such as a [`Vec<(Entity, Bundle)>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") or an array `[(Entity, Bundle); N]`.

This will keep any pre-existing components shared by the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") type and discard the new values. Use [`Commands::try_insert_batch`](../../prelude/struct.Commands.html#method.try_insert_batch "method bevy::prelude::Commands::try_insert_batch") to overwrite the pre-existing components instead.

This method is equivalent to iterating the batch and calling [`insert_if_new`](../../prelude/struct.EntityCommands.html#method.insert_if_new "method bevy::prelude::EntityCommands::insert_if_new") for each pair, but is faster by caching data that is shared between entities.

##### Fallible

This command will fail if any of the given entities do not exist.

It will internally return a [`TryInsertBatchError`](../world/error/struct.TryInsertBatchError.html "struct bevy::ecs::world::error::TryInsertBatchError"), which will be handled by [logging the error at the `warn` level](../error/fn.warn.html "fn bevy::ecs::error::warn").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#868)

#### pub fn [init\_resource](#method.init_resource)<R>(&mut self)

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Inserts a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") into the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") with an inferred value.

The inferred value is determined by the [`FromWorld`](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") trait of the resource. Note that any resource with the [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") trait automatically implements [`FromWorld`](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"), and those default values will be used.

If the resource already exists when the command is applied, nothing happens.

##### Example

```rust
#[derive(Resource, Default)]
struct Scoreboard {
    current_score: u32,
    high_score: u32,
}

fn initialize_scoreboard(mut commands: Commands) {
    commands.init_resource::<Scoreboard>();
}
```

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/custom\_phase\_item.rs ([line 211](../../../src/custom_phase_item/custom_phase_item.rs.html#211))

```rust
210fn prepare_custom_phase_item_buffers(mut commands: Commands) {
211    commands.init_resource::<CustomPhaseItemBuffers>();
212}
```

Hide additional examples

examples/diagnostics/log\_diagnostics.rs ([line 89](../../../src/log_diagnostics/log_diagnostics.rs.html#89))

```rust
58fn setup(
59    mut commands: Commands,
60    mut meshes: ResMut<Assets<Mesh>>,
61    mut materials: ResMut<Assets<StandardMaterial>>,
62) {
63    // circular base
64    commands.spawn((
65        Mesh3d(meshes.add(Circle::new(4.0))),
66        MeshMaterial3d(materials.add(Color::WHITE)),
67        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
68    ));
69    // cube
70    commands.spawn((
71        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
72        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
73        Transform::from_xyz(0.0, 0.5, 0.0),
74    ));
75    // light
76    commands.spawn((
77        PointLight {
78            shadow_maps_enabled: true,
79            ..default()
80        },
81        Transform::from_xyz(4.0, 8.0, 4.0),
82    ));
83    // camera
84    commands.spawn((
85        Camera3d::default(),
86        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
87    ));
88
89    commands.init_resource::<LogDiagnosticsFilters>();
90    commands.init_resource::<LogDiagnosticsStatus>();
91
92    commands.spawn((
93        LogDiagnosticsCommands,
94        Node {
95            top: px(5),
96            left: px(5),
97            flex_direction: FlexDirection::Column,
98            ..default()
99        },
100    ));
101}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#895)

#### pub fn [insert\_resource](#method.insert_resource)<R>(&mut self, resource: R)

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Inserts a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") into the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") with a specific value.

This will overwrite any previous value of the same resource type.

##### Example

```rust
#[derive(Resource)]
struct Scoreboard {
    current_score: u32,
    high_score: u32,
}

fn system(mut commands: Commands) {
    commands.insert_resource(Scoreboard {
        current_score: 0,
        high_score: 0,
    });
}
```

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/audio/pitch.rs ([line 22](../../../src/pitch/pitch.rs.html#22))

```rust
21fn setup(mut commands: Commands) {
22    commands.insert_resource(PitchFrequency(220.0));
23}
```

Hide additional examples

examples/async\_tasks/async\_channel\_pattern.rs ([line 91](../../../src/async_channel_pattern/async_channel_pattern.rs.html#91))

```rust
89fn setup_channel(mut commands: Commands) {
90    let (sender, receiver) = crossbeam_channel::unbounded();
91    commands.insert_resource(CubeChannel { sender, receiver });
92}
93
94/// A channel for communicating between async tasks and the main thread.
95#[derive(Resource)]
96struct CubeChannel {
97    sender: Sender<CubeFinished>,
98    receiver: Receiver<CubeFinished>,
99}
100
101/// Represents the completion of a cube task, containing the cube's transform
102#[derive(Debug)]
103struct CubeFinished {
104    transform: Transform,
105}
106
107/// Resource holding the mesh handle for the box (used for spawning cubes)
108#[derive(Resource, Deref)]
109struct BoxMeshHandle(Handle<Mesh>);
110
111/// Resource holding the material handle for the box (used for spawning cubes)
112#[derive(Resource, Deref)]
113struct BoxMaterialHandle(Handle<StandardMaterial>);
114
115/// Sets up the shared mesh and material for the cubes.
116fn setup_assets(
117    mut commands: Commands,
118    mut meshes: ResMut<Assets<Mesh>>,
119    mut materials: ResMut<Assets<StandardMaterial>>,
120) {
121    // Create and store a cube mesh
122    let box_mesh_handle = meshes.add(Cuboid::new(0.4, 0.4, 0.4));
123    commands.insert_resource(BoxMeshHandle(box_mesh_handle));
124
125    // Create and store a red material
126    let box_material_handle = materials.add(Color::srgb(1.0, 0.2, 0.3));
127    commands.insert_resource(BoxMaterialHandle(box_material_handle));
128}
```

examples/app/headless\_renderer.rs ([lines 314-316](../../../src/headless_renderer/headless_renderer.rs.html#314-316))

```rust
313fn image_copy_extract(mut commands: Commands, image_copiers: Extract<Query<&ImageCopier>>) {
314    commands.insert_resource(ImageCopiers(
315        image_copiers.iter().cloned().collect::<Vec<ImageCopier>>(),
316    ));
317}
```

examples/2d/texture\_atlas.rs ([line 34](../../../src/texture_atlas/texture_atlas.rs.html#34))

```rust
32fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
33    // Load multiple, individual sprites from a folder
34    commands.insert_resource(RpgSpriteFolder(asset_server.load_folder("textures/rpg")));
35}
```

examples/shader\_advanced/custom\_shader\_instancing.rs ([lines 243-246](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#243-246))

```rust
238fn init_custom_pipeline(
239    mut commands: Commands,
240    asset_server: Res<AssetServer>,
241    mesh_pipeline: Res<MeshPipeline>,
242) {
243    commands.insert_resource(CustomPipeline {
244        shader: asset_server.load(SHADER_ASSET_PATH),
245        mesh_pipeline: mesh_pipeline.clone(),
246    });
247}
```

examples/shader\_advanced/custom\_render\_phase.rs ([lines 174-177](../../../src/custom_render_phase/custom_render_phase.rs.html#174-177))

```rust
169fn init_stencil_pipeline(
170    mut commands: Commands,
171    mesh_pipeline: Res<MeshPipeline>,
172    asset_server: Res<AssetServer>,
173) {
174    commands.insert_resource(StencilPipeline {
175        mesh_pipeline: mesh_pipeline.clone(),
176        shader_handle: asset_server.load(SHADER_ASSET_PATH),
177    });
178}
```

Additional examples can be found in:  

*   [examples/ecs/change\_detection.rs](../../../src/change_detection/change_detection.rs.html#32)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#177-180)
*   [examples/2d/mesh2d\_manual.rs](../../../src/mesh2d_manual/mesh2d_manual.rs.html#146-150)
*   [examples/async\_tasks/async\_compute.rs](../../../src/async_compute/async_compute.rs.html#50)
*   [examples/3d/mirror.rs](../../../src/mirror/mirror.rs.html#243)
*   [examples/asset/processing/asset\_processing.rs](../../../src/asset_processing/asset_processing.rs.html#240-246)
*   [examples/2d/sprite\_tile.rs](../../../src/sprite_tile/sprite_tile.rs.html#24-29)
*   [examples/audio/soundtrack.rs](../../../src/soundtrack/soundtrack.rs.html#49)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#261)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../../src/external_source_external_thread/external_source_external_thread.rs.html#43)
*   [examples/ecs/ecs\_guide.rs](../../../src/ecs_guide/ecs_guide.rs.html#185-189)
*   [examples/showcase/loading\_screen.rs](../../../src/loading_screen/loading_screen.rs.html#78)
*   [examples/window/window\_settings.rs](../../../src/window_settings/window_settings.rs.html#179)
*   [examples/2d/dynamic\_mip\_generation.rs](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#788)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#72-75)
*   [examples/shader\_advanced/compute\_mesh.rs](../../../src/compute_mesh/compute_mesh.rs.html#240)
*   [examples/showcase/game\_menu.rs](../../../src/game_menu/game_menu.rs.html#98)
*   [examples/shader\_advanced/manual\_material.rs](../../../src/manual_material/manual_material.rs.html#95)
*   [examples/remote/server.rs](../../../src/server/server.rs.html#47-50)
*   [examples/ui/text/font\_atlas\_debug.rs](../../../src/font_atlas_debug/font_atlas_debug.rs.html#110)
*   [examples/asset/multi\_asset\_sync.rs](../../../src/multi_asset_sync/multi_asset_sync.rs.html#146-156)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#316-319)
*   [examples/state/custom\_transitions.rs](../../../src/custom_transitions/custom_transitions.rs.html#278)
*   [examples/state/states.rs](../../../src/states/states.rs.html#91)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#124)
*   [examples/3d/skybox.rs](../../../src/skybox/skybox.rs.html#91-95)
*   [examples/state/sub\_states.rs](../../../src/sub_states/sub_states.rs.html#191)
*   [examples/shader/storage\_buffer.rs](../../../src/storage_buffer/storage_buffer.rs.html#44)
*   [examples/movement/smooth\_follow.rs](../../../src/smooth_follow/smooth_follow.rs.html#86)
*   [examples/2d/tilemap\_chunk.rs](../../../src/tilemap_chunk/tilemap_chunk.rs.html#64)
*   [examples/animation/animated\_mesh\_control.rs](../../../src/animated_mesh_control/animated_mesh_control.rs.html#49)
*   [examples/asset/generated\_assets.rs](../../../src/generated_assets/generated_assets.rs.html#42)
*   [examples/showcase/contributors.rs](../../../src/contributors/contributors.rs.html#136)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#576)
*   [examples/gizmos/axes.rs](../../../src/axes/axes.rs.html#94)
*   [examples/3d/motion\_blur.rs](../../../src/motion_blur/motion_blur.rs.html#60-64)
*   [examples/3d/atmosphere.rs](../../../src/atmosphere/atmosphere.rs.html#136-139)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#95-98)
*   [examples/3d/rect\_light.rs](../../../src/rect_light/rect_light.rs.html#41)
*   [examples/math/cubic\_splines.rs](../../../src/cubic_splines/cubic_splines.rs.html#38)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#208)
*   [examples/2d/cpu\_draw.rs](../../../src/cpu_draw/cpu_draw.rs.html#83)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../../src/custom_post_processing/custom_post_processing.rs.html#223-227)
*   [examples/transforms/align.rs](../../../src/align/align.rs.html#115)
*   [examples/math/random\_sampling.rs](../../../src/random_sampling/random_sampling.rs.html#58)
*   [examples/state/computed\_states.rs](../../../src/computed_states/computed_states.rs.html#400-402)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#406)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#435)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#196)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#144)
*   [examples/shader/gpu\_readback.rs](../../../src/gpu_readback/gpu_readback.rs.html#123)
*   [examples/3d/auto\_exposure.rs](../../../src/auto_exposure/auto_exposure.rs.html#55-66)
*   [examples/stress\_tests/bevymark\_3d.rs](../../../src/bevymark_3d/bevymark_3d.rs.html#293)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#319)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../../src/drag_to_scroll/drag_to_scroll.rs.html#27)
*   [examples/showcase/breakout.rs](../../../src/breakout/breakout.rs.html#183)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#132-135)
*   [examples/3d/lighting.rs](../../../src/lighting/lighting.rs.html#123-127)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#902)

#### pub fn [insert\_resource\_if\_neq](#method.insert_resource_if_neq)<R>(&mut self, resource: R)

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Inserts a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") into the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") with a specific value if the resource is different or missing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#932)

#### pub fn [remove\_resource](#method.remove_resource)<R>(&mut self)

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Removes a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") from the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### Example

```rust
#[derive(Resource)]
struct Scoreboard {
    current_score: u32,
    high_score: u32,
}

fn system(mut commands: Commands) {
    commands.remove_resource::<Scoreboard>();
}
```

##### [Examples found in repository](#scraped-examples-11)[?](../../../scrape-examples-help.html)

examples/showcase/desk\_toy.rs ([line 267](../../../src/desk_toy/desk_toy.rs.html#267))

```rust
266fn end_drag(mut commands: Commands) {
267    commands.remove_resource::<DragOperation>();
268}
```

Hide additional examples

examples/asset/multi\_asset\_sync.rs ([line 278](../../../src/multi_asset_sync/multi_asset_sync.rs.html#278))

```rust
271fn despawn_loading_state_entities(mut commands: Commands, loading: Query<Entity, With<Loading>>) {
272    // Despawn entities in the loading phase.
273    for entity in loading.iter() {
274        commands.entity(entity).despawn();
275    }
276
277    // Despawn resources used in the loading phase.
278    commands.remove_resource::<AssetBarrier>();
279    commands.remove_resource::<AsyncLoadingState>();
280}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#954)

#### pub fn [run\_system](#method.run_system)(&mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[SystemId](struct.SystemId.html "struct bevy::ecs::system::SystemId")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"))

Runs the system corresponding to the given [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId"). Before running a system, it must first be registered via [`Commands::register_system`](../../prelude/struct.Commands.html#method.register_system "method bevy::prelude::Commands::register_system") or [`World::register_system`](../../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system").

The system is run in an exclusive and single-threaded way. Running slow systems can become a bottleneck.

There is no way to get the output of a system when run as a command, because the execution of the system happens later. To get the output of a system, use [`World::run_system`](../../prelude/struct.World.html#method.run_system "method bevy::prelude::World::run_system") or [`World::run_system_with`](../../prelude/struct.World.html#method.run_system_with "method bevy::prelude::World::run_system_with") instead of running the system as a command.

##### Fallible

This command will fail if the given [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") does not correspond to a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

It will internally return a [`RegisteredSystemError`](enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError"), which will be handled by [logging the error at the `warn` level](../error/fn.warn.html "fn bevy::ecs::error::warn").

##### [Examples found in repository](#scraped-examples-12)[?](../../../scrape-examples-help.html)

examples/ecs/callbacks.rs ([line 51](../../../src/callbacks/callbacks.rs.html#51))

```rust
49fn run_callbacks(mut commands: Commands, query: Query<&Callback>) {
50    for callback in query.iter() {
51        commands.run_system(callback.system_id);
52    }
53}
```

Hide additional examples

examples/ecs/one\_shot\_systems.rs ([line 77](../../../src/one_shot_systems/one_shot_systems.rs.html#77))

```rust
75fn evaluate_callbacks(query: Query<(Entity, &Callback), With<Triggered>>, mut commands: Commands) {
76    for (entity, callback) in query.iter() {
77        commands.run_system(callback.0);
78        commands.entity(entity).remove::<Triggered>();
79    }
80}
```

examples/showcase/loading\_screen.rs ([line 107](../../../src/loading_screen/loading_screen.rs.html#107))

```rust
98fn level_selection(
99    mut commands: Commands,
100    keyboard: Res<ButtonInput<KeyCode>>,
101    level_data: Res<LevelData>,
102    loading_state: Res<LoadingState>,
103) {
104    // Only trigger a load if the current level is fully loaded.
105    if let LoadingState::LevelReady = loading_state.as_ref() {
106        if keyboard.just_pressed(KeyCode::Digit1) {
107            commands.run_system(level_data.unload_level_id);
108            commands.run_system(level_data.level_1_id);
109        } else if keyboard.just_pressed(KeyCode::Digit2) {
110            commands.run_system(level_data.unload_level_id);
111            commands.run_system(level_data.level_2_id);
112        }
113    }
114}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#976-981)

#### pub fn [run\_system\_with](#method.run_system_with)<I>( &mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[SystemId](struct.SystemId.html "struct bevy::ecs::system::SystemId")<I>> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"), input: <I as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'static>, )

where I: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, <I as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'static>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

Runs the system corresponding to the given [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") with input. Before running a system, it must first be registered via [`Commands::register_system`](../../prelude/struct.Commands.html#method.register_system "method bevy::prelude::Commands::register_system") or [`World::register_system`](../../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system").

The system is run in an exclusive and single-threaded way. Running slow systems can become a bottleneck.

There is no way to get the output of a system when run as a command, because the execution of the system happens later. To get the output of a system, use [`World::run_system`](../../prelude/struct.World.html#method.run_system "method bevy::prelude::World::run_system") or [`World::run_system_with`](../../prelude/struct.World.html#method.run_system_with "method bevy::prelude::World::run_system_with") instead of running the system as a command.

##### Fallible

This command will fail if the given [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") does not correspond to a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

It will internally return a [`RegisteredSystemError`](enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError"), which will be handled by [logging the error at the `warn` level](../error/fn.warn.html "fn bevy::ecs::error::warn").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1044-1050)

#### pub fn [register\_system](#method.register_system)<I, O, M>( &mut self, system: impl [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static, ) -> [SystemId](struct.SystemId.html "struct bevy::ecs::system::SystemId")<I, O>

where I: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, O: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Registers a system and returns its [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") so it can later be called by [`Commands::run_system`](../../prelude/struct.Commands.html#method.run_system "method bevy::prelude::Commands::run_system") or [`World::run_system`](../../prelude/struct.World.html#method.run_system "method bevy::prelude::World::run_system").

This is different from adding systems to a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"), because the [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") that is returned can be used anywhere in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") to run the associated system.

Using a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") is still preferred for most cases due to its better performance and ability to run non-conflicting systems simultaneously.

##### Note

If the same system is registered more than once, each registration will be considered a different system, and they will each be given their own [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId").

If you want to avoid registering the same system multiple times, consider using [`Commands::run_system_cached`](../../prelude/struct.Commands.html#method.run_system_cached "method bevy::prelude::Commands::run_system_cached") or storing the [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") in a [`Local`](../../prelude/struct.Local.html "struct bevy::prelude::Local").

##### Example

```rust
#[derive(Resource)]
struct Counter(i32);

fn register_system(
    mut commands: Commands,
    mut local_system: Local<Option<SystemId>>,
) {
    if let Some(system) = *local_system {
        commands.run_system(system);
    } else {
        *local_system = Some(commands.register_system(increment_counter));
    }
}

fn increment_counter(mut value: ResMut<Counter>) {
    value.0 += 1;
}
```

##### [Examples found in repository](#scraped-examples-13)[?](../../../scrape-examples-help.html)

examples/ecs/one\_shot\_systems.rs ([line 43](../../../src/one_shot_systems/one_shot_systems.rs.html#43))

```rust
42fn setup_with_commands(mut commands: Commands) {
43    let system_id = commands.register_system(system_a);
44    commands.spawn((Callback(system_id), A));
45}
```

Hide additional examples

examples/showcase/loading\_screen.rs ([line 74](../../../src/loading_screen/loading_screen.rs.html#74))

```rust
72fn setup(mut commands: Commands) {
73    let level_data = LevelData {
74        unload_level_id: commands.register_system(unload_current_level),
75        level_1_id: commands.register_system(load_level_1),
76        level_2_id: commands.register_system(load_level_2),
77    };
78    commands.insert_resource(level_data);
79
80    // Spawns the UI that will show the user prompts.
81    let text_style = TextFont {
82        font_size: FontSize::Px(42.0),
83        ..default()
84    };
85    commands
86        .spawn((
87            Node {
88                justify_self: JustifySelf::Center,
89                align_self: AlignSelf::FlexEnd,
90                ..default()
91            },
92            BackgroundColor(Color::NONE),
93        ))
94        .with_child((Text::new("Press 1 or 2 to load a new scene."), text_style));
95}
```

examples/ecs/callbacks.rs ([lines 23-25](../../../src/callbacks/callbacks.rs.html#23-25))

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1072-1075)

#### pub fn [unregister\_system](#method.unregister_system)<I, O>(&mut self, system\_id: [SystemId](struct.SystemId.html "struct bevy::ecs::system::SystemId")<I, O>)

where I: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, O: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Removes a system previously registered with [`Commands::register_system`](../../prelude/struct.Commands.html#method.register_system "method bevy::prelude::Commands::register_system") or [`World::register_system`](../../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system").

After removing a system, the [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") becomes invalid and attempting to use it afterwards will result in an error. Re-adding the removed system will register it with a new `SystemId`.

##### Fallible

This command will fail if the given [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") does not correspond to a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System").

It will internally return a [`RegisteredSystemError`](enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError"), which will be handled by [logging the error at the `warn` level](../error/fn.warn.html "fn bevy::ecs::error::warn").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1092-1097)

#### pub fn [unregister\_system\_cached](#method.unregister_system_cached)<I, O, M, S>(&mut self, system: S)

where I: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, O: 'static, M: 'static, S: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Removes a system previously registered with one of the following:

*   [`Commands::run_system_cached`](../../prelude/struct.Commands.html#method.run_system_cached "method bevy::prelude::Commands::run_system_cached")
*   [`World::run_system_cached`](../../prelude/struct.World.html#method.run_system_cached "method bevy::prelude::World::run_system_cached")
*   [`World::register_system_cached`](../../prelude/struct.World.html#method.register_system_cached "method bevy::prelude::World::register_system_cached")

##### Fallible

This command will fail if the given system is not currently cached in a [`CachedSystemId`](struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId") resource.

It will internally return a [`RegisteredSystemError`](enum.RegisteredSystemError.html "enum bevy::ecs::system::RegisteredSystemError"), which will be handled by [logging the error at the `warn` level](../error/fn.warn.html "fn bevy::ecs::error::warn").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1124-1127)

#### pub fn [run\_system\_cached](#method.run_system_cached)<M, S>(&mut self, system: S)

where M: 'static, S: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), M> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Runs a cached system, registering it if necessary.

Unlike [`Commands::run_system`](../../prelude/struct.Commands.html#method.run_system "method bevy::prelude::Commands::run_system"), this method does not require manual registration.

The first time this method is called for a particular system, it will register the system and store its [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") in a [`CachedSystemId`](struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId") resource for later.

If you would rather manage the [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") yourself, or register multiple copies of the same system, use [`Commands::register_system`](../../prelude/struct.Commands.html#method.register_system "method bevy::prelude::Commands::register_system") instead.

##### Limitations

This method only accepts ZST (zero-sized) systems to guarantee that any two systems of the same type must be equal. This means that closures that capture the environment, and function pointers, are not accepted.

If you want to access values from the environment within a system, consider passing them in as inputs via [`Commands::run_system_cached_with`](../../prelude/struct.Commands.html#method.run_system_cached_with "method bevy::prelude::Commands::run_system_cached_with").

If that’s not an option, consider [`Commands::register_system`](../../prelude/struct.Commands.html#method.register_system "method bevy::prelude::Commands::register_system") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1156-1160)

#### pub fn [run\_system\_cached\_with](#method.run_system_cached_with)<I, M, S>( &mut self, system: S, input: <I as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'static>, )

where I: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, <I as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'static>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"), M: 'static, S: [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), M> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Runs a cached system with an input, registering it if necessary.

Unlike [`Commands::run_system_with`](../../prelude/struct.Commands.html#method.run_system_with "method bevy::prelude::Commands::run_system_with"), this method does not require manual registration.

To use the supplied input, the system should have a [`SystemInput`](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") as the first parameter.

The first time this method is called for a particular system, it will register the system and store its [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") in a [`CachedSystemId`](struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId") resource for later.

If you would rather manage the [`SystemId`](struct.SystemId.html "struct bevy::ecs::system::SystemId") yourself, or register multiple copies of the same system, use [`Commands::register_system`](../../prelude/struct.Commands.html#method.register_system "method bevy::prelude::Commands::register_system") instead.

##### Limitations

This method only accepts ZST (zero-sized) systems to guarantee that any two systems of the same type must be equal. This means that closures that capture the environment, and function pointers, are not accepted.

If you want to access values from the environment within a system, consider passing them in as inputs.

If that’s not an option, consider [`Commands::register_system`](../../prelude/struct.Commands.html#method.register_system "method bevy::prelude::Commands::register_system") instead.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1169)

#### pub fn [trigger](#method.trigger)<'a>(&mut self, event: impl Event : Default>)

where <impl [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event") as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'a>: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Triggers the given [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event"), which will run any [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.

##### [Examples found in repository](#scraped-examples-14)[?](../../../scrape-examples-help.html)

examples/asset/asset\_saving.rs ([lines 195-198](../../../src/asset_saving/asset_saving.rs.html#195-198))

```rust
194fn on_drag_start(event: On<Pointer<DragStart>>, mut commands: Commands) {
195    commands.trigger(TryPlot {
196        entity: event.entity,
197        location: event.pointer_location.clone(),
198    });
199}
200
201fn on_drag(event: On<Pointer<Drag>>, mut commands: Commands) {
202    commands.trigger(TryPlot {
203        entity: event.entity,
204        location: event.pointer_location.clone(),
205    });
206}
```

Hide additional examples

examples/usage/context\_menu.rs ([line 66](../../../src/context_menu/context_menu.rs.html#66))

```rust
60fn setup(mut commands: Commands) {
61    commands.spawn(Camera2d);
62
63    commands.spawn(background_and_button()).observe(
64        // any click bubbling up here should lead to closing any open menu
65        |_: On<Pointer<Press>>, mut commands: Commands| {
66            commands.trigger(CloseContextMenus);
67        },
68    );
69}
70
71fn on_trigger_close_menus(
72    _event: On<CloseContextMenus>,
73    mut commands: Commands,
74    menus: Query<Entity, With<ContextMenu>>,
75) {
76    for e in menus.iter() {
77        commands.entity(e).despawn();
78    }
79}
80
81fn on_trigger_menu(event: On<OpenContextMenu>, mut commands: Commands) {
82    commands.trigger(CloseContextMenus);
83
84    let pos = event.pos;
85
86    debug!("open context menu at: {pos}");
87
88    commands
89        .spawn((
90            Name::new("context menu"),
91            ContextMenu,
92            Node {
93                position_type: PositionType::Absolute,
94                left: px(pos.x),
95                top: px(pos.y),
96                flex_direction: FlexDirection::Column,
97                border_radius: BorderRadius::all(px(4)),
98                ..default()
99            },
100            BorderColor::all(Color::BLACK),
101            BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
102            children![
103                context_item("fuchsia", basic::FUCHSIA),
104                context_item("gray", basic::GRAY),
105                context_item("maroon", basic::MAROON),
106                context_item("purple", basic::PURPLE),
107                context_item("teal", basic::TEAL),
108            ],
109        ))
110        .observe(
111            |event: On<Pointer<Press>>,
112             menu_items: Query<&ContextMenuItem>,
113             mut clear_col: ResMut<ClearColor>,
114             mut commands: Commands| {
115                let target = event.original_event_target();
116
117                if let Ok(item) = menu_items.get(target) {
118                    clear_col.0 = item.0.into();
119                    commands.trigger(CloseContextMenus);
120                }
121            },
122        );
123}
124
125fn context_item(text: &str, col: Srgba) -> impl Bundle {
126    (
127        Name::new(format!("item-{text}")),
128        ContextMenuItem(col),
129        Button,
130        Node {
131            padding: UiRect::all(px(5)),
132            ..default()
133        },
134        children![(
135            Pickable::IGNORE,
136            Text::new(text),
137            TextFont {
138                font_size: FontSize::Px(24.0),
139                ..default()
140            },
141            TextColor(Color::WHITE),
142        )],
143    )
144}
145
146fn background_and_button() -> impl Bundle {
147    (
148        Name::new("background"),
149        Node {
150            width: percent(100),
151            height: percent(100),
152            align_items: AlignItems::Center,
153            justify_content: JustifyContent::Center,
154            ..default()
155        },
156        ZIndex(-10),
157        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
158            parent
159                .spawn((
160                    Name::new("button"),
161                    Button,
162                    Node {
163                        width: px(250),
164                        height: px(65),
165                        border: UiRect::all(px(5)),
166                        justify_content: JustifyContent::Center,
167                        align_items: AlignItems::Center,
168                        border_radius: BorderRadius::MAX,
169                        ..default()
170                    },
171                    BorderColor::all(Color::BLACK),
172                    BackgroundColor(Color::BLACK),
173                    children![(
174                        Pickable::IGNORE,
175                        Text::new("Context Menu"),
176                        TextFont {
177                            font_size: FontSize::Px(28.0),
178                            ..default()
179                        },
180                        TextColor(Color::WHITE),
181                        TextShadow::default(),
182                    )],
183                ))
184                .observe(|mut event: On<Pointer<Press>>, mut commands: Commands| {
185                    // by default this event would bubble up further leading to the `CloseContextMenus`
186                    // event being triggered and undoing the opening of one here right away.
187                    event.propagate(false);
188
189                    debug!("click: {}", event.pointer_location.position);
190
191                    commands.trigger(OpenContextMenu {
192                        pos: event.pointer_location.position,
193                    });
194                });
195        })),
196    )
197}
```

examples/ecs/observer\_propagation.rs ([line 74](../../../src/observer_propagation/observer_propagation.rs.html#74))

```rust
70fn attack_armor(entities: Query<Entity, With<Armor>>, mut commands: Commands) {
71    let mut rng = rng();
72    if let Some(entity) = entities.iter().choose(&mut rng) {
73        let damage = rng.random_range(1..20);
74        commands.trigger(Attack { damage, entity });
75        info!("⚔️  Attack for {} damage", damage);
76    }
77}
```

examples/ui/scroll\_and\_overflow/scroll.rs ([line 46](../../../src/scroll/scroll.rs.html#46))

```rust
27fn send_scroll_events(
28    mut mouse_wheel_reader: MessageReader<MouseWheel>,
29    hover_map: Res<HoverMap>,
30    keyboard_input: Res<ButtonInput<KeyCode>>,
31    mut commands: Commands,
32) {
33    for mouse_wheel in mouse_wheel_reader.read() {
34        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);
35
36        if mouse_wheel.unit == MouseScrollUnit::Line {
37            delta *= LINE_HEIGHT;
38        }
39
40        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
41            std::mem::swap(&mut delta.x, &mut delta.y);
42        }
43
44        for pointer_map in hover_map.values() {
45            for entity in pointer_map.keys().copied() {
46                commands.trigger(Scroll { entity, delta });
47            }
48        }
49    }
50}
```

examples/ui/navigation/directional\_navigation.rs ([lines 449-471](../../../src/directional_navigation/directional_navigation.rs.html#449-471))

```rust
439fn interact_with_focused_button(
440    action_state: Res<ActionState>,
441    input_focus: Res<InputFocus>,
442    mut commands: Commands,
443) {
444    if action_state
445        .pressed_actions
446        .contains(&DirectionalNavigationAction::Select)
447        && let Some(focused_entity) = input_focus.get()
448    {
449        commands.trigger(Pointer::new(
450            PointerId::Mouse,
451            Location {
452                target: NormalizedRenderTarget::None {
453                    width: 0,
454                    height: 0,
455                },
456                position: Vec2::ZERO,
457            },
458            Click {
459                button: PointerButton::Primary,
460                hit: HitData {
461                    camera: Entity::PLACEHOLDER,
462                    depth: 0.0,
463                    position: None,
464                    normal: None,
465                    extra: None,
466                },
467                count: 1,
468                duration: Duration::from_secs_f32(0.1),
469            },
470            focused_entity,
471        ));
472    }
473}
```

examples/ui/navigation/directional\_navigation\_overrides.rs ([lines 842-864](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#842-864))

```rust
832fn interact_with_focused_button(
833    action_state: Res<ActionState>,
834    input_focus: Res<InputFocus>,
835    mut commands: Commands,
836) {
837    if action_state
838        .pressed_actions
839        .contains(&DirectionalNavigationAction::Select)
840        && let Some(focused_entity) = input_focus.get()
841    {
842        commands.trigger(Pointer::new(
843            PointerId::Mouse,
844            Location {
845                target: NormalizedRenderTarget::None {
846                    width: 0,
847                    height: 0,
848                },
849                position: Vec2::ZERO,
850            },
851            Click {
852                button: PointerButton::Primary,
853                hit: HitData {
854                    camera: Entity::PLACEHOLDER,
855                    depth: 0.0,
856                    position: None,
857                    normal: None,
858                    extra: None,
859                },
860                count: 1,
861                duration: Duration::from_secs_f32(0.1),
862            },
863            focused_entity,
864        ));
865    }
866}
```

Additional examples can be found in:  

*   [examples/ui/images/image\_node\_resizing.rs](../../../src/image_node_resizing/image_node_resizing.rs.html#186)
*   [examples/ecs/observers.rs](../../../src/observers/observers.rs.html#32)
*   [examples/showcase/breakout.rs](../../../src/breakout/breakout.rs.html#353)
*   [examples/ui/widgets/feathers\_gallery.rs](../../../src/feathers_gallery/feathers_gallery.rs.html#820-823)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1178-1182)

#### pub fn [trigger\_with](#method.trigger_with)<E>( &mut self, event: E, trigger: <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'static>, )

where E: [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"), <E as [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'static>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

Triggers the given [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") using the given [`Trigger`](../event/trait.Trigger.html "trait bevy::ecs::event::Trigger"), which will run any [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1200)

#### pub fn [add\_observer](#method.add_observer)<M>( &mut self, observer: impl [IntoObserver](../observer/trait.IntoObserver.html "trait bevy::ecs::observer::IntoObserver")<M>, ) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

Spawns an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") and returns the [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") associated with the entity that stores the observer.

`observer` can be any system whose first parameter is [`On`](../../prelude/struct.On.html "struct bevy::prelude::On").

**Calling [`observe`](../../prelude/struct.EntityCommands.html#method.observe "method bevy::prelude::EntityCommands::observe") on the returned [`EntityCommands`](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") will observe the observer itself, which you very likely do not want.**

##### Panics

Panics if the given system is an exclusive system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1217)

#### pub fn [write\_message](#method.write_message)<M>(&mut self, message: M) -> &mut [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

Writes an arbitrary [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message").

This is a convenience method for writing messages without requiring a [`MessageWriter`](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter").

##### Performance

Since this is a command, exclusive world access is used, which means that it will not profit from system-level parallelism on supported platforms.

If these messages are performance-critical or very frequently sent, consider using a [`MessageWriter`](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter") instead.

##### [Examples found in repository](#scraped-examples-15)[?](../../../scrape-examples-help.html)

tests/window/desktop\_request\_redraw.rs ([line 105](../../../src/desktop_request_redraw/desktop_request_redraw.rs.html#105))

```rust
103fn redraw(mut commands: Commands, query: Query<Entity, With<AnimationActive>>) {
104    if query.iter().next().is_some() {
105        commands.write_message(RequestRedraw);
106    }
107}
```

Hide additional examples

examples/app/settings.rs ([line 135](../../../src/settings/settings.rs.html#135))

```rust
131fn on_window_close(mut close: MessageReader<WindowCloseRequested>, mut commands: Commands) {
132    // Save settings immediately, then quit.
133    if let Some(_close_event) = close.read().next() {
134        commands.queue(SaveSettingsSync::IfChanged);
135        commands.write_message(AppExit::Success);
136    }
137}
```

examples/window/persisting\_window\_settings.rs ([line 133](../../../src/persisting_window_settings/persisting_window_settings.rs.html#133))

```rust
129fn on_window_close(mut close: MessageReader<WindowCloseRequested>, mut commands: Commands) {
130    // Save settings immediately, then quit.
131    if let Some(_close_event) = close.read().next() {
132        commands.queue(SaveSettingsSync::IfChanged);
133        commands.write_message(AppExit::Success);
134    }
135}
```

examples/3d/clustered\_decals.rs ([line 166](../../../src/clustered_decals/clustered_decals.rs.html#166))

```rust
154fn setup(
155    mut commands: Commands,
156    asset_server: Res<AssetServer>,
157    app_status: Res<AppStatus>,
158    render_device: Res<RenderDevice>,
159    render_adapter: Res<RenderAdapter>,
160    mut meshes: ResMut<Assets<Mesh>>,
161    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CustomDecalExtension>>>,
162) {
163    // Error out if clustered decals aren't supported on the current platform.
164    if !decal::clustered::clustered_decals_are_usable(&render_device, &render_adapter) {
165        error!("Clustered decals aren't usable on this platform.");
166        commands.write_message(AppExit::error());
167    }
168
169    spawn_cube(&mut commands, &mut meshes, &mut materials);
170    spawn_camera(&mut commands);
171    spawn_light(&mut commands);
172    spawn_decals(&mut commands, &asset_server);
173    spawn_buttons(&mut commands);
174    spawn_help_text(&mut commands, &app_status);
175}
```

examples/3d/light\_textures.rs ([line 155](../../../src/light_textures/light_textures.rs.html#155))

```rust
143fn setup(
144    mut commands: Commands,
145    asset_server: Res<AssetServer>,
146    app_status: Res<AppStatus>,
147    render_device: Res<RenderDevice>,
148    render_adapter: Res<RenderAdapter>,
149    mut meshes: ResMut<Assets<Mesh>>,
150    mut materials: ResMut<Assets<StandardMaterial>>,
151) {
152    // Error out if clustered decals (and so light textures) aren't supported on the current platform.
153    if !decal::clustered::clustered_decals_are_usable(&render_device, &render_adapter) {
154        error!("Light textures aren't usable on this platform.");
155        commands.write_message(AppExit::error());
156    }
157
158    spawn_cubes(&mut commands, &mut meshes, &mut materials);
159    spawn_camera(&mut commands);
160    spawn_light(&mut commands, &asset_server);
161    spawn_buttons(&mut commands);
162    spawn_help_text(&mut commands, &app_status);
163    spawn_light_textures(&mut commands, &asset_server, &mut meshes, &mut materials);
164}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1265)

#### pub fn [run\_schedule](#method.run_schedule)(&mut self, label: impl [ScheduleLabel](../schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"))

Runs the schedule corresponding to the given [`ScheduleLabel`](../schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel").

Calls [`World::try_run_schedule`](../../prelude/struct.World.html#method.try_run_schedule "method bevy::prelude::World::try_run_schedule").

##### Fallible

This command will fail if the given [`ScheduleLabel`](../schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") does not correspond to a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule").

It will internally return a [`TryRunScheduleError`](../world/error/struct.TryRunScheduleError.html "struct bevy::ecs::world::error::TryRunScheduleError"), which will be handled by [logging the error at the `warn` level](../error/fn.warn.html "fn bevy::ecs::error::warn").

##### Example

```rust
#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct FooSchedule;

commands.run_schedule(FooSchedule);
```

## Trait Implementations

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#372)

### impl<'w, 's> [CommandsSceneExt](../../prelude/trait.CommandsSceneExt.html "trait bevy::prelude::CommandsSceneExt") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#373)

#### fn [spawn\_scene](../../prelude/trait.CommandsSceneExt.html#tymethod.spawn_scene)<S>(&mut self, scene: S) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Spawns the given [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") as soon as [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") are applied. This will resolve the Scene (using [`Scene::resolve`](../../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve")). If that fails (for example, if there are dependencies that have not been loaded yet), it will log a [`SpawnSceneError`](../../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") as an error. If resolving the [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is successful, the scene will be spawned. [Read more](../../prelude/trait.CommandsSceneExt.html#tymethod.spawn_scene)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#386)

#### fn [queue\_spawn\_scene](../../prelude/trait.CommandsSceneExt.html#tymethod.queue_spawn_scene)<S>(&mut self, scene: S) -> [EntityCommands](../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Queues the `scene` to be spawned. This will evaluate the `scene`’s dependencies (via [`Scene::register_dependencies`](../../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](../../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error. [Read more](../../prelude/trait.CommandsSceneExt.html#tymethod.queue_spawn_scene)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#397)

#### fn [spawn\_scene\_list](../../prelude/trait.CommandsSceneExt.html#tymethod.spawn_scene_list)<L>(&mut self, scenes: L)

where L: [SceneList](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

Spawns the given [`SceneList`](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") as soon as [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") are applied. This will resolve the scene list (using [`SceneList::resolve_list`](../../prelude/trait.SceneList.html#tymethod.resolve_list "method bevy::prelude::SceneList::resolve_list")). If that fails (for example, if there are dependencies that have not been loaded yet), it will log a [`SpawnSceneError`](../../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") as an error. If resolving the [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is successful, the scene list will be spawned. [Read more](../../prelude/trait.CommandsSceneExt.html#tymethod.spawn_scene_list)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#405)

#### fn [queue\_spawn\_scene\_list](../../prelude/trait.CommandsSceneExt.html#tymethod.queue_spawn_scene_list)<L>(&mut self, scenes: L)

where L: [SceneList](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

Queues the `scene_list` to be spawned. This will evaluate the `scene_list`’s dependencies (via [`Scene::register_dependencies`](../../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](../../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error. [Read more](../../prelude/trait.CommandsSceneExt.html#tymethod.queue_spawn_scene_list)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/commands.rs.html#27)

### impl [CommandsStatesExt](../../prelude/trait.CommandsStatesExt.html "trait bevy::prelude::CommandsStatesExt") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/commands.rs.html#28)

#### fn [set\_state](../../prelude/trait.CommandsStatesExt.html#tymethod.set_state)<S>(&mut self, state: S)

where S: [FreelyMutableState](../../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState"),

Sets the next state the app should move to. [Read more](../../prelude/trait.CommandsStatesExt.html#tymethod.set_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/commands.rs.html#38)

#### fn [set\_state\_if\_neq](../../prelude/trait.CommandsStatesExt.html#tymethod.set_state_if_neq)<S>(&mut self, state: S)

where S: [FreelyMutableState](../../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState"),

Sets the next state the app should move to, skipping any state transitions if the next state is the same as the current state. [Read more](../../prelude/trait.CommandsStatesExt.html#tymethod.set_state_if_neq)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#110)

### impl<'w, 's> [DelayedCommandsExt](../../prelude/trait.DelayedCommandsExt.html "trait bevy::prelude::DelayedCommandsExt")<'w> for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#111)

#### fn [delayed](../../prelude/trait.DelayedCommandsExt.html#tymethod.delayed)(&mut self) -> [DelayedCommands](../../time/struct.DelayedCommands.html "struct bevy::time::DelayedCommands")<'w, '\_>

Returns a [`DelayedCommands`](../../time/struct.DelayedCommands.html "struct bevy::time::DelayedCommands") instance that can be used to queue commands to be submitted at a later point in time. [Read more](../../prelude/trait.DelayedCommandsExt.html#tymethod.delayed)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#205-208)

### impl<'w, 's> [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

where [Deferred](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred")<'s, [CommandQueue](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue")\>: [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"), &'w [Entities](../entity/struct.Entities.html "struct bevy::ecs::entity::Entities"): [ReadOnlySystemParam](trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#112)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#115)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#128)

### impl [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#129)

#### type [State](trait.SystemParam.html#associatedtype.State) = FetchState

Used to store data which persists across invocations of a system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#131)

#### type [Item](trait.SystemParam.html#associatedtype.Item)<'w, 's> = [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

The item type returned when constructing this system param. The value of this associated type should be `Self`, instantiated with new lifetimes. [Read more](trait.SystemParam.html#associatedtype.Item)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#134)

#### fn [init\_state](trait.SystemParam.html#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

Creates a new instance of this param’s [`State`](trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#142-147)

#### fn [init\_access](trait.SystemParam.html#tymethod.init_access)( state: &<[Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

Registers any [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") access used by this [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"). [Read more](trait.SystemParam.html#tymethod.init_access)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#156-160)

#### fn [apply](trait.SystemParam.html#method.apply)( state: &mut <[Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

Applies any deferred mutations stored in this [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")’s state. This is used to apply [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") during [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#168-172)

#### fn [queue](trait.SystemParam.html#method.queue)( state: &mut <[Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

Queues any deferred mutations to be applied at the next [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#182-187)

#### unsafe fn [get\_param](trait.SystemParam.html#tymethod.get_param)<'w, 's>( state: &'s mut <[Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'w, 's>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

Creates a parameter to be passed into a [`SystemParamFunction`](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction"). [Read more](trait.SystemParam.html#tymethod.get_param)

## Auto Trait Implementations

### impl<'w, 's> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

### impl<'w, 's> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

### impl<'w, 's> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

### impl<'w, 's> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

### impl<'w, 's> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>

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

### impl<T> [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

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