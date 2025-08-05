use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn scan_binaries() -> HashMap<String, PathBuf> {
    let mut cache = HashMap::new();

    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        let name = file_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_string();

                        if is_executable(&file_path) {
                            cache.insert(name, file_path);
                        }
                    }
                }
            }
        }
    }

    cache
}

// Cross-platform check: is the file executable?
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
            matches!(ext.to_lowercase().as_str(), "exe" | "bat" | "cmd")
        } else {
            false
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        false // For unknown platforms, default to not executable
    }
}
