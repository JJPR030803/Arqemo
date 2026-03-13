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

