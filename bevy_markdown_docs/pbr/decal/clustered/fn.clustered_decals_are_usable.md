[bevy](../../../index.html)::[pbr](../../index.html)::[decal](../index.html)::[clustered](index.html)

# Function clustered\_decals\_are\_usable 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#553-556)

```rust
pub fn clustered_decals_are_usable(
    render_device: &RenderDevice,
    render_adapter: &RenderAdapter,
) -> bool
```

Returns true if clustered decals are usable on the current platform or false otherwise.

Clustered decals are currently disabled on macOS and iOS due to insufficient texture bindings and limited bindless support in `wgpu`.

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/3d/clustered\_decals.rs ([line 164](../../../../src/clustered_decals/clustered_decals.rs.html#164))

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

Hide additional examples

examples/3d/light\_textures.rs ([line 153](../../../../src/light_textures/light_textures.rs.html#153))

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