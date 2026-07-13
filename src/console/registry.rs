use std::collections::{BTreeMap, HashMap};

use bevy::prelude::*;
use serde::Serialize;

use super::{ConsoleCommandResult, ConsoleError, ConsoleInvocation};

pub type ConsoleHandler =
    fn(&mut World, &ConsoleInvocation) -> Result<ConsoleCommandResult, ConsoleError>;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ConsoleCommandMetadata {
    pub name: String,
    pub aliases: Vec<String>,
    pub signature: String,
    pub help: String,
    pub reference_callable: bool,
    pub mutating: bool,
}

#[derive(Clone)]
pub struct ConsoleCommand {
    pub metadata: ConsoleCommandMetadata,
    pub requires_reference: bool,
    pub handler: ConsoleHandler,
}

impl ConsoleCommand {
    pub fn new(
        name: impl Into<String>,
        signature: impl Into<String>,
        help: impl Into<String>,
        handler: ConsoleHandler,
    ) -> Self {
        let name = name.into().to_ascii_lowercase();
        Self {
            metadata: ConsoleCommandMetadata {
                signature: signature.into(),
                help: help.into(),
                name,
                aliases: Vec::new(),
                reference_callable: false,
                mutating: false,
            },
            requires_reference: false,
            handler,
        }
    }

    pub fn aliases(mut self, aliases: &[&str]) -> Self {
        self.metadata.aliases = aliases
            .iter()
            .map(|alias| alias.to_ascii_lowercase())
            .collect();
        self
    }

    pub fn reference_callable(mut self, required: bool) -> Self {
        self.metadata.reference_callable = true;
        self.requires_reference = required;
        self
    }

    pub fn mutating(mut self) -> Self {
        self.metadata.mutating = true;
        self
    }
}

#[derive(Resource, Default)]
pub struct ConsoleRegistry {
    commands: BTreeMap<String, ConsoleCommand>,
    aliases: HashMap<String, String>,
}

impl ConsoleRegistry {
    pub fn register(&mut self, command: ConsoleCommand) -> Result<(), ConsoleError> {
        let name = command.metadata.name.clone();
        if self.commands.contains_key(&name) || self.aliases.contains_key(&name) {
            return Err(ConsoleError::new(
                "duplicate_command",
                format!("console command '{name}' is already registered"),
            ));
        }
        for alias in &command.metadata.aliases {
            if self.commands.contains_key(alias) || self.aliases.contains_key(alias) {
                return Err(ConsoleError::new(
                    "duplicate_command",
                    format!("console command alias '{alias}' is already registered"),
                ));
            }
        }
        for alias in &command.metadata.aliases {
            self.aliases.insert(alias.clone(), name.clone());
        }
        self.commands.insert(name, command);
        Ok(())
    }

    pub(crate) fn resolve(&self, name: &str) -> Option<&ConsoleCommand> {
        let canonical = self.aliases.get(name).map_or(name, String::as_str);
        self.commands.get(canonical)
    }

    pub fn metadata(&self) -> Vec<ConsoleCommandMetadata> {
        self.commands
            .values()
            .map(|command| command.metadata.clone())
            .collect()
    }

    pub fn completion_names(&self) -> Vec<String> {
        let mut names = self
            .commands
            .values()
            .flat_map(|command| {
                std::iter::once(command.metadata.name.clone())
                    .chain(command.metadata.aliases.iter().cloned())
            })
            .collect::<Vec<_>>();
        names.sort_by_key(|name| name.to_ascii_lowercase());
        names
    }
}
