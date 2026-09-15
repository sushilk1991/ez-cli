# ez

Single-crate clap binary. Command catalog: `cargo run -- --help`.

## Commands

```
cargo build
cargo test
cargo clippy --all-targets
cargo run -- --help
cargo run -- list --json
cargo run -- schema list --json
```

`cargo clippy --all-targets` exits 0 with existing warnings. There is no `tests/` tree and no `#[cfg(test)]` yet; `cargo test` is still the suite (currently 0 tests). `[dev-dependencies]` already has `assert_cmd`, `tempfile`, `predicates`.

## Done

All three must exit 0:

```
cargo build
cargo test
cargo clippy --all-targets
```

`cargo clippy --all-targets -- -D warnings` and `cargo fmt --all --check` fail on the current tree. They are not gates.

## New command

Wire every new subcommand in all four places:

1. `src/commands/<name>.rs` — `pub fn execute(...) -> Result<CommandOutput, EzError>`
2. `src/commands/mod.rs` — `pub mod <name>;`
3. `src/main.rs` — clap variant on `Commands`, `command_name` match, dispatch match
4. `src/commands/schema.rs` — entry in `build_registry()`

Copy `src/commands/copy.rs` and the `Copy` arms in `src/main.rs`. `chain` is the exception (`chain::run`).

JSON: print human text only when `!ctx.json`. Return `CommandOutput::new("name", data)`. `output_result` in `src/output.rs` writes the `--json` envelope. Fail with `EzError` (exit 1–5); do not `std::process::exit` in a command.

Globals `--json --yes --dry-run` become `CommandContext`. Confirmations: `ctx.should_confirm()`. Preview: `ctx.dry_run`. Copy `src/commands/remove.rs`.

Piped paths: only the `src/main.rs` arms that already refill an empty `Vec<PathBuf>` via `utils::read_paths_from_stdin()`.

## Guardrails

- Leave `Cargo.lock` untracked (`.gitignore`); do not `git add` it.
- Format only files you already change; do not run a crate-wide `cargo fmt --all`.
- Keep `cargo clippy --all-targets` at exit 0; do not add `#[allow(...)]` to hide a new lint, and do not sweep existing warnings.
- Return `CommandOutput`; do not `println!` JSON from a command (`schema` pretty-prints only when `--json` is off).
- Update `src/main.rs` and `src/commands/schema.rs` when adding a subcommand.

## Maintaining this file

Keep this file for knowledge useful to almost every future agent session in this project.
Do not repeat what the codebase already shows; point to the authoritative file or command instead.
Prefer rewriting or pruning existing entries over appending new ones.
When updating this file, preserve this bar for all agents and keep entries concise.
