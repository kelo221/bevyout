use super::*;

#[test]
fn live_projection_preserves_exterior_worldspace_and_authored_pose() {
    let location = project_current_world_location(
        Some(0x0001_51e3),
        0x0002_0001,
        [12.345678, 3.500001, -8.125004],
        [0.1234567, -0.7070001, 0.2222222, 0.6543211],
    );

    assert_eq!(
        location,
        WorldLocation::Exterior(WorldLocationExterior {
            worldspace_form_id: 0x0001_51e3,
            position: [12.345678, 3.500001, -8.125004],
            rotation_xyzw: [0.1234567, -0.7070001, 0.2222222, 0.6543211],
        })
    );
}

#[test]
fn live_projection_preserves_interior_cell_and_authored_pose() {
    let location = project_current_world_location(
        None,
        0x0002_0001,
        [-4.125003, 7.750002, 0.000007],
        [-0.3333333, 0.4444444, -0.5555555, 0.6666666],
    );

    assert_eq!(
        location,
        WorldLocation::Interior(WorldLocationInterior {
            cell_form_id: 0x0002_0001,
            position: [-4.125003, 7.750002, 0.000007],
            rotation_xyzw: [-0.3333333, 0.4444444, -0.5555555, 0.6666666],
        })
    );
}
