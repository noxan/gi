# gi

An oppinionated Git(hub) productivity command line interface (CLI).

## Usage

```
Usage: gi [<issue>]

An oppinionated Git(hub) workflow tool.

Positional Arguments:
  issue

Options:
  --help            display usage information
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

## Configuration

Create a configuration file at `$HOME/.config/gi.toml` with the following content.

```toml
[github]
token = "<YOUR_GITHUB_ACCESS_TOKEN>"
```

You can generate Github personal access tokens at https://github.com/settings/tokens.
