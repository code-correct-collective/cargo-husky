# C3 Cargo Husky

This project is an implementation of husky that relies on git hooks with a task runner. This approach
is an attempt to create a cross platfrom consistent implementation to run commands when a git hook is invoked.

Cargo and rust are not required for this implementation. You can use this tool with any git repository and any programming language.
Simply have the `cargo-husky` (mac/linux) or `cargo-husky.exe` (windows) executable on your path and call `cargo-husky install`. 
This will configure the git hooks that leverage the task runner. 


