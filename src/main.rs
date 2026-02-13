use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use gh_profile_gen::config::toml_io;
use gh_profile_gen::error::ConfigError;
use gh_profile_gen::render::markdown;

#[derive(Parser)]
#[command(name = "gh-profile-gen", version = "0.1.0")]
#[command(about = "Generate comprehensive GitHub profile README.md files")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a starter profile.toml with all fields shown as examples
    Init {
        /// Output path (default: profile.toml)
        #[arg(short, long, default_value = "profile.toml")]
        output: PathBuf,

        /// Overwrite existing file
        #[arg(long)]
        force: bool,
    },
    /// Render a profile.toml into README.md
    Render {
        /// Path to the TOML configuration file
        file: PathBuf,

        /// Output path (default: README.md)
        #[arg(short, long, default_value = "README.md")]
        output: PathBuf,

        /// Print to stdout instead of writing to file
        #[arg(long)]
        stdout: bool,
    },
    /// Preview the rendered README in the terminal
    Preview {
        /// Path to the TOML configuration file
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { output, force }) => cmd_init(&output, force),
        Some(Commands::Render {
            file,
            output,
            stdout,
        }) => cmd_render(&file, &output, stdout),
        Some(Commands::Preview { file }) => cmd_preview(&file),
        None => {
            println!("gh-profile-gen v0.1.0");
            println!("Use --help for usage information, or run a subcommand:");
            println!("  init     Generate a starter profile.toml");
            println!("  render   Render profile.toml to README.md");
            println!("  preview  Preview rendered README in terminal");
            Ok(())
        }
    }
}

fn cmd_init(output: &Path, force: bool) -> Result<()> {
    if output.exists() && !force {
        anyhow::bail!(
            "{} already exists. Use --force to overwrite.",
            output.display()
        );
    }

    let starter: String = toml_io::generate_starter_toml();
    std::fs::write(output, starter)
        .with_context(|| format!("could not write {}", output.display()))?;
    println!("Created {}", output.display());
    Ok(())
}

fn cmd_render(file: &Path, output: &Path, stdout: bool) -> Result<()> {
    let config = toml_io::load_config(file)?;

    if config.meta.username.is_empty() {
        return Err(ConfigError::MissingUsername.into());
    }

    let readme: String = markdown::render(&config);

    if stdout {
        print!("{}", readme);
    } else {
        std::fs::write(output, &readme)
            .with_context(|| format!("could not write {}", output.display()))?;
        println!("Wrote {}", output.display());
    }

    Ok(())
}

fn cmd_preview(file: &Path) -> Result<()> {
    let config = toml_io::load_config(file)?;

    if config.meta.username.is_empty() {
        return Err(ConfigError::MissingUsername.into());
    }

    let readme: String = markdown::render(&config);
    print!("{}", readme);
    Ok(())
}
