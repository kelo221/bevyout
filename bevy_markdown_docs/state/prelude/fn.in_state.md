[bevy](../../index.html)::[state](../index.html)::[prelude](index.html)

# Function in\_state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/condition.rs.html#103)

```rust
pub fn in_state<S>(state: S) -> impl FnMut(Option<Res<'_, State<S>>>) + Clonewhere
    S: States,
```

Generates a [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the state machine is currently in `state`.

Will return `false` if the state does not exist or if not in `state`.

## Example

```rust
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum GameState {
    #[default]
    Playing,
    Paused,
}

app
    .init_state::<GameState>()
    .add_systems(Update, (
        // `in_state` will only return true if the
        // given state equals the given value
        play_system.run_if(in_state(GameState::Playing)),
        pause_system.run_if(in_state(GameState::Paused)),
    ));

fn play_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

fn pause_system(mut counter: ResMut<Counter>) {
    counter.0 -= 1;
}

// We default to `GameState::Playing` so `play_system` runs
app.update();
assert_eq!(app.world().resource::<Counter>().0, 1);

app.insert_state(GameState::Paused);

// Now that we are in `GameState::Pause`, `pause_system` will run
app.update();
assert_eq!(app.world().resource::<Counter>().0, 0);
```

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/2d/texture\_atlas.rs ([line 17](../../../src/texture_atlas/texture_atlas.rs.html#17))

```rust
12fn main() {
13    App::new()
14        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // fallback to nearest sampling
15        .init_state::<AppState>()
16        .add_systems(OnEnter(AppState::Setup), load_textures)
17        .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
18        .add_systems(OnEnter(AppState::Finished), setup)
19        .run();
20}
```

Hide additional examples

examples/showcase/game\_menu.rs ([line 63](../../../src/game_menu/game_menu.rs.html#63))

```rust
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
```

examples/ecs/generic\_system.rs ([line 42](../../../src/generic_system/generic_system.rs.html#42))

```rust
33fn main() {
34    App::new()
35        .add_plugins(DefaultPlugins)
36        .init_state::<AppState>()
37        .add_systems(Startup, setup_system)
38        .add_systems(
39            Update,
40            (
41                print_text_system,
42                transition_to_in_game_system.run_if(in_state(AppState::MainMenu)),
43            ),
44        )
45        // Cleanup systems.
46        // Pass in the types your system should operate on using the ::<T> (turbofish) syntax
47        .add_systems(OnExit(AppState::MainMenu), cleanup_system::<MenuClose>)
48        .add_systems(OnExit(AppState::InGame), cleanup_system::<LevelUnload>)
49        .run();
50}
```

examples/showcase/alien\_cake\_addict.rs ([line 40](../../../src/alien_cake_addict/alien_cake_addict.rs.html#40))

```rust
20fn main() {
21    App::new()
22        .add_plugins(DefaultPlugins)
23        .init_resource::<Game>()
24        .insert_resource(BonusSpawnTimer(Timer::from_seconds(
25            5.0,
26            TimerMode::Repeating,
27        )))
28        .init_state::<GameState>()
29        .add_systems(Startup, setup_cameras)
30        .add_systems(OnEnter(GameState::Playing), setup)
31        .add_systems(
32            Update,
33            (
34                move_player,
35                focus_camera,
36                rotate_bonus,
37                scoreboard_system,
38                spawn_bonus,
39            )
40                .run_if(in_state(GameState::Playing)),
41        )
42        .add_systems(OnEnter(GameState::GameOver), display_score)
43        .add_systems(
44            Update,
45            game_over_keyboard.run_if(in_state(GameState::GameOver)),
46        )
47        .run();
48}
```

examples/math/bounding\_2d.rs ([line 23](../../../src/bounding_2d/bounding_2d.rs.html#23))

```rust
9fn main() {
10    App::new()
11        .add_plugins(DefaultPlugins)
12        .init_state::<Test>()
13        .add_systems(Startup, setup)
14        .add_systems(
15            Update,
16            (update_text, spin, update_volumes, update_test_state),
17        )
18        .add_systems(
19            PostUpdate,
20            (
21                render_shapes,
22                (
23                    aabb_intersection_system.run_if(in_state(Test::AabbSweep)),
24                    circle_intersection_system.run_if(in_state(Test::CircleSweep)),
25                    ray_cast_system.run_if(in_state(Test::RayCast)),
26                    aabb_cast_system.run_if(in_state(Test::AabbCast)),
27                    bounding_circle_cast_system.run_if(in_state(Test::CircleCast)),
28                ),
29                render_volumes,
30            )
31                .chain(),
32        )
33        .run();
34}
```

examples/state/custom\_transitions.rs ([line 37](../../../src/custom_transitions/custom_transitions.rs.html#37))

```rust
27fn main() {
28    App::new()
29        // We insert the custom transitions plugin for `AppState`.
30        .add_plugins((
31            DefaultPlugins,
32            IdentityTransitionsPlugin::<AppState>::default(),
33        ))
34        .init_state::<AppState>()
35        .add_systems(Startup, setup)
36        .add_systems(OnEnter(AppState::Menu), setup_menu)
37        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
38        .add_systems(OnExit(AppState::Menu), cleanup_menu)
39        // We will restart the game progress every time we re-enter into it.
40        .add_systems(OnReenter(AppState::InGame), setup_game)
41        .add_systems(OnReexit(AppState::InGame), teardown_game)
42        // Doing it this way allows us to restart the game without any additional in-between states.
43        .add_systems(
44            Update,
45            ((movement, change_color, trigger_game_restart).run_if(in_state(AppState::InGame)),),
46        )
47        .add_systems(Update, log_transitions::<AppState>)
48        .run();
49}
```

Additional examples can be found in:  

*   [examples/asset/multi\_asset\_sync.rs](../../../src/multi_asset_sync/multi_asset_sync.rs.html#35)
*   [examples/state/sub\_states.rs](../../../src/sub_states/sub_states.rs.html#43)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#40)
*   [examples/state/states.rs](../../../src/states/states.rs.html#21)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#45)
*   [examples/state/computed\_states.rs](../../../src/computed_states/computed_states.rs.html#187)