use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, Context};

mod config;
mod generator;

#[derive(Parser, Debug)]
#[command(name = "shrcer")]
#[command(author = "shrcer")]
#[command(version = "0.1.0")]
#[command(about = "A tool to generate .zshrc/.fishrc files from TOML configuration")]
struct Args {
    /// Path to the config.toml file
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// Path to output .zshrc file
    #[arg(short, long)]
    zshrc: Option<PathBuf>,

    /// Path to output .fishrc file
    #[arg(short, long)]
    fishrc: Option<PathBuf>,

    /// Print to stdout instead of writing to files
    #[arg(short, long)]
    print: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read and parse the config file
    let config_content = std::fs::read_to_string(&args.config)
        .with_context(|| format!("Failed to read config file: {}", args.config.display()))?;
    
    let config = config::parse_config(&config_content)
        .with_context(|| "Failed to parse config file")?;

    // Generate zsh config if requested
    if let Some(zshrc_path) = &args.zshrc {
        let zsh_content = generator::generate_zsh(&config)
            .with_context(|| "Failed to generate zsh configuration")?;
        
        if args.print {
            println!("# ZSH Configuration\n{}", zsh_content);
        } else {
            std::fs::write(zshrc_path, zsh_content)
                .with_context(|| format!("Failed to write zshrc to {}", zshrc_path.display()))?;
            println!("ZSH configuration written to {}", zshrc_path.display());
        }
    }

    // Generate fish config if requested
    if let Some(fishrc_path) = &args.fishrc {
        let fish_content = generator::generate_fish(&config)
            .with_context(|| "Failed to generate fish configuration")?;
        
        if args.print {
            println!("# Fish Configuration\n{}", fish_content);
        } else {
            std::fs::write(fishrc_path, fish_content)
                .with_context(|| format!("Failed to write fishrc to {}", fishrc_path.display()))?;
            println!("Fish configuration written to {}", fishrc_path.display());
        }
    }

    // If no output file specified, just print the zsh config
    if args.zshrc.is_none() && args.fishrc.is_none() {
        let zsh_content = generator::generate_zsh(&config)
            .with_context(|| "Failed to generate zsh configuration")?;
        println!("# ZSH Configuration\n{}", zsh_content);
    }

    Ok(())
} 