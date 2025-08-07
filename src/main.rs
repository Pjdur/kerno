mod util;
use home::home_dir;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use toml::Value;
use util::scan_binaries;

static VERSION: &str = "1.0.0";
static AUTHOR: &str = "Pjdur";

fn get_kerno_path() -> Option<PathBuf> {
    home_dir().map(|h| h.join("kerno.toml"))
}

fn load_env_vars() -> HashMap<String, String> {
    let mut env_vars = env::vars().collect::<HashMap<String, String>>();

    if let Some(path) = get_kerno_path()
        && path.exists()
        && let Ok(contents) = fs::read_to_string(&path)
        && let Ok(value) = contents.parse::<Value>()
        && let Some(table) = value.as_table()
    {
        for (k, v) in table {
            if let Some(s) = v.as_str() {
                env_vars.insert(k.clone(), s.to_string());
            }
        }
    }

    env_vars
}

fn write_env_vars(env_vars: &HashMap<String, String>) {
    if let Some(path) = get_kerno_path()
        && let Ok(f) = File::create(&path)
    {
        let mut writer = BufWriter::new(f);
        for (k, v) in env_vars {
            let safe_value = v.replace('"', "\\\"");
            writeln!(writer, "{k} = \"{safe_value}\"").unwrap_or_else(|e| {
                eprintln!("Failed to write to kerno.toml: {e}");
            });
        }
    }
}

fn main() {
    if let Some(home) = home_dir()
        && let Err(e) = env::set_current_dir(&home)
    {
        eprintln!("Failed to set default directory: {e}");
    }

    let mut env_vars = load_env_vars();
    let mut history: Vec<String> = Vec::new();
    println!("\x1b[1;36mWelcome to Kerno v{VERSION} 🌟\x1b[0m");

    loop {
        let binary_cache = scan_binaries();
        let cwd = env::current_dir().unwrap_or_default();
        print!("\x1b[1;32m[kerno] \x1b[0;93m{}>\x1b[0m ", cwd.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        history.push(trimmed.to_string());

        if trimmed == "exit" {
            write_env_vars(&env_vars);
            break;
        }

        execute_command(trimmed, &binary_cache, &mut env_vars, &mut history);
    }
}

fn execute_command(
    input: &str,
    binary_cache: &HashMap<String, PathBuf>,
    env_vars: &mut HashMap<String, String>,
    history: &mut [String],
) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "echo" => println!("{}", parts[1..].join(" ")),
        "scanpath" => {
            if let Some(paths) = env::var_os("PATH") {
                let paths = env::split_paths(&paths);
                let mut executables = Vec::new();

                for path in paths {
                    if let Ok(entries) = fs::read_dir(path) {
                        for entry in entries.flatten() {
                            let file_path = entry.path();
                            if file_path.is_file() {
                                let file_name =
                                    file_path.file_name().and_then(|f| f.to_str()).unwrap_or("");

                                #[cfg(unix)]
                                let is_exec = {
                                    use std::os::unix::fs::PermissionsExt;
                                    entry
                                        .metadata()
                                        .map(|m| m.permissions().mode() & 0o111 != 0)
                                        .unwrap_or(false)
                                };

                                #[cfg(windows)]
                                let is_exec = {
                                    let ext = file_path
                                        .extension()
                                        .and_then(|e| e.to_str())
                                        .unwrap_or("")
                                        .to_lowercase();
                                    matches!(ext.as_str(), "exe" | "bat" | "cmd")
                                };

                                if is_exec {
                                    executables.push(file_name.to_string());
                                }
                            }
                        }
                    }
                }

                executables.sort();
                executables.dedup();
                for exe in executables {
                    println!("{exe}");
                }
            } else {
                eprintln!("No PATH variable found.");
            }
        }
        "set" if parts.len() >= 3 => {
            env_vars.insert(parts[1].to_string(), parts[2..].join(" "));
        }
        "get" if parts.len() == 2 => match env_vars.get(parts[1]) {
            Some(val) => println!("{val}"),
            None => println!("Variable not found"),
        },
        "unset" if parts.len() == 2 => {
            env_vars.remove(parts[1]);
        }
        "env" => {
            for (k, v) in env_vars.iter() {
                println!("{k}={v}");
            }
        }
        "cd" if parts.len() == 2 => {
            if let Err(e) = env::set_current_dir(parts[1]) {
                eprintln!("cd failed: {e}");
            }
        }
        "pwd" => match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => eprintln!("Error: {e}"),
        },
        "version" => println!("kerno v{VERSION}"),
        "shellinfo" | "about" => {
            println!("kerno v{VERSION}");
            println!("Author: {AUTHOR}");
            println!("Written in Rust 🦀");
            println!("Cross-platform, lightweight and lightning fast.");
        }
        "ls" => match fs::read_dir(env::current_dir().unwrap_or_default()) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    println!("{}", entry.path().display());
                }
            }
            Err(e) => eprintln!("ls failed: {e}"),
        },
        "cat" if parts.len() == 2 => match File::open(parts[1]) {
            Ok(f) => {
                for line in BufReader::new(f).lines().map_while(Result::ok) {
                    println!("{line}");
                }
            }
            Err(e) => eprintln!("Failed to open file: {e}"),
        },
        "touch" if parts.len() == 2 => {
            if let Err(e) = File::create(parts[1]) {
                eprintln!("touch failed: {e}");
            }
        }
        "rm" | "del" if parts.len() == 2 => {
            if let Err(e) = fs::remove_file(parts[1]) {
                eprintln!("File delete failed: {e}");
            }
        }
        "mkdir" if parts.len() == 2 => {
            if let Err(e) = fs::create_dir(parts[1]) {
                eprintln!("mkdir failed: {e}");
            }
        }
        "rmdir" if parts.len() == 2 => {
            if let Err(e) = fs::remove_dir(parts[1]) {
                eprintln!("rmdir failed: {e}");
            }
        }
        "date" => {
            let now = chrono::Local::now();
            println!("{now}");
        }
        "clear" | "cls" => {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
        "write" if parts.len() >= 3 => {
            let content = parts[2..].join(" ");
            if let Ok(mut f) = File::create(parts[1]) {
                if let Err(e) = writeln!(f, "{content}") {
                    eprintln!("write failed: {e}");
                }
            } else {
                eprintln!("Failed to open file for writing");
            }
        }
        "read" if parts.len() == 2 => match File::open(parts[1]) {
            Ok(f) => {
                for line in BufReader::new(f).lines().map_while(Result::ok) {
                    println!("{line}");
                }
            }
            Err(e) => eprintln!("Failed to read file: {e}"),
        },
        "exit" => std::process::exit(0),
        "help" => {
            println!("Available commands:");
            println!(
                "echo, scanpath, set, get, unset, env, cd, pwd, ls, cat, touch, rm, mkdir, rmdir, date, clear, write, read, exit, help, history"
            );
        }
        "history" => {
            for (i, cmd) in history.iter().enumerate() {
                println!("{}: {}", i + 1, cmd);
            }
        }
        _ => {
            let cmd = parts[0];
            let args = &parts[1..];

            if let Some(path) = binary_cache.get(cmd) {
                if let Err(e) = Command::new(path)
                    .args(args)
                    .spawn()
                    .and_then(|mut child| child.wait())
                {
                    eprintln!("Error: {e}");
                }
            } else {
                eprintln!("Unknown command: {cmd}");
            }
        }
    }
}
