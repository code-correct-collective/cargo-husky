# C3 Cargo Husky

This project is an implementation of husky that relies on git hooks with a task runner. This approach
is an attempt to create a cross platfrom consistent implementation to run commands when a git hook is invoked.

Cargo and rust are not required for this implementation. You can use this tool with any git repository and any programming language.
Simply have the `cargo-husky` (mac/linux) or `cargo-husky.exe` (windows) executable on your path and call `cargo-husky install`. 
This will configure the git hooks that leverage the task runner. 

Heavy inspiration for this rust port is taken from the great tool [Husky.Net](https://github.com/alirezanet/husky.net)

## Installation

Eventually just run (not published yet)

```shell
cargo install c3-cargo-husky --locked
```

## Usage

Once `cargo-husky` is installed on your path, install the git hooks:

```shell
cargo husky install
```

See the list of your tasks:
```shell
cargo husky list
```

Run all the defined hooks:
```shell
cargo husky run
```

Install your first hook with the following command

```shell
cargo husky set pre-commit -c "cargo husky run -n welcome-message-example"
```

## TODOs:

- [x] Initial installation of husky settings
- [x] Uninstallation of the hooks
- [x] Create default hook
- [x] Create task runner
- [x] Run all tasks
- [x] Run tasks by group name
- [x] Run task by name
- [ ] Task `cwd` support.
- [ ] Task Runner Variables Support
- [ ] Task Output Verbosity
- [ ] Task Branch Filtering
- [ ] Task Include/Exclude Regex
- [ ] Task Filtering rules.
- [ ] Colorize output





