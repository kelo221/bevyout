//! Stable, compact exterior streaming diagnostics.

use bevy::prelude::{Resource, World};
use serde_json::{Value, json};

use super::lifecycle::ExteriorStreamState;

const PROCESS_MEMORY_METHOD: &str = "sysinfo_process_resident_set";
const PROCESS_MEMORY_METRIC: &str = "resident_set_bytes";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProcessMemorySupport {
    Supported,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessMemoryStatus {
    Supported,
    Unsupported,
    NotYetSampled,
}

impl ProcessMemoryStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Unsupported => "unsupported",
            Self::NotYetSampled => "not_yet_sampled",
        }
    }
}

#[derive(Debug, Resource)]
pub(crate) struct ProcessMemoryDiagnostics {
    support: ProcessMemorySupport,
    current_bytes: Option<u64>,
    peak_bytes: Option<u64>,
    ending_bytes: Option<u64>,
    sample_count: u64,
    trace_active: bool,
}

impl Default for ProcessMemoryDiagnostics {
    fn default() -> Self {
        Self::new(process_memory_support())
    }
}

impl ProcessMemoryDiagnostics {
    fn new(support: ProcessMemorySupport) -> Self {
        Self {
            support,
            current_bytes: None,
            peak_bytes: None,
            ending_bytes: None,
            sample_count: 0,
            trace_active: false,
        }
    }

    #[cfg(test)]
    pub(crate) fn supported_for_tests() -> Self {
        Self::new(ProcessMemorySupport::Supported)
    }

    #[cfg(test)]
    pub(crate) fn unsupported_for_tests() -> Self {
        Self::new(ProcessMemorySupport::Unsupported)
    }

    pub(crate) fn begin_trace(&mut self) {
        self.current_bytes = None;
        self.peak_bytes = None;
        self.ending_bytes = None;
        self.sample_count = 0;
        self.trace_active = true;
    }

    pub(crate) fn record_sample(&mut self, bytes: u64) {
        if self.support == ProcessMemorySupport::Unsupported {
            return;
        }
        self.current_bytes = Some(bytes);
        self.peak_bytes = Some(self.peak_bytes.map_or(bytes, |peak| peak.max(bytes)));
        self.sample_count = self.sample_count.saturating_add(1);
    }

    pub(crate) fn finish_trace(&mut self) {
        if self.trace_active {
            self.ending_bytes = self.current_bytes;
            self.trace_active = false;
        }
    }

    fn observe_trace_state(&mut self, trace_enabled: bool) {
        match (self.trace_active, trace_enabled) {
            (false, true) => {
                self.begin_trace();
                self.sample_current();
            }
            (true, false) => {
                self.sample_current();
                self.finish_trace();
            }
            _ => self.sample_current(),
        }
    }

    fn sample_current(&mut self) {
        if self.support != ProcessMemorySupport::Supported {
            return;
        }
        if let Some(bytes) = read_current_process_memory() {
            self.record_sample(bytes);
        }
    }

    fn report(&self) -> ProcessMemoryReport {
        ProcessMemoryReport {
            status: match self.support {
                ProcessMemorySupport::Unsupported => ProcessMemoryStatus::Unsupported,
                ProcessMemorySupport::Supported if self.sample_count == 0 => {
                    ProcessMemoryStatus::NotYetSampled
                }
                ProcessMemorySupport::Supported => ProcessMemoryStatus::Supported,
            },
            current_bytes: self.current_bytes,
            peak_bytes: self.peak_bytes,
            ending_bytes: self.ending_bytes,
            sample_count: self.sample_count,
            trace_active: self.trace_active,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProcessMemoryReport {
    status: ProcessMemoryStatus,
    current_bytes: Option<u64>,
    peak_bytes: Option<u64>,
    ending_bytes: Option<u64>,
    sample_count: u64,
    trace_active: bool,
}

pub(crate) trait StatusSource {
    fn status_json(self) -> Value;
}

pub(crate) fn status<S: StatusSource>(source: S) -> Value {
    source.status_json()
}

impl StatusSource for &ExteriorStreamState {
    fn status_json(self) -> Value {
        status_with_memory(self, &ProcessMemoryDiagnostics::default())
    }
}

impl StatusSource for &mut World {
    fn status_json(self) -> Value {
        let Some(trace_enabled) = self
            .get_resource::<ExteriorStreamState>()
            .map(|state| state.trace)
        else {
            return Value::Null;
        };
        let mut memory = self.get_resource_or_insert_with(ProcessMemoryDiagnostics::default);
        memory.observe_trace_state(trace_enabled);
        let memory = memory.report();
        self.get_resource::<ExteriorStreamState>()
            .map(|state| status_value(state, memory))
            .unwrap_or(Value::Null)
    }
}

pub(crate) fn status_with_memory(
    state: &ExteriorStreamState,
    memory: &ProcessMemoryDiagnostics,
) -> Value {
    status_value(state, memory.report())
}

fn status_value(state: &ExteriorStreamState, memory: ProcessMemoryReport) -> Value {
    let mut counts = [0usize; 7];
    for cell in state.cells.values() {
        let index = match cell.state.lifecycle {
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Unloaded => 0,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Queued => 1,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading => 2,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Ready => 3,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Resident => 4,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Evicting => 5,
            bevyout_core::manifest::exterior::ExteriorCellLifecycle::Failed => 6,
        };
        counts[index] += 1;
    }
    let resident_cells = state
        .cells
        .values()
        .filter(|cell| cell.root.is_some())
        .count();
    json!({
        "initialized": state.initialized,
        "worldspace": state.worldspace_form_id,
        "grid": [state.current_grid.x, state.current_grid.y],
        "unloaded": counts[0],
        "queued": counts[1],
        "loading": counts[2],
        "ready": counts[3],
        "resident": counts[4],
        "evicting": counts[5],
        "failed": counts[6],
        "requests": state.requests,
        "ready_total": state.ready,
        "evictions": state.evictions,
        "cancellations": state.cancellations,
        "stale_completions": state.stale_completions,
        "failures": state.failures,
        "collision_tracked": state.collision_cells.len(),
        "collision_pending": state
            .cells
            .values()
            .filter(|cell| {
                cell.state.lifecycle == bevyout_core::manifest::exterior::ExteriorCellLifecycle::Loading
                    && !cell.collision_ready
            })
            .count(),
        "resident_budget": state.resident_budget,
        "byte_budget": state.byte_budget,
        "byte_budget_kind": "estimated_package_serialization",
        "resident_cells": resident_cells,
        "resident_bytes": memory.current_bytes,
        "peak_resident_cells": state.peak_resident_cells,
        "peak_memory": memory.peak_bytes,
        "ending_memory": memory.ending_bytes,
        "memory_measurement": if memory.status == ProcessMemoryStatus::Supported {
            "process_resident_set"
        } else {
            "unmeasured"
        },
        "memory_measurement_status": memory.status.as_str(),
        "memory_measurement_method": if memory.status == ProcessMemoryStatus::Unsupported {
            "unsupported"
        } else {
            PROCESS_MEMORY_METHOD
        },
        "memory_measurement_metric": PROCESS_MEMORY_METRIC,
        "memory_measurement_platform": std::env::consts::OS,
        "memory_sample_count": memory.sample_count,
        "memory_trace_active": memory.trace_active,
        "resident_package_bytes_estimate": state.resident_bytes,
        "peak_package_bytes_estimate": state.peak_memory,
    })
}

fn process_memory_support() -> ProcessMemorySupport {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    {
        ProcessMemorySupport::Supported
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        ProcessMemorySupport::Unsupported
    }
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn read_current_process_memory() -> Option<u64> {
    let pid = sysinfo::get_current_pid().ok()?;
    let mut system = sysinfo::System::new();
    let _ = system.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
    system
        .process(pid)
        .map(|process| process.memory())
        .filter(|bytes| *bytes > 0)
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn read_current_process_memory() -> Option<u64> {
    None
}

pub(crate) fn cells(state: &ExteriorStreamState) -> serde_json::Value {
    serde_json::Value::Array(
        state
            .cells
            .iter()
            .map(|(grid, cell)| {
                json!({
                    "grid": [grid.x, grid.y],
                    "form_id": format!("{:08x}", cell.state.cell_form_id),
                    "lifecycle": format!("{:?}", cell.state.lifecycle),
                    "generation": cell.state.generation,
                    "bytes": cell.state.estimated_bytes,
                    "collision_ready": cell.collision_ready,
                })
            })
            .collect(),
    )
}

#[cfg(test)]
#[path = "diagnostics_tests.rs"]
mod tests;
