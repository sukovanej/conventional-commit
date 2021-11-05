# git convetional-commit

A git subcommand for easy convetional commits.

## Installation

 - `git clone` this repository
 - use `make install` to build and install the binary
 - add the `.cargo/bin` path to your `PATH` env variable

### Setup git alias

For quick access to the command use the command bellow to setup alias for the git command.

```bash
git config --global alias.cc conventional-commit
```

Now you can call the command using `git cc <args>`.

## Configuration

Put **convetional_commit.toml** into the project root directory.

```toml
emoji = true
```

This configuration makes the command use emoji by default.

## Examples

```bash
# commit a fix with a message
$ git cc fix -am "replace everything with rust"
fix: replace everything with rust

# commit a fix with a message and the scope
$ git cc feat -am "hello world" -s "my-package"
feat(my-package): hello world

# commit a fix (displayed as a github emoji) with a message and the scope
$ git cc feat -aem "hello world" -s "my-package"
:sparkles: my-package: hello world

# commit a fix (displayed as a github emoji) with a message and the scope and include the issue number
$ git cc feat -aem "hello world" -s "my-package" -i "#12"
:sparkles: my-package: hello world (#12)
```

## Help

```
git-conventional-commit 0.1

Milan Suk <Milansuk@email.cz>

USAGE:
    git-conventional-commit [OPTIONS] --message <MESSAGE> <COMMIT_TYPE>

ARGS:
    <COMMIT_TYPE>    [possible values: build, chore, ci, docs, feat, fix, perf, refactor,
                     revert, style, test]

OPTIONS:
    -a, --add-all-files        -a option for the git commit command
    -d, --dry-run              Don't do the commit, only show the commit message
    -e, --emoji                If turned on, an emoji will be used instead of name of the commit
                               type
    -h, --help                 Print help information
    -i, --issue <ISSUE>        Identifier of the github issue / jira issue / ...
    -m, --message <MESSAGE>    Commit message
    -s, --scope <SCOPE>        Optional information specifying the section of the codebase
    -V, --version              Print version information
```
