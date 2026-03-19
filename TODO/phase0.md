# Phase 0 — Apply Pipeline Scaffold

This document tracks the scaffolding of the `apply/` module in `arqemo-core`
and what remains to make `arqemo apply brutalist` fully functional.

---

## What Was Scaffolded

### `apply/error.rs`

**Invariant:** One error variant per failure domain.

Defines `ApplyError` enum with variants for Hyprland not running, socket missing,
hyprctl failures, wallpaper tool missing, wallpaper command failures, and
unimplemented wallpaper modes. Each variant carries context (`PathBuf` or `String`).

### `apply/preflight.rs`

**Invariant:** Environment readiness checks.

Owns two checks:
1. **Hard check** — `HYPRLAND_INSTANCE_SIGNATURE` env var exists and socket path exists.
   Failure aborts the pipeline.
2. **Soft checks** — Tools referenced in config (swww, swaybg) exist in PATH.
   Failures are collected as warnings and reported, but do not abort.

Returns `PreflightResult { warnings: Vec<String> }` on success.

### `apply/hyprland.rs`

**Invariant:** Hyprland configuration via hyprctl.

Maps `[hyprland]` section fields to `hyprctl keyword` calls:
- `general:gaps_in`
- `general:gaps_out`
- `decoration:rounding`
- `general:border_size`
- `general:col.active_border` (from `colors.accent`)

Uses `std::process::Command`, not the IPC socket (Phase 3+).

### `apply/wallpaper.rs`

**Invariant:** Wallpaper activation by mode.

Dispatches on `wallpaper.mode`:
- `image` — dispatches on `backend` field: `swww` (with transition config) or `hyprpaper` (via hyprctl IPC)
- `solid` — stub, returns `WallpaperModeUnimplemented` (Phase 1)
- `glsl` — stub, returns `WallpaperModeUnimplemented` (Phase 3)
- `renderer` — stub, returns `WallpaperModeUnimplemented` (Phase 3)

Handles tilde expansion for paths. Respects `[wallpaper.transition]` for swww.

### `apply/mod.rs`

**Invariant:** Pipeline sequencing.

Executes stages in order: preflight → hyprland → wallpaper.
Only `apply()` and `ApplyError` are public. Submodules are private.

### `lib.rs` (modified)

Wired `apply::apply()` into the public `apply()` function.
Now loads theme, validates (file + semantic), and runs the apply pipeline.
Dry-run mode prints intent without applying.

---

## What Is NOT Scaffolded (and Why)

### `widgets.rs`

Not created. Widget layer is Phase 2 scope (Astal/GTK4 bar, launcher, OSD).
No invariant to own yet — would be an empty stub.

### IPC socket client (`arqemo-ipc`)

Phase 0 uses `hyprctl` via `std::process::Command`.
Direct socket communication is Phase 3+ when we need event streams and
lower latency for window decoration system.

### Template rendering (`template.rs`)

Tera rendering is not wired into the apply pipeline yet.
Currently only `templates_dir()` path helper exists.
Need to render `hyprland.conf.tera` and other templates to cache.

---

## Remaining Work for `arqemo apply brutalist`

### 1. Template rendering (Tera)

`template.rs` needs:
- Load Tera templates from `~/.config/arqemo/templates/`
- Render with `ThemeConfig` as context
- Write outputs to `~/.cache/arqemo/rendered/`

Currently a stub with path helpers only.

### 2. Hyprland keyword syntax verification

`hyprland.rs` sends commands but doesn't verify:
- Correct key names for Hyprland 0.54 (may have changed)
- `col.active_border` format (`rgb()` vs `rgba()` vs hex)

Need to test against a live Hyprland instance.

### 3. Wallpaper tool invocation

`wallpaper.rs` `image` mode works for both backends but:
- `swww` requires `swww-daemon` running first — should check or auto-start
- Pool+default resolution not yet wired — `activate_image()` only handles `path`, not `pool`+`default` join
- `WallpaperOverride` cache not read at apply time yet

### 4. `list_themes()` stub

Currently prints to stdout directly. Should:
- Return `Vec<String>` for programmatic use
- Integrate with `ThemeRegistry` from config module

---

## Recommended Implementation Order

1. **Template rendering** — Tera integration, render to cache
2. **Hyprland syntax fixes** — Test hyprctl keywords on live compositor
3. **Wallpaper daemon handling** — swww-daemon auto-start
4. **list_themes() refactor** — Return data, not print

---

## Current State

```
arqemo apply <theme>
  ├─ ✓ load theme.toml → serde → ThemeConfig
  ├─ ✓ validate::file::validate_file()
  ├─ ✓ validate::semantic::validate_semantic()
  ├─ ✗ render templates → ~/.cache/arqemo/ (not implemented)
  ├─ ✓ preflight checks (env var, socket, tools)
  ├─ ✓ hyprland: gaps, rounding, border (via hyprctl)
  └─ ✓ wallpaper: image mode (swww w/ transitions, hyprpaper via hyprctl)
```

The workspace compiles cleanly. `todo!()` macros remain in wallpaper
subcommand stubs (`set`, `next`, `random`, `reset`). All apply pipeline
stages are functional or return explicit `WallpaperModeUnimplemented`
errors for unimplemented modes.
