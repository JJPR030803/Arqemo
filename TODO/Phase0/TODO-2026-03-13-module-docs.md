# Module State — 2026-03-13

Current state of the two newest modules in `arqemo-core`.

---

## `config/` — runtime configuration discovery

**Purpose:** Resolve XDG paths and build a registry of available themes.

| File | What it does |
|---|---|
| `mod.rs` | Re-exports `root`, `registry`, `error` |
| `error.rs` | `ConfigError` — 7 variants covering XDG resolution, missing dirs, theme lookup |
| `root.rs` | `ConfigRoot` — resolves `~/.config/arqemo/` and `~/.cache/arqemo/`, validates existence |
| `registry.rs` | `ThemeRegistry` — scans themes dir, indexes by name, provides `theme_path()` lookup |

**Flow:** `ConfigRoot::locate()` → `ThemeRegistry::scan(&root)` → `registry.theme_path("brutalist")`

**Tests:** 1 integration test (`tests/config_reader.rs`)

**No unit tests yet** — needs tests for missing dirs, empty themes dir, theme not found.

---

## `validate/` — two-phase theme validation

**Purpose:** Validate a theme.toml in two phases: file integrity, then semantic rules.

| File | What it does |
|---|---|
| `mod.rs` | Re-exports `errors`, `file`, `semantic`; has empty `helpers` |
| `errors.rs` | `ValidationError` (wraps `FileError` or `SemanticError`), `FileError` (7 variants), `SemanticError` (8 variants) |
| `file.rs` | `validate_file(path) → Result<ThemeConfig>` — exists, is_file, .toml, not empty, parses |
| `semantic.rs` | `validate_semantic(&config) → Result<()>` — wallpaper mode table, empty strings, #RRGGBB format |
| `helpers.rs` | Empty — reserved for shared utilities |

**Flow:** `validate_file(path)?` → `validate_semantic(&config)?` → config is safe to use

**Tests:** 7 unit tests in `file.rs`, 12 unit tests in `semantic.rs`, 2 integration tests

---

## `lib.rs` state

- `pub mod config` — wired in
- `pub mod validate` — wired in
- `apply()` — still `todo!()` stub
- `validate()` — commented out (API surface being rethought)
- `list()` — still `todo!()` stub

---

## Fully documented

All public items in both modules have `///` doc comments with:
- Description and purpose
- `# Errors` sections listing specific variants
- `# Examples` with `rust,no_run` code blocks
- Field-level docs on struct fields and enum variant fields

Doc-tests: 10 compile-checked examples across both modules.
