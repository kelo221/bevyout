//! Pure glow-card naming policy (issue #270, PERF wave 1).
//!
//! Kept free of Bevy imports so the executable-specification suite in
//! `tests/features.rs` can include this file verbatim (the same pattern as
//! `realtime_shadow_policy.rs`).

/// True for converted-mesh node names authored as Fallout glow-card
/// billboards (`LightGlow*`).
///
/// Allocation-free: the previous implementation built a fresh lowercased
/// `String` per candidate per rescan (`name.to_ascii_lowercase().starts_
/// with("lightglow")`). Prefix slicing plus `eq_ignore_ascii_case` keeps
/// the same semantics -- an ASCII case-insensitive `lightglow` prefix --
/// without touching the allocator. `str::get` avoids panicking on a
/// non-char boundary for short multibyte names.
pub(crate) fn is_glow_card_mesh_name(name: &str) -> bool {
    const PREFIX: &str = "lightglow";
    name.get(..PREFIX.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(PREFIX))
}

#[cfg(test)]
#[path = "tests/glow_card_policy.rs"]
mod tests;
