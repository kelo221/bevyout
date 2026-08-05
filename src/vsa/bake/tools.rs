//! External KTX tool discovery helpers.

use super::*;
use std::process::Command;

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

pub(crate) fn find_unified_ktx_tool(explicit: Option<PathBuf>) -> Result<KtxTool> {
    if let Some(path) = explicit {
        let tool = KtxTool {
            kind: ktx_tool_kind(&path),
            path,
        };
        if !matches!(tool.kind, KtxToolKind::UnifiedKtx) {
            bail!(
                "prepared texture export requires the unified KTX executable (ktx.exe), not legacy toktx.exe"
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
            "prepared texture export requires the unified KTX executable (ktx.exe), not legacy toktx.exe"
        );
    }
    Ok(tool)
}

/// KTX 5 added `@file` input lists, which avoid exceeding Windows' process
/// command-line limit when a cubemap array has many faces.
pub(crate) fn ktx_supports_input_file_lists(path: &Path) -> bool {
    let Ok(output) = Command::new(path).arg("--version").output() else {
        return false;
    };
    let version = String::from_utf8_lossy(&output.stdout);
    version
        .split(|character: char| !character.is_ascii_digit())
        .find(|part| !part.is_empty())
        .and_then(|major| major.parse::<u32>().ok())
        .is_some_and(|major| major >= 5)
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

pub(crate) fn job_path(path: &Path) -> String {
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
