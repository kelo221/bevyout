[bevy](../index.html)::[prelude](index.html)

# Trait ObserverSystemExt 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#621)

```rust
pub trait ObserverSystemExt<E, B, M>: Sized + IntoObserverSystem<E, B, M>where
    E: Event,
    B: Bundle,{
    // Provided method
    fn run_if<C, CM>(self, condition: C) -> ObserverWithCondition<E, B, M, Self>
       where C: SystemCondition<CM> { ... }
}
```

Extension trait for adding run conditions to observer systems.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#626-628)

#### fn [run\_if](#method.run_if)<C, CM>(self, condition: C) -> [ObserverWithCondition](../ecs/observer/struct.ObserverWithCondition.html "struct bevy::ecs::observer::ObserverWithCondition")<E, B, M, Self>

where C: [SystemCondition](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<CM>,

Adds a run condition to this observer system.

The observer will only run if the condition returns `true`. Multiple conditions can be chained (AND semantics).

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/ecs/observers.rs ([line 36](../../src/observers/observers.rs.html#36))

```rust
8fn main() {
9    App::new()
10        .add_plugins(DefaultPlugins)
11        .init_resource::<SpatialIndex>()
12        .init_resource::<ExplosionsEnabled>()
13        .add_systems(Startup, setup)
14        .add_systems(Update, (draw_shapes, handle_click, toggle_explosions))
15        // Observers are systems that run when an event is "triggered". This observer runs whenever
16        // `ExplodeMines` is triggered.
17        //
18        // Observers can have run conditions, just like systems! This observer only runs when
19        // explosions are enabled. Press Space to toggle.
20        .add_observer(
21            (|explode_mines: On<ExplodeMines>,
22              mines: Query<&Mine>,
23              index: Res<SpatialIndex>,
24              mut commands: Commands| {
25                // Access resources
26                for entity in index.get_nearby(explode_mines.pos) {
27                    // Run queries
28                    let mine = mines.get(entity).unwrap();
29                    if mine.pos.distance(explode_mines.pos) < mine.size + explode_mines.radius {
30                        // And queue commands, including triggering additional events
31                        // Here we trigger the `Explode` event for entity `e`
32                        commands.trigger(Explode { entity });
33                    }
34                }
35            })
36            .run_if(|enabled: Res<ExplosionsEnabled>| enabled.0),
37        )
38        // This observer runs whenever the `Mine` component is added to an entity, and places it in a simple spatial index.
39        .add_observer(on_add_mine)
40        // This observer runs whenever the `Mine` component is removed from an entity (including despawning it)
41        // and removes it from the spatial index.
42        .add_observer(on_remove_mine)
43        .run();
44}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#638)

### impl<E, B, M, T> [ObserverSystemExt](trait.ObserverSystemExt.html "trait bevy::prelude::ObserverSystemExt")<E, B, M> for T

where E: [Event](trait.Event.html "trait bevy::prelude::Event"), B: [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"), T: [IntoObserverSystem](../ecs/system/trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem")<E, B, M>,