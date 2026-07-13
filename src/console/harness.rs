use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

use super::script::{AdvanceFrames, ConsoleExecutionMode, Transcript, run_script_with_harness};
use super::{ConsoleExecutor, ConsolePlugin, ConsoleRequest, ConsoleSessionId, RefRegistry};

pub struct ConsoleHarness {
    app: App,
    session: ConsoleSessionId,
}

impl Default for ConsoleHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleHarness {
    pub fn new() -> Self {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, ConsolePlugin))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
                1.0 / 60.0,
            )))
            .insert_resource(ConsoleExecutionMode::Harness);
        app.update();
        Self {
            app,
            session: ConsoleSessionId::new("harness"),
        }
    }

    pub fn synthetic() -> Self {
        let mut harness = Self::new();
        let player = harness.spawn_reference(0x14, Some("player"), "Synthetic Player", Vec3::ZERO);
        harness
            .app
            .world_mut()
            .resource_mut::<RefRegistry>()
            .set_player(player);
        harness.spawn_reference(
            0x0100,
            Some("TestCrate"),
            "Synthetic Crate",
            Vec3::new(1.0, 2.0, 3.0),
        );
        harness
    }

    pub fn spawn_reference(
        &mut self,
        form_id: u32,
        editor_id: Option<&str>,
        name: &str,
        position: Vec3,
    ) -> Entity {
        let entity = self
            .app
            .world_mut()
            .spawn((
                Name::new(name.to_string()),
                Transform::from_translation(position),
            ))
            .id();
        self.app
            .world_mut()
            .resource_mut::<RefRegistry>()
            .register(entity, form_id, editor_id);
        entity
    }

    pub fn session(&self) -> &ConsoleSessionId {
        &self.session
    }

    pub fn set_session(&mut self, session: ConsoleSessionId) {
        self.session = session;
    }

    pub fn exec(&mut self, line: &str) -> super::ConsoleOutput {
        let output = ConsoleExecutor::execute(
            self.app.world_mut(),
            ConsoleRequest {
                session: self.session.clone(),
                line: line.to_string(),
            },
        );
        let frames = {
            let mut advance = self.app.world_mut().resource_mut::<AdvanceFrames>();
            std::mem::take(&mut advance.0)
        };
        self.advance(frames);
        output
    }

    pub fn advance(&mut self, frames: u64) {
        for _ in 0..frames {
            self.app.update();
        }
    }

    pub fn run_script(&mut self, path: &Path, keep_going: bool) -> Result<Transcript> {
        run_script_with_harness(self, path, keep_going)
    }

    pub fn world(&self) -> &World {
        self.app.world()
    }

    pub fn world_mut(&mut self) -> &mut World {
        self.app.world_mut()
    }
}
