//! Pure AI package location/target resolution (issue #195, depends on #193).
//!
//! Resolves a selected package's `PLDT` location and `PTDT` target into a
//! concrete world position (and, when known, the world entity behind it),
//! given a plain snapshot of the runtime the actor can see. This is exactly
//! the resolution the prepared catalog *deferred*: `package_catalog.rs`'s
//! `OUT_OF_SCOPE_LOCATION_TYPES`/`OUT_OF_SCOPE_TARGET_TYPES` are the
//! FormID-carrying types that "can point anywhere in the game, not just this
//! prepare pass's decoded content" -- runtime is where that content exists,
//! so runtime is where they resolve.
//!
//! No pathing to the resolved point happens here (that is the package
//! families, #196+); this module only answers "where is it, and what entity
//! is it". Every unresolvable case yields a deterministic
//! [`ResolutionDiagnostic`] -- never a panic and never a silent `(0,0,0)`.
//!
//! std/serde-only (no Bevy) so it compiles verbatim into `tests/features.rs`
//! via `#[path]`. The Bevy console adapter builds the [`ResolutionContext`]
//! snapshot from live placements/transforms.

use std::collections::{HashMap, HashSet};

/// Plain mirror of `PackageLocationInput` (`PLDT`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PackageLocation {
    pub location_type: u32,
    pub form_id: Option<u32>,
    pub raw_value: u32,
    pub radius: i32,
}

/// Plain mirror of `PackageTargetInput` (`PTDT`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PackageTarget {
    pub target_type: i32,
    pub form_id: Option<u32>,
    pub raw_value: u32,
    pub count_or_distance: i32,
}

/// A live reference the actor can see.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedReference {
    pub reference_form_id: u32,
    pub base_form_id: u32,
    pub cell_form_id: u32,
    pub position: [f32; 3],
    /// The Bevy entity bits behind this reference, when it is spawned.
    pub entity: Option<u64>,
    /// This reference's own `XLKR` linked reference (issue #213), if any --
    /// carried through so the patrol marker chain-walk can hop from one
    /// resolved marker to the next without a second lookup pass.
    pub linked_reference: Option<u32>,
}

/// The runtime snapshot resolution reads. All fields are plain data the Bevy
/// adapter gathers once per query -- no engine handles leak into this module.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResolutionContext {
    pub current_cell_form_id: u32,
    /// The actor whose package is being resolved.
    pub actor_position: [f32; 3],
    /// The actor's authored editor location (`NearEditorLocation`), if known.
    pub actor_editor_location: Option<[f32; 3]>,
    /// The point the package itself was authored at (`AtPackageLocation`).
    pub package_location_anchor: Option<[f32; 3]>,
    /// The actor's linked reference (`NearLinkedReference`).
    pub linked_reference: Option<u32>,
    /// The follow leader/target for follow packages.
    pub follow_target: Option<u32>,
    /// Every visible reference, keyed by reference FormID.
    pub references: HashMap<u32, ResolvedReference>,
    /// Base FormID -> the reference FormIDs of that base currently present,
    /// for nearest-instance ("Object ID") resolution.
    pub bases: HashMap<u32, Vec<u32>>,
}

/// How a point was resolved -- reported by `showpackages` so the human sees
/// which rule fired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionSource {
    Reference(u32),
    ActorPosition,
    InCell(u32),
    EditorLocation,
    PackageAnchor,
    LinkedReference(u32),
    NearestOfBase(u32),
    FollowTarget(u32),
}

impl ResolutionSource {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Reference(_) => "reference",
            Self::ActorPosition => "actor-position",
            Self::InCell(_) => "in-cell",
            Self::EditorLocation => "editor-location",
            Self::PackageAnchor => "package-anchor",
            Self::LinkedReference(_) => "linked-reference",
            Self::NearestOfBase(_) => "nearest-of-base",
            Self::FollowTarget(_) => "follow-target",
        }
    }
}

/// A resolved world point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedPoint {
    pub position: [f32; 3],
    pub entity: Option<u64>,
    /// The `PLDT` radius, or a `PTDT` distance -- always non-negative.
    pub radius: f32,
    pub source: ResolutionSource,
}

/// A deterministic reason a location/target could not be resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionDiagnostic {
    pub message: String,
}

impl ResolutionDiagnostic {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ResolutionDiagnostic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

type Resolution = Result<ResolvedPoint, ResolutionDiagnostic>;

fn distance_squared(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

/// Picks the nearest present reference of `base_form_id` to `from`.
fn nearest_of_base(
    context: &ResolutionContext,
    base_form_id: u32,
    from: [f32; 3],
) -> Option<&ResolvedReference> {
    let candidates = context.bases.get(&base_form_id)?;
    candidates
        .iter()
        .filter_map(|reference_form_id| context.references.get(reference_form_id))
        .min_by(|left, right| {
            distance_squared(left.position, from)
                .partial_cmp(&distance_squared(right.position, from))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

fn reference_point(
    context: &ResolutionContext,
    reference_form_id: u32,
    radius: f32,
    source: ResolutionSource,
) -> Resolution {
    let reference = context.references.get(&reference_form_id).ok_or_else(|| {
        ResolutionDiagnostic::new(format!(
            "reference {reference_form_id:08x} is not present in the current runtime"
        ))
    })?;
    Ok(ResolvedPoint {
        position: reference.position,
        entity: reference.entity,
        radius,
        source,
    })
}

/// Resolves a `PLDT` location. `location_type` follows fopdoc's Fallout3 PACK
/// page (0 Near Reference .. 7 At Package Location).
pub fn resolve_location(location: &PackageLocation, context: &ResolutionContext) -> Resolution {
    let radius = (location.radius.max(0)) as f32;
    match location.location_type {
        // Near Reference.
        0 => {
            let form_id = location.form_id.ok_or_else(|| {
                ResolutionDiagnostic::new("near-reference location has no reference FormID")
            })?;
            reference_point(
                context,
                form_id,
                radius,
                ResolutionSource::Reference(form_id),
            )
        }
        // In Cell.
        1 => {
            let cell = location
                .form_id
                .ok_or_else(|| ResolutionDiagnostic::new("in-cell location has no cell FormID"))?;
            if cell == context.current_cell_form_id {
                Ok(ResolvedPoint {
                    position: context.actor_position,
                    entity: None,
                    radius,
                    source: ResolutionSource::InCell(cell),
                })
            } else {
                Err(ResolutionDiagnostic::new(format!(
                    "in-cell location targets cell {cell:08x}, not the current cell {:08x}",
                    context.current_cell_form_id
                )))
            }
        }
        // Near Current Location.
        2 => Ok(ResolvedPoint {
            position: context.actor_position,
            entity: None,
            radius,
            source: ResolutionSource::ActorPosition,
        }),
        // Near Editor Location.
        3 => {
            let position = context.actor_editor_location.ok_or_else(|| {
                ResolutionDiagnostic::new("near-editor-location has no authored editor location")
            })?;
            Ok(ResolvedPoint {
                position,
                entity: None,
                radius,
                source: ResolutionSource::EditorLocation,
            })
        }
        // Object ID -- nearest present reference of this base.
        4 => {
            let base = location.form_id.ok_or_else(|| {
                ResolutionDiagnostic::new("object-id location has no base FormID")
            })?;
            let reference =
                nearest_of_base(context, base, context.actor_position).ok_or_else(|| {
                    ResolutionDiagnostic::new(format!(
                        "no present reference of base {base:08x} to resolve object-id location"
                    ))
                })?;
            Ok(ResolvedPoint {
                position: reference.position,
                entity: reference.entity,
                radius,
                source: ResolutionSource::NearestOfBase(base),
            })
        }
        // Object Type -- needs a form-type index this runtime snapshot does
        // not carry; deterministically diagnosed rather than guessed.
        5 => Err(ResolutionDiagnostic::new(format!(
            "object-type location (type {}) is not resolvable without a form-type index",
            location.raw_value
        ))),
        // Near Linked Reference.
        6 => {
            let linked = context.linked_reference.ok_or_else(|| {
                ResolutionDiagnostic::new("near-linked-reference location has no linked reference")
            })?;
            reference_point(
                context,
                linked,
                radius,
                ResolutionSource::LinkedReference(linked),
            )
        }
        // At Package Location.
        7 => {
            let position = context.package_location_anchor.ok_or_else(|| {
                ResolutionDiagnostic::new("at-package-location has no authored package anchor")
            })?;
            Ok(ResolvedPoint {
                position,
                entity: None,
                radius,
                source: ResolutionSource::PackageAnchor,
            })
        }
        other => Err(ResolutionDiagnostic::new(format!(
            "unsupported location type {other}"
        ))),
    }
}

/// Resolves a `PTDT` target. `target_type` follows fopdoc's Fallout3 PACK page
/// (0 Specific Reference, 1 Object ID, 2 Object Type, 3 Linked/Follow).
pub fn resolve_target(target: &PackageTarget, context: &ResolutionContext) -> Resolution {
    let radius = (target.count_or_distance.max(0)) as f32;
    match target.target_type {
        // Specific Reference.
        0 => {
            let form_id = target.form_id.ok_or_else(|| {
                ResolutionDiagnostic::new("specific-reference target has no reference FormID")
            })?;
            reference_point(
                context,
                form_id,
                radius,
                ResolutionSource::Reference(form_id),
            )
        }
        // Object ID -- nearest present reference of this base.
        1 => {
            let base = target
                .form_id
                .ok_or_else(|| ResolutionDiagnostic::new("object-id target has no base FormID"))?;
            let reference =
                nearest_of_base(context, base, context.actor_position).ok_or_else(|| {
                    ResolutionDiagnostic::new(format!(
                        "no present reference of base {base:08x} to resolve object-id target"
                    ))
                })?;
            Ok(ResolvedPoint {
                position: reference.position,
                entity: reference.entity,
                radius,
                source: ResolutionSource::NearestOfBase(base),
            })
        }
        // Object Type -- as with locations, needs a form-type index.
        2 => Err(ResolutionDiagnostic::new(format!(
            "object-type target (type {}) is not resolvable without a form-type index",
            target.raw_value
        ))),
        // Linked / follow target.
        3 => {
            let follow = context
                .follow_target
                .or(context.linked_reference)
                .ok_or_else(|| {
                    ResolutionDiagnostic::new(
                        "follow/linked target has no follow or linked reference",
                    )
                })?;
            reference_point(
                context,
                follow,
                radius,
                ResolutionSource::FollowTarget(follow),
            )
        }
        other => Err(ResolutionDiagnostic::new(format!(
            "unsupported target type {other}"
        ))),
    }
}

/// Defensive cap on a patrol marker chain-walk (issue #213): authored FO3
/// patrol routes are single digits long, so a malformed or authored cycle
/// hitting this cap is unambiguously a data problem, not a real route --
/// the walk still terminates cleanly rather than allocating unbounded
/// waypoints.
const MAX_LINKED_REFERENCE_CHAIN: usize = 256;

/// Walks a Patrol package's `XLKR` linked-reference chain from `start` (the
/// acting actor's own `linked_reference`, i.e. the first marker), resolving
/// each hop to a world point in authored order (issue #213). Each marker's
/// own `linked_reference` (carried on [`ResolvedReference`]) names the next
/// hop; the walk stops -- cleanly, returning everything resolved so far,
/// never panicking and never looping -- on a reference the context does not
/// know about, a marker with no further link, a revisited FormID (a cycle),
/// or the defensive length cap.
#[must_use]
pub fn linked_reference_chain(context: &ResolutionContext, start: u32) -> Vec<ResolvedPoint> {
    let mut points = Vec::new();
    let mut visited = HashSet::new();
    let mut current = Some(start);
    while let Some(form_id) = current {
        if points.len() >= MAX_LINKED_REFERENCE_CHAIN || !visited.insert(form_id) {
            break;
        }
        let Some(reference) = context.references.get(&form_id) else {
            break;
        };
        points.push(ResolvedPoint {
            position: reference.position,
            entity: reference.entity,
            radius: 0.0,
            source: ResolutionSource::LinkedReference(form_id),
        });
        current = reference.linked_reference;
    }
    points
}

/// Resolves a package's location/target into a world point for a family,
/// preferring one slot but falling back to the other (issue #218: moved out
/// of the console layer, mechanically, so both `runpackage` and the
/// autonomous package driver share one implementation instead of two that
/// could drift). `location`/`target` are the package's `PLDT`/`PTDT` mirrors
/// (`None` when the package has no such slot); `prefer_target` puts the
/// target slot first -- a follow's leader is authored there.
pub fn resolve_family_point(
    location: Option<PackageLocation>,
    target: Option<PackageTarget>,
    context: &ResolutionContext,
    prefer_target: bool,
) -> Resolution {
    let location = location.map(|location| resolve_location(&location, context));
    let target = target.map(|target| resolve_target(&target, context));
    let (first, second) = if prefer_target {
        (target, location)
    } else {
        (location, target)
    };
    if let Some(Ok(point)) = &first {
        return Ok(*point);
    }
    if let Some(Ok(point)) = &second {
        return Ok(*point);
    }
    // Neither resolved: surface the preferred slot's diagnostic.
    Err(first.or(second).and_then(Result::err).unwrap_or_else(|| {
        ResolutionDiagnostic::new("package has no resolvable location or target")
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(reference_form_id: u32, base: u32, position: [f32; 3]) -> ResolvedReference {
        ResolvedReference {
            reference_form_id,
            base_form_id: base,
            cell_form_id: 0x1000,
            position,
            entity: Some(u64::from(reference_form_id)),
            linked_reference: None,
        }
    }

    fn context_with(references: Vec<ResolvedReference>) -> ResolutionContext {
        let mut bases: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut refs = HashMap::new();
        for reference in references {
            bases
                .entry(reference.base_form_id)
                .or_default()
                .push(reference.reference_form_id);
            refs.insert(reference.reference_form_id, reference);
        }
        ResolutionContext {
            current_cell_form_id: 0x1000,
            actor_position: [0.0, 0.0, 0.0],
            references: refs,
            bases,
            ..ResolutionContext::default()
        }
    }

    fn location(location_type: u32, form_id: Option<u32>, radius: i32) -> PackageLocation {
        PackageLocation {
            location_type,
            form_id,
            raw_value: form_id.unwrap_or_default(),
            radius,
        }
    }

    #[test]
    fn near_reference_resolves_to_that_references_position_and_entity() {
        let context = context_with(vec![reference(0x20, 0xAA, [3.0, 4.0, 5.0])]);
        let resolved = resolve_location(&location(0, Some(0x20), 7), &context).unwrap();
        assert_eq!(resolved.position, [3.0, 4.0, 5.0]);
        assert_eq!(resolved.entity, Some(0x20));
        assert_eq!(resolved.radius, 7.0);
        assert_eq!(resolved.source, ResolutionSource::Reference(0x20));
    }

    #[test]
    fn near_reference_missing_reference_is_a_diagnostic_not_a_panic() {
        let context = context_with(vec![]);
        let error = resolve_location(&location(0, Some(0xDEAD), 0), &context).unwrap_err();
        assert!(error.message.contains("0000dead"));
    }

    #[test]
    fn in_cell_resolves_only_for_the_current_cell() {
        let context = context_with(vec![]);
        let resolved = resolve_location(&location(1, Some(0x1000), 0), &context).unwrap();
        assert_eq!(resolved.source, ResolutionSource::InCell(0x1000));
        assert_eq!(resolved.position, context.actor_position);

        let other = resolve_location(&location(1, Some(0x2000), 0), &context).unwrap_err();
        assert!(other.message.contains("not the current cell"));
    }

    #[test]
    fn near_current_location_is_the_actor_position() {
        let mut context = context_with(vec![]);
        context.actor_position = [1.0, 2.0, 3.0];
        let resolved = resolve_location(&location(2, None, 0), &context).unwrap();
        assert_eq!(resolved.position, [1.0, 2.0, 3.0]);
        assert_eq!(resolved.source, ResolutionSource::ActorPosition);
    }

    #[test]
    fn near_editor_location_needs_an_authored_point() {
        let mut context = context_with(vec![]);
        assert!(resolve_location(&location(3, None, 0), &context).is_err());
        context.actor_editor_location = Some([9.0, 9.0, 9.0]);
        let resolved = resolve_location(&location(3, None, 0), &context).unwrap();
        assert_eq!(resolved.position, [9.0, 9.0, 9.0]);
        assert_eq!(resolved.source, ResolutionSource::EditorLocation);
    }

    #[test]
    fn object_id_location_picks_the_nearest_instance() {
        let context = context_with(vec![
            reference(0x20, 0xAA, [10.0, 0.0, 0.0]),
            reference(0x21, 0xAA, [2.0, 0.0, 0.0]),
        ]);
        let resolved = resolve_location(&location(4, Some(0xAA), 0), &context).unwrap();
        assert_eq!(resolved.position, [2.0, 0.0, 0.0]);
        assert_eq!(resolved.source, ResolutionSource::NearestOfBase(0xAA));
    }

    #[test]
    fn object_id_location_with_no_instance_is_a_diagnostic() {
        let context = context_with(vec![]);
        let error = resolve_location(&location(4, Some(0xAA), 0), &context).unwrap_err();
        assert!(
            error
                .message
                .contains("no present reference of base 000000aa")
        );
    }

    #[test]
    fn object_type_location_is_deterministically_unresolvable() {
        let context = context_with(vec![]);
        let error = resolve_location(&location(5, None, 0), &context).unwrap_err();
        assert!(error.message.contains("form-type index"));
    }

    #[test]
    fn near_linked_reference_resolves_through_the_context() {
        let mut context = context_with(vec![reference(0x30, 0xBB, [5.0, 6.0, 7.0])]);
        context.linked_reference = Some(0x30);
        let resolved = resolve_location(&location(6, None, 0), &context).unwrap();
        assert_eq!(resolved.position, [5.0, 6.0, 7.0]);
        assert_eq!(resolved.source, ResolutionSource::LinkedReference(0x30));
    }

    #[test]
    fn at_package_location_uses_the_anchor() {
        let mut context = context_with(vec![]);
        context.package_location_anchor = Some([-1.0, -2.0, -3.0]);
        let resolved = resolve_location(&location(7, None, 0), &context).unwrap();
        assert_eq!(resolved.position, [-1.0, -2.0, -3.0]);
        assert_eq!(resolved.source, ResolutionSource::PackageAnchor);
    }

    #[test]
    fn unsupported_location_type_is_diagnosed() {
        let context = context_with(vec![]);
        let error = resolve_location(&location(99, None, 0), &context).unwrap_err();
        assert!(error.message.contains("unsupported location type 99"));
    }

    #[test]
    fn specific_reference_target_resolves() {
        let context = context_with(vec![reference(0x40, 0xCC, [1.0, 1.0, 1.0])]);
        let target = PackageTarget {
            target_type: 0,
            form_id: Some(0x40),
            raw_value: 0x40,
            count_or_distance: 5,
        };
        let resolved = resolve_target(&target, &context).unwrap();
        assert_eq!(resolved.position, [1.0, 1.0, 1.0]);
        assert_eq!(resolved.radius, 5.0);
        assert_eq!(resolved.source, ResolutionSource::Reference(0x40));
    }

    #[test]
    fn object_id_target_picks_nearest() {
        let context = context_with(vec![
            reference(0x50, 0xDD, [8.0, 0.0, 0.0]),
            reference(0x51, 0xDD, [1.0, 0.0, 0.0]),
        ]);
        let target = PackageTarget {
            target_type: 1,
            form_id: Some(0xDD),
            raw_value: 0xDD,
            count_or_distance: 0,
        };
        let resolved = resolve_target(&target, &context).unwrap();
        assert_eq!(resolved.position, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn follow_target_resolves_through_context() {
        let mut context = context_with(vec![reference(0x60, 0xEE, [2.0, 2.0, 2.0])]);
        context.follow_target = Some(0x60);
        let target = PackageTarget {
            target_type: 3,
            form_id: None,
            raw_value: 0,
            count_or_distance: 0,
        };
        let resolved = resolve_target(&target, &context).unwrap();
        assert_eq!(resolved.position, [2.0, 2.0, 2.0]);
        assert_eq!(resolved.source, ResolutionSource::FollowTarget(0x60));
    }

    #[test]
    fn object_type_target_is_deterministically_unresolvable() {
        let context = context_with(vec![]);
        let target = PackageTarget {
            target_type: 2,
            form_id: None,
            raw_value: 42,
            count_or_distance: 0,
        };
        let error = resolve_target(&target, &context).unwrap_err();
        assert!(error.message.contains("form-type index"));
    }

    #[test]
    fn unsupported_target_type_is_diagnosed() {
        let context = context_with(vec![]);
        let target = PackageTarget {
            target_type: 77,
            ..PackageTarget::default()
        };
        let error = resolve_target(&target, &context).unwrap_err();
        assert!(error.message.contains("unsupported target type 77"));
    }

    /// Like `reference`, but carries an `XLKR` link to the next marker in a
    /// patrol chain (issue #213).
    fn linked_marker(
        reference_form_id: u32,
        position: [f32; 3],
        linked_reference: Option<u32>,
    ) -> ResolvedReference {
        ResolvedReference {
            linked_reference,
            ..reference(reference_form_id, 0x34, position)
        }
    }

    #[test]
    fn linked_reference_chain_walks_a_single_marker() {
        let context = context_with(vec![linked_marker(0x10, [1.0, 2.0, 3.0], None)]);
        let points = linked_reference_chain(&context, 0x10);
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].position, [1.0, 2.0, 3.0]);
        assert_eq!(points[0].source, ResolutionSource::LinkedReference(0x10));
    }

    #[test]
    fn linked_reference_chain_walks_a_three_marker_cycle_and_terminates() {
        // A -> B -> C -> A: the chain must yield exactly the three markers,
        // in order, then stop instead of looping forever.
        let context = context_with(vec![
            linked_marker(0xA, [0.0, 0.0, 0.0], Some(0xB)),
            linked_marker(0xB, [1.0, 0.0, 0.0], Some(0xC)),
            linked_marker(0xC, [2.0, 0.0, 0.0], Some(0xA)),
        ]);
        let points = linked_reference_chain(&context, 0xA);
        assert_eq!(
            points
                .iter()
                .map(|point| point.position)
                .collect::<Vec<_>>(),
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]]
        );
    }

    #[test]
    fn linked_reference_chain_terminates_cleanly_on_a_broken_or_missing_link() {
        // The start reference itself is not present in the runtime at all.
        let empty_context = context_with(vec![]);
        assert_eq!(linked_reference_chain(&empty_context, 0x99), Vec::new());

        // A real first marker whose own link points at a FormID the context
        // does not know about: one waypoint, then a clean stop.
        let dangling_context =
            context_with(vec![linked_marker(0x10, [5.0, 5.0, 5.0], Some(0xDEAD))]);
        let points = linked_reference_chain(&dangling_context, 0x10);
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].position, [5.0, 5.0, 5.0]);
    }

    /// Issue #218's extraction: this is the console's old private
    /// `resolve_family_point` behaviour, pinned here so the moved copy both
    /// `runpackage` and the autonomous driver share cannot silently drift.
    #[test]
    fn resolve_family_point_prefers_the_first_slot_and_falls_back_to_the_second() {
        let context = context_with(vec![reference(0x70, 0xFF, [7.0, 0.0, 0.0])]);
        let loc = location(0, Some(0x70), 0);
        let tgt = PackageTarget {
            target_type: 2, // object-type: deterministically unresolvable
            ..PackageTarget::default()
        };

        // location preferred (prefer_target = false): resolves directly.
        let resolved =
            resolve_family_point(Some(loc), Some(tgt), &context, false).expect("location resolves");
        assert_eq!(resolved.position, [7.0, 0.0, 0.0]);

        // target preferred but unresolvable: falls back to the location slot.
        let resolved = resolve_family_point(Some(loc), Some(tgt), &context, true)
            .expect("falls back to location");
        assert_eq!(resolved.position, [7.0, 0.0, 0.0]);
    }

    #[test]
    fn resolve_family_point_reports_the_preferred_slots_diagnostic_when_neither_resolves() {
        let context = context_with(vec![]);
        let loc = location(3, None, 0); // NearEditorLocation with none authored
        let tgt = PackageTarget {
            target_type: 2,
            ..PackageTarget::default()
        };
        let error = resolve_family_point(Some(loc), Some(tgt), &context, false).unwrap_err();
        assert!(
            error.message.contains("editor location") || error.message.contains("authored"),
            "expected the preferred (location) slot's diagnostic, got: {}",
            error.message
        );
    }

    #[test]
    fn resolve_family_point_with_neither_slot_present_is_a_diagnostic() {
        let context = context_with(vec![]);
        let error = resolve_family_point(None, None, &context, false).unwrap_err();
        assert!(error.message.contains("no resolvable location or target"));
    }
}
