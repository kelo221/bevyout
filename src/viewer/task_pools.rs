//! Viewer-specific task-pool initialization.
//!
//! Bevy's default IO workers use the platform thread-stack default. Loading a
//! full 5x5 exterior residency window can exceed that stack while glTF scene
//! dependencies are decoded. Initialize only the IO pool early with the same
//! worker count Bevy would choose, but with explicit stack headroom; the
//! ordinary `TaskPoolPlugin` still creates and ticks every global pool.

use bevy::prelude::{App, Plugin};
use bevy::tasks::{IoTaskPool, TaskPoolBuilder, available_parallelism};

const VIEWER_IO_TASK_STACK_BYTES: usize = 16 * 1024 * 1024;

pub(crate) struct ViewerIoTaskPoolPlugin;

impl Plugin for ViewerIoTaskPoolPlugin {
    fn build(&self, _app: &mut App) {
        let io_threads = default_io_thread_count(available_parallelism());
        IoTaskPool::get_or_init(|| {
            TaskPoolBuilder::default()
                .num_threads(io_threads)
                .thread_name("IO Task Pool".into())
                .stack_size(VIEWER_IO_TASK_STACK_BYTES)
                .build()
        });
    }
}

/// Mirrors Bevy 0.19's default IO assignment: 25 percent of available
/// parallelism, rounded to the nearest whole worker and clamped to 1..=4.
fn default_io_thread_count(total_threads: usize) -> usize {
    ((total_threads.saturating_add(2)) / 4).clamp(1, 4)
}

#[cfg(test)]
#[path = "tests/task_pools.rs"]
mod tests;
