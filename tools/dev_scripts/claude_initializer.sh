#!/usr/bin/env bash
# =============================================================================
# Arqemo — Claude Code context initializer
# Run this from the workspace root: ~/projects/arqemo/
# Creates .claude/ with project context for code generation sessions.
# =============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLAUDE_DIR="$SCRIPT_DIR/.claude"

echo "Initializing .claude/ context at $CLAUDE_DIR"
mkdir -p "$CLAUDE_DIR"

# =============================================================================
# CLAUDE.md — the file Claude Code reads automatically on every session
# =============================================================================
cat > "$CLAUDE_DIR/CLAUDE.md" << 'EOF'
# Arqemo — Claude Code Context

You are working on **Arqemo** (*arquitectura emocional*), a Rust workspace
that implements a Linux desktop theming framework for Arch Linux, Hyprland,
and Wayland. Read this file before writing any code.

---

## What this project is

A framework for building desktop environments as deliberate aesthetic
experiences. Not a rice manager. Not a dotfile framework. A platform where
generative art and creative coding are first-class citizens of the desktop —
where the desktop itself can become a world indistinguishable from one built
in a game engine.

The framework provides contracts, lifecycle management, and utilities.
Each theme is its own world. The framework has no opinions about aesthetics.

---

## Stack

| Layer | Tool | Notes |
|---|---|---|
| Framework core | Rust | Single compiled binary `arqemo` |
| GPU / world engine | wgpu | Compute + render, all 4 Wayland layer surfaces |
| Shaders | GLSL | Via wgpu pipeline or standalone mpvpaper |
| Widget layer | TypeScript / Astal | GTK4, replaceable per theme |
| Neovim integration | Lua | Watches cache file, hot-reloads colorscheme |
| Theme data | TOML | Single source of truth, validated against typed Rust structs |

---

## Workspace layout

```
arqemo/
├── crates/
│   ├── arqemo-cli/       # binary entry point — thin, no logic
│   ├── arqemo-core/      # orchestration, schema, validation, templates
│   ├── arqemo-ipc/       # Hyprland socket bridge
│   ├── arqemo-renderer/  # wgpu world engine (Phase 3+)
│   ├── arqemo-audio/     # sound lifecycle (Phase 5+)
│   └── arqemo-color/     # OKLCH author utilities (Phase 6+)
└── .claude/
    ├── CLAUDE.md         # this file — read every session
    ├── schema.md         # full TOML schema reference
    ├── architecture.md   # crate responsibilities and boundaries
    ├── conventions.md    # code quality rules — READ BEFORE WRITING CODE
    └── phases.md         # current phase and what is in scope
```

**Runtime directories (not in this repo):**
- `~/.config/arqemo/themes/` — theme.toml files
- `~/.config/arqemo/templates/` — Tera templates
- `~/.cache/arqemo/` — rendered outputs
- `~/.local/bin/arqemo` — installed binary

---

## Before writing any code

1. Read `.claude/conventions.md` — code quality rules are strict
2. Read `.claude/phases.md` — only implement what is in scope for the current phase
3. Read `.claude/architecture.md` — understand which crate owns which responsibility
4. Never put logic in `arqemo-cli`. It routes commands to `arqemo-core` only.
5. Never use `.unwrap()` or `.expect()` outside `#[cfg(test)]` blocks.

EOF

echo "  wrote CLAUDE.md"

# =============================================================================
# schema.md — full TOML schema reference
# =============================================================================
cat > "$CLAUDE_DIR/schema.md" << 'EOF'
# Arqemo — Theme Schema Reference

The schema is the single source of truth for every theme.
Deserialized via serde into typed Rust structs in `arqemo-core/src/schema.rs`.

## Design rules

- **All color keys are required.** Missing keys are hard errors — no silent
  fallback, no derivation.
- **Color generation is not the default path.** The artist decides the colors.
  They write them in theme.toml. That is it.
- **Mode-conditional validation.** Forbidden keys for the wrong wallpaper mode
  are hard errors in `validate.rs`, not warnings.
- **The schema never grows to accommodate component needs.** Only named
  artistic decisions live in it. Components derive locally from the palette
  they receive.

---

## Full schema

```toml
# [meta] — identity and routing signals only
# Nothing templates touch directly.
[meta]
name        = "brutalist"
version     = "0.1"
description = "Cold. Structural. No decoration."
tags        = ["dark", "animated", "minimal"]
renderer    = "wgpu-sketch"   # routing signal → binary type to look up
widgets     = "astal"         # routing signal → which reload hook to call

# [typography] — font choices, as deliberate as color choices
[typography]
font_mono = "Iosevka"
font_size = 13

# [colors] — hand-authored palette
# ALL keys required. apply errors on missing keys.
[colors]
bg  = "#0a0a0a"
fg  = "#e0e0e0"
# 16 ANSI
black          = "#0a0a0a"
red            = "#ff5555"
green          = "#55ff55"
yellow         = "#f1c40f"
blue           = "#6699cc"
magenta        = "#cc66ff"
cyan           = "#66ffcc"
white          = "#e0e0e0"
bright_black   = "#333333"
bright_red     = "#ff7777"
bright_green   = "#77ff77"
bright_yellow  = "#f5d84f"
bright_blue    = "#88bbee"
bright_magenta = "#dd88ff"
bright_cyan    = "#88ffdd"
bright_white   = "#ffffff"
# Arqemo semantic
accent   = "#ffffff"
surface0 = "#111111"
surface1 = "#1a1a1a"
surface2 = "#242424"
muted    = "#555555"

# [hyprland] — window manager parameters
[hyprland]
border_size = 2
gaps_in     = 2
gaps_out    = 4
rounding    = 0
blur        = false

[hyprland.animations]
preset = "none"   # none | soft | fluid | snappy
# OR: [[hyprland.animations.custom]] array for full bezier control
# custom takes priority over preset when present

# [workspaces] — per-workspace layout (layout IS part of the aesthetic)
[workspaces]
1 = { layout = "dwindle" }
2 = { layout = "dwindle" }
3 = { layout = "scroll" }
4 = { layout = "scroll" }
9 = { layout = "monocle" }

# [wallpaper] — activation mode + mode-specific config
# mode routes apply to the correct activation path
# Forbidden keys for wrong mode are hard errors
[wallpaper]
mode = "renderer"
# mode = "image"   requires: path
# mode = "solid"   requires: color     forbidden: path, shader
# mode = "glsl"    requires: shader    forbidden: path, color
# mode = "renderer" requires: [renderer] section

# [renderer] — Level 2 themes only
[renderer]
binary = "~/.local/bin/arqemo-brutalist"
args   = ["--mode", "reaction-diffusion"]
fps    = 30

[renderer.params]
# Passed as ARQEMO_PARAM_KEY=value env vars to the renderer process.
# Fully opaque to the framework — renderer owns its own param validation.
chaos    = 0.1
contrast = 1.8
seed     = "random"

# [widgets] — component model, mix-and-match per theme
[widgets]
[widgets.bar]
type   = "astal"
config = "bar.ts"   # optional — omit for arqemo default

# [widgets.launcher]   type = "rofi" | "astal" | "none" | "custom"
# [widgets.osd]        type = "astal" | "none"
# [widgets.notifications]  type = "dunst" | "none"
```

---

## Wallpaper mode validation table

| mode | required keys | forbidden keys |
|---|---|---|
| `image` | `path` | `color`, `shader` |
| `solid` | `color` | `path`, `shader` |
| `glsl` | `shader` | `path`, `color` |
| `renderer` | `[renderer]` section | `path`, `color`, `shader` |

EOF

echo "  wrote schema.md"

# =============================================================================
# architecture.md — crate responsibilities and boundaries
# =============================================================================
cat > "$CLAUDE_DIR/architecture.md" << 'EOF'
# Arqemo — Architecture Reference

## The most important rule

**The CLI crate contains zero logic.**
`arqemo-cli/src/main.rs` parses arguments with clap and calls `arqemo-core`.
Nothing else. All logic lives in library crates.

---

## Crate responsibilities

### `arqemo-cli`
- Binary entry point
- Clap argument parsing
- Routes to `arqemo-core` functions
- Dependencies: `arqemo-core`, `clap`, `anyhow`, `tokio`

### `arqemo-core`
- Apply pipeline orchestration
- Schema deserialization (`schema.rs`)
- Semantic validation (`validate.rs`)
- Tera template rendering (`template.rs`)
- Cache file management (`cache.rs`)
- Process lifecycle (renderer, widgets)
- Dependencies: `serde`, `toml`, `tera`, `anyhow`, `thiserror`, `tokio`, `dirs`

### `arqemo-ipc`
- Hyprland Unix socket bridge
- Event stream parsing (text → typed structs)
- Window geometry double buffer
- Hyprctl command dispatch
- Dependencies: `tokio`, `bincode`, `glam`
- **Phase 3+. Currently a stub.**

### `arqemo-renderer`
- wgpu world engine
- All 4 Wayland layer surfaces (background, bottom, top, overlay)
- ArqemoRenderer trait
- Window decoration system (spring-damper)
- Software cursor
- Dependencies: `wgpu`, `glam`, `raw-window-handle`
- **Phase 3+. Currently a stub.**

### `arqemo-audio`
- Ambient sound lifecycle
- Event sound dispatch
- Dependencies: `rodio` or `cpal`
- **Phase 5+. Currently a stub.**

### `arqemo-color`
- OKLCH perceptual color math
- Terminal color derivation from hand-authored palette
- WCAG contrast checking
- Dependencies: `palette`
- **Phase 6+. Currently a stub.**

---

## Framework vs theme boundary

```
Framework owns:
  → Compiled CLI binary
  → Orchestration lifecycle (apply, reload, teardown)
  → Schema validation (typed + semantic + filesystem)
  → Template rendering
  → Hyprland IPC client
  → Process lifecycle management
  → Cache management

Theme owns:
  → Its palette (hand-authored — always)
  → Its world simulation (wgpu binary, GLSL, static image, nothing)
  → Its window decoration style
  → Its widget layer
  → Its Hyprland parameters
  → Its acoustic environment
  → Its complexity level (TOML-only or compiled renderer)
```

---

## Runtime paths

```rust
// Config: ~/.config/arqemo/
fn config_dir() -> PathBuf {
    dirs::config_dir().unwrap().join("arqemo")
}

// Cache: ~/.cache/arqemo/
fn cache_dir() -> PathBuf {
    dirs::cache_dir().unwrap().join("arqemo")
}

// Themes: ~/.config/arqemo/themes/<name>/theme.toml
fn theme_path(name: &str) -> PathBuf {
    config_dir().join("themes").join(name).join("theme.toml")
}

// Templates: ~/.config/arqemo/templates/
fn templates_dir() -> PathBuf {
    config_dir().join("templates")
}
```

---

## Apply pipeline (Phase 0 target)

```
arqemo apply <theme>
  │
  ├─ load theme.toml → serde → ThemeConfig
  ├─ validate::validate(&config) → ValidationError or Ok
  ├─ render templates → ~/.cache/arqemo/
  ├─ hyprland IPC: set gaps, rounding, border color
  └─ wallpaper activation (mode-dependent)
```

---

## Error handling pattern

```
Library crates (arqemo-core, arqemo-ipc, etc.):
  → thiserror: typed error enums, one per module
  → ? operator for propagation
  → .with_context() at I/O boundaries

Binary (arqemo-cli):
  → anyhow::Result<()> in main
  → errors propagate up and print to stderr automatically
```

EOF

echo "  wrote architecture.md"

# =============================================================================
# conventions.md — code quality rules
# =============================================================================
cat > "$CLAUDE_DIR/conventions.md" << 'EOF'
# Arqemo — Code Conventions

Read this before writing any code. These rules are enforced by the
compiler and CI — violations will not compile.

---

## Non-negotiable rules

### No unwrap outside tests

```rust
// NEVER — hard compile error via clippy::deny(unwrap_used)
let value = something.unwrap();
let value = something.expect("message");

// CORRECT — propagate with ?
let value = something?;

// CORRECT — propagate with context
use anyhow::Context;
let value = something
    .with_context(|| format!("failed to read {path}"))?;

// OK — inside #[cfg(test)] blocks only
#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    // unwrap is fine here
}
```

### Error types

```rust
// Library crates: typed errors with thiserror
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("descriptive message: {0}")]
    Variant(String),
}

// Binary / surface: anyhow
fn main() -> anyhow::Result<()> { ... }
```

### No logic in arqemo-cli

```rust
// main.rs — routing ONLY
match cli.command {
    Commands::Apply { theme, dry_run } => {
        arqemo_core::apply(&theme, dry_run).await?;
    }
}
// NO business logic here. Call core and return.
```

---

## Lints (enforced workspace-wide)

```toml
# These are in the root Cargo.toml — do not disable them
[workspace.lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
todo        = "warn"    # warns but does not block compilation
pedantic    = "warn"
```

If clippy pedantic flags something you genuinely disagree with,
silence it at the specific site with a comment explaining why:

```rust
#[allow(clippy::module_name_repetitions)]  // name is intentionally explicit
pub struct ThemeConfig { ... }
```

Never silence a lint category wholesale.

---

## Formatting

- `cargo fmt` before every commit — non-negotiable
- `rustfmt.toml` at workspace root configures it
- Do not manually format — let rustfmt handle it

---

## Testing

```rust
// Unit tests: in the same file as the code
#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn descriptive_test_name() { ... }
}

// Integration tests: crates/arqemo-core/tests/
// One file per concern: schema_valid.rs, schema_invalid.rs, template_rendering.rs

// Compositor tests: marked ignore, never block CI
#[test]
#[ignore = "requires live Hyprland compositor"]
fn hyprctl_sets_gaps() { ... }
```

Test names describe behavior, not implementation:
- `missing_color_key_returns_error` ✓
- `test_validate_colors` ✗

---

## Module structure in arqemo-core

Each module has one job. Do not put validation logic in schema.rs.
Do not put template logic in cache.rs.

```
schema.rs   → serde structs only, no logic
validate.rs → semantic rules only, reads schema structs
template.rs → Tera rendering only
cache.rs    → file write helpers only
apply.rs    → pipeline orchestration, calls the others in order
```

---

## The one-liner gate

Before committing, run:

```bash
cargo fmt --check && cargo clippy --workspace -- -D warnings && cargo test --workspace
```

All three must pass. If any fails, fix it before committing.

EOF

echo "  wrote conventions.md"

# =============================================================================
# phases.md — current phase and scope
# =============================================================================
cat > "$CLAUDE_DIR/phases.md" << 'EOF'
# Arqemo — Development Phases

## Current phase: Phase 0 — Foundation

**Goal:** `arqemo apply brutalist` — typed from anywhere, no runner —
changes gaps, rounding, border colors, and wallpaper.

**In scope:**
- Workspace compiles cleanly with all 6 crates
- `arqemo-core/src/schema.rs` — all structs for the full schema
- `arqemo-core/src/validate.rs` — mode-conditional wallpaper validation
- `arqemo-core/src/template.rs` — Tera renders `hyprland.conf.tera`
- `arqemo-ipc` — basic hyprctl socket calls (gaps, rounding, border color)
- `arqemo-cli` — `arqemo apply <theme>` and `arqemo validate <theme>`
- `brutalist/theme.toml` — TOML-only, static wallpaper image
- Hyprland 0.54, per-workspace layouts configured

**NOT in scope yet (do not implement):**
- Astal bar or any widget layer
- Neovim / foot / dunst templates
- wgpu renderer or any GPU code
- Sound system
- Color engine
- `arqemo list`, `arqemo new`, `arqemo preview`

---

## Upcoming phases (reference only)

| Phase | Focus | Done when |
|---|---|---|
| 1 | Schema hardening — strict errors, dry-run | Every error tells you exactly what's wrong |
| 2 | Widget layer + terminal | `arqemo apply monochrome` changes every surface including Neovim live |
| 3 | wgpu world engine | brutalist has a living reaction-diffusion wallpaper |
| 4 | Window decorations + cursor | Decorations follow windows. Cursor is a simulation object. |
| 5 | Texture + sound | A recording could identify the active theme without seeing colors |
| 6 | Color tooling | Given a palette, derive all secondary colors and verify contrast |
| 7 | Interaction grammar | Switching themes changes how you talk to the machine |
| 8 | Theme library | calm and fallout coexist — stranger can't guess they share infrastructure |
| 9 | Temporal layer | fallout at 3am feels different from noon without a manual switch |
| 10 | Polish | System is comfortable to extend and has something worth showing |

EOF

echo "  wrote phases.md"

# =============================================================================
# Done
# =============================================================================
echo ""
echo "Done. Context initialized at .claude/"
echo ""
echo "Files created:"
echo "  .claude/CLAUDE.md       — read automatically by Claude Code every session"
echo "  .claude/schema.md       — full TOML schema reference"
echo "  .claude/architecture.md — crate boundaries and runtime paths"
echo "  .claude/conventions.md  — code quality rules (enforced)"
echo "  .claude/phases.md       — current phase and scope"
echo ""
echo "Claude Code reads CLAUDE.md automatically."
echo "Reference the others in prompts with: 'read .claude/conventions.md first'"
echo ""
echo "Keep .claude/ in version control."
echo "Update phases.md when you move to a new phase."
echo "Update schema.md if the TOML schema changes."