[bevy](../index.html)::[scene](index.html)

# Trait SceneBox 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#82)

```rust
pub trait SceneBox:
    Send
    + Sync
    + 'static {
    // Required methods
    fn resolve_box(
        self: Box<Self>,
        context: &mut ResolveContext<'_>,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError>;
    fn register_dependencies_box(&self, _dependencies: &mut SceneDependencies);
}
```

Boxed version of [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"), which enables implementing [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [`Box<dyn Scene>`](../prelude/struct.Box.html "struct bevy::prelude::Box"). Most developers do not need to think about or use this trait.

Related: [`SceneListBox`](trait.SceneListBox.html "trait bevy::scene::SceneListBox").

### Why does this exist?

[`Scene::resolve`](../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve") consumes `self`, which by default is not something that [`Box<dyn Scene>`](../prelude/struct.Box.html "struct bevy::prelude::Box") can do in Rust, as `dyn Scene` is “unsized”. The “way out” is to have every [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") type _also_ know how to resolve itself for `self: Box<Self>`. [`SceneBox`](trait.SceneBox.html "trait bevy::scene::SceneBox") has a blanket impl for `Scene + Sized` (which can just rely on the [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") impl). Then [`Box<dyn Scene>`](../prelude/struct.Box.html "struct bevy::prelude::Box") has a manual [`SceneBox`](trait.SceneBox.html "trait bevy::scene::SceneBox") impl that relies on the _stored_ [`SceneBox::resolve_box`](trait.SceneBox.html#tymethod.resolve_box "method bevy::scene::SceneBox::resolve_box") impl.

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#84-88)

#### fn [resolve\_box](#tymethod.resolve_box)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<Self>, context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, scene: &mut [ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

See [`Scene::resolve`](../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#91)

#### fn [register\_dependencies\_box](#tymethod.register_dependencies_box)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

See [`Scene::register_dependencies`](../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#94)

### impl<S> [SceneBox](trait.SceneBox.html "trait bevy::scene::SceneBox") for S

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),