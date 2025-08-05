use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt; // or os::windows::fs

pub fn scan_binaries() -> HashMap<String, PathBuf> {
    let mut cache = HashMap::new();

    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let file_path = entry.path();
                    let name = file_path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();

                    #[cfg(unix)]
                    let is_exec = entry.metadata()
                        .map(|m| m.permissions().mode() & 0o111 != 0)
                        .unwrap_or(false);

                    #[cfg(windows)]
                    let is_exec = {
                        let ext = file_path.extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();
                        matches!(ext.as_str(), "exe" | "bat" | "cmd")
                    };

                    if file_path.is_file() && is_exec {
                        cache.insert(name, file_path);
                    }
                }
            }
        }
    }

    cache
}
