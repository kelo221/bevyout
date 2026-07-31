//! Optional YarnSpinner compiler boundary for the Bevy 0.19 dialogue adapter.
//!
//! The Bevyout runtime state machine remains Yarn-free in the default build;
//! this module is enabled only by `dialogue-yarn` and validates the explicit
//! prepared source list before a bundle becomes runnable.

use std::fs;
use std::path::Path;

use crate::vsa::dialogue::DialogueDiagnostic;
use yarnspinner::prelude::{YarnCompiler, YarnFile, YarnProgram};

pub(crate) fn compile_sources(
    asset_root: &Path,
    source_paths: &[String],
) -> Result<YarnProgram, Vec<DialogueDiagnostic>> {
    let mut compiler = YarnCompiler::new();
    let mut diagnostics = Vec::new();
    for source_path in source_paths {
        let relative = source_path.strip_prefix("dialogue/").unwrap_or(source_path);
        let path = asset_root.join("dialogue").join(relative);
        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(error) => {
                diagnostics.push(DialogueDiagnostic {
                    severity: "error".into(),
                    code: "source_read".into(),
                    source_path: Some(source_path.clone()),
                    line: None,
                    message: error.to_string(),
                });
                continue;
            }
        };
        compiler.add_file(YarnFile {
            file_name: source_path.clone(),
            source,
        });
    }
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    match compiler.compile() {
        Ok(compilation) => compilation.program.ok_or_else(|| {
            vec![DialogueDiagnostic {
                severity: "error".into(),
                code: "missing_program".into(),
                source_path: None,
                line: None,
                message: "Yarn compiler returned no program".into(),
            }]
        }),
        Err(error) => Err(vec![DialogueDiagnostic {
            severity: "error".into(),
            code: "yarn_compile".into(),
            source_path: None,
            line: None,
            message: error.to_string(),
        }]),
    }
}
