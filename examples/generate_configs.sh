#!/bin/bash
# Example script to generate shell configurations

# Make sure shrcer is installed
if ! command -v shrcer &> /dev/null; then
    echo "Error: shrcer is not installed. Please install it first."
    echo "cargo install --path ."
    exit 1
fi

# Create backup of existing config files
if [ -f ~/.zshrc ]; then
    echo "Creating backup of existing .zshrc file"
    cp ~/.zshrc ~/.zshrc.bak
fi

if [ -f ~/.config/fish/config.fish ]; then
    echo "Creating backup of existing fish config file"
    cp ~/.config/fish/config.fish ~/.config/fish/config.fish.bak
fi

# Generate new config files
echo "Generating new shell configuration files..."

# For ZSH
shrcer -c ./config.toml -z ~/.zshrc

# For Fish
mkdir -p ~/.config/fish
shrcer -c ./config.toml -f ~/.config/fish/config.fish

echo "Done! Shell configuration files have been generated."
echo "If you need to restore the previous configurations, use:"
echo "  mv ~/.zshrc.bak ~/.zshrc"
echo "  mv ~/.config/fish/config.fish.bak ~/.config/fish/config.fish" 