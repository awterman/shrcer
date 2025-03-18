h# shrcer

A config-based generator for shell configuration files (.zshrc, .fishrc) from a TOML configuration.

## Features

- Generate .zshrc and .fishrc files from a single TOML configuration
- Configure environment variables, path manipulations, and aliases
- Custom shell code sections for each shell
- Plugin management with znap (for zsh)
- Tool-specific configurations with conditional loading

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Generate .zshrc file
shrcer -z ~/.zshrc

# Generate .fishrc file
shrcer -f ~/.config/fish/config.fish

# Generate both
shrcer -z ~/.zshrc -f ~/.config/fish/config.fish

# Use a specific config file
shrcer -c path/to/config.toml -z ~/.zshrc

# Print to stdout instead of writing to file
shrcer -p
```

## Configuration

Create a `config.toml` file with the following structure:

```toml
# ZSH Options module
[[modules]]
zsh_code = """
# ZSH Options
setopt appendhistory
setopt hist_ignore_all_dups
"""

# Environment variables
[exports]
EDITOR = "nvim"  # Common export for all shells

[exports.zsh]    # ZSH-specific exports
HISTFILE = "~/.zsh_history"

[exports.fish]   # Fish-specific exports
fish_greeting = ""

# Path configuration
[path]
prepend = ["$HOME/.local/bin"]
append = ["/usr/local/bin"]

# Aliases
[aliases]
ls = { cmd = "eza --group-directories-first --icons=auto", if = "eza" }
cd = { cmd = "z", if = "zoxide" }

# Znap configuration (for ZSH plugin management)
[znap]
custom_path = "$HOME/.znap/znap.zsh"

# Plugins module
[[modules]]
plugins = [
    "zsh-users/zsh-autosuggestions",
    "zsh-users/zsh-syntax-highlighting",
]

# Tool configurations
[tools.cargo]
if_file = "~/.cargo/env"
zsh_code = "source ~/.cargo/env"
fish_code = "source ~/.cargo/env"

[tools.starship]
if_cmd = "starship"
zsh_code = "source <(starship init zsh --print-full-init)"
fish_code = "starship init fish --print-full-init | source"
```

## License

MIT
