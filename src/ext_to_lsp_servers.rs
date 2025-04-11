// most of these are AI generated, people who actually know, please expand and request this list as
// required, I only care about rust-analyzer at the moment
// Also, try to maintain alphabetical sorting by extentsion

pub fn ext_to_lsp_servers(ext: &str) -> &'static [&'static str] {
    match ext {
        "bash" | "sh" => &["bash-language-server"],
        "c" | "cpp" | "h" | "hpp" => &["clangd"],
        "cs" => &["omnisharp"],
        "css" | "html" | "less" | "scss" => {
            &["vscode-html-language-server", "vscode-css-language-server"]
        }
        "dart" => &["dart-language-server"],
        "dockerfile" => &["dockerfile-language-server"],
        "elm" => &["elm-language-server"],
        "ex" | "exs" => &["elixir-ls"],
        "fs" | "fsi" | "fsx" => &["fsharp-language-server"],
        "go" => &["gopls"],
        "haskell" | "hs" => &["haskell-language-server"],
        "java" => &["jdtls"],
        "js" | "jsx" | "ts" | "tsx" => &["typescript-language-server", "eslint-language-server"],
        "json" => &["vscode-json-language-server"],
        "kt" | "kts" => &["kotlin-language-server"],
        "lua" => &["lua-language-server"],
        "md" => &["marksman"],
        "php" => &["intelephense"],
        "py" => &["pyright", "python-language-server"],
        "r" => &["r-language-server"],
        "rb" => &["solargraph"],
        "rs" => &["rust-analyzer"],
        "svelte" => &["svelte-language-server"],
        "swift" => &["sourcekit-lsp"],
        "tf" | "tfvars" => &["terraform-ls"],
        "vue" => &["volar"],
        "wgsl" => &["wgsl-analyzer"],
        "yaml" | "yml" => &["yaml-language-server"],
        "zig" => &["zls"],
        _ => &[],
    }
}
