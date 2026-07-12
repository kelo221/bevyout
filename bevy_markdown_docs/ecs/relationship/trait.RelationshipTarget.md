[bevy](../../index.html)::[ecs](../index.html)::[relationship](index.html)

# Trait RelationshipTarget 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#270)

```rust
pub trait RelationshipTarget: Sized + Component<Mutability = Mutable> {
    type Relationship: Relationship<RelationshipTarget = Self>;
    type Collection: RelationshipSourceCollection;

    const LINKED_SPAWN: bool;

    // Required methods
    fn collection(&self) -> &Self::Collection;
    fn collection_mut_risky(&mut self) -> &mut Self::Collection;
    fn from_collection_risky(collection: Self::Collection) -> Self;

    // Provided methods
    fn on_discard(world: DeferredWorld<'_>, _: HookContext) { ... }
    fn on_despawn(world: DeferredWorld<'_>, _: HookContext) { ... }
    fn with_capacity(capacity: usize) -> Self { ... }
    fn iter(
        &self,
    ) -> <Self::Collection as RelationshipSourceCollection>::SourceIter<'_> { ... }
    fn len(&self) -> usize { ... }
    fn is_empty(&self) -> bool { ... }
}
```

A [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") containing the collection of entities that relate to this [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") via the associated `Relationship` type. See the [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") documentation for more information.

## Required Associated Constants

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#278)

#### const [LINKED\_SPAWN](#associatedconstant.LINKED_SPAWN): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

If this is true, when despawning or cloning (when [linked cloning is enabled](../entity/struct.EntityClonerBuilder.html#method.linked_cloning "method bevy::ecs::entity::EntityClonerBuilder::linked_cloning")), the related entities targeting this entity will also be despawned or cloned.

For example, this is set to `true` for Bevy’s built-in parent-child relation, defined by [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") and [`Children`](../../prelude/struct.Children.html "struct bevy::prelude::Children"). This means that when a parent is despawned, any children targeting that parent are also despawned (and the same applies to cloning).

To get around this behavior, you can first break the relationship between entities, and _then_ despawn or clone. This defaults to false when derived.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#280)

#### type [Relationship](#associatedtype.Relationship): [Relationship](trait.Relationship.html "trait bevy::ecs::relationship::Relationship")<RelationshipTarget = Self>

The [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") that populates this [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#287)

#### type [Collection](#associatedtype.Collection): [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection")

The collection type that stores the “source” entities for this [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component.

Check the list of types which implement [`RelationshipSourceCollection`](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") for the data structures that can be used inside of your component. If you need a new collection type, you can implement the [`RelationshipSourceCollection`](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") trait for a type you own which wraps the collection you want to use (to avoid the orphan rule), or open an issue on the Bevy repository to request first-party support for your collection type.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#290)

#### fn [collection](#tymethod.collection)(&self) -> &Self::[Collection](../../prelude/trait.RelationshipTarget.html#associatedtype.Collection "type bevy::prelude::RelationshipTarget::Collection")

Returns a reference to the stored [`RelationshipTarget::Collection`](../../prelude/trait.RelationshipTarget.html#associatedtype.Collection "associated type bevy::prelude::RelationshipTarget::Collection").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#296)

#### fn [collection\_mut\_risky](#tymethod.collection_mut_risky)(&mut self) -> &mut Self::[Collection](../../prelude/trait.RelationshipTarget.html#associatedtype.Collection "type bevy::prelude::RelationshipTarget::Collection")

Returns a mutable reference to the stored [`RelationshipTarget::Collection`](../../prelude/trait.RelationshipTarget.html#associatedtype.Collection "associated type bevy::prelude::RelationshipTarget::Collection").

##### Warning

This should generally not be called by user code, as modifying the internal collection could invalidate the relationship. The collection should not contain duplicates.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#303)

#### fn [from\_collection\_risky](#tymethod.from_collection_risky)(collection: Self::[Collection](../../prelude/trait.RelationshipTarget.html#associatedtype.Collection "type bevy::prelude::RelationshipTarget::Collection")) -> Self

Creates a new [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") from the given [`RelationshipTarget::Collection`](../../prelude/trait.RelationshipTarget.html#associatedtype.Collection "associated type bevy::prelude::RelationshipTarget::Collection").

##### Warning

This should generally not be called by user code, as constructing the internal collection could invalidate the relationship. The collection should not contain duplicates.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#307-314)

#### fn [on\_discard](#method.on_discard)(world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, \_: [HookContext](../lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))

The `on_discard` component hook that maintains the [`Relationship`](trait.Relationship.html "trait bevy::ecs::relationship::Relationship") / [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") connection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#332)

#### fn [on\_despawn](#method.on_despawn)(world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, \_: [HookContext](../lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))

The `on_despawn` component hook that despawns entities stored in an entity’s [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") when that entity is despawned.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#341)

#### fn [with\_capacity](#method.with_capacity)(capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> Self

Creates this [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") with the given pre-allocated entity capacity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#349)

#### fn [iter](#method.iter)( &self, ) -> <Self::[Collection](../../prelude/trait.RelationshipTarget.html#associatedtype.Collection "type bevy::prelude::RelationshipTarget::Collection") as [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection")\>::[SourceIter](trait.RelationshipSourceCollection.html#associatedtype.SourceIter "type bevy::ecs::relationship::RelationshipSourceCollection::SourceIter")<'\_>

Iterates the entities stored in this collection.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ui/text/text\_background\_colors.rs ([line 95](../../../src/text_background_colors/text_background_colors.rs.html#95))

```rust
87fn cycle_text_background_colors(
88    time: Res<Time>,
89    children_query: Query<&Children, With<Text>>,
90    mut text_background_colors_query: Query<&mut TextBackgroundColor>,
91) {
92    let n = time.elapsed_secs() as usize;
93    let children = children_query.single().unwrap();
94
95    for (i, child) in children.iter().enumerate() {
96        text_background_colors_query.get_mut(child).unwrap().0 = PALETTE[(i + n) % PALETTE.len()];
97    }
98}
```

Hide additional examples

examples/usage/context\_menu.rs ([line 52](../../../src/context_menu/context_menu.rs.html#52))

```rust
40fn text_color_on_hover<T: Debug + Clone + Reflect>(
41    color: Color,
42) -> impl FnMut(On<Pointer<T>>, Query<&mut TextColor>, Query<&Children>) {
43    move |mut event: On<Pointer<T>>,
44          mut text_color: Query<&mut TextColor>,
45          children: Query<&Children>| {
46        let Ok(children) = children.get(event.original_event_target()) else {
47            return;
48        };
49        event.propagate(false);
50
51        // find the text among children and change its color
52        for child in children.iter() {
53            if let Ok(mut col) = text_color.get_mut(child) {
54                col.0 = color;
55            }
56        }
57    }
58}
```

examples/ui/widgets/standard\_widgets.rs ([line 796](../../../src/standard_widgets/standard_widgets.rs.html#796))

```rust
787fn on_menu_event(
788    menu_event: On<MenuEvent>,
789    q_anchor: Single<(Entity, &Children), With<DemoMenuAnchor>>,
790    q_popup: Query<Entity, With<MenuPopup>>,
791    assets: Res<AssetServer>,
792    mut focus: ResMut<InputFocus>,
793    mut commands: Commands,
794) {
795    let (anchor, children) = q_anchor.into_inner();
796    let popup = children.iter().find_map(|c| q_popup.get(c).ok());
797    info!("Menu action: {:?}", menu_event.action);
798    match menu_event.action {
799        MenuAction::Open(_) => {
800            if popup.is_none() {
801                spawn_menu(anchor, assets, commands);
802            }
803        }
804        MenuAction::Toggle => match popup {
805            Some(popup) => commands.entity(popup).despawn(),
806            None => spawn_menu(anchor, assets, commands),
807        },
808        MenuAction::CloseAll => {
809            if let Some(popup) = popup {
810                commands.entity(popup).despawn();
811            }
812        }
813        MenuAction::FocusRoot => {
814            focus.set(anchor, FocusCause::Navigated);
815        }
816    }
817}
```

examples/3d/motion\_blur.rs ([line 318](../../../src/motion_blur/motion_blur.rs.html#318))

```rust
300fn move_cars(
301    time: Res<Time>,
302    mut movables: Query<(&mut Transform, &Moves, &Children)>,
303    mut spins: Query<&mut Transform, (Without<Moves>, With<Rotates>)>,
304) {
305    for (mut transform, moves, children) in &mut movables {
306        let time = time.elapsed_secs() * 0.25;
307        let t = time + 0.5 * moves.0;
308        let dx = ops::cos(t);
309        let dz = -ops::sin(3.0 * t);
310        let speed_variation = (dx * dx + dz * dz).sqrt() * 0.15;
311        let t = t + speed_variation;
312        let prev = transform.translation;
313        transform.translation.x = race_track_pos(0.0, t).x;
314        transform.translation.z = race_track_pos(0.0, t).y;
315        transform.translation.y = -0.59;
316        let delta = transform.translation - prev;
317        transform.look_to(delta, Vec3::Y);
318        for child in children.iter() {
319            let Ok(mut wheel) = spins.get_mut(child) else {
320                continue;
321            };
322            let radius = wheel.scale.x;
323            let circumference = 2.0 * std::f32::consts::PI * radius;
324            let angle = delta.length() / circumference * std::f32::consts::PI * 2.0;
325            wheel.rotate_local_y(angle);
326        }
327    }
328}
```

examples/ecs/relationships.rs ([line 90](../../../src/relationships/relationships.rs.html#90))

```rust
78    fn debug_relationships(
79        // Not all of our entities are targeted by something, so we use `Option` in our query to handle this case.
80        relations_query: Query<(&Name, &Targeting, Option<&TargetedBy>)>,
81        name_query: Query<&Name>,
82    ) {
83        let mut relationships = String::new();
84
85        for (name, targeting, maybe_targeted_by) in relations_query.iter() {
86            let targeting_name = name_query.get(targeting.0).unwrap();
87            let targeted_by_string = if let Some(targeted_by) = maybe_targeted_by {
88                let mut vec_of_names = Vec::<&Name>::new();
89
90                for entity in targeted_by.iter() {
91                    let name = name_query.get(entity).unwrap();
92                    vec_of_names.push(name);
93                }
94
95                // Convert this to a nice string for printing.
96                let vec_of_str: Vec<&str> = vec_of_names.iter().map(|name| name.as_str()).collect();
97                vec_of_str.join(", ")
98            } else {
99                "nobody".to_string()
100            };
101
102            relationships.push_str(&format!(
103                "{name} is targeting {targeting_name}, and is targeted by {targeted_by_string}\n",
104            ));
105        }
106
107        println!("{relationships}");
108    }
```

examples/3d/ssr.rs ([line 845](../../../src/ssr/ssr.rs.html#845))

```rust
677fn adjust_app_settings(
678    mut commands: Commands,
679    mut app_settings: ResMut<AppSettings>,
680    mut cameras: Query<Entity, With<Camera>>,
681    mut visibilities: Query<&mut Visibility>,
682    model_queries: ModelQueries,
683    mut widget_click_events: MessageReader<WidgetClickEvent<ExampleSetting>>,
684    mut background_colors: Query<&mut BackgroundColor>,
685    radio_buttons: Query<
686        (
687            Entity,
688            Has<BackgroundColor>,
689            Has<Text>,
690            &WidgetClickSender<ExampleSetting>,
691        ),
692        Or<(With<RadioButton>, With<RadioButtonText>)>,
693    >,
694    range_value_text: Query<(Entity, &RangeValueText)>,
695    text_children: Query<&Children>,
696    mut writer: TextUiWriter,
697    text_query: Query<Entity, With<Text>>,
698) {
699    let mut any_changes = false;
700
701    for event in widget_click_events.read() {
702        any_changes = true;
703        match **event {
704            ExampleSetting::Ssr(on) => app_settings.ssr_on = on,
705            ExampleSetting::Model(model) => app_settings.displayed_model = model,
706            ExampleSetting::Base(base) => app_settings.displayed_base = base,
707            ExampleSetting::MinRoughnessStart(adj) => {
708                app_settings.min_perceptual_roughness.start =
709                    adjust(app_settings.min_perceptual_roughness.start, adj, 0.005);
710            }
711            ExampleSetting::MinRoughnessEnd(adj) => {
712                app_settings.min_perceptual_roughness.end =
713                    adjust(app_settings.min_perceptual_roughness.end, adj, 0.005);
714            }
715            ExampleSetting::MaxRoughnessStart(adj) => {
716                app_settings.max_perceptual_roughness.start =
717                    adjust(app_settings.max_perceptual_roughness.start, adj, 0.005);
718            }
719            ExampleSetting::MaxRoughnessEnd(adj) => {
720                app_settings.max_perceptual_roughness.end =
721                    adjust(app_settings.max_perceptual_roughness.end, adj, 0.005);
722            }
723            ExampleSetting::EdgeFadeoutStart(adj) => {
724                app_settings.edge_fadeout.start =
725                    adjust(app_settings.edge_fadeout.start, adj, 0.001);
726            }
727            ExampleSetting::EdgeFadeoutEnd(adj) => {
728                app_settings.edge_fadeout.end = adjust(app_settings.edge_fadeout.end, adj, 0.001);
729            }
730        }
731    }
732
733    if !any_changes {
734        return;
735    }
736
737    // Update SSR settings.
738    for camera in cameras.iter_mut() {
739        if app_settings.ssr_on {
740            commands.entity(camera).insert(ScreenSpaceReflections {
741                min_perceptual_roughness: app_settings.min_perceptual_roughness.clone(),
742                max_perceptual_roughness: app_settings.max_perceptual_roughness.clone(),
743                edge_fadeout: app_settings.edge_fadeout.clone(),
744                ..default()
745            });
746        } else {
747            commands.entity(camera).remove::<ScreenSpaceReflections>();
748        }
749    }
750
751    // Set model visibility.
752    for entity in model_queries.cube_models.iter() {
753        if let Ok(mut visibility) = visibilities.get_mut(entity) {
754            *visibility = if app_settings.displayed_model == DisplayedModel::Cube {
755                Visibility::Visible
756            } else {
757                Visibility::Hidden
758            };
759        }
760    }
761    for entity in model_queries.flight_helmet_models.iter() {
762        if let Ok(mut visibility) = visibilities.get_mut(entity) {
763            *visibility = if app_settings.displayed_model == DisplayedModel::FlightHelmet {
764                Visibility::Visible
765            } else {
766                Visibility::Hidden
767            };
768        }
769    }
770    for entity in model_queries.capsule_models.iter() {
771        if let Ok(mut visibility) = visibilities.get_mut(entity) {
772            *visibility = if app_settings.displayed_model == DisplayedModel::Capsules {
773                Visibility::Visible
774            } else {
775                Visibility::Hidden
776            };
777        }
778    }
779    for entity in model_queries.metallic_base_models.iter() {
780        if let Ok(mut visibility) = visibilities.get_mut(entity) {
781            *visibility = if app_settings.displayed_base == DisplayedBase::Metallic {
782                Visibility::Visible
783            } else {
784                Visibility::Hidden
785            };
786        }
787    }
788    for entity in model_queries.non_metallic_base_models.iter() {
789        if let Ok(mut visibility) = visibilities.get_mut(entity) {
790            *visibility = if app_settings.displayed_base == DisplayedBase::RedPlane {
791                Visibility::Visible
792            } else {
793                Visibility::Hidden
794            };
795        }
796    }
797    for entity in model_queries.water_models.iter() {
798        if let Ok(mut visibility) = visibilities.get_mut(entity) {
799            *visibility = if app_settings.displayed_base == DisplayedBase::Water {
800                Visibility::Visible
801            } else {
802                Visibility::Hidden
803            };
804        }
805    }
806
807    // Update radio buttons.
808    for (entity, has_background, has_text, sender) in radio_buttons.iter() {
809        let selected = match **sender {
810            ExampleSetting::Ssr(on) => app_settings.ssr_on == on,
811            ExampleSetting::Model(model) => app_settings.displayed_model == model,
812            ExampleSetting::Base(base) => app_settings.displayed_base == base,
813            _ => {
814                if has_background
815                    && let Ok(mut background_color) = background_colors.get_mut(entity)
816                {
817                    *background_color = BackgroundColor(Color::BLACK);
818                }
819                if has_text {
820                    update_ui_radio_button_text(entity, &mut writer, false);
821                }
822                continue;
823            }
824        };
825
826        if has_background && let Ok(mut background_color) = background_colors.get_mut(entity) {
827            update_ui_radio_button(&mut background_color, selected);
828        }
829        if has_text {
830            update_ui_radio_button_text(entity, &mut writer, selected);
831        }
832    }
833
834    // Update range value text.
835    for (parent, marker) in range_value_text.iter() {
836        let val = match marker {
837            RangeValueText::MinRoughnessStart => app_settings.min_perceptual_roughness.start,
838            RangeValueText::MinRoughnessEnd => app_settings.min_perceptual_roughness.end,
839            RangeValueText::MaxRoughnessStart => app_settings.max_perceptual_roughness.start,
840            RangeValueText::MaxRoughnessEnd => app_settings.max_perceptual_roughness.end,
841            RangeValueText::EdgeFadeoutStart => app_settings.edge_fadeout.start,
842            RangeValueText::EdgeFadeoutEnd => app_settings.edge_fadeout.end,
843        };
844        if let Ok(children) = text_children.get(parent) {
845            for child in children.iter() {
846                if text_query.get(child).is_ok() {
847                    *writer.text(child, 0) = format!("{:.2}", val);
848                    writer.for_each_color(child, |mut color| {
849                        color.0 = Color::BLACK;
850                    });
851                }
852            }
853        }
854    }
855}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#355)

#### fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of entities in this collection.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/ecs/hierarchy.rs ([line 82](../../../src/hierarchy/hierarchy.rs.html#82))

```rust
62fn rotate(
63    mut commands: Commands,
64    time: Res<Time>,
65    mut parents_query: Query<(Entity, &Children), With<Sprite>>,
66    mut transform_query: Query<&mut Transform, With<Sprite>>,
67) {
68    for (parent, children) in &mut parents_query {
69        if let Ok(mut transform) = transform_query.get_mut(parent) {
70            transform.rotate_z(-PI / 2. * time.delta_secs());
71        }
72
73        // To iterate through the entities children, just treat the Children component as a Vec
74        // Alternatively, you could query entities that have a ChildOf component
75        for child in children {
76            if let Ok(mut transform) = transform_query.get_mut(*child) {
77                transform.rotate_z(PI * time.delta_secs());
78            }
79        }
80
81        // To demonstrate removing children, we'll remove a child after a couple of seconds.
82        if time.elapsed_secs() >= 2.0 && children.len() == 2 {
83            let child = children.last().unwrap();
84            commands.entity(*child).despawn();
85        }
86
87        if time.elapsed_secs() >= 4.0 {
88            // This will remove the entity from its parent's list of children, as well as despawn
89            // any children the entity has.
90            commands.entity(parent).despawn();
91        }
92    }
93}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#361)

#### fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if this entity collection is empty.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/ui/widgets/standard\_widgets\_observers.rs ([line 171](../../../src/standard_widgets_observers/standard_widgets_observers.rs.html#171))

```rust
153fn button_on_interaction<E: EntityEvent, C: Component>(
154    event: On<E, C>,
155    mut buttons: Query<
156        (
157            &Hovered,
158            Has<InteractionDisabled>,
159            Has<Pressed>,
160            &mut BackgroundColor,
161            &mut BorderColor,
162            &Children,
163        ),
164        With<DemoButton>,
165    >,
166    mut text_query: Query<&mut Text>,
167) {
168    if let Ok((hovered, disabled, pressed, mut color, mut border_color, children)) =
169        buttons.get_mut(event.event_target())
170    {
171        if children.is_empty() {
172            return;
173        }
174        let Ok(mut text) = text_query.get_mut(children[0]) else {
175            return;
176        };
177        let hovered = hovered.get();
178        // These "removal event checks" exist because the `Remove` event is triggered _before_ the component is actually
179        // removed, meaning it still shows up in the query. We're investigating the best way to improve this scenario.
180        let pressed = pressed && !(E::is::<Remove>() && C::is::<Pressed>());
181        let disabled = disabled && !(E::is::<Remove>() && C::is::<InteractionDisabled>());
182        match (disabled, hovered, pressed) {
183            // Disabled button
184            (true, _, _) => {
185                **text = "Disabled".to_string();
186                *color = NORMAL_BUTTON.into();
187                border_color.set_all(GRAY);
188            }
189
190            // Pressed and hovered button
191            (false, true, true) => {
192                **text = "Press".to_string();
193                *color = PRESSED_BUTTON.into();
194                border_color.set_all(RED);
195            }
196
197            // Hovered, unpressed button
198            (false, true, false) => {
199                **text = "Hover".to_string();
200                *color = HOVERED_BUTTON.into();
201                border_color.set_all(WHITE);
202            }
203
204            // Unhovered button (either pressed or not).
205            (false, false, _) => {
206                **text = "Button".to_string();
207                *color = NORMAL_BUTTON.into();
208                border_color.set_all(BLACK);
209            }
210        }
211    }
212}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#147)

### impl [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") for [Children](../../prelude/struct.Children.html "struct bevy::prelude::Children")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#147)

#### const [LINKED\_SPAWN](#associatedconstant.LINKED_SPAWN): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#147)

#### type [Relationship](#associatedtype.Relationship) = [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#147)

#### type [Collection](#associatedtype.Collection) = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#59)

### impl [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") for [HasWindows](../../window/struct.HasWindows.html "struct bevy::window::HasWindows")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#59)

#### const [LINKED\_SPAWN](#associatedconstant.LINKED_SPAWN): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#59)

#### type [Relationship](#associatedtype.Relationship) = [OnMonitor](../../window/struct.OnMonitor.html "struct bevy::window::OnMonitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#59)

#### type [Collection](#associatedtype.Collection) = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>