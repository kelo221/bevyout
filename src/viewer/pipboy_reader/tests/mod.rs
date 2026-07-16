use super::*;
use bevy::state::app::StatesPlugin;

use crate::vsa::{PreparedItemCategory, PreparedItemDefinition};

fn catalog_with_item(base_form_id: u32, stats: PreparedItemStats) -> PreparedItemCatalog {
    PreparedItemCatalog {
        revision: "test".into(),
        source_fingerprint: "test".into(),
        items: vec![PreparedItemDefinition {
            base_form_id,
            record_kind: "BOOK".into(),
            category: PreparedItemCategory::Misc,
            editor_id: Some("TestNote".into()),
            display_name: Some("Test Note".into()),
            source_model_path: None,
            icon_asset_path: None,
            world_asset_path: None,
            physics_asset_path: None,
            drop_collider: Default::default(),
            value: None,
            weight: None,
            quest_item: false,
            stats,
            audio: Default::default(),
        }],
    }
}

fn test_app(catalog: PreparedItemCatalog) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin))
        .init_state::<GameplayModal>()
        .insert_resource(catalog);
    install(&mut app);
    app.world_mut()
        .resource_mut::<NextState<GameplayModal>>()
        .set(GameplayModal::PipBoy);
    app.update();
    app
}

fn overlay_count(app: &mut App) -> usize {
    app.world_mut()
        .query_filtered::<Entity, With<ReaderOverlay>>()
        .iter(app.world())
        .count()
}

// F99.3: the reader opens with the requested Book/Note text.
#[test]
fn reader_opens_with_the_requested_note_text() {
    let mut app = test_app(catalog_with_item(
        0x100,
        PreparedItemStats::Note {
            text: Some("Meet me at the water tower.".into()),
        },
    ));

    app.world_mut().write_message(OpenReaderRequested {
        base_form_id: 0x100,
    });
    app.update();

    assert_eq!(overlay_count(&mut app), 1);
    assert!(app.world().contains_resource::<ReaderState>());
    let mut texts = app.world_mut().query::<&Text>();
    let joined: String = texts
        .iter(app.world())
        .map(|text| text.0.clone())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        joined.contains("Meet me at the water tower."),
        "reader body missing note text, got: {joined}"
    );
    assert!(
        joined.contains("Test Note"),
        "reader title missing, got: {joined}"
    );
}

// F99.3: closing the reader despawns the overlay and returns to the
// Pip-Boy (i.e. leaves `GameplayModal::PipBoy` untouched).
#[test]
fn closing_the_reader_returns_to_the_pipboy() {
    let mut app = test_app(catalog_with_item(
        0x100,
        PreparedItemStats::Note {
            text: Some("Some note text.".into()),
        },
    ));
    app.world_mut().write_message(OpenReaderRequested {
        base_form_id: 0x100,
    });
    app.update();
    assert_eq!(overlay_count(&mut app), 1);

    let close_button = app
        .world_mut()
        .query_filtered::<Entity, With<ReaderCloseButton>>()
        .single(app.world())
        .unwrap();
    *app.world_mut()
        .get_mut::<Interaction>(close_button)
        .unwrap() = Interaction::Pressed;
    app.update();

    assert_eq!(overlay_count(&mut app), 0);
    assert!(!app.world().contains_resource::<ReaderState>());
    assert_eq!(
        *app.world().resource::<State<GameplayModal>>().get(),
        GameplayModal::PipBoy
    );
}

// F99.1/F99.3: a Book without authored text is not readable; the reader
// refuses to open (no overlay, no panic on an absent text field).
#[test]
fn a_textless_book_does_not_open_the_reader() {
    let mut app = test_app(catalog_with_item(
        0x200,
        PreparedItemStats::Book {
            flags: None,
            text: None,
        },
    ));

    app.world_mut().write_message(OpenReaderRequested {
        base_form_id: 0x200,
    });
    app.update();

    assert_eq!(overlay_count(&mut app), 0);
    assert!(!app.world().contains_resource::<ReaderState>());
}
