use anyhow::Result;
use clap::Parser;

use bevyout::{Cli, CommandLine, apply, bake, prepare, render, view};

fn main() -> Result<()> {
    let mut cli = Cli::parse();
    apply(&mut cli)?;
    match cli.command {
        CommandLine::Prepare(args) => prepare(args),
        CommandLine::Bake(args) => bake(args),
        CommandLine::Render(args) => render(args),
        CommandLine::View(args) => view(args),
    }
}
