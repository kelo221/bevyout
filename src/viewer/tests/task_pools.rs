use super::*;

#[test]
fn viewer_io_pool_preserves_bevys_default_worker_count() {
    for (total, expected) in [(1, 1), (2, 1), (4, 1), (6, 2), (8, 2), (14, 4), (32, 4)] {
        assert_eq!(default_io_thread_count(total), expected, "total={total}");
    }
}

#[test]
fn viewer_io_pool_has_exterior_asset_loading_stack_headroom() {
    assert_eq!(VIEWER_IO_TASK_STACK_BYTES, 16 * 1024 * 1024);
}
