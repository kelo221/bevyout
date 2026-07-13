//! Declared support registry (F37.4).
//!
//! Every table here is a closed, explicit list of what bevyout currently
//! supports. Anything encountered by the generator that is *not* listed
//! falls back to `SupportStatus::Unknown` in [`super::generator`] — it is
//! never defaulted to `Supported`.

use super::schema::SupportStatus;

/// `(status, save_affecting)` for a declared record kind, or `None` if the
/// kind is undeclared (caller defaults to `Unknown`, `save_affecting: true`).
pub(crate) fn record_status(kind: &str) -> Option<(SupportStatus, bool)> {
    use SupportStatus::*;
    Some(match kind {
        // Fully modelled and used by the prepared-scene pipeline.
        "STAT" | "MSTT" | "LIGH" | "DOOR" | "CONT" | "ACTI" | "TACT" | "FURN" | "TERM" | "WEAP"
        | "AMMO" | "ARMO" | "ALCH" | "MISC" | "BOOK" | "NOTE" | "KEYM" | "CELL" | "IMGS"
        | "SOUN" | "SNDR" | "ASPC" | "MUSC" | "LGTM" | "NAVM" | "REFR" | "ACHR" | "ACRE" => {
            (Supported, false)
        }
        // Recognized and retained, but only minimal fields are extracted
        // (no AI packages, no leveled-list resolution).
        "NPC_" | "CREA" | "LVLI" | "LVLN" | "LVLC" => (Partial, false),
        // Plugin header metadata, not renderable content.
        "TES4" => (IgnoredByDesign, false),
        // Known to exist in Fallout 3 content, explicitly not implemented
        // yet. Quest/dialogue/script state is exactly the kind of thing a
        // save file can depend on, so default these save-affecting.
        "QUST" => (Unsupported, true),
        "SCPT" => (Unsupported, true),
        "DIAL" | "INFO" => (Unsupported, true),
        "PERK" | "SPEL" => (Unsupported, true),
        // Structural/definition-only data with no per-save mutable state.
        "RACE" | "FACT" | "WRLD" => (Unsupported, false),
        _ => return None,
    })
}

/// `(status, save_affecting)` for a declared generic subrecord signature.
/// Applies to any record kind unless a more specific rule in
/// [`super::generator`] overrides it (CTDA, SCPT script data, MODL/MOD2).
pub(crate) fn subrecord_status(signature: &str) -> Option<(SupportStatus, bool)> {
    use SupportStatus::*;
    Some(match signature {
        "EDID" | "FULL" | "DATA" | "TPLT" | "CNTO" => (Supported, false),
        // Bounding boxes are read by nothing in the current viewer pipeline
        // (collision geometry comes from Havok/physics sidecars instead).
        "OBND" => (IgnoredByDesign, false),
        _ => return None,
    })
}

/// `(status, save_affecting)` for a declared asset extension (lower-case,
/// leading dot), keyed off MODL/MOD2 model paths.
pub(crate) fn asset_format_status(extension: &str) -> Option<(SupportStatus, bool)> {
    use SupportStatus::*;
    Some(match extension {
        ".nif" => (Supported, false),
        // Animation/morph companions to a NIF; not converted by this slice.
        ".kf" | ".tri" | ".egm" => (Unsupported, false),
        _ => return None,
    })
}
