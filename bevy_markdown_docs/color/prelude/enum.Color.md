[bevy](../../index.html)::[color](../index.html)::[prelude](index.html)

# Enum Color 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#56)

```rust
pub enum Color {
    Srgba(Srgba),
    LinearRgba(LinearRgba),
    Hsla(Hsla),
    Hsva(Hsva),
    Hwba(Hwba),
    Laba(Laba),
    Lcha(Lcha),
    Oklaba(Oklaba),
    Oklcha(Oklcha),
    Xyza(Xyza),
}
```

An enumerated type that can represent any of the color types in this crate.

This is useful when you need to store a color in a data structure that can’t be generic over the color type.

## Conversion

Conversion between the various color spaces is achieved using Rust’s native [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") trait. Because certain color spaces are defined by their transformation to and from another space, these [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") implementations reflect that set of definitions.

```rust
let color = Srgba::rgb(0.5, 0.5, 0.5);

// Using From explicitly
let linear_color = LinearRgba::from(color);

// Using Into
let linear_color: LinearRgba = color.into();
```

For example, the [sRGB](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") space is defined by its relationship with [Linear RGB](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba"), and [HWB](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") by its with [sRGB](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba"). As such, it is the responsibility of [sRGB](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") to provide [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") implementations for [Linear RGB](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba"), and [HWB](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") for [sRGB](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba"). To then provide conversion between [Linear RGB](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba") and [HWB](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") directly, [HWB](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") is responsible for implementing these conversions, delegating to [sRGB](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") as an intermediatory. This ensures that all conversions take the shortest path between any two spaces, and limit the proliferation of domain specific knowledge for each color space to their respective definitions.

@import url("https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css");'#graph-div{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:16px;fill:#ccc;}#graph-div .error-icon{fill:#a44141;}#graph-div .error-text{fill:#ddd;stroke:#ddd;}#graph-div .edge-thickness-normal{stroke-width:2px;}#graph-div .edge-thickness-thick{stroke-width:3.5px;}#graph-div .edge-pattern-solid{stroke-dasharray:0;}#graph-div .edge-pattern-dashed{stroke-dasharray:3;}#graph-div .edge-pattern-dotted{stroke-dasharray:2;}#graph-div .marker{fill:lightgrey;stroke:lightgrey;}#graph-div .marker.cross{stroke:lightgrey;}#graph-div svg{font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:16px;}#graph-div .label{font-family:"trebuchet ms",verdana,arial,sans-serif;color:#ccc;}#graph-div .cluster-label text{fill:#F9FFFE;}#graph-div .cluster-label span,#graph-div p{color:#F9FFFE;}#graph-div .label text,#graph-div span,#graph-div p{fill:#ccc;color:#ccc;}#graph-div .node rect,#graph-div .node circle,#graph-div .node ellipse,#graph-div .node polygon,#graph-div .node path{fill:#1f2020;stroke:#81B1DB;stroke-width:1px;}#graph-div .flowchart-label text{text-anchor:middle;}#graph-div .node .label{text-align:center;}#graph-div .node.clickable{cursor:pointer;}#graph-div .arrowheadPath{fill:lightgrey;}#graph-div .edgePath .path{stroke:lightgrey;stroke-width:2.0px;}#graph-div .flowchart-link{stroke:lightgrey;fill:none;}#graph-div .edgeLabel{background-color:hsl(0, 0%, 34.4117647059%);text-align:center;}#graph-div .edgeLabel rect{opacity:0.5;background-color:hsl(0, 0%, 34.4117647059%);fill:hsl(0, 0%, 34.4117647059%);}#graph-div .labelBkg{background-color:rgba(87.75, 87.75, 87.75, 0.5);}#graph-div .cluster rect{fill:hsl(180, 1.5873015873%, 28.3529411765%);stroke:rgba(255, 255, 255, 0.25);stroke-width:1px;}#graph-div .cluster text{fill:#F9FFFE;}#graph-div .cluster span,#graph-div p{color:#F9FFFE;}#graph-div div.mermaidTooltip{position:absolute;text-align:center;max-width:200px;padding:2px;font-family:"trebuchet ms",verdana,arial,sans-serif;font-size:12px;background:hsl(20, 1.5873015873%, 12.3529411765%);border:1px solid rgba(255, 255, 255, 0.25);border-radius:2px;pointer-events:none;z-index:100;}#graph-div .flowchartTitleText{text-anchor:middle;font-size:18px;fill:#ccc;}#graph-div .label foreignObject{overflow:visible;}#graph-div :root{--mermaid-font-family:"trebuchet ms",verdana,arial,sans-serif;}

[Conversion](https://bottosson.github.io/posts/oklab/#converting-from-linear-srgb-to-oklab)

[Conversion](https://bottosson.github.io/posts/oklab/#the-oklab-color-space)

[Conversion](http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html)

[Conversion](http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Lab.html)

[Conversion](http://www.brucelindbloom.com/index.html?Eqn_Lab_to_LCH.html)

[Conversion](https://en.wikipedia.org/wiki/SRGB#From_sRGB_to_CIE_XYZ)

[Conversion](http://alvyray.com/Papers/CG/HWB_JGTv208.pdf)

[Conversion](http://alvyray.com/Papers/CG/HWB_JGTv208.pdf)

[Conversion](https://en.wikipedia.org/wiki/HSL_and_HSV#Interconversion)

[Linear  
sRGB](https://en.wikipedia.org/wiki/Rgb)

[Oklab](https://oklch.com/)

[Oklch](https://oklch.com/)

[XYZ](https://en.wikipedia.org/wiki/XYZ_color)

[Lab](https://en.wikipedia.org/wiki/Lab_color)

[Lch](https://en.wikipedia.org/wiki/CIELAB_color_space#Cylindrical_model)

[sRGB](https://en.wikipedia.org/wiki/Srgb)

[HWB](https://en.wikipedia.org/wiki/HWB_color_model)

[HSV](https://en.wikipedia.org/wiki/HSL_and_HSV)

[HSL](https://en.wikipedia.org/wiki/HSL_and_HSV)

GPU

## Operations

[`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") supports all the standard color operations, such as [mixing](../../prelude/trait.Mix.html "trait bevy::prelude::Mix"), [luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") and [hue](../../prelude/trait.Hue.html "trait bevy::prelude::Hue") adjustment, and [diffing](../color_difference/trait.EuclideanDistance.html "trait bevy::color::color_difference::EuclideanDistance"). These operations delegate to the concrete color space contained by [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color"), but will convert to [`Oklch`](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha") for operations which aren’t supported in the current space. After performing the operation, if a conversion was required, the result will be converted back into the original color space.

```rust
let red_hsv = Color::hsv(0., 1., 1.);
let red_srgb = Color::srgb(1., 0., 0.);

// HSV has a definition of hue, so it will be returned.
red_hsv.hue();

// SRGB doesn't have a native definition for hue.
// Converts to Oklch and returns that result.
red_srgb.hue();
```

[`Oklch`](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha") has been chosen as the intermediary space in cases where conversion is required due to its perceptual uniformity and broad support for Bevy’s color operations. To avoid the cost of repeated conversion, and ensure consistent results where that is desired, first convert this [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") into your desired color space.

## Variants

### Srgba([Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba"))

A color in the sRGB color space with alpha.

### LinearRgba([LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba"))

A color in the linear sRGB color space with alpha.

### Hsla([Hsla](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla"))

A color in the HSL color space with alpha.

### Hsva([Hsva](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva"))

A color in the HSV color space with alpha.

### Hwba([Hwba](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba"))

A color in the HWB color space with alpha.

### Laba([Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba"))

A color in the LAB color space with alpha.

### Lcha([Lcha](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha"))

A color in the LCH color space with alpha.

### Oklaba([Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba"))

A color in the Oklab color space with alpha.

### Oklcha([Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha"))

A color in the Oklch color space with alpha.

### Xyza([Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza"))

A color in the XYZ color space with alpha.

## Implementations

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#81)

### impl [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#503)

#### pub const [WHITE](#associatedconstant.WHITE): [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

A fully white [`Color::LinearRgba`](../../prelude/enum.Color.html#variant.LinearRgba "variant bevy::prelude::Color::LinearRgba") color with an alpha of 1.0.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#506)

#### pub const [BLACK](#associatedconstant.BLACK): [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

A fully black [`Color::LinearRgba`](../../prelude/enum.Color.html#variant.LinearRgba "variant bevy::prelude::Color::LinearRgba") color with an alpha of 1.0.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#509)

#### pub const [NONE](#associatedconstant.NONE): [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

A fully transparent [`Color::LinearRgba`](../../prelude/enum.Color.html#variant.LinearRgba "variant bevy::prelude::Color::LinearRgba") color with 0 red, green and blue.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#83)

#### pub fn [to\_linear](#method.to_linear)(&self) -> [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

Return the color as a linear RGBA color.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ui/ui\_material.rs ([line 101](../../../src/ui_material/ui_material.rs.html#101))

```rust
90fn animate(
91    mut materials: ResMut<Assets<CustomUiMaterial>>,
92    q: Query<&MaterialNode<CustomUiMaterial>>,
93    time: Res<Time>,
94) {
95    let duration = 2.0;
96    for handle in &q {
97        if let Some(mut material) = materials.get_mut(handle) {
98            // rainbow color effect
99            let new_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 1., 0.5);
100            let border_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 0.75, 0.75);
101            material.color = new_color.to_linear().to_vec4();
102            material.slider.x =
103                ((time.elapsed_secs() % (duration * 2.0)) - duration).abs() / duration;
104            material.border_color = border_color.to_linear().to_vec4();
105        }
106    }
107}
```

Hide additional examples

examples/3d/light\_textures.rs ([line 262](../../../src/light_textures/light_textures.rs.html#262))

```rust
228fn spawn_light_textures(
229    commands: &mut Commands,
230    asset_server: &AssetServer,
231    meshes: &mut Assets<Mesh>,
232    materials: &mut Assets<StandardMaterial>,
233) {
234    commands.spawn((
235        SpotLight {
236            color: Color::srgb(1.0, 1.0, 0.8),
237            intensity: 10e6,
238            outer_angle: 0.25,
239            inner_angle: 0.25,
240            shadow_maps_enabled: true,
241            ..default()
242        },
243        Transform::from_translation(Vec3::new(6.0, 1.0, 2.0)).looking_at(Vec3::ZERO, Vec3::Y),
244        SpotLightTexture {
245            image: asset_server.load("lightmaps/torch_spotlight_texture.png"),
246        },
247        Visibility::Inherited,
248        Selection::SpotLight,
249    ));
250
251    commands.spawn((
252        Visibility::Hidden,
253        Transform::from_translation(Vec3::new(0.0, 1.8, 0.01)).with_scale(Vec3::splat(0.1)),
254        Selection::PointLight,
255        children![
256            WorldAssetRoot(
257                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Faces/faces.glb")),
258            ),
259            (
260                Mesh3d(meshes.add(Sphere::new(1.0))),
261                MeshMaterial3d(materials.add(StandardMaterial {
262                    emissive: Color::srgb(0.0, 0.0, 300.0).to_linear(),
263                    ..default()
264                })),
265            ),
266            (
267                PointLight {
268                    color: Color::srgb(0.0, 0.0, 1.0),
269                    intensity: 1e6,
270                    shadow_maps_enabled: true,
271                    ..default()
272                },
273                PointLightTexture {
274                    image: asset_server.load("lightmaps/faces_pointlight_texture_blurred.png"),
275                    cubemap_layout: CubemapLayout::CrossVertical,
276                },
277            )
278        ],
279    ));
280}
```

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#88)

#### pub fn [to\_srgba](#method.to_srgba)(&self) -> [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

Return the color as an SRGBA color.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/2d/wireframe\_2d.rs ([line 119](../../../src/wireframe_2d/wireframe_2d.rs.html#119))

```rust
101fn update_colors(
102    keyboard_input: Res<ButtonInput<KeyCode>>,
103    mut config: ResMut<Wireframe2dConfig>,
104    mut wireframe_colors: Query<&mut Wireframe2dColor>,
105    mut text: Single<&mut Text>,
106) {
107    text.0 = format!(
108        "Controls
109---------------
110Z - Toggle global
111X - Change global color
112C - Change color of the circle wireframe
113
114Wireframe2dConfig
115-------------
116Global: {}
117Color: {:?}",
118        config.global,
119        config.default_color.to_srgba(),
120    );
121
122    // Toggle showing a wireframe on all meshes
123    if keyboard_input.just_pressed(KeyCode::KeyZ) {
124        config.global = !config.global;
125    }
126
127    // Toggle the global wireframe color
128    if keyboard_input.just_pressed(KeyCode::KeyX) {
129        config.default_color = if config.default_color == WHITE.into() {
130            RED.into()
131        } else {
132            WHITE.into()
133        };
134    }
135
136    // Toggle the color of a wireframe using `Wireframe2dColor` and not the global color
137    if keyboard_input.just_pressed(KeyCode::KeyC) {
138        for mut color in &mut wireframe_colors {
139            color.color = if color.color == GREEN.into() {
140                RED.into()
141            } else {
142                GREEN.into()
143            };
144        }
145    }
146}
```

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#100)

#### pub const fn [srgba](#method.srgba)(red: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), green: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), blue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Srgba`](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") color.

##### Arguments

*   `red` - Red channel. \[0.0, 1.0\]
*   `green` - Green channel. \[0.0, 1.0\]
*   `blue` - Blue channel. \[0.0, 1.0\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/picking/dragdrop\_picking.rs ([line 116](../../../src/dragdrop_picking/dragdrop_picking.rs.html#116))

```rust
102fn on_drag_enter(
103    mut event: On<Pointer<DragEnter>>,
104    button: Single<Entity, With<DraggableButton>>,
105    mut commands: Commands,
106    mut meshes: ResMut<Assets<Mesh>>,
107    mut materials: ResMut<Assets<ColorMaterial>>,
108) {
109    if event.dragged == *button {
110        let Some(position) = event.hit.position else {
111            return;
112        };
113        commands.spawn((
114            GhostPreview,
115            Mesh2d(meshes.add(Circle::new(ELEMENT_SIZE))),
116            MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 0.6, 0.5))),
117            Transform::from_translation(position + 2. * Vec3::Z),
118            Pickable::IGNORE,
119        ));
120        event.propagate(false);
121    }
122}
```

Hide additional examples

examples/3d/clearcoat.rs ([line 140](../../../src/clearcoat/clearcoat.rs.html#140))

```rust
127fn spawn_coated_glass_bubble_sphere(
128    commands: &mut Commands,
129    materials: &mut Assets<StandardMaterial>,
130    sphere: &Handle<Mesh>,
131) {
132    commands
133        .spawn((
134            Mesh3d(sphere.clone()),
135            MeshMaterial3d(materials.add(StandardMaterial {
136                clearcoat: 1.0,
137                clearcoat_perceptual_roughness: 0.1,
138                metallic: 0.5,
139                perceptual_roughness: 0.1,
140                base_color: Color::srgba(0.9, 0.9, 0.9, 0.3),
141                alpha_mode: AlphaMode::Blend,
142                ..default()
143            })),
144            Transform::from_xyz(-1.0, -1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
145        ))
146        .insert(ExampleSphere);
147}
```

examples/2d/transparency\_2d.rs ([line 26](../../../src/transparency_2d/transparency_2d.rs.html#26))

```rust
13fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
14    commands.spawn(Camera2d);
15
16    let sprite_handle = asset_server.load("branding/icon.png");
17
18    commands.spawn((
19        Sprite::from_image(sprite_handle.clone()),
20        Transform::from_xyz(-100.0, 0.0, 0.0),
21    ));
22    commands.spawn((
23        Sprite {
24            image: sprite_handle.clone(),
25            // Alpha channel of the color controls transparency.
26            color: Color::srgba(0.0, 0.0, 1.0, 0.7),
27            ..default()
28        },
29        Transform::from_xyz(0.0, 0.0, 0.1),
30    ));
31    commands.spawn((
32        Sprite {
33            image: sprite_handle,
34            color: Color::srgba(0.0, 1.0, 0.0, 0.3),
35            ..default()
36        },
37        Transform::from_xyz(100.0, 0.0, 0.2),
38    ));
39}
```

examples/3d/atmospheric\_fog.rs ([line 31](../../../src/atmospheric_fog/atmospheric_fog.rs.html#31))

```rust
26fn setup_camera_fog(mut commands: Commands) {
27    commands.spawn((
28        Camera3d::default(),
29        Transform::from_xyz(-1.0, 0.1, 1.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
30        DistanceFog {
31            color: Color::srgba(0.35, 0.48, 0.66, 1.0),
32            directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
33            directional_light_exponent: 30.0,
34            falloff: FogFalloff::from_visibility_colors(
35                15.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
36                Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
37                Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
38            ),
39        },
40    ));
41}
```

examples/dev\_tools/infinite\_grid.rs ([lines 52-54](../../../src/infinite_grid/infinite_grid.rs.html#52-54))

```rust
25fn setup_system(
26    mut commands: Commands,
27    mut meshes: ResMut<Assets<Mesh>>,
28    mut standard_materials: ResMut<Assets<StandardMaterial>>,
29) {
30    commands.spawn((
31        // You need to spawn an entity with this component
32        InfiniteGrid,
33        // Optional component you can use to configure the grid
34        InfiniteGridSettings::default(),
35    ));
36
37    commands.spawn((
38        Camera3d::default(),
39        Transform::from_xyz(-12.5, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
40        FreeCamera::default(),
41    ));
42
43    commands.spawn((
44        DirectionalLight { ..default() },
45        Transform::from_translation(Vec3::X * 15. + Vec3::Y * 20.).looking_at(Vec3::ZERO, Vec3::Y),
46    ));
47
48    // cube
49    commands.spawn((
50        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
51        MeshMaterial3d(
52            standard_materials.add(StandardMaterial::from_color(Color::srgba(
53                1.0, 1.0, 1.0, 0.5,
54            ))),
55        ),
56        Transform::from_xyz(0.0, 2.0, 0.0),
57    ));
58
59    commands.spawn((
60        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
61        MeshMaterial3d(
62            standard_materials.add(StandardMaterial::from_color(Color::srgba(
63                1.0, 1.0, 1.0, 0.5,
64            ))),
65        ),
66        Transform::from_xyz(0.0, -2.0, 0.0),
67    ));
68}
```

examples/3d/texture.rs ([line 39](../../../src/texture/texture.rs.html#39))

```rust
15fn setup(
16    mut commands: Commands,
17    asset_server: Res<AssetServer>,
18    mut meshes: ResMut<Assets<Mesh>>,
19    mut materials: ResMut<Assets<StandardMaterial>>,
20) {
21    // load a texture and retrieve its aspect ratio
22    let texture_handle = asset_server.load("branding/bevy_logo_dark_big.png");
23    let aspect = 0.25;
24
25    // create a new quad mesh. this is what we will apply the texture to
26    let quad_width = 8.0;
27    let quad_handle = meshes.add(Rectangle::new(quad_width, quad_width * aspect));
28
29    // this material renders the texture normally
30    let material_handle = materials.add(StandardMaterial {
31        base_color_texture: Some(texture_handle.clone()),
32        alpha_mode: AlphaMode::Blend,
33        unlit: true,
34        ..default()
35    });
36
37    // this material modulates the texture to make it red (and slightly transparent)
38    let red_material_handle = materials.add(StandardMaterial {
39        base_color: Color::srgba(1.0, 0.0, 0.0, 0.5),
40        base_color_texture: Some(texture_handle.clone()),
41        alpha_mode: AlphaMode::Blend,
42        unlit: true,
43        ..default()
44    });
45
46    // and lets make this one blue! (and also slightly transparent)
47    let blue_material_handle = materials.add(StandardMaterial {
48        base_color: Color::srgba(0.0, 0.0, 1.0, 0.5),
49        base_color_texture: Some(texture_handle),
50        alpha_mode: AlphaMode::Blend,
51        unlit: true,
52        ..default()
53    });
54
55    // textured quad - normal
56    commands.spawn((
57        Mesh3d(quad_handle.clone()),
58        MeshMaterial3d(material_handle),
59        Transform::from_xyz(0.0, 0.0, 1.5).with_rotation(Quat::from_rotation_x(-PI / 5.0)),
60    ));
61    // textured quad - modulated
62    commands.spawn((
63        Mesh3d(quad_handle.clone()),
64        MeshMaterial3d(red_material_handle),
65        Transform::from_rotation(Quat::from_rotation_x(-PI / 5.0)),
66    ));
67    // textured quad - modulated
68    commands.spawn((
69        Mesh3d(quad_handle),
70        MeshMaterial3d(blue_material_handle),
71        Transform::from_xyz(0.0, 0.0, -1.5).with_rotation(Quat::from_rotation_x(-PI / 5.0)),
72    ));
73    // camera
74    commands.spawn((
75        Camera3d::default(),
76        Transform::from_xyz(3.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
77    ));
78}
```

Additional examples can be found in:  

*   [examples/ui/styling/transparency\_ui.rs](../../../src/transparency_ui/transparency_ui.rs.html#49)
*   [examples/math/random\_sampling.rs](../../../src/random_sampling/random_sampling.rs.html#75)
*   [examples/showcase/stepping.rs](../../../src/breakout/stepping.rs.html#180)
*   [examples/3d/transparency\_3d.rs](../../../src/transparency_3d/transparency_3d.rs.html#34)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#188)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#128)
*   [examples/ui/navigation/directional\_navigation.rs](../../../src/directional_navigation/directional_navigation.rs.html#145)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#449)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#195)
*   [examples/testbed/full\_ui.rs](../../../src/testbed_full_ui/full_ui.rs.html#328)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#116)

#### pub const fn [srgb](#method.srgb)(red: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), green: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), blue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Srgba`](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") color with an alpha of 1.0.

##### Arguments

*   `red` - Red channel. \[0.0, 1.0\]
*   `green` - Green channel. \[0.0, 1.0\]
*   `blue` - Blue channel. \[0.0, 1.0\]

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/dev\_tools/fps\_overlay.rs ([line 12](../../../src/fps_overlay/fps_overlay.rs.html#12))

```rust
12    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
13    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
```

Hide additional examples

examples/showcase/game\_menu.rs ([line 7](../../../src/game_menu/game_menu.rs.html#7))

```rust
7const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
8
9// Enum that will be used as a global state for the game
10#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
11enum GameState {
12    #[default]
13    Splash,
14    Menu,
15    Game,
16}
17
18// One of the two settings that can be set through the menu. It will be a resource in the app
19#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
20enum DisplayQuality {
21    Low,
22    Medium,
23    High,
24}
25
26#[derive(Component)]
27struct Setting<T>(T);
28
29// One of the two settings that can be set through the menu. It will be a resource in the app
30#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
31struct Volume(u32);
32
33fn main() {
34    App::new()
35        .add_plugins(DefaultPlugins)
36        // Insert as resource the initial value for the settings resources
37        .insert_resource(DisplayQuality::Medium)
38        .insert_resource(Volume(7))
39        // Declare the game state, whose starting value is determined by the `Default` trait
40        .init_state::<GameState>()
41        .add_systems(Startup, setup)
42        // Adds the plugins for each state
43        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
44        .run();
45}
46
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
```

examples/showcase/stepping.rs ([line 88](../../../src/breakout/stepping.rs.html#88))

```rust
88const FONT_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
```

examples/ui/layout/display\_and\_visibility.rs ([line 10](../../../src/display_and_visibility/display_and_visibility.rs.html#10))

```rust
10const HIDDEN_COLOR: Color = Color::srgb(1.0, 0.7, 0.7);
11
12fn main() {
13    App::new()
14        .add_plugins(DefaultPlugins)
15        .add_systems(Startup, setup)
16        .add_systems(
17            Update,
18            (
19                buttons_handler::<Display>,
20                buttons_handler::<Visibility>,
21                text_hover,
22            ),
23        )
24        .run();
25}
26
27#[derive(Component)]
28struct Target<T> {
29    id: Entity,
30    phantom: std::marker::PhantomData<T>,
31}
32
33impl<T> Target<T> {
34    fn new(id: Entity) -> Self {
35        Self {
36            id,
37            phantom: std::marker::PhantomData,
38        }
39    }
40}
41
42trait TargetUpdate {
43    type TargetComponent: Component<Mutability = Mutable>;
44    const NAME: &'static str;
45    fn update_target(&self, target: &mut Self::TargetComponent) -> String;
46}
47
48impl TargetUpdate for Target<Display> {
49    type TargetComponent = Node;
50    const NAME: &'static str = "Display";
51    fn update_target(&self, node: &mut Self::TargetComponent) -> String {
52        node.display = match node.display {
53            Display::Flex => Display::None,
54            Display::None => Display::Flex,
55            Display::Block | Display::Grid => unreachable!(),
56        };
57        format!("{}::{:?} ", Self::NAME, node.display)
58    }
59}
60
61impl TargetUpdate for Target<Visibility> {
62    type TargetComponent = Visibility;
63    const NAME: &'static str = "Visibility";
64    fn update_target(&self, visibility: &mut Self::TargetComponent) -> String {
65        *visibility = match *visibility {
66            Visibility::Inherited => Visibility::Visible,
67            Visibility::Visible => Visibility::Hidden,
68            Visibility::Hidden => Visibility::Inherited,
69        };
70        format!("{}::{visibility:?}", Self::NAME)
71    }
72}
73
74fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
75    let palette: [Color; 4] = PALETTE.map(|hex| Srgba::hex(hex).unwrap().into());
76
77    let text_font = TextFont {
78        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
79        ..default()
80    };
81
82    commands.spawn(Camera2d);
83    commands
84        .spawn((
85            Node {
86                width: percent(100),
87                height: percent(100),
88                flex_direction: FlexDirection::Column,
89                align_items: AlignItems::Center,
90                justify_content: JustifyContent::SpaceEvenly,
91                ..Default::default()
92            },
93            BackgroundColor(Color::BLACK),
94        ))
95        .with_children(|parent| {
96            parent.spawn((
97                Text::new("Use the panel on the right to change the Display and Visibility properties for the respective nodes of the panel on the left"),
98                text_font.clone(),
99                TextLayout::justify(Justify::Center),
100                Node {
101                    margin: UiRect::bottom(px(10)),
102                    ..Default::default()
103                },
104            ));
105
106            parent
107                .spawn(Node {
108                    width: percent(100),
109                    ..default()
110                })
111                .with_children(|parent| {
112                    let mut target_ids = vec![];
113                    parent
114                        .spawn(Node {
115                            width: percent(50),
116                            height: px(520),
117                            justify_content: JustifyContent::Center,
118                            ..default()
119                        })
120                        .with_children(|parent| {
121                            target_ids = spawn_left_panel(parent, &palette);
122                        });
123
124                    parent
125                        .spawn(Node {
126                            width: percent(50),
127                            justify_content: JustifyContent::Center,
128                            ..default()
129                        })
130                        .with_children(|parent| {
131                            spawn_right_panel(parent, text_font, &palette, target_ids);
132                        });
133                });
134
135            parent
136                .spawn(Node {
137                    flex_direction: FlexDirection::Row,
138                    align_items: AlignItems::Start,
139                    justify_content: JustifyContent::Start,
140                    column_gap: px(10),
141                    ..default()
142                })
143                .with_children(|builder| {
144                    let text_font = TextFont {
145                        font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
146                        ..default()
147                    };
148
149                    builder.spawn((
150                        Text::new("Display::None\nVisibility::Hidden\nVisibility::Inherited"),
151                        text_font.clone(),
152                        TextColor(HIDDEN_COLOR),
153                        TextLayout::justify(Justify::Center),
154                    ));
155                    builder.spawn((
156                        Text::new("-\n-\n-"),
157                        text_font.clone(),
158                        TextColor(DARK_GRAY.into()),
159                        TextLayout::justify(Justify::Center),
160                    ));
161                    builder.spawn((Text::new("The UI Node and its descendants will not be visible and will not be allotted any space in the UI layout.\nThe UI Node will not be visible but will still occupy space in the UI layout.\nThe UI node will inherit the visibility property of its parent. If it has no parent it will be visible."), text_font));
162                });
163        });
164}
165
166fn spawn_left_panel(builder: &mut ChildSpawnerCommands, palette: &[Color; 4]) -> Vec<Entity> {
167    let mut target_ids = vec![];
168    builder
169        .spawn((
170            Node {
171                padding: UiRect::all(px(10)),
172                ..default()
173            },
174            BackgroundColor(Color::WHITE),
175        ))
176        .with_children(|parent| {
177            parent
178                .spawn((Node::default(), BackgroundColor(Color::BLACK)))
179                .with_children(|parent| {
180                    let id = parent
181                        .spawn((
182                            Node {
183                                align_items: AlignItems::FlexEnd,
184                                justify_content: JustifyContent::FlexEnd,
185                                ..default()
186                            },
187                            BackgroundColor(palette[0]),
188                            Outline {
189                                width: px(4),
190                                color: DARK_CYAN.into(),
191                                offset: px(10),
192                            },
193                        ))
194                        .with_children(|parent| {
195                            parent.spawn(Node {
196                                width: px(100),
197                                height: px(500),
198                                ..default()
199                            });
200
201                            let id = parent
202                                .spawn((
203                                    Node {
204                                        height: px(400),
205                                        align_items: AlignItems::FlexEnd,
206                                        justify_content: JustifyContent::FlexEnd,
207                                        ..default()
208                                    },
209                                    BackgroundColor(palette[1]),
210                                ))
211                                .with_children(|parent| {
212                                    parent.spawn(Node {
213                                        width: px(100),
214                                        height: px(400),
215                                        ..default()
216                                    });
217
218                                    let id = parent
219                                        .spawn((
220                                            Node {
221                                                height: px(300),
222                                                align_items: AlignItems::FlexEnd,
223                                                justify_content: JustifyContent::FlexEnd,
224                                                ..default()
225                                            },
226                                            BackgroundColor(palette[2]),
227                                        ))
228                                        .with_children(|parent| {
229                                            parent.spawn(Node {
230                                                width: px(100),
231                                                height: px(300),
232                                                ..default()
233                                            });
234
235                                            let id = parent
236                                                .spawn((
237                                                    Node {
238                                                        width: px(200),
239                                                        height: px(200),
240                                                        ..default()
241                                                    },
242                                                    BackgroundColor(palette[3]),
243                                                ))
244                                                .id();
245                                            target_ids.push(id);
246                                        })
247                                        .id();
248                                    target_ids.push(id);
249                                })
250                                .id();
251                            target_ids.push(id);
252                        })
253                        .id();
254                    target_ids.push(id);
255                });
256        });
257    target_ids
258}
259
260fn spawn_right_panel(
261    parent: &mut ChildSpawnerCommands,
262    text_font: TextFont,
263    palette: &[Color; 4],
264    mut target_ids: Vec<Entity>,
265) {
266    let spawn_buttons = |parent: &mut ChildSpawnerCommands, target_id| {
267        spawn_button::<Display>(parent, text_font.clone(), target_id);
268        spawn_button::<Visibility>(parent, text_font.clone(), target_id);
269    };
270    parent
271        .spawn((
272            Node {
273                padding: UiRect::all(px(10)),
274                ..default()
275            },
276            BackgroundColor(Color::WHITE),
277        ))
278        .with_children(|parent| {
279            parent
280                .spawn((
281                    Node {
282                        width: px(500),
283                        height: px(500),
284                        flex_direction: FlexDirection::Column,
285                        align_items: AlignItems::FlexEnd,
286                        justify_content: JustifyContent::SpaceBetween,
287                        padding: UiRect {
288                            left: px(5),
289                            top: px(5),
290                            ..default()
291                        },
292                        ..default()
293                    },
294                    BackgroundColor(palette[0]),
295                    Outline {
296                        width: px(4),
297                        color: DARK_CYAN.into(),
298                        offset: px(10),
299                    },
300                ))
301                .with_children(|parent| {
302                    spawn_buttons(parent, target_ids.pop().unwrap());
303
304                    parent
305                        .spawn((
306                            Node {
307                                width: px(400),
308                                height: px(400),
309                                flex_direction: FlexDirection::Column,
310                                align_items: AlignItems::FlexEnd,
311                                justify_content: JustifyContent::SpaceBetween,
312                                padding: UiRect {
313                                    left: px(5),
314                                    top: px(5),
315                                    ..default()
316                                },
317                                ..default()
318                            },
319                            BackgroundColor(palette[1]),
320                        ))
321                        .with_children(|parent| {
322                            spawn_buttons(parent, target_ids.pop().unwrap());
323
324                            parent
325                                .spawn((
326                                    Node {
327                                        width: px(300),
328                                        height: px(300),
329                                        flex_direction: FlexDirection::Column,
330                                        align_items: AlignItems::FlexEnd,
331                                        justify_content: JustifyContent::SpaceBetween,
332                                        padding: UiRect {
333                                            left: px(5),
334                                            top: px(5),
335                                            ..default()
336                                        },
337                                        ..default()
338                                    },
339                                    BackgroundColor(palette[2]),
340                                ))
341                                .with_children(|parent| {
342                                    spawn_buttons(parent, target_ids.pop().unwrap());
343
344                                    parent
345                                        .spawn((
346                                            Node {
347                                                width: px(200),
348                                                height: px(200),
349                                                align_items: AlignItems::FlexStart,
350                                                justify_content: JustifyContent::SpaceBetween,
351                                                flex_direction: FlexDirection::Column,
352                                                padding: UiRect {
353                                                    left: px(5),
354                                                    top: px(5),
355                                                    ..default()
356                                                },
357                                                ..default()
358                                            },
359                                            BackgroundColor(palette[3]),
360                                        ))
361                                        .with_children(|parent| {
362                                            spawn_buttons(parent, target_ids.pop().unwrap());
363
364                                            parent.spawn(Node {
365                                                width: px(100),
366                                                height: px(100),
367                                                ..default()
368                                            });
369                                        });
370                                });
371                        });
372                });
373        });
374}
375
376fn spawn_button<T>(parent: &mut ChildSpawnerCommands, text_font: TextFont, target: Entity)
377where
378    T: Default + std::fmt::Debug + Send + Sync + 'static,
379    Target<T>: TargetUpdate,
380{
381    parent
382        .spawn((
383            Button,
384            Node {
385                align_self: AlignSelf::FlexStart,
386                padding: UiRect::axes(px(5), px(1)),
387                ..default()
388            },
389            BackgroundColor(Color::BLACK.with_alpha(0.5)),
390            Target::<T>::new(target),
391        ))
392        .with_children(|builder| {
393            builder.spawn((
394                Text(format!("{}::{:?}", Target::<T>::NAME, T::default())),
395                text_font,
396                TextLayout::justify(Justify::Center),
397            ));
398        });
399}
400
401fn buttons_handler<T>(
402    mut left_panel_query: Query<&mut <Target<T> as TargetUpdate>::TargetComponent>,
403    mut visibility_button_query: Query<(&Target<T>, &Interaction, &Children), Changed<Interaction>>,
404    mut text_query: Query<(&mut Text, &mut TextColor)>,
405) where
406    T: Send + Sync,
407    Target<T>: TargetUpdate + Component,
408{
409    for (target, interaction, children) in visibility_button_query.iter_mut() {
410        if matches!(interaction, Interaction::Pressed) {
411            let mut target_value = left_panel_query.get_mut(target.id).unwrap();
412            for &child in children {
413                if let Ok((mut text, mut text_color)) = text_query.get_mut(child) {
414                    **text = target.update_target(target_value.as_mut());
415                    text_color.0 = if text.contains("None") || text.contains("Hidden") {
416                        Color::srgb(1.0, 0.7, 0.7)
417                    } else {
418                        Color::WHITE
419                    };
420                }
421            }
422        }
423    }
424}
```

examples/ui/widgets/vertical\_slider.rs ([line 13](../../../src/vertical_slider/vertical_slider.rs.html#13))

```rust
13const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
14const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);
15
16fn main() {
17    App::new()
18        .add_plugins((DefaultPlugins, TabNavigationPlugin))
19        .add_systems(Startup, setup)
20        .add_systems(Update, (update_slider_visuals, update_value_labels))
21        .run();
22}
23
24#[derive(Component)]
25struct ValueLabel(Entity);
26
27#[derive(Component)]
28struct DemoSlider;
29
30#[derive(Component)]
31struct DemoSliderThumb;
32
33#[derive(Component)]
34struct VerticalSlider;
35
36fn setup(mut commands: Commands, assets: Res<AssetServer>) {
37    commands.spawn(Camera2d);
38
39    commands
40        .spawn((
41            Node {
42                width: percent(100),
43                height: percent(100),
44                align_items: AlignItems::Center,
45                justify_content: JustifyContent::Center,
46                display: Display::Flex,
47                flex_direction: FlexDirection::Row,
48                column_gap: px(50),
49                ..default()
50            },
51            TabGroup::default(),
52        ))
53        .with_children(|parent| {
54            // Vertical slider
55            parent
56                .spawn(Node {
57                    display: Display::Flex,
58                    flex_direction: FlexDirection::Column,
59                    align_items: AlignItems::Center,
60                    row_gap: px(10),
61                    ..default()
62                })
63                .with_children(|parent| {
64                    parent.spawn((
65                        Text::new("Vertical"),
66                        TextFont {
67                            font: assets.load("fonts/FiraSans-Bold.ttf").into(),
68                            font_size: FontSize::Px(20.0),
69                            ..default()
70                        },
71                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
72                    ));
73
74                    let label_id = parent
75                        .spawn((
76                            Text::new("50"),
77                            TextFont {
78                                font: assets.load("fonts/FiraSans-Bold.ttf").into(),
79                                font_size: FontSize::Px(24.0),
80                                ..default()
81                            },
82                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
83                        ))
84                        .id();
85
86                    parent.spawn((
87                        vertical_slider(),
88                        ValueLabel(label_id),
89                        observe(slider_self_update),
90                    ));
91                });
92
93            // Horizontal slider
94            parent
95                .spawn(Node {
96                    display: Display::Flex,
97                    flex_direction: FlexDirection::Column,
98                    align_items: AlignItems::Center,
99                    row_gap: px(10),
100                    ..default()
101                })
102                .with_children(|parent| {
103                    parent.spawn((
104                        Text::new("Horizontal"),
105                        TextFont {
106                            font: assets.load("fonts/FiraSans-Bold.ttf").into(),
107                            font_size: FontSize::Px(20.0),
108                            ..default()
109                        },
110                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
111                    ));
112
113                    let label_id = parent
114                        .spawn((
115                            Text::new("50"),
116                            TextFont {
117                                font: assets.load("fonts/FiraSans-Bold.ttf").into(),
118                                font_size: FontSize::Px(24.0),
119                                ..default()
120                            },
121                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
122                        ))
123                        .id();
124
125                    parent.spawn((
126                        horizontal_slider(),
127                        ValueLabel(label_id),
128                        observe(slider_self_update),
129                    ));
130                });
131        });
132}
```

examples/showcase/breakout.rs ([line 45](../../../src/breakout/breakout.rs.html#45))

```rust
45const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
46const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
47const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
48const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
49const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
50const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
51const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
```

Additional examples can be found in:  

*   [examples/state/custom\_transitions.rs](../../../src/custom_transitions/custom_transitions.rs.html#240)
*   [examples/state/states.rs](../../../src/states/states.rs.html#49)
*   [examples/ui/styling/box\_shadow.rs](../../../src/box_shadow/box_shadow.rs.html#5)
*   [examples/ui/widgets/button.rs](../../../src/button/button.rs.html#20)
*   [examples/ui/widgets/standard\_widgets.rs](../../../src/standard_widgets/standard_widgets.rs.html#53)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../../src/standard_widgets_observers/standard_widgets_observers.rs.html#44)
*   [examples/ui/widgets/tab\_navigation.rs](../../../src/tab_navigation/tab_navigation.rs.html#20)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#96)
*   [examples/ui/layout/flex\_layout.rs](../../../src/flex_layout/flex_layout.rs.html#4)
*   [examples/ui/layout/size\_constraints.rs](../../../src/size_constraints/size_constraints.rs.html#22)
*   [examples/state/computed\_states.rs](../../../src/computed_states/computed_states.rs.html#324)
*   [examples/state/sub\_states.rs](../../../src/sub_states/sub_states.rs.html#149)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#12)
*   [examples/window/clear\_color.rs](../../../src/clear_color/clear_color.rs.html#9)
*   [examples/ecs/removal\_detection.rs](../../../src/removal_detection/removal_detection.rs.html#54)
*   [examples/3d/fog.rs](../../../src/fog/fog.rs.html#46)
*   [examples/math/cubic\_splines.rs](../../../src/cubic_splines/cubic_splines.rs.html#177)
*   [examples/async\_tasks/async\_compute.rs](../../../src/async_compute/async_compute.rs.html#52)
*   [examples/ui/text/text.rs](../../../src/text/text.rs.html#181-185)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../../src/async_channel_pattern/async_channel_pattern.rs.html#126)
*   [examples/ecs/state\_scoped.rs](../../../src/state_scoped/state_scoped.rs.html#49)
*   [examples/ui/layout/ghost\_nodes.rs](../../../src/ghost_nodes/ghost_nodes.rs.html#87)
*   [examples/camera/2d\_top\_down\_camera.rs](../../../src/2d_top_down_camera/2d_top_down_camera.rs.html#39)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../../src/fullscreen_material/fullscreen_material.rs.html#39)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#50)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../../src/scrollbars/scrollbars.rs.html#36)
*   [examples/asset/multi\_asset\_sync.rs](../../../src/multi_asset_sync/multi_asset_sync.rs.html#211)
*   [examples/ui/layout/anchor\_layout.rs](../../../src/anchor_layout/anchor_layout.rs.html#127)
*   [tests/window/minimizing.rs](../../../src/minimizing/minimizing.rs.html#38)
*   [tests/window/resizing.rs](../../../src/resizing/resizing.rs.html#114)
*   [examples/3d/atmospheric\_fog.rs](../../../src/atmospheric_fog/atmospheric_fog.rs.html#36)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#173)
*   [examples/animation/animated\_mesh.rs](../../../src/animated_mesh/animated_mesh.rs.html#115)
*   [examples/window/scale\_factor\_override.rs](../../../src/scale_factor_override/scale_factor_override.rs.html#44)
*   [examples/3d/parenting.rs](../../../src/parenting/parenting.rs.html#33)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#252)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../../src/overflow_debug/overflow_debug.rs.html#187)
*   [examples/app/settings.rs](../../../src/settings/settings.rs.html#81)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#30)
*   [examples/3d/mesh\_ray\_cast.rs](../../../src/mesh_ray_cast/mesh_ray_cast.rs.html#47)
*   [examples/camera/2d\_screen\_shake.rs](../../../src/2d_screen_shake/2d_screen_shake.rs.html#187)
*   [examples/camera/camera\_orbit.rs](../../../src/camera_orbit/camera_orbit.rs.html#61)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#152)
*   [examples/window/screenshot.rs](../../../src/screenshot/screenshot.rs.html#58)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../../src/custom_post_processing/custom_post_processing.rs.html#266)
*   [tests/window/desktop\_request\_redraw.rs](../../../src/desktop_request_redraw/desktop_request_redraw.rs.html#25)
*   [examples/3d/two\_passes.rs](../../../src/two_passes/two_passes.rs.html#21)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#103)
*   [examples/window/low\_power.rs](../../../src/low_power/low_power.rs.html#175)
*   [examples/movement/smooth\_follow.rs](../../../src/smooth_follow/smooth_follow.rs.html#50)
*   [examples/ui/relative\_cursor\_position.rs](../../../src/relative_cursor_position/relative_cursor_position.rs.html#45)
*   [examples/3d/vertex\_colors.rs](../../../src/vertex_colors/vertex_colors.rs.html#21)
*   [examples/animation/animated\_mesh\_control.rs](../../../src/animated_mesh_control/animated_mesh_control.rs.html#60)
*   [examples/3d/orthographic.rs](../../../src/orthographic/orthographic.rs.html#34)
*   [examples/remote/app\_under\_test.rs](../../../src/app_under_test/app_under_test.rs.html#108)
*   [examples/stress\_tests/many\_buttons.rs](../../../src/many_buttons/many_buttons.rs.html#307)
*   [examples/3d/spherical\_area\_lights.rs](../../../src/spherical_area_lights/spherical_area_lights.rs.html#31)
*   [examples/2d/bloom\_2d.rs](../../../src/bloom_2d/bloom_2d.rs.html#37)
*   [examples/gizmos/axes.rs](../../../src/axes/axes.rs.html#67)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#236)
*   [examples/3d/ssao.rs](../../../src/ssao/ssao.rs.html#40)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#109)
*   [examples/ecs/error\_handling.rs](../../../src/error_handling/error_handling.rs.html#70)
*   [examples/camera/projection\_zoom.rs](../../../src/projection_zoom/projection_zoom.rs.html#71)
*   [examples/3d/order\_independent\_transparency.rs](../../../src/order_independent_transparency/order_independent_transparency.rs.html#256)
*   [examples/3d/rect\_light.rs](../../../src/rect_light/rect_light.rs.html#56)
*   [examples/ui/images/ui\_texture\_slice.rs](../../../src/ui_texture_slice/ui_texture_slice.rs.html#91)
*   [examples/3d/visibility\_range.rs](../../../src/visibility_range/visibility_range.rs.html#122)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#85)
*   [examples/time/virtual\_time.rs](../../../src/virtual_time/virtual_time.rs.html#104)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#725)
*   [examples/3d/anti\_aliasing.rs](../../../src/anti_aliasing/anti_aliasing.rs.html#422)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#60)
*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#59)
*   [examples/transforms/align.rs](../../../src/align/align.rs.html#64)
*   [examples/ui/styling/transparency\_ui.rs](../../../src/transparency_ui/transparency_ui.rs.html#38)
*   [examples/math/random\_sampling.rs](../../../src/random_sampling/random_sampling.rs.html#63)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#194)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#109)
*   [examples/picking/sprite\_picking.rs](../../../src/sprite_picking/sprite_picking.rs.html#64)
*   [examples/ecs/iter\_combinations.rs](../../../src/iter_combinations/iter_combinations.rs.html#68-72)
*   [examples/ui/text/text\_wrap\_debug.rs](../../../src/text_wrap_debug/text_wrap_debug.rs.html#103)
*   [examples/3d/render\_to\_texture.rs](../../../src/render_to_texture/render_to_texture.rs.html#42)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#187)
*   [examples/gizmos/transform\_gizmo.rs](../../../src/transform_gizmo/transform_gizmo.rs.html#53)
*   [examples/3d/transparency\_3d.rs](../../../src/transparency_3d/transparency_3d.rs.html#23)
*   [examples/asset/alter\_mesh.rs](../../../src/alter_mesh/alter_mesh.rs.html#102)
*   [examples/ui/text/system\_fonts.rs](../../../src/system_fonts/system_fonts.rs.html#39)
*   [examples/3d/auto\_exposure.rs](../../../src/auto_exposure/auto_exposure.rs.html#88-92)
*   [examples/asset/asset\_loading.rs](../../../src/asset_loading/asset_loading.rs.html#71)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../../src/overflow/overflow.rs.html#54)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../../src/overflow_clip_margin/overflow_clip_margin.rs.html#51)
*   [examples/shader/shader\_prepass.rs](../../../src/shader_prepass/shader_prepass.rs.html#64)
*   [examples/3d/split\_screen.rs](../../../src/split_screen/split_screen.rs.html#27)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../../src/scroll/scroll.rs.html#156)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#225)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#24)
*   [examples/3d/deferred\_rendering.rs](../../../src/deferred_rendering/deferred_rendering.rs.html#85)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#234)
*   [examples/2d/text2d.rs](../../../src/text2d/text2d.rs.html#81)
*   [examples/animation/custom\_skinned\_mesh.rs](../../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#174-178)
*   [examples/usage/debug\_frustum\_culling.rs](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#191)
*   [examples/3d/blend\_modes.rs](../../../src/blend_modes/blend_modes.rs.html#33)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#268)
*   [examples/animation/animated\_transform.rs](../../../src/animated_transform/animated_transform.rs.html#147)
*   [examples/ui/styling/borders.rs](../../../src/borders/borders.rs.html#222)
*   [examples/3d/camera\_sub\_view.rs](../../../src/camera_sub_view/camera_sub_view.rs.html#36)
*   [examples/ui/text/text\_debug.rs](../../../src/text_debug/text_debug.rs.html#116)
*   [examples/ui/layout/grid.rs](../../../src/grid/grid.rs.html#88)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#110)
*   [examples/ui/styling/gradients.rs](../../../src/gradients/gradients.rs.html#201)
*   [examples/testbed/full\_ui.rs](../../../src/testbed_full_ui/full_ui.rs.html#53)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#129)

#### pub const fn [srgb\_from\_array](#method.srgb_from_array)(array: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Reads an array of floats to creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Srgba`](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") color with an alpha of 1.0.

##### Arguments

*   `array` - Red, Green and Blue channels. Each channel is in the range \[0.0, 1.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#146)

#### pub const fn [srgba\_u8](#method.srgba_u8)(red: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), green: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), blue: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), alpha: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Srgba`](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") color from [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8") values.

##### Arguments

*   `red` - Red channel. \[0, 255\]
*   `green` - Green channel. \[0, 255\]
*   `blue` - Blue channel. \[0, 255\]
*   `alpha` - Alpha channel. \[0, 255\]

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/3d/anti\_aliasing.rs ([line 476](../../../src/anti_aliasing/anti_aliasing.rs.html#476))

```rust
412fn setup(
413    mut commands: Commands,
414    mut meshes: ResMut<Assets<Mesh>>,
415    mut materials: ResMut<Assets<StandardMaterial>>,
416    mut images: ResMut<Assets<Image>>,
417    asset_server: Res<AssetServer>,
418) {
419    // Plane
420    commands.spawn((
421        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
422        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.2, 0.1))),
423    ));
424
425    let cube_material = materials.add(StandardMaterial {
426        base_color_texture: Some(images.add(uv_debug_texture())),
427        ..default()
428    });
429
430    // Cubes
431    for i in 0..5 {
432        commands.spawn((
433            Mesh3d(meshes.add(Cuboid::new(0.25, 0.25, 0.25))),
434            MeshMaterial3d(cube_material.clone()),
435            Transform::from_xyz(i as f32 * 0.25 - 1.0, 0.125, -i as f32 * 0.5),
436        ));
437    }
438
439    // Flight Helmet
440    commands.spawn(WorldAssetRoot(asset_server.load(
441        GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"),
442    )));
443
444    // Light
445    commands.spawn((
446        DirectionalLight {
447            illuminance: light_consts::lux::FULL_DAYLIGHT,
448            shadow_maps_enabled: true,
449            ..default()
450        },
451        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
452        CascadeShadowConfigBuilder {
453            maximum_distance: 3.0,
454            first_cascade_far_bound: 0.9,
455            ..default()
456        }
457        .build(),
458    ));
459
460    // Camera
461    commands.spawn((
462        Camera3d::default(),
463        Hdr,
464        Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
465        ContrastAdaptiveSharpening {
466            enabled: false,
467            ..default()
468        },
469        EnvironmentMapLight {
470            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
471            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
472            intensity: 150.0,
473            ..default()
474        },
475        DistanceFog {
476            color: Color::srgba_u8(43, 44, 47, 255),
477            falloff: FogFalloff::Linear {
478                start: 1.0,
479                end: 4.0,
480            },
481            ..default()
482        },
483    ));
484
485    // example instructions
486    commands.spawn((
487        Text::default(),
488        Node {
489            position_type: PositionType::Absolute,
490            top: px(12),
491            left: px(12),
492            ..default()
493        },
494    ));
495}
```

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#162)

#### pub const fn [srgb\_u8](#method.srgb_u8)(red: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), green: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), blue: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Srgba`](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") color from [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8") values with an alpha of 1.0.

##### Arguments

*   `red` - Red channel. \[0, 255\]
*   `green` - Green channel. \[0, 255\]
*   `blue` - Blue channel. \[0, 255\]

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/picking/debug\_picking.rs ([line 96](../../../src/debug_picking/debug_picking.rs.html#96))

```rust
86fn on_click_spawn_cube(
87    _click: On<Pointer<Click>>,
88    mut commands: Commands,
89    mut meshes: ResMut<Assets<Mesh>>,
90    mut materials: ResMut<Assets<StandardMaterial>>,
91    mut num: Local<usize>,
92) {
93    commands
94        .spawn((
95            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
96            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
97            Transform::from_xyz(0.0, 0.25 + 0.55 * *num as f32, 0.0),
98        ))
99        // With the MeshPickingPlugin added, you can add pointer event observers to meshes:
100        .observe(on_drag_rotate);
101    *num += 1;
102}
```

Hide additional examples

examples/picking/simple\_picking.rs ([line 72](../../../src/simple_picking/simple_picking.rs.html#72))

```rust
62fn on_click_spawn_cube(
63    _click: On<Pointer<Click>>,
64    mut commands: Commands,
65    mut meshes: ResMut<Assets<Mesh>>,
66    mut materials: ResMut<Assets<StandardMaterial>>,
67    mut num: Local<usize>,
68) {
69    commands
70        .spawn((
71            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
72            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
73            Transform::from_xyz(0.0, 0.25 + 0.55 * *num as f32, 0.0),
74        ))
75        // With the MeshPickingPlugin added, you can add pointer event observers to meshes:
76        .observe(on_drag_rotate);
77    *num += 1;
78}
```

examples/app/externally\_driven\_headless\_renderer.rs ([line 142](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#142))

```rust
130fn spawn_test_scene(
131    mut commands: Commands,
132    mut meshes: ResMut<Assets<Mesh>>,
133    mut materials: ResMut<Assets<StandardMaterial>>,
134) {
135    commands.spawn((
136        Mesh3d(meshes.add(Circle::new(4.0))),
137        MeshMaterial3d(materials.add(Color::WHITE)),
138        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
139    ));
140    commands.spawn((
141        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
142        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
143        Transform::from_xyz(0.0, 1.0, 0.0),
144    ));
145    commands.spawn((
146        PointLight {
147            shadow_maps_enabled: true,
148            ..default()
149        },
150        Transform::from_xyz(4.0, 8.0, 4.0),
151    ));
152}
```

examples/3d/color\_grading.rs ([line 329](../../../src/color_grading/color_grading.rs.html#329))

```rust
322fn add_camera(commands: &mut Commands, asset_server: &AssetServer, color_grading: ColorGrading) {
323    commands.spawn((
324        Camera3d::default(),
325        Hdr,
326        Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
327        color_grading,
328        DistanceFog {
329            color: Color::srgb_u8(43, 44, 47),
330            falloff: FogFalloff::Linear {
331                start: 1.0,
332                end: 8.0,
333            },
334            ..default()
335        },
336        EnvironmentMapLight {
337            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
338            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
339            intensity: 2000.0,
340            ..default()
341        },
342    ));
343}
```

examples/3d/3d\_scene.rs ([line 24](../../../src/3d_scene/3d_scene.rs.html#24))

```rust
13fn scene() -> impl SceneList {
14    bsn_list! [
15        (
16            #CircularBase
17            Mesh3d(asset_value(Circle::new(4.0)))
18            MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
19            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
20        ),
21        (
22            #Cube
23            Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
24            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
25            Transform::from_xyz(0.0, 0.5, 0.0)
26        ),
27        (
28            PointLight {
29                shadow_maps_enabled: true,
30            }
31            Transform::from_xyz(4.0, 8.0, 4.0)
32        ),
33        (
34            Camera3d
35            template_value(Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
36        )
37    ]
38}
```

examples/3d/motion\_blur.rs ([line 182](../../../src/motion_blur/motion_blur.rs.html#182))

```rust
174fn spawn_barriers(
175    meshes: &mut Assets<Mesh>,
176    materials: &mut Assets<StandardMaterial>,
177    commands: &mut Commands,
178) {
179    const N_CONES: usize = 100;
180    let capsule = meshes.add(Capsule3d::default());
181    let matl = materials.add(StandardMaterial {
182        base_color: Color::srgb_u8(255, 87, 51),
183        reflectance: 1.0,
184        ..default()
185    });
186    let mut spawn_with_offset = |offset: f32| {
187        for i in 0..N_CONES {
188            let pos = race_track_pos(
189                offset,
190                (i as f32) / (N_CONES as f32) * std::f32::consts::PI * 2.0,
191            );
192            commands.spawn((
193                Mesh3d(capsule.clone()),
194                MeshMaterial3d(matl.clone()),
195                Transform::from_xyz(pos.x, -0.65, pos.y).with_scale(Vec3::splat(0.07)),
196            ));
197        }
198    };
199    spawn_with_offset(0.04);
200    spawn_with_offset(-0.04);
201}
```

Additional examples can be found in:  

*   [examples/3d/tonemapping.rs](../../../src/tonemapping/tonemapping.rs.html#73)
*   [examples/3d/post\_processing.rs](../../../src/post_processing/post_processing.rs.html#85)
*   [examples/remote/server.rs](../../../src/server/server.rs.html#41)
*   [examples/camera/custom\_projection.rs](../../../src/custom_projection/custom_projection.rs.html#74)
*   [examples/diagnostics/log\_diagnostics.rs](../../../src/log_diagnostics/log_diagnostics.rs.html#72)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../../src/custom_render_phase/custom_render_phase.rs.html#91)
*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#412-416)
*   [examples/3d/skybox.rs](../../../src/skybox/skybox.rs.html#92)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#679)
*   [examples/app/headless\_renderer.rs](../../../src/headless_renderer/headless_renderer.rs.html#84)
*   [examples/app/render\_recovery.rs](../../../src/render_recovery/render_recovery.rs.html#48)
*   [examples/3d/spotlight.rs](../../../src/spotlight/spotlight.rs.html#56)
*   [examples/gizmos/light\_gizmos.rs](../../../src/light_gizmos/light_gizmos.rs.html#55)
*   [examples/3d/parallax\_mapping.rs](../../../src/parallax_mapping/parallax_mapping.rs.html#246)
*   [examples/3d/deferred\_rendering.rs](../../../src/deferred_rendering/deferred_rendering.rs.html#41)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#182)

#### pub fn [srgb\_u32](#method.srgb_u32)(color: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Srgba`](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") color from a [`u32`](https://doc.rust-lang.org/nightly/std/primitive.u32.html "primitive u32") value with an alpha of 1.0.

For example, a value of `0x000000` results in black, and a value of `0xff0000` results in red.

##### Examples

```rust
let black = Color::srgb_u32(0x000000);
let red = Color::srgb_u32(0xff0000);
```

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#201)

#### pub fn [srgba\_u32](#method.srgba_u32)(color: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Srgba`](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba") color from a [`u32`](https://doc.rust-lang.org/nightly/std/primitive.u32.html "primitive u32") value with the alpha value extracted from the input.

For example, a value of `0x000000ff` results in black with full opacity, and a value of `0xff000080` results in red with half opacity.

##### Examples

```rust
let black = Color::srgba_u32(0x000000ff);
let semi_transparent_red = Color::srgba_u32(0xff000080);
```

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#218)

#### pub const fn [linear\_rgba](#method.linear_rgba)(red: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), green: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), blue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`LinearRgba`](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba") color.

##### Arguments

*   `red` - Red channel. \[0.0, 1.0\]
*   `green` - Green channel. \[0.0, 1.0\]
*   `blue` - Blue channel. \[0.0, 1.0\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/2d/tilemap\_chunk\_orientation.rs ([line 39](../../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#39))

```rust
17fn setup(mut commands: Commands, assets: Res<AssetServer>) {
18    let chunk_size = UVec2::splat(8);
19    let tile_display_size = UVec2::splat(64);
20
21    // We'll use each possible orientation, one per column
22    let orientation = [
23        TileOrientation::Default,
24        TileOrientation::Rotate90,
25        TileOrientation::Rotate180,
26        TileOrientation::Rotate270,
27        TileOrientation::MirrorH,
28        TileOrientation::MirrorHRotate90,
29        TileOrientation::MirrorHRotate180,
30        TileOrientation::MirrorHRotate270,
31    ];
32
33    // Show different color/alpha on each row
34    let colors = [
35        Color::WHITE,
36        Color::linear_rgb(1.0, 0.0, 0.0),
37        Color::linear_rgb(0.0, 1.0, 0.0),
38        Color::linear_rgb(0.0, 0.0, 1.0),
39        Color::linear_rgba(1.0, 0.0, 0.0, 0.25),
40        Color::linear_rgba(0.0, 1.0, 0.0, 0.25),
41        Color::linear_rgba(0.0, 0.0, 1.0, 0.25),
42        Color::linear_rgba(1.0, 1.0, 1.0, 0.5),
43    ];
44
45    let tile_data = (0..chunk_size.element_product())
46        .map(|i| {
47            let row = i / 8;
48            let col = i % 8;
49            Some(TileData {
50                // Alternate tiles per row
51                tileset_index: (row % 2) as u16,
52                color: colors[row as usize],
53                // Last (top) row is invisible
54                visible: row != 7,
55                orientation: orientation[col as usize],
56            })
57        })
58        .collect();
59
60    commands.spawn((
61        TilemapChunk {
62            chunk_size,
63            tile_display_size,
64            tileset: assets
65                .load_builder()
66                .with_settings(|settings: &mut ImageLoaderSettings| {
67                    // The tileset texture is expected to be an array of tile textures, so we tell the
68                    // `ImageLoader` that our texture is composed of 2 stacked tile images.
69                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 2 });
70                })
71                .load("textures/arrow.png"),
72            alpha_mode: AlphaMode2d::Blend,
73        },
74        TilemapChunkTileData(tile_data),
75    ));
76
77    commands.spawn(Camera2d);
78}
```

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#234)

#### pub const fn [linear\_rgb](#method.linear_rgb)(red: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), green: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), blue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`LinearRgba`](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba") color with an alpha of 1.0.

##### Arguments

*   `red` - Red channel. \[0.0, 1.0\]
*   `green` - Green channel. \[0.0, 1.0\]
*   `blue` - Blue channel. \[0.0, 1.0\]

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_cubes.rs ([line 594](../../../src/many_cubes/many_cubes.rs.html#594))

```rust
588fn update_materials(mut materials: ResMut<Assets<StandardMaterial>>, time: Res<Time>) {
589    let elapsed = time.elapsed_secs();
590    for (i, (_, material)) in materials.iter_mut().enumerate() {
591        let hue = (elapsed + i as f32 * 0.005).rem_euclid(1.0);
592        // This is much faster than using base_color.set_hue(hue), and in a tight loop it shows.
593        let color = fast_hue_to_rgb(hue);
594        material.base_color = Color::linear_rgb(color.x, color.y, color.z);
595    }
596}
```

Hide additional examples

examples/stress\_tests/bevymark\_3d.rs ([line 313](../../../src/bevymark_3d/bevymark_3d.rs.html#313))

```rust
297fn mouse_handler(
298    mut commands: Commands,
299    args: Res<Args>,
300    time: Res<Time>,
301    mouse_button_input: Res<ButtonInput<MouseButton>>,
302    cube_resources: ResMut<CubeResources>,
303    mut counter: ResMut<BevyCounter>,
304    mut rng: Local<Option<ChaCha8Rng>>,
305    mut wave: Local<usize>,
306) {
307    if rng.is_none() {
308        *rng = Some(ChaCha8Rng::seed_from_u64(42));
309    }
310    let rng = rng.as_mut().unwrap();
311
312    if mouse_button_input.just_released(MouseButton::Left) {
313        counter.color = Color::linear_rgb(rng.random(), rng.random(), rng.random());
314    }
315
316    if mouse_button_input.pressed(MouseButton::Left) {
317        let spawn_count = (CUBES_PER_SECOND as f64 * time.delta_secs_f64()) as usize;
318        spawn_cubes(
319            &mut commands,
320            args.into_inner(),
321            &mut counter,
322            spawn_count,
323            cube_resources.into_inner(),
324            None,
325            *wave,
326        );
327        *wave += 1;
328    }
329}
330
331fn cube_velocity_transform(
332    mut translation: Vec3,
333    velocity_rng: &mut ChaCha8Rng,
334    waves: Option<usize>,
335    dt: f32,
336) -> (Transform, Vec3) {
337    let mut velocity = Vec3::new(0., 0., MAX_VELOCITY * velocity_rng.random::<f32>());
338
339    if let Some(waves) = waves {
340        for _ in 0..(waves * (FIXED_TIMESTEP / dt).round() as usize) {
341            step_movement(&mut translation, &mut velocity, dt);
342            handle_collision(&translation, &mut velocity);
343        }
344    }
345    (Transform::from_translation(translation), velocity)
346}
347
348const FIXED_DELTA_TIME: f32 = 1.0 / 60.0;
349
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
406
407fn step_movement(translation: &mut Vec3, velocity: &mut Vec3, dt: f32) {
408    translation.x += velocity.x * dt;
409    translation.y += velocity.y * dt;
410    translation.z += velocity.z * dt;
411    velocity.y += GRAVITY * dt;
412}
413
414fn movement_system(
415    args: Res<Args>,
416    time: Res<Time>,
417    mut cube_query: Query<(&mut Cube, &mut Transform)>,
418) {
419    let dt = if args.benchmark {
420        FIXED_DELTA_TIME
421    } else {
422        time.delta_secs()
423    };
424    for (mut cube, mut transform) in &mut cube_query {
425        step_movement(&mut transform.translation, &mut cube.velocity, dt);
426    }
427}
428
429fn handle_collision(translation: &Vec3, velocity: &mut Vec3) {
430    if (velocity.x > 0. && translation.x + HALF_CUBE_SIZE > VOLUME_SIZE.x / 2.0)
431        || (velocity.x <= 0. && translation.x - HALF_CUBE_SIZE < -VOLUME_SIZE.x / 2.0)
432    {
433        velocity.x = -velocity.x;
434    }
435    if (velocity.z > 0. && translation.z + HALF_CUBE_SIZE > VOLUME_SIZE.z / 2.0)
436        || (velocity.z <= 0. && translation.z - HALF_CUBE_SIZE < -VOLUME_SIZE.z / 2.0)
437    {
438        velocity.z = -velocity.z;
439    }
440
441    let velocity_y = velocity.y;
442    if velocity_y < 0. && translation.y - HALF_CUBE_SIZE < -VOLUME_SIZE.y / 2.0 {
443        velocity.y = -velocity_y;
444    }
445    if translation.y + HALF_CUBE_SIZE > VOLUME_SIZE.y / 2.0 && velocity_y > 0.0 {
446        velocity.y = 0.0;
447    }
448}
449
450fn collision_system(mut cube_query: Query<(&mut Cube, &Transform)>) {
451    cube_query.par_iter_mut().for_each(|(mut cube, transform)| {
452        handle_collision(&transform.translation, &mut cube.velocity);
453    });
454}
455
456fn counter_system(
457    diagnostics: Res<DiagnosticsStore>,
458    counter: Res<BevyCounter>,
459    query: Single<Entity, With<StatsText>>,
460    mut writer: TextUiWriter,
461) {
462    let text = *query;
463
464    if counter.is_changed() {
465        *writer.text(text, 2) = counter.count.to_string();
466    }
467
468    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
469        if let Some(raw) = fps.value() {
470            *writer.text(text, 4) = format!("{raw:.2}");
471        }
472        if let Some(sma) = fps.average() {
473            *writer.text(text, 6) = format!("{sma:.2}");
474        }
475        if let Some(ema) = fps.smoothed() {
476            *writer.text(text, 8) = format!("{ema:.2}");
477        }
478    };
479}
480
481fn init_textures(textures: &mut Vec<Handle<Image>>, args: &Args, images: &mut Assets<Image>) {
482    let mut color_rng = ChaCha8Rng::seed_from_u64(42);
483    while textures.len() < args.material_texture_count {
484        let pixel = [
485            color_rng.random(),
486            color_rng.random(),
487            color_rng.random(),
488            255,
489        ];
490        textures.push(images.add(Image::new_fill(
491            Extent3d {
492                width: CUBE_TEXTURE_SIZE as u32,
493                height: CUBE_TEXTURE_SIZE as u32,
494                depth_or_array_layers: 1,
495            },
496            TextureDimension::D2,
497            &pixel,
498            TextureFormat::Rgba8UnormSrgb,
499            RenderAssetUsages::RENDER_WORLD,
500        )));
501    }
502}
503
504fn init_materials(
505    args: &Args,
506    textures: &[Handle<Image>],
507    assets: &mut Assets<StandardMaterial>,
508) -> Vec<Handle<StandardMaterial>> {
509    let mut capacity = if args.vary_per_instance {
510        args.per_wave * args.waves
511    } else {
512        args.material_texture_count.max(args.waves)
513    };
514    if !args.benchmark {
515        capacity = capacity.max(256);
516    }
517    capacity = capacity.max(1);
518
519    let alpha_mode = match args.alpha_mode {
520        AlphaMode::Opaque => bevy::prelude::AlphaMode::Opaque,
521        AlphaMode::Blend => bevy::prelude::AlphaMode::Blend,
522        AlphaMode::AlphaMask => bevy::prelude::AlphaMode::Mask(0.5),
523    };
524
525    let mut materials = Vec::with_capacity(capacity);
526    materials.push(assets.add(StandardMaterial {
527        base_color: Color::WHITE,
528        base_color_texture: textures.first().cloned(),
529        alpha_mode,
530        ..default()
531    }));
532
533    let mut color_rng = ChaCha8Rng::seed_from_u64(42);
534    let mut texture_rng = ChaCha8Rng::seed_from_u64(42);
535    materials.extend(
536        std::iter::repeat_with(|| {
537            assets.add(StandardMaterial {
538                base_color: Color::linear_rgb(
539                    color_rng.random(),
540                    color_rng.random(),
541                    color_rng.random(),
542                ),
543                base_color_texture: textures.choose(&mut texture_rng).cloned(),
544                alpha_mode,
545                ..default()
546            })
547        })
548        .take(capacity - materials.len()),
549    );
550
551    materials
552}
```

examples/stress\_tests/bevymark.rs ([line 346](../../../src/bevymark/bevymark.rs.html#346))

```rust
323fn mouse_handler(
324    mut commands: Commands,
325    args: Res<Args>,
326    time: Res<Time>,
327    mouse_button_input: Res<ButtonInput<MouseButton>>,
328    window: Query<&Window>,
329    bird_resources: ResMut<BirdResources>,
330    mut counter: ResMut<BevyCounter>,
331    mut rng: Local<Option<ChaCha8Rng>>,
332    mut wave: Local<usize>,
333) {
334    let Ok(window) = window.single() else {
335        return;
336    };
337
338    if rng.is_none() {
339        // We're seeding the PRNG here to make this example deterministic for testing purposes.
340        // This isn't strictly required in practical use unless you need your app to be deterministic.
341        *rng = Some(ChaCha8Rng::seed_from_u64(42));
342    }
343    let rng = rng.as_mut().unwrap();
344
345    if mouse_button_input.just_released(MouseButton::Left) {
346        counter.color = Color::linear_rgb(rng.random(), rng.random(), rng.random());
347    }
348
349    if mouse_button_input.pressed(MouseButton::Left) {
350        let spawn_count = (BIRDS_PER_SECOND as f64 * time.delta_secs_f64()) as usize;
351        spawn_birds(
352            &mut commands,
353            args.into_inner(),
354            &window.resolution,
355            &mut counter,
356            spawn_count,
357            bird_resources.into_inner(),
358            None,
359            *wave,
360        );
361        *wave += 1;
362    }
363}
364
365fn bird_velocity_transform(
366    half_extents: Vec2,
367    mut translation: Vec3,
368    velocity_rng: &mut ChaCha8Rng,
369    waves: Option<usize>,
370    dt: f32,
371) -> (Transform, Vec3) {
372    let mut velocity = Vec3::new(MAX_VELOCITY * (velocity_rng.random::<f32>() - 0.5), 0., 0.);
373
374    if let Some(waves) = waves {
375        // Step the movement and handle collisions as if the wave had been spawned at fixed time intervals
376        // and with dt-spaced frames of simulation
377        for _ in 0..(waves * (FIXED_TIMESTEP / dt).round() as usize) {
378            step_movement(&mut translation, &mut velocity, dt);
379            handle_collision(half_extents, &translation, &mut velocity);
380        }
381    }
382    (
383        Transform::from_translation(translation).with_scale(Vec3::splat(BIRD_SCALE)),
384        velocity,
385    )
386}
387
388const FIXED_DELTA_TIME: f32 = 1.0 / 60.0;
389
390fn spawn_birds(
391    commands: &mut Commands,
392    args: &Args,
393    primary_window_resolution: &WindowResolution,
394    counter: &mut BevyCounter,
395    spawn_count: usize,
396    bird_resources: &mut BirdResources,
397    waves_to_simulate: Option<usize>,
398    wave: usize,
399) {
400    let bird_x = (primary_window_resolution.width() / -2.) + HALF_BIRD_SIZE;
401    let bird_y = (primary_window_resolution.height() / 2.) - HALF_BIRD_SIZE;
402
403    let half_extents = 0.5 * primary_window_resolution.size();
404
405    let color = counter.color;
406    let current_count = counter.count;
407
408    match args.mode {
409        Mode::Sprite => {
410            let batch = (0..spawn_count)
411                .map(|count| {
412                    let bird_z = if args.ordered_z {
413                        (current_count + count) as f32 * 0.00001
414                    } else {
415                        bird_resources.transform_rng.random::<f32>()
416                    };
417
418                    let (transform, velocity) = bird_velocity_transform(
419                        half_extents,
420                        Vec3::new(bird_x, bird_y, bird_z),
421                        &mut bird_resources.velocity_rng,
422                        waves_to_simulate,
423                        FIXED_DELTA_TIME,
424                    );
425
426                    let color = if args.vary_per_instance {
427                        Color::linear_rgb(
428                            bird_resources.color_rng.random(),
429                            bird_resources.color_rng.random(),
430                            bird_resources.color_rng.random(),
431                        )
432                    } else {
433                        color
434                    };
435                    (
436                        Sprite {
437                            image: bird_resources
438                                .textures
439                                .choose(&mut bird_resources.material_rng)
440                                .unwrap()
441                                .clone(),
442                            color,
443                            ..default()
444                        },
445                        transform,
446                        Bird { velocity },
447                    )
448                })
449                .collect::<Vec<_>>();
450            commands.spawn_batch(batch);
451        }
452        Mode::SpriteMesh => {
453            let alpha_mode = match args.alpha_mode {
454                AlphaMode::Opaque => SpriteAlphaMode::Opaque,
455                AlphaMode::Blend => SpriteAlphaMode::Blend,
456                AlphaMode::AlphaMask => SpriteAlphaMode::Mask(0.5),
457            };
458
459            let batch = (0..spawn_count)
460                .map(|count| {
461                    let bird_z = if args.ordered_z {
462                        (current_count + count) as f32 * 0.00001
463                    } else {
464                        bird_resources.transform_rng.random::<f32>()
465                    };
466
467                    let (transform, velocity) = bird_velocity_transform(
468                        half_extents,
469                        Vec3::new(bird_x, bird_y, bird_z),
470                        &mut bird_resources.velocity_rng,
471                        waves_to_simulate,
472                        FIXED_DELTA_TIME,
473                    );
474
475                    let color = if args.vary_per_instance {
476                        Color::linear_rgb(
477                            bird_resources.color_rng.random(),
478                            bird_resources.color_rng.random(),
479                            bird_resources.color_rng.random(),
480                        )
481                    } else {
482                        color
483                    };
484                    (
485                        SpriteMesh {
486                            image: bird_resources
487                                .textures
488                                .choose(&mut bird_resources.material_rng)
489                                .unwrap()
490                                .clone(),
491                            color,
492                            alpha_mode,
493                            ..default()
494                        },
495                        transform,
496                        Bird { velocity },
497                    )
498                })
499                .collect::<Vec<_>>();
500            commands.spawn_batch(batch);
501        }
502        Mode::Mesh2d => {
503            let batch = (0..spawn_count)
504                .map(|count| {
505                    let bird_z = if args.ordered_z {
506                        (current_count + count) as f32 * 0.00001
507                    } else {
508                        bird_resources.transform_rng.random::<f32>()
509                    };
510
511                    let (transform, velocity) = bird_velocity_transform(
512                        half_extents,
513                        Vec3::new(bird_x, bird_y, bird_z),
514                        &mut bird_resources.velocity_rng,
515                        waves_to_simulate,
516                        FIXED_DELTA_TIME,
517                    );
518
519                    let material =
520                        if args.vary_per_instance || args.material_texture_count > args.waves {
521                            bird_resources
522                                .materials
523                                .choose(&mut bird_resources.material_rng)
524                                .unwrap()
525                                .clone()
526                        } else {
527                            bird_resources.materials[wave % bird_resources.materials.len()].clone()
528                        };
529                    (
530                        Mesh2d(bird_resources.quad.clone()),
531                        MeshMaterial2d(material),
532                        transform,
533                        Bird { velocity },
534                    )
535                })
536                .collect::<Vec<_>>();
537            commands.spawn_batch(batch);
538        }
539    }
540
541    counter.count += spawn_count;
542    counter.color = Color::linear_rgb(
543        bird_resources.color_rng.random(),
544        bird_resources.color_rng.random(),
545        bird_resources.color_rng.random(),
546    );
547}
```

examples/usage/context\_menu.rs ([line 101](../../../src/context_menu/context_menu.rs.html#101))

```rust
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
```

examples/2d/cpu\_draw.rs ([lines 102-106](../../../src/cpu_draw/cpu_draw.rs.html#102-106))

```rust
92fn draw(
93    my_handle: Res<MyProcGenImage>,
94    mut images: ResMut<Assets<Image>>,
95    // Used to keep track of where we are
96    mut i: Local<u32>,
97    mut draw_color: Local<Color>,
98    mut seeded_rng: ResMut<SeededRng>,
99) {
100    if *i == 0 {
101        // Generate a random color on first run.
102        *draw_color = Color::linear_rgb(
103            seeded_rng.0.random(),
104            seeded_rng.0.random(),
105            seeded_rng.0.random(),
106        );
107    }
108
109    // Get the image from Bevy's asset storage.
110    let mut image = images.get_mut(&my_handle.0).expect("Image not found");
111
112    // Compute the position of the pixel to draw.
113
114    let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
115    let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
116    let rot_speed = 0.0123;
117    let period = 0.12345;
118
119    let r = ops::sin(*i as f32 * period) * max_radius;
120    let xy = Vec2::from_angle(*i as f32 * rot_speed) * r + center;
121    let (x, y) = (xy.x as u32, xy.y as u32);
122
123    // Get the old color of that pixel.
124    let old_color = image.get_color_at(x, y).unwrap();
125
126    // If the old color is our current color, change our drawing color.
127    let tolerance = 1.0 / 255.0;
128    if old_color.distance(&draw_color) <= tolerance {
129        *draw_color = Color::linear_rgb(
130            seeded_rng.0.random(),
131            seeded_rng.0.random(),
132            seeded_rng.0.random(),
133        );
134    }
135
136    // Set the new color, but keep old alpha value from image.
137    image
138        .set_color_at(x, y, draw_color.with_alpha(old_color.alpha()))
139        .unwrap();
140
141    *i += 1;
142}
```

examples/2d/2d\_viewport\_to\_world.rs ([line 173](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#173))

```rust
126fn setup(
127    mut commands: Commands,
128    mut meshes: ResMut<Assets<Mesh>>,
129    mut materials: ResMut<Assets<ColorMaterial>>,
130    window: Single<&Window>,
131) {
132    let window_size = window.resolution.physical_size().as_vec2();
133
134    // Initialize centered, non-window-filling viewport
135    commands.spawn((
136        Camera2d,
137        Camera {
138            viewport: Some(Viewport {
139                physical_position: (window_size * 0.125).as_uvec2(),
140                physical_size: (window_size * 0.75).as_uvec2(),
141                ..default()
142            }),
143            ..default()
144        },
145    ));
146
147    // Create a minimal UI explaining how to interact with the example
148    commands.spawn((
149        Text::new(
150            "Move the mouse to see the circle follow your cursor.\n\
151                    Use the arrow keys to move the camera.\n\
152                    Use the comma and period keys to zoom in and out.\n\
153                    Use the WASD keys to move the viewport.\n\
154                    Use the IJKL keys to resize the viewport.",
155        ),
156        Node {
157            position_type: PositionType::Absolute,
158            top: px(12),
159            left: px(12),
160            ..default()
161        },
162    ));
163
164    // Add mesh to make camera movement visible
165    commands.spawn((
166        Mesh2d(meshes.add(Rectangle::new(40.0, 20.0))),
167        MeshMaterial2d(materials.add(Color::from(GREEN))),
168    ));
169
170    // Add background to visualize viewport bounds
171    commands.spawn((
172        Mesh2d(meshes.add(Rectangle::new(50000.0, 50000.0))),
173        MeshMaterial2d(materials.add(Color::linear_rgb(0.01, 0.01, 0.01))),
174        Transform::from_translation(Vec3::new(0.0, 0.0, -200.0)),
175    ));
176}
```

Additional examples can be found in:  

*   [examples/3d/motion\_blur.rs](../../../src/motion_blur/motion_blur.rs.html#79)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#36)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#294-298)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#251)

#### pub const fn [hsla](#method.hsla)( hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), saturation: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Hsla`](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla") color.

##### Arguments

*   `hue` - Hue channel. \[0.0, 360.0\]
*   `saturation` - Saturation channel. \[0.0, 1.0\]
*   `lightness` - Lightness channel. \[0.0, 1.0\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/custom\_shader\_instancing.rs ([line 65](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#65))

```rust
56fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
57    commands.spawn((
58        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
59        InstanceMaterialData(
60            (1..=10)
61                .flat_map(|x| (1..=10).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
62                .map(|(x, y)| InstanceData {
63                    position: Vec3::new(x * 10.0 - 5.0, y * 10.0 - 5.0, 0.0),
64                    scale: 1.0,
65                    color: LinearRgba::from(Color::hsla(x * 360., y, 0.5, 1.0)).to_f32_array(),
66                })
67                .collect(),
68        ),
69        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
70        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
71        // instanced cubes will be culled.
72        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
73        // instancing, and that is not taken into account with the built-in frustum culling.
74        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
75        // component to avoid incorrect culling.
76        NoFrustumCulling,
77    ));
78
79    // camera
80    commands.spawn((
81        Camera3d::default(),
82        Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
83        // We need this component because we use `draw_indexed` and `draw`
84        // instead of `draw_indirect_indexed` and `draw_indirect` in
85        // `DrawMeshInstanced::render`.
86        NoIndirectDrawing,
87    ));
88}
```

Hide additional examples

examples/testbed/2d.rs ([line 227](../../../src/testbed_2d/2d.rs.html#227))

```rust
193    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
194        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Text)));
195
196        for (i, justify) in [
197            Justify::Left,
198            Justify::Right,
199            Justify::Center,
200            Justify::Justified,
201        ]
202        .into_iter()
203        .enumerate()
204        {
205            let y = 230. - 150. * i as f32;
206            spawn_anchored_text(&mut commands, -300. * Vec3::X + y * Vec3::Y, justify, None);
207            spawn_anchored_text(
208                &mut commands,
209                300. * Vec3::X + y * Vec3::Y,
210                justify,
211                Some(TextBounds::new(150., 60.)),
212            );
213        }
214
215        let sans_serif = TextFont::from(asset_server.load("fonts/FiraSans-Bold.ttf"));
216
217        const NUM_ITERATIONS: usize = 10;
218        for i in 0..NUM_ITERATIONS {
219            let fraction = i as f32 / (NUM_ITERATIONS - 1) as f32;
220
221            commands.spawn((
222                Text2d::new("Bevy"),
223                sans_serif.clone(),
224                Transform::from_xyz(0.0, fraction * 200.0, i as f32)
225                    .with_scale(1.0 + Vec2::splat(fraction).extend(1.))
226                    .with_rotation(Quat::from_rotation_z(fraction * core::f32::consts::PI)),
227                TextColor(Color::hsla(fraction * 360.0, 0.8, 0.8, 0.8)),
228                DespawnOnExit(super::Scene::Text),
229            ));
230        }
231
232        commands.spawn((
233            Text2d::new("This text is invisible."),
234            Visibility::Hidden,
235            DespawnOnExit(super::Scene::Text),
236        ));
237    }
```

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#267)

#### pub const fn [hsl](#method.hsl)(hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), saturation: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Hsla`](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla") color with an alpha of 1.0.

##### Arguments

*   `hue` - Hue channel. \[0.0, 360.0\]
*   `saturation` - Saturation channel. \[0.0, 1.0\]
*   `lightness` - Lightness channel. \[0.0, 1.0\]

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/ecs/observers.rs ([line 184](../../../src/observers/observers.rs.html#184))

```rust
179fn draw_shapes(mut gizmos: Gizmos, mines: Query<&Mine>) {
180    for mine in &mines {
181        gizmos.circle_2d(
182            mine.pos,
183            mine.size,
184            Color::hsl((mine.size - 4.0) / 16.0 * 360.0, 1.0, 0.8),
185        );
186    }
187}
```

Hide additional examples

examples/stress\_tests/many\_materials.rs ([lines 96-100](../../../src/many_materials/many_materials.rs.html#96-100))

```rust
89fn animate_materials(
90    material_handles: Query<&MeshMaterial3d<StandardMaterial>>,
91    time: Res<Time>,
92    mut materials: ResMut<Assets<StandardMaterial>>,
93) {
94    for (i, material_handle) in material_handles.iter().enumerate() {
95        if let Some(mut material) = materials.get_mut(material_handle) {
96            let color = Color::hsl(
97                ((i as f32 * 2.345 + time.elapsed_secs()) * 100.0) % 360.0,
98                1.0,
99                0.5,
100            );
101            material.base_color = color;
102        }
103    }
104}
```

examples/ui/ui\_material.rs ([line 99](../../../src/ui_material/ui_material.rs.html#99))

```rust
90fn animate(
91    mut materials: ResMut<Assets<CustomUiMaterial>>,
92    q: Query<&MaterialNode<CustomUiMaterial>>,
93    time: Res<Time>,
94) {
95    let duration = 2.0;
96    for handle in &q {
97        if let Some(mut material) = materials.get_mut(handle) {
98            // rainbow color effect
99            let new_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 1., 0.5);
100            let border_color = Color::hsl((time.elapsed_secs() * 60.0) % 360.0, 0.75, 0.75);
101            material.color = new_color.to_linear().to_vec4();
102            material.slider.x =
103                ((time.elapsed_secs() % (duration * 2.0)) - duration).abs() / duration;
104            material.border_color = border_color.to_linear().to_vec4();
105        }
106    }
107}
```

examples/ecs/entity\_disabling.rs ([line 123](../../../src/entity_disabling/entity_disabling.rs.html#123))

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

examples/stress\_tests/many\_gradients.rs ([line 153](../../../src/many_gradients/many_gradients.rs.html#153))

```rust
137fn animate_gradients(
138    mut gradients: Query<(&mut BackgroundGradient, &GradientNode)>,
139    args: Res<Args>,
140    time: Res<Time>,
141) {
142    if !args.animate {
143        return;
144    }
145
146    let t = time.elapsed_secs();
147
148    for (mut bg_gradient, node) in &mut gradients {
149        let offset = node.index as f32 * 0.01;
150        let hue_shift = sin(t + offset) * 0.5 + 0.5;
151
152        if let Some(Gradient::Linear(gradient)) = bg_gradient.0.get_mut(0) {
153            let color1 = Color::hsl(hue_shift * 360.0, 1.0, 0.5);
154            let color2 = Color::hsl((hue_shift + 0.3) * 360.0 % 360.0, 1.0, 0.5);
155
156            gradient.stops = vec![
157                ColorStop::new(color1, percent(0)),
158                ColorStop::new(color2, percent(100)),
159                ColorStop::new(
160                    Color::hsl((hue_shift + 0.1) * 360.0 % 360.0, 1.0, 0.5),
161                    percent(20),
162                ),
163                ColorStop::new(
164                    Color::hsl((hue_shift + 0.15) * 360.0 % 360.0, 1.0, 0.5),
165                    percent(40),
166                ),
167                ColorStop::new(
168                    Color::hsl((hue_shift + 0.2) * 360.0 % 360.0, 1.0, 0.5),
169                    percent(60),
170                ),
171                ColorStop::new(
172                    Color::hsl((hue_shift + 0.25) * 360.0 % 360.0, 1.0, 0.5),
173                    percent(80),
174                ),
175                ColorStop::new(
176                    Color::hsl((hue_shift + 0.28) * 360.0 % 360.0, 1.0, 0.5),
177                    percent(90),
178                ),
179            ];
180        }
181    }
182}
```

examples/testbed/2d.rs ([line 139](../../../src/testbed_2d/2d.rs.html#139))

```rust
112    pub fn setup(
113        mut commands: Commands,
114        mut meshes: ResMut<Assets<Mesh>>,
115        mut materials: ResMut<Assets<ColorMaterial>>,
116    ) {
117        commands.spawn((Camera2d, DespawnOnExit(super::Scene::Shapes)));
118
119        let shapes = [
120            meshes.add(Circle::new(50.0)),
121            meshes.add(CircularSector::new(50.0, 1.0)),
122            meshes.add(CircularSegment::new(50.0, 1.25)),
123            meshes.add(Ellipse::new(25.0, 50.0)),
124            meshes.add(Annulus::new(25.0, 50.0)),
125            meshes.add(Capsule2d::new(25.0, 50.0)),
126            meshes.add(Rhombus::new(75.0, 100.0)),
127            meshes.add(Rectangle::new(50.0, 100.0)),
128            meshes.add(RegularPolygon::new(50.0, 6)),
129            meshes.add(Triangle2d::new(
130                Vec2::Y * 50.0,
131                Vec2::new(-50.0, -50.0),
132                Vec2::new(50.0, -50.0),
133            )),
134        ];
135        let num_shapes = shapes.len();
136
137        for (i, shape) in shapes.into_iter().enumerate() {
138            // Distribute colors evenly across the rainbow.
139            let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
140
141            commands.spawn((
142                Mesh2d(shape),
143                MeshMaterial2d(materials.add(color)),
144                Transform::from_xyz(
145                    // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
146                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
147                    0.0,
148                    0.0,
149                ),
150                DespawnOnExit(super::Scene::Shapes),
151            ));
152        }
153    }
```

Additional examples can be found in:  

*   [examples/stress\_tests/many\_buttons.rs](../../../src/many_buttons/many_buttons.rs.html#169)
*   [examples/stress\_tests/many\_lights.rs](../../../src/many_lights/many_lights.rs.html#81)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../../src/drag_to_scroll/drag_to_scroll.rs.html#84)
*   [examples/2d/2d\_shapes.rs](../../../src/2d_shapes/2d_shapes.rs.html#83)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#1213)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#284)

#### pub const fn [hsva](#method.hsva)(hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), saturation: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Hsva`](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva") color.

##### Arguments

*   `hue` - Hue channel. \[0.0, 360.0\]
*   `saturation` - Saturation channel. \[0.0, 1.0\]
*   `value` - Value channel. \[0.0, 1.0\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/3d/specular\_tint.rs ([line 111](../../../src/specular_tint/specular_tint.rs.html#111))

```rust
75fn setup(
76    mut commands: Commands,
77    asset_server: Res<AssetServer>,
78    app_status: Res<AppStatus>,
79    mut meshes: ResMut<Assets<Mesh>>,
80    mut standard_materials: ResMut<Assets<StandardMaterial>>,
81) {
82    // Spawns a camera.
83    commands.spawn((
84        Transform::from_xyz(-2.0, 0.0, 3.5).looking_at(Vec3::ZERO, Vec3::Y),
85        Hdr,
86        Camera3d::default(),
87        Skybox {
88            image: Some(asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2")),
89            brightness: 3000.0,
90            ..default()
91        },
92        EnvironmentMapLight {
93            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
94            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
95            // We want relatively high intensity here in order for the specular
96            // tint to show up well.
97            intensity: 25000.0,
98            ..default()
99        },
100    ));
101
102    // Spawn the sphere.
103    commands.spawn((
104        Transform::from_rotation(Quat::from_rotation_x(PI * 0.5)),
105        Mesh3d(meshes.add(Sphere::default().mesh().uv(32, 18))),
106        MeshMaterial3d(standard_materials.add(StandardMaterial {
107            // We want only reflected specular light here, so we set the base
108            // color as black.
109            base_color: Color::BLACK,
110            reflectance: 1.0,
111            specular_tint: Color::hsva(app_status.hue, 1.0, 1.0, 1.0),
112            // The object must not be metallic, or else the reflectance is
113            // ignored per the Filament spec:
114            //
115            // <https://google.github.io/filament/Filament.md.html#listing_fnormal>
116            metallic: 0.0,
117            perceptual_roughness: 0.0,
118            ..default()
119        })),
120    ));
121
122    // Spawn the help text.
123    commands.spawn((
124        Node {
125            position_type: PositionType::Absolute,
126            bottom: px(12),
127            left: px(12),
128            ..default()
129        },
130        app_status.create_text(),
131    ));
132}
133
134/// Rotates the camera a bit every frame.
135fn rotate_camera(mut cameras: Query<&mut Transform, With<Camera3d>>) {
136    for mut camera_transform in cameras.iter_mut() {
137        camera_transform.translation =
138            Quat::from_rotation_y(ROTATION_SPEED) * camera_transform.translation;
139        camera_transform.look_at(Vec3::ZERO, Vec3::Y);
140    }
141}
142
143/// Alters the hue of the solid color a bit every frame.
144fn shift_hue(
145    mut app_status: ResMut<AppStatus>,
146    objects_with_materials: Query<&MeshMaterial3d<StandardMaterial>>,
147    mut standard_materials: ResMut<Assets<StandardMaterial>>,
148) {
149    if app_status.tint_type != TintType::Solid {
150        return;
151    }
152
153    app_status.hue += HUE_SHIFT_SPEED;
154
155    for material_handle in objects_with_materials.iter() {
156        let Some(mut material) = standard_materials.get_mut(material_handle) else {
157            continue;
158        };
159        material.specular_tint = Color::hsva(app_status.hue, 1.0, 1.0, 1.0);
160    }
161}
```

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#300)

#### pub const fn [hsv](#method.hsv)(hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), saturation: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Hsva`](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva") color with an alpha of 1.0.

##### Arguments

*   `hue` - Hue channel. \[0.0, 360.0\]
*   `saturation` - Saturation channel. \[0.0, 1.0\]
*   `value` - Value channel. \[0.0, 1.0\]

##### [Examples found in repository](#scraped-examples-11)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_cameras\_lights.rs ([line 59](../../../src/many_cameras_lights/many_cameras_lights.rs.html#59))

```rust
34fn setup(
35    mut commands: Commands,
36    mut meshes: ResMut<Assets<Mesh>>,
37    mut materials: ResMut<Assets<StandardMaterial>>,
38    window: Query<&Window>,
39) -> Result {
40    // circular base
41    commands.spawn((
42        Mesh3d(meshes.add(Circle::new(4.0))),
43        MeshMaterial3d(materials.add(Color::WHITE)),
44        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
45    ));
46
47    // cube
48    commands.spawn((
49        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
50        MeshMaterial3d(materials.add(Color::WHITE)),
51        Transform::from_xyz(0.0, 0.5, 0.0),
52    ));
53
54    // lights
55    for i in 0..NUM_LIGHTS {
56        let angle = (i as f32) / (NUM_LIGHTS as f32) * PI * 2.0;
57        commands.spawn((
58            PointLight {
59                color: Color::hsv(angle.to_degrees(), 1.0, 1.0),
60                intensity: 2_000_000.0 / NUM_LIGHTS as f32,
61                shadow_maps_enabled: true,
62                ..default()
63            },
64            Transform::from_xyz(sin(angle) * 4.0, 2.0, cos(angle) * 4.0),
65        ));
66    }
67
68    // cameras
69    let window = window.single()?;
70    let width = window.resolution.width() / CAMERA_COLS as f32 * window.resolution.scale_factor();
71    let height = window.resolution.height() / CAMERA_ROWS as f32 * window.resolution.scale_factor();
72    let mut i = 0;
73    for y in 0..CAMERA_COLS {
74        for x in 0..CAMERA_ROWS {
75            let angle = i as f32 / (CAMERA_ROWS * CAMERA_COLS) as f32 * PI * 2.0;
76            commands.spawn((
77                Camera3d::default(),
78                Camera {
79                    viewport: Some(Viewport {
80                        physical_position: UVec2::new(
81                            (x as f32 * width) as u32,
82                            (y as f32 * height) as u32,
83                        ),
84                        physical_size: UVec2::new(width as u32, height as u32),
85                        ..default()
86                    }),
87                    order: i,
88                    ..default()
89                },
90                Transform::from_xyz(sin(angle) * 4.0, 2.5, cos(angle) * 4.0)
91                    .looking_at(Vec3::ZERO, Vec3::Y),
92            ));
93            i += 1;
94        }
95    }
96    Ok(())
97}
```

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#317)

#### pub const fn [hwba](#method.hwba)(hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), whiteness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), blackness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Hwba`](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") color.

##### Arguments

*   `hue` - Hue channel. \[0.0, 360.0\]
*   `whiteness` - Whiteness channel. \[0.0, 1.0\]
*   `blackness` - Blackness channel. \[0.0, 1.0\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#333)

#### pub const fn [hwb](#method.hwb)(hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), whiteness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), blackness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Hwba`](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba") color with an alpha of 1.0.

##### Arguments

*   `hue` - Hue channel. \[0.0, 360.0\]
*   `whiteness` - Whiteness channel. \[0.0, 1.0\]
*   `blackness` - Blackness channel. \[0.0, 1.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#350)

#### pub const fn [laba](#method.laba)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), a: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), b: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Laba`](../../prelude/struct.Laba.html "struct bevy::prelude::Laba") color.

##### Arguments

*   `lightness` - Lightness channel. \[0.0, 1.5\]
*   `a` - a axis. \[-1.5, 1.5\]
*   `b` - b axis. \[-1.5, 1.5\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#366)

#### pub const fn [lab](#method.lab)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), a: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), b: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Laba`](../../prelude/struct.Laba.html "struct bevy::prelude::Laba") color with an alpha of 1.0.

##### Arguments

*   `lightness` - Lightness channel. \[0.0, 1.5\]
*   `a` - a axis. \[-1.5, 1.5\]
*   `b` - b axis. \[-1.5, 1.5\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#383)

#### pub const fn [lcha](#method.lcha)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), chroma: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Lcha`](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha") color.

##### Arguments

*   `lightness` - Lightness channel. \[0.0, 1.5\]
*   `chroma` - Chroma channel. \[0.0, 1.5\]
*   `hue` - Hue channel. \[0.0, 360.0\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#399)

#### pub const fn [lch](#method.lch)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), chroma: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Lcha`](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha") color with an alpha of 1.0.

##### Arguments

*   `lightness` - Lightness channel. \[0.0, 1.5\]
*   `chroma` - Chroma channel. \[0.0, 1.5\]
*   `hue` - Hue channel. \[0.0, 360.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#416)

#### pub const fn [oklaba](#method.oklaba)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), a: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), b: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Oklaba`](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba") color.

##### Arguments

*   `lightness` - Lightness channel. \[0.0, 1.0\]
*   `a` - Green-red channel. \[-1.0, 1.0\]
*   `b` - Blue-yellow channel. \[-1.0, 1.0\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#432)

#### pub const fn [oklab](#method.oklab)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), a: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), b: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Oklaba`](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba") color with an alpha of 1.0.

##### Arguments

*   `lightness` - Lightness channel. \[0.0, 1.0\]
*   `a` - Green-red channel. \[-1.0, 1.0\]
*   `b` - Blue-yellow channel. \[-1.0, 1.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#449)

#### pub const fn [oklcha](#method.oklcha)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), chroma: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Oklcha`](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha") color.

##### Arguments

*   `lightness` - Lightness channel. \[0.0, 1.0\]
*   `chroma` - Chroma channel. \[0.0, 1.0\]
*   `hue` - Hue channel. \[0.0, 360.0\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#465)

#### pub const fn [oklch](#method.oklch)(lightness: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), chroma: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Oklcha`](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha") color with an alpha of 1.0.

##### Arguments

*   `lightness` - Lightness channel. \[0.0, 1.0\]
*   `chroma` - Chroma channel. \[0.0, 1.0\]
*   `hue` - Hue channel. \[0.0, 360.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#482)

#### pub const fn [xyza](#method.xyza)(x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Xyza`](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza") color.

##### Arguments

*   `x` - x-axis. \[0.0, 1.0\]
*   `y` - y-axis. \[0.0, 1.0\]
*   `z` - z-axis. \[0.0, 1.0\]
*   `alpha` - Alpha channel. \[0.0, 1.0\]

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#493)

#### pub const fn [xyz](#method.xyz)(x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Creates a new [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") object storing a [`Xyza`](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza") color with an alpha of 1.0.

##### Arguments

*   `x` - x-axis. \[0.0, 1.0\]
*   `y` - y-axis. \[0.0, 1.0\]
*   `z` - z-axis. \[0.0, 1.0\]

## Trait Implementations

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#519)

### impl [Alpha](../../prelude/trait.Alpha.html "trait bevy::prelude::Alpha") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#520)

#### fn [with\_alpha](../../prelude/trait.Alpha.html#tymethod.with_alpha)(&self, alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Return a new version of this color with the given alpha value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#539)

#### fn [alpha](../../prelude/trait.Alpha.html#tymethod.alpha)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Return the alpha component of this color.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#554)

#### fn [set\_alpha](../../prelude/trait.Alpha.html#tymethod.set_alpha)(&mut self, alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Sets the alpha component of this color.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#70)

#### fn [is\_fully\_transparent](../../prelude/trait.Alpha.html#method.is_fully_transparent)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Is the alpha component of this color less than or equal to 0.0?

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#75)

#### fn [is\_fully\_opaque](../../prelude/trait.Alpha.html#method.is_fully_opaque)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Is the alpha component of this color greater than or equal to 1.0?

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#512)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#514)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

A fully white [`Color::LinearRgba`](../../prelude/enum.Color.html#variant.LinearRgba "variant bevy::prelude::Color::LinearRgba") color with an alpha of 1.0.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#51)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#51)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [Enum](../../reflect/enums/trait.Enum.html "trait bevy::reflect::enums::Enum") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [field](../../reflect/enums/trait.Enum.html#tymethod.field)(&self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field (in the current variant) with the given name. [Read more](../../reflect/enums/trait.Enum.html#tymethod.field)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [field\_at](../../reflect/enums/trait.Enum.html#tymethod.field_at)( &self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field (in the current variant) at the given index.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [field\_mut](../../reflect/enums/trait.Enum.html#tymethod.field_mut)( &mut self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field (in the current variant) with the given name. [Read more](../../reflect/enums/trait.Enum.html#tymethod.field_mut)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [field\_at\_mut](../../reflect/enums/trait.Enum.html#tymethod.field_at_mut)( &mut self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field (in the current variant) at the given index.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [index\_of](../../reflect/enums/trait.Enum.html#tymethod.index_of)(&self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Returns the index of the field (in the current variant) with the given name. [Read more](../../reflect/enums/trait.Enum.html#tymethod.index_of)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [name\_at](../../reflect/enums/trait.Enum.html#tymethod.name_at)(&self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the field (in the current variant) with the given index. [Read more](../../reflect/enums/trait.Enum.html#tymethod.name_at)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [iter\_fields](../../reflect/enums/trait.Enum.html#tymethod.iter_fields)(&self) -> [VariantFieldIter](../../reflect/enums/struct.VariantFieldIter.html "struct bevy::reflect::enums::VariantFieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the current variant’s fields.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [field\_len](../../reflect/enums/trait.Enum.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the current variant.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [variant\_name](../../reflect/enums/trait.Enum.html#tymethod.variant_name)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

The name of the current variant.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [variant\_index](../../reflect/enums/trait.Enum.html#tymethod.variant_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

The index of the current variant.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [variant\_type](../../reflect/enums/trait.Enum.html#tymethod.variant_type)(&self) -> [VariantType](../../reflect/enums/enum.VariantType.html "enum bevy::reflect::enums::VariantType")

The type of the current variant.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [to\_dynamic\_enum](../../reflect/enums/trait.Enum.html#method.to_dynamic_enum)(&self) -> [DynamicEnum](../../reflect/enums/struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum")

Creates a new [`DynamicEnum`](../../reflect/enums/struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum") from this enum.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#133)

#### fn [is\_variant](../../reflect/enums/trait.Enum.html#method.is_variant)(&self, variant\_type: [VariantType](../../reflect/enums/enum.VariantType.html "enum bevy::reflect::enums::VariantType")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the current variant’s type matches the given one.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#137)

#### fn [variant\_path](../../reflect/enums/trait.Enum.html#method.variant_path)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Returns the full path to the current variant.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#144)

#### fn [get\_represented\_enum\_info](../../reflect/enums/trait.Enum.html#method.get_represented_enum_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [EnumInfo](../../reflect/enums/struct.EnumInfo.html "struct bevy::reflect::enums::EnumInfo")\>

Will return `None` if [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#916)

### impl [EuclideanDistance](../color_difference/trait.EuclideanDistance.html "trait bevy::color::color_difference::EuclideanDistance") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#917)

#### fn [distance\_squared](../color_difference/trait.EuclideanDistance.html#tymethod.distance_squared)(&self, other: &[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Distance squared from `self` to `other`.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_difference.rs.html#9)

#### fn [distance](../color_difference/trait.EuclideanDistance.html#method.distance)(&self, other: &Self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Distance from `self` to `other`.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#570)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#571)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#587)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#588)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#604)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [Hsla](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#605)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [Hsla](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#621)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [Hsva](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#622)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [Hsva](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#638)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [Hwba](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#639)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [Hwba](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#655)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#656)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#672)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [Lcha](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#673)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [Lcha](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#689)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#690)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#706)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#707)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#723)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#724)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#11)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [ClearColorConfig](../../prelude/enum.ClearColorConfig.html "enum bevy::prelude::ClearColorConfig")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#11)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [ClearColorConfig](../../prelude/enum.ClearColorConfig.html "enum bevy::prelude::ClearColorConfig")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#943)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [StandardMaterial](../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#944)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(color: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [StandardMaterial](../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#82)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [ColorStop](../../prelude/struct.ColorStop.html "struct bevy::prelude::ColorStop")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#83)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(color: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [ColorStop](../../prelude/struct.ColorStop.html "struct bevy::prelude::ColorStop")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#194)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [AngularColorStop](../../prelude/struct.AngularColorStop.html "struct bevy::prelude::AngularColorStop")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#195)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(color: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [AngularColorStop](../../prelude/struct.AngularColorStop.html "struct bevy::prelude::AngularColorStop")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#67)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\> for [ColorMaterial](../../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#68)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(color: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [ColorMaterial](../../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Hsla](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Hsla](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Hsva](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Hsva](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Hwba](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Hwba](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Lcha](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Lcha](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")\> for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### type [This](../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

The type to convert into. [Read more](../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [from\_arg](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color") as [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [from\_reflect](../../prelude/trait.FromReflect.html#tymethod.from_reflect)(\_\_param0: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [GetOwnership](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [ownership](../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [get\_type\_registration](../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [register\_type\_dependencies](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#817)

### impl [Hue](../../prelude/trait.Hue.html "trait bevy::prelude::Hue") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#818)

#### fn [with\_hue](../../prelude/trait.Hue.html#tymethod.with_hue)(&self, hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Return a new version of this color with the hue channel set to the given value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#837)

#### fn [hue](../../prelude/trait.Hue.html#tymethod.hue)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Return the hue of this color \[0.0, 360.0\].

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#852)

#### fn [set\_hue](../../prelude/trait.Hue.html#tymethod.set_hue)(&mut self, hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Sets the hue of this color.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#106)

#### fn [rotate\_hue](../../prelude/trait.Hue.html#method.rotate_hue)(&self, degrees: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Return a new version of this color with the hue channel rotated by the given degrees.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [IntoReturn](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [into\_return](../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"): 'into\_return,

Converts [`Self`](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#743)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#744)

#### fn [luminance](../../prelude/trait.Luminance.html#tymethod.luminance)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Return the luminance of this color (0.0 - 1.0).

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#759)

#### fn [with\_luminance](../../prelude/trait.Luminance.html#tymethod.with_luminance)(&self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Return a new version of this color with the given luminance. The resulting color will be clamped to the valid range for the color space; for some color spaces, clamping may cause the hue or chroma to change.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#778)

#### fn [darker](../../prelude/trait.Luminance.html#tymethod.darker)(&self, amount: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Return a darker version of this color. The `amount` should be between 0.0 and 1.0. The amount represents an absolute decrease in luminance, and is distributive: `color.darker(a).darker(b) == color.darker(a + b)`. Colors are clamped to black if the amount would cause them to go below black. [Read more](../../prelude/trait.Luminance.html#tymethod.darker)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#797)

#### fn [lighter](../../prelude/trait.Luminance.html#tymethod.lighter)(&self, amount: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Return a lighter version of this color. The `amount` should be between 0.0 and 1.0. The amount represents an absolute increase in luminance, and is distributive: `color.lighter(a).lighter(b) == color.lighter(a + b)`. Colors are clamped to white if the amount would cause them to go above white. [Read more](../../prelude/trait.Luminance.html#tymethod.lighter)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#895)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#896)

#### fn [mix](../../prelude/trait.Mix.html#tymethod.mix)(&self, other: &[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Linearly interpolate between this and another color, by factor. Factor should be between 0.0 and 1.0.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#40)

#### fn [mix\_assign](../../prelude/trait.Mix.html#method.mix_assign)(&mut self, other: Self, factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Linearly interpolate between this and another color, by factor, storing the result in this color. Factor should be between 0.0 and 1.0.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [get\_represented\_type\_info](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [try\_apply](../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, \_\_value\_param: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [reflect\_kind](../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [reflect\_ref](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [reflect\_owned](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\>) -> [ReflectOwned](../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [try\_into\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [try\_as\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [try\_as\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [into\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [as\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [as\_partial\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [reflect\_hash](../../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#49)

#### fn [reflect\_partial\_eq](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [reflect\_partial\_cmp](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#49)

#### fn [reflect\_clone](../../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](../../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#363)

#### fn [debug](../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [into\_any](../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [as\_any](../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [as\_any\_mut](../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [into\_reflect](../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [as\_reflect](../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [as\_reflect\_mut](../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [set](../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#857)

### impl [Saturation](../../prelude/trait.Saturation.html "trait bevy::prelude::Saturation") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#858)

#### fn [with\_saturation](../../prelude/trait.Saturation.html#tymethod.with_saturation)(&self, saturation: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

Return a new version of this color with the saturation channel set to the given value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#875)

#### fn [saturation](../../prelude/trait.Saturation.html#tymethod.saturation)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Return the saturation of this color \[0.0, 1.0\].

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#890)

#### fn [set\_saturation](../../prelude/trait.Saturation.html#tymethod.set_saturation)(&mut self, saturation: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Sets the saturation of this color.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#51)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#51)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#45)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#933)

### impl [TryStableInterpolate](../../math/trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#934)

#### type [Error](../../math/trait.TryStableInterpolate.html#associatedtype.Error) = [MismatchedUnitsError](../../math/struct.MismatchedUnitsError.html "struct bevy::math::MismatchedUnitsError")

Error produced when the value cannot be interpolated.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#936)

#### fn [try\_interpolate\_stable](../../math/trait.TryStableInterpolate.html#tymethod.try_interpolate_stable)( &self, other: &[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), <[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color") as [TryStableInterpolate](../../math/trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate")\>::[Error](../../math/trait.TryStableInterpolate.html#associatedtype.Error "type bevy::math::TryStableInterpolate::Error")\>

Attempt to interpolate the value. This may fail if the two interpolation values have different units, or if the type is not interpolable.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

#### fn [type\_info](../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

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

[Source](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)

### impl<T> [Brush](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/style/brush/trait.Brush.html "trait parley::style::brush::Brush") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](../../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](../../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](../../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](../../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](../../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

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

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](../../prelude/trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](../../prelude/trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](../../prelude/trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](../../prelude/trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

### impl<T> [IntoResult](../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

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

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#method.clone_into)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","VariantFieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../reflect/enums/struct.VariantFieldIter.html\\" title=\\"struct bevy::reflect::enums::VariantFieldIter\\">VariantFieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../reflect/enums/struct.VariantFieldIter.html\\" title=\\"struct bevy::reflect::enums::VariantFieldIter\\">VariantFieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"enum\\" href=\\"../../reflect/enums/enum.VariantField.html\\" title=\\"enum bevy::reflect::enums::VariantField\\">VariantField</a>&lt;'a&gt;;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}