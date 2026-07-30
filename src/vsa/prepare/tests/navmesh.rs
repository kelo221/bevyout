use super::*;

#[cfg(test)]
mod blocker_footprint_tests {
    use super::*;

    /// Real authored Havok door collision is routinely a single
    /// zero-thickness plane (`MetroGateLoad` in FranklinMetro02): f32
    /// rounding leaves `convex_hull_xz` returning three or four *collinear*
    /// points, which has no interior for any overlap or containment test to
    /// find. It must be thickened, not discarded.
    #[test]
    fn a_flat_collision_plane_is_thickened_into_a_footprint() {
        let points: Vec<[f32; 3]> = (0..8)
            .map(|step| {
                [
                    9.558576 + (step % 3) as f32 * 1.0e-5,
                    105.0 + step as f32 * 0.25,
                    -75.0 + step as f32 * 0.5,
                ]
            })
            .collect();
        let footprint = blocker_footprint(&points);
        assert_eq!(footprint.len(), 4, "{footprint:?}");
        let min_x = footprint.iter().fold(f32::INFINITY, |acc, p| acc.min(p[0]));
        let max_x = footprint
            .iter()
            .fold(f32::NEG_INFINITY, |acc, p| acc.max(p[0]));
        assert!(
            (max_x - min_x - 2.0 * BLOCKER_MIN_HALF_THICKNESS).abs() < 1.0e-3,
            "{footprint:?}"
        );
    }

    #[test]
    fn a_solid_box_keeps_its_own_hull() {
        let points = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 2.0, 1.0],
            [0.0, 2.0, 1.0],
        ];
        assert_eq!(blocker_footprint(&points).len(), 4);
    }

    #[test]
    fn a_single_point_has_no_footprint() {
        assert!(blocker_footprint(&[[1.0, 2.0, 3.0]]).is_empty());
    }
}

#[cfg(test)]
mod landmass_acceptance_tests {
    //! Issue #189 feature 1. [`verify_landmass_acceptance`] is the guard added
    //! *because* Vault 101 shipped with zero navigation while every prepare
    //! metric read a healthy 98% component share -- and until this module it
    //! appeared exactly twice in the repository: its definition and its single
    //! call site, with no test anywhere. A guard against this project's worst
    //! shipped failure had no proof it still fires.
    //!
    //! These tests are written to go red if the guard is *weakened*, not only
    //! if it is deleted: the rejecting cases below are rejected by `landmass`
    //! for reasons the pre-filter in `verify_landmass_acceptance` deliberately
    //! does not screen out, so broadening that filter to "fix" a failing cell
    //! (the tempting move, and the one that would restore the 98% lie) turns
    //! these red rather than green.

    use super::*;

    fn polygon(index: u32, vertex_indices: [u32; 3]) -> PreparedNavPolygon {
        PreparedNavPolygon {
            index,
            vertex_indices,
            ..PreparedNavPolygon::default()
        }
    }

    fn mesh(vertices: Vec<[f32; 3]>, polygons: Vec<PreparedNavPolygon>) -> PreparedNavMesh {
        PreparedNavMesh {
            form_id: 0x0001_0000,
            vertices,
            polygons,
            ..PreparedNavMesh::default()
        }
    }

    /// Two counter-clockwise triangles sharing one edge: the ordinary,
    /// healthy case, which must pass unchanged.
    #[test]
    fn a_valid_mesh_is_accepted() {
        let mesh = mesh(
            vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
            vec![polygon(0, [0, 2, 1]), polygon(1, [0, 3, 2])],
        );
        assert!(verify_landmass_acceptance(&mesh).is_ok());
    }

    /// A mesh wound entirely the other way is *not* a rejection: the guard
    /// retries with both windings exactly as the runtime does, and authored
    /// FO3 `NAVM` winding is not guaranteed to match `landmass`'s. Pinned so
    /// the retry cannot be dropped as dead code.
    #[test]
    fn a_uniformly_reverse_wound_mesh_is_accepted_by_the_retry() {
        let mesh = mesh(
            vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
            vec![polygon(0, [1, 2, 0]), polygon(1, [2, 3, 0])],
        );
        assert!(verify_landmass_acceptance(&mesh).is_ok());
    }

    /// The Vault 101 shape: one polygon wound against the rest. Neither
    /// winding pass can satisfy `landmass`, so it rejects the *entire* mesh
    /// and the cell ends up with no navigation at all -- while every
    /// prepare-side connectivity metric, which measures the graph this pass
    /// built rather than the graph the runtime accepts, still reads healthy.
    #[test]
    fn a_mesh_with_one_reverse_wound_polygon_is_rejected() {
        let mesh = mesh(
            vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
            // Polygon 1 is wound against polygon 0, so neither the forward
            // nor the reversed pass can satisfy `landmass`.
            vec![polygon(0, [0, 2, 1]), polygon(1, [2, 3, 0])],
        );
        let error = verify_landmass_acceptance(&mesh)
            .expect_err("a mixed-winding mesh must not be accepted");
        assert!(
            error.contains("concave") || error.contains("clockwise"),
            "{error}"
        );
    }

    /// Winding-independent rejection: three triangles sharing one edge is a
    /// non-manifold mesh `landmass` refuses in either winding. This is the
    /// case that survives *any* re-winding retry, so it pins the guard's
    /// verdict rather than the retry's.
    #[test]
    fn a_non_manifold_mesh_is_rejected_in_either_winding() {
        let mesh = mesh(
            vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.5, 0.0, 1.0],
                [0.5, 0.0, -1.0],
                [0.5, 0.0, 2.0],
            ],
            vec![
                polygon(0, [1, 0, 2]),
                polygon(1, [0, 1, 3]),
                polygon(2, [1, 0, 4]),
            ],
        );
        let error = verify_landmass_acceptance(&mesh)
            .expect_err("an edge shared by three polygons must not be accepted");
        assert!(error.contains("more than two polygons"), "{error}");
    }

    /// A mesh with nothing walkable left is the runtime's own documented
    /// empty case, not a rejection -- pinned so the early return keeps its
    /// meaning.
    #[test]
    fn a_mesh_with_no_walkable_polygons_is_accepted() {
        let mut mesh = mesh(
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]],
            vec![polygon(0, [0, 1, 2])],
        );
        mesh.polygons[0].walkable = false;
        assert!(verify_landmass_acceptance(&mesh).is_ok());
    }
}

#[cfg(test)]
mod interior_polygon_gate_tests {
    //! Issue #189 feature 2. These pin the *verdict*, not the wording: if the
    //! gate is weakened back to a diagnostic-only report (the shape it had
    //! when it shipped), `a_single_unreported_interior_polygon_fails_prepare`
    //! goes red.

    use super::*;

    #[test]
    fn no_unreported_interior_polygons_is_a_clean_pass() {
        let mut diagnostics = Vec::new();
        assert!(interior_polygon_gate(&[], false, &mut diagnostics).is_ok());
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn a_single_unreported_interior_polygon_fails_prepare() {
        let mut diagnostics = Vec::new();
        let error =
            interior_polygon_gate(&[(0x0001_0000, 42, 0x0002_4710)], false, &mut diagnostics)
                .expect_err("walkable ground inside a closed blocker must stop the build");
        let error = error.to_string();
        assert!(error.contains("00024710"), "{error}");
        assert!(error.contains("polygon 42"), "{error}");
        // Every offender is still enumerated, at error severity, so the
        // manifest names them rather than only reporting a count.
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, "error");
    }

    #[test]
    fn the_documented_escape_hatch_downgrades_the_failure_to_warnings() {
        let mut diagnostics = Vec::new();
        assert!(
            interior_polygon_gate(&[(0x0001_0000, 42, 0x0002_4710)], true, &mut diagnostics)
                .is_ok()
        );
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, "warning");
    }

    /// The hatch is opt-in: an unset (or arbitrary) environment must leave the
    /// invariant fatal, so a stray value cannot quietly retire it.
    #[test]
    fn the_escape_hatch_is_off_unless_explicitly_affirmative() {
        // SAFETY: single-threaded within this test; the variable is read only
        // by `interior_polygon_escape_hatch`, which no other test calls.
        unsafe { std::env::remove_var(NAV_INTERIOR_ESCAPE_ENV) };
        assert!(!interior_polygon_escape_hatch());
        unsafe { std::env::set_var(NAV_INTERIOR_ESCAPE_ENV, "0") };
        assert!(!interior_polygon_escape_hatch());
        unsafe { std::env::set_var(NAV_INTERIOR_ESCAPE_ENV, "yes") };
        assert!(!interior_polygon_escape_hatch());
        unsafe { std::env::set_var(NAV_INTERIOR_ESCAPE_ENV, "1") };
        assert!(interior_polygon_escape_hatch());
        unsafe { std::env::remove_var(NAV_INTERIOR_ESCAPE_ENV) };
    }
}
