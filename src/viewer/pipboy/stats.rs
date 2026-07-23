use super::*;

fn status_sprite(
    parent: &mut ChildSpawnerCommands,
    assets: &AssetServer,
    name: &'static str,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
) {
    parent.spawn((
        ImageNode {
            image: assets.load(format!("staging/interface/stats/{name}.ktx2")),
            color: GREEN,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(left),
            top: Val::Px(top),
            width: Val::Px(width),
            height: Val::Px(height),
            ..default()
        },
    ));
}

pub(super) fn spawn_stats_body(
    screen: &mut ChildSpawnerCommands,
    sources: &ScreenSources,
    status: &PlayerStatus,
) {
    screen
        .spawn(Node {
            flex_grow: 1.0,
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|body| {
            body.spawn(Node {
                width: Val::Percent(15.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(20.0),
                padding: UiRect::left(Val::Px(12.0)).with_top(Val::Px(40.0)),
                ..default()
            })
            .with_children(|labels| {
                for (label, selected) in [("CND", true), ("RAD", false), ("EFF", false)] {
                    labels.spawn((
                        Text::new(label),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(26.0),
                            ..default()
                        },
                        glow(),
                        Node {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(4.0)),
                            border: UiRect::all(Val::Px(if selected { 2.0 } else { 0.0 })),
                            ..default()
                        },
                        BorderColor::all(GREEN),
                    ));
                }
            });
            body.spawn(Node {
                width: Val::Percent(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|figure| {
                figure
                    .spawn(Node {
                        width: Val::Px(420.0),
                        height: Val::Px(350.0),
                        position_type: PositionType::Relative,
                        ..default()
                    })
                    .with_children(|body| {
                        for (name, left, top, width, height) in [
                            ("head", 148.0, 0.0, 123.0, 133.0),
                            ("face_00", 175.0, 48.0, 70.0, 93.0),
                            ("torso", 136.0, 112.0, 148.0, 186.0),
                            ("left_arm", 265.0, 113.0, 145.0, 75.0),
                            ("right_arm", 10.0, 108.0, 139.0, 78.0),
                            ("left_leg", 215.0, 230.0, 104.0, 120.0),
                            ("right_leg", 100.0, 228.0, 122.0, 122.0),
                        ] {
                            status_sprite(body, &sources.assets, name, left, top, width, height);
                        }
                    });
                figure.spawn((
                    Text::new(format!("{} - Level {}", status.name, status.level)),
                    TextColor(GREEN),
                    TextFont {
                        font_size: FontSize::Px(28.0),
                        ..default()
                    },
                    glow(),
                ));
            });
            body.spawn(Node {
                width: Val::Percent(25.0),
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::right(Val::Px(12.0)).with_top(Val::Px(48.0)),
                ..default()
            })
            .with_children(|quick| {
                if let Some(line) = quick_aid_line(&sources.inventory, &sources.catalog) {
                    quick.spawn((
                        Text::new(line),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                        glow(),
                    ));
                }
            });
        });
}

pub(super) fn quick_aid_line(
    inventory: &PlayerInventory,
    catalog: &PreparedItemCatalog,
) -> Option<String> {
    let mut aid: Vec<(i32, &PreparedItemDefinition)> = inventory
        .stack_states()
        .into_iter()
        .filter_map(|stack| {
            catalog
                .items
                .iter()
                .find(|item| {
                    item.base_form_id == stack.base_form_id
                        && item.category == PreparedItemCategory::Aid
                })
                .map(|item| (stack.count, item))
        })
        .collect();
    aid.sort_by(|(a_count, a), (b_count, b)| {
        let a_stimpak = item_name(a).to_ascii_lowercase().contains("stimpak");
        let b_stimpak = item_name(b).to_ascii_lowercase().contains("stimpak");
        b_stimpak
            .cmp(&a_stimpak)
            .then(b_count.cmp(a_count))
            .then(a.base_form_id.cmp(&b.base_form_id))
    });
    aid.first()
        .map(|(count, item)| format!("({count}) {}", item_name(item)))
}
