use std::fs;
use std::io::{self, Write};
use std::path::Path;

use anyhow::{Context, Result, bail};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::cli::{ScriptArgs, ScriptCommand, ScriptRunArgs};

use super::grammar::parse_command;
use super::harness::ConsoleHarness;
use super::{
    ConsoleCommand, ConsoleCommandResult, ConsoleError, ConsoleExecutor, ConsoleInvocation,
    ConsoleOutput, ConsoleRegistry, ConsoleRequest,
};

#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConsoleExecutionMode {
    #[default]
    Interactive,
    Harness,
}

#[derive(Resource, Default)]
pub struct AdvanceFrames(pub u64);

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ConsoleRng {
    pub seed: u64,
    pub state: u64,
}

pub(crate) fn register_script_commands(registry: &mut ConsoleRegistry) {
    for command in [
        ConsoleCommand::new(
            "advance",
            "advance <frames>",
            "Advance a deterministic headless harness by exact 1/60-second frames.",
            advance,
        )
        .mutating(),
        ConsoleCommand::new(
            "seed",
            "seed <u64>",
            "Reset the centralized deterministic console RNG seed.",
            seed,
        )
        .mutating(),
        ConsoleCommand::new(
            "expect",
            "expect (<command>) <op> <literal> [tol <value>]",
            "Run a non-mutating command and assert its scalar result.",
            expect,
        ),
    ] {
        registry
            .register(command)
            .expect("script command is unique");
    }
}

fn advance(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "advance expects exactly one frame count",
        ));
    }
    if *world.resource::<ConsoleExecutionMode>() != ConsoleExecutionMode::Harness {
        return Err(ConsoleError::new(
            "unsupported",
            "advance is available only in the deterministic script harness",
        ));
    }
    let frames = invocation.args[0]
        .parse::<u64>()
        .map_err(|_| ConsoleError::new("bad_type", "frames must be an unsigned integer"))?;
    if frames > 1_000_000 {
        return Err(ConsoleError::new(
            "out_of_range",
            "advance is limited to 1000000 frames per command",
        ));
    }
    world.resource_mut::<AdvanceFrames>().0 = frames;
    Ok(ConsoleCommandResult::value(json!({ "frames": frames })))
}

fn seed(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "seed expects exactly one unsigned integer",
        ));
    }
    let seed = invocation.args[0]
        .parse::<u64>()
        .map_err(|_| ConsoleError::new("bad_type", "seed must be an unsigned integer"))?;
    *world.resource_mut::<ConsoleRng>() = ConsoleRng { seed, state: seed };
    Ok(ConsoleCommandResult::value(json!({ "seed": seed })))
}

fn expect(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let expression = invocation.args.join(" ");
    let Some(inner_end) = expression.find(')') else {
        return Err(ConsoleError::new(
            "expect_syntax",
            "expect requires a parenthesized command",
        ));
    };
    let Some(inner) = expression
        .strip_prefix('(')
        .map(|value| &value[..inner_end - 1])
    else {
        return Err(ConsoleError::new(
            "expect_syntax",
            "expect requires a parenthesized command",
        ));
    };
    let remainder = expression[inner_end + 1..]
        .split_whitespace()
        .collect::<Vec<_>>();
    if remainder.len() != 2 && remainder.len() != 4 {
        return Err(ConsoleError::new(
            "expect_syntax",
            "expected: expect (<command>) <op> <literal> [tol <value>]",
        ));
    }
    if remainder.len() == 4 && !remainder[2].eq_ignore_ascii_case("tol") {
        return Err(ConsoleError::new(
            "expect_syntax",
            "optional tolerance must use 'tol <value>'",
        ));
    }

    let parsed = parse_command(inner)?
        .ok_or_else(|| ConsoleError::new("expect_syntax", "the parenthesized command is empty"))?;
    let mutating = world
        .resource::<ConsoleRegistry>()
        .resolve(&parsed.name)
        .map(|command| command.metadata.mutating)
        .ok_or_else(|| {
            ConsoleError::new(
                "unknown_command",
                format!("unknown command '{}'", parsed.name),
            )
        })?;
    if mutating {
        return Err(ConsoleError::new(
            "expect_mutating_command",
            "expect accepts only non-mutating commands",
        ));
    }

    let nested = ConsoleExecutor::execute(
        world,
        ConsoleRequest {
            session: invocation.session.clone(),
            line: inner.to_string(),
        },
    );
    if !nested.ok {
        return Err(ConsoleError::new(
            "expect_command_failed",
            nested
                .error
                .map(|error| error.message)
                .unwrap_or_else(|| "nested command failed".into()),
        ));
    }
    let expected = parse_literal(remainder[1]);
    let tolerance = if remainder.len() == 4 {
        Some(
            remainder[3]
                .parse::<f64>()
                .map_err(|_| ConsoleError::new("bad_type", "expect tolerance must be a number"))?,
        )
    } else {
        None
    };
    if tolerance.is_some_and(|value| !value.is_finite() || value < 0.0) {
        return Err(ConsoleError::new(
            "bad_type",
            "expect tolerance must be finite and non-negative",
        ));
    }
    let passed = compare_values(&nested.value, remainder[0], &expected, tolerance)?;
    let result = json!({
        "actual": nested.value,
        "expected": expected,
        "operator": remainder[0],
        "tolerance": tolerance,
    });
    if passed {
        Ok(ConsoleCommandResult::new(
            result,
            vec!["expect passed".into()],
        ))
    } else {
        Err(ConsoleError::new(
            "assertion_failed",
            format!(
                "expectation failed: {} {} {}",
                result["actual"], remainder[0], result["expected"]
            ),
        ))
    }
}

fn parse_literal(value: &str) -> Value {
    serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_string()))
}

fn compare_values(
    actual: &Value,
    operator: &str,
    expected: &Value,
    tolerance: Option<f64>,
) -> Result<bool, ConsoleError> {
    if matches!(operator, "==" | "!=") {
        let equal = match (actual.as_f64(), expected.as_f64()) {
            (Some(actual), Some(expected)) => (actual - expected).abs() <= tolerance.unwrap_or(0.0),
            _ => actual == expected,
        };
        return Ok(if operator == "==" { equal } else { !equal });
    }
    let actual = actual.as_f64().ok_or_else(|| {
        ConsoleError::new(
            "expect_type",
            "ordered comparisons require a numeric result",
        )
    })?;
    let expected = expected.as_f64().ok_or_else(|| {
        ConsoleError::new(
            "expect_type",
            "ordered comparisons require a numeric literal",
        )
    })?;
    let tolerance = tolerance.unwrap_or(0.0);
    Ok(match operator {
        "<" => actual < expected || (actual - expected).abs() <= tolerance,
        ">" => actual > expected || (actual - expected).abs() <= tolerance,
        "<=" => actual <= expected + tolerance,
        ">=" => actual + tolerance >= expected,
        _ => {
            return Err(ConsoleError::new(
                "expect_operator",
                format!("unsupported expect operator '{operator}'"),
            ));
        }
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TranscriptRecord {
    pub line_no: usize,
    pub input: String,
    pub output: ConsoleOutput,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Transcript {
    pub records: Vec<TranscriptRecord>,
}

impl Transcript {
    pub fn to_jsonl(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        for record in &self.records {
            serde_json::to_writer(&mut bytes, record)?;
            bytes.push(b'\n');
        }
        Ok(bytes)
    }

    pub fn has_failures(&self) -> bool {
        self.records.iter().any(|record| !record.output.ok)
    }
}

pub fn run_script_with_harness(
    harness: &mut ConsoleHarness,
    path: &Path,
    keep_going: bool,
) -> Result<Transcript> {
    let source = fs::read_to_string(path)
        .with_context(|| format!("reading console script {}", path.display()))?;
    let mut transcript = Transcript::default();
    for (index, line) in source.lines().enumerate() {
        if line.trim().is_empty() || line.trim_start().starts_with(';') {
            continue;
        }
        let output = harness.exec(line);
        let failed = !output.ok;
        transcript.records.push(TranscriptRecord {
            line_no: index + 1,
            input: line.to_string(),
            output,
        });
        if failed && !keep_going {
            break;
        }
    }
    Ok(transcript)
}

pub fn run(args: ScriptRunArgs) -> Result<()> {
    let _headless = args.headless;
    let mut harness = ConsoleHarness::synthetic();
    let transcript = harness.run_script(&args.file, args.keep_going)?;
    let bytes = transcript.to_jsonl()?;
    if let Some(path) = &args.transcript {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, &bytes)
            .with_context(|| format!("writing transcript {}", path.display()))?;
    } else {
        io::stdout().write_all(&bytes)?;
    }
    if transcript.has_failures() {
        bail!("console script failed; see transcript for structured errors");
    }
    Ok(())
}

pub fn dispatch(args: ScriptArgs) -> Result<()> {
    match args.command {
        ScriptCommand::Run(args) => run(args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::ConsoleFrame;

    fn temp_script(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "bevyout-console-{name}-{}.bscript",
            std::process::id()
        ));
        fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn expect_covers_pass_failure_and_tolerance_edges() {
        let mut harness = ConsoleHarness::synthetic();
        assert!(harness.exec("prid TestCrate").ok);
        assert!(harness.exec("expect (getpos x) == 1").ok);
        assert!(harness.exec("expect (getpos x) == 1.001 tol 0.0011").ok);
        let failed = harness.exec("expect (getpos x) == 2");
        assert_eq!(failed.error.unwrap().code, "assertion_failed");
        let mutating = harness.exec("expect (setpos x 4) == 4");
        assert_eq!(mutating.error.unwrap().code, "expect_mutating_command");
        assert_eq!(harness.exec("getpos x").value, json!(1.0));
    }

    #[test]
    fn advance_runs_exactly_the_requested_fixed_frames() {
        let mut harness = ConsoleHarness::synthetic();
        let before = harness.world().resource::<ConsoleFrame>().0;
        assert!(harness.exec("advance 7").ok);
        let after = harness.world().resource::<ConsoleFrame>().0;
        assert_eq!(after - before, 7);
    }

    #[test]
    fn transcripts_are_byte_deterministic_and_keep_going_is_observable() {
        let path = temp_script(
            "determinism",
            "prid TestCrate\ngetpos\nexpect (getpos x) == 9\ngetpos z\n",
        );
        let mut first = ConsoleHarness::synthetic();
        let stopped = first.run_script(&path, false).unwrap();
        assert_eq!(stopped.records.len(), 3);
        let mut second = ConsoleHarness::synthetic();
        let kept = second.run_script(&path, true).unwrap();
        assert_eq!(kept.records.len(), 4);
        let mut third = ConsoleHarness::synthetic();
        let kept_again = third.run_script(&path, true).unwrap();
        assert_eq!(kept.to_jsonl().unwrap(), kept_again.to_jsonl().unwrap());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn committed_golden_matches_or_regenerates_explicitly() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let script = root.join("tests/console_scripts/basic.bscript");
        let golden = root.join("tests/goldens/basic.jsonl");
        let mut harness = ConsoleHarness::synthetic();
        let actual = harness
            .run_script(&script, false)
            .unwrap()
            .to_jsonl()
            .unwrap();
        if std::env::var_os("UPDATE_GOLDENS").is_some() {
            fs::write(&golden, actual).unwrap();
        } else {
            assert_eq!(actual, fs::read(golden).unwrap());
        }
    }
}
