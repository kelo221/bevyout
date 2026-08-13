use anyhow::Result;
use clap::Parser;

use bevyout::{
    Cli, CommandLine, animation_zoo, apply, bake, cache, cells, exterior_catalog,
    exterior_conversion_report, nif_convert, prepare, ragdoll_lab, render, report, script, view,
};

fn main() -> Result<()> {
    let mut cli = Cli::parse();
    apply(&mut cli)?;
    match cli.command {
        CommandLine::Cache(args) => cache(args),
        CommandLine::Prepare(args) => prepare(args),
        CommandLine::Bake(args) => bake(args),
        CommandLine::Render(args) => render(args),
        CommandLine::View(args) => view(args),
        CommandLine::RagdollLab(args) => ragdoll_lab(args),
        CommandLine::AnimationZoo(args) => animation_zoo(args),
        CommandLine::Report(args) => report(args),
        CommandLine::Cells(args) => cells(args),
        CommandLine::Script(args) => script(args),
        CommandLine::NifConvert(args) => nif_convert(args),
        CommandLine::ExteriorConversionReport(args) => exterior_conversion_report(args),
        CommandLine::ExteriorCatalog(args) => exterior_catalog(args),
    }
}
