//! Pure use/read classification rules (issue #99).
//!
//! Dependency-free (std only, no Bevy) exactly like `item_rules`/
//! `container_policy` -- see their module doc comments -- so
//! `tests/features.rs` can include it verbatim. It mirrors the use/read-
//! relevant shape of `vsa::manifest::PreparedItemStats` with a local plain
//! enum ([`ItemStats`]) rather than depending on that (Bevy-touching) type;
//! the Bevy caller (`viewer::pipboy`) maps the real `PreparedItemStats`
//! variant into `ItemStats` before calling [`classify`].

/// Mirrors the use/read-relevant shape of `vsa::manifest::PreparedItemStats`.
/// `Weapon`/`Apparel`/`Ammo` collapse into `Other` -- equipping them is
/// issue #98's scope, not this one's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemStats {
    Aid,
    Book { has_text: bool },
    Note { has_text: bool },
    Key,
    Misc,
    Other,
}

/// What selecting "use"/"read" on a stack from the Pip-Boy Items view does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemUseAction {
    /// Aid item: consumes exactly one from the stack.
    Use,
    /// Book/Note with authored text: opens the reader; the stack is
    /// unchanged.
    Read,
    /// Key/Misc, a textless Book/Note, or a quest-flagged Aid item: no
    /// action.
    Inert,
}

/// F99.1: Aid is usable, Book/Note with text is readable, Key/Misc are
/// inert. Quest-item interplay (issue #81): a quest-flagged item can never
/// be consumed away, so a quest-flagged Aid item degrades from `Use` to
/// `Inert` rather than staying usable. Reading never removes a stack, so a
/// quest-flagged Book/Note stays `Read`.
pub(crate) fn classify(stats: ItemStats, quest_item: bool) -> ItemUseAction {
    match stats {
        ItemStats::Aid if quest_item => ItemUseAction::Inert,
        ItemStats::Aid => ItemUseAction::Use,
        ItemStats::Book { has_text: true } | ItemStats::Note { has_text: true } => {
            ItemUseAction::Read
        }
        _ => ItemUseAction::Inert,
    }
}

/// F99.2: using a consumable removes exactly one from its stack, never the
/// whole stack, regardless of how many are held.
pub(crate) const USE_CONSUMES_COUNT: i32 = 1;

#[cfg(test)]
mod tests {
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
}
