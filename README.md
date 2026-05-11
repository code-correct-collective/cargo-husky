# C3 Cargo Husky

This project is an implementation of husky that relies on git hooks with a task runner. This approach
is an attempt to create a cross platfrom consistent implementation to run commands when a git hook is invoked.

Cargo and rust are not required for this implementation. You can use this tool with any git repository and any programming language.
Simply have the `cargo-husky` (mac/linux) or `cargo-husky.exe` (windows) executable on your path and call `cargo-husky install`. 
This will configure the git hooks that leverage the task runner. 

Heavy inspiration for this rust port is taken from the great tool [Husky.Net](https://github.com/alirezanet/husky.net)

## Installation

```shell
cargo install c3-cargo-husky --locked
```

## Example/Tutorial

### Install/enable the hooks in your git repository
We need to enable the git hooks and husky configuration in your git hooks:

```shell
cargo husky install
```

### Add a task to the task runner configuration file

In your editor open `.husky/task-runner.json` and add the following task:

```json
{
    {
        "name": "cargo-check",
        "group": "pre-commit",
        "command": "cargo",
        "args": [ "check" ]
    },
    {
        "name": "cargo-clippy",
        "group": "pre-commit",
        "command": "cargo",
        "args": [ "clippy", "--", "-Dwarnings" ]
    },
    {
        "name": "cargo-fmt-verify",
        "group": "pre-commit",
        "command": "cargo",
        "args": [ "fmt", "--all", "--check" ]
    }
}
```

### Verify the tasks are configured correctly

Run the following command to list the tasks you have defined:

```shell
cargo husky list
```

You should see the list of tasks as defined above. You can run all of them with the following command:

```shell
cargo husky run
```

### Create your first hooks and command

You can run this command multiple times to append multiple commands to an existing git hook:

```shell
cargo husky set pre-commit -c "cargo husky run -g pre-commit"
```
### Add the `.husky` files to your git repository and commit your changes

Add and commit the `.husky` configuration files to your repository:

```shell
git add .
git commit -m "chore: add cargo husky configuraiton files"
```

Notice the git `pre-commit` hook is executed before the commit is applied to your repository.

### Enabling the checks for future developers:

Future developers will need to have the `husky` subcommand installed. Then they need to run the following command 
to enable the git hooks in their local repository:

```shell
cargo husky install
```

