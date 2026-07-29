//! First-person Fallout 3 HUD overlay.
//!
//! Reproduces the vanilla in-game HUD: centre crosshair with its phosphor
//! glow, the `[ HIDDEN ]` sneak indicator (only while sneaking), the
//! scrolling direction-strip compass beneath the lower-left HP gauge,
//! and the lower-right AP / CND block. Sprite bytes are staged from the game data during
//! `prepare` (`vsa::prepare::interface`) and converted to KTX2; like the
//! Gamebryo HUD, the staged sprites are white shapes baked into the alpha
//! channel, so every `ImageNode` here tints them with the phosphor HUD
//! colour at draw time.
//!
//! Layout metrics were measured off a captured 3440x1440 gameplay
//! screenshot (values below are in vh/pct of that capture): corner gauge
//! blocks x[0.023,0.170] / x[0.827,0.976], label row y[0.846,0.869],
//! separator row y[0.900,0.945], fill inset inside the separator frame.

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::app_state::GameplayModal;

use super::fallout_ui::{PHOSPHOR, glow};
use super::plugins::ViewerSet;

/// Degrees of heading visible across the compass width (matches the
/// vanilla HUD's field of view on the compass strip).
const COMPASS_FOV_DEGREES: f32 = 140.0;
/// One direction-strip repeat spans the full 360°. This is the repeat
/// width as a percentage of the compass container's own width.
const STRIP_REPEAT_PERCENT: f32 = 360.0 / COMPASS_FOV_DEGREES * 100.0;
/// Cap for the notification feed; the `HudMessages::push` entry point
/// below is the API future gameplay systems notify through.
#[allow(dead_code)]
const MAX_MESSAGES: usize = 5;
/// Full crosshair pulse cycle, seconds (slow phosphor breathing).
const CROSSHAIR_PULSE_SECONDS: f32 = 3.2;

/// Corner-cluster metrics measured from the supplied 3440x1440 reference.
/// `vh` keeps the authored HUD proportions stable across widescreen widths.
const CLUSTER_WIDTH_VH: f32 = 36.6;
const CLUSTER_HEIGHT_VH: f32 = 11.6;
const CLUSTER_SIDE_PCT: f32 = 2.2;
const CLUSTER_BOTTOM_PCT: f32 = 3.9;
const SEPARATOR_HEIGHT_VH: f32 = CLUSTER_WIDTH_VH / 4.0;
const LABEL_INSET_VH: f32 = 0.6;
const LABEL_FONT_VH: f32 = 2.5;

/// The authored gauge fill is a row of narrow phosphor ticks, not a solid
/// rectangle. The clip node changes width while these fixed-position sprites
/// retain the vanilla spacing.
const GAUGE_TRACK_LEFT_VH: f32 = 0.9;
const GAUGE_TRACK_RIGHT_VH: f32 = 0.9;
const GAUGE_TRACK_TOP_VH: f32 = 3.0;
const GAUGE_TRACK_HEIGHT_VH: f32 = 2.6;
const GAUGE_TICK_COUNT: usize = 44;
const GAUGE_TICK_WIDTH_VH: f32 = 0.45;
const GAUGE_TICK_PITCH_VH: f32 = 0.78;

/// Compass and condition rows occupy the lower half of their corner clusters.
const COMPASS_LEFT_VH: f32 = 0.8;
const COMPASS_TOP_VH: f32 = 5.25;
const COMPASS_WIDTH_VH: f32 = 34.6;
const COMPASS_HEIGHT_VH: f32 = 3.0;
const CONDITION_LEFT_VH: f32 = 6.0;
const CONDITION_BOTTOM_VH: f32 = 0.4;
const CONDITION_BAR_WIDTH_VH: f32 = 6.3;
const CONDITION_BAR_HEIGHT_VH: f32 = 2.1;

const HUD_SPRITE_ROOT: &str = "staging/interface/hud";

pub(crate) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HudVitals>()
            .init_resource::<HudMessages>()
            .init_resource::<HudSneaking>()
            .add_systems(Startup, spawn_hud)
            .add_systems(
                Update,
                (
                    update_compass,
                    update_vitals,
                    refresh_messages,
                    pulse_crosshair,
                    sync_sneak_visibility,
                    sync_modal_visibility,
                )
                    .in_set(ViewerSet::Ui),
            );
    }
}

/// Presentational vitals read by the bars. A `Resource` so the future
/// health/combat slice can write it without touching HUD internals.
#[derive(Resource)]
pub(crate) struct HudVitals {
    /// 0.0..=1.0 fractions of maximum for the bar fills.
    hp_fraction: f32,
    ap_fraction: f32,
    condition_fraction: f32,
    condition_visible: bool,
    ammo_mag: u32,
    ammo_reserve: u32,
    /// The ammo counter only renders while a weapon is drawn (vanilla
    /// hides it with the weapon holstered, and the gauge frame for a
    /// holstered player is what the reference capture shows).
    weapon_drawn: bool,
}

impl Default for HudVitals {
    fn default() -> Self {
        Self {
            hp_fraction: 1.0,
            ap_fraction: 1.0,
            condition_fraction: 1.0,
            condition_visible: true,
            ammo_mag: 32,
            ammo_reserve: 96,
            weapon_drawn: false,
        }
    }
}

/// Whether the sneak indicator renders. Written by the (future) sneak
/// state; HUD-only otherwise.
#[derive(Resource, Default)]
pub(crate) struct HudSneaking(pub(crate) bool);

/// Top-left notification feed. Kept deliberately narrow: `push` records a
/// line, and the presentation system rebuilds the column when dirty.
#[derive(Resource, Default)]
pub(crate) struct HudMessages {
    lines: VecDeque<String>,
    dirty: bool,
}

impl HudMessages {
    #[expect(dead_code)]
    pub(crate) fn push(&mut self, line: impl Into<String>) {
        self.lines.push_back(line.into());
        while self.lines.len() > MAX_MESSAGES {
            self.lines.pop_front();
        }
        self.dirty = true;
    }
}

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct CrosshairGlow;

#[derive(Component)]
struct StealthIndicator;

#[derive(Component)]
struct MessageColumn;

#[derive(Component)]
struct CompassRoot;

/// One of three stretched copies of the 360° direction strip. `copy`
/// offsets the copy by whole repeats so the centre of the compass is
/// covered for every heading in [0°, 360°).
#[derive(Component)]
struct CompassStrip {
    copy: i32,
}

#[derive(Component, Clone, Copy)]
enum CompassMarker {
    Landmark { bearing: f32 },
    Objective { bearing: f32 },
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum Fill {
    Ap,
    Hp,
    Cnd,
}

#[derive(Component)]
struct AmmoRoot;

#[derive(Component)]
struct AmmoMag;

#[derive(Component)]
struct AmmoReserve;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum ClusterSide {
    Left,
    Right,
}

#[derive(Component)]
struct HudSeparator;

#[derive(Component)]
struct GaugeTick;

#[derive(Component)]
struct ConditionRoot;

fn hud_sprite(assets: &AssetServer, name: &str) -> Handle<Image> {
    assets.load(format!("{HUD_SPRITE_ROOT}/{name}.ktx2"))
}

/// Tinted sprite at full strength; the atlas textures are white-on-alpha.
fn sprite(assets: &AssetServer, name: &str) -> ImageNode {
    ImageNode {
        image: hud_sprite(assets, name),
        color: PHOSPHOR,
        ..default()
    }
}

fn spawn_hud(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            HudRoot,
            super::console::GameUi,
            ZIndex(100),
        ))
        .with_children(|root| {
            spawn_crosshair(root, &assets);
            spawn_stealth(root);
            spawn_messages(root);
            spawn_corner_cluster(root, &assets, ClusterSide::Left);
            spawn_corner_cluster(root, &assets, ClusterSide::Right);
        });
}

fn spawn_crosshair(root: &mut ChildSpawnerCommands, assets: &AssetServer) {
    // Box is the glow sprite (64px vs the 32px core); measured crosshair
    // diameter on the capture is ~1.7vh, so the glow box is ~3.4vh.
    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Vh(3.4),
            height: Val::Vh(3.4),
            ..default()
        },
        // Centre on the anchor point (left/top are the node's upper-left).
        UiTransform::from_translation(Val2::percent(-50.0, -50.0)),
        CrosshairGlow,
        ImageNode {
            image: hud_sprite(assets, "glow_crosshair"),
            color: PHOSPHOR.with_alpha(0.85),
            ..default()
        },
    ))
    .with_children(|glow_node| {
        glow_node.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(25.0),
                top: Val::Percent(25.0),
                width: Val::Percent(50.0),
                height: Val::Percent(50.0),
                ..default()
            },
            sprite(assets, "crosshair"),
        ));
    });
}

fn spawn_stealth(root: &mut ChildSpawnerCommands) {
    // Vanilla only draws the sneak indicator while sneaking.
    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(8.5),
            display: Display::None,
            ..default()
        },
        UiTransform::from_translation(Val2::percent(-50.0, 0.0)),
        StealthIndicator,
        Text::new("[ H I D D E N ]"),
        TextColor(PHOSPHOR),
        TextFont {
            font_size: FontSize::Px(26.0),
            ..default()
        },
        glow(),
    ));
}

fn spawn_messages(root: &mut ChildSpawnerCommands) {
    // Children are (re)spawned by `refresh_messages` while `HudMessages`
    // is dirty; the feed is empty until a real system pushes a line.
    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(2.6),
            top: Val::Percent(3.2),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        },
        MessageColumn,
    ));
}

fn spawn_corner_cluster(root: &mut ChildSpawnerCommands, assets: &AssetServer, side: ClusterSide) {
    let (label, fill, separator) = match side {
        ClusterSide::Left => ("HP", Fill::Hp, "hud_left_seperator"),
        ClusterSide::Right => ("AP", Fill::Ap, "hud_right_seperator"),
    };
    root.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: if side == ClusterSide::Left {
                Val::Percent(CLUSTER_SIDE_PCT)
            } else {
                Val::Auto
            },
            right: if side == ClusterSide::Right {
                Val::Percent(CLUSTER_SIDE_PCT)
            } else {
                Val::Auto
            },
            bottom: Val::Percent(CLUSTER_BOTTOM_PCT),
            width: Val::Vh(CLUSTER_WIDTH_VH),
            height: Val::Vh(CLUSTER_HEIGHT_VH),
            ..default()
        },
        side,
    ))
    .with_children(|cluster| {
        spawn_gauge_fill(cluster, assets, fill, side);

        match side {
            ClusterSide::Left => spawn_compass(cluster, assets),
            ClusterSide::Right => {
                spawn_condition(cluster);
                spawn_ammo(cluster, assets);
            }
        }

        // The authored separator owns the outer bracket and the central
        // horizontal rule. It overlays the tick fill as it does in Fallout 3.
        cluster.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Vh(CLUSTER_WIDTH_VH),
                height: Val::Vh(SEPARATOR_HEIGHT_VH),
                ..default()
            },
            HudSeparator,
            sprite(assets, separator),
        ));

        cluster.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: if side == ClusterSide::Left {
                    Val::Vh(LABEL_INSET_VH)
                } else {
                    Val::Auto
                },
                right: if side == ClusterSide::Right {
                    Val::Vh(LABEL_INSET_VH)
                } else {
                    Val::Auto
                },
                ..default()
            },
            Text::new(label),
            TextColor(PHOSPHOR),
            TextFont {
                font_size: FontSize::Vh(LABEL_FONT_VH),
                ..default()
            },
            glow(),
        ));
    });
}

fn spawn_gauge_fill(
    cluster: &mut ChildSpawnerCommands,
    assets: &AssetServer,
    fill_kind: Fill,
    side: ClusterSide,
) {
    cluster
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Vh(GAUGE_TRACK_LEFT_VH),
            right: Val::Vh(GAUGE_TRACK_RIGHT_VH),
            top: Val::Vh(GAUGE_TRACK_TOP_VH),
            height: Val::Vh(GAUGE_TRACK_HEIGHT_VH),
            ..default()
        })
        .with_children(|track| {
            track
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: if side == ClusterSide::Left {
                            Val::Px(0.0)
                        } else {
                            Val::Auto
                        },
                        right: if side == ClusterSide::Right {
                            Val::Px(0.0)
                        } else {
                            Val::Auto
                        },
                        top: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    fill_kind,
                ))
                .with_children(|ticks| {
                    for index in 0..GAUGE_TICK_COUNT {
                        let offset = index as f32 * GAUGE_TICK_PITCH_VH;
                        ticks.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: if side == ClusterSide::Left {
                                    Val::Vh(offset)
                                } else {
                                    Val::Auto
                                },
                                right: if side == ClusterSide::Right {
                                    Val::Vh(offset)
                                } else {
                                    Val::Auto
                                },
                                top: Val::Px(0.0),
                                width: Val::Vh(GAUGE_TICK_WIDTH_VH),
                                height: Val::Vh(GAUGE_TRACK_HEIGHT_VH),
                                ..default()
                            },
                            GaugeTick,
                            sprite(assets, "hud_tick_mark"),
                        ));
                    }
                });
        });
}

fn spawn_compass(cluster: &mut ChildSpawnerCommands, assets: &AssetServer) {
    cluster
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Vh(COMPASS_LEFT_VH),
                top: Val::Vh(COMPASS_TOP_VH),
                width: Val::Vh(COMPASS_WIDTH_VH),
                height: Val::Vh(COMPASS_HEIGHT_VH),
                overflow: Overflow::clip(),
                ..default()
            },
            CompassRoot,
        ))
        .with_children(|compass| {
            // Three stretched strip copies tile the 360-degree sweep. The
            // source already contains all cardinal labels and minor ticks.
            for copy in 0..3 {
                compass.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Vh(0.35),
                        left: Val::Percent(50.0),
                        width: Val::Percent(STRIP_REPEAT_PERCENT),
                        height: Val::Vh(2.2),
                        ..default()
                    },
                    CompassStrip { copy },
                    sprite(assets, "hud_comp_direction_strip"),
                ));
            }

            compass
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                })
                .with_children(|track| {
                    for (bearing, objective) in [
                        (55.0_f32, true),
                        (30.0, false),
                        (120.0, false),
                        (160.0, false),
                    ] {
                        track.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                top: Val::Px(0.0),
                                left: Val::Percent(-100.0),
                                width: Val::Vh(if objective { 2.2 } else { 0.8 }),
                                height: Val::Vh(if objective { 2.2 } else { 1.5 }),
                                ..default()
                            },
                            UiTransform::from_translation(Val2::percent(-50.0, 0.0)),
                            if objective {
                                CompassMarker::Objective { bearing }
                            } else {
                                CompassMarker::Landmark { bearing }
                            },
                            sprite(
                                assets,
                                if objective {
                                    "glow_hud_compass_objective_marker"
                                } else {
                                    "hud_compass_mark"
                                },
                            ),
                        ));
                    }
                });

            compass.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Px(0.0),
                    width: Val::Vh(1.2),
                    height: Val::Vh(2.4),
                    ..default()
                },
                UiTransform::from_translation(Val2::percent(-50.0, 0.0)),
                sprite(assets, "glow_hud_tick_mark"),
            ));
        });
}

fn spawn_condition(cluster: &mut ChildSpawnerCommands) {
    cluster
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Vh(CONDITION_LEFT_VH),
                bottom: Val::Vh(CONDITION_BOTTOM_VH),
                width: Val::Vh(15.0),
                height: Val::Vh(2.8),
                ..default()
            },
            ConditionRoot,
        ))
        .with_children(|condition| {
            condition.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                Text::new("CND"),
                TextColor(PHOSPHOR),
                TextFont {
                    font_size: FontSize::Vh(2.4),
                    ..default()
                },
                glow(),
            ));
            condition
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Vh(5.4),
                    top: Val::Vh(0.35),
                    width: Val::Vh(CONDITION_BAR_WIDTH_VH),
                    height: Val::Vh(CONDITION_BAR_HEIGHT_VH),
                    overflow: Overflow::clip(),
                    ..default()
                })
                .with_children(|bar| {
                    bar.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(PHOSPHOR.with_alpha(0.85)),
                        Fill::Cnd,
                    ));
                });
        });
}

fn spawn_ammo(cluster: &mut ChildSpawnerCommands, assets: &AssetServer) {
    cluster
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Vh(0.8),
                bottom: Val::Vh(0.35),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::End,
                display: Display::None,
                ..default()
            },
            AmmoRoot,
        ))
        .with_children(|block| {
            block
                .spawn(Node {
                    align_items: AlignItems::Baseline,
                    column_gap: Val::Vh(0.3),
                    ..default()
                })
                .with_children(|line| {
                    line.spawn((
                        Text::new("32"),
                        TextColor(PHOSPHOR),
                        TextFont {
                            font_size: FontSize::Vh(2.1),
                            ..default()
                        },
                        glow(),
                        AmmoMag,
                    ));
                    line.spawn((
                        Text::new("/ 96"),
                        TextColor(PHOSPHOR.with_alpha(0.85)),
                        TextFont {
                            font_size: FontSize::Vh(1.4),
                            ..default()
                        },
                        glow(),
                        AmmoReserve,
                    ));
                });
            block.spawn((
                Node {
                    width: Val::Vh(8.4),
                    height: Val::Vh(0.3),
                    margin: UiRect::top(Val::Vh(0.2)),
                    ..default()
                },
                ImageNode {
                    image: hud_sprite(assets, "hud_bottom_info_seperator"),
                    color: PHOSPHOR.with_alpha(0.7),
                    ..default()
                },
            ));
        });
}

/// Shortest signed angular difference `b - a` in (-180°, 180°].
fn angular_difference(a: f32, b: f32) -> f32 {
    let mut d = (b - a).rem_euclid(360.0);
    if d > 180.0 {
        d -= 360.0;
    }
    d
}

/// Horizontal compass offset in percent-of-compass-width for `bearing`
/// relative to the player `heading`.
fn compass_offset_percent(heading: f32, bearing: f32) -> f32 {
    50.0 + angular_difference(heading, bearing) / COMPASS_FOV_DEGREES * 100.0
}

fn camera_heading_degrees(cameras: &Query<&GlobalTransform, With<Camera3d>>) -> Option<f32> {
    let transform = cameras.iter().next()?;
    let forward = transform.forward();
    // -Z forward heading: 0° when facing -Z (vanilla "north"), positive
    // turning right (clockwise from above, matching the strip's N->E sweep).
    Some(
        f32::atan2(forward.x, -forward.z)
            .to_degrees()
            .rem_euclid(360.0),
    )
}

#[allow(clippy::type_complexity)]
fn update_compass(
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    time: Res<Time>,
    mut strips: Query<(&CompassStrip, &mut Node), Without<CompassMarker>>,
    mut markers: Query<(&CompassMarker, &mut Node, &mut ImageNode), Without<CompassStrip>>,
) {
    let Some(heading) = camera_heading_degrees(&cameras) else {
        return;
    };

    let heading_percent = heading / COMPASS_FOV_DEGREES * 100.0;
    for (strip, mut node) in &mut strips {
        // copy 0,1,2 -> repeats ending at, centred on, and after centre.
        node.left = Val::Percent(
            heading_percent.mul_add(-1.0, 50.0) - (strip.copy - 1) as f32 * STRIP_REPEAT_PERCENT,
        );
    }
    let blink = (time.elapsed_secs() * std::f32::consts::TAU / 2.4).sin() * 0.225 + 0.775;
    for (marker, mut node, mut image) in &mut markers {
        let (bearing, objective) = match *marker {
            CompassMarker::Landmark { bearing } => (bearing, false),
            CompassMarker::Objective { bearing } => (bearing, true),
        };
        node.left = Val::Percent(compass_offset_percent(heading, bearing));
        image.color = PHOSPHOR.with_alpha(if objective { blink } else { 1.0 });
    }
}

#[allow(clippy::type_complexity)]
fn update_vitals(
    vitals: Res<HudVitals>,
    mut fills: Query<(&Fill, &mut Node), Without<AmmoRoot>>,
    mut mag: Query<&mut Text, (With<AmmoMag>, Without<AmmoReserve>)>,
    mut reserve: Query<&mut Text, (With<AmmoReserve>, Without<AmmoMag>)>,
    mut ammo_root: Query<&mut Node, (With<AmmoRoot>, Without<Fill>)>,
    mut condition_root: Query<&mut Node, (With<ConditionRoot>, Without<AmmoRoot>, Without<Fill>)>,
) {
    if !vitals.is_changed() {
        return;
    }
    for (fill, mut node) in &mut fills {
        let fraction = match fill {
            Fill::Ap => vitals.ap_fraction,
            Fill::Hp => vitals.hp_fraction,
            Fill::Cnd => vitals.condition_fraction,
        };
        node.width = Val::Percent(fraction.clamp(0.0, 1.0) * 100.0);
    }
    for mut text in &mut mag {
        **text = vitals.ammo_mag.to_string();
    }
    for mut text in &mut reserve {
        **text = format!("/ {}", vitals.ammo_reserve);
    }
    for mut node in &mut ammo_root {
        node.display = if vitals.weapon_drawn {
            Display::Flex
        } else {
            Display::None
        };
    }
    for mut node in &mut condition_root {
        node.display = if vitals.condition_visible {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn refresh_messages(
    mut messages: ResMut<HudMessages>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    column: Query<Entity, With<MessageColumn>>,
) {
    if !messages.dirty {
        return;
    }
    messages.dirty = false;
    let Ok(column) = column.single() else {
        return;
    };
    commands
        .entity(column)
        .despawn_children()
        .with_children(|root| {
            for line in &messages.lines {
                root.spawn((
                    Text::new(line.clone()),
                    TextColor(PHOSPHOR),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    glow(),
                ));
                root.spawn((
                    Node {
                        width: Val::Percent(60.0),
                        height: Val::Px(6.0),
                        ..default()
                    },
                    ImageNode {
                        image: hud_sprite(&assets, "glow_messages_seperator"),
                        color: PHOSPHOR.with_alpha(0.8),
                        ..default()
                    },
                ));
            }
        });
}

fn pulse_crosshair(time: Res<Time>, mut glows: Query<&mut ImageNode, With<CrosshairGlow>>) {
    let alpha = (time.elapsed_secs() * std::f32::consts::TAU / CROSSHAIR_PULSE_SECONDS).sin()
        * 0.125
        + 0.875;
    for mut image in &mut glows {
        image.color = PHOSPHOR.with_alpha(alpha);
    }
}

fn sync_sneak_visibility(
    sneaking: Res<HudSneaking>,
    mut indicators: Query<&mut Node, With<StealthIndicator>>,
) {
    if !sneaking.is_changed() {
        return;
    }
    let display = if sneaking.0 {
        Display::Flex
    } else {
        Display::None
    };
    for mut node in &mut indicators {
        node.display = display;
    }
}

fn sync_modal_visibility(
    modal: Res<State<GameplayModal>>,
    mut roots: Query<&mut Node, With<HudRoot>>,
) {
    if !modal.is_changed() {
        return;
    }
    let display = if *modal.get() == GameplayModal::None {
        Display::Flex
    } else {
        Display::None
    };
    for mut node in &mut roots {
        node.display = display;
    }
}

#[cfg(test)]
mod tests {
    use bevy::asset::AssetPlugin;
    use bevy::state::app::StatesPlugin;

    use super::*;

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
}
