use super::*;

#[test]
fn aid_is_usable() {
    assert_eq!(classify(ItemStats::Aid, false), ItemUseAction::Use);
}

#[test]
fn quest_flagged_aid_can_never_be_consumed() {
    assert_eq!(classify(ItemStats::Aid, true), ItemUseAction::Inert);
}

#[test]
fn book_with_text_is_readable() {
    assert_eq!(
        classify(ItemStats::Book { has_text: true }, false),
        ItemUseAction::Read
    );
}

#[test]
fn textless_book_is_inert() {
    assert_eq!(
        classify(ItemStats::Book { has_text: false }, false),
        ItemUseAction::Inert
    );
}

#[test]
fn note_with_text_is_readable() {
    assert_eq!(
        classify(ItemStats::Note { has_text: true }, false),
        ItemUseAction::Read
    );
}

#[test]
fn quest_flagged_note_stays_readable() {
    assert_eq!(
        classify(ItemStats::Note { has_text: true }, true),
        ItemUseAction::Read
    );
}

#[test]
fn key_and_misc_are_inert() {
    assert_eq!(classify(ItemStats::Key, false), ItemUseAction::Inert);
    assert_eq!(classify(ItemStats::Misc, false), ItemUseAction::Inert);
}
