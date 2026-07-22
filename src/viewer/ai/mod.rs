//! AI package runtime behavior layer (issues #193/#194/#195, M4 package
//! wave). The prepared side already shipped: `PACK` decoding, the revisioned
//! `PreparedPackageCatalog` (`vsa::prepare::package_catalog`), and the
//! `showpackages` console surface (#175/#176). This module is the *runtime*
//! layer that decides which package an actor runs, drives its lifecycle over
//! time, and resolves its location/target into world space.
//!
//! Each submodule is a pure, std/serde-only decision module (no Bevy imports)
//! following the codebase's "decision logic in pure modules, thin Bevy
//! systems consume them" rule -- so all three compile verbatim into
//! `tests/features.rs` via `#[path]`. Their only runtime consumer today is
//! the `showpackages` console command (`viewer::console::ai_package_commands`),
//! which builds the plain input types below from prepared data and live
//! placements and reports the selected package, its lifecycle phase, and its
//! resolved location/target. Per-package-family behavior (#196/#197/#198),
//! navigation, and animation are explicitly out of scope for this wave -- no
//! always-on system ticks actors here yet.

pub(crate) mod families;
pub(crate) mod family_runtime;
pub(crate) mod lifecycle;
pub(crate) mod resolution;
pub(crate) mod selection;

pub(crate) use family_runtime::AiPackagePlugin;
