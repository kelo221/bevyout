mod cli;
mod viewer;
mod vsa;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, CommandLine};

fn main() -> Result<()> {
    match Cli::parse().command {
        CommandLine::Prepare(args) => vsa::prepare(args),
        CommandLine::View(args) => viewer::view(args),
    }
}
