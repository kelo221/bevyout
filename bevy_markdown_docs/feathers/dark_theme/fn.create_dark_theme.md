[bevy](../../index.html)::[feathers](../index.html)::[dark\_theme](index.html)

# Function create\_dark\_theme 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/dark_theme.rs.html#9)

```rust
pub fn create_dark_theme() -> ThemeProps
```

Create a [`ThemeProps`](../theme/struct.ThemeProps.html "struct bevy::feathers::theme::ThemeProps") object and populate it with the colors for the default dark theme.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ui/widgets/virtual\_keyboard.rs ([line 17](../../../src/virtual_keyboard/virtual_keyboard.rs.html#17))

```rust
14fn main() {
15    App::new()
16        .add_plugins((DefaultPlugins, FeathersPlugins))
17        .insert_resource(UiTheme(create_dark_theme()))
18        .add_systems(Startup, scene.spawn())
19        .run();
20}
```

Hide additional examples

examples/ui/widgets/feathers\_gallery.rs ([line 67](../../../src/feathers_gallery/feathers_gallery.rs.html#67))

```rust
64fn main() {
65    App::new()
66        .add_plugins((DefaultPlugins, FeathersPlugins))
67        .insert_resource(UiTheme(create_dark_theme()))
68        .insert_resource(DemoWidgetStates {
69            rgb_color: palettes::tailwind::EMERALD_800.with_alpha(0.7),
70            hsl_color: palettes::tailwind::AMBER_800.into(),
71            scalar_prop: 7.0,
72            vec3_prop: Vec3::new(10.1, 7.124, 100.0),
73        })
74        .add_systems(Startup, scene.spawn())
75        .add_systems(Update, update_colors)
76        .run();
77}
```

examples/ui/widgets/feathers\_counter.rs ([line 31](../../../src/feathers_counter/feathers_counter.rs.html#31))

```rust
22fn main() {
23    App::new()
24        .add_plugins((
25            DefaultPlugins,
26            // Don't forget to add the plugin.
27            // Make sure you are using FeathersPlugins with an `s`
28            FeathersPlugins,
29        ))
30        // Configure feathers to use the dark theme
31        .insert_resource(UiTheme(create_dark_theme()))
32        .insert_resource(Counter(0))
33        .add_systems(Startup, scene.spawn())
34        .add_systems(
35            Update,
36            update_counter_text.run_if(resource_changed::<Counter>),
37        )
38        .run();
39}
```