//! Plugin loading and content-set fingerprints.

use super::*;

pub(crate) fn load_plugin_chain(selected: &Path, data_root: &Path) -> Result<Vec<LoadedPlugin>> {
    fn visit(
        path: &Path,
        data_root: &Path,
        loaded: &mut Vec<LoadedPlugin>,
        loaded_names: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
    ) -> Result<()> {
        let path = fs::canonicalize(path)
            .with_context(|| format!("plugin dependency does not exist: {}", path.display()))?;
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .context("plugin path has no UTF-8 filename")?
            .to_string();
        let key = name.to_ascii_lowercase();
        if loaded_names.contains(&key) {
            return Ok(());
        }
        if !visiting.insert(key.clone()) {
            bail!("cyclic Fallout plugin dependency involving {name}")
        }
        let bytes = fs::read(&path)
            .with_context(|| format!("failed to read plugin dependency {}", path.display()))?;
        for master in read_master_names(&bytes)? {
            visit(
                &data_root.join(master),
                data_root,
                loaded,
                loaded_names,
                visiting,
            )?;
        }
        visiting.remove(&key);
        loaded_names.insert(key);
        loaded.push(LoadedPlugin { name, path, bytes });
        Ok(())
    }

    let mut loaded = Vec::new();
    visit(
        selected,
        data_root,
        &mut loaded,
        &mut HashSet::new(),
        &mut HashSet::new(),
    )?;
    Ok(loaded)
}

pub(crate) fn content_set_fingerprint(plugins: &[LoadedPlugin]) -> String {
    let mut hasher = Sha256::new();
    for plugin in plugins {
        hasher.update(plugin.name.to_ascii_lowercase().as_bytes());
        hasher.update([0]);
        hasher.update((plugin.bytes.len() as u64).to_le_bytes());
        hasher.update(&plugin.bytes);
    }
    format!("{:x}", hasher.finalize())
}
