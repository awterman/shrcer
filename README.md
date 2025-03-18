# zshrcer

A Rust tool to generate shell configuration files (`.zshrc` for Zsh and `config.fish` for Fish) from a single TOML configuration.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Generate .zshrc and print to stdout (for preview)
zshrcer

# Generate config.fish for Fish shell
zshrcer -s fish

# Redirect output to a file
zshrcer > ~/.zshrc
zshrcer -s fish > ~/.config/fish/config.fish

# Specify a custom config file
zshrcer -c /path/to/config.toml

# Specify a custom output file
zshrcer -o /path/to/output/.zshrc
zshrcer -s fish -o /path/to/output/config.fish

# Specify both custom config and output
zshrcer -c /path/to/config.toml -o /path/to/output/.zshrc
zshrcer -s fish -c /path/to/config.toml -o /path/to/output/config.fish
```

## Shell Support

zshrcer supports generating configuration for:

- **Zsh**: Full support for all features
- **Fish**: Support for most features, with automatic conversion of Zsh-specific syntax to Fish-compatible syntax

Note: Some Zsh-specific features (like `setopt` and `unsetopt`) are not available in Fish and will be commented out in the generated Fish configuration.

## Configuration Format

The configuration file uses TOML format. Here's an example:

```toml
plugins = [
    "zsh-users/zsh-autosuggestions",
    "zsh-users/zsh-syntax-highlighting",
    "marlonrichert/zsh-autocomplete",
]

[znap]
custom_path = "~/code/zsh/znap/znap.zsh"  # Optional, use a custom path for znap

[exports]
EDITOR = "nvim"
TERM = "xterm-256color"

[path]
prepend = ["$HOME/.cargo/bin", "$HOME/.local/bin"]
append = ["/usr/local/sbin"]

[aliases]
cd = "z"
ll = { cmd = "eza -al", if = "eza" }
ls = { cmd = "ls -la" }

[[modules]]
zsh_code = """
# ZSH Options
setopt appendhistory
setopt hist_ignore_all_dups
"""

[[modules]]
plugins = [
    "marlonrichert/zsh-autocomplete",
    "zsh-users/zsh-autosuggestions",
]

[[modules]]
zsh_code = """
# Source files
[ -f ~/.zsh_secret ] && source ~/.zsh_secret
[ -f $HOME/.local/bin/env ] && source $HOME/.local/bin/env
"""
fish_code = """
# Source files
if test -f ~/.fish_secret
    . ~/.fish_secret
end
if test -f $HOME/.local/bin/env
    . $HOME/.local/bin/env
end
"""

[tools.starship]
if_cmd = "starship"
commands = ['eval "$(starship init zsh)"']
fish_commands = ['starship init fish | source']

[tools.fzf]
enabled = true
if-file = "~/.fzf.zsh"
commands = ['source ~/.fzf.zsh']

[tools.nvm]
enabled = true
exports = { NVM_DIR = "$HOME/.nvm" }
commands = [
    '[ -s "$NVM_DIR/nvm.sh" ] && source "$NVM_DIR/nvm.sh"',
    '[ -s "$NVM_DIR/bash_completion" ] && source "$NVM_DIR/bash_completion"',
]
fish_commands = [
  'if test -s "$NVM_DIR/nvm.sh"
    bass source "$NVM_DIR/nvm.sh"
  end'
]

[tools.pnpm]
exports = { PNPM_HOME = "$HOME/.local/share/pnpm" }
path.prepend = ["$PNPM_HOME/bin"]
```

## Configuration Sections

### Plugins

List ZSH plugins to load with znap:

```toml
plugins = [
  "zsh-users/zsh-autosuggestions",
  "zsh-users/zsh-syntax-highlighting",
  "marlonrichert/zsh-autocomplete"
]
```

The tool uses [znap](https://github.com/marlonrichert/zsh-snap) as the plugin manager. If znap is not found on your system, it will be automatically installed.

### Znap Configuration

Optionally configure znap with a custom path:

```toml
[znap]
custom_path = "~/code/zsh/znap/znap.zsh"  # Use a custom path for znap
```

### Exports

Define environment variables to export:

```toml
[exports]
EDITOR = "nvim"
VISUAL = "nvim"
TERM = "xterm-256color"
```

### Path

Configure PATH environment variable:

```toml
[path]
prepend = ["$HOME/.cargo/bin", "$HOME/.local/bin"]
append = ["/usr/local/sbin", "/opt/bin"]
```

### Aliases

Define aliases with two different formats:

Simple aliases:

```toml
[aliases]
la = "ls -A"
cd = "z"
```

Conditional aliases (only created if a command exists):

```toml
[aliases]
ll = { cmd = "eza -al", if = "eza" }  # Only create this alias if 'eza' command exists
```

### Modules

Define modular components for your zsh configuration:

```toml
# Module with ZSH options
[[modules]]
zsh_code = """
# ZSH Options
setopt appendhistory
setopt hist_ignore_all_dups
"""

# Module with plugins
[[modules]]
plugins = [
    "marlonrichert/zsh-autocomplete",
    "zsh-users/zsh-autosuggestions",
]

# Module with custom code for sourcing files
[[modules]]
zsh_code = """
# Source files
[ -f ~/.zsh_secret ] && source ~/.zsh_secret
[ -f $HOME/.local/bin/env ] && source $HOME/.local/bin/env
"""
fish_code = """
# Source files
if test -f ~/.fish_secret
    . ~/.fish_secret
end
if test -f $HOME/.local/bin/env
    . $HOME/.local/bin/env
end
"""
```

Modules allow you to organize your configuration into logical groups. Each module can contain:

- `plugins`: A list of plugins to load with znap
- `zsh_code`: Raw shell code to include in the generated .zshrc file
- `fish_code`: Raw shell code to include in the generated config.fish file

Use the `zsh_code` and `fish_code` fields in modules to:

- Set ZSH options with `setopt` and `unsetopt`
- Source files with proper existence checks
- Add custom shell functions
- Include any other shell code that doesn't fit into the other configuration sections

The modules are processed in the order they appear in the config file, so you can control the sequence of operations in your .zshrc.

### Tools

Configure tools with their own exports, path modifications, and initialization commands:

```toml
# Basic tool configuration
[tools.starship]
commands = ['eval "$(starship init zsh)"']

# Tool with command existence check
[tools.zoxide]
if_cmd = "zoxide"  # Only execute if the command exists
commands = ['eval "$(zoxide init zsh)"']

# Tool with conditional file check
[tools.fzf]
enabled = true  # Optional, defaults to true if not specified
if-file = "~/.fzf.zsh"  # Only execute commands if this file exists
commands = ['source ~/.fzf.zsh']

# Tool with exports and commands
[tools.nvm]
enabled = true
exports = { NVM_DIR = "$HOME/.nvm" }
commands = [
  '[ -s "$NVM_DIR/nvm.sh" ] && source "$NVM_DIR/nvm.sh"',
  '[ -s "$NVM_DIR/bash_completion" ] && source "$NVM_DIR/bash_completion"'
]
fish_commands = [
  'if test -s "$NVM_DIR/nvm.sh"
    bass source "$NVM_DIR/nvm.sh"
  end'
]

# Tool with path configuration
[tools.pnpm]
exports = { PNPM_HOME = "$HOME/.local/share/pnpm" }
path.prepend = ["$PNPM_HOME/bin"]
```

Tool configuration options:

- `enabled`: Optional boolean to enable/disable the tool (defaults to true if not specified)
- `if-file`: Optional path to a file that must exist for the tool to be configured
- `if_cmd`: Optional command that must exist for the tool to be configured
- `exports`: Optional map of environment variables to export
- `path.prepend`: Optional list of paths to prepend to PATH
- `path.append`: Optional list of paths to append to PATH
- `commands`: Optional list of commands to execute (used for Zsh by default)
- `fish_commands`: Optional list of commands to execute specifically for Fish shell

### Shell-Specific Tool Commands

When configuring tools, you can specify different initialization commands for each shell type:

```toml
[tools.starship]
if_cmd = "starship"
# Commands used for Zsh
commands = ['eval "$(starship init zsh)"']
# Commands used for Fish
fish_commands = ['starship init fish | source']

[tools.pyenv]
if_cmd = "pyenv"
# Commands used for Zsh
commands = ['eval "$(pyenv init -)"']
# Commands used specifically for Fish
fish_commands = ['status --is-interactive; and pyenv init - | source']

[tools.nvm]
exports = { NVM_DIR = "$HOME/.nvm" }
# Zsh-specific commands
commands = [
  '[ -s "$NVM_DIR/nvm.sh" ] && source "$NVM_DIR/nvm.sh"',
  '[ -s "$NVM_DIR/bash_completion" ] && source "$NVM_DIR/bash_completion"'
]
# Fish-specific commands (using bass to run bash scripts in fish)
fish_commands = [
  'if test -s "$NVM_DIR/nvm.sh"
    bass source "$NVM_DIR/nvm.sh"
  end'
]
```

If `fish_commands` is provided, it will be used exclusively for the Fish shell. If not provided, the tool will attempt to convert the standard `commands` to Fish syntax.

## License

MIT

## Fish-Specific Considerations

When generating Fish shell configuration, zshrcer automatically converts Zsh syntax to Fish-compatible syntax:

### Automatic Conversions

1. **Environment Variables**:

   - Zsh: `export VAR=value`
   - Fish: `set -gx VAR value`

2. **Path Management**:

   - Zsh: `export PATH=$HOME/.local/bin:$PATH`
   - Fish: `fish_add_path --prepend $HOME/.local/bin`

3. **Sourcing Files**:

   - Zsh: `source file.sh`
   - Fish: `. file.sh`

4. **Conditional File Sourcing**:

   - Zsh: `[ -f file.sh ] && source file.sh`
   - Fish: `if test -f file.sh; . file.sh; end`

5. **Tool Initialization**:
   - Zsh: `eval "$(tool init zsh)"`
   - Fish: `eval "$(tool init fish)"` (for tools that support Fish)

### Unsupported Features

Some Zsh features don't have direct equivalents in Fish:

1. **Shell Options**: Zsh's `setopt` and `unsetopt` commands are commented out in Fish configuration.

2. **Znap Plugin Manager**: Fish doesn't use Znap, so plugin loading is skipped in Fish configuration.

### Best Practices

For maximum compatibility between shells:

1. Use the `modules` section with `zsh_code` and `fish_code` blocks for shell-specific customizations.

2. For tools that have different initialization commands between shells, use the `tools` section with appropriate `if_cmd` conditions.

3. Avoid using shell-specific syntax in your configuration when possible.

### Shell-Specific Code Blocks

The `modules` section now supports defining shell-specific code blocks using `zsh_code` and `fish_code`:

```toml
[[modules]]
# Code that will only be included in .zshrc
zsh_code = """
# ZSH-specific configuration
setopt histignorealldups
bindkey '^[[A' history-substring-search-up
"""

[[modules]]
# Code that will only be included in config.fish
fish_code = """
# Fish-specific configuration
set fish_greeting ""
bind \e\[A history-search-backward
"""

[[modules]]
# You can include both in the same module
zsh_code = """
# ZSH-specific way to set up tool X
source ~/.toolx/init.zsh
"""
fish_code = """
# Fish-specific way to set up tool X
. ~/.toolx/init.fish
"""
```

When generating shell configurations:

- For Zsh: Only the `zsh_code` blocks will be included
- For Fish: The `fish_code` blocks will be included when available
- For Fish: If no `fish_code` is provided but `zsh_code` exists, the `zsh_code` will be automatically converted to Fish syntax

This approach allows you to:

1. Provide optimized, shell-specific code for each shell
2. Fallback to automatic conversion when only Zsh code is available
3. Organize related configurations together even when the syntax differs between shells

### Tool-Specific Commands for Fish

Many tools have different initialization methods between Zsh and Fish. To handle this, the `tools` section supports a dedicated `fish_commands` field:

```toml
# Tool initialization with shell-specific commands
[tools.starship]
if_cmd = "starship"
# Zsh-specific initialization
commands = ['eval "$(starship init zsh)"']
# Fish-specific initialization
fish_commands = ['starship init fish \| source']
```

Benefits of shell-specific tool commands:

1. **Optimized for each shell**: No automatic conversion required, use the exact command syntax recommended for each shell
2. **More reliable**: Some tools work differently between shells and require completely different initialization approaches
3. **Cleaner syntax**: Use idiomatic Fish shell syntax rather than converted Zsh commands

Common tools that benefit from shell-specific commands:

| Tool     | Zsh Command                   | Fish Command                                        |
| -------- | ----------------------------- | --------------------------------------------------- |
| starship | `eval "$(starship init zsh)"` | `starship init fish \| source`                      |
| zoxide   | `eval "$(zoxide init zsh)"`   | `zoxide init fish \| source`                        |
| pyenv    | `eval "$(pyenv init -)"`      | `status is-interactive; and pyenv init - \| source` |
| nvm      | Source with bash-style checks | Often requires `bass` to work properly              |
| direnv   | `eval "$(direnv hook zsh)"`   | `direnv hook fish \| source`                        |
