//! External Blender and KTX tool discovery helpers.

use super::*;

pub(crate) fn find_ktx_tool(explicit: Option<PathBuf>) -> Result<Option<KtxTool>> {
    if let Some(path) = explicit {
        if path.exists() {
            return Ok(Some(KtxTool {
                kind: ktx_tool_kind(&path),
                path,
            }));
        }
        bail!("KTX executable does not exist: {}", path.display());
    }
    for (command, kind) in [
        ("toktx", KtxToolKind::LegacyToktx),
        ("ktx", KtxToolKind::UnifiedKtx),
    ] {
        if Command::new(command)
            .arg("--version")
            .output()
            .is_ok_and(|output| output.status.success())
        {
            return Ok(Some(KtxTool {
                path: PathBuf::from(command),
                kind,
            }));
        }
    }
    for path in [
        PathBuf::from(r"C:\Program Files\KTX-Software\bin\toktx.exe"),
        PathBuf::from(r"C:\Program Files\KTX-Software\bin\ktx.exe"),
    ] {
        if path.exists() {
            return Ok(Some(KtxTool {
                kind: ktx_tool_kind(&path),
                path,
            }));
        }
    }
    Ok(None)
}

pub(crate) fn find_irradiance_ktx_tool(explicit: Option<PathBuf>) -> Result<KtxTool> {
    if let Some(path) = explicit {
        let tool = KtxTool {
            kind: ktx_tool_kind(&path),
            path,
        };
        if !matches!(tool.kind, KtxToolKind::UnifiedKtx) {
            bail!(
                "irradiance volume export requires the unified KTX executable (ktx.exe), not legacy toktx.exe"
            );
        }
        if !tool.path.exists() {
            bail!("KTX executable does not exist: {}", tool.path.display());
        }
        return Ok(tool);
    }
    for path in [
        PathBuf::from("ktx"),
        PathBuf::from(r"C:\Program Files\KTX-Software\bin\ktx.exe"),
    ] {
        if (path.is_absolute() && path.exists())
            || (!path.is_absolute()
                && Command::new(&path)
                    .arg("--version")
                    .output()
                    .is_ok_and(|output| output.status.success()))
        {
            return Ok(KtxTool {
                kind: KtxToolKind::UnifiedKtx,
                path,
            });
        }
    }
    let Some(tool) = find_ktx_tool(None)? else {
        bail!(
            "KTX-Software was not found; install the unified ktx.exe or pass --toktx with its path"
        );
    };
    if !matches!(tool.kind, KtxToolKind::UnifiedKtx) {
        bail!(
            "irradiance volume export requires the unified KTX executable (ktx.exe), not legacy toktx.exe"
        );
    }
    Ok(tool)
}

pub(crate) fn find_irradiance_blender(
    irradiance_explicit: Option<&Path>,
    general_explicit: Option<&Path>,
) -> Result<PathBuf> {
    if let Some(path) = irradiance_explicit {
        validate_blender_45(path)?;
        return Ok(path.to_path_buf());
    }
    if let Some(path) = general_explicit {
        if !path.exists() {
            bail!("Blender executable does not exist: {}", path.display());
        }
        if validate_blender_45(path).is_ok() {
            return Ok(path.to_path_buf());
        }
        let preferred =
            PathBuf::from(r"C:\Program Files\Blender Foundation\Blender 4.5\blender.exe");
        if preferred.exists() {
            validate_blender_45(&preferred)?;
            println!(
                "configured Blender {} is not 4.5 LTS; using {} for irradiance baking",
                path.display(),
                preferred.display()
            );
            return Ok(preferred);
        }
        validate_blender_45(path)?;
        unreachable!("validated Blender path must return above")
    }
    let preferred = PathBuf::from(r"C:\Program Files\Blender Foundation\Blender 4.5\blender.exe");
    if preferred.exists() {
        validate_blender_45(&preferred)?;
        return Ok(preferred);
    }
    let blender = find_blender(None)?;
    validate_blender_45(&blender)?;
    Ok(blender)
}

pub(crate) fn validate_blender_45(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("Blender executable does not exist: {}", path.display());
    }
    let output = Command::new(path)
        .arg("--version")
        .output()
        .with_context(|| format!("could not run Blender at {}", path.display()))?;
    let version = String::from_utf8_lossy(&output.stdout);
    if !output.status.success()
        || !version
            .lines()
            .next()
            .is_some_and(|line| line.contains("4.5"))
    {
        bail!(
            "irradiance-volume baking requires Blender 4.5 LTS; {} reported {}",
            path.display(),
            version.lines().next().unwrap_or("an unknown version")
        );
    }
    Ok(())
}

pub(crate) fn ktx_tool_kind(path: &Path) -> KtxToolKind {
    if path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .is_some_and(|stem| stem.eq_ignore_ascii_case("ktx"))
    {
        KtxToolKind::UnifiedKtx
    } else {
        KtxToolKind::LegacyToktx
    }
}

pub(crate) fn relative_asset_path(root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)
        .with_context(|| {
            format!(
                "{} is outside asset root {}",
                path.display(),
                root.display()
            )
        })?
        .to_string_lossy()
        .replace('\\', "/"))
}

pub(crate) fn blender_path(path: &Path) -> String {
    let value = path.to_string_lossy();
    value.strip_prefix(r"\\?\").unwrap_or(&value).to_owned()
}

pub(crate) fn tail(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let lines = text.lines().collect::<Vec<_>>();
    lines
        .iter()
        .rev()
        .take(40)
        .copied()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}
