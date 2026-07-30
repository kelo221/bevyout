use super::*;
use bevy::asset::AssetPlugin;
use bevy::state::app::StatesPlugin;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
        .init_asset::<Image>()
        .init_state::<GameplayModal>()
        .add_plugins(HudPlugin);
    app.update();
    app
}

#[test]
fn angular_difference_shortest_signed_arc() {
    assert_eq!(angular_difference(0.0, 90.0), 90.0);
    assert_eq!(angular_difference(350.0, 10.0), 20.0);
    assert_eq!(angular_difference(10.0, 350.0), -20.0);
    assert!(compass_offset_percent(350.0, 10.0) > 50.0);
    assert!(compass_offset_percent(10.0, 350.0) < 50.0);
}

#[test]
fn corner_clusters_match_the_reference_anchors_and_labels() {
    let mut app = test_app();
    let world = app.world_mut();

    let mut clusters = world.query::<(&ClusterSide, &Node)>();
    let mut cluster_count = 0;
    for (cluster, node) in clusters.iter(world) {
        cluster_count += 1;
        assert_eq!(node.position_type, PositionType::Absolute);
        assert_eq!(node.bottom, Val::Percent(CLUSTER_BOTTOM_PCT));
        assert_eq!(node.width, Val::Vh(CLUSTER_WIDTH_VH));
        assert_eq!(node.height, Val::Vh(CLUSTER_HEIGHT_VH));
        match cluster {
            ClusterSide::Left => {
                assert_eq!(node.left, Val::Percent(CLUSTER_SIDE_PCT));
                assert_eq!(node.right, Val::Auto);
            }
            ClusterSide::Right => {
                assert_eq!(node.left, Val::Auto);
                assert_eq!(node.right, Val::Percent(CLUSTER_SIDE_PCT));
            }
        }
    }
    assert_eq!(cluster_count, 2);

    let mut labels = world.query::<(&Text, &ChildOf)>();
    let mut found_hp = false;
    let mut found_ap = false;
    for (text, parent) in labels.iter(world) {
        if text.0 == "HP" {
            assert_eq!(
                *world.get::<ClusterSide>(parent.parent()).unwrap(),
                ClusterSide::Left,
            );
            found_hp = true;
        } else if text.0 == "AP" {
            assert_eq!(
                *world.get::<ClusterSide>(parent.parent()).unwrap(),
                ClusterSide::Right,
            );
            found_ap = true;
        }
    }
    assert!(found_hp && found_ap);
}

#[test]
fn each_corner_uses_one_authored_separator_and_ticks_fill_inward() {
    let mut app = test_app();
    let world = app.world_mut();

    let separator_parents = world
        .query_filtered::<&ChildOf, With<HudSeparator>>()
        .iter(world)
        .map(|parent| *world.get::<ClusterSide>(parent.parent()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(separator_parents.len(), 2);
    assert!(separator_parents.contains(&ClusterSide::Left));
    assert!(separator_parents.contains(&ClusterSide::Right));

    let tick_count = world.query::<&GaugeTick>().iter(world).count();
    assert_eq!(tick_count, GAUGE_TICK_COUNT * 2);

    let mut fills = world.query::<(&Fill, &Node)>();
    for (fill, node) in fills.iter(world) {
        match fill {
            Fill::Hp => {
                assert_eq!(node.left, Val::Px(0.0));
                assert_eq!(node.right, Val::Auto);
            }
            Fill::Ap => {
                assert_eq!(node.left, Val::Auto);
                assert_eq!(node.right, Val::Px(0.0));
            }
            Fill::Cnd => {}
        }
    }
}

#[test]
fn compass_is_always_visible_inside_the_left_cluster() {
    let mut app = test_app();
    let world = app.world_mut();
    let mut compass = world.query_filtered::<(&Node, &ChildOf), With<CompassRoot>>();
    let (node, parent) = compass.single(world).unwrap();
    assert_ne!(node.display, Display::None);
    assert_eq!(node.left, Val::Vh(COMPASS_LEFT_VH));
    assert_eq!(node.top, Val::Vh(COMPASS_TOP_VH));
    assert_eq!(
        *world.get::<ClusterSide>(parent.parent()).unwrap(),
        ClusterSide::Left,
    );
}

#[test]
fn vitals_clamp_fill_widths_and_toggle_condition_and_ammo() {
    let mut app = test_app();
    {
        let mut vitals = app.world_mut().resource_mut::<HudVitals>();
        vitals.hp_fraction = 0.25;
        vitals.ap_fraction = 1.5;
        vitals.condition_fraction = -0.5;
        vitals.condition_visible = false;
        vitals.weapon_drawn = true;
        vitals.ammo_mag = 7;
        vitals.ammo_reserve = 42;
    }
    app.update();

    let world = app.world_mut();
    let mut fills = world.query::<(&Fill, &Node)>();
    for (fill, node) in fills.iter(world) {
        let expected = match fill {
            Fill::Hp => 25.0,
            Fill::Ap => 100.0,
            Fill::Cnd => 0.0,
        };
        assert_eq!(node.width, Val::Percent(expected));
    }
    assert_eq!(
        world
            .query_filtered::<&Node, With<ConditionRoot>>()
            .single(world)
            .unwrap()
            .display,
        Display::None
    );
    assert_eq!(
        world
            .query_filtered::<&Node, With<AmmoRoot>>()
            .single(world)
            .unwrap()
            .display,
        Display::Flex
    );
}

#[test]
fn hud_root_participates_in_game_ui_visibility() {
    let mut app = test_app();
    let world = app.world_mut();
    let count = world
        .query_filtered::<Entity, (With<HudRoot>, With<super::super::console::GameUi>)>()
        .iter(world)
        .count();
    assert_eq!(count, 1);
}
