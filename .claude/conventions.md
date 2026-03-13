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

