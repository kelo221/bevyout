//! Texture staging and conversion.

use super::*;
use std::ffi::OsString;

/// Converts a DirectX tangent-space normal texel to glTF/Bevy's convention.
/// Fallout stores specular strength in alpha, so only the Y (green) channel
/// may change here.
pub(crate) fn flip_directx_normal_y_texel(texel: &mut [u8; 4]) {
    texel[1] = 255 - texel[1];
}

/// Blender's repair path already identifies normal maps from these Fallout
/// filename conventions. Keep staging aligned with that behavior.
pub(crate) fn is_blender_normal_texture_path(path: impl AsRef<Path>) -> bool {
    let name = path
        .as_ref()
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    name.contains("_n.") || name.contains("normal")
}

pub(crate) fn staged_texture_conversion_required(dds: &Path, png_exists: bool) -> bool {
    !png_exists || is_blender_normal_texture_path(dds)
}

pub(crate) fn imagemagick_texture_arguments(
    dds: &Path,
    output: &Path,
    flip_normal_y: bool,
) -> Vec<OsString> {
    let mut arguments = vec![dds.as_os_str().to_owned()];
    if flip_normal_y {
        arguments.extend(
            ["-channel", "G", "-negate", "+channel"]
                .into_iter()
                .map(OsString::from),
        );
    }
    arguments.extend([OsString::from("-strip"), output.as_os_str().to_owned()]);
    arguments
}

pub(crate) fn stage_textures(
    nif_bytes: &[u8],
    data_root: &Path,
    archives: &[BsaArchive],
    staging_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    for texture in texture_references(nif_bytes) {
        let Some(bytes) = resolve_asset(data_root, archives, &texture)
            .with_context(|| format!("reading texture {texture}"))?
        else {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!("missing texture {texture}"),
            });
            continue;
        };
        let destination = staging_dir.join(texture.replace('/', std::path::MAIN_SEPARATOR_STR));
        if destination.exists() {
            continue;
        }
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(destination, bytes)?;
    }
    Ok(())
}

pub(crate) fn convert_staged_textures(
    staging_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let converter = {
        let installed = PathBuf::from(r"C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe");
        if installed.exists() {
            Some(installed)
        } else {
            ["magick.exe", "magick"]
                .into_iter()
                .find(|name| {
                    Command::new(name)
                        .arg("-version")
                        .output()
                        .is_ok_and(|output| output.status.success())
                })
                .map(PathBuf::from)
        }
    };
    let Some(converter) = converter else {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: "ImageMagick was not found; DDS textures will remain unconverted".into(),
        });
        return Ok(());
    };
    let mut dds_files = Vec::new();
    collect_files_with_extension(staging_dir, "dds", &mut dds_files)?;
    for dds in dds_files {
        let png = dds.with_extension("png");
        let flip_normal_y = is_blender_normal_texture_path(&dds);
        if !staged_texture_conversion_required(&dds, png.exists()) {
            fs::remove_file(&dds)?;
            continue;
        }
        let converted = if flip_normal_y {
            png.with_extension(format!("normal-y-{}.tmp.png", std::process::id()))
        } else {
            png.clone()
        };
        let result = Command::new(&converter)
            .args(imagemagick_texture_arguments(
                &dds,
                &converted,
                flip_normal_y,
            ))
            .output();
        match result {
            Ok(output) if output.status.success() => {
                if flip_normal_y {
                    atomic_replace(&converted, &png)?;
                }
                fs::remove_file(&dds)?;
            }
            Ok(output) => {
                let _ = fs::remove_file(&converted);
                diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!(
                        "could not convert {} to PNG: {}",
                        dds.display(),
                        String::from_utf8_lossy(&output.stderr).trim()
                    ),
                });
            }
            Err(error) => {
                let _ = fs::remove_file(&converted);
                diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!("could not run ImageMagick for {}: {error}", dds.display()),
                });
            }
        }
    }
    Ok(())
}

#[cfg(windows)]
pub(crate) fn atomic_replace(source: &Path, destination: &Path) -> Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    let source_display = source.display().to_string();
    let destination_display = destination.display().to_string();
    let source = source
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let destination = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let result = unsafe {
        MoveFileExW(
            source.as_ptr(),
            destination.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if result == 0 {
        return Err(std::io::Error::last_os_error()).with_context(|| {
            format!(
                "could not atomically replace {} with {}",
                destination_display, source_display
            )
        });
    }
    Ok(())
}

#[cfg(not(windows))]
pub(crate) fn atomic_replace(source: &Path, destination: &Path) -> Result<()> {
    fs::rename(source, destination).with_context(|| {
        format!(
            "could not atomically replace {} with {}",
            destination.display(),
            source.display()
        )
    })
}

fn collect_files_with_extension(
    root: &Path,
    extension: &str,
    output: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files_with_extension(&path, extension, output)?;
        } else if path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(extension))
        {
            output.push(path);
        }
    }
    Ok(())
}

pub(crate) fn texture_references(bytes: &[u8]) -> Vec<String> {
    const EXTENSIONS: [&str; 5] = [".dds", ".tga", ".bmp", ".png", ".jpg"];
    let mut found = HashSet::new();
    let mut start = None;
    let mut inspect = |run: &[u8]| {
        if run.len() < 5 {
            return;
        }
        let text = String::from_utf8_lossy(run);
        let lower = text.to_ascii_lowercase();
        for extension in EXTENSIONS {
            let mut search_from = 0;
            while let Some(relative) = lower[search_from..].find(extension) {
                let end = search_from + relative + extension.len();
                let prefix = &lower[..search_from + relative];
                let separator = prefix
                    .rfind('\\')
                    .max(prefix.rfind('/'))
                    .map(|index| index + 1);
                let candidate_start = prefix.rfind("textures").or(separator).unwrap_or(0);
                let candidate = normalize_asset_path(&text[candidate_start..end]);
                // Fallout asset folders legitimately contain spaces (for
                // example `textures/dungeons/wasteland homes`). Keep the
                // complete normalized path instead of dropping those valid
                // texture references.
                if candidate.ends_with(extension) && candidate.starts_with("textures/") {
                    found.insert(candidate);
                }
                search_from = end;
                if search_from >= lower.len() {
                    break;
                }
            }
        }
    };
    for (index, byte) in bytes.iter().copied().enumerate() {
        if (0x20..=0x7e).contains(&byte) {
            if start.is_none() {
                start = Some(index);
            }
        } else if let Some(begin) = start.take() {
            inspect(&bytes[begin..index]);
        }
    }
    if let Some(begin) = start {
        inspect(&bytes[begin..]);
    }
    found.into_iter().collect()
}
