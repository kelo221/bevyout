//! Pip-Boy book/note reader overlay (issue #99, F99.3).
//!
//! Follows `pipboy`'s `QuantityPicker` precedent: an overlay entity plus a
//! small resource, spawned/despawned on top of the current Pip-Boy screen
//! without leaving `GameplayModal::PipBoy`, so "closing returns to the
//! Pip-Boy" needs no new state-machine transition in `app_state::plugin`.
//!
//! [`OpenReaderRequested`] is the public seam: it carries only a
//! `base_form_id` and resolves the catalog entry itself, so it does not
//! assume the Pip-Boy Items view is the only caller. Issue #100's Data ->
//! Notes list is expected to write this same message.

use bevy::prelude::*;

use crate::app_state::GameplayModal;

use super::{PreparedItemCatalog, PreparedItemDefinition, PreparedItemStats};

const GREEN: Color = Color::srgb(0.18, 1.0, 0.48);
const GREEN_DIM: Color = Color::srgba(0.08, 0.45, 0.22, 0.85);
const SCREEN: Color = Color::srgba(0.005, 0.025, 0.012, 0.97);

/// Request to open the reader on `base_form_id`'s prepared Book/Note text.
/// Not every caller has (or should need) a `StackKey` or an inventory
/// count -- reading a note does not consume it -- so the base form id is
/// enough to resolve the catalog entry.
#[derive(Message, Clone, Copy, Debug)]
pub(crate) struct OpenReaderRequested {
    pub(crate) base_form_id: u32,
}

#[derive(Resource, Clone, Copy, Debug)]
struct ReaderState {
    #[allow(dead_code)]
    base_form_id: u32,
}

#[derive(Component)]
struct ReaderOverlay;

#[derive(Component)]
struct ReaderCloseButton;

pub(crate) struct PipBoyReaderPlugin;

impl Plugin for PipBoyReaderPlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
    app.add_message::<OpenReaderRequested>().add_systems(
        Update,
        (open_reader, handle_close_button)
            .in_set(super::plugins::ViewerSet::Ui)
            .run_if(in_state(GameplayModal::PipBoy)),
    );
}

fn item_title(item: &PreparedItemDefinition) -> String {
    item.display_name
        .clone()
        .or_else(|| item.editor_id.clone())
        .unwrap_or_else(|| format!("{:08X}", item.base_form_id))
}

/// F99.3: resolves the most recent open request against the prepared
/// catalog and (re)spawns the overlay. Books whose `flags` byte marks a
/// skill (skill system is out of M3 scope) log a deterministic placeholder
/// line rather than silently dropping that data.
fn open_reader(
    mut commands: Commands,
    mut requests: MessageReader<OpenReaderRequested>,
    catalog: Res<PreparedItemCatalog>,
    overlays: Query<Entity, With<ReaderOverlay>>,
) {
    let Some(request) = requests.read().last() else {
        return;
    };
    let Some(item) = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == request.base_form_id)
    else {
        warn!(
            "reader requested for unknown base {:08x}",
            request.base_form_id
        );
        return;
    };
    let (text, skill_flags) = match &item.stats {
        PreparedItemStats::Book { flags, text } => (text.clone(), *flags),
        PreparedItemStats::Note { text } => (text.clone(), None),
        _ => {
            warn!(
                "reader requested for base {:08x} which is not a Book/Note",
                request.base_form_id
            );
            return;
        }
    };
    let Some(text) = text.filter(|text| !text.is_empty()) else {
        warn!(
            "reader requested for base {:08x} with no authored text",
            request.base_form_id
        );
        return;
    };
    if let Some(flags) = skill_flags
        && flags != 0
    {
        info!(
            "book {:08x} teaches a skill (skill system out of M3 scope)",
            request.base_form_id
        );
    }
    for entity in &overlays {
        commands.entity(entity).despawn();
    }
    commands.insert_resource(ReaderState {
        base_form_id: request.base_form_id,
    });
    spawn_overlay(&mut commands, &item_title(item), &text);
}

fn spawn_overlay(commands: &mut Commands, title: &str, text: &str) {
    commands
        .spawn((
            ReaderOverlay,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(12.0),
                top: Val::Percent(10.0),
                width: Val::Percent(76.0),
                height: Val::Percent(80.0),
                padding: UiRect::all(Val::Px(28.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(SCREEN),
            BorderColor::all(GREEN),
            GlobalZIndex(540),
        ))
        .with_children(|overlay| {
            overlay.spawn((
                Text::new(title.to_owned()),
                TextColor(GREEN),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                Node {
                    height: Val::Px(48.0),
                    border: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
                BorderColor::all(GREEN_DIM),
            ));
            overlay
                .spawn(Node {
                    flex_grow: 1.0,
                    width: Val::Percent(100.0),
                    overflow: Overflow::scroll_y(),
                    padding: UiRect::top(Val::Px(16.0)),
                    ..default()
                })
                .with_children(|body| {
                    body.spawn((
                        Text::new(text.to_owned()),
                        TextColor(GREEN),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                    ));
                });
            overlay
                .spawn(Node {
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::End,
                    ..default()
                })
                .with_children(|footer| {
                    footer
                        .spawn((
                            Button,
                            ReaderCloseButton,
                            Node {
                                padding: UiRect::axes(Val::Px(18.0), Val::Px(8.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BorderColor::all(GREEN),
                            BackgroundColor(GREEN_DIM),
                        ))
                        .with_child((Text::new("CLOSE"), TextColor(GREEN)));
                });
        });
}

fn close_reader(commands: &mut Commands, overlays: &Query<Entity, With<ReaderOverlay>>) {
    for entity in overlays {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<ReaderState>();
}

fn handle_close_button(
    mut commands: Commands,
    buttons: Query<&Interaction, (With<ReaderCloseButton>, Changed<Interaction>)>,
    overlays: Query<Entity, With<ReaderOverlay>>,
) {
    if buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        close_reader(&mut commands, &overlays);
    }
}

#[cfg(test)]
#[path = "pipboy_reader/tests/mod.rs"]
mod tests;
