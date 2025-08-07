use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Scans the system PATH for executable files and returns a map of name → path.
pub fn scan_binaries() -> HashMap<String, PathBuf> {
    let mut cache = HashMap::new();

    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let file_path = entry.path();
                    if file_path.is_file() && is_executable(&file_path) {
                        let name = normalize_name(&file_path);
                        // Avoid overwriting existing entries unless preferred
                        cache.entry(name).or_insert(file_path);
                    }
                }
            }
        }
    }

    cache
}

/// Determines if a file is executable on the current platform.
fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(path) {
            let mode = metadata.permissions().mode();
            return mode & 0o111 != 0;
        }
        false
    }

    #[cfg(windows)]
    {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            matches!(
                ext.to_lowercase().as_str(),
                "exe" | "bat" | "cmd" | "com" | "ps1"
            )
        } else {
            false
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}

/// Normalizes the executable name for consistent cross-platform behavior.
fn normalize_name(path: &Path) -> String {
    #[cfg(windows)]
    {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase()
    }

    #[cfg(unix)]
    {
        path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string()
    }

    #[cfg(not(any(unix, windows)))]
    {
        path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string()
    }
}
