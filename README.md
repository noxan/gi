# gi

An oppinionated Git(hub) productivity command line interface (CLI).

## Usage

```
gi - list all issues
gi <issue_number> - create branch to work on issue <issue_number>
```

## Configuration

Create a configuration file at `$HOME/.config/gi.toml` with the following content.

```toml
[github]
token = "<YOUR_GITHUB_ACCESS_TOKEN>"
```

You can generate Github personal access tokens at https://github.com/settings/tokens.
