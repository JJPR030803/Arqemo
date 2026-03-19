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

        /// Informational only, no errors
        #[arg(long)]
        info: bool,
    },
    /// List all available themes
    List {
        #[arg(long)]
        complete_path: bool,
    },
    /// Initialise the arqemo directory structure
    Init,
    /// Manage wallpaper within the active theme
    Wallpaper {
        #[command(subcommand)]
        command: WallpaperCommands,
    },
}

#[derive(Subcommand)]
enum WallpaperCommands {
    /// Set a specific wallpaper from the active theme's pool
    Set {
        /// Wallpaper filename within the pool
        name: String,
    },
    /// Cycle to the next wallpaper in the pool
    Next,
    /// Pick a random wallpaper from the pool
    Random,
    /// Clear override and revert to theme default
    Reset,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Apply { theme, dry_run } => arqemo_core::apply(&theme, dry_run).await,
        Commands::Validate { theme, info } => arqemo_core::validate_theme(&theme, info),
        Commands::List { complete_path } => arqemo_core::list_themes(complete_path),
        Commands::Init => arqemo_core::init(),
        Commands::Wallpaper { command } => match command {
            WallpaperCommands::Set { name } => arqemo_core::wallpaper_set(&name),
            WallpaperCommands::Next => arqemo_core::wallpaper_next(),
            WallpaperCommands::Random => arqemo_core::wallpaper_random(),
            WallpaperCommands::Reset => arqemo_core::wallpaper_reset(),
        },
    }
}
