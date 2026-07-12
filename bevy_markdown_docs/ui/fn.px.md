[bevy](../index.html)::[ui](index.html)

# Function px 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#541)

```rust
pub fn px<T>(value: T) -> Valwhere
    T: ValNum,
```

Returns a [`Val::Px`](../prelude/enum.Val.html#variant.Px "variant bevy::prelude::Val::Px") representing a value in logical pixels.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/3d/../helpers/widgets.rs ([line 51](../../src/clustered_decal_maps/helpers/widgets.rs.html#51))

```rust
47pub fn main_ui_node() -> Node {
48    Node {
49        flex_direction: FlexDirection::Column,
50        position_type: PositionType::Absolute,
51        row_gap: px(6),
52        left: px(10),
53        bottom: px(10),
54        ..default()
55    }
56}
57
58/// Spawns a single radio button that allows configuration of a setting.
59///
60/// The type parameter specifies the value that will be packaged up and sent in
61/// a [`WidgetClickEvent`] when the radio button is clicked.
62pub fn option_button<T>(
63    option_value: T,
64    option_name: &str,
65    is_selected: bool,
66    is_first: bool,
67    is_last: bool,
68) -> impl Bundle
69where
70    T: Clone + Send + Sync + 'static,
71{
72    let (bg_color, fg_color) = if is_selected {
73        (Color::WHITE, Color::BLACK)
74    } else {
75        (Color::BLACK, Color::WHITE)
76    };
77
78    // Add the button node.
79    (
80        Button,
81        Node {
82            border: BUTTON_BORDER.with_left(if is_first { px(1) } else { px(0) }),
83            justify_content: JustifyContent::Center,
84            align_items: AlignItems::Center,
85            padding: BUTTON_PADDING,
86            border_radius: BorderRadius::ZERO
87                .with_left(if is_first {
88                    BUTTON_BORDER_RADIUS_SIZE
89                } else {
90                    px(0)
91                })
92                .with_right(if is_last {
93                    BUTTON_BORDER_RADIUS_SIZE
94                } else {
95                    px(0)
96                }),
97            ..default()
98        },
99        BUTTON_BORDER_COLOR,
100        BackgroundColor(bg_color),
101        RadioButton,
102        WidgetClickSender(option_value.clone()),
103        children![(
104            ui_text(option_name, fg_color),
105            RadioButtonText,
106            WidgetClickSender(option_value),
107        )],
108    )
109}
110
111/// Spawns the buttons that allow configuration of a setting.
112///
113/// The user may change the setting to any one of the labeled `options`. The
114/// value of the given type parameter will be packaged up and sent as a
115/// [`WidgetClickEvent`] when one of the radio buttons is clicked.
116pub fn option_buttons<T>(title: &str, options: &[(T, &str)]) -> impl Bundle
117where
118    T: Clone + Send + Sync + 'static,
119{
120    let buttons = options
121        .iter()
122        .cloned()
123        .enumerate()
124        .map(|(option_index, (option_value, option_name))| {
125            option_button(
126                option_value,
127                option_name,
128                option_index == 0,
129                option_index == 0,
130                option_index == options.len() - 1,
131            )
132        })
133        .collect::<Vec<_>>();
134    // Add the parent node for the row.
135    (
136        Node {
137            align_items: AlignItems::Center,
138            ..default()
139        },
140        Children::spawn((
141            Spawn((
142                ui_text(title, Color::WHITE),
143                Node {
144                    width: px(150),
145                    ..default()
146                },
147            )),
148            SpawnIter(buttons.into_iter()),
149        )),
150    )
151}
```

Hide additional examples

examples/3d/post\_processing.rs ([line 149](../../src/post_processing/post_processing.rs.html#149))

```rust
144fn spawn_text(commands: &mut Commands) {
145    commands.spawn((
146        Text::default(),
147        Node {
148            position_type: PositionType::Absolute,
149            top: px(12),
150            left: px(12),
151            ..default()
152        },
153    ));
154}
```

examples/3d/occlusion\_culling.rs ([line 381](../../src/occlusion_culling/occlusion_culling.rs.html#381))

```rust
376fn spawn_help_text(commands: &mut Commands) {
377    commands.spawn((
378        Text::new(""),
379        Node {
380            position_type: PositionType::Absolute,
381            top: px(12),
382            left: px(12),
383            ..default()
384        },
385    ));
386}
```

examples/3d/fog.rs ([line 130](../../src/fog/fog.rs.html#130))

```rust
125fn setup_instructions(mut commands: Commands) {
126    commands.spawn((
127        Text::default(),
128        Node {
129            position_type: PositionType::Absolute,
130            top: px(12),
131            left: px(12),
132            ..default()
133        },
134    ));
135}
```

examples/animation/animation\_graph.rs ([line 263](../../src/animation_graph/animation_graph.rs.html#263))

```rust
258fn setup_help_text(commands: &mut Commands) {
259    commands.spawn((
260        Text::new(HELP_TEXT),
261        Node {
262            position_type: PositionType::Absolute,
263            top: px(12),
264            left: px(12),
265            ..default()
266        },
267    ));
268}
269
270/// Initializes the node UI widgets.
271fn setup_node_rects(commands: &mut Commands) {
272    for (node_rect, node_type) in NODE_RECTS.iter().zip(NODE_TYPES.iter()) {
273        let node_string = match *node_type {
274            NodeType::Clip(ref clip) => clip.text,
275            NodeType::Blend(text) => text,
276        };
277
278        let text = commands
279            .spawn((
280                Text::new(node_string),
281                TextFont {
282                    font_size: FontSize::Px(16.0),
283                    ..default()
284                },
285                TextColor(ANTIQUE_WHITE.into()),
286                TextLayout::justify(Justify::Center),
287            ))
288            .id();
289
290        let container = {
291            let mut container = commands.spawn((
292                Node {
293                    position_type: PositionType::Absolute,
294                    bottom: px(node_rect.bottom),
295                    left: px(node_rect.left),
296                    height: px(node_rect.height),
297                    width: px(node_rect.width),
298                    align_items: AlignItems::Center,
299                    justify_items: JustifyItems::Center,
300                    align_content: AlignContent::Center,
301                    justify_content: JustifyContent::Center,
302                    ..default()
303                },
304                BorderColor::all(WHITE),
305                Outline::new(px(1), Val::ZERO, Color::WHITE),
306            ));
307
308            if let NodeType::Clip(clip) = node_type {
309                container.insert((
310                    Interaction::None,
311                    RelativeCursorPosition::default(),
312                    (*clip).clone(),
313                ));
314            }
315
316            container.id()
317        };
318
319        // Create the background color.
320        if let NodeType::Clip(_) = node_type {
321            let background = commands
322                .spawn((
323                    Node {
324                        position_type: PositionType::Absolute,
325                        top: px(0),
326                        left: px(0),
327                        height: px(node_rect.height),
328                        width: px(node_rect.width),
329                        ..default()
330                    },
331                    BackgroundColor(DARK_GREEN.into()),
332                ))
333                .id();
334
335            commands.entity(container).add_child(background);
336        }
337
338        commands.entity(container).add_child(text);
339    }
340}
341
342/// Creates boxes for the horizontal and vertical lines.
343///
344/// This is a bit hacky: it uses 1-pixel-wide and 1-pixel-high boxes to draw
345/// vertical and horizontal lines, respectively.
346fn setup_node_lines(commands: &mut Commands) {
347    for line in &HORIZONTAL_LINES {
348        commands.spawn((
349            Node {
350                position_type: PositionType::Absolute,
351                bottom: px(line.bottom),
352                left: px(line.left),
353                height: px(0),
354                width: px(line.length),
355                border: UiRect::bottom(px(1)),
356                ..default()
357            },
358            BorderColor::all(WHITE),
359        ));
360    }
361
362    for line in &VERTICAL_LINES {
363        commands.spawn((
364            Node {
365                position_type: PositionType::Absolute,
366                bottom: px(line.bottom),
367                left: px(line.left),
368                height: px(line.length),
369                width: px(0),
370                border: UiRect::left(px(1)),
371                ..default()
372            },
373            BorderColor::all(WHITE),
374        ));
375    }
376}
377
378/// Attaches the animation graph to the scene, and plays all three animations.
379fn init_animations(
380    mut commands: Commands,
381    mut query: Query<(Entity, &mut AnimationPlayer)>,
382    animation_graph: Res<ExampleAnimationGraph>,
383    mut done: Local<bool>,
384) {
385    if *done {
386        return;
387    }
388
389    for (entity, mut player) in query.iter_mut() {
390        commands.entity(entity).insert((
391            AnimationGraphHandle(animation_graph.0.clone()),
392            ExampleAnimationWeights::default(),
393        ));
394        for &node_index in &CLIP_NODE_INDICES {
395            player.play(node_index.into()).repeat();
396        }
397
398        *done = true;
399    }
400}
401
402/// Read cursor position relative to clip nodes, allowing the user to change weights
403/// when dragging the node UI widgets.
404fn handle_weight_drag(
405    mut interaction_query: Query<(&Interaction, &RelativeCursorPosition, &ClipNode)>,
406    mut animation_weights_query: Query<&mut ExampleAnimationWeights>,
407) {
408    for (interaction, relative_cursor, clip_node) in &mut interaction_query {
409        if !matches!(*interaction, Interaction::Pressed) {
410            continue;
411        }
412
413        let Some(pos) = relative_cursor.normalized else {
414            continue;
415        };
416
417        for mut animation_weights in animation_weights_query.iter_mut() {
418            animation_weights.weights[clip_node.index] = pos.x.clamp(0., 1.);
419        }
420    }
421}
422
423// Updates the UI based on the weights that the user has chosen.
424fn update_ui(
425    mut text_query: Query<&mut Text>,
426    mut background_query: Query<&mut Node, Without<Text>>,
427    container_query: Query<(&Children, &ClipNode)>,
428    animation_weights_query: Query<&ExampleAnimationWeights, Changed<ExampleAnimationWeights>>,
429) {
430    for animation_weights in animation_weights_query.iter() {
431        for (children, clip_node) in &container_query {
432            // Draw the green background color to visually indicate the weight.
433            let mut bg_iter = background_query.iter_many_mut(children);
434            if let Some(mut node) = bg_iter.fetch_next() {
435                // All nodes are the same width, so `NODE_RECTS[0]` is as good as any other.
436                node.width = px(NODE_RECTS[0].width * animation_weights.weights[clip_node.index]);
437            }
438
439            // Update the node labels with the current weights.
440            let mut text_iter = text_query.iter_many_mut(children);
441            if let Some(mut text) = text_iter.fetch_next() {
442                **text = format!(
443                    "{}\n{:.2}",
444                    clip_node.text, animation_weights.weights[clip_node.index]
445                );
446            }
447        }
448    }
449}
```

examples/3d/light\_probe\_blending.rs ([line 382](../../src/light_probe_blending/light_probe_blending.rs.html#382))

```rust
377fn spawn_help_text(commands: &mut Commands) {
378    commands.spawn((
379        Text::new(""),
380        Node {
381            position_type: PositionType::Absolute,
382            top: px(12),
383            left: px(12),
384            ..default()
385        },
386        HelpText,
387    ));
388}
```

Additional examples can be found in:  

*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#296)
*   [examples/app/logs.rs](../../src/logs/logs.rs.html#26)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#143)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#235)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#284)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#206)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#180)
*   [examples/app/log\_layers\_ecs.rs](../../src/log_layers_ecs/log_layers_ecs.rs.html#135)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#652)
*   [examples/camera/pan\_camera\_controller.rs](../../src/pan_camera_controller/pan_camera_controller.rs.html#27)
*   [examples/camera/2d\_top\_down\_camera.rs](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#56)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#216)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#181)
*   [examples/audio/play\_sound\_effect.rs](../../src/play_sound_effect/play_sound_effect.rs.html#37)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#91)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#92)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#104)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#145)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#92)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#102)
*   [examples/camera/first\_person\_view\_model.rs](../../src/first_person_view_model/first_person_view_model.rs.html#194)
*   [examples/window/custom\_cursor\_image.rs](../../src/custom_cursor_image/custom_cursor_image.rs.html#80)
*   [examples/ecs/state\_scoped.rs](../../src/state_scoped/state_scoped.rs.html#52)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#232)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#242)
*   [examples/ui/widgets/tab\_navigation.rs](../../src/tab_navigation/tab_navigation.rs.html#58)
*   [examples/ui/layout/ghost\_nodes.rs](../../src/ghost_nodes/ghost_nodes.rs.html#76)
*   [examples/dev\_tools/fps\_overlay.rs](../../src/fps_overlay/fps_overlay.rs.html#67)
*   [examples/gizmos/2d\_gizmos.rs](../../src/2d_gizmos/2d_gizmos.rs.html#34)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../src/scrollbars/scrollbars.rs.html#28)
*   [examples/ui/window\_fallthrough.rs](../../src/window_fallthrough/window_fallthrough.rs.html#40)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#22)
*   [examples/audio/audio\_control.rs](../../src/audio_control/audio_control.rs.html#26)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#155)
*   [examples/ecs/entity\_disabling.rs](../../src/entity_disabling/entity_disabling.rs.html#79)
*   [examples/ui/text/font\_atlas\_debug.rs](../../src/font_atlas_debug/font_atlas_debug.rs.html#58)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#44)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#144)
*   [examples/window/scale\_factor\_override.rs](../../src/scale_factor_override/scale_factor_override.rs.html#39)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#66)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#93)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#266)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#92)
*   [examples/ui/text/editable\_text\_filter.rs](../../src/editable_text_filter/editable_text_filter.rs.html#30)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#96)
*   [examples/ui/images/image\_node.rs](../../src/image_node/image_node.rs.html#33)
*   [examples/ui/layout/flex\_layout.rs](../../src/flex_layout/flex_layout.rs.html#136)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#533)
*   [examples/window/screenshot.rs](../../src/screenshot/screenshot.rs.html#84)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#109)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#374)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#94)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#258)
*   [examples/state/states.rs](../../src/states/states.rs.html#71)
*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#89)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#95)
*   [examples/ui/widgets/feathers\_counter.rs](../../src/feathers_counter/feathers_counter.rs.html#69)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#305)
*   [examples/stress\_tests/many\_glyphs.rs](../../src/many_glyphs/many_glyphs.rs.html#89)
*   [examples/ui/widgets/virtual\_keyboard.rs](../../src/virtual_keyboard/virtual_keyboard.rs.html#55)
*   [examples/state/sub\_states.rs](../../src/sub_states/sub_states.rs.html#171)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#193)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#55)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#62)
*   [examples/2d/wireframe\_2d.rs](../../src/wireframe_2d/wireframe_2d.rs.html#93)
*   [examples/usage/context\_menu.rs](../../src/context_menu/context_menu.rs.html#94)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#69)
*   [examples/ui/relative\_cursor\_position.rs](../../src/relative_cursor_position/relative_cursor_position.rs.html#40)
*   [examples/ui/ui\_material.rs](../../src/ui_material/ui_material.rs.html#41)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#90)
*   [examples/ui/text/font\_weights.rs](../../src/font_weights/font_weights.rs.html#38)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#86)
*   [examples/window/multiple\_windows.rs](../../src/multiple_windows/multiple_windows.rs.html#49)
*   [examples/ui/ui\_scaling.rs](../../src/ui_scaling/ui_scaling.rs.html#51)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#263)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#158)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#37)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#150)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../src/ui_texture_atlas/ui_texture_atlas.rs.html#41)
*   [examples/2d/bloom\_2d.rs](../../src/bloom_2d/bloom_2d.rs.html#63)
*   [examples/ui/text/font\_variations.rs](../../src/font_variations/font_variations.rs.html#39)
*   [examples/ecs/observers.rs](../../src/observers/observers.rs.html#108)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#83)
*   [examples/animation/animated\_ui.rs](../../src/animated_ui/animated_ui.rs.html#149)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#93)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#82)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#71)
*   [examples/ui/text/text\_input.rs](../../src/text_input/text_input.rs.html#54)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#64)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#94)
*   [examples/ui/images/ui\_texture\_slice.rs](../../src/ui_texture_slice/ui_texture_slice.rs.html#74)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#126)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#77)
*   [examples/camera/2d\_on\_ui.rs](../../src/2d_on_ui/2d_on_ui.rs.html#45)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#179)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#97)
*   [examples/math/cubic\_splines.rs](../../src/cubic_splines/cubic_splines.rs.html#83)
*   [examples/ui/text/ime\_support.rs](../../src/ime_support/ime_support.rs.html#46)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#129)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#164)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#43)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#85)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#46)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#124)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#88)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#78)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#490)
*   [examples/ui/text/text\_background\_colors.rs](../../src/text_background_colors/text_background_colors.rs.html#45)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#186)
*   [examples/ui/text/generic\_font\_families.rs](../../src/generic_font_families/generic_font_families.rs.html#42)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#92)
*   [examples/picking/dragdrop\_picking.rs](../../src/dragdrop_picking/dragdrop_picking.rs.html#52)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#108)
*   [examples/ui/styling/transparency\_ui.rs](../../src/transparency_ui/transparency_ui.rs.html#32)
*   [examples/ui/layout/anchor\_layout.rs](../../src/anchor_layout/anchor_layout.rs.html#27)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#123)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#91)
*   [examples/showcase/stepping.rs](../../src/breakout/stepping.rs.html#177)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#346)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#190)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#43)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#174)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#130)
*   [examples/ui/text/system\_fonts.rs](../../src/system_fonts/system_fonts.rs.html#36)
*   [examples/3d/pbr.rs](../../src/pbr/pbr.rs.html#70)
*   [examples/ui/ui\_target\_camera.rs](../../src/ui_target_camera/ui_target_camera.rs.html#26)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#266)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#131)
*   [examples/animation/easing\_functions.rs](../../src/easing_functions/easing_functions.rs.html#117)
*   [examples/window/multi\_window\_text.rs](../../src/multi_window_text/multi_window_text.rs.html#57)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../src/overflow/overflow.rs.html#42)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#24)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#125)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#236)
*   [examples/shader/shader\_prepass.rs](../../src/shader_prepass/shader_prepass.rs.html#127)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#48)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#263)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#101)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#101)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#97)
*   [examples/2d/2d\_shapes.rs](../../src/2d_shapes/2d_shapes.rs.html#154)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../src/scroll/scroll.rs.html#151)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#152)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#174)
*   [examples/ui/layout/z\_index.rs](../../src/z_index/z_index.rs.html#37)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#98)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#95)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#29)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#304)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#138)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#62)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#38)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#189)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#246)
*   [examples/ui/text/text.rs](../../src/text/text.rs.html#50)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#218)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#157)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#174)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#218)
*   [examples/ui/styling/borders.rs](../../src/borders/borders.rs.html#39)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#188)
*   [examples/ui/text/text\_debug.rs](../../src/text_debug/text_debug.rs.html#53)
*   [examples/ui/layout/grid.rs](../../src/grid/grid.rs.html#56)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#330)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#120)
*   [examples/ui/text/font\_query.rs](../../src/font_query/font_query.rs.html#25)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#45)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#33)
*   [examples/ui/widgets/feathers\_gallery.rs](../../src/feathers_gallery/feathers_gallery.rs.html#425)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#49)