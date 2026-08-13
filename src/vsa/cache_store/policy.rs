use sha2::{Digest, Sha256};
use std::fmt;

const RECIPE_DOMAIN: &[u8] = b"bevyout-prepared-recipe-v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CacheStorePolicyError(String);

impl fmt::Display for CacheStorePolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CacheStorePolicyError {}

fn invalid(message: impl Into<String>) -> CacheStorePolicyError {
    CacheStorePolicyError(message.into())
}

/// Canonicalizes a Fallout source identity, not a payload-object path.
pub(crate) fn normalize_source_path(path: &str) -> Result<String, CacheStorePolicyError> {
    if path.trim().is_empty() {
        return Err(invalid("source path is empty"));
    }
    let path = path.replace('\\', "/");
    if path.starts_with('/') || path.starts_with("//") {
        return Err(invalid("source path must be relative"));
    }
    let mut parts = Vec::new();
    for part in path.split('/') {
        if part.is_empty() {
            continue;
        }
        if part == "." || part == ".." {
            return Err(invalid("source path contains traversal"));
        }
        if part.contains(':') {
            return Err(invalid("source path contains a drive or URI prefix"));
        }
        parts.push(part.to_ascii_lowercase());
    }
    if parts
        .first()
        .is_some_and(|component| component.eq_ignore_ascii_case("data"))
    {
        parts.remove(0);
    }
    if parts.is_empty() {
        return Err(invalid("source path has no asset components"));
    }
    Ok(parts.join("/"))
}

fn append_field(hasher: &mut Sha256, bytes: &[u8]) -> Result<(), CacheStorePolicyError> {
    let length = u64::try_from(bytes.len()).map_err(|_| invalid("recipe field is too large"))?;
    hasher.update(length.to_le_bytes());
    hasher.update(bytes);
    Ok(())
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Hashes unambiguous, length-prefixed recipe fields under a versioned domain.
#[allow(clippy::too_many_arguments)]
pub(crate) fn recipe_identity(
    kind: &str,
    recipe_version: u32,
    source_identity: &str,
    input_hashes: &[String],
    converter_revision: &str,
    format_policy_revision: &str,
    canonical_settings: &[u8],
) -> Result<String, CacheStorePolicyError> {
    if kind.is_empty()
        || !kind
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
    {
        return Err(invalid("object kind is not canonical"));
    }
    let normalized_source = normalize_source_path(source_identity)?;
    if normalized_source != source_identity {
        return Err(invalid("source identity is not normalized"));
    }
    if input_hashes.iter().any(|hash| !valid_sha256(hash)) {
        return Err(invalid("recipe input hash is not lowercase SHA-256"));
    }
    if converter_revision.is_empty() || format_policy_revision.is_empty() {
        return Err(invalid("recipe revisions must not be empty"));
    }

    let mut hasher = Sha256::new();
    append_field(&mut hasher, RECIPE_DOMAIN)?;
    append_field(&mut hasher, kind.as_bytes())?;
    hasher.update(recipe_version.to_le_bytes());
    append_field(&mut hasher, source_identity.as_bytes())?;
    let input_count = u64::try_from(input_hashes.len())
        .map_err(|_| invalid("recipe has too many input hashes"))?;
    hasher.update(input_count.to_le_bytes());
    for input_hash in input_hashes {
        append_field(&mut hasher, input_hash.as_bytes())?;
    }
    append_field(&mut hasher, converter_revision.as_bytes())?;
    append_field(&mut hasher, format_policy_revision.as_bytes())?;
    append_field(&mut hasher, canonical_settings)?;
    Ok(format!("{:x}", hasher.finalize()))
}

pub(super) fn is_canonical_sha256(value: &str) -> bool {
    valid_sha256(value)
}
