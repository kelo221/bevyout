use anyhow::{Context, Result, bail};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs::{self, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub(super) struct ScannedFile {
    pub(super) absolute_path: PathBuf,
    pub(super) relative_path: String,
    pub(super) logical_bytes: u64,
    pub(super) allocated_bytes: u64,
    pub(super) payload_id: String,
    pub(super) sha256: Option<String>,
}

#[derive(Debug)]
pub(super) struct CacheScan {
    pub(super) cache_root: PathBuf,
    pub(super) directory_count: u64,
    pub(super) files: Vec<ScannedFile>,
}

pub(super) fn scan_cache(cache_root: &Path, manifest_set: Option<&Path>) -> Result<CacheScan> {
    let cache_root = fs::canonicalize(cache_root)
        .with_context(|| format!("could not resolve cache root {}", cache_root.display()))?;
    if !cache_root.is_dir() {
        bail!("cache root is not a directory: {}", cache_root.display());
    }

    let selected_paths = match manifest_set {
        Some(path) => Some(resolve_manifest_set(&cache_root, path)?),
        None => None,
    };
    let mut directories = BTreeSet::new();
    let mut file_paths = BTreeSet::new();
    match selected_paths {
        Some(mut paths) => {
            expand_glb_external_references(&cache_root, &mut paths)?;
            for path in paths {
                walk_path(&path, &mut directories, &mut file_paths)?;
            }
        }
        None => walk_path(&cache_root, &mut directories, &mut file_paths)?,
    }

    let mut files = Vec::with_capacity(file_paths.len());
    for absolute_path in file_paths {
        let metadata = fs::metadata(&absolute_path)
            .with_context(|| format!("could not stat {}", absolute_path.display()))?;
        let relative_path = absolute_path
            .strip_prefix(&cache_root)
            .with_context(|| {
                format!(
                    "selected path escaped cache root: {}",
                    absolute_path.display()
                )
            })?
            .to_string_lossy()
            .replace('\\', "/");
        let logical_bytes = metadata.len();
        files.push(ScannedFile {
            allocated_bytes: allocated_file_size(&absolute_path, logical_bytes),
            absolute_path,
            relative_path,
            logical_bytes,
            payload_id: String::new(),
            sha256: None,
        });
    }
    assign_payload_identities(&mut files)?;

    Ok(CacheScan {
        cache_root,
        directory_count: directories.len() as u64,
        files,
    })
}

pub(crate) fn collect_live_cache_files(cache_root: &Path) -> Result<BTreeSet<PathBuf>> {
    let cache_root = fs::canonicalize(cache_root)
        .with_context(|| format!("could not resolve cache root {}", cache_root.display()))?;
    let mut selected = BTreeSet::new();

    let scenes = cache_root.join("scenes");
    if scenes.is_dir() {
        let mut entries = fs::read_dir(&scenes)?.collect::<std::io::Result<Vec<_>>>()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let manifest = entry.path().join("scene.ron");
            if manifest.is_file() {
                select_scene_manifest(&cache_root, &manifest, &mut selected)?;
            }
        }
    }

    // Top-level RON files are authoritative catalogs, resumable job roots,
    // or retained indexes. Parse their file references without treating the
    // whole cache root as live.
    let mut top_level = fs::read_dir(&cache_root)?.collect::<std::io::Result<Vec<_>>>()?;
    top_level.sort_by_key(|entry| entry.file_name());
    for entry in top_level {
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ron"))
        {
            select_ron_root_and_references(&cache_root, &path, &mut selected)?;
        }
    }

    let worldspaces = cache_root.join("worldspaces");
    if worldspaces.is_dir() {
        let mut entries = fs::read_dir(&worldspaces)?.collect::<std::io::Result<Vec<_>>>()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            select_worldspace_index(&cache_root, &entry.path().join("index.ron"), &mut selected)?;
        }
    }

    let mut directories = BTreeSet::new();
    let mut files = BTreeSet::new();
    for path in selected {
        walk_path(&path, &mut directories, &mut files)?;
    }
    expand_transitive_file_references(&cache_root, &mut files)?;
    Ok(files)
}

fn expand_transitive_file_references(
    cache_root: &Path,
    selected: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    let mut pending = selected.iter().cloned().collect::<Vec<_>>();
    let mut inspected = BTreeSet::new();
    while let Some(path) = pending.pop() {
        if !inspected.insert(path.clone()) {
            continue;
        }
        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default();
        let discovered = if extension.eq_ignore_ascii_case("ron") {
            let source = fs::read_to_string(&path)
                .with_context(|| format!("could not read live RON root {}", path.display()))?;
            let value: ron::Value = ron::from_str(&source)
                .with_context(|| format!("could not parse live RON root {}", path.display()))?;
            let mut references = Vec::new();
            collect_ron_strings(&value, &mut references);
            references
                .into_iter()
                .filter_map(|value| {
                    resolve_selected_file(cache_root, path.parent().unwrap_or(cache_root), value)
                })
                .collect::<Vec<_>>()
        } else if extension.eq_ignore_ascii_case("glb") {
            glb_external_references(cache_root, &path)?
        } else {
            Vec::new()
        };
        for reference in discovered {
            if selected.insert(reference.clone()) {
                pending.push(reference);
            }
        }
    }
    Ok(())
}

pub(super) fn expand_glb_external_references(
    cache_root: &Path,
    selected: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    let glbs = selected
        .iter()
        .filter(|path| {
            path.extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("glb"))
        })
        .cloned()
        .collect::<Vec<_>>();
    for glb in glbs {
        for path in glb_external_references(cache_root, &glb)? {
            selected.insert(path);
        }
    }
    Ok(())
}

fn glb_external_references(cache_root: &Path, glb: &Path) -> Result<Vec<PathBuf>> {
    let bytes =
        fs::read(glb).with_context(|| format!("could not read selected GLB {}", glb.display()))?;
    let json = glb_json(&bytes)
        .with_context(|| format!("could not inspect selected GLB {}", glb.display()))?;
    Ok(json
        .get("images")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|image| image.get("uri").and_then(serde_json::Value::as_str))
        .filter_map(|uri| {
            let canonical_uri = uri.strip_prefix('/').unwrap_or(uri);
            resolve_selected_file(cache_root, cache_root, canonical_uri)
        })
        .collect())
}

fn glb_json(bytes: &[u8]) -> Result<serde_json::Value> {
    if bytes.len() < 20 || &bytes[..4] != b"glTF" {
        bail!("invalid GLB header");
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    if version != 2 {
        bail!("unsupported GLB version {version}");
    }
    let declared = usize::try_from(u32::from_le_bytes(bytes[8..12].try_into().unwrap()))
        .context("GLB length does not fit usize")?;
    if declared > bytes.len() {
        bail!("GLB declared length exceeds file length");
    }
    let json_len = usize::try_from(u32::from_le_bytes(bytes[12..16].try_into().unwrap()))
        .context("GLB JSON length does not fit usize")?;
    let json_kind = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
    if json_kind != 0x4e4f_534a || 20usize.saturating_add(json_len) > declared {
        bail!("GLB has no valid leading JSON chunk");
    }
    serde_json::from_slice(&bytes[20..20 + json_len]).context("invalid GLB JSON")
}

fn walk_path(
    path: &Path,
    directories: &mut BTreeSet<PathBuf>,
    files: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    let metadata = fs::symlink_metadata(path)
        .with_context(|| format!("cache entry vanished during scan: {}", path.display()))?;
    if metadata.file_type().is_symlink() {
        bail!(
            "cache stats does not follow symbolic links: {}",
            path.display()
        );
    }
    if metadata.is_file() {
        files.insert(fs::canonicalize(path)?);
        return Ok(());
    }
    if !metadata.is_dir() {
        return Ok(());
    }

    let directory = fs::canonicalize(path)?;
    if !directories.insert(directory.clone()) {
        return Ok(());
    }
    let mut entries = fs::read_dir(&directory)
        .with_context(|| format!("could not read cache directory {}", directory.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        walk_path(&entry.path(), directories, files)?;
    }
    Ok(())
}

fn resolve_manifest_set(cache_root: &Path, manifest_set: &Path) -> Result<BTreeSet<PathBuf>> {
    let source = fs::read_to_string(manifest_set)
        .with_context(|| format!("could not read manifest set {}", manifest_set.display()))?;
    let value: ron::Value = ron::from_str(&source)
        .with_context(|| format!("could not parse manifest set {}", manifest_set.display()))?;
    let mut strings = Vec::new();
    collect_ron_strings(&value, &mut strings);

    let manifest_parent = manifest_set.parent().unwrap_or_else(|| Path::new("."));
    let mut selected = BTreeSet::new();
    let mut manifests = BTreeSet::new();
    for value in strings {
        if let Some(path) = resolve_selected_path(cache_root, manifest_parent, value) {
            if path.file_name().is_some_and(|name| name == "scene.ron") {
                manifests.insert(path.clone());
                if let Some(parent) = path.parent() {
                    selected.insert(parent.to_path_buf());
                }
            } else {
                selected.insert(path);
            }
        }
    }
    if selected.is_empty() {
        bail!(
            "manifest set {} did not resolve any paths below {}",
            manifest_set.display(),
            cache_root.display()
        );
    }

    for manifest in manifests {
        select_scene_manifest(cache_root, &manifest, &mut selected)?;
    }
    Ok(selected)
}

fn select_scene_manifest(
    cache_root: &Path,
    manifest: &Path,
    selected: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    if let Some(parent) = manifest.parent() {
        selected.insert(fs::canonicalize(parent)?);
    }
    let source = fs::read_to_string(manifest)
        .with_context(|| format!("could not read scene manifest {}", manifest.display()))?;
    let value: ron::Value = ron::from_str(&source)
        .with_context(|| format!("could not parse scene manifest {}", manifest.display()))?;
    select_ron_file_references(
        cache_root,
        manifest.parent().unwrap_or(cache_root),
        &value,
        selected,
    );

    // Compact exterior scene roots intentionally do not embed the cell
    // package or even repeat its path. Its location is canonical from the
    // scene's cell/worldspace identity, so include both that package and the
    // shared worldspace index.
    if let Ok(scene) = ron::from_str::<bevyout_core::manifest::PreparedSceneManifest>(&source)
        && !scene.cell.interior
        && let Some(worldspace_form_id) = scene.cell.worldspace_form_id
    {
        let package = cache_root
            .join("worldspaces")
            .join(format!("{worldspace_form_id:08x}"))
            .join("cells")
            .join(format!("{:08x}.ron", scene.cell.form_id));
        select_ron_root_and_references(cache_root, &package, selected)?;
        let index = cache_root
            .join("worldspaces")
            .join(format!("{worldspace_form_id:08x}"))
            .join("index.ron");
        select_worldspace_index(cache_root, &index, selected)?;
    }
    Ok(())
}

pub(super) fn select_worldspace_index(
    cache_root: &Path,
    path: &Path,
    selected: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    if !path.is_file() {
        return Ok(());
    }
    let canonical = fs::canonicalize(path)?;
    if !canonical.starts_with(cache_root) {
        return Ok(());
    }
    selected.insert(canonical.clone());
    let bytes = fs::read(&canonical)
        .with_context(|| format!("could not read worldspace index {}", canonical.display()))?;
    let index: bevyout_core::manifest::exterior::ExteriorWorldspaceIndex =
        ron::de::from_bytes(&bytes)
            .with_context(|| format!("could not parse worldspace index {}", canonical.display()))?;

    // `index.cells` names every prepared package in the worldspace. Those
    // are availability metadata, not dependencies of one selected cohort;
    // the selected scene packages were added above. Only include actual
    // shared asset references owned by the index.
    for value in index
        .worldspace_lod
        .iter()
        .map(|asset| asset.asset_path.as_str())
        .chain(
            index
                .persistent_references
                .iter()
                .filter_map(|reference| reference.asset_path.as_deref()),
        )
    {
        if let Some(path) = resolve_selected_file(cache_root, cache_root, value) {
            selected.insert(path);
        }
    }
    Ok(())
}

fn select_ron_root_and_references(
    cache_root: &Path,
    path: &Path,
    selected: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    if !path.is_file() {
        return Ok(());
    }
    let canonical = fs::canonicalize(path)?;
    if !canonical.starts_with(cache_root) {
        return Ok(());
    }
    selected.insert(canonical.clone());
    let source = fs::read_to_string(&canonical)
        .with_context(|| format!("could not read prepared root {}", canonical.display()))?;
    let value: ron::Value = ron::from_str(&source)
        .with_context(|| format!("could not parse prepared root {}", canonical.display()))?;
    select_ron_file_references(
        cache_root,
        canonical.parent().unwrap_or(cache_root),
        &value,
        selected,
    );
    Ok(())
}

fn select_ron_file_references(
    cache_root: &Path,
    relative_to: &Path,
    value: &ron::Value,
    selected: &mut BTreeSet<PathBuf>,
) {
    let mut references = Vec::new();
    collect_ron_strings(value, &mut references);
    for value in references {
        if let Some(path) = resolve_selected_file(cache_root, relative_to, value) {
            selected.insert(path);
        }
    }
}

fn collect_ron_strings<'a>(value: &'a ron::Value, output: &mut Vec<&'a str>) {
    match value {
        ron::Value::String(value) => output.push(value),
        ron::Value::Seq(values) => {
            for value in values {
                collect_ron_strings(value, output);
            }
        }
        ron::Value::Map(values) => {
            for (key, value) in values.iter() {
                collect_ron_strings(key, output);
                collect_ron_strings(value, output);
            }
        }
        ron::Value::Option(Some(value)) => collect_ron_strings(value, output),
        _ => {}
    }
}

fn resolve_selected_path(cache_root: &Path, relative_to: &Path, value: &str) -> Option<PathBuf> {
    let input = Path::new(value);
    let cache_parent = cache_root.parent().unwrap_or(cache_root);
    let candidates = if input.is_absolute() {
        vec![input.to_path_buf()]
    } else {
        vec![
            cache_root.join(input),
            relative_to.join(input),
            cache_parent.join(input),
            PathBuf::from(value),
            cache_root.join("scenes").join(value).join("scene.ron"),
        ]
    };
    candidates.into_iter().find_map(|candidate| {
        let canonical = fs::canonicalize(candidate).ok()?;
        canonical.starts_with(cache_root).then_some(canonical)
    })
}

fn resolve_selected_file(cache_root: &Path, relative_to: &Path, value: &str) -> Option<PathBuf> {
    resolve_selected_path(cache_root, relative_to, value).filter(|path| path.is_file())
}

fn assign_payload_identities(files: &mut [ScannedFile]) -> Result<()> {
    let mut by_size = BTreeMap::<u64, Vec<usize>>::new();
    for (index, file) in files.iter().enumerate() {
        by_size.entry(file.logical_bytes).or_default().push(index);
    }
    let candidates = by_size
        .values()
        .filter(|indexes| indexes.len() > 1)
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    let digests = candidates
        .par_iter()
        .map(|&index| hash_file(&files[index].absolute_path).map(|digest| (index, digest)))
        .collect::<Result<Vec<_>>>()?;
    for (index, digest) in digests {
        files[index].payload_id = digest.clone();
        files[index].sha256 = Some(digest);
    }
    for file in files {
        if file.payload_id.is_empty() {
            file.payload_id = format!("unique:{}", file.relative_path);
        }
    }
    Ok(())
}

fn hash_file(path: &Path) -> Result<String> {
    let file = File::open(path).with_context(|| format!("could not hash {}", path.display()))?;
    let mut reader = BufReader::with_capacity(1024 * 1024, file);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 1024 * 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let mut output = String::with_capacity(64);
    for byte in hasher.finalize() {
        write!(&mut output, "{byte:02x}").expect("writing to String cannot fail");
    }
    Ok(output)
}

#[cfg(windows)]
pub(crate) fn allocated_file_size(path: &Path, logical_bytes: u64) -> u64 {
    use std::{iter, os::windows::ffi::OsStrExt};
    use windows_sys::Win32::{
        Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_READ_ATTRIBUTES, FILE_SHARE_DELETE,
            FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_STANDARD_INFO, FileStandardInfo,
            GetFileInformationByHandleEx, OPEN_EXISTING,
        },
    };

    let wide = path
        .as_os_str()
        .encode_wide()
        .chain(iter::once(0))
        .collect::<Vec<_>>();
    // SAFETY: `wide` is a NUL-terminated UTF-16 path. All pointer arguments are
    // valid for the duration of each call, and the handle is closed exactly once.
    let handle = unsafe {
        CreateFileW(
            wide.as_ptr(),
            FILE_READ_ATTRIBUTES,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            std::ptr::null(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            std::ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        logical_bytes
    } else {
        let mut info = FILE_STANDARD_INFO::default();
        // SAFETY: `info` is correctly sized and writable, and `handle` is valid.
        let succeeded = unsafe {
            GetFileInformationByHandleEx(
                handle,
                FileStandardInfo,
                std::ptr::addr_of_mut!(info).cast(),
                std::mem::size_of::<FILE_STANDARD_INFO>() as u32,
            )
        };
        // SAFETY: `handle` was returned by `CreateFileW` above and is not used again.
        unsafe { CloseHandle(handle) };
        if succeeded != 0 && info.AllocationSize >= 0 {
            info.AllocationSize as u64
        } else {
            logical_bytes
        }
    }
}

#[cfg(unix)]
pub(crate) fn allocated_file_size(path: &Path, logical_bytes: u64) -> u64 {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path)
        .map(|metadata| metadata.blocks().saturating_mul(512))
        .unwrap_or(logical_bytes)
}

#[cfg(not(any(windows, unix)))]
pub(crate) fn allocated_file_size(_path: &Path, logical_bytes: u64) -> u64 {
    logical_bytes
}
