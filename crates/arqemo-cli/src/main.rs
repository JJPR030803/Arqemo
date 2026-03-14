use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "arqemo",
    about = "arquitectura emocional — desktop theming framework"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply a theme to the desktop
    Apply {
        /// Theme name (must exist in ~/.config/arqemo/themes/)
        theme: String,

        /// Validate and report changes without applying anything
        #[arg(long)]
        dry_run: bool,
    },
    /// Validate a theme file without applying it
    Validate {
        /// Theme name to errors
        theme: String,
    },
    /// List all available themes
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Apply { theme, dry_run } => arqemo_core::apply(&theme, dry_run).await,
        Commands::Validate { theme } => arqemo_core::list(),
        Commands::List => arqemo_core::list(),
    }
}
