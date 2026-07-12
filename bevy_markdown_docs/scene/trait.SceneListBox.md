[bevy](../index.html)::[scene](index.html)

# Trait SceneListBox 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#43)

```rust
pub trait SceneListBox:
    Send
    + Sync
    + 'static {
    // Required methods
    fn resolve_list_box(
        self: Box<Self>,
        context: &mut ResolveContext<'_>,
        scenes: &mut Vec<ResolvedScene>,
    ) -> Result<(), ResolveSceneError>;
    fn register_dependencies_box(&self, dependencies: &mut SceneDependencies);
}
```

Boxed version of [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), which enables implementing [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [`Box<dyn SceneList>`](../prelude/struct.Box.html "struct bevy::prelude::Box"). Most developers do not need to think about or use this trait.

Related: [`SceneBox`](trait.SceneBox.html "trait bevy::scene::SceneBox").

### Why does this exist?

[`SceneList::resolve_list`](../prelude/trait.SceneList.html#tymethod.resolve_list "method bevy::prelude::SceneList::resolve_list") consumes `self`, which by default is not something that [`Box<dyn SceneList>`](../prelude/struct.Box.html "struct bevy::prelude::Box") can do in Rust, as `dyn Scene` is “unsized”. The “way out” is to have every [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") type _also_ know how to resolve itself for `self: Box<Self>`. [`SceneListBox`](trait.SceneListBox.html "trait bevy::scene::SceneListBox") has a blanket impl for `SceneList + Sized` (which can just rely on the [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") impl). Then [`Box<dyn SceneList>`](../prelude/struct.Box.html "struct bevy::prelude::Box") has a manual [`SceneListBox`](trait.SceneListBox.html "trait bevy::scene::SceneListBox") impl that relies on the _stored_ [`SceneListBox::resolve_list_box`](trait.SceneListBox.html#tymethod.resolve_list_box "method bevy::scene::SceneListBox::resolve_list_box") impl.

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#45-49)

#### fn [resolve\_list\_box](#tymethod.resolve_list_box)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<Self>, context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

See [`SceneList::resolve_list`](../prelude/trait.SceneList.html#tymethod.resolve_list "method bevy::prelude::SceneList::resolve_list").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#52)

#### fn [register\_dependencies\_box](#tymethod.register_dependencies_box)(&self, dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

See [`SceneList::register_dependencies`](../prelude/trait.SceneList.html#tymethod.register_dependencies "method bevy::prelude::SceneList::register_dependencies").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#55)

### impl<L> [SceneListBox](trait.SceneListBox.html "trait bevy::scene::SceneListBox") for L

where L: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),