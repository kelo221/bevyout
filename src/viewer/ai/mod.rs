//! AI package runtime behavior layer (issues #193/#194/#195, M4 package
//! wave). The prepared side already shipped: `PACK` decoding, the revisioned
//! `PreparedPackageCatalog` (`vsa::prepare::package_catalog`), and the
//! `showpackages` console surface (#175/#176). This module is the *runtime*
//! layer that decides which package an actor runs, drives its lifecycle over
//! time, and resolves its location/target into world space.
//!
//! `lifecycle`/`resolution`/`selection`/`families`/`autonomous_gate` are pure,
//! std/serde-only decision modules (no Bevy imports) following the codebase's
//! "decision logic in pure modules, thin Bevy systems consume them" rule --
//! so they compile verbatim into `tests/features.rs` via `#[path]`. The
//! console command `showpackages`/`runpackage`
//! (`viewer::console::ai_package_commands`) and the always-on `autonomous`
//! package driver (issue #218) are the two Bevy consumers: both build the
//! plain input types below from prepared data and live placements, select +
//! resolve a package, and run its family through `family_runtime`'s
//! `ActorPackageController` -- `autonomous` on every alive actor as soon as
//! its life state is seeded, no console command required; `runpackage` on
//! demand for debugging.

mod autonomous;
mod autonomous_gate;
pub(crate) mod catalog_cache;
pub(crate) mod families;
pub(crate) mod family_runtime;
pub(crate) mod lifecycle;
pub(crate) mod resolution;
pub(crate) mod selection;

pub(crate) use family_runtime::AiPackagePlugin;
