//! Pure console history and completion helpers adapted from OpenMW's console UX.

use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::Path;

pub const HISTORY_LIMIT: usize = 200;
pub const COMPLETION_LIST_LIMIT: usize = 50;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandHistory {
    entries: VecDeque<String>,
    cursor: Option<usize>,
    draft: String,
}

impl CommandHistory {
    pub fn from_entries(entries: impl IntoIterator<Item = String>) -> Self {
        let mut history = Self::default();
        for entry in entries {
            history.record(entry);
        }
        history
    }

    pub fn entries(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(String::as_str)
    }

    pub fn record(&mut self, command: impl Into<String>) {
        let command = command.into();
        if command.is_empty() {
            return;
        }
        if self.entries.back() != Some(&command) {
            self.entries.push_back(command);
            while self.entries.len() > HISTORY_LIMIT {
                self.entries.pop_front();
            }
        }
        self.cursor = None;
        self.draft.clear();
    }

    pub fn up(&mut self, current: &str) -> String {
        if self.entries.is_empty() {
            return current.to_string();
        }
        let index = match self.cursor {
            None => {
                self.draft = current.to_string();
                self.entries.len() - 1
            }
            Some(0) => 0,
            Some(index) => index - 1,
        };
        self.cursor = Some(index);
        self.entries[index].clone()
    }

    pub fn down(&mut self, current: &str) -> String {
        let Some(index) = self.cursor else {
            return current.to_string();
        };
        if index + 1 < self.entries.len() {
            self.cursor = Some(index + 1);
            self.entries[index + 1].clone()
        } else {
            self.cursor = None;
            std::mem::take(&mut self.draft)
        }
    }
}

pub fn load_history(path: &Path) -> io::Result<CommandHistory> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(CommandHistory::from_entries(
            text.lines().map(str::to_string),
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(CommandHistory::default()),
        Err(error) => Err(error),
    }
}

pub fn save_history(path: &Path, history: &CommandHistory) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut contents = history.entries().collect::<Vec<_>>().join("\n");
    if !contents.is_empty() {
        contents.push('\n');
    }
    fs::write(path, contents)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompletionState {
    last_completed_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionResult {
    pub text: String,
    pub matches: Vec<String>,
    pub list_candidates: bool,
}

impl CompletionState {
    pub fn reset(&mut self) {
        self.last_completed_text = None;
    }

    pub fn complete(
        &mut self,
        input: &str,
        candidates: impl IntoIterator<Item = String>,
    ) -> CompletionResult {
        let (prefix, fragment) = completion_fragment(input);
        let mut matches = candidates
            .into_iter()
            .filter(|candidate| {
                candidate
                    .to_ascii_lowercase()
                    .starts_with(&fragment.to_ascii_lowercase())
            })
            .collect::<Vec<_>>();
        matches.sort_by_key(|candidate| candidate.to_ascii_lowercase());
        matches.dedup_by(|left, right| left.eq_ignore_ascii_case(right));

        let text = match matches.as_slice() {
            [] => input.to_string(),
            [only] => format!("{prefix}{only} "),
            _ => format!("{prefix}{}", longest_common_prefix(&matches)),
        };
        let repeated = self.last_completed_text.as_deref() == Some(input);
        let list_candidates = repeated && text == input && !matches.is_empty();
        self.last_completed_text = Some(text.clone());
        CompletionResult {
            text,
            matches: matches.into_iter().take(COMPLETION_LIST_LIMIT).collect(),
            list_candidates,
        }
    }
}

fn completion_fragment(input: &str) -> (&str, &str) {
    let start = input
        .char_indices()
        .rev()
        .find_map(|(index, character)| character.is_whitespace().then_some(index + 1))
        .unwrap_or(0);
    (&input[..start], &input[start..])
}

fn longest_common_prefix(matches: &[String]) -> String {
    let first = &matches[0];
    let mut end = first.len();
    for candidate in &matches[1..] {
        let shared = first
            .bytes()
            .zip(candidate.bytes())
            .take_while(|(left, right)| left.eq_ignore_ascii_case(right))
            .count();
        end = end.min(shared);
    }
    first[..end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_restores_unfinished_draft() {
        let mut history = CommandHistory::from_entries(["help".into(), "getpos z".into()]);
        assert_eq!(history.up("setpos "), "getpos z");
        assert_eq!(history.up("ignored"), "help");
        assert_eq!(history.down("ignored"), "getpos z");
        assert_eq!(history.down("ignored"), "setpos ");
    }

    #[test]
    fn history_caps_and_deduplicates_consecutive_entries() {
        let mut history = CommandHistory::default();
        for index in 0..=HISTORY_LIMIT {
            history.record(format!("command {index}"));
        }
        history.record(format!("command {HISTORY_LIMIT}"));
        let entries = history.entries().collect::<Vec<_>>();
        assert_eq!(entries.len(), HISTORY_LIMIT);
        assert_eq!(entries[0], "command 1");
        assert_eq!(entries.last(), Some(&"command 200"));
    }

    #[test]
    fn completion_uses_longest_prefix_then_lists_on_repeated_tab() {
        let candidates = vec!["getpos".into(), "getformid".into(), "getangle".into()];
        let mut completion = CompletionState::default();
        let first = completion.complete("get", candidates.clone());
        assert_eq!(first.text, "get");
        assert!(!first.list_candidates);
        let second = completion.complete("get", candidates);
        assert!(second.list_candidates);
        assert_eq!(second.matches.len(), 3);
    }
}
