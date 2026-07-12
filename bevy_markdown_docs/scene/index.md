[bevy](../index.html)

# Crate scene 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/lib.rs.html#1-2962)

Composable scene authoring for Bevy, defined using the Bevy Scene Notation (BSN) format.

Game entities rarely exist in isolation. A 3D level might be made up of walls, floors, props and enemies. A 2D character might need a distinct sprite entity for weapon, hat and boots. A UI popup might need text and multiple buttons for accept, cancel, minimize and close actions. Spawning these collections as individual, disjointed entities is tedious, error-prone, and hard to reuse. A **scene** lets you describe a conceptual **object**, made of an entity, its components, children, and assets, once and spawn it wherever you need it.

Any scene system must overcome three challenges:

*   **Composability**: combining smaller scenes into larger ones without duplicating shared constants and setup code.
*   **Granular overrides**: when reusing a scene, overriding _individual fields_ on a component (like changing just a button’s width) without having to respecify every other field on that component.
*   **Asset integration**: referencing assets (meshes, textures, sounds) from within scenes without manually wiring up asset handles.

This crate tackles all three, via [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") composition, [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template")\-based field-level patching, and automatic string-to-asset-handle resolution.

The \[`bsn!`\] macro exposes these ideas, and makes the process of scene-authoring pleasant by providing a terse syntax for defining [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene")s inline. This brevity is essential: making it easier to review and understand scenes at a glance, resolve merge conflicts and keep file sizes under control. The macro includes best-effort Rust-Analyzer support. Autocomplete, go-to-definition, and hover docs should work inside the macros, and this effort should transfer over correctly to other LSPs!

### BSN syntax reference

For a quick rundown on how to read and write BSN syntax, see the docs for \[`bsn!`\].

### Quick Start

Spawn entities in a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") by calling [`World::spawn_scene`](../prelude/trait.WorldSceneExt.html#tymethod.spawn_scene "method bevy::prelude::WorldSceneExt::spawn_scene"), wrapping a call to the \[`bsn!`\] macro.

```rust
#[derive(Component, Default, Clone)]
struct Score(usize);

#[derive(Component, Default, Clone)]
struct Sword;

#[derive(Component, Default, Clone)]
struct Shield;

// #Player adds a `Name("Player")` component to the root entity.
// Children spawns two child entities: one with Sword, one with Shield.
world.spawn_scene(bsn! {
    #Player // This names the entity "Player"
    Score(0)
    Children [
        Sword,
        Shield,
    ]
});
```

### Core Concepts

*   **[`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene")**: Describes what a spawned [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") should look like, created using \[`bsn!`\] or, in the future, `.bsn` asset files. Conceptually, a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") contains a list of “entries” to apply to an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity").
*   **[`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList")**: A list of scenes, returned by [`bsn_list!`](../prelude/macro.bsn_list.html "macro bevy::prelude::bsn_list"). Each [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") in the list produces one [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity").
*   **Scene Composition**: Composition works by including scenes in other scenes. The included scenes “entries” will be treated as if they were written in the outer scene.
*   **[`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template")**: A [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") is something that, given a spawn context (target [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [`World`](../prelude/struct.World.html "struct bevy::prelude::World"), etc), can produce some output. Think of it as a “superpowered ECS-aware constructor” for a type. In the context of scenes, [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template")s are used to produce [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")s and [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")s. This enables defining scenes without needing to pass in a bunch of their dependencies (such as assets). The [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") trait is used to associate some final output type (ex: a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")) with a canonical [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that produces it. [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") / [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") is automatically implemented for types that implement [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), which is generally preferred. You should manually derive [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") when a type needs custom template logic (ex: one of its fields is an “asset handle”, which has custom template logic).
*   **[`RelatedScenes`](struct.RelatedScenes.html "struct bevy::scene::RelatedScenes")**: These add a [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") as related to this [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") by a specific [`relationship`](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"). This kind of change is added to the [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") by specifying a [`RelationshipTarget`](../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component like [`Children`](../prelude/struct.Children.html "struct bevy::prelude::Children"), followed by a [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList").

### Spawning Scenes

There are two approaches to spawning scenes:

*   **Immediate**: [`World::spawn_scene`](../prelude/trait.WorldSceneExt.html#tymethod.spawn_scene "method bevy::prelude::WorldSceneExt::spawn_scene") and [`Commands::spawn_scene`](../prelude/trait.CommandsSceneExt.html#tymethod.spawn_scene "method bevy::prelude::CommandsSceneExt::spawn_scene") resolve and spawn in one step. Returns an error if any asset dependencies are not yet loaded.
*   **Queued**: [`World::queue_spawn_scene`](../prelude/trait.WorldSceneExt.html#tymethod.queue_spawn_scene "method bevy::prelude::WorldSceneExt::queue_spawn_scene") and [`Commands::queue_spawn_scene`](../prelude/trait.CommandsSceneExt.html#tymethod.queue_spawn_scene "method bevy::prelude::CommandsSceneExt::queue_spawn_scene") register the scene’s dependencies and wait for them to load before resolving and spawning. When the dependencies are loaded (or there are no dependencies), the scene will spawn during that frame’s [`SpawnScene`](../prelude/struct.SpawnScene.html "struct bevy::prelude::SpawnScene") schedule, between [`Update`](../prelude/struct.Update.html "struct bevy::prelude::Update") and [`PostUpdate`](../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate").

In all cases, your `*_spawn_scene` method call should wrap an invocation of the \[`bsn!`\] macro, or call a function which returns a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene").

See the [`WorldSceneExt`](../prelude/trait.WorldSceneExt.html "trait bevy::prelude::WorldSceneExt"), [`CommandsSceneExt`](../prelude/trait.CommandsSceneExt.html "trait bevy::prelude::CommandsSceneExt"), [`EntityWorldMutSceneExt`](../prelude/trait.EntityWorldMutSceneExt.html "trait bevy::prelude::EntityWorldMutSceneExt"), and [`EntityCommandsSceneExt`](../prelude/trait.EntityCommandsSceneExt.html "trait bevy::prelude::EntityCommandsSceneExt") extension traits for the full set of scene-spawning APIs.

### Entity Hierarchies and Relationships

Use `Children [scene1, scene2]` inside \[`bsn!`\] to spawn child entities. [`Children`](../prelude/struct.Children.html "struct bevy::prelude::Children") (and entities within [`bsn_list!`](../prelude/macro.bsn_list.html "macro bevy::prelude::bsn_list")) are separated by commas; add multiple components to the same entity by listing them without a comma:

[ⓘ](# "This example is not tested")

```rust
// Spawns one child entity with components A, B and C
bsn! { #Parent Children [A B C] }

// Spawns two child entities, one with A and B, the other with C, due to the added comma
bsn! { #Parent Children [A B, C] }

// Spawns two child entities, but more clearly separated due to parentheses.
bsn! { #Parent Children [(A B), C] }
```

These invocations can be nested to build deeper hierarchies.

[ⓘ](# "This example is not tested")

```rust
bsn! {
  #Parent
  Children [
    #Child1 SomeComponent,
    #Child2
    SomeComponent
    Children [
       #GrandChild1 SomeComponent,
       #GrandChild2
    ]
  ]
}
```

We can improve clarity at the cost of compactness through the careful use of newlines, parentheses and indentation:

[ⓘ](# "This example is not tested")

```rust
bsn! {
  #Parent
  Children [
     (
       #Child1
       SomeComponent
     ),
     (
       #Child2
       Children [
          (
            #GrandChild1
            SomeComponent
          ),
          (
            #GrandChild2
          )
       ]
     ),
  ]
}
```

This is fundamentally a stylistic choice: white space, Rust comments (`//` and `/* */`), and parentheses used in this way are ignored.

The tools discussed here are not limited to [`Children`](../prelude/struct.Children.html "struct bevy::prelude::Children"): any [`RelationshipTarget`](../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") type can be used the same way.

### Named Entity References

The `#Name` syntax assigns a [`Name`](../prelude/struct.Name.html "struct bevy::prelude::Name") to an entity and registers it for cross-referencing within the same macro invocation. Within the same bsn! invocation / scope, it is possible to reference an entity by its `#Name`, generating an [`EntityTemplate`](../ecs/template/enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate") which ultimately resolves to an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity"):

[ⓘ](# "This example is not tested")

```rust
bsn! {
    #Name
    my_scene(#Name)
    ComponentA(#Name)
    ComponentB { entity: #Name }
    Children [
        ComponentC(#Name)
    ]
}
```

Notice that the “child entity” was able to access the parent entity via `#Name`. It is also possible for ancestors to access their descendants:

[ⓘ](# "This example is not tested")

```rust
bsn! {
    #Root
    Children [
        Reference(#Root)
    ]
}
```

Using `#Name` as a value in \[`bsn!`\] will result in an [`EntityTemplate`](../ecs/template/enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate"), which is a [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that resolves to an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")s with [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") fields should generally derive [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"), because [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") uses [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") to map to [`EntityTemplate`](../ecs/template/enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate").

#### Scope rules

Each \[`bsn!`\] invocation creates its own name scope. A name is visible to the root entity, its children, and any deeper descendants in the same call. The reverse is also true: descendants can “look up” the hierarchy. Composed scenes (via `my_scene(#Name)`) or [`SceneComponents`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") each contain their own \[`bsn!`\] invocation and therefore their own scope, so re-using the same name across multiple different scenes is fine. However, the results of a named entity reference, the [`EntityTemplate`](../ecs/template/enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate"), can be passed to other scenes. It is valid only during the spawning of a scene. That means [`Components`](../prelude/trait.Component.html "trait bevy::prelude::Component") should never store [`EntityTemplate`](../ecs/template/enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate") fields, they should store the resolved [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") instead and derive [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") to convert [`EntityTemplate`](../ecs/template/enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate") automatically.

If both a parent and a composed child define the same name (e.g. both use `#X`), each scope’s `#X` resolves to its own entity, avoiding conflicts or potentially unintuitive shadowing.

In a [`bsn_list!`](../prelude/macro.bsn_list.html "macro bevy::prelude::bsn_list"), all root entities share a single name scope, so sibling scenes can reference each other by name. This is useful for wiring up relationships between entities that are spawned together. For example, a group of UI panels where each panel needs a relationship to its neighbor:

[ⓘ](# "This example is not tested")

```rust
fn linked_pair() -> impl SceneList {
    bsn_list![
        (#Left  Link(#Right)),
        (#Right Link(#Left)),
    ]
}
```

#### Dynamic Name Values and Entity References

`#SomeName` syntax will set the value of the [`Name`](../prelude/struct.Name.html "struct bevy::prelude::Name") component to `Name("SomeName")`, and make the entity reference-able in \[`bsn!`\]. `#Name` syntax is _always_ scoped and doesn’t support “dynamic” names. If you would like to _both_ reference an entity in \[`bsn!`\] _and_ provide a dynamic name, you can do this:

[ⓘ](# "This example is not tested")

```rust
let i = 0;
bsn! {
  #Root
  Name({format!("Entity {i}")})
  Children [
    Reference(#Root)
  ]
}
```

Adding `Name("desired name")` after the `#SomeName` reference will patch over the `Name` component created by the reference to give it a custom name.

### Patching

When you insert a component into an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") in normal ECS code, the entire pre-existing value is replaced. If a scene sets `Button { width: 100, height: 300 }` and a caller wants to change just `width`, ordinary component insertion would force them to respecify `height` too.

**Patching** avoids this. When you write `Button { width: 200 }` in \[`bsn!`\], it creates a _patch_ that sets only the `width` field. Unmentioned fields keep their existing values (from a included scene, an earlier patch, or the type’s defaults). Multiple patches to the same component and its values are applied in order, only overwriting the fields they changed.

The following scenes all end up with a button which is 200 wide and 300 high.

[ⓘ](# "This example is not tested")

```rust
impl Default for Button {
    fn default() -> Self {
        Button { width: 100, height: 300 }
    }
}

bsn! { Button { width: 200, height: 300 } } // fully specified
bsn! { Button { width: 200 } }              // only changing width, height defaults to 300

bsn! {
    Button                 // inserts defaults
    Button { width: 200 }  // changes width
    Button { height: 300 } // changes height
}
```

#### Required Traits

To make a component available in \[`bsn!`\], derive either [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), or [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"). Both support patching: unmentioned fields keep their values from earlier patches or the type’s defaults, and multiple patches merge rather than overwrite.

The distinction is about what values a field can hold at spawn time:

*   **[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default")** (e.g. `#[derive(Component, Default, Clone)]`): covers the simple case, and should be your default choice.
*   **[`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")** (e.g. `#[derive(Component, FromTemplate)]`) is needed when a field requires spawn-time context. Examples include [`Handle<T>`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") fields which need [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") to resolve asset paths, or [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") fields which resolve [`EntityTemplate`](../ecs/template/enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate")s from named entity references. If any of your fields’ types implement [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") manually / have custom template logic, you should derive it for the parent type as well if you want your type to use that logic.

Deriving [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") and [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") on the same type is not allowed, as both would supply a [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") impl and conflict. [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") derivers still have access to a default constructor of sorts though: the derive generates a companion struct for `YourType` named `YourTypeTemplate` which implements `Default`, so `YourTypeTemplate::default()` serves the same purpose.

##### Enums in bsn

Enums are special-cased to allow for better implicit defaults: \[`bsn!`\] requires that enums have defaults for all variant arms, not just the type as a whole.

When \[`bsn!`\] encounters a Enum, it will try to get the default value for the variant using static methods like `default_{variant_lower}`. To help with setting up these methods, theres a pseudo-`derive` called [`VariantDefaults`](../ecs/derive.VariantDefaults.html "derive bevy::ecs::VariantDefaults"). It works like a normal `derive` macro, but without a matching Trait. It just generates a impl block with the `default_{variant_lower}` static methods.

Deriving [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") also implies/works like [`VariantDefaults`](../ecs/derive.VariantDefaults.html "derive bevy::ecs::VariantDefaults").

### Composition

Composition relies on patching to work nicely, allowing you to include other scenes in the current ones. All of their patches will be applied at the position they’re included.

Example:

```rust
#[derive(Component, FromTemplate)]
struct Health {
 current: u32,
 max: u32
}

fn enemy() -> impl Scene {
    bsn! { Health { current: 100, max: 100 } }
}

// Include `enemy()` and patch just the `max` field:
world.spawn_scene(bsn! {
    enemy()
    Health { max: 200 }
});
```

The spawned entity has `Health { current: 100, max: 200 }`: the `max` field is overridden while `current` retains the value from `enemy()`. Tuples of [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene")s also implement [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"), so patches from multiple sources merge into a single [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene").

For programmatic patching outside of \[`bsn!`\], see the [`PatchFromTemplate`](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") and [`PatchTemplate`](../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") traits.

### Scene Caching

Note: Caching is currently only implemented for scene assets. It hasn’t yet been wired up for “function scenes” or [`SceneComponent`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent")s. Attempting to use it in those cases will result in a compile error.

Scenes can be cached, improving performance. Since this can change the semantics in some cases, this requires an explicit opt-in. Caching works by resolving the included scene and storing the resulting [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") for future use. When the outer scene is spawned again, it will not need to resolve the included scene again, instead patching on top of the cached version (using copy-on-write semantics for each [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template")). This means caching can only be used if the scene is the first scene entry.

This scene includes an uncached “enemy” scene:

[ⓘ](# "This example is not tested")

```rust
bsn! {
    enemy()
    Health { max: 200 }
}
```

This scene caches the “enemy” scene by adding the `:` prefix (however caching scene functions like this is not currently supported)

[ⓘ](# "This example is not tested")

```rust
bsn! {
    :enemy
    Health { max: 200 }
}
```

Scene assets always need to be cached using the `:` prefix. Note that the `.bsn` file format is not yet released. (This already works, assuming theres a loader for the asset format)

[ⓘ](# "This example is not tested")

```rust
bsn! {
   :"enemy.bsn"
   Health { max: 200 }
}
```

### Loading Assets into Scenes

Without the use of scenes, loading an asset requires referencing the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") explicitly:

[ⓘ](# "This example is not tested")

```rust
let handle: Handle<Image> = asset_server.load("player.png");
commands.spawn(Sprite { image: handle, ..default() });
```

This can be particularly frustrating when defining helper functions for spawning entities, which require you to pass [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") or handles through multiple layers of function calls.

In \[`bsn!`\], asset paths work directly as field values. When a component field is a [`Handle<T>`](../prelude/enum.Handle.html "enum bevy::prelude::Handle"), the \[`bsn!`\] macro accepts a string literal in its place. Under the hood, this creates a [`HandleTemplate`](../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate") that calls [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load") at resolve time. If the asset has already been loaded, this returns the existing handle rather than loading it again.

[ⓘ](# "This example is not tested")

```rust
commands.spawn_scene(bsn! {
    Sprite { image: "player.png" }
});
```

A [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") must also derive [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") to accept asset paths:

[ⓘ](# "This example is not tested")

```rust
#[derive(Component, FromTemplate)]
struct Icon {
    image: Handle<Image>,
    tint: Color,
}

// "icon.png" is converted to a HandleTemplate<Image> via implicit .into()
commands.spawn_scene(bsn! {
    Icon { image: "icon.png", tint: Color::WHITE }
});
```

### Observers

Use [`on()`](../prelude/fn.on.html "fn bevy::prelude::on") inside \[`bsn!`\] to attach an entity [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer"). Entity observers are closures or functions which fire when a given [`EntityEvent`](../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") is triggered and targets this entity. The first parameter’s type determines which event is observed. Multiple observers can be added to the same entity, and the observer has full access to the ECS via [system parameters](../ecs/system/index.html#system-parameter-list "mod bevy::ecs::system"):

[ⓘ](# "This example is not tested")

```rust
#[derive(EntityEvent)]
struct Damage {
    entity: Entity,
    amount: u32,
}

#[derive(EntityEvent)]
struct Heal {
    entity: Entity,
    amount: u32,
}

fn player() -> impl Scene {
    bsn! {
        Health { max: 100, current: 100 }
        // Each `on(...)` attaches a separate observer.
        on(|damage: On<Damage>, mut query: Query<&mut Health>| {
            let mut health = query.get_mut(damage.entity).unwrap();
            health.current = health.current.saturating_sub(damage.amount);
        })
        on(on_heal)
    }
}

fn on_heal(heal: On<Heal>, query: Query<&mut Health>){
    let mut health = query.get_mut(heal.entity).unwrap();
    health.current = (health.current + heal.amount).min(health.max);
}
```

This is useful for self-contained logic like click handlers, damage reactions, or scripting-style triggers. Closures passed to [`on`](../prelude/fn.on.html "fn bevy::prelude::on") work like any Rust closure: you can use [`move`](https://doc.rust-lang.org/std/keyword.move.html) and capture variables from the enclosing scope normally.

### Using Dynamic Expressions in Scenes

The \[`bsn!`\] macro is not limited to static data. Because scene functions are plain Rust functions, you can accept parameters and capture variables from the enclosing scope. Use `{...}` (curly braces) anywhere a value is expected to embed an arbitrary Rust expression:

[ⓘ](# "This example is not tested")

```rust
fn enemy(hp: u32, name: &str) -> impl Scene {
    let name_string = name.to_string();
    bsn! {
        #{name}
        Health { current: {hp / 2}, max: hp }
        Sprite { image: {name_string + ".png"} }
    }
}

// Call it like an ordinary Rust function
commands.spawn_scene(bsn! { enemy(200, "goblin") });
```

Braces are required when the macro would otherwise misparse the expression and for complex expressions like `{hp * 2}`.

#### Dynamic template values

A [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") value, such as an instance of a Component, cannot be directly passed in to a `bsn!` block, as `bsn!` expects “scene variables” in that position. Instead use `template_value(...)` which accepts a given component [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") value and returns a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") implementation for it.

[ⓘ](# "This example is not tested")

```rust
fn enemy(translation: Vec3){
    let transform = Transform::from_translation(translation);
    bsn! {
        #Foo
        template_value(transform)
    }

}
```

#### Ad-hoc template functions

Sometimes you need custom behavior or world access to create a [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template"). If this is the case, you can use [`template`](../prelude/fn.template.html "fn bevy::prelude::template") instead of a custom [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") or [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") implementation. In [`template`](../prelude/fn.template.html "fn bevy::prelude::template") you get access to a [`TemplateContext`](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext") which contains the [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") and a collection of named entity references.

[ⓘ](# "This example is not tested")

```rust
bsn! {
    #Foo
    template(|ctx| {
        Foo(ctx.resource::<MyAssetCollection>().get("generated_asset_name"))
    })
}
```

#### Expressions as scenes

You can insert a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") or [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") in another Scene using curly-bracketed expressions:

[ⓘ](# "This example is not tested")

```rust
fn container(contents: impl SceneList) -> impl Scene {
    bsn! {
        Children [
            #Header,
            {contents},
            #Footer,
        ]
    }
}

let items = bsn_list![#A, #B, #C]; // or bsn! if container takes a `impl Scene`
commands.spawn_scene(container(items));
```

#### Conditional values

There is no `if`/`match` syntax inside the \[`bsn!`\] grammar (yet!), but you can embed conditionals via `{...}` blocks or handle them outside the macro:

[ⓘ](# "This example is not tested")

```rust
fn unit(is_boss: bool) -> impl Scene {
    let hp = if is_boss { 500 } else { 100 };
    bsn! { Health { current: hp, max: hp } }
}
```

One way to achieve conditional scenes is using a [`Box<dyn Scene>`](../prelude/struct.Box.html "struct bevy::prelude::Box") to store different scenes in one variable.

[ⓘ](# "This example is not tested")

```rust
fn unit(is_boss: bool, level: u32) -> impl Scene {
    let scene: Box<dyn Scene> = if is_boss {
        Box::new(bsn! {
            Boss
            Followers [ // the boss is followed by some grunts
                :unit(false, level - 1) #Grunt1,
                :unit(false, level - 2) #Grunt2
            ]
        })
    } else {
        Box::new(bsn! { Grunt })
    };
    bsn! {
        Level(level)
        {scene}
    }
}
```

We plan on making “conditional scenes” easier to define in future releases.

### Scene Components

A [`SceneComponent`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") is a specialized type of [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") that has an associated [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"):

```rust
#[derive(SceneComponent, Default, Clone)]
struct Player {
    score: usize
}

impl Player {
    fn scene() -> impl Scene {
        bsn! {
            #Player
            Children [
                #RightHand Sword,
                #LeftHand Shield,
            ]
        }
    }
}
```

This enables including the [`SceneComponent`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent") as a scene, using the following syntax:

```rust
world.spawn_scene(bsn! {
 @Player { score: 0 }
});
```

This will spawn the `Player` component _and_ the entire scene with it. This means that you write systems that query for the `Player` component, they can generally assume the rest of the scene will be there too!

[`SceneComponent`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent")s can only be spawned using scene APIs like [`World::spawn_scene`](../prelude/trait.WorldSceneExt.html#tymethod.spawn_scene "method bevy::prelude::WorldSceneExt::spawn_scene"). Spawning them using [`World::spawn`](../prelude/struct.World.html#method.spawn "method bevy::prelude::World::spawn") will log an error.

#### Custom Scene Functions

When deriving [`SceneComponent`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent"), it defaults to using `Self::scene` as the “scene function”. Scene functions can also be manually specified:

```rust
#[derive(SceneComponent, Default, Clone)]
#[scene(player)]
struct Player;

fn player() -> impl Scene {
   bsn! { /* scene here */}
}
```

#### `SceneComponent` Asset Paths

Note: Currently, Bevy does not include a `.bsn` asset format. These docs exist to help you understand what is planned, and what is currently possible with third-party asset formats.

Alternatively, a scene asset path can be specified:

```rust
#[derive(SceneComponent, Default, Clone)]
#[scene("player.bsn")]
struct Player {
    score: usize
}
```

#### Scene Components are Template-able

Just like other [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")s, [`SceneComponent`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent")s are “template-able”

```rust
#[derive(SceneComponent, FromTemplate)]
struct Player {
    image: Handle<Image>,
}

impl Player {
    fn scene() -> impl Scene {
        bsn! { /* scene here */}
    }
}

world.spawn_scene(bsn! {
   @Player { image: "player.png" }
});
```

#### `SceneComponent` Props

Sometimes it is desirable to “parameterize” a scene: pass in values to the scene which determine what the scene outputs are. The answer to this in BSN is “scene props”:

```rust
/// A UI widget that repeats "hello" text a given number of times.
#[derive(SceneComponent, Default, Clone)]
#[scene(HelloRepeaterProps)]
struct HelloRepeater;

#[derive(Default)]
struct HelloRepeaterProps {
    repeat: usize,
}

impl HelloRepeater {
    fn scene(props: HelloRepeaterProps) -> impl Scene {
        let hellos = (0..props.repeat)
            .map(|_| bsn! { Text("hello") })
            .collect::<Vec<_>>();
        bsn! {
            Node
            Children [
                {hellos}
            ]
        }
    }
}

world.spawn_scene(bsn! {
   @HelloRepeater {
       @repeat: 5
   }
});
```

Notice the `@field` syntax, which specifies that a prop is being set instead of a field. Props are evaluated “immediately” when the scene is included in another scene. This means that they are not “patchable”, as at that point they have already been evaluated, and they _produce_ “patchable” outputs.

You can set _both_ props and normal fields at the same time:

```rust
#[derive(SceneComponent, Default, Clone)]
#[scene(WidgetProps)]
struct Widget {
    value: usize
}

#[derive(Default)]
struct WidgetProps {
    border: bool,
}

world.spawn_scene(bsn! {
   @Widget {
       @border: true,
       value: 10,
   }
});
```

#### The Scene Component is Always Added

Specifying the scene component manually in the scene function is not necessary. It will be added automatically:

```rust
#[derive(SceneComponent, Default, Clone)]
struct Player;

impl Player {
    fn scene() -> impl Scene {
        bsn! {
            // No need to specify a Player component here.
            // It is implied!
        }
    }
}
```

However you _can_ patch the scene component in the scene if you would like. This comes in handy if you would like props to contribute to the scene component’s fields:

```rust
impl Player {
    fn scene(props: PlayerProps) -> impl Scene {
        bsn! {
            Player {
                size_in_meters: {props.size_in_millimeters / 1000. }
            }
        }
    }
}
```

#### Scene Components vs Required Components

At first glance, Scene Components and [Required Components](../prelude/trait.Component.html "trait bevy::prelude::Component") solve similar problems. They both provide a mechanism to initialize components with other components.

They are functionally quite different however. It is worth understanding the differences and tradeoffs:

*   **Required Components**: Context-less (ex: Default constructors), non-hierarchical, can always be applied immediately, not dependency aware, automatically enforced at runtime as components are added, not patchable, pretty low overhead, not a lot of features / functionality
*   **Scene Components**: Require context (ex: World access and “Entity Spawn Context”, such as entity references), hierarchical (spawn children), cannot always be applied immediately (can have dependencies that aren’t loaded yet), dependency aware, only enforced at spawn time, patchable, more dynamic / higher overhead, many features.

Some good rules of thumb:

*   Are you building something “hierarchical” / with related entities? Use [`SceneComponent`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent").
*   Do you want or need the full capabilities of the scene system? Use [`SceneComponent`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent").
*   Are you spawning something that has dependencies / needs World access? use [`SceneComponent`](../prelude/trait.SceneComponent.html "trait bevy::prelude::SceneComponent").
*   Are you defining “flat” components that aren’t really scenes on their own? Use required components.
*   Do you need the “required” components to be automatically added in non-scene contexts? Use required components.
*   Is spawn performance a very high priority? Use required components.

### .bsn Asset Format

Bevy does not currently have support for `.bsn` files, but intends to offer a `.bsn` asset format in future releases.

This would allow you to define your scenes on disk, creating/modifying them in various authoring tools and using asset hot-reloading.

This format is intended to have broad syntactic compatibility with the `bsn!` macro, making it easy to port your content between both the macro and the asset form.

When planning your future use of `.bsn` asset files (which are not currently shipped), be aware that unlike `bsn!` macro calls `.bsn` assets will not support expressions or other dynamic features directly.

For now, you should use existing non-Bevy asset formats like glTF, search for ecosystem implementations or stick to `bsn!` macro calls.

Note that the architecture to support an asset format already exists, allowing community implementations/experimentation until an official version exists. An example of how to go about this can be found in the [scene benchmarks](https://github.com/bevyengine/bevy/blob/v0.19.0/benches/benches/bevy_scene/spawn.rs#L414)

## Modules

[macro\_utils](macro_utils/index.html "mod bevy::scene::macro_utils")

Functionality used by the \[`bsn!`\] macro.

[prelude](prelude/index.html "mod bevy::scene::prelude")

The Bevy Scene prelude.

## Macros

[bsn\_list](macro.bsn_list.html "macro bevy::scene::bsn_list")

Creates a `SceneList` using BSN (Bevy Scene Notation) syntax.

## Structs

[CachedSceneAsset](struct.CachedSceneAsset.html "struct bevy::scene::CachedSceneAsset")

A [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") that will include the cached [`ScenePatch`](struct.ScenePatch.html "struct bevy::scene::ScenePatch") stored at the given [`AssetPath`](../asset/struct.AssetPath.html "struct bevy::asset::AssetPath"). This will _not_ resolve the cached scene directly on top of this [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"). Instead it will set [`ResolvedScene::include_cached`](struct.ResolvedScene.html#method.include_cached "method bevy::scene::ResolvedScene::include_cached"), which (when spawning the [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")) will apply the cached [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") first. _Then_ the top-level [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") will be applied.

[EntityScene](struct.EntityScene.html "struct bevy::scene::EntityScene")

Corresponds to a single member of a [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") (an [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") with an `S` [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene")).

[InitTemplate](struct.InitTemplate.html "struct bevy::scene::InitTemplate")

A [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") that initializes a template if it doesn’t yet exist.

[NameEntityReference](struct.NameEntityReference.html "struct bevy::scene::NameEntityReference")

Sets up a given name as an “entity reference” for the current entity. This pairs the [`Self::name`](struct.NameEntityReference.html#structfield.name "field bevy::scene::NameEntityReference::name") field to a given [`Self::reference`](struct.NameEntityReference.html#structfield.reference "field bevy::scene::NameEntityReference::reference") field.

[OnTemplate](struct.OnTemplate.html "struct bevy::scene::OnTemplate")

A [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") / [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") that will create an [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer") of a given [`EntityEvent`](../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") on the current [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") entity. This is typically initialized using the [`on()`](../prelude/fn.on.html "fn bevy::prelude::on") function, which returns an [`OnTemplate`](struct.OnTemplate.html "struct bevy::scene::OnTemplate").

[QueuedScenes](struct.QueuedScenes.html "struct bevy::scene::QueuedScenes")

A [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") that tracks entities / scenes that have been queued to spawn.

[RelatedResolvedScenes](struct.RelatedResolvedScenes.html "struct bevy::scene::RelatedResolvedScenes")

A collection of [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")s that are related to a given [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") by a [`Relationship`](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"). Each [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") added here will be spawned as a new [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") when the “parent” [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") is spawned.

[RelatedScenes](struct.RelatedScenes.html "struct bevy::scene::RelatedScenes")

A [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") that adds an `L` [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") as “related scenes”, using the `R` [`Relationship`](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship")

[ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")

Context used by [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") implementations during [`Scene::resolve`](../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve").

[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")

A final resolved scene (usually produced by calling [`Scene::resolve`](../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve")). This consists of:

[ResolvedSceneListRoot](struct.ResolvedSceneListRoot.html "struct bevy::scene::ResolvedSceneListRoot")

A final “spawnable” root list of [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")s.

[ResolvedSceneRoot](struct.ResolvedSceneRoot.html "struct bevy::scene::ResolvedSceneRoot")

A final “spawnable” root [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene").

[SceneComponentInfo](struct.SceneComponentInfo.html "struct bevy::scene::SceneComponentInfo")

Indicates that this entity includes a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") that must always be spawned with a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene").

[SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies")

A collection of asset dependencies required by a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene").

[SceneDependency](struct.SceneDependency.html "struct bevy::scene::SceneDependency")

An asset dependency of a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene").

[SceneFunction](struct.SceneFunction.html "struct bevy::scene::SceneFunction")

A [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") that uses a function `F` to perform arbitrary [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") logic.

[SceneListPatch](struct.SceneListPatch.html "struct bevy::scene::SceneListPatch")

An [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") that holds a [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), tracks its dependencies, and holds a [`ResolvedSceneListRoot`](struct.ResolvedSceneListRoot.html "struct bevy::scene::ResolvedSceneListRoot") (after the [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") has been loaded and resolved)

[SceneListScope](struct.SceneListScope.html "struct bevy::scene::SceneListScope")

A [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") that will create a new “entity scope” and fully resolve the given scene list `L` on top of the current [`Vec<ResolvedScene>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec") (using that scope). It is not cached.

[ScenePatch](struct.ScenePatch.html "struct bevy::scene::ScenePatch")

An [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") that holds a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"), tracks its dependencies, and holds the [`ResolvedSceneRoot`](struct.ResolvedSceneRoot.html "struct bevy::scene::ResolvedSceneRoot") (after the [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") has been loaded and resolved).

[ScenePatchInstance](struct.ScenePatchInstance.html "struct bevy::scene::ScenePatchInstance")

A component that, when added, will queue applying the given [`ScenePatch`](struct.ScenePatch.html "struct bevy::scene::ScenePatch") after the scene and its dependencies have been loaded and resolved.

[ScenePatchInstanceTemplate](struct.ScenePatchInstanceTemplate.html "struct bevy::scene::ScenePatchInstanceTemplate")

[ScenePlugin](struct.ScenePlugin.html "struct bevy::scene::ScenePlugin")

Adds support for spawning Bevy Scenes. See [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"), [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), [`ScenePatch`](struct.ScenePatch.html "struct bevy::scene::ScenePatch"), and the \[`bsn!`\] macro for more information.

[SceneScope](struct.SceneScope.html "struct bevy::scene::SceneScope")

A [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") that will create a new “entity scope” and fully resolve the given scene `S` on top of the current [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") (using that scope). It is not cached.

[TemplatePatch](struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")

A [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") that patches a [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") of type `T` with a given function `F`.

[WaitingScenes](struct.WaitingScenes.html "struct bevy::scene::WaitingScenes")

A [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") that tracks entities / scenes that are waiting for an asset to load

## Enums

[ApplySceneError](enum.ApplySceneError.html "enum bevy::scene::ApplySceneError")

An error produced when applying a [`ResolvedScene`](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene").

[CachedSceneError](enum.CachedSceneError.html "enum bevy::scene::CachedSceneError")

The error returned by [`ResolvedScene::include_cached`](struct.ResolvedScene.html#method.include_cached "method bevy::scene::ResolvedScene::include_cached").

[ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")

An [`Error`](https://docs.rs/thiserror-impl/1.0.69/x86_64-unknown-linux-gnu/thiserror_impl/derive.Error.html "derive thiserror_impl::Error") that occurs during [`Scene::resolve`](../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve").

[SpawnSceneError](enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError")

An [`Error`](https://docs.rs/thiserror-impl/1.0.69/x86_64-unknown-linux-gnu/thiserror_impl/derive.Error.html "derive thiserror_impl::Error") that occurs during scene spawning.

## Traits

[CommandsSceneExt](trait.CommandsSceneExt.html "trait bevy::scene::CommandsSceneExt")

Adds scene spawning functionality to [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands").

[EntityCommandsSceneExt](trait.EntityCommandsSceneExt.html "trait bevy::scene::EntityCommandsSceneExt")

Adds scene functionality to [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

[EntityWorldMutSceneExt](trait.EntityWorldMutSceneExt.html "trait bevy::scene::EntityWorldMutSceneExt")

Adds scene functionality to [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

[ErasedBundleTemplate](trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate")

A type-erased, object-safe, downcastable version of [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that produces a [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), which will be added immediately to a given `entity`.

[ErasedComponentTemplate](trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate")

A type-erased, object-safe, downcastable version of [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that produces a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component"), which will be added to the given [`BundleWriter`](../ecs/bundle/struct.BundleWriter.html "struct bevy::ecs::bundle::BundleWriter").

[PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::scene::PatchFromTemplate")

A helper function that returns a [`TemplatePatch`](struct.TemplatePatch.html "struct bevy::scene::TemplatePatch") [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for something that implements [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"). It will use [`FromTemplate::Template`](../prelude/trait.FromTemplate.html#associatedtype.Template "associated type bevy::prelude::FromTemplate::Template") as the “patched template”.

[PatchTemplate](trait.PatchTemplate.html "trait bevy::scene::PatchTemplate")

A helper function that returns a [`TemplatePatch`](struct.TemplatePatch.html "struct bevy::scene::TemplatePatch") [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for something that implements [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template").

[Scene](trait.Scene.html "trait bevy::scene::Scene")

Conceptually, a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") describes what a spawned [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") should look like. This often describes what [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")s the entity should have.

[SceneBox](trait.SceneBox.html "trait bevy::scene::SceneBox")

Boxed version of [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"), which enables implementing [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [`Box<dyn Scene>`](../prelude/struct.Box.html "struct bevy::prelude::Box"). Most developers do not need to think about or use this trait.

[SceneComponent](trait.SceneComponent.html "trait bevy::scene::SceneComponent")

Implemented for [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component")s that have an associated [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"), which can be constructed with [`Self::Props`](../prelude/trait.SceneComponent.html#associatedtype.Props "associated type bevy::prelude::SceneComponent::Props").

[SceneList](trait.SceneList.html "trait bevy::scene::SceneList")

This behaves like a list of [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"), where each entry in the list is a new entity (see [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for more details).

[SceneListBox](trait.SceneListBox.html "trait bevy::scene::SceneListBox")

Boxed version of [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), which enables implementing [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [`Box<dyn SceneList>`](../prelude/struct.Box.html "struct bevy::prelude::Box"). Most developers do not need to think about or use this trait.

[SpawnListSystem](trait.SpawnListSystem.html "trait bevy::scene::SpawnListSystem")

Returns a system that spawns the given [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"). This should generally only be added to schedules that run once, such as [`Startup`](../prelude/struct.Startup.html "struct bevy::prelude::Startup").

[SpawnSystem](trait.SpawnSystem.html "trait bevy::scene::SpawnSystem")

Returns a system that spawns the given [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"). This should generally only be added to schedules that run once, such as [`Startup`](../prelude/struct.Startup.html "struct bevy::prelude::Startup").

[WorldSceneExt](trait.WorldSceneExt.html "trait bevy::scene::WorldSceneExt")

Adds scene spawning functionality to [`World`](../prelude/struct.World.html "struct bevy::prelude::World").

## Functions

[on](fn.on.html "fn bevy::scene::on")

Returns an [`OnTemplate`](struct.OnTemplate.html "struct bevy::scene::OnTemplate") that will create an [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer") of a given [`EntityEvent`](../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") on the current [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") entity.

[on\_add\_scene\_patch\_instance](fn.on_add_scene_patch_instance.html "fn bevy::scene::on_add_scene_patch_instance")

An [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer") system that queues newly added [`ScenePatchInstance`](../prelude/struct.ScenePatchInstance.html "struct bevy::prelude::ScenePatchInstance") entities.

[resolve\_scene\_patches](fn.resolve_scene_patches.html "fn bevy::scene::resolve_scene_patches")

A [`System`](../prelude/trait.System.html "trait bevy::prelude::System") that resolves [`ScenePatch`](struct.ScenePatch.html "struct bevy::scene::ScenePatch") and [`SceneListPatch`](struct.SceneListPatch.html "struct bevy::scene::SceneListPatch") assets whose dependencies have been fully loaded.

[spawn\_queued](fn.spawn_queued.html "fn bevy::scene::spawn_queued")

A system that spawns queued scenes when they are loaded.

[template\_value](fn.template_value.html "fn bevy::scene::template_value")

Returns a [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") that completely overwrites the current value of a [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") `T` with the given `value`. The `value` is cloned each time the [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") is built.

## Derive Macros

[SceneComponent](derive.SceneComponent.html "derive bevy::scene::SceneComponent")