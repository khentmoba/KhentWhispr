use std::fs;
use std::path::{Path, PathBuf};

pub const ENGINE_ARCHIVE_CANDIDATES: &[&str] = &["resources/engine.tar.xz", "resources/engine.zip"];

fn is_non_empty_file(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.len() > 0)
        .unwrap_or(false)
}

pub fn select_engine_archive(
    manifest_dir: &Path,
    allow_placeholder: bool,
) -> Result<PathBuf, String> {
    let candidates: Vec<PathBuf> = ENGINE_ARCHIVE_CANDIDATES
        .iter()
        .map(|relative| manifest_dir.join(relative))
        .collect();

    if allow_placeholder {
        if let Some(archive) = candidates
            .iter()
            .find(|candidate| is_non_empty_file(candidate))
        {
            return Ok(archive.clone());
        }

        let placeholder = &candidates[0];
        if !placeholder.exists() {
            fs::File::create(placeholder).map_err(|error| {
                format!(
                    "Failed to create engine placeholder file for development {}: {error}",
                    placeholder.display()
                )
            })?;
        }
        return Ok(placeholder.clone());
    }

    let release_archive = &candidates[0];
    if is_non_empty_file(release_archive) {
        return Ok(release_archive.clone());
    }

    Err(format!(
        "A non-empty {} is required for release builds. Run `uv run --locked python scripts/build_engine.py` first, or reuse a previously verified archive.",
        release_archive.display()
    ))
}
