mod cli;
mod config;
mod viewer;
mod vsa;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, CommandLine};

fn main() -> Result<()> {
    let mut cli = Cli::parse();
    config::apply(&mut cli)?;
    match cli.command {
        CommandLine::Prepare(args) => vsa::prepare(args),
        CommandLine::Bake(args) => vsa::bake(args),
        CommandLine::View(args) => viewer::view(args),
    }
}
