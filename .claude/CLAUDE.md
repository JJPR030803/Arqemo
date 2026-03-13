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

