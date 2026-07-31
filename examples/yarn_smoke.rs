//! Wave 0 Yarn compatibility smoke. It uses the Bevy-free Yarn compiler and
//! runtime so enabling this example cannot pull a second Bevy version.

use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use yarnspinner::prelude::{Dialogue, DialogueEvent, YarnCompiler};
use yarnspinner::runtime::{MemoryVariableStorage, StringTableTextProvider};

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("features")
        .join("fixtures")
        .join("dialogue_wave0.yarn");
    let compilation = YarnCompiler::new()
        .read_file(path)
        .compile()
        .expect("Yarn compiles");
    let mut text_provider = StringTableTextProvider::new();
    text_provider.extend_base_language(
        compilation
            .string_table
            .iter()
            .map(|(id, info)| (id.clone(), info.text.clone())),
    );
    let mut dialogue = Dialogue::new(
        Box::new(MemoryVariableStorage::new()),
        Box::new(text_provider),
    );
    dialogue.replace_program(compilation.program.expect("Yarn program"));
    let function_called = Arc::new(AtomicBool::new(false));
    let function_called_by_yarn = Arc::clone(&function_called);
    dialogue.library_mut().add_function("bo_smoke", move || {
        function_called_by_yarn.store(true, Ordering::Relaxed);
        true
    });
    dialogue.set_node("Start").expect("Start node exists");

    let mut saw_line = false;
    let mut saw_options = false;
    let mut saw_command = false;
    let mut saw_complete = false;
    while dialogue.can_continue() {
        let events = dialogue.continue_().expect("Yarn runtime continues");
        for event in events {
            match event {
                DialogueEvent::Line(_) => saw_line = true,
                DialogueEvent::Options(options) => {
                    saw_options = true;
                    dialogue
                        .set_selected_option(options[0].id)
                        .expect("option selection succeeds");
                }
                DialogueEvent::Command(_) => saw_command = true,
                DialogueEvent::DialogueComplete => saw_complete = true,
                DialogueEvent::NodeComplete(_)
                | DialogueEvent::NodeStart(_)
                | DialogueEvent::LineHints(_) => {}
            }
        }
        if saw_complete {
            break;
        }
    }
    assert!(
        saw_line
            && saw_options
            && saw_command
            && function_called.load(Ordering::Relaxed)
            && saw_complete
    );
    println!("dialogue-yarn smoke: line/function/command/options/completion passed");
}
