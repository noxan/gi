# gi

An oppinionated Git(hub) workflow tool.

## Usage

```
Usage: gi [<issue>] [<command>] [<args>]

An oppinionated Git(hub) workflow tool.

Positional Arguments:
  issue

Options:
  --help            display usage information

Commands:
  pr                Pull request command.
```

### Examples

1. List issues with `gi`
    ```
    ❯ gi
    #10 Find better base of new branches
    #8 Exclude pull requests in issue list
    #7 Add color formatting to output
    #6 Prevent losing local changes when checking out a branch
    ```
2. Checkout an issue with `gi <issue>`
    ```
    ❯ gi 10
    Switched to branch '10-find-better-base-of-new-branches'
    ```
3. Create a new pull request with `gi pr`
    ```
    ❯ gi pr
    Open new pull request with https://github.com/noxan/gi/compare/10-find-better-base-of-new-branches?...
    ```

## Configuration

Create a configuration file at `$HOME/.config/gi.toml` with the following content.

```toml
[github]
token = "<YOUR_GITHUB_ACCESS_TOKEN>"
```

You can generate Github personal access tokens at https://github.com/settings/tokens.
