use crate::{
    check_install::check_server_installation, ext_to_lsp_servers::lsp_servers_from_ext,
    settings::Settings,
};
use std::{collections::HashSet, path::PathBuf};
use walkdir::WalkDir;

// Check the extension of all files in the provided folder using ext_to_lsp_servers
// Build a set of relevant LSP servers
// Check if each of those servers are installed
//
pub fn relevant_lsp_servers(workspace: PathBuf, settings: &Settings) -> Vec<&'static str> {
    let mut res = vec![];
    // println!("Scanning {}...", workspace.to_str().unwrap()); // for debug
    let mut relevant_lsp_servers = HashSet::<&'static str>::new();
    for entry in WalkDir::new(&workspace).into_iter() {
        match entry {
            Ok(file) => {
                if file.file_type().is_file() {
                    if let Some(_ext) = file.path().extension() {
                        if let Some(ext) = _ext.to_str() {
                            let servers = lsp_servers_from_ext(ext);
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
        if settings.installed_only {
            if check_server_installation(relevant_lsp_server) {
                res.push(relevant_lsp_server);
            }
        } else {
            res.push(relevant_lsp_server);
        }
    }

    res
}
