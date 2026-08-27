use super::*;
use std::time::Duration;

const STATUS_BODY_PARTS: [StatusBodyPart; 7] = [
    StatusBodyPart::Head,
    StatusBodyPart::Face,
    StatusBodyPart::Torso,
    StatusBodyPart::LeftArm,
    StatusBodyPart::RightArm,
    StatusBodyPart::LeftLeg,
    StatusBodyPart::RightLeg,
];

const STATUS_DRAW_ORDER: [StatusBodyPart; 7] = [
    StatusBodyPart::LeftLeg,
    StatusBodyPart::RightLeg,
    StatusBodyPart::LeftArm,
    StatusBodyPart::RightArm,
    StatusBodyPart::Torso,
    StatusBodyPart::Head,
    StatusBodyPart::Face,
];

const STATUS_PART_LAYOUTS: [StatusPartLayout; 7] = [
    StatusPartLayout::new(173, 45, 123, 133),
    StatusPartLayout::new(185, 63, 70, 93),
    StatusPartLayout::new(148, 113, 148, 186),
    StatusPartLayout::new(250, 133, 145, 75),
    StatusPartLayout::new(51, 125, 139, 78),
    StatusPartLayout::new(214, 204, 104, 162),
    StatusPartLayout::new(116, 210, 122, 162),
];

const REPEAT_DELAY: Duration = Duration::from_millis(300);
const REPEAT_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StatusBodyPart {
    Head,
    Face,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

impl StatusBodyPart {
    fn index(self) -> usize {
        STATUS_BODY_PARTS
            .iter()
            .position(|part| *part == self)
            .expect("every body part belongs to the fixed layout")
    }

    fn asset_name(self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Face => "face_00",
            Self::Torso => "torso",
            Self::LeftArm => "left_arm",
            Self::RightArm => "right_arm",
            Self::LeftLeg => "left_leg",
            Self::RightLeg => "right_leg",
        }
    }

    fn display_name(self) -> &'static str {
        match self {
            Self::Head => "Head",
            Self::Face => "Face",
            Self::Torso => "Torso",
            Self::LeftArm => "Left arm",
            Self::RightArm => "Right arm",
            Self::LeftLeg => "Left leg",
            Self::RightLeg => "Right leg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct StatusPartLayout {
    pub(super) left: i32,
    pub(super) top: i32,
    pub(super) width: i32,
    pub(super) height: i32,
}

impl StatusPartLayout {
    const fn new(left: i32, top: i32, width: i32, height: i32) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub(super) struct StatusFigureLayout {
    parts: [StatusPartLayout; 7],
}

impl Default for StatusFigureLayout {
    fn default() -> Self {
        Self {
            parts: STATUS_PART_LAYOUTS,
        }
    }
}

impl StatusFigureLayout {
    pub(super) fn part(&self, part: StatusBodyPart) -> StatusPartLayout {
        self.parts[part.index()]
    }

    pub(super) fn move_part(&mut self, part: StatusBodyPart, dx: i32, dy: i32) {
        let layout = &mut self.parts[part.index()];
        layout.left = layout.left.saturating_add(dx);
        layout.top = layout.top.saturating_add(dy);
    }

    pub(super) fn clipboard_text(&self) -> String {
        let mut output = String::from("const STATUS_PART_LAYOUTS: [StatusPartLayout; 7] = [\n");
        for part in STATUS_BODY_PARTS {
            let layout = self.part(part);
            output.push_str(&format!(
                "    StatusPartLayout::new({}, {}, {}, {}),\n",
                layout.left, layout.top, layout.width, layout.height
            ));
        }
        output.push_str("];");
        output
    }
}

#[derive(Resource, Debug)]
pub(super) struct StatusFigureEditor {
    pub(super) enabled: bool,
    pub(super) selected: StatusBodyPart,
    feedback: String,
}

impl Default for StatusFigureEditor {
    fn default() -> Self {
        Self {
            enabled: false,
            selected: StatusBodyPart::Head,
            feedback: String::new(),
        }
    }
}

impl StatusFigureEditor {
    pub(super) fn select_previous(&mut self) {
        let index = self.selected.index();
        self.selected =
            STATUS_BODY_PARTS[(index + STATUS_BODY_PARTS.len() - 1) % STATUS_BODY_PARTS.len()];
        self.feedback.clear();
    }

    pub(super) fn select_next(&mut self) {
        self.selected = STATUS_BODY_PARTS[(self.selected.index() + 1) % STATUS_BODY_PARTS.len()];
        self.feedback.clear();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

impl MoveDirection {
    fn delta(self) -> (i32, i32) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
        }
    }
}

#[derive(Resource, Debug, Default)]
pub(super) struct StatusMoveRepeat {
    pub(super) direction: Option<MoveDirection>,
    elapsed: Duration,
    next_repeat: Duration,
}

impl StatusMoveRepeat {
    pub(super) fn steps(
        &mut self,
        direction: MoveDirection,
        pressed: bool,
        just_pressed: bool,
        delta: Duration,
    ) -> u32 {
        if !pressed {
            self.reset();
            return 0;
        }
        if just_pressed || self.direction != Some(direction) {
            self.direction = Some(direction);
            self.elapsed = Duration::ZERO;
            self.next_repeat = REPEAT_DELAY;
            return 1;
        }

        self.elapsed = self.elapsed.saturating_add(delta);
        let mut steps = 0;
        while self.elapsed >= self.next_repeat {
            steps += 1;
            self.next_repeat = self.next_repeat.saturating_add(REPEAT_INTERVAL);
        }
        steps
    }

    fn reset(&mut self) {
        self.direction = None;
        self.elapsed = Duration::ZERO;
        self.next_repeat = REPEAT_DELAY;
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub(super) struct StatusFigurePart(pub(super) StatusBodyPart);

#[derive(Component)]
pub(super) struct StatusFigureDebugOverlay;

fn status_sprite(
    parent: &mut ChildSpawnerCommands,
    assets: &AssetServer,
    part: StatusBodyPart,
    layout: StatusPartLayout,
    selected: bool,
) {
    parent.spawn((
        StatusFigurePart(part),
        ImageNode {
            image: assets.load(format!(
                "staging/interface/stats/{}.ktx2",
                part.asset_name()
            )),
            color: GREEN,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(layout.left as f32),
            top: Val::Px(layout.top as f32),
            width: Val::Px(layout.width as f32),
            height: Val::Px(layout.height as f32),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BorderColor::all(if selected { GREEN } else { Color::NONE }),
    ));
}

fn spawn_status_figure(
    parent: &mut ChildSpawnerCommands,
    assets: &AssetServer,
    layout: &StatusFigureLayout,
    editor: &StatusFigureEditor,
) {
    for part in STATUS_DRAW_ORDER {
        status_sprite(
            parent,
            assets,
            part,
            layout.part(part),
            editor.enabled && editor.selected == part,
        );
    }
}

fn debug_overlay_text(editor: &StatusFigureEditor, layout: &StatusFigureLayout) -> String {
    let part = layout.part(editor.selected);
    let feedback = if editor.feedback.is_empty() {
        String::new()
    } else {
        format!("\n{}", editor.feedback)
    };
    format!(
        "BODY LAYOUT\n{}  X {}  Y {}  W {}  H {}\n\
         F2/F3 select  Arrows move\nShift = 10 px  F4 copy  F1 close{}",
        editor.selected.display_name(),
        part.left,
        part.top,
        part.width,
        part.height,
        feedback
    )
}

fn active_move_direction(keys: &ButtonInput<KeyCode>) -> Option<MoveDirection> {
    [
        (KeyCode::ArrowLeft, MoveDirection::Left),
        (KeyCode::ArrowRight, MoveDirection::Right),
        (KeyCode::ArrowUp, MoveDirection::Up),
        (KeyCode::ArrowDown, MoveDirection::Down),
    ]
    .into_iter()
    .find_map(|(key, direction)| keys.pressed(key).then_some(direction))
}

fn direction_key(direction: MoveDirection) -> KeyCode {
    match direction {
        MoveDirection::Left => KeyCode::ArrowLeft,
        MoveDirection::Right => KeyCode::ArrowRight,
        MoveDirection::Up => KeyCode::ArrowUp,
        MoveDirection::Down => KeyCode::ArrowDown,
    }
}

#[derive(bevy::ecs::system::SystemParam)]
pub(super) struct StatusEditorParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    time: Res<'w, Time<Real>>,
    state: Res<'w, PipBoyState>,
    editor: ResMut<'w, StatusFigureEditor>,
    layout: ResMut<'w, StatusFigureLayout>,
    repeat: ResMut<'w, StatusMoveRepeat>,
    clipboard: ResMut<'w, Clipboard>,
    parts: Query<
        'w,
        's,
        (
            &'static StatusFigurePart,
            &'static mut Node,
            &'static mut BorderColor,
        ),
    >,
    overlay:
        Query<'w, 's, (&'static mut Text, &'static mut Visibility), With<StatusFigureDebugOverlay>>,
}

pub(super) fn handle_status_figure_editor(params: StatusEditorParams) {
    let StatusEditorParams {
        keys,
        time,
        state,
        mut editor,
        mut layout,
        mut repeat,
        mut clipboard,
        mut parts,
        mut overlay,
    } = params;
    if state.view != PipBoyView::Stats {
        repeat.reset();
        return;
    }

    if keys.just_pressed(KeyCode::F1) {
        editor.enabled = !editor.enabled;
        editor.feedback.clear();
        repeat.reset();
    }

    if editor.enabled {
        if keys.just_pressed(KeyCode::F2) {
            editor.select_previous();
            repeat.reset();
        }
        if keys.just_pressed(KeyCode::F3) {
            editor.select_next();
            repeat.reset();
        }
        if keys.just_pressed(KeyCode::F4) {
            editor.feedback = match clipboard.set_text(layout.clipboard_text()) {
                Ok(()) => "Copied complete layout".to_owned(),
                Err(error) => format!("Copy failed: {error}"),
            };
        }

        if let Some(direction) = active_move_direction(&keys) {
            let key = direction_key(direction);
            let steps = repeat.steps(direction, true, keys.just_pressed(key), time.delta());
            if steps > 0 {
                let scale = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight)
                {
                    10
                } else {
                    1
                };
                let (dx, dy) = direction.delta();
                layout.move_part(
                    editor.selected,
                    dx.saturating_mul(scale).saturating_mul(steps as i32),
                    dy.saturating_mul(scale).saturating_mul(steps as i32),
                );
                editor.feedback.clear();
            }
        } else {
            repeat.reset();
        }
    } else {
        repeat.reset();
    }

    for (part, mut node, mut border) in &mut parts {
        let part_layout = layout.part(part.0);
        node.left = Val::Px(part_layout.left as f32);
        node.top = Val::Px(part_layout.top as f32);
        node.width = Val::Px(part_layout.width as f32);
        node.height = Val::Px(part_layout.height as f32);
        *border = BorderColor::all(if editor.enabled && editor.selected == part.0 {
            GREEN
        } else {
            Color::NONE
        });
    }
    for (mut text, mut visibility) in &mut overlay {
        text.0 = debug_overlay_text(&editor, &layout);
        *visibility = if editor.enabled {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn condition_meter(parent: &mut ChildSpawnerCommands, left: f32, top: f32, width: f32, value: f32) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                top: Val::Px(top),
                width: Val::Px(width),
                height: Val::Px(12.0),
                padding: UiRect::all(Val::Px(2.0)),
                border: UiRect {
                    top: Val::Px(1.0),
                    right: Val::Px(1.0),
                    ..default()
                },
                ..default()
            },
            BorderColor::all(GREEN),
        ))
        .with_child((
            Node {
                width: Val::Percent(value.clamp(0.0, 1.0) * 100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(GREEN),
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
                width: Val::Percent(14.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(14.0),
                padding: UiRect::left(Val::Px(5.0)).with_top(Val::Px(28.0)),
                ..default()
            })
            .with_children(|labels| {
                for (label, selected) in [("CND", true), ("RAD", false), ("EFF", false)] {
                    labels.spawn((
                        Text::new(label),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        glow(),
                        Node {
                            padding: UiRect::axes(Val::Px(10.0), Val::Px(3.0)),
                            border: UiRect::all(Val::Px(if selected { 1.0 } else { 0.0 })),
                            ..default()
                        },
                        BorderColor::all(GREEN),
                    ));
                }
            });
            body.spawn(Node {
                width: Val::Percent(64.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            })
            .with_children(|figure| {
                figure
                    .spawn(Node {
                        width: Val::Px(420.0),
                        height: Val::Px(420.0),
                        position_type: PositionType::Relative,
                        ..default()
                    })
                    .with_children(|body| {
                        spawn_status_figure(
                            body,
                            &sources.assets,
                            &sources.figure_layout,
                            &sources.figure_editor,
                        );
                        let limbs = sources
                            .progression
                            .as_ref()
                            .map(|progression| &progression.limbs);
                        for (left, top, width, part) in [
                            (169.0, 0.0, 82.0, bevyout_core::combat::BodyPartId::Head),
                            (8.0, 128.0, 94.0, bevyout_core::combat::BodyPartId::LeftArm),
                            (
                                318.0,
                                128.0,
                                94.0,
                                bevyout_core::combat::BodyPartId::RightArm,
                            ),
                            (169.0, 162.0, 82.0, bevyout_core::combat::BodyPartId::Torso),
                            (58.0, 292.0, 94.0, bevyout_core::combat::BodyPartId::LeftLeg),
                            (
                                268.0,
                                292.0,
                                94.0,
                                bevyout_core::combat::BodyPartId::RightLeg,
                            ),
                        ] {
                            let value = limbs.map_or(1.0, |state| state.part(part).fraction());
                            condition_meter(body, left, top, width, value);
                        }
                    });
                figure.spawn((
                    Text::new(format!("{} - Level {}", status.name, status.level)),
                    TextColor(GREEN),
                    TextFont {
                        font_size: FontSize::Px(21.0),
                        ..default()
                    },
                    glow(),
                ));
            });
            body.spawn((
                StatusFigureDebugOverlay,
                Text::new(debug_overlay_text(
                    &sources.figure_editor,
                    &sources.figure_layout,
                )),
                TextColor(GREEN),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                glow(),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(8.0),
                    top: Val::Px(72.0),
                    width: Val::Px(270.0),
                    padding: UiRect::all(Val::Px(9.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(SCREEN.with_alpha(0.94)),
                BorderColor::all(GREEN),
                if sources.figure_editor.enabled {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
            ));
            body.spawn(Node {
                width: Val::Percent(22.0),
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::right(Val::Px(5.0)).with_top(Val::Px(32.0)),
                ..default()
            })
            .with_children(|quick| {
                if let Some(line) = quick_aid_line(&sources.inventory, &sources.catalog) {
                    quick.spawn((
                        Text::new(line),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(17.0),
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
