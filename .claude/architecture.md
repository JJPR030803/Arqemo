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

