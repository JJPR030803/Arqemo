# Arqemo — Design Journal

Decisions recorded at the moment they are made.
Not documentation — a journal.

---

## 2026-03-13 — Workspace initialized

Rust workspace created with 6 crates:
- arqemo-cli: binary entry point, clap routing
- arqemo-core: orchestration, schema, validation, templates
- arqemo-ipc: Hyprland socket bridge (Phase 3)
- arqemo-renderer: wgpu world engine (Phase 3)
- arqemo-audio: sound lifecycle (Phase 5)
- arqemo-color: OKLCH author utilities (Phase 6)

Code quality pipeline: rustfmt + clippy pedantic + deny(unwrap_used).
Error handling: thiserror in libraries, anyhow at the surface.
Test strategy: unit tests in-file, integration tests in crates/*/tests/,
compositor tests marked #[ignore].
