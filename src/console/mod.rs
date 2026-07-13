//! Structured Gamebryo-style console shared by UI, BRP/MCP, and scripts.

mod commands;
mod executor;
mod grammar;
mod registry;

pub mod harness;
pub mod openmw_ui;
pub mod script;

use bevy::prelude::*;
use bevy::transform::TransformSystems;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use executor::{
    ConsoleEntityHooks, ConsoleExecutor, ConsoleFrame, ConsoleQueue, ConsoleQueuedResponse,
    ConsoleResponses, ConsoleSessionStore, RefIdentity, RefRegistry, resolve_reference,
};
pub use registry::{ConsoleCommand, ConsoleCommandMetadata, ConsoleHandler, ConsoleRegistry};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConsoleSessionId(pub String);

impl ConsoleSessionId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsoleRequest {
    pub session: ConsoleSessionId,
    pub line: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ConsoleOutput {
    pub request_id: u64,
    pub ok: bool,
    pub value: Value,
    pub log: Vec<String>,
    pub frame: u64,
    pub error: Option<ConsoleError>,
}

impl ConsoleOutput {
    fn success(request_id: u64, frame: u64, value: Value, log: Vec<String>) -> Self {
        Self {
            request_id,
            ok: true,
            value,
            log,
            frame,
            error: None,
        }
    }

    fn failure(request_id: u64, frame: u64, error: ConsoleError) -> Self {
        Self {
            request_id,
            ok: false,
            value: Value::Null,
            log: Vec::new(),
            frame,
            error: Some(error),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsoleError {
    pub code: String,
    pub message: String,
}

impl ConsoleError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn internal(error: impl std::fmt::Display) -> Self {
        Self::new("internal_error", error.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct ConsoleInvocation {
    pub request_id: u64,
    pub frame: u64,
    pub session: ConsoleSessionId,
    pub command: String,
    pub args: Vec<String>,
    pub target: Option<Entity>,
}

#[derive(Clone, Debug)]
pub struct ConsoleCommandResult {
    pub value: Value,
    pub log: Vec<String>,
}

impl ConsoleCommandResult {
    pub fn new(value: Value, log: Vec<String>) -> Self {
        Self { value, log }
    }

    pub fn value(value: Value) -> Self {
        Self::new(value, Vec::new())
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsoleSystemSet {
    Execute,
}

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        let mut registry = ConsoleRegistry::default();
        commands::register_engine_commands(&mut registry);
        script::register_script_commands(&mut registry);
        app.insert_resource(registry)
            .init_resource::<executor::ConsoleCounters>()
            .init_resource::<ConsoleFrame>()
            .init_resource::<ConsoleSessionStore>()
            .init_resource::<RefRegistry>()
            .init_resource::<ConsoleEntityHooks>()
            .init_resource::<ConsoleQueue>()
            .init_resource::<ConsoleResponses>()
            .init_resource::<script::ConsoleExecutionMode>()
            .init_resource::<script::AdvanceFrames>()
            .init_resource::<script::ConsoleRng>()
            .add_systems(First, executor::increment_console_frame)
            .add_systems(
                PostUpdate,
                executor::drain_console_queue
                    .in_set(ConsoleSystemSet::Execute)
                    .before(TransformSystems::Propagate),
            );
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
