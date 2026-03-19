//! Theme schema types. Deserialized directly from theme.toml via serde.
//! Type validation happens here (serde). Semantic validation is in errors.rs.

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ThemeConfig {
    pub meta: Meta,
    pub typography: Typography,
    pub colors: Colors,
    pub hyprland: Hyprland,
    pub workspaces: Option<HashMap<String, WorkspaceConfig>>,
    pub wallpaper: Wallpaper,
    pub renderer: Option<Renderer>,
    pub widgets: Option<Widgets>,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub tags: Vec<String>,
    pub renderer: String,
    pub widgets: String,
}

#[derive(Deserialize, Debug)]
pub struct Typography {
    pub font_mono: String,
    pub font_size: u8,
}

#[derive(Deserialize, Debug)]
pub struct Colors {
    // Terminal bg/fg
    pub bg: String,
    pub fg: String,
    // 16 ANSI
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
    // Arqemo semantic
    pub accent: String,
    pub surface0: String,
    pub surface1: String,
    pub surface2: String,
    pub muted: String,
}

#[derive(Deserialize, Debug)]
pub struct Hyprland {
    pub border_size: u8,
    pub gaps_in: u8,
    pub gaps_out: u8,
    pub rounding: u8,
    pub blur: bool,
    pub animations: Option<Animations>,
}

#[derive(Deserialize, Debug)]
pub struct Animations {
    pub preset: Option<String>,
    pub custom: Option<Vec<AnimationEntry>>,
}

#[derive(Deserialize, Debug)]
pub struct AnimationEntry {
    pub name: String,
    pub bezier: String,
    pub style: String,
}

#[derive(Deserialize, Debug)]
pub struct WorkspaceConfig {
    pub layout: String,
}

/// Wallpaper display mode.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum WallpaperMode {
    Image,
    Solid,
    Glsl,
    Renderer,
}

/// Wallpaper backend for image mode.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum WallpaperBackend {
    Hyprpaper,
    Swww,
}

/// Swww transition configuration.
#[derive(Debug, Deserialize)]
pub struct WallpaperTransition {
    /// Transition type: "fade", "wipe", "wave", "grow", "outer", "random".
    #[serde(rename = "type")]
    pub kind: String,
    /// Duration in seconds.
    pub duration: f32,
    /// Frames per second.
    pub fps: u32,
    /// Optional bezier curve.
    pub bezier: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Wallpaper {
    pub mode: WallpaperMode,

    // image mode — single file (mutually exclusive with pool)
    pub path: Option<String>,

    // image mode — pool
    pub pool: Option<String>,
    pub default: Option<String>,

    // image mode — backend (None = default to hyprpaper at apply time)
    pub backend: Option<WallpaperBackend>,

    // swww transition (only valid when backend = Some(Swww))
    pub transition: Option<WallpaperTransition>,

    // solid mode
    pub color: Option<String>,

    // glsl mode
    pub shader: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Renderer {
    pub binary: String,
    pub args: Vec<String>,
    pub fps: u8,
    pub params: Option<HashMap<String, toml::Value>>,
}

#[derive(Deserialize, Debug)]
pub struct Widgets {
    pub preset: Option<String>,
    pub bar: Option<WidgetComponent>,
    pub launcher: Option<WidgetComponent>,
    pub osd: Option<WidgetComponent>,
    pub notifications: Option<WidgetComponent>,
}

#[derive(Deserialize, Debug)]
pub struct WidgetComponent {
    pub r#type: String,
    pub config: Option<String>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn placeholder_schema_test() {
        // Replace with real fixture parsing in Phase 1
        let _ = std::mem::size_of::<ThemeConfig>();
    }
}
