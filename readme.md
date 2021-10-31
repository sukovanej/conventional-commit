# git convetional-commit

A git subcommand for easy convetional commits.


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
