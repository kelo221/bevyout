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

/// Fallout 3 package distances use the same native-unit basis as placements.
/// Keep this pure runtime conversion local so the resolver remains std-only;
/// positions have already crossed the prepare boundary, while PLDT/PTDT
/// distances have not.
const FO3_SCALE: f32 = 1.0 / 70.0;

fn native_distance_to_metres(value: i32) -> f32 {
    value.max(0) as f32 * FO3_SCALE
}

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
    /// This reference's authored facing yaw in radians (issue #242), when the
    /// runtime adapter has rotation data for it. Patrol markers use this while
    /// holding their dwell pose; other references may leave it `None`.
    pub orientation_yaw: Option<f32>,
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
    /// The `PLDT` radius, or a `PTDT` distance, in Bevy metres -- always
    /// non-negative.
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
    let radius = native_distance_to_metres(location.radius);
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
    let radius = native_distance_to_metres(target.count_or_distance);
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

/// Returns the authored facing yaw for a point produced by a linked-reference
/// patrol chain. Keeping this lookup separate leaves `ResolvedPoint` usable by
/// every location/target family without adding marker-only presentation data.
#[must_use]
pub fn point_orientation_yaw(context: &ResolutionContext, point: &ResolvedPoint) -> Option<f32> {
    let ResolutionSource::LinkedReference(form_id) = point.source else {
        return None;
    };
    context.references.get(&form_id)?.orientation_yaw
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
#[path = "tests/resolution.rs"]
mod tests;
