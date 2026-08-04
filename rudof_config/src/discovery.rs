use crate::ConfigError;
use std::path::{Path, PathBuf};

/// Recursively deep merges `overlay` into `base`
pub fn merge_tables(base: &mut toml::Table, overlay: toml::Table) {
    for (key, overlay_value) in overlay {
        match (base.get_mut(&key), overlay_value) {
            (Some(toml::Value::Table(base_table)), toml::Value::Table(overlay_table)) => {
                merge_tables(base_table, overlay_table);
            }
            (_, overlay_value) => {
                base.insert(key, overlay_value);
            }
        }
    }
}

/// Reads a TOML file into a [`toml::Table`]
///
/// # Errors
///
/// Returns [`ConfigError::Read`] if the file cannot be read, or
/// [`ConfigError::Parse`] if its contents are not a valid TOML table
pub fn read_toml_table<P: AsRef<Path>>(path: P) -> Result<toml::Table, ConfigError> {
    let path = path.as_ref();
    let location = path.display().to_string();
    let contents = std::fs::read_to_string(path).map_err(|e| ConfigError::Read {
        location: location.clone(),
        error: e.to_string(),
    })?;
    toml::from_str::<toml::Table>(&contents).map_err(|e| ConfigError::Parse {
        location,
        error: e.to_string(),
    })
}

/// Collects every `file_name` from `start` up to the filesystem root
///
/// The result is returned root-first, so that the file closest to `start` appears
/// last
pub fn find_config_files_from(start: &Path, file_name: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dir = start.to_path_buf();
    loop {
        let candidate = dir.join(file_name);
        if candidate.is_file() {
            files.push(candidate);
        }
        if !dir.pop() {
            break;
        }
    }
    files.reverse();
    files
}

/// Returns the platform-specific user config file for an application, if the
/// user config directory can be determined
///
/// The path is `<config_dir>/<app_name>/<file_name>`, where `<config_dir>` is:
/// - Linux: `~/.config`
/// - Windows: `%LOCALAPPDATA%`
/// - macOS: `~/Library/Application Support`
///
/// The returned path is not guaranteed to exist; callers should check.
pub fn user_config_file(app_name: &str, file_name: &str) -> Option<PathBuf> {
    dirs::config_local_dir().map(|dir| dir.join(app_name).join(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_overrides_scalars_and_merges_subtables() {
        let mut base: toml::Table = toml::from_str("a = 1\n[t]\nx = 1\ny = 1\n").unwrap();
        let overlay: toml::Table = toml::from_str("a = 2\n[t]\ny = 9\n").unwrap();
        merge_tables(&mut base, overlay);
        assert_eq!(base["a"].as_integer(), Some(2));
        let t = base["t"].as_table().unwrap();
        assert_eq!(t["x"].as_integer(), Some(1));
        assert_eq!(t["y"].as_integer(), Some(9));
    }

    #[test]
    fn find_files_returns_root_first() {
        let tmp = std::env::temp_dir().join(format!("rudof_cfg_disc_{}", std::process::id()));
        let nested = tmp.join("a").join("b");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(tmp.join("rudof.toml"), "x = 1\n").unwrap();
        std::fs::write(nested.join("rudof.toml"), "x = 2\n").unwrap();

        let files = find_config_files_from(&nested, "rudof.toml");
        assert_eq!(files.first().unwrap(), &tmp.join("rudof.toml"));
        assert_eq!(files.last().unwrap(), &nested.join("rudof.toml"));

        std::fs::remove_dir_all(&tmp).ok();
    }
}
