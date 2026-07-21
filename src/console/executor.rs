use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use super::grammar::parse_command;
use super::{
    ConsoleCommandResult, ConsoleError, ConsoleInvocation, ConsoleOutput, ConsoleRegistry,
    ConsoleRequest, ConsoleSessionId,
};

#[derive(Resource, Default)]
pub(crate) struct ConsoleCounters {
    next_request_id: u64,
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct ConsoleFrame(pub u64);

#[derive(Resource, Default)]
pub struct ConsoleSessionStore {
    selected: HashMap<ConsoleSessionId, Entity>,
}

impl ConsoleSessionStore {
    pub fn selected(&self, session: &ConsoleSessionId) -> Option<Entity> {
        self.selected.get(session).copied()
    }

    pub fn select(&mut self, session: ConsoleSessionId, entity: Entity) {
        self.selected.insert(session, entity);
    }

    pub fn clear_entity(&mut self, entity: Entity) {
        self.selected.retain(|_, selected| *selected != entity);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefIdentity {
    pub form_id: Option<u32>,
    pub editor_id: Option<String>,
}

#[derive(Resource, Default)]
pub struct RefRegistry {
    by_form_id: HashMap<u32, Entity>,
    by_editor_id: HashMap<String, Vec<Entity>>,
    by_entity: HashMap<Entity, RefIdentity>,
    player: Option<Entity>,
}

impl RefRegistry {
    pub fn register(&mut self, entity: Entity, form_id: u32, editor_id: Option<&str>) {
        self.unregister(entity);
        if let Some(previous) = self.by_form_id.get(&form_id).copied()
            && previous != entity
        {
            self.unregister(previous);
        }
        self.by_form_id.insert(form_id, entity);
        if let Some(editor_id) = editor_id.filter(|value| !value.is_empty()) {
            self.by_editor_id
                .entry(editor_id.to_ascii_lowercase())
                .or_default()
                .push(entity);
        }
        self.by_entity.insert(
            entity,
            RefIdentity {
                form_id: Some(form_id),
                editor_id: editor_id.map(str::to_string),
            },
        );
    }

    pub fn set_player(&mut self, entity: Entity) {
        if let Some(previous) = self.player.replace(entity) {
            self.unregister(previous);
        }
        self.register(entity, 0x14, Some("player"));
        self.player = Some(entity);
    }

    pub fn clear_player(&mut self, entity: Entity) {
        if self.player == Some(entity) {
            self.player = None;
            self.unregister(entity);
        }
    }

    pub fn unregister(&mut self, entity: Entity) {
        let Some(identity) = self.by_entity.remove(&entity) else {
            return;
        };
        if let Some(form_id) = identity.form_id
            && self.by_form_id.get(&form_id) == Some(&entity)
        {
            self.by_form_id.remove(&form_id);
        }
        if let Some(editor_id) = identity.editor_id {
            let key = editor_id.to_ascii_lowercase();
            if let Some(entities) = self.by_editor_id.get_mut(&key) {
                entities.retain(|candidate| *candidate != entity);
                if entities.is_empty() {
                    self.by_editor_id.remove(&key);
                }
            }
        }
    }

    pub fn identity(&self, entity: Entity) -> Option<&RefIdentity> {
        self.by_entity.get(&entity)
    }

    /// Live entity currently registered for `form_id`, if any. Read-only
    /// lookup for per-frame systems (e.g. the cinema camera) that must
    /// re-resolve a tracked reference each frame without exclusive world
    /// access. Callers should still confirm the entity via their own query.
    pub fn entity_of(&self, form_id: u32) -> Option<Entity> {
        self.by_form_id.get(&form_id).copied()
    }

    pub fn label(&self, entity: Entity) -> String {
        self.identity(entity).map_or_else(
            || format!("entity {}", entity.to_bits()),
            |identity| match (&identity.editor_id, identity.form_id) {
                (Some(editor_id), Some(form_id)) => format!("{editor_id} ({form_id:08x})"),
                (Some(editor_id), None) => editor_id.clone(),
                (None, Some(form_id)) => format!("{form_id:08x}"),
                (None, None) => format!("entity {}", entity.to_bits()),
            },
        )
    }

    pub fn completion_names(&self) -> Vec<String> {
        let mut names = self
            .by_entity
            .values()
            .flat_map(|identity| {
                identity
                    .editor_id
                    .iter()
                    .cloned()
                    .chain(identity.form_id.map(|form_id| format!("{form_id:08x}")))
            })
            .collect::<Vec<_>>();
        names.sort_by_key(|name| name.to_ascii_lowercase());
        names.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
        names
    }

    fn resolve(&self, selector: &str) -> Result<Entity, ConsoleError> {
        if selector.eq_ignore_ascii_case("player") {
            return self.player.ok_or_else(|| {
                ConsoleError::new("reference_not_found", "the active player does not exist")
            });
        }
        if let Some(form_id) = parse_form_id(selector) {
            return self.by_form_id.get(&form_id).copied().ok_or_else(|| {
                ConsoleError::new(
                    "reference_not_found",
                    format!("FormID {form_id:08x} is not present in the live scene"),
                )
            });
        }
        let matches = self
            .by_editor_id
            .get(&selector.to_ascii_lowercase())
            .map(Vec::as_slice)
            .unwrap_or_default();
        match matches {
            [] => Err(ConsoleError::new(
                "reference_not_found",
                format!("EditorID '{selector}' is not present in the live scene"),
            )),
            [entity] => Ok(*entity),
            _ => Err(ConsoleError::new(
                "ambiguous_reference",
                format!(
                    "EditorID '{selector}' matches {} live references; use a FormID",
                    matches.len()
                ),
            )),
        }
    }
}

fn parse_form_id(selector: &str) -> Option<u32> {
    let digits = selector
        .strip_prefix("0x")
        .or_else(|| selector.strip_prefix("0X"))
        .unwrap_or(selector);
    ((selector.starts_with("0x") || selector.starts_with("0X") || digits.len() == 8)
        && digits.bytes().all(|byte| byte.is_ascii_hexdigit()))
    .then(|| u32::from_str_radix(digits, 16).ok())
    .flatten()
}

pub fn resolve_reference(world: &World, selector: &str) -> Result<Entity, ConsoleError> {
    let entity = world.resource::<RefRegistry>().resolve(selector)?;
    if world.entities().contains(entity) {
        Ok(entity)
    } else {
        Err(ConsoleError::new(
            "entity_unavailable",
            format!("reference '{selector}' no longer exists"),
        ))
    }
}

#[derive(Resource, Default)]
pub struct ConsoleEntityHooks {
    transform_mutated: Vec<fn(&mut World, Entity)>,
    angle_getters: Vec<fn(&World, Entity) -> Option<Vec3>>,
    angle_setters: Vec<fn(&mut World, Entity, Vec3) -> bool>,
}

impl ConsoleEntityHooks {
    pub fn register_transform_mutated(&mut self, hook: fn(&mut World, Entity)) {
        self.transform_mutated.push(hook);
    }

    pub fn register_angle_adapter(
        &mut self,
        getter: fn(&World, Entity) -> Option<Vec3>,
        setter: fn(&mut World, Entity, Vec3) -> bool,
    ) {
        self.angle_getters.push(getter);
        self.angle_setters.push(setter);
    }
}

pub(crate) fn notify_transform_mutated(world: &mut World, entity: Entity) {
    let hooks = world
        .resource::<ConsoleEntityHooks>()
        .transform_mutated
        .clone();
    for hook in hooks {
        hook(world, entity);
    }
}

pub(crate) fn adapted_angles(world: &World, entity: Entity) -> Option<Vec3> {
    world
        .resource::<ConsoleEntityHooks>()
        .angle_getters
        .iter()
        .find_map(|getter| getter(world, entity))
}

pub(crate) fn set_adapted_angles(world: &mut World, entity: Entity, angles: Vec3) -> bool {
    let setters = world.resource::<ConsoleEntityHooks>().angle_setters.clone();
    setters
        .into_iter()
        .any(|setter| setter(world, entity, angles))
}

#[derive(Clone, Debug)]
pub struct ConsoleQueuedResponse {
    pub request: ConsoleRequest,
    pub output: ConsoleOutput,
}

#[derive(Resource, Default)]
pub struct ConsoleQueue(pub VecDeque<ConsoleRequest>);

#[derive(Resource, Default)]
pub struct ConsoleResponses(pub VecDeque<ConsoleQueuedResponse>);

pub struct ConsoleExecutor;

impl ConsoleExecutor {
    pub fn execute(world: &mut World, request: ConsoleRequest) -> ConsoleOutput {
        let (request_id, frame) = {
            let frame = world.resource::<ConsoleFrame>().0;
            let mut counters = world.resource_mut::<ConsoleCounters>();
            counters.next_request_id = counters.next_request_id.saturating_add(1);
            (counters.next_request_id, frame)
        };

        let parsed = match parse_command(&request.line) {
            Ok(Some(parsed)) => parsed,
            Ok(None) => {
                return ConsoleOutput::success(request_id, frame, serde_json::Value::Null, vec![]);
            }
            Err(error) => return ConsoleOutput::failure(request_id, frame, error),
        };

        let (handler, reference_callable, requires_reference) = {
            let registry = world.resource::<ConsoleRegistry>();
            let Some(command) = registry.resolve(&parsed.name) else {
                return ConsoleOutput::failure(
                    request_id,
                    frame,
                    ConsoleError::new(
                        "unknown_command",
                        format!("unknown command '{}'", parsed.name),
                    ),
                );
            };
            (
                command.handler,
                command.metadata.reference_callable,
                command.requires_reference,
            )
        };

        if parsed.reference.is_some() && !reference_callable {
            return ConsoleOutput::failure(
                request_id,
                frame,
                ConsoleError::new(
                    "not_reference_callable",
                    format!(
                        "command '{}' does not accept an explicit reference",
                        parsed.name
                    ),
                ),
            );
        }

        let target = match parsed.reference.as_deref() {
            Some(selector) => match resolve_reference(world, selector) {
                Ok(entity) => Some(entity),
                Err(error) => return ConsoleOutput::failure(request_id, frame, error),
            },
            None if reference_callable => {
                let selected = world
                    .resource::<ConsoleSessionStore>()
                    .selected(&request.session);
                match selected {
                    Some(entity) if world.entities().contains(entity) => Some(entity),
                    Some(_) => {
                        return ConsoleOutput::failure(
                            request_id,
                            frame,
                            ConsoleError::new(
                                "entity_unavailable",
                                "the selected reference no longer exists",
                            ),
                        );
                    }
                    None if requires_reference => {
                        return ConsoleOutput::failure(
                            request_id,
                            frame,
                            ConsoleError::new(
                                "no_selection",
                                "select a reference with prid or use reference.command",
                            ),
                        );
                    }
                    None => None,
                }
            }
            None => None,
        };

        let invocation = ConsoleInvocation {
            request_id,
            frame,
            session: request.session,
            command: parsed.name,
            args: parsed.args,
            target,
        };
        match handler(world, &invocation) {
            Ok(ConsoleCommandResult { value, log }) => {
                ConsoleOutput::success(request_id, frame, value, log)
            }
            Err(error) => ConsoleOutput::failure(request_id, frame, error),
        }
    }
}

pub(crate) fn increment_console_frame(mut frame: ResMut<ConsoleFrame>) {
    frame.0 = frame.0.saturating_add(1);
}

pub(crate) fn drain_console_queue(world: &mut World) {
    let requests = {
        let mut queue = world.resource_mut::<ConsoleQueue>();
        std::mem::take(&mut queue.0)
    };
    for request in requests {
        let output = ConsoleExecutor::execute(world, request.clone());
        world
            .resource_mut::<ConsoleResponses>()
            .0
            .push_back(ConsoleQueuedResponse { request, output });
    }
}
