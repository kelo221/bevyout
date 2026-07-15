//! Pure state-persistence policy for issues #60 (apply persistent +
//! enable-parent state on load) and #61 (capture dynamic/inventory/container
//! state on unload). Std-only (no Bevy, no additional crates) so
//! `tests/features.rs` can pull this in verbatim exactly like `swap_policy.rs`
//! (issue #52) does -- see that module's doc comment for the pattern this
//! follows.
//!
//! Three seams live here (F60.1):
//!
//! - `resolve_effective_enabled`: per-reference visibility from the
//!   manifest's `initially_enabled` baseline plus `enable_parent` chains
//!   (mirroring `vsa::openmw_esm4::enable::resolve_initially_enabled`'s
//!   cascade-with-inversion semantics, read -- not copied -- from that
//!   module) and save deltas, which may override any reference in the chain.
//! - `plan_apply`: merges that resolution with a delta's transform/activated/
//!   body fields into one `PlacementApplication` per placement.
//! - `diff_capture`: the opposite direction -- a manifest baseline plus a
//!   runtime snapshot produces the minimal delta set (untouched references
//!   produce no delta at all).
//!
//! `ReferenceDelta`/`TransformDelta`/`BodyDelta` deliberately mirror the
//! runtime-relevant fields of `save::PersistentReferenceDelta`/
//! `save::SavedTransform`/`save::SavedBodyState` with their own plain types
//! rather than depending on `src/save` (std/serde-only, but pulls in `anyhow`
//! and `sha2`) -- the same stand-in approach `swap_policy::ReferenceDelta`
//! already uses.

use std::collections::HashMap;

/// Defends `resolve_effective_enabled`'s recursion against a cyclic or
/// pathologically deep enable-parent chain in corrupt/adversarial manifest
/// data; real content never nests this deep.
const MAX_ENABLE_CHAIN_DEPTH: usize = 64;

const TRANSFORM_EPSILON: f32 = 1e-4;
const VELOCITY_EPSILON: f32 = 1e-3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TransformDelta {
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: [f32; 3],
}

impl TransformDelta {
    fn approx_eq(&self, other: &TransformDelta) -> bool {
        components(self)
            .zip(components(other))
            .all(|(a, b)| (a - b).abs() <= TRANSFORM_EPSILON)
    }
}

fn components(transform: &TransformDelta) -> impl Iterator<Item = f32> {
    transform
        .translation
        .into_iter()
        .chain(transform.rotation_xyzw)
        .chain(transform.scale)
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct BodyDelta {
    pub(crate) linear_velocity: [f32; 3],
    pub(crate) angular_velocity: [f32; 3],
    pub(crate) sleeping: bool,
}

impl BodyDelta {
    fn is_at_rest(&self) -> bool {
        self.linear_velocity
            .into_iter()
            .chain(self.angular_velocity)
            .all(|value| value.abs() <= VELOCITY_EPSILON)
    }
}

/// Mirrors the runtime-relevant fields of `save::PersistentReferenceDelta`
/// (F60.1b/c): unlike `swap_policy::ReferenceDelta`, this also carries
/// `activated` (door/container open state), `enable_root_form_id` (an
/// enable-group root recorded on a delta, see `resolve_effective_enabled`'s
/// doc comment), and `body` (dynamic-body pose/velocity), since #60/#61 need
/// all of them; `lock_level` stays out of this seam exactly like
/// `swap_policy::ReferenceDelta` leaves it out (no lock-picking UI in this
/// wave's scope). Container `inventory`/leveled-resolved state (issue #76)
/// deliberately does *not* live here: `ContainerDelta` below is its own
/// small, non-`Copy` seam (a container's stack list is variable-length,
/// which would cost every other caller of this `Copy` type a `.clone()`),
/// captured/applied by `diff_capture_containers`/`plan_apply_containers`
/// alongside this module's placement-shaped capture/apply.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct ReferenceDelta {
    pub(crate) enabled: Option<bool>,
    pub(crate) deleted: bool,
    pub(crate) activated: Option<bool>,
    pub(crate) enable_root_form_id: Option<u32>,
    pub(crate) transform: Option<TransformDelta>,
    pub(crate) body: Option<BodyDelta>,
}

/// One reference's enable-parent link, mirroring
/// `vsa::manifest::PreparedEnableParent`'s connectivity fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EnableParentLink {
    pub(crate) reference_form_id: u32,
    pub(crate) inverted: bool,
}

/// A placement's enable-relevant manifest fields, mirroring
/// `vsa::manifest::PreparedPlacement`'s `initially_enabled`/`enable_parent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlacementInfo {
    pub(crate) reference_form_id: u32,
    pub(crate) initially_enabled: bool,
    pub(crate) enable_parent: Option<EnableParentLink>,
}

/// F60.1(a): resolves every placement's effective enabled/visible state.
///
/// A placement with no `enable_parent` uses its own delta's `enabled`
/// override if present, else its manifest `initially_enabled` baseline
/// (matching `swap_policy::apply_persistent_cell_state`'s existing
/// behavior for non-grouped placements).
///
/// A placement with an `enable_parent` link ignores its own
/// `initially_enabled` baseline entirely (matching
/// `resolve_initially_enabled`'s prepare-time cascade) and instead resolves
/// to its parent's effective state, inverted if the link says so -- unless
/// its own delta carries a direct `enabled` override, which always wins
/// (T60.1's "delta overrides baseline"), or a `enable_root_form_id`
/// redirect (see below).
///
/// `enable_root_form_id` on a delta redirects resolution to another
/// reference's effective-enabled value instead of the manifest's own
/// `enable_parent` chain, letting one save delta cascade an enable-group's
/// root toggle to every descendant without a delta per descendant (no
/// runtime command produces this yet -- see this module's issue -- but the
/// resolution must already honor it since the save format carries the
/// field).
pub(crate) fn resolve_effective_enabled(
    placements: &[PlacementInfo],
    deltas: &HashMap<u32, ReferenceDelta>,
) -> HashMap<u32, bool> {
    let by_id: HashMap<u32, &PlacementInfo> = placements
        .iter()
        .map(|placement| (placement.reference_form_id, placement))
        .collect();
    let mut memo = HashMap::new();
    for placement in placements {
        resolve_one(placement.reference_form_id, &by_id, deltas, &mut memo, 0);
    }
    memo
}

fn resolve_one(
    form_id: u32,
    by_id: &HashMap<u32, &PlacementInfo>,
    deltas: &HashMap<u32, ReferenceDelta>,
    memo: &mut HashMap<u32, bool>,
    depth: usize,
) -> bool {
    if let Some(value) = memo.get(&form_id) {
        return *value;
    }
    // Defensive: a runaway/cyclic chain resolves open rather than hiding
    // content because of corrupt data.
    if depth >= MAX_ENABLE_CHAIN_DEPTH {
        return true;
    }
    let delta = deltas.get(&form_id);
    if let Some(enabled) = delta.and_then(|delta| delta.enabled) {
        memo.insert(form_id, enabled);
        return enabled;
    }
    if let Some(root) = delta.and_then(|delta| delta.enable_root_form_id)
        && root != form_id
    {
        let value = resolve_one(root, by_id, deltas, memo, depth + 1);
        memo.insert(form_id, value);
        return value;
    }
    let value = match by_id.get(&form_id) {
        Some(placement) => match placement.enable_parent {
            Some(link) => {
                let parent_enabled =
                    resolve_one(link.reference_form_id, by_id, deltas, memo, depth + 1);
                if link.inverted {
                    !parent_enabled
                } else {
                    parent_enabled
                }
            }
            None => placement.initially_enabled,
        },
        None => true,
    };
    memo.insert(form_id, value);
    value
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VisibilityDecision {
    Visible,
    Hidden,
}

/// One placement's activation-time decision (F60.1b): whether to show it,
/// and whatever transform/activated/body deltas apply.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PlacementApplication {
    pub(crate) reference_form_id: u32,
    pub(crate) visibility: VisibilityDecision,
    pub(crate) transform: Option<TransformDelta>,
    pub(crate) activated: Option<bool>,
    pub(crate) body: Option<BodyDelta>,
}

/// F60.1(b): merges `resolve_effective_enabled` with each delta's
/// transform/activated/body fields into one per-placement application.
pub(crate) fn plan_apply(
    placements: &[PlacementInfo],
    deltas: &HashMap<u32, ReferenceDelta>,
) -> Vec<PlacementApplication> {
    let effective = resolve_effective_enabled(placements, deltas);
    placements
        .iter()
        .map(|placement| {
            let delta = deltas.get(&placement.reference_form_id);
            let deleted = delta.is_some_and(|delta| delta.deleted);
            let enabled = effective
                .get(&placement.reference_form_id)
                .copied()
                .unwrap_or(placement.initially_enabled);
            PlacementApplication {
                reference_form_id: placement.reference_form_id,
                visibility: if deleted || !enabled {
                    VisibilityDecision::Hidden
                } else {
                    VisibilityDecision::Visible
                },
                transform: delta.and_then(|delta| delta.transform),
                activated: delta.and_then(|delta| delta.activated),
                body: delta.and_then(|delta| delta.body),
            }
        })
        .collect()
}

/// A placement's baseline (prepared) pose, used by `diff_capture` to decide
/// whether a runtime snapshot's pose actually moved.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct BaselinePlacement {
    pub(crate) reference_form_id: u32,
    pub(crate) transform: TransformDelta,
}

/// One placement's runtime state at capture time (F60.1c/F61.1): `present`
/// is `false` for a taken pickup (despawned); `transform`/`body` are `None`
/// for placements this seam does not track pose/velocity for (anything
/// without a live `Transform`/dynamic body); `activated` is `Some(true)`
/// only while a door/container is currently open.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RuntimeSnapshot {
    pub(crate) reference_form_id: u32,
    pub(crate) present: bool,
    pub(crate) transform: Option<TransformDelta>,
    pub(crate) activated: Option<bool>,
    pub(crate) body: Option<BodyDelta>,
}

/// F60.1(c): the minimal delta set reproducing `snapshots` against
/// `baselines` -- a snapshot indistinguishable from its baseline (same pose,
/// not open, no meaningful velocity) produces no delta at all (T60.2).
pub(crate) fn diff_capture(
    baselines: &[BaselinePlacement],
    snapshots: &[RuntimeSnapshot],
) -> HashMap<u32, ReferenceDelta> {
    let baseline_by_id: HashMap<u32, &BaselinePlacement> = baselines
        .iter()
        .map(|baseline| (baseline.reference_form_id, baseline))
        .collect();
    let mut deltas = HashMap::new();
    for snapshot in snapshots {
        if !snapshot.present {
            deltas.insert(
                snapshot.reference_form_id,
                ReferenceDelta {
                    deleted: true,
                    ..Default::default()
                },
            );
            continue;
        }
        let mut delta = ReferenceDelta::default();
        let transform_changed = match (
            snapshot.transform,
            baseline_by_id.get(&snapshot.reference_form_id),
        ) {
            (Some(transform), Some(baseline)) if !transform.approx_eq(&baseline.transform) => {
                delta.transform = Some(transform);
                true
            }
            _ => false,
        };
        if let Some(body) = snapshot.body
            && (transform_changed || !body.is_at_rest())
        {
            delta.body = Some(body);
        }
        if snapshot.activated == Some(true) {
            delta.activated = Some(true);
        }
        if delta != ReferenceDelta::default() {
            deltas.insert(snapshot.reference_form_id, delta);
        }
    }
    deltas
}

/// Issue #76's fixed seam: mirrors agent B's runtime `interaction::
/// ContainerStates` entry exactly (`ContainerState { stacks: Vec<(u32, i32)>,
/// resolved: bool }`, not present in this worktree -- #75 builds it in a
/// parallel worktree). `persist.rs`'s `ContainerStatesStub` resource is the
/// one Bevy-side adapter point that converts to/from this type; the
/// orchestrator swaps that stub for B's real resource at integration since
/// the shapes already match field-for-field.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ContainerSnapshot {
    pub(crate) stacks: Vec<(u32, i32)>,
    pub(crate) resolved: bool,
}

/// A container reference's manifest baseline (F76.2): its fixed, non-leveled
/// inventory entries as normalized `(base_form_id, count)` stacks (see
/// `normalize_stacks`). Leveled entries never appear here -- they only ever
/// exist post-resolution, in a `ContainerSnapshot`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ContainerBaseline {
    pub(crate) stacks: Vec<(u32, i32)>,
}

/// F76.1's mirrored delta shape: `inventory`/`leveled_resolved` ride
/// `save::PersistentReferenceDelta`'s same-named fields. Not `Copy` (see
/// `ReferenceDelta`'s doc comment) since `inventory` is variable-length.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ContainerDelta {
    pub(crate) inventory: Option<Vec<(u32, i32)>>,
    pub(crate) leveled_resolved: Option<bool>,
}

/// Sorts `stacks` by FormID, coalesces duplicate FormIDs by summing their
/// counts, and drops any that net to zero -- the same "strictly sorted, no
/// zero counts" shape `save::validate_inventory` requires, computed here so
/// both `diff_capture_containers` (snapshot stacks) and `persist.rs`
/// (manifest baseline stacks) produce identical, comparable output.
pub(crate) fn normalize_stacks(stacks: &[(u32, i32)]) -> Vec<(u32, i32)> {
    let mut merged: HashMap<u32, i32> = HashMap::new();
    for &(form_id, count) in stacks {
        *merged.entry(form_id).or_insert(0) += count;
    }
    let mut result: Vec<(u32, i32)> = merged
        .into_iter()
        .filter(|&(_, count)| count != 0)
        .collect();
    result.sort_by_key(|&(form_id, _)| form_id);
    result
}

/// F76.2: the minimal per-container delta set -- exactly `diff_capture`'s
/// "untouched -> no delta" contract, but for a container's stacks and its
/// resolved-leveled marker instead of transform/activated/body. A reference
/// absent from `snapshots` was never opened this session (or is not a
/// container) and produces no delta; `persist.rs` only merges observed
/// references, so an unobserved container's previously loaded delta (if any)
/// is left untouched rather than cleared.
pub(crate) fn diff_capture_containers(
    baselines: &HashMap<u32, ContainerBaseline>,
    snapshots: &HashMap<u32, ContainerSnapshot>,
) -> HashMap<u32, ContainerDelta> {
    let mut deltas = HashMap::new();
    for (form_id, snapshot) in snapshots {
        let baseline_stacks = baselines
            .get(form_id)
            .map(|baseline| baseline.stacks.as_slice())
            .unwrap_or(&[]);
        let stacks = normalize_stacks(&snapshot.stacks);
        let mut delta = ContainerDelta::default();
        if stacks.as_slice() != baseline_stacks {
            delta.inventory = Some(stacks);
        }
        if snapshot.resolved {
            delta.leveled_resolved = Some(true);
        }
        if delta != ContainerDelta::default() {
            deltas.insert(*form_id, delta);
        }
    }
    deltas
}

/// F76.3: rebuilds a container's runtime seed state from its save delta (if
/// any) plus the manifest baseline, before first activation. A reference
/// with no delta is not seeded at all -- absence from the returned map means
/// the container is still unresolved and rolls on first open, matching
/// `plan_apply`'s "no delta -> baseline" contract.
pub(crate) fn plan_apply_containers(
    baselines: &HashMap<u32, ContainerBaseline>,
    deltas: &HashMap<u32, ContainerDelta>,
) -> HashMap<u32, ContainerSnapshot> {
    deltas
        .iter()
        .map(|(form_id, delta)| {
            let baseline_stacks = baselines
                .get(form_id)
                .map(|baseline| baseline.stacks.clone())
                .unwrap_or_default();
            (
                *form_id,
                ContainerSnapshot {
                    stacks: delta.inventory.clone().unwrap_or(baseline_stacks),
                    resolved: delta.leveled_resolved.unwrap_or(false),
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transform(translation: [f32; 3]) -> TransformDelta {
        TransformDelta {
            translation,
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    // T60.1: a placement with no enable_parent uses its own baseline.
    #[test]
    fn a_standalone_placement_uses_its_own_baseline() {
        let placements = [PlacementInfo {
            reference_form_id: 0x10,
            initially_enabled: false,
            enable_parent: None,
        }];
        let effective = resolve_effective_enabled(&placements, &HashMap::new());
        assert!(!effective[&0x10]);
    }

    // T60.1: parent disabled -> child hidden.
    #[test]
    fn a_disabled_parent_hides_its_child() {
        let placements = [
            PlacementInfo {
                reference_form_id: 0x1,
                initially_enabled: false,
                enable_parent: None,
            },
            PlacementInfo {
                reference_form_id: 0x2,
                initially_enabled: true, // ignored: enable_parent overrides it
                enable_parent: Some(EnableParentLink {
                    reference_form_id: 0x1,
                    inverted: false,
                }),
            },
        ];
        let effective = resolve_effective_enabled(&placements, &HashMap::new());
        assert!(!effective[&0x1]);
        assert!(!effective[&0x2]);
    }

    // T60.1: the inverted flag is honored when the manifest models it.
    #[test]
    fn an_inverted_enable_parent_link_flips_the_parent_state() {
        let placements = [
            PlacementInfo {
                reference_form_id: 0x1,
                initially_enabled: false,
                enable_parent: None,
            },
            PlacementInfo {
                reference_form_id: 0x2,
                initially_enabled: true,
                enable_parent: Some(EnableParentLink {
                    reference_form_id: 0x1,
                    inverted: true,
                }),
            },
        ];
        let effective = resolve_effective_enabled(&placements, &HashMap::new());
        assert!(!effective[&0x1]);
        assert!(effective[&0x2]);
    }

    // T60.1: an explicit delta on the child overrides both its own baseline
    // and the parent cascade.
    #[test]
    fn a_delta_overrides_the_enable_parent_cascade() {
        let placements = [
            PlacementInfo {
                reference_form_id: 0x1,
                initially_enabled: true,
                enable_parent: None,
            },
            PlacementInfo {
                reference_form_id: 0x2,
                initially_enabled: true,
                enable_parent: Some(EnableParentLink {
                    reference_form_id: 0x1,
                    inverted: false,
                }),
            },
        ];
        let deltas = HashMap::from([(
            0x2,
            ReferenceDelta {
                enabled: Some(false),
                ..Default::default()
            },
        )]);
        let effective = resolve_effective_enabled(&placements, &deltas);
        assert!(effective[&0x1]);
        assert!(!effective[&0x2]);
    }

    // F60.1(a): `enable_root_form_id` redirects resolution to another
    // reference's effective state even when the manifest itself has no
    // `enable_parent` link for this placement.
    #[test]
    fn enable_root_form_id_redirects_resolution_without_a_manifest_link() {
        let placements = [
            PlacementInfo {
                reference_form_id: 0x1,
                initially_enabled: false,
                enable_parent: None,
            },
            PlacementInfo {
                reference_form_id: 0x2,
                initially_enabled: true,
                enable_parent: None,
            },
        ];
        let deltas = HashMap::from([(
            0x2,
            ReferenceDelta {
                enable_root_form_id: Some(0x1),
                ..Default::default()
            },
        )]);
        let effective = resolve_effective_enabled(&placements, &deltas);
        assert!(!effective[&0x2]);
    }

    fn placement_info(reference_form_id: u32) -> PlacementInfo {
        PlacementInfo {
            reference_form_id,
            initially_enabled: true,
            enable_parent: None,
        }
    }

    // T60.2: a moved dynamic body produces a transform+body delta.
    #[test]
    fn a_moved_dynamic_body_produces_transform_and_body_deltas() {
        let baselines = [BaselinePlacement {
            reference_form_id: 0x100,
            transform: transform([0.0, 0.0, 0.0]),
        }];
        let snapshots = [RuntimeSnapshot {
            reference_form_id: 0x100,
            present: true,
            transform: Some(transform([1.0, 0.0, 0.0])),
            activated: None,
            body: Some(BodyDelta {
                linear_velocity: [0.5, 0.0, 0.0],
                angular_velocity: [0.0, 0.0, 0.0],
                sleeping: false,
            }),
        }];
        let deltas = diff_capture(&baselines, &snapshots);
        let delta = deltas.get(&0x100).expect("expected a delta");
        assert_eq!(delta.transform, Some(transform([1.0, 0.0, 0.0])));
        assert!(delta.body.is_some());
    }

    // T60.2: an opened container produces an activated delta.
    #[test]
    fn an_open_container_produces_an_activated_delta() {
        let baselines = [BaselinePlacement {
            reference_form_id: 0x200,
            transform: transform([0.0, 0.0, 0.0]),
        }];
        let snapshots = [RuntimeSnapshot {
            reference_form_id: 0x200,
            present: true,
            transform: Some(transform([0.0, 0.0, 0.0])),
            activated: Some(true),
            body: None,
        }];
        let deltas = diff_capture(&baselines, &snapshots);
        assert_eq!(deltas[&0x200].activated, Some(true));
    }

    // T60.2: a taken pickup produces a deleted delta.
    #[test]
    fn a_taken_pickup_produces_a_deleted_delta() {
        let baselines = [BaselinePlacement {
            reference_form_id: 0x300,
            transform: transform([0.0, 0.0, 0.0]),
        }];
        let snapshots = [RuntimeSnapshot {
            reference_form_id: 0x300,
            present: false,
            transform: None,
            activated: None,
            body: None,
        }];
        let deltas = diff_capture(&baselines, &snapshots);
        assert!(deltas[&0x300].deleted);
    }

    // T60.2: an untouched reference produces no delta at all.
    #[test]
    fn an_untouched_reference_produces_no_delta() {
        let baselines = [BaselinePlacement {
            reference_form_id: 0x400,
            transform: transform([0.0, 0.0, 0.0]),
        }];
        let snapshots = [RuntimeSnapshot {
            reference_form_id: 0x400,
            present: true,
            transform: Some(transform([0.0, 0.0, 0.0])),
            activated: None,
            body: Some(BodyDelta::default()),
        }];
        let deltas = diff_capture(&baselines, &snapshots);
        assert!(!deltas.contains_key(&0x400));
    }

    // T60.3: capturing an applied delta and re-applying it reproduces the
    // same visibility/transform/activated/body decisions (pure round trip).
    #[test]
    fn capture_then_apply_round_trips_a_moved_open_reference() {
        let placements = [placement_info(0x500)];
        let deltas = HashMap::from([(
            0x500,
            ReferenceDelta {
                transform: Some(transform([2.0, 3.0, 4.0])),
                activated: Some(true),
                body: Some(BodyDelta {
                    linear_velocity: [0.1, 0.0, 0.0],
                    angular_velocity: [0.0, 0.0, 0.0],
                    sleeping: false,
                }),
                ..Default::default()
            },
        )]);
        let applications = plan_apply(&placements, &deltas);
        let application = applications
            .iter()
            .find(|application| application.reference_form_id == 0x500)
            .unwrap();
        assert_eq!(application.visibility, VisibilityDecision::Visible);

        let baselines = [BaselinePlacement {
            reference_form_id: 0x500,
            transform: transform([0.0, 0.0, 0.0]),
        }];
        let snapshots = [RuntimeSnapshot {
            reference_form_id: 0x500,
            present: true,
            transform: application.transform,
            activated: application.activated,
            body: application.body,
        }];
        let recaptured = diff_capture(&baselines, &snapshots);
        assert_eq!(recaptured.get(&0x500), Some(&deltas[&0x500]));
    }

    // T60.2/T60.3: a deleted reference is always hidden regardless of any
    // enable-parent chain, and produces the same deleted-only delta.
    #[test]
    fn a_deleted_reference_is_hidden_even_when_its_enable_parent_is_active() {
        let placements = [
            placement_info(0x1),
            PlacementInfo {
                reference_form_id: 0x2,
                initially_enabled: true,
                enable_parent: Some(EnableParentLink {
                    reference_form_id: 0x1,
                    inverted: false,
                }),
            },
        ];
        let deltas = HashMap::from([(
            0x2,
            ReferenceDelta {
                deleted: true,
                ..Default::default()
            },
        )]);
        let applications = plan_apply(&placements, &deltas);
        let application = applications
            .iter()
            .find(|application| application.reference_form_id == 0x2)
            .unwrap();
        assert_eq!(application.visibility, VisibilityDecision::Hidden);
    }

    // Issue #76 (T76.2): a container whose live stacks match the manifest
    // baseline and was never resolved produces no delta at all.
    #[test]
    fn a_baseline_identical_container_captures_no_delta() {
        let baselines = HashMap::from([(
            0x700,
            ContainerBaseline {
                stacks: vec![(0x10, 3)],
            },
        )]);
        let snapshots = HashMap::from([(
            0x700,
            ContainerSnapshot {
                stacks: vec![(0x10, 3)],
                resolved: false,
            },
        )]);
        let deltas = diff_capture_containers(&baselines, &snapshots);
        assert!(!deltas.contains_key(&0x700));
    }

    // T76.2: a looted container (stacks differ from baseline, no roll
    // happened) produces a minimal inventory-only delta.
    #[test]
    fn a_looted_container_captures_a_minimal_inventory_delta() {
        let baselines = HashMap::from([(
            0x701,
            ContainerBaseline {
                stacks: vec![(0x10, 3)],
            },
        )]);
        let snapshots = HashMap::from([(
            0x701,
            ContainerSnapshot {
                stacks: vec![(0x10, 1)],
                resolved: false,
            },
        )]);
        let deltas = diff_capture_containers(&baselines, &snapshots);
        let delta = deltas.get(&0x701).expect("expected a delta");
        assert_eq!(delta.inventory, Some(vec![(0x10, 1)]));
        assert_eq!(delta.leveled_resolved, None);
    }

    // T76.2: a container whose leveled roll happened but whose resulting
    // stacks still match the baseline (e.g. chance-none) produces an
    // LVLR-only delta.
    #[test]
    fn a_resolved_but_baseline_identical_container_captures_an_lvlr_only_delta() {
        let baselines = HashMap::from([(
            0x702,
            ContainerBaseline {
                stacks: vec![(0x10, 3)],
            },
        )]);
        let snapshots = HashMap::from([(
            0x702,
            ContainerSnapshot {
                stacks: vec![(0x10, 3)],
                resolved: true,
            },
        )]);
        let deltas = diff_capture_containers(&baselines, &snapshots);
        let delta = deltas.get(&0x702).expect("expected a delta");
        assert_eq!(delta.inventory, None);
        assert_eq!(delta.leveled_resolved, Some(true));
    }

    // T76.2: duplicate FormIDs and zero-count entries normalize away before
    // comparison, so a container that nets to the baseline after merging
    // still captures no delta.
    #[test]
    fn container_stacks_normalize_before_comparison() {
        let baselines = HashMap::from([(
            0x703,
            ContainerBaseline {
                stacks: vec![(0x10, 3)],
            },
        )]);
        let snapshots = HashMap::from([(
            0x703,
            ContainerSnapshot {
                stacks: vec![(0x10, 5), (0x10, -2), (0x20, 1), (0x20, -1)],
                resolved: false,
            },
        )]);
        let deltas = diff_capture_containers(&baselines, &snapshots);
        assert!(!deltas.contains_key(&0x703));
    }

    // Issue #76 (F76.3): a container with a saved delta is seeded with the
    // saved stacks and resolved marker before first activation.
    #[test]
    fn apply_seeds_a_container_from_its_delta() {
        let baselines = HashMap::from([(
            0x800,
            ContainerBaseline {
                stacks: vec![(0x10, 3)],
            },
        )]);
        let deltas = HashMap::from([(
            0x800,
            ContainerDelta {
                inventory: Some(vec![(0x10, 1)]),
                leveled_resolved: Some(true),
            },
        )]);
        let seeded = plan_apply_containers(&baselines, &deltas);
        let snapshot = seeded.get(&0x800).expect("expected a seeded snapshot");
        assert_eq!(snapshot.stacks, vec![(0x10, 1)]);
        assert!(snapshot.resolved);
    }

    // F76.3: a container with no saved delta is not seeded at all, so it
    // stays unresolved and rolls on first open.
    #[test]
    fn apply_does_not_seed_an_unopened_container() {
        let baselines = HashMap::from([(
            0x801,
            ContainerBaseline {
                stacks: vec![(0x10, 3)],
            },
        )]);
        let seeded = plan_apply_containers(&baselines, &HashMap::new());
        assert!(!seeded.contains_key(&0x801));
    }

    // F76.3: an LVLR-only delta (resolved, but stacks matched baseline at
    // capture) seeds the container with the baseline stacks and
    // `resolved = true`.
    #[test]
    fn apply_seeds_baseline_stacks_for_an_lvlr_only_delta() {
        let baselines = HashMap::from([(
            0x802,
            ContainerBaseline {
                stacks: vec![(0x10, 3)],
            },
        )]);
        let deltas = HashMap::from([(
            0x802,
            ContainerDelta {
                inventory: None,
                leveled_resolved: Some(true),
            },
        )]);
        let seeded = plan_apply_containers(&baselines, &deltas);
        let snapshot = seeded.get(&0x802).expect("expected a seeded snapshot");
        assert_eq!(snapshot.stacks, vec![(0x10, 3)]);
        assert!(snapshot.resolved);
    }
}
