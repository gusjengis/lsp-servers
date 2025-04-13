// Check if an LSP server is in the PATH
// Ultimately need to find a better way to check if something is installed, but this is a good start.
pub(crate) fn check_server_installation(server_name: &str) -> bool {
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
