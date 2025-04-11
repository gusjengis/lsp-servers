mod ext_to_lsp_servers;
mod settings;

use std::{collections::HashSet, path::PathBuf};

use ext_to_lsp_servers::ext_to_lsp_servers;
use settings::Settings;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut settings = Settings::defaults();
    let mut target_path = None;

    if let Some(cwd) = match std::env::current_dir() {
        Ok(path) => Some(path),
        Err(e) => {
            println!("Failed to get current directory: {}", e);
            None
        }
    } {
        // Process all arguments
        for arg in &args[1..] {
            if arg.starts_with("-") {
                // This is a flag
                match arg.as_str() {
                    "-i" => settings.installed_only = true,
                    // Add more flag handling here as needed
                    _ => println!("Unknown flag: {}", arg),
                }
            } else {
                // This is likely a path
                match PathBuf::from(&arg).canonicalize() {
                    Ok(absolute_path) => {
                        target_path = Some(absolute_path);
                    }
                    Err(e) => {
                        println!("Failed to resolve path '{}': {}", arg, e);
                    }
                }
            }
        }

        // Use the specified path or default to current directory
        let path_to_scan = target_path.unwrap_or(cwd);
        scan_workspace(path_to_scan, &settings);
    }
}

// Check the extension of all files in the provided folder using ext_to_lsp_servers
// Build a set of relevant LSP servers
// Check if each of those servers are installed
//
fn scan_workspace(workspace: PathBuf, settings: &Settings) {
    // println!("Scanning {}...", workspace.to_str().unwrap()); // for debug
    let mut relevant_lsp_servers = HashSet::<&'static str>::new();
    for entry in WalkDir::new(&workspace).into_iter() {
        match entry {
            Ok(file) => {
                if file.file_type().is_file() {
                    if let Some(_ext) = file.path().extension() {
                        if let Some(ext) = _ext.to_str() {
                            let servers = ext_to_lsp_servers(ext);
                            for server in servers {
                                relevant_lsp_servers.insert(server);
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!(
                "Error while walking directory: {}\n{}",
                workspace.to_str().unwrap(),
                e
            ),
        }
    }
    for relevant_lsp_server in relevant_lsp_servers {
        if !settings.installed_only || check_server_installation(relevant_lsp_server) {
            print!("{}", relevant_lsp_server);
            print!("\n");
        }
    }
}

// Check if an LSP server is in the PATH
// Ultimately need to find a better way to check if something is installed, but this is a good start.
fn check_server_installation(server_name: &str) -> bool {
    // Determine the command to use based on the OS
    let unix_cmd_str = &format!("which {} > /dev/null 2>&1", server_name); // had to init this here for lifetime reasons
    let (command, args) = match std::env::consts::OS {
        "windows" => {
            // On Windows, use 'where' command
            ("where", vec![server_name])
        }
        _ => {
            // On Unix-like systems (Linux, macOS), use 'which' command with silent output
            ("sh", vec!["-c", unix_cmd_str])
        }
    };

    let status = std::process::Command::new(command).args(args).status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                // print!("  ✓ Already installed");
                true
            } else {
                // print!("  ✗ Not installed");
                false
            }
        }
        Err(e) => {
            print!("  ? Could not determine installation status\n{}", e);
            false
        }
    }
}
