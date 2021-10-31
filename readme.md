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

## Examples

```bash
# commit a fix with a message
$ git conventional-commit -am "replace everything with rust" -t fix
fix: replace everything with rust

# commit a fix with a message and the scope
$ git conventional-commit -am "hello world" -t feat -s "my-package"
feat(my-package): hello world

# commit a fix (displayed as a github emoji) with a message and the scope
$ git conventional-commit -aem "hello world" -t feat -s "my-package"
:sparkles: my-package: hello world

# commit a fix (displayed as a github emoji) with a message and the scope and include the issue number
$ git conventional-commit -aem "hello world" -t feat -s "my-package" -i "#12"
:sparkles: my-package: hello world (#12)
```

## Help

```
git-conventional-commit 0.1

Milan Suk <Milansuk@email.cz>

USAGE:
    git-conventional-commit [OPTIONS] --message <MESSAGE> --commit-type <COMMIT_TYPE>

OPTIONS:
    -a, --add-all-files                
    -d, --dry-run                      
    -e, --emoji                        
    -h, --help                         Print help information
    -i, --issue <ISSUE>                
    -m, --message <MESSAGE>            
    -s, --scope <SCOPE>                
    -t, --commit-type <COMMIT_TYPE>    [possible values: build, chore, ci, docs, feat, fix, perf,
                                       refactor, revert, style, test]
    -V, --version                      Print version information
```
