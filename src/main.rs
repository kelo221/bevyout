use anyhow::Result;
use clap::Parser;

use bevyout::{
    Cli, CommandLine, apply, bake, cells, nif_convert, prepare, render, report, script, view,
};

fn main() -> Result<()> {
    let mut cli = Cli::parse();
    apply(&mut cli)?;
    match cli.command {
        CommandLine::Prepare(args) => prepare(args),
        CommandLine::Bake(args) => bake(args),
        CommandLine::Render(args) => render(args),
        CommandLine::View(args) => view(args),
        CommandLine::Report(args) => report(args),
        CommandLine::Cells(args) => cells(args),
        CommandLine::Script(args) => script(args),
        CommandLine::NifConvert(args) => nif_convert(args),
    }
}
