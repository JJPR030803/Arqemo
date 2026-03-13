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

