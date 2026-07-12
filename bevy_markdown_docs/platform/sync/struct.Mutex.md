[bevy](../../index.html)::[platform](../index.html)::[sync](index.html)

# Struct Mutex 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#227)

```rust
pub struct Mutex<T>where
    T: ?Sized,{ /* private fields */ }
```

A mutual exclusion primitive useful for protecting shared data

This mutex will block threads waiting for the lock to become available. The mutex can be created via a [`new`](struct.Mutex.html#method.new "associated function bevy::platform::sync::Mutex::new") constructor. Each mutex has a type parameter which represents the data that it is protecting. The data can only be accessed through the RAII guards returned from [`lock`](struct.Mutex.html#method.lock "method bevy::platform::sync::Mutex::lock") and [`try_lock`](struct.Mutex.html#method.try_lock "method bevy::platform::sync::Mutex::try_lock"), which guarantees that the data is only ever accessed when the mutex is locked.

## Poisoning

The mutexes in this module implement a strategy called “poisoning” where a mutex becomes poisoned if it recognizes that the thread holding it has panicked.

Once a mutex is poisoned, all other threads are unable to access the data by default as it is likely tainted (some invariant is not being upheld). For a mutex, this means that the [`lock`](struct.Mutex.html#method.lock "method bevy::platform::sync::Mutex::lock") and [`try_lock`](struct.Mutex.html#method.try_lock "method bevy::platform::sync::Mutex::try_lock") methods return a [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") which indicates whether a mutex has been poisoned or not. Most usage of a mutex will simply [`unwrap()`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.unwrap "method core::result::Result::unwrap") these results, propagating panics among threads to ensure that a possibly invalid invariant is not witnessed.

Poisoning is only advisory: the [`PoisonError`](struct.PoisonError.html "struct bevy::platform::sync::PoisonError") type has an [`into_inner`](struct.PoisonError.html#method.into_inner "method bevy::platform::sync::PoisonError::into_inner") method which will return the guard that would have otherwise been returned on a successful lock. This allows access to the data, despite the lock being poisoned.

In addition, the panic detection is not ideal, so even unpoisoned mutexes need to be handled with care, since certain panics may have been skipped. Here is a non-exhaustive list of situations where this might occur:

*   If a mutex is locked while a panic is underway, e.g. within a [`Drop`](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop") implementation or a [panic hook](https://doc.rust-lang.org/nightly/std/panicking/fn.set_hook.html "fn std::panicking::set_hook"), panicking for the second time while the lock is held will leave the mutex unpoisoned. Note that while double panic usually aborts the program, [`catch_unwind`](https://doc.rust-lang.org/nightly/std/panic/fn.catch_unwind.html "fn std::panic::catch_unwind") can prevent this.
    
*   Locking and unlocking the mutex across different panic contexts, e.g. by storing the guard to a [`Cell`](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html "struct core::cell::Cell") within [`Drop::drop`](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#method.drop "method core::ops::drop::Drop::drop") and accessing it outside, or vice versa, can affect poisoning status in an unexpected way.
    
*   Foreign exceptions do not currently trigger poisoning even in absence of other panics.
    

While this rarely happens in realistic code, `unsafe` code cannot rely on poisoning for soundness, since the behavior of poisoning can depend on outside context. Here’s an example of **incorrect** use of poisoning:

```rust
use std::sync::Mutex;

struct MutexBox<T> {
    data: Mutex<*mut T>,
}

impl<T> MutexBox<T> {
    pub fn new(value: T) -> Self {
        Self {
            data: Mutex::new(Box::into_raw(Box::new(value))),
        }
    }

    pub fn replace_with(&self, f: impl FnOnce(T) -> T) {
        let ptr = self.data.lock().expect("poisoned");
        // While `f` is running, the data is moved out of `*ptr`. If `f`
        // panics, `*ptr` keeps pointing at a dropped value. The intention
        // is that this will poison the mutex, so the following calls to
        // `replace_with` will panic without reading `*ptr`. But since
        // poisoning is not guaranteed to occur if this is run from a panic
        // hook, this can lead to use-after-free.
        unsafe {
            (*ptr).write(f((*ptr).read()));
        }
    }
}
```

## Examples

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

const N: usize = 10;

// Spawn a few threads to increment a shared variable (non-atomically), and
// let the main thread know once all increments are done.
//
// Here we're using an Arc to share memory among threads, and the data inside
// the Arc is protected with a mutex.
let data = Arc::new(Mutex::new(0));

let (tx, rx) = channel();
for _ in 0..N {
    let (data, tx) = (Arc::clone(&data), tx.clone());
    thread::spawn(move || {
        // The shared state can only be accessed once the lock is held.
        // Our non-atomic increment is safe because we're the only thread
        // which can access the shared state when the lock is held.
        //
        // We unwrap() the return value to assert that we are not expecting
        // threads to ever fail while holding the lock.
        let mut data = data.lock().unwrap();
        *data += 1;
        if *data == N {
            tx.send(()).unwrap();
        }
        // the lock is unlocked here when `data` goes out of scope.
    });
}

rx.recv().unwrap();
```

To recover from a poisoned mutex:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let lock = Arc::new(Mutex::new(0_u32));
let lock2 = Arc::clone(&lock);

let _ = thread::spawn(move || -> () {
    // This thread will acquire the mutex first, unwrapping the result of
    // `lock` because the lock has not been poisoned.
    let _guard = lock2.lock().unwrap();

    // This panic while holding the lock (`_guard` is in scope) will poison
    // the mutex.
    panic!();
}).join();

// The lock is poisoned by this point, but the returned result can be
// pattern matched on to return the underlying guard on both branches.
let mut guard = match lock.lock() {
    Ok(guard) => guard,
    Err(poisoned) => poisoned.into_inner(),
};

*guard += 1;
```

To unlock a mutex guard sooner than the end of the enclosing scope, either create an inner scope or drop the guard manually.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

const N: usize = 3;

let data_mutex = Arc::new(Mutex::new(vec![1, 2, 3, 4]));
let res_mutex = Arc::new(Mutex::new(0));

let mut threads = Vec::with_capacity(N);
(0..N).for_each(|_| {
    let data_mutex_clone = Arc::clone(&data_mutex);
    let res_mutex_clone = Arc::clone(&res_mutex);

    threads.push(thread::spawn(move || {
        // Here we use a block to limit the lifetime of the lock guard.
        let result = {
            let mut data = data_mutex_clone.lock().unwrap();
            // This is the result of some important and long-ish work.
            let result = data.iter().fold(0, |acc, x| acc + x * 2);
            data.push(result);
            result
            // The mutex guard gets dropped here, together with any other values
            // created in the critical section.
        };
        // The guard created here is a temporary dropped at the end of the statement, i.e.
        // the lock would not remain being held even if the thread did some additional work.
        *res_mutex_clone.lock().unwrap() += result;
    }));
});

let mut data = data_mutex.lock().unwrap();
// This is the result of some important and long-ish work.
let result = data.iter().fold(0, |acc, x| acc + x * 2);
data.push(result);
// We drop the `data` explicitly because it's not necessary anymore and the
// thread still has work to do. This allows other threads to start working on
// the data immediately, without waiting for the rest of the unrelated work
// to be done here.
//
// It's even more important here than in the threads because we `.join` the
// threads after that. If we had not dropped the mutex guard, a thread could
// be waiting forever for it, causing a deadlock.
// As in the threads, a block could have been used instead of calling the
// `drop` function.
drop(data);
// Here the mutex guard is not assigned to a variable and so, even if the
// scope does not end after this line, the mutex is still released: there is
// no deadlock.
*res_mutex.lock().unwrap() += result;

threads.into_iter().for_each(|thread| {
    thread
        .join()
        .expect("The thread creating or execution failed !")
});

assert_eq!(*res_mutex.lock().unwrap(), 800);
```

## Implementations

[Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#337)

### impl<T> [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

1.0.0 (const: 1.63.0) · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#350)

#### pub const fn [new](#method.new)(t: T) -> [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

Creates a new mutex in an unlocked state ready for use.

##### Examples

```rust
use std::sync::Mutex;

let mutex = Mutex::new(0);
```

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/3d/occlusion\_culling.rs ([line 124](../../../src/occlusion_culling/occlusion_culling.rs.html#124))

```rust
123    fn new() -> Self {
124        Self(Arc::new(Mutex::new(None)))
125    }
```

[Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#373-375)

#### pub fn [get\_cloned](#method.get_cloned)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [PoisonError](struct.PoisonError.html "struct bevy::platform::sync::PoisonError")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

🔬This is a nightly-only experimental API. (`lock_value_accessors`)

Returns the contained value by cloning it.

##### Errors

If another user of this mutex panicked while holding the mutex, then this call will return an error instead.

##### Examples

```rust
#![feature(lock_value_accessors)]

use std::sync::Mutex;

let mut mutex = Mutex::new(7);

assert_eq!(mutex.get_cloned().unwrap(), 7);
```

[Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#405)

#### pub fn [set](#method.set)(&self, value: T) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [PoisonError](struct.PoisonError.html "struct bevy::platform::sync::PoisonError")<T>>

🔬This is a nightly-only experimental API. (`lock_value_accessors`)

Sets the contained value.

##### Errors

If another user of this mutex panicked while holding the mutex, then this call will return an error containing the provided `value` instead.

##### Examples

```rust
#![feature(lock_value_accessors)]

use std::sync::Mutex;

let mut mutex = Mutex::new(7);

assert_eq!(mutex.get_cloned().unwrap(), 7);
mutex.set(11).unwrap();
assert_eq!(mutex.get_cloned().unwrap(), 11);
```

[Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#443)

#### pub fn [replace](#method.replace)(&self, value: T) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [PoisonError](struct.PoisonError.html "struct bevy::platform::sync::PoisonError")<T>>

🔬This is a nightly-only experimental API. (`lock_value_accessors`)

Replaces the contained value with `value`, and returns the old contained value.

##### Errors

If another user of this mutex panicked while holding the mutex, then this call will return an error containing the provided `value` instead.

##### Examples

```rust
#![feature(lock_value_accessors)]

use std::sync::Mutex;

let mut mutex = Mutex::new(7);

assert_eq!(mutex.replace(11).unwrap(), 7);
assert_eq!(mutex.get_cloned().unwrap(), 11);
```

[Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#451)

### impl<T> [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#490)

#### pub fn [lock](#method.lock)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[MutexGuard](struct.MutexGuard.html "struct bevy::platform::sync::MutexGuard")<'\_, T>, [PoisonError](struct.PoisonError.html "struct bevy::platform::sync::PoisonError")<[MutexGuard](struct.MutexGuard.html "struct bevy::platform::sync::MutexGuard")<'\_, T>>>

Acquires a mutex, blocking the current thread until it is able to do so.

This function will block the local thread until it is available to acquire the mutex. Upon returning, the thread is the only thread with the lock held. An RAII guard is returned to allow scoped unlock of the lock. When the guard goes out of scope, the mutex will be unlocked.

The exact behavior on locking a mutex in the thread which already holds the lock is left unspecified. However, this function will not return on the second call (it might panic or deadlock, for example).

##### Errors

If another user of this mutex panicked while holding the mutex, then this call will return an error once the mutex is acquired. The acquired mutex guard will be contained in the returned error.

##### Panics

This function might panic when called if the lock is already held by the current thread.

##### Examples

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let mutex = Arc::new(Mutex::new(0));
let c_mutex = Arc::clone(&mutex);

thread::spawn(move || {
    *c_mutex.lock().unwrap() = 10;
}).join().expect("thread::spawn failed");
assert_eq!(*mutex.lock().unwrap(), 10);
```

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/3d/occlusion\_culling.rs ([line 133](../../../src/occlusion_culling/occlusion_culling.rs.html#133))

```rust
128fn init_saved_indirect_parameters(
129    render_device: Res<RenderDevice>,
130    gpu_preprocessing_support: Res<GpuPreprocessingSupport>,
131    saved_indirect_parameters: Res<SavedIndirectParameters>,
132) {
133    let mut saved_indirect_parameters = saved_indirect_parameters.0.lock().unwrap();
134    *saved_indirect_parameters = Some(SavedIndirectParametersData {
135        data: vec![],
136        count: 0,
137        occlusion_culling_supported: gpu_preprocessing_support.is_culling_supported(),
138        // In order to determine how many meshes were culled, we look at the indirect count buffer
139        // that Bevy only populates if the platform supports `multi_draw_indirect_count`. So, if we
140        // don't have that feature, then we don't bother to display how many meshes were culled.
141        occlusion_culling_introspection_supported: render_device
142            .features()
143            .contains(WgpuFeatures::MULTI_DRAW_INDIRECT_COUNT),
144    });
145}
146
147/// The demo's current settings.
148#[derive(Resource)]
149struct AppStatus {
150    /// Whether occlusion culling is presently enabled.
151    ///
152    /// By default, this is set to true.
153    occlusion_culling: bool,
154}
155
156impl Default for AppStatus {
157    fn default() -> Self {
158        AppStatus {
159            occlusion_culling: true,
160        }
161    }
162}
163
164fn main() {
165    let render_debug_flags = RenderDebugFlags::ALLOW_COPIES_FROM_INDIRECT_PARAMETERS;
166
167    App::new()
168        .add_plugins(
169            DefaultPlugins
170                .set(WindowPlugin {
171                    primary_window: Some(Window {
172                        title: "Bevy Occlusion Culling Example".into(),
173                        ..default()
174                    }),
175                    ..default()
176                })
177                .set(RenderPlugin {
178                    debug_flags: render_debug_flags,
179                    ..default()
180                })
181                .set(PbrPlugin {
182                    debug_flags: render_debug_flags,
183                    ..default()
184                }),
185        )
186        .add_plugins(ReadbackIndirectParametersPlugin)
187        .init_resource::<AppStatus>()
188        .add_systems(Startup, setup)
189        .add_systems(Update, spin_small_cubes)
190        .add_systems(Update, spin_large_cube)
191        .add_systems(Update, update_status_text)
192        .add_systems(Update, toggle_occlusion_culling_on_request)
193        .run();
194}
195
196impl Plugin for ReadbackIndirectParametersPlugin {
197    fn build(&self, app: &mut App) {
198        // Create the `SavedIndirectParameters` resource that we're going to use
199        // to communicate between the thread that the GPU-to-CPU readback
200        // callback runs on and the main application threads. This resource is
201        // atomically reference counted. We store one reference to the
202        // `SavedIndirectParameters` in the main app and another reference in
203        // the render app.
204        let saved_indirect_parameters = SavedIndirectParameters::new();
205        app.insert_resource(saved_indirect_parameters.clone());
206
207        // Fetch the render app.
208        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
209            return;
210        };
211
212        render_app
213            // Insert another reference to the `SavedIndirectParameters`.
214            .insert_resource(saved_indirect_parameters)
215            // Setup the parameters in RenderStartup.
216            .add_systems(RenderStartup, init_saved_indirect_parameters)
217            .init_resource::<IndirectParametersStagingBuffers>()
218            .add_systems(ExtractSchedule, readback_indirect_parameters)
219            .add_systems(
220                Render,
221                create_indirect_parameters_staging_buffers
222                    .in_set(RenderSystems::PrepareResourcesFlush),
223            )
224            .add_systems(
225                Core3d,
226                // Add the node that allows us to read the indirect parameters back
227                // from the GPU to the CPU, which allows us to determine how many
228                // meshes were culled.
229                readback_indirect_parameters_node
230                    // We read back the indirect parameters any time after
231                    // `MainPass`. Readback doesn't particularly need to execute
232                    // before PostProcess, but we order it that way anyway.
233                    .after(Core3dSystems::MainPass)
234                    .before(Core3dSystems::PostProcess),
235            );
236    }
237}
238
239/// Spawns all the objects in the scene.
240fn setup(
241    mut commands: Commands,
242    asset_server: Res<AssetServer>,
243    mut meshes: ResMut<Assets<Mesh>>,
244    mut materials: ResMut<Assets<StandardMaterial>>,
245) {
246    spawn_small_cubes(&mut commands, &mut meshes, &mut materials);
247    spawn_large_cube(&mut commands, &asset_server, &mut meshes, &mut materials);
248    spawn_light(&mut commands);
249    spawn_camera(&mut commands);
250    spawn_help_text(&mut commands);
251}
252
253/// Spawns the rotating sphere of small cubes.
254fn spawn_small_cubes(
255    commands: &mut Commands,
256    meshes: &mut Assets<Mesh>,
257    materials: &mut Assets<StandardMaterial>,
258) {
259    // Add the cube mesh.
260    let small_cube = meshes.add(Cuboid::new(
261        SMALL_CUBE_SIZE,
262        SMALL_CUBE_SIZE,
263        SMALL_CUBE_SIZE,
264    ));
265
266    // Add the cube material.
267    let small_cube_material = materials.add(StandardMaterial {
268        base_color: SILVER.into(),
269        ..default()
270    });
271
272    // Create the entity that the small cubes will be parented to. This is the
273    // entity that we rotate.
274    let sphere_parent = commands
275        .spawn(Transform::from_translation(Vec3::ZERO))
276        .insert(Visibility::default())
277        .insert(SphereParent)
278        .id();
279
280    // Now we have to figure out where to place the cubes. To do that, we create
281    // a sphere mesh, but we don't add it to the scene. Instead, we inspect the
282    // sphere mesh to find the positions of its vertices, and spawn a small cube
283    // at each one. That way, we end up with a bunch of cubes arranged in a
284    // spherical shape.
285
286    // Create the sphere mesh, and extract the positions of its vertices.
287    let sphere = Sphere::new(OUTER_RADIUS)
288        .mesh()
289        .ico(OUTER_SUBDIVISION_COUNT)
290        .unwrap();
291    let sphere_positions = sphere.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
292
293    // At each vertex, create a small cube.
294    for sphere_position in sphere_positions.as_float3().unwrap() {
295        let sphere_position = Vec3::from_slice(sphere_position);
296        let small_cube = commands
297            .spawn(Mesh3d(small_cube.clone()))
298            .insert(MeshMaterial3d(small_cube_material.clone()))
299            .insert(Transform::from_translation(sphere_position))
300            .id();
301        commands.entity(sphere_parent).add_child(small_cube);
302    }
303}
304
305/// Spawns the large cube at the center of the screen.
306///
307/// This cube rotates chaotically and occludes small cubes behind it.
308fn spawn_large_cube(
309    commands: &mut Commands,
310    asset_server: &AssetServer,
311    meshes: &mut Assets<Mesh>,
312    materials: &mut Assets<StandardMaterial>,
313) {
314    commands
315        .spawn(Mesh3d(meshes.add(Cuboid::new(
316            LARGE_CUBE_SIZE,
317            LARGE_CUBE_SIZE,
318            LARGE_CUBE_SIZE,
319        ))))
320        .insert(MeshMaterial3d(materials.add(StandardMaterial {
321            base_color: WHITE.into(),
322            base_color_texture: Some(asset_server.load("branding/icon.png")),
323            ..default()
324        })))
325        .insert(Transform::IDENTITY)
326        .insert(LargeCube);
327}
328
329// Spins the outer sphere a bit every frame.
330//
331// This ensures that the set of cubes that are hidden and shown varies over
332// time.
333fn spin_small_cubes(mut sphere_parents: Query<&mut Transform, With<SphereParent>>) {
334    for mut sphere_parent_transform in &mut sphere_parents {
335        sphere_parent_transform.rotate_y(ROTATION_SPEED);
336    }
337}
338
339/// Spins the large cube a bit every frame.
340///
341/// The chaotic rotation adds a bit of randomness to the scene to better
342/// demonstrate the dynamicity of the occlusion culling.
343fn spin_large_cube(mut large_cubes: Query<&mut Transform, With<LargeCube>>) {
344    for mut transform in &mut large_cubes {
345        transform.rotate(Quat::from_euler(
346            EulerRot::XYZ,
347            0.13 * ROTATION_SPEED,
348            0.29 * ROTATION_SPEED,
349            0.35 * ROTATION_SPEED,
350        ));
351    }
352}
353
354/// Spawns a directional light to illuminate the scene.
355fn spawn_light(commands: &mut Commands) {
356    commands
357        .spawn(DirectionalLight::default())
358        .insert(Transform::from_rotation(Quat::from_euler(
359            EulerRot::ZYX,
360            0.0,
361            PI * -0.15,
362            PI * -0.15,
363        )));
364}
365
366/// Spawns a camera that includes the depth prepass and occlusion culling.
367fn spawn_camera(commands: &mut Commands) {
368    commands
369        .spawn(Camera3d::default())
370        .insert(Transform::from_xyz(0.0, 0.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
371        .insert(DepthPrepass)
372        .insert(OcclusionCulling);
373}
374
375/// Spawns the help text at the upper left of the screen.
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
387
388fn readback_indirect_parameters_node(
389    mut render_context: RenderContext,
390    indirect_parameters_buffers: Res<IndirectParametersBuffers>,
391    indirect_parameters_mapping_buffers: Res<IndirectParametersStagingBuffers>,
392) {
393    // Get the indirect parameters buffers corresponding to the opaque 3D
394    // phase, since all our meshes are in that phase.
395    let Some(phase_indirect_parameters_buffers) =
396        indirect_parameters_buffers.get(&TypeId::of::<Opaque3d>())
397    else {
398        return;
399    };
400
401    // Grab both the buffers we're copying from and the staging buffers
402    // we're copying to. Remember that we can't map the indirect parameters
403    // buffers directly, so we have to copy their contents to a staging
404    // buffer.
405    let (
406        Some(indexed_data_buffer),
407        Some(indexed_batch_sets_buffer),
408        Some(indirect_parameters_staging_data_buffer),
409        Some(indirect_parameters_staging_batch_sets_buffer),
410    ) = (
411        phase_indirect_parameters_buffers.indexed.data_buffer(),
412        phase_indirect_parameters_buffers
413            .indexed
414            .batch_sets_buffer(),
415        indirect_parameters_mapping_buffers.data.as_ref(),
416        indirect_parameters_mapping_buffers.batch_sets.as_ref(),
417    )
418    else {
419        return;
420    };
421
422    // Copy from the indirect parameters buffers to the staging buffers.
423    render_context.command_encoder().copy_buffer_to_buffer(
424        indexed_data_buffer,
425        0,
426        indirect_parameters_staging_data_buffer,
427        0,
428        indexed_data_buffer.size(),
429    );
430    render_context.command_encoder().copy_buffer_to_buffer(
431        indexed_batch_sets_buffer,
432        0,
433        indirect_parameters_staging_batch_sets_buffer,
434        0,
435        indexed_batch_sets_buffer.size(),
436    );
437}
438
439/// Creates the staging buffers that we use to read back the indirect parameters
440/// from the GPU to the CPU.
441///
442/// We read the indirect parameters from the GPU to the CPU in order to display
443/// the number of meshes that were culled each frame.
444///
445/// We need these staging buffers because `wgpu` doesn't allow us to read the
446/// contents of the indirect parameters buffers directly. We must first copy
447/// them from the GPU to a staging buffer, and then read the staging buffer.
448fn create_indirect_parameters_staging_buffers(
449    mut indirect_parameters_staging_buffers: ResMut<IndirectParametersStagingBuffers>,
450    indirect_parameters_buffers: Res<IndirectParametersBuffers>,
451    render_device: Res<RenderDevice>,
452) {
453    let Some(phase_indirect_parameters_buffers) =
454        indirect_parameters_buffers.get(&TypeId::of::<Opaque3d>())
455    else {
456        return;
457    };
458
459    // Fetch the indirect parameters buffers that we're going to copy from.
460    let (Some(indexed_data_buffer), Some(indexed_batch_set_buffer)) = (
461        phase_indirect_parameters_buffers.indexed.data_buffer(),
462        phase_indirect_parameters_buffers
463            .indexed
464            .batch_sets_buffer(),
465    ) else {
466        return;
467    };
468
469    // Build the staging buffers. Make sure they have the same sizes as the
470    // buffers we're copying from.
471    indirect_parameters_staging_buffers.data =
472        Some(render_device.create_buffer(&BufferDescriptor {
473            label: Some("indexed data staging buffer"),
474            size: indexed_data_buffer.size(),
475            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
476            mapped_at_creation: false,
477        }));
478    indirect_parameters_staging_buffers.batch_sets =
479        Some(render_device.create_buffer(&BufferDescriptor {
480            label: Some("indexed batch set staging buffer"),
481            size: indexed_batch_set_buffer.size(),
482            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
483            mapped_at_creation: false,
484        }));
485}
486
487/// Updates the app status text at the top of the screen.
488fn update_status_text(
489    saved_indirect_parameters: Res<SavedIndirectParameters>,
490    mut texts: Query<&mut Text>,
491    meshes: Query<Entity, With<Mesh3d>>,
492    app_status: Res<AppStatus>,
493) {
494    // How many meshes are in the scene?
495    let total_mesh_count = meshes.iter().count();
496
497    // Sample the rendered object count. Note that we don't synchronize beyond
498    // locking the data and therefore this will value will generally at least
499    // one frame behind. This is fine; this app is just a demonstration after
500    // all.
501    let (
502        rendered_object_count,
503        occlusion_culling_supported,
504        occlusion_culling_introspection_supported,
505    ): (u32, bool, bool) = {
506        let saved_indirect_parameters = saved_indirect_parameters.lock().unwrap();
507        let Some(saved_indirect_parameters) = saved_indirect_parameters.as_ref() else {
508            // Bail out early if the resource isn't initialized yet.
509            return;
510        };
511        (
512            saved_indirect_parameters
513                .data
514                .iter()
515                .take(saved_indirect_parameters.count as usize)
516                .map(|indirect_parameters| indirect_parameters.instance_count)
517                .sum(),
518            saved_indirect_parameters.occlusion_culling_supported,
519            saved_indirect_parameters.occlusion_culling_introspection_supported,
520        )
521    };
522
523    // Change the text.
524    for mut text in &mut texts {
525        text.0 = String::new();
526        if !occlusion_culling_supported {
527            text.0
528                .push_str("Occlusion culling not supported on this platform");
529            continue;
530        }
531
532        let _ = writeln!(
533            &mut text.0,
534            "Occlusion culling {} (Press Space to toggle)",
535            if app_status.occlusion_culling {
536                "ON"
537            } else {
538                "OFF"
539            },
540        );
541
542        if !occlusion_culling_introspection_supported {
543            continue;
544        }
545
546        let _ = write!(
547            &mut text.0,
548            "{rendered_object_count}/{total_mesh_count} meshes rendered"
549        );
550    }
551}
552
553/// A system that reads the indirect parameters back from the GPU so that we can
554/// report how many meshes were culled.
555fn readback_indirect_parameters(
556    mut indirect_parameters_staging_buffers: ResMut<IndirectParametersStagingBuffers>,
557    saved_indirect_parameters: Res<SavedIndirectParameters>,
558) {
559    // If culling isn't supported on this platform, bail.
560    if !saved_indirect_parameters
561        .lock()
562        .unwrap()
563        .as_ref()
564        .unwrap()
565        .occlusion_culling_supported
566    {
567        return;
568    }
569
570    // Grab the staging buffers.
571    let (Some(data_buffer), Some(batch_sets_buffer)) = (
572        indirect_parameters_staging_buffers.data.take(),
573        indirect_parameters_staging_buffers.batch_sets.take(),
574    ) else {
575        return;
576    };
577
578    // Read the GPU buffers back.
579    let saved_indirect_parameters_0 = (**saved_indirect_parameters).clone();
580    let saved_indirect_parameters_1 = (**saved_indirect_parameters).clone();
581    readback_buffer::<IndirectParametersIndexed>(data_buffer, move |indirect_parameters| {
582        saved_indirect_parameters_0
583            .lock()
584            .unwrap()
585            .as_mut()
586            .unwrap()
587            .data = indirect_parameters.to_vec();
588    });
589    readback_buffer::<u32>(batch_sets_buffer, move |indirect_parameters_count| {
590        saved_indirect_parameters_1
591            .lock()
592            .unwrap()
593            .as_mut()
594            .unwrap()
595            .count = indirect_parameters_count[0];
596    });
597}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#539)

#### pub fn [try\_lock](#method.try_lock)( &self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[MutexGuard](struct.MutexGuard.html "struct bevy::platform::sync::MutexGuard")<'\_, T>, [TryLockError](enum.TryLockError.html "enum bevy::platform::sync::TryLockError")<[MutexGuard](struct.MutexGuard.html "struct bevy::platform::sync::MutexGuard")<'\_, T>>>

Attempts to acquire this lock.

If the lock could not be acquired at this time, then [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned. Otherwise, an RAII guard is returned. The lock will be unlocked when the guard is dropped.

This function does not block.

##### Errors

If another user of this mutex panicked while holding the mutex, then this call will return the [`Poisoned`](enum.TryLockError.html#variant.Poisoned "variant bevy::platform::sync::TryLockError::Poisoned") error if the mutex would otherwise be acquired. An acquired lock guard will be contained in the returned error.

If the mutex could not be acquired because it is already locked, then this call will return the [`WouldBlock`](enum.TryLockError.html#variant.WouldBlock "variant bevy::platform::sync::TryLockError::WouldBlock") error.

##### Examples

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let mutex = Arc::new(Mutex::new(0));
let c_mutex = Arc::clone(&mutex);

thread::spawn(move || {
    let mut lock = c_mutex.try_lock();
    if let Ok(ref mut mutex) = lock {
        **mutex = 10;
    } else {
        println!("try_lock failed");
    }
}).join().expect("thread::spawn failed");
assert_eq!(*mutex.lock().unwrap(), 10);
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#572)

#### pub fn [is\_poisoned](#method.is_poisoned)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Determines whether the mutex is poisoned.

If another thread is active, the mutex can still become poisoned at any time. You should not trust a `false` value for program correctness without additional synchronization.

##### Examples

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let mutex = Arc::new(Mutex::new(0));
let c_mutex = Arc::clone(&mutex);

let _ = thread::spawn(move || {
    let _lock = c_mutex.lock().unwrap();
    panic!(); // the mutex gets poisoned
}).join();
assert_eq!(mutex.is_poisoned(), true);
```

1.77.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#610)

#### pub fn [clear\_poison](#method.clear_poison)(&self)

Clear the poisoned state from a mutex.

If the mutex is poisoned, it will remain poisoned until this function is called. This allows recovering from a poisoned state and marking that it has recovered. For example, if the value is overwritten by a known-good value, then the mutex can be marked as un-poisoned. Or possibly, the value could be inspected to determine if it is in a consistent state, and if so the poison is removed.

##### Examples

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let mutex = Arc::new(Mutex::new(0));
let c_mutex = Arc::clone(&mutex);

let _ = thread::spawn(move || {
    let _lock = c_mutex.lock().unwrap();
    panic!(); // the mutex gets poisoned
}).join();

assert_eq!(mutex.is_poisoned(), true);
let x = mutex.lock().unwrap_or_else(|mut e| {
    **e.get_mut() = 1;
    mutex.clear_poison();
    e.into_inner()
});
assert_eq!(mutex.is_poisoned(), false);
assert_eq!(*x, 1);
```

1.6.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#631-633)

#### pub fn [into\_inner](#method.into_inner)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [PoisonError](struct.PoisonError.html "struct bevy::platform::sync::PoisonError")<T>>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Consumes this mutex, returning the underlying data.

##### Errors

If another user of this mutex panicked while holding the mutex, then this call will return an error containing the underlying data instead.

##### Examples

```rust
use std::sync::Mutex;

let mutex = Mutex::new(0);
assert_eq!(mutex.into_inner().unwrap(), 0);
```

1.6.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#664)

#### pub fn [get\_mut](#method.get_mut)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [PoisonError](struct.PoisonError.html "struct bevy::platform::sync::PoisonError")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>>

Returns a mutable reference to the underlying data.

Since this call borrows the `Mutex` mutably, no actual locking needs to take place – the mutable borrow statically guarantees no new locks can be acquired while this reference exists. Note that this method does not clear any previous abandoned locks (e.g., via [`forget()`](https://doc.rust-lang.org/nightly/core/mem/fn.forget.html "fn core::mem::forget") on a [`MutexGuard`](struct.MutexGuard.html "struct bevy::platform::sync::MutexGuard")).

##### Errors

If another user of this mutex panicked while holding the mutex, then this call will return an error containing a mutable reference to the underlying data instead.

##### Examples

```rust
use std::sync::Mutex;

let mut mutex = Mutex::new(0);
*mutex.get_mut().unwrap() = 10;
assert_eq!(*mutex.lock().unwrap(), 10);
```

[Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#676)

#### pub const fn [data\_ptr](#method.data_ptr)(&self) -> [\*mut T](https://doc.rust-lang.org/nightly/std/primitive.pointer.html)

🔬This is a nightly-only experimental API. (`mutex_data_ptr`)

Returns a raw pointer to the underlying data.

The returned pointer is always non-null and properly aligned, but it is the user’s responsibility to ensure that any reads and writes through it are properly synchronized to avoid data races, and that it is not read or written through after the mutex is dropped.

## Trait Implementations

[Source](https://docs.rs/sharded-slab/0.1.7/x86_64-unknown-linux-gnu/src/sharded_slab/clear.rs.html#81)

### impl<T> [Clear](https://docs.rs/sharded-slab/0.1.7/x86_64-unknown-linux-gnu/sharded_slab/clear/trait.Clear.html "trait sharded_slab::clear::Clear") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [Clear](https://docs.rs/sharded-slab/0.1.7/x86_64-unknown-linux-gnu/sharded_slab/clear/trait.Clear.html "trait sharded_slab::clear::Clear"),

[Source](https://docs.rs/sharded-slab/0.1.7/x86_64-unknown-linux-gnu/src/sharded_slab/clear.rs.html#83)

#### fn [clear](https://docs.rs/sharded-slab/0.1.7/x86_64-unknown-linux-gnu/sharded_slab/clear/trait.Clear.html#tymethod.clear)(&mut self)

Clear all data in `self`, retaining the allocated capacithy.

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#672)

### impl<C> [ClockSequence](../../asset/uuid/trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<C>

where C: [ClockSequence](../../asset/uuid/trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") + [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#673)

#### type [Output](../../asset/uuid/trait.ClockSequence.html#associatedtype.Output) = <C as [ClockSequence](../../asset/uuid/trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../../asset/uuid/trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output")

The type of sequence returned by this counter.

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#675)

#### fn [generate\_sequence](../../asset/uuid/trait.ClockSequence.html#tymethod.generate_sequence)( &self, seconds: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), subsec\_nanos: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> <[Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<C> as [ClockSequence](../../asset/uuid/trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../../asset/uuid/trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output")

Get the next value in the sequence to feed into a timestamp. [Read more](../../asset/uuid/trait.ClockSequence.html#tymethod.generate_sequence)

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#681-685)

#### fn [generate\_timestamp\_sequence](../../asset/uuid/trait.ClockSequence.html#method.generate_timestamp_sequence)( &self, seconds: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), subsec\_nanos: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> (<[Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<C> as [ClockSequence](../../asset/uuid/trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../../asset/uuid/trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output"), [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Get the next value in the sequence, potentially also adjusting the timestamp. [Read more](../../asset/uuid/trait.ClockSequence.html#method.generate_timestamp_sequence)

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#691-693)

#### fn [usable\_bits](../../asset/uuid/trait.ClockSequence.html#method.usable_bits)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where <[Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<C> as [ClockSequence](../../asset/uuid/trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence")\>::[Output](../../asset/uuid/trait.ClockSequence.html#associatedtype.Output "type bevy::asset::uuid::ClockSequence::Output"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The number of usable bits from the least significant bit in the result of [`ClockSequence::generate_sequence`](../../asset/uuid/trait.ClockSequence.html#tymethod.generate_sequence "method bevy::asset::uuid::ClockSequence::generate_sequence") or [`ClockSequence::generate_timestamp_sequence`](../../asset/uuid/trait.ClockSequence.html#method.generate_timestamp_sequence "method bevy::asset::uuid::ClockSequence::generate_timestamp_sequence"). [Read more](../../asset/uuid/trait.ClockSequence.html#method.usable_bits)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#699)

### impl<T> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#700)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

1.10.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#691)

### impl<T> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#693)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

Creates a `Mutex<T>`, with the `Default` value for T.

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/impls.rs.html#2107-2111)

### impl<'de, T> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

Available on **crate feature `std`** only.

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/impls.rs.html#2107-2111)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>, <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

1.24.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#682)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

[Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#685)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

Creates a new mutex in an unlocked state ready for use. This is equivalent to [`Mutex::new`](struct.Mutex.html#method.new "associated function bevy::platform::sync::Mutex::new").

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#808-810)

### impl<'a, W> [MakeWriter](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<W>

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") + 'a,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#812)

#### type [Writer](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#associatedtype.Writer) = [MutexGuardWriter](../../log/tracing_subscriber/fmt/writer/struct.MutexGuardWriter.html "struct bevy::log::tracing_subscriber::fmt::writer::MutexGuardWriter")<'a, W>

The concrete [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") implementation returned by [`make_writer`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#tymethod.make_writer "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer").

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#814)

#### fn [make\_writer](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#tymethod.make_writer)(&'a self) -> <[Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<W> as [MakeWriter](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>>::[Writer](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer")

Returns an instance of [`Writer`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#associatedtype.Writer "associated type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer"). [Read more](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#tymethod.make_writer)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#208)

#### fn [make\_writer\_for](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#method.make_writer_for)(&'a self, meta: &[Metadata](../../log/tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> Self::[Writer](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer")

Returns a [`Writer`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#associatedtype.Writer "associated type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer") for writing data from the span or event described by the provided [`Metadata`](../../log/tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata"). [Read more](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#method.make_writer_for)

1.12.0 · [Source](https://doc.rust-lang.org/nightly/src/std/panic.rs.html#271)

### impl<T> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#238)

### impl<T> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

`T` must be `Send` for a [`Mutex`](struct.Mutex.html "struct bevy::platform::sync::Mutex") to be `Send` because it is possible to acquire the owned `T` from the `Mutex` via [`into_inner`](struct.Mutex.html#method.into_inner "method bevy::platform::sync::Mutex::into_inner").

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/ser/impls.rs.html#629-631)

### impl<T> [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `std`** only.

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/ser/impls.rs.html#633-635)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/reactive_context.rs.html#389)

### impl [SubscriberList](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/reactive_context/trait.SubscriberList.html "trait dioxus_core::reactive_context::SubscriberList") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<[HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<[ReactiveContext](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/reactive_context/struct.ReactiveContext.html "struct dioxus_core::reactive_context::ReactiveContext")\>>

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/reactive_context.rs.html#390)

#### fn [add](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/reactive_context/trait.SubscriberList.html#tymethod.add)(&self, subscriber: [ReactiveContext](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/reactive_context/struct.ReactiveContext.html "struct dioxus_core::reactive_context::ReactiveContext"))

Add a subscriber to the list.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/reactive_context.rs.html#398)

#### fn [remove](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/reactive_context/trait.SubscriberList.html#tymethod.remove)(&self, subscriber: &[ReactiveContext](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/reactive_context/struct.ReactiveContext.html "struct dioxus_core::reactive_context::ReactiveContext"))

Remove a subscriber from the list.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/reactive_context.rs.html#406)

#### fn [visit](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/reactive_context/trait.SubscriberList.html#tymethod.visit)(&self, f: &mut dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&[ReactiveContext](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/reactive_context/struct.ReactiveContext.html "struct dioxus_core::reactive_context::ReactiveContext")))

Visit all subscribers in the list.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison/mutex.rs.html#257)

### impl<T> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

`T` must be `Send` for [`Mutex`](struct.Mutex.html "struct bevy::platform::sync::Mutex") to be `Sync`. This ensures that the protected data can be accessed safely from multiple threads without causing data races or other unsafe behavior.

[`Mutex<T>`](struct.Mutex.html "struct bevy::platform::sync::Mutex") provides mutable access to `T` to one thread at a time. However, it’s essential for `T` to be `Send` because it’s not safe for non-`Send` structures to be accessed in this manner. For instance, consider [`Rc`](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc"), a non-atomic reference counted smart pointer, which is not `Send`. With `Rc`, we can have multiple copies pointing to the same heap allocation with a non-atomic reference count. If we were to use `Mutex<Rc<_>>`, it would only protect one instance of `Rc` from shared access, leaving other copies vulnerable to potential data races.

Also note that it is not necessary for `T` to be `Sync` as `&T` is only made available to one thread at a time if `T` is not `Sync`.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/libstd.rs.html#114)

### impl<T> [Type](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html "trait zvariant::type::Type") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [Type](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html "trait zvariant::type::Type") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/libstd.rs.html#114)

#### const [SIGNATURE](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html#associatedconstant.SIGNATURE): &'static [Signature](https://docs.rs/zvariant_utils/3.3.0/x86_64-unknown-linux-gnu/zvariant_utils/signature/enum.Signature.html "enum zvariant_utils::signature::Signature") = T::SIGNATURE

The signature for the implementing type, in parsed format. [Read more](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html#associatedconstant.SIGNATURE)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#4)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#4)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#4)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#4)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#4)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#4)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

1.9.0 · [Source](https://doc.rust-lang.org/nightly/src/std/panic.rs.html#264)

### impl<T> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

## Auto Trait Implementations

### impl<T> ![Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

### impl<T> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<T> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where T: [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

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

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#39-41)

### impl<'de, T> [DynamicDeserialize](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html "trait zvariant::type::dynamic::DynamicDeserialize")<'de> for T

where T: [Type](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html "trait zvariant::type::Type") + [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#43)

#### type [Deserializer](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html#associatedtype.Deserializer) = [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

A [DeserializeSeed](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeSeed.html "trait serde_core::de::DeserializeSeed") implementation for this type.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#45)

#### fn [deserializer\_for\_signature](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html#tymethod.deserializer_for_signature)( signature: &[Signature](https://docs.rs/zvariant_utils/3.3.0/x86_64-unknown-linux-gnu/zvariant_utils/signature/enum.Signature.html "enum zvariant_utils::signature::Signature"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [DynamicDeserialize](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html "trait zvariant::type::dynamic::DynamicDeserialize")<'de>>::[Deserializer](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicDeserialize.html#associatedtype.Deserializer "type zvariant::type::dynamic::DynamicDeserialize::Deserializer"), [Error](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/error/enum.Error.html "enum zvariant::error::Error")\>

Get a deserializer compatible with this parsed signature.

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#30-32)

### impl<T> [DynamicType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicType.html "trait zvariant::type::dynamic::DynamicType") for T

where T: [Type](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/trait.Type.html "trait zvariant::type::Type") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/type/dynamic.rs.html#34)

#### fn [signature](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicType.html#tymethod.signature)(&self) -> [Signature](https://docs.rs/zvariant_utils/3.3.0/x86_64-unknown-linux-gnu/zvariant_utils/signature/enum.Signature.html "enum zvariant_utils::signature::Signature")

The type signature for `self`. [Read more](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/type/dynamic/trait.DynamicType.html#tymethod.signature)

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

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#804)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[!](https://doc.rust-lang.org/nightly/std/primitive.never.html)\> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#805)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: [!](https://doc.rust-lang.org/nightly/std/primitive.never.html)) -> T

Converts to this type from the input type.

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

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

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

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#1221)

### impl<'a, M> [MakeWriterExt](../../log/tracing_subscriber/prelude/trait._.html "trait bevy::log::tracing_subscriber::prelude::_")<'a> for M

where M: [MakeWriter](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#279-281)

#### fn [with\_max\_level](../../log/tracing_subscriber/prelude/trait._.html#method.with_max_level)(self, level: [Level](../../log/struct.Level.html "struct bevy::log::Level")) -> [WithMaxLevel](../../log/tracing_subscriber/fmt/writer/struct.WithMaxLevel.html "struct bevy::log::tracing_subscriber::fmt::writer::WithMaxLevel")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wraps `self` and returns a [`MakeWriter`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") that will only write output for events at or below the provided verbosity [`Level`](../../log/struct.Level.html "struct bevy::log::Level"). For instance, `Level::TRACE` is considered to be \_more verbose`than`Level::INFO\`. [Read more](../../log/tracing_subscriber/prelude/trait._.html#method.with_max_level)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#321-323)

#### fn [with\_min\_level](../../log/tracing_subscriber/prelude/trait._.html#method.with_min_level)(self, level: [Level](../../log/struct.Level.html "struct bevy::log::Level")) -> [WithMinLevel](../../log/tracing_subscriber/fmt/writer/struct.WithMinLevel.html "struct bevy::log::tracing_subscriber::fmt::writer::WithMinLevel")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wraps `self` and returns a [`MakeWriter`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") that will only write output for events at or above the provided verbosity [`Level`](../../log/struct.Level.html "struct bevy::log::Level"). [Read more](../../log/tracing_subscriber/prelude/trait._.html#method.with_min_level)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#400-403)

#### fn [with\_filter](../../log/tracing_subscriber/prelude/trait._.html#method.with_filter)<F>(self, filter: F) -> [WithFilter](../../log/tracing_subscriber/fmt/writer/struct.WithFilter.html "struct bevy::log::tracing_subscriber::fmt::writer::WithFilter")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[Metadata](../../log/tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Wraps `self` with a predicate that takes a span or event’s [`Metadata`](../../log/tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata") and returns a `bool`. The returned [`MakeWriter`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")’s [`MakeWriter::make_writer_for`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#method.make_writer_for "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer_for") method will check the predicate to determine if a writer should be produced for a given span or event. [Read more](../../log/tracing_subscriber/prelude/trait._.html#method.with_filter)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#455-458)

#### fn [and](../../log/tracing_subscriber/prelude/trait._.html#method.and)<B>(self, other: B) -> [Tee](../../log/tracing_subscriber/fmt/writer/struct.Tee.html "struct bevy::log::tracing_subscriber::fmt::writer::Tee")<Self, B> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [MakeWriter](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>,

Combines `self` with another type implementing [`MakeWriter`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter"), returning a new [`MakeWriter`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") that produces [writers](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") that write to _both_ outputs. [Read more](../../log/tracing_subscriber/prelude/trait._.html#method.and)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#486-490)

#### fn [or\_else](../../log/tracing_subscriber/prelude/trait._.html#method.or_else)<W, B>(self, other: B) -> [OrElse](../../log/tracing_subscriber/fmt/writer/struct.OrElse.html "struct bevy::log::tracing_subscriber::fmt::writer::OrElse")<Self, B>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [MakeWriter](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a, Writer = [EitherWriter](../../log/tracing_subscriber/fmt/writer/enum.EitherWriter.html "enum bevy::log::tracing_subscriber::fmt::writer::EitherWriter")<W, [Sink](https://doc.rust-lang.org/nightly/core/io/util/struct.Sink.html "struct core::io::util::Sink")\>>, B: [MakeWriter](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>, W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

Combines `self` with another type implementing [`MakeWriter`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter"), returning a new [`MakeWriter`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") that calls `other`’s [`make_writer`](../../log/tracing_subscriber/fmt/trait.MakeWriter.html#tymethod.make_writer "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer") if `self`’s `make_writer` returns [`OptionalWriter::none`](../../log/tracing_subscriber/fmt/writer/enum.EitherWriter.html#method.none "associated function bevy::log::tracing_subscriber::fmt::writer::EitherWriter::none"). [Read more](../../log/tracing_subscriber/prelude/trait._.html#method.or_else)

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

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Tee<Self, B>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing\_subscriber/fmt/writer/struct.Tee.html\\" title=\\"struct bevy::log::tracing\_subscriber::fmt::writer::Tee\\">Tee</a>&lt;A, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A, B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../log/tracing\_subscriber/fmt/writer/struct.Tee.html\\" title=\\"struct bevy::log::tracing\_subscriber::fmt::writer::Tee\\">Tee</a>&lt;A, B&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,\\n B: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}