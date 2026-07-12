[bevy](../index.html)::[animation](index.html)

# Struct ActiveAnimation 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#509)

```rust
pub struct ActiveAnimation { /* private fields */ }
```

An animation that an [`AnimationPlayer`](../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer") is currently either playing or was playing, but is presently paused.

A stopped animation is considered no longer active.

## Implementations

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#548)

### impl [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#553)

#### pub fn [is\_finished](#method.is_finished)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Check if the animation has finished, based on its repetition behavior and the number of times it has repeated.

Note: An animation with `RepeatAnimation::Forever` will never finish.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#595)

#### pub fn [replay](#method.replay)(&mut self)

Reset back to the initial state as if no time has elapsed.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/animation/animated\_mesh\_control.rs ([line 225](../../src/animated_mesh_control/animated_mesh_control.rs.html#225))

```rust
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#604)

#### pub fn [weight](#method.weight)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the current weight of this animation.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#609)

#### pub fn [set\_weight](#method.set_weight)(&mut self, weight: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> &mut [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Sets the weight of this animation.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/animation/animation\_graph.rs ([line 468](../../src/animation_graph/animation_graph.rs.html#468))

```rust
453fn sync_weights(mut query: Query<(&mut AnimationPlayer, &ExampleAnimationWeights)>) {
454    for (mut animation_player, animation_weights) in query.iter_mut() {
455        for (&animation_node_index, &animation_weight) in CLIP_NODE_INDICES
456            .iter()
457            .zip(animation_weights.weights.iter())
458        {
459            // If the animation happens to be no longer active, restart it.
460            if !animation_player.is_playing_animation(animation_node_index.into()) {
461                animation_player.play(animation_node_index.into());
462            }
463
464            // Set the weight.
465            if let Some(active_animation) =
466                animation_player.animation_mut(animation_node_index.into())
467            {
468                active_animation.set_weight(animation_weight);
469            }
470        }
471    }
472}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#615)

#### pub fn [pause](#method.pause)(&mut self) -> &mut [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Pause the animation.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/testbed/3d.rs ([line 357](../../src/testbed_3d/3d.rs.html#357))

```rust
344    fn pause_animation_frame(
345        scene_ready: On<WorldInstanceReady>,
346        children: Query<&Children>,
347        mut commands: Commands,
348        animation: Res<Animation>,
349        mut players: Query<(Entity, &mut AnimationPlayer)>,
350    ) {
351        for child in children.iter_descendants(scene_ready.entity) {
352            if let Ok((entity, mut player)) = players.get_mut(child) {
353                let mut transitions = AnimationTransitions::new();
354                transitions
355                    .play(&mut player, animation.animation, Duration::ZERO)
356                    .seek_to(0.5)
357                    .pause();
358
359                commands
360                    .entity(entity)
361                    .insert(AnimationGraphHandle(animation.graph.clone()))
362                    .insert(transitions);
363            }
364        }
365    }
```

Hide additional examples

examples/animation/animated\_mesh\_control.rs ([line 181](../../src/animated_mesh_control/animated_mesh_control.rs.html#181))

```rust
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#621)

#### pub fn [resume](#method.resume)(&mut self) -> &mut [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Unpause the animation.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/animation/animated\_mesh\_control.rs ([line 179](../../src/animated_mesh_control/animated_mesh_control.rs.html#179))

```rust
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#630)

#### pub fn [is\_paused](#method.is_paused)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if this animation is currently paused.

Note that paused animations are still [`ActiveAnimation`](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")s.

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/animation/animated\_mesh\_control.rs ([line 178](../../src/animated_mesh_control/animated_mesh_control.rs.html#178))

```rust
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#635)

#### pub fn [set\_repeat](#method.set_repeat)(&mut self, repeat: [RepeatAnimation](enum.RepeatAnimation.html "enum bevy::animation::RepeatAnimation")) -> &mut [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Sets the repeat mode for this playing animation.

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/animation/animated\_mesh\_control.rs ([line 224](../../src/animated_mesh_control/animated_mesh_control.rs.html#224))

```rust
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#641)

#### pub fn [repeat](#method.repeat)(&mut self) -> &mut [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Marks this animation as repeating forever.

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/3d/irradiance\_volumes.rs ([line 524](../../src/irradiance_volumes/irradiance_volumes.rs.html#524))

```rust
515fn play_animations(
516    mut commands: Commands,
517    assets: Res<ExampleAssets>,
518    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
519) {
520    for (entity, mut player) in players.iter_mut() {
521        commands
522            .entity(entity)
523            .insert(AnimationGraphHandle(assets.fox_animation_graph.clone()));
524        player.play(assets.fox_animation_node).repeat();
525    }
526}
```

Hide additional examples

examples/animation/animation\_graph.rs ([line 395](../../src/animation_graph/animation_graph.rs.html#395))

```rust
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
```

examples/animation/morph\_targets.rs ([line 68](../../src/morph_targets/morph_targets.rs.html#68))

```rust
58fn play_animation_when_ready(
59    scene_ready: On<WorldInstanceReady>,
60    mut commands: Commands,
61    children: Query<&Children>,
62    animations_to_play: Query<&AnimationToPlay>,
63    mut players: Query<&mut AnimationPlayer>,
64) {
65    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
66        for child in children.iter_descendants(scene_ready.entity) {
67            if let Ok(mut player) = players.get_mut(child) {
68                player.play(animation_to_play.index).repeat();
69
70                commands
71                    .entity(child)
72                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
73            }
74        }
75    }
76}
```

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 107](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#107))

```rust
97fn play_animation(
98    trigger: On<WorldInstanceReady>,
99    mut commands: Commands,
100    children: Query<&Children>,
101    animations: Query<&PendingAnimation>,
102    mut players: Query<&mut AnimationPlayer>,
103) {
104    if let Ok(PendingAnimation((graph_handle, graph_node_index))) = animations.get(trigger.entity) {
105        for child in children.iter_descendants(trigger.entity) {
106            if let Ok(mut player) = players.get_mut(child) {
107                player.play(*graph_node_index).set_speed(0.6).repeat();
108
109                commands
110                    .entity(child)
111                    .insert(AnimationGraphHandle(graph_handle.clone()));
112            }
113        }
114    }
115
116    commands.entity(trigger.entity).remove::<PendingAnimation>();
117}
```

examples/stress\_tests/many\_foxes.rs ([line 261](../../src/many_foxes/many_foxes.rs.html#261))

```rust
251fn setup_scene_once_loaded(
252    scene_ready: On<WorldInstanceReady>,
253    animations: Res<Animations>,
254    foxes: Res<Foxes>,
255    mut commands: Commands,
256    children: Query<&Children>,
257    mut players: Query<&mut AnimationPlayer>,
258) {
259    for child in children.iter_descendants(scene_ready.entity) {
260        if let Ok(mut player) = players.get_mut(child) {
261            let playing_animation = player.play(animations.node_indices[0]).repeat();
262            if !foxes.sync {
263                playing_animation.seek_to(scene_ready.entity.index_u32() as f32 / 10.0);
264            }
265            commands.entity(child).insert((
266                AnimationGraphHandle(animations.graph.clone()),
267                AnimationTransitions::default(),
268            ));
269        }
270    }
271}
272
273fn update_fox_rings(
274    time: Res<Time>,
275    foxes: Res<Foxes>,
276    mut rings: Query<(&Ring, &RotationDirection, &mut Transform)>,
277) {
278    if !foxes.moving {
279        return;
280    }
281
282    let dt = time.delta_secs();
283    for (ring, rotation_direction, mut transform) in &mut rings {
284        let angular_velocity = foxes.speed / ring.radius;
285        transform.rotate_y(rotation_direction.sign() * angular_velocity * dt);
286    }
287}
288
289fn keyboard_animation_control(
290    keyboard_input: Res<ButtonInput<KeyCode>>,
291    mut animation_player: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
292    animations: Res<Animations>,
293    mut current_animation: Local<usize>,
294    mut foxes: ResMut<Foxes>,
295) {
296    if keyboard_input.just_pressed(KeyCode::Space) {
297        foxes.moving = !foxes.moving;
298    }
299
300    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
301        foxes.speed *= 1.25;
302    }
303
304    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
305        foxes.speed *= 0.8;
306    }
307
308    if keyboard_input.just_pressed(KeyCode::Enter) {
309        *current_animation = (*current_animation + 1) % animations.node_indices.len();
310    }
311
312    for (mut player, mut transitions) in &mut animation_player {
313        if keyboard_input.just_pressed(KeyCode::Space) {
314            if player.all_paused() {
315                player.resume_all();
316            } else {
317                player.pause_all();
318            }
319        }
320
321        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
322            player.adjust_speeds(1.25);
323        }
324
325        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
326            player.adjust_speeds(0.8);
327        }
328
329        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
330            player.seek_all_by(-0.1);
331        }
332
333        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
334            player.seek_all_by(0.1);
335        }
336
337        if keyboard_input.just_pressed(KeyCode::Enter) {
338            transitions
339                .play(
340                    &mut player,
341                    animations.node_indices[*current_animation],
342                    Duration::from_millis(250),
343                )
344                .repeat();
345        }
346    }
347}
```

examples/animation/animated\_mesh\_control.rs ([line 157](../../src/animated_mesh_control/animated_mesh_control.rs.html#157))

```rust
142fn setup_scene(
143    _ready: On<WorldInstanceReady>,
144    mut commands: Commands,
145    animations: Res<Animations>,
146    player: Single<(Entity, &mut AnimationPlayer)>,
147) {
148    let (entity, mut player) = player.into_inner();
149    let mut transitions = AnimationTransitions::new();
150
151    // Make sure to start the animation via the `AnimationTransitions`
152    // component. The `AnimationTransitions` component wants to manage all
153    // the animations and will get confused if the animations are started
154    // directly via the `AnimationPlayer`.
155    transitions
156        .play(&mut player, animations.animations[0], Duration::ZERO)
157        .repeat();
158
159    commands
160        .entity(entity)
161        .insert(AnimationGraphHandle(animations.graph_handle.clone()))
162        .insert(transitions);
163}
164
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

Additional examples can be found in:  

*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#385)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#78)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#642)
*   [examples/animation/animation\_events.rs](../../src/animation_events/animation_events.rs.html#92)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#88)
*   [examples/animation/animated\_ui.rs](../../src/animated_ui/animated_ui.rs.html#137)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#36)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#185)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#402)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#140)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#646)

#### pub fn [repeat\_mode](#method.repeat_mode)(&self) -> [RepeatAnimation](enum.RepeatAnimation.html "enum bevy::animation::RepeatAnimation")

Returns the repeat mode assigned to this active animation.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#651)

#### pub fn [completions](#method.completions)(&self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the number of times this animation has completed.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#656)

#### pub fn [is\_playback\_reversed](#method.is_playback_reversed)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the animation is playing in reverse.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#661)

#### pub fn [speed](#method.speed)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the speed of the animation playback.

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/animation/animated\_mesh\_control.rs ([line 187](../../src/animated_mesh_control/animated_mesh_control.rs.html#187))

```rust
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#666)

#### pub fn [set\_speed](#method.set_speed)(&mut self, speed: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> &mut [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Sets the speed of the animation playback.

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 107](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#107))

```rust
97fn play_animation(
98    trigger: On<WorldInstanceReady>,
99    mut commands: Commands,
100    children: Query<&Children>,
101    animations: Query<&PendingAnimation>,
102    mut players: Query<&mut AnimationPlayer>,
103) {
104    if let Ok(PendingAnimation((graph_handle, graph_node_index))) = animations.get(trigger.entity) {
105        for child in children.iter_descendants(trigger.entity) {
106            if let Ok(mut player) = players.get_mut(child) {
107                player.play(*graph_node_index).set_speed(0.6).repeat();
108
109                commands
110                    .entity(child)
111                    .insert(AnimationGraphHandle(graph_handle.clone()));
112            }
113        }
114    }
115
116    commands.entity(trigger.entity).remove::<PendingAnimation>();
117}
```

Hide additional examples

examples/stress\_tests/many\_morph\_targets.rs ([line 386](../../src/many_morph_targets/many_morph_targets.rs.html#386))

```rust
366fn play_animation(
367    trigger: On<WorldInstanceReady>,
368    mut commands: Commands,
369    args: Res<Args>,
370    children: Query<&Children>,
371    animations_to_play: Query<&AnimationToPlay>,
372    mut players: Query<&mut AnimationPlayer>,
373) {
374    if args.weights == ArgWeights::Animated
375        && let Ok(animation_to_play) = animations_to_play.get(trigger.entity)
376    {
377        for child in children.iter_descendants(trigger.entity) {
378            if let Ok(mut player) = players.get_mut(child) {
379                commands
380                    .entity(child)
381                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
382
383                player
384                    .play(animation_to_play.index)
385                    .repeat()
386                    .set_speed(animation_to_play.speed);
387            }
388        }
389    }
390}
```

examples/animation/animated\_mesh\_control.rs ([line 188](../../src/animated_mesh_control/animated_mesh_control.rs.html#188))

```rust
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#672)

#### pub fn [elapsed](#method.elapsed)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the amount of time the animation has been playing.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#677)

#### pub fn [last\_seek\_time](#method.last_seek_time)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>

Returns the last seek time of the animation.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#682)

#### pub fn [just\_completed](#method.just_completed)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the animation was completed at least once this tick.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#689)

#### pub fn [seek\_time](#method.seek_time)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the seek time of the animation.

This is nonnegative and no more than the clip duration.

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/animation/animated\_mesh\_control.rs ([line 199](../../src/animated_mesh_control/animated_mesh_control.rs.html#199))

```rust
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#697)

#### pub fn [set\_seek\_time](#method.set_seek_time)(&mut self, seek\_time: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> &mut [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Seeks to a specific time in the animation.

This will not trigger events between the current time and `seek_time`. Use [`seek_to`](struct.ActiveAnimation.html#method.seek_to "method bevy::animation::ActiveAnimation::seek_to") if this is desired.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#708)

#### pub fn [seek\_to](#method.seek_to)(&mut self, seek\_time: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> &mut [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Seeks to a specific time in the animation.

Note that any events between the current time and `seek_time` will be triggered on the next update. Use [`set_seek_time`](struct.ActiveAnimation.html#method.set_seek_time "method bevy::animation::ActiveAnimation::set_seek_time") if this is undesired.

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_foxes.rs ([line 263](../../src/many_foxes/many_foxes.rs.html#263))

```rust
251fn setup_scene_once_loaded(
252    scene_ready: On<WorldInstanceReady>,
253    animations: Res<Animations>,
254    foxes: Res<Foxes>,
255    mut commands: Commands,
256    children: Query<&Children>,
257    mut players: Query<&mut AnimationPlayer>,
258) {
259    for child in children.iter_descendants(scene_ready.entity) {
260        if let Ok(mut player) = players.get_mut(child) {
261            let playing_animation = player.play(animations.node_indices[0]).repeat();
262            if !foxes.sync {
263                playing_animation.seek_to(scene_ready.entity.index_u32() as f32 / 10.0);
264            }
265            commands.entity(child).insert((
266                AnimationGraphHandle(animations.graph.clone()),
267                AnimationTransitions::default(),
268            ));
269        }
270    }
271}
```

Hide additional examples

examples/testbed/3d.rs ([line 356](../../src/testbed_3d/3d.rs.html#356))

```rust
344    fn pause_animation_frame(
345        scene_ready: On<WorldInstanceReady>,
346        children: Query<&Children>,
347        mut commands: Commands,
348        animation: Res<Animation>,
349        mut players: Query<(Entity, &mut AnimationPlayer)>,
350    ) {
351        for child in children.iter_descendants(scene_ready.entity) {
352            if let Ok((entity, mut player)) = players.get_mut(child) {
353                let mut transitions = AnimationTransitions::new();
354                transitions
355                    .play(&mut player, animation.animation, Duration::ZERO)
356                    .seek_to(0.5)
357                    .pause();
358
359                commands
360                    .entity(entity)
361                    .insert(AnimationGraphHandle(animation.graph.clone()))
362                    .insert(transitions);
363            }
364        }
365    }
```

examples/animation/animated\_mesh\_control.rs ([line 200](../../src/animated_mesh_control/animated_mesh_control.rs.html#200))

```rust
165fn keyboard_control(
166    keyboard_input: Res<ButtonInput<KeyCode>>,
167    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
168    animations: Res<Animations>,
169    mut current_animation: Local<usize>,
170) {
171    for (mut player, mut transitions) in &mut animation_players {
172        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
173            continue;
174        };
175
176        if keyboard_input.just_pressed(KeyCode::Space) {
177            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
178            if playing_animation.is_paused() {
179                playing_animation.resume();
180            } else {
181                playing_animation.pause();
182            }
183        }
184
185        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
186            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
187            let speed = playing_animation.speed();
188            playing_animation.set_speed(speed * 1.2);
189        }
190
191        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
192            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
193            let speed = playing_animation.speed();
194            playing_animation.set_speed(speed * 0.8);
195        }
196
197        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
198            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
199            let elapsed = playing_animation.seek_time();
200            playing_animation.seek_to(elapsed - 0.1);
201        }
202
203        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
204            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
205            let elapsed = playing_animation.seek_time();
206            playing_animation.seek_to(elapsed + 0.1);
207        }
208
209        if keyboard_input.just_pressed(KeyCode::Enter) {
210            *current_animation = (*current_animation + 1) % animations.animations.len();
211
212            transitions
213                .play(
214                    &mut player,
215                    animations.animations[*current_animation],
216                    Duration::from_millis(250),
217                )
218                .repeat();
219        }
220
221        if keyboard_input.just_pressed(KeyCode::Digit1) {
222            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
223            playing_animation
224                .set_repeat(RepeatAnimation::Count(1))
225                .replay();
226        }
227
228        if keyboard_input.just_pressed(KeyCode::Digit2) {
229            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
230            playing_animation
231                .set_repeat(RepeatAnimation::Count(2))
232                .replay();
233        }
234
235        if keyboard_input.just_pressed(KeyCode::Digit3) {
236            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
237            playing_animation
238                .set_repeat(RepeatAnimation::Count(3))
239                .replay();
240        }
241
242        if keyboard_input.just_pressed(KeyCode::KeyL) {
243            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
244            playing_animation.set_repeat(RepeatAnimation::Forever);
245        }
246    }
247}
```

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#719)

#### pub fn [rewind](#method.rewind)(&mut self) -> &mut [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Seeks to the beginning of the animation.

Note that any events between the current time and `0.0` will be triggered on the next update. Use [`set_seek_time`](struct.ActiveAnimation.html#method.set_seek_time "method bevy::animation::ActiveAnimation::set_seek_time") if this is undesired.

## Trait Implementations

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#532)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#533)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)( arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [from\_reflect](../prelude/trait.FromReflect.html#tymethod.from_reflect)( reflect: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [get\_represented\_type\_info](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [try\_apply](../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [reflect\_kind](../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [reflect\_ref](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [reflect\_owned](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [try\_into\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [try\_as\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [try\_as\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [into\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [as\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [as\_partial\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [reflect\_partial\_eq](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [reflect\_partial\_cmp](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#508)

#### fn [reflect\_clone](../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#363)

#### fn [debug](../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [into\_any](../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [as\_any](../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [as\_any\_mut](../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [into\_reflect](../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [as\_reflect](../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [as\_reflect\_mut](../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [set](../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [field](../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [field\_mut](../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [field\_at](../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [field\_at\_mut](../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [name\_at](../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [index\_of\_name](../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [field\_len](../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [iter\_fields](../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [to\_dynamic\_struct](../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

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

### impl<T> [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](../prelude/trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#273)

#### fn [get\_field](../prelude/trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#278)

#### fn [get\_field\_mut](../prelude/trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](../prelude/trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](../prelude/trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](../prelude/trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](../prelude/trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.path_mut)

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

### impl<G> [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#method.clone_into)

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

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}