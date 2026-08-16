mod lock;
mod model;
mod output;
mod policy;
mod scan;

use crate::cli::CacheGcArgs;
use anyhow::{Context, Result};
use std::fs;

pub(crate) fn cache_gc(args: CacheGcArgs) -> Result<()> {
    let cache_root = fs::canonicalize(&args.cache)
        .with_context(|| format!("could not resolve cache root {}", args.cache.display()))?;
    let _lock = lock::CacheGcLock::acquire(&cache_root)?;
    let mut report = scan::plan_gc(
        &cache_root,
        args.dry_run,
        args.grace_hours,
        args.include_rebuildable,
    )?;
    scan::sweep(&mut report)?;
    output::print_report(&report);
    if let Some(path) = args.json.as_deref() {
        output::write_json(path, &report)?;
        println!("cache gc report: json {}", path.display());
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
