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

## Configuration

Create a configuration file at `$HOME/.config/gi.toml` with the following content.

```toml
[github]
token = "<YOUR_GITHUB_ACCESS_TOKEN>"
```

You can generate Github personal access tokens at https://github.com/settings/tokens.
