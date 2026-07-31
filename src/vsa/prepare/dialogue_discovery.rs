#[path = "../dialogue/discovery.rs"]
mod implementation;

pub(crate) use implementation::{DialogueVoiceDiscovery, discover_dialogue_voice};
