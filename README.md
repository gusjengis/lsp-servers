# LSP Servers
This is a simple utility library with only one function. 

Given a directory, it returns a list of relevant language servers. Relevant meaning that they support file types that were found in the directory.

With the corresponding setting enabled only installed language servers will be returned.

The purpose of this is just to check which servers need to be spun up when creating an LSP client that operates on a specific directory.

I'm creating this for personal use in [this](https://github.com/gusjengis/mermaid-class-diagrams) project that generates mermaid diagrams of codebases. If anyone else finds this useful and wants to improve it I would welcome PRs.
