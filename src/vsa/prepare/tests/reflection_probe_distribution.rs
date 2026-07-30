use super::allocate_probe_counts;

#[test]
fn allocates_by_region_area() {
    assert_eq!(allocate_probe_counts(&[64.0, 576.0], 12.0, 16), [1, 4]);
}

#[test]
fn cap_prioritizes_largest_regions() {
    assert_eq!(
        allocate_probe_counts(&[16.0, 64.0, 36.0], 12.0, 2),
        [0, 1, 1]
    );
}
