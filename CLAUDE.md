# rustforge — CLAUDE.md
<!-- VERIFIED: 2026-04-14 | WATCH: ~/Desktop/rustforge-legacy-modernization/Cargo.toml,~/Desktop/rustforge-legacy-modernization/src/lib.rs -->

## Identity
TYPE:    cli + library
STACK:   Rust 2021 / nalgebra 0.33 / ndarray 0.15 / clap 4 (derive) / criterion 0.5
SCOPE:   COBOL/Fortran → Rust migration engine. Provides analysis, batch processing, financial calculations, matrix ops. Not a web app. Not an API server.
DEPLOY:  ~/.cargo/bin/cargo build --release → target/release/rustforge
LIVE:    mpuodziukas-labs/rustforge-legacy-modernization (GitHub)

## Commands (cargo path: ~/.cargo/bin/cargo)
DEV:     ~/.cargo/bin/cargo run
BUILD:   ~/.cargo/bin/cargo build --release
TEST:    ~/.cargo/bin/cargo test
BENCH:   ~/.cargo/bin/cargo bench
LINT:    ~/.cargo/bin/cargo clippy -- -D warnings
DEPLOY:  git push origin main (CI handles release)

## File Map
ENTRY (lib):  src/lib.rs        ← public API surface, module declarations
ENTRY (bin):  src/main.rs       ← CLI entry, clap arg parsing
CONFIG:       Cargo.toml
STYLES:       N/A
TYPES:        N/A (types inline per module)
ENV:          N/A
DB:           N/A
BINARIES:     src/bin/analyze.rs | src/bin/benchmark.rs

## Key Modules (read before touching related logic)
- src/account_balance.rs     — financial account operations
- src/batch_processor.rs     — batch job processing
- src/eigenvalue_solver.rs   — matrix eigenvalue computation (nalgebra)
- src/matrix_operations.rs   — matrix math (ndarray + nalgebra)
- src/parity.rs              — parity checking
- src/report_generator.rs    — output formatting
- src/cobol_analyzer.rs      — COBOL source analysis
- src/gl_validator.rs        — general ledger validation
- src/payroll.rs             — payroll calculations
- src/loan_calculator.rs     — loan amortization

## Project Rules
- Cargo.toml has package-wide clippy lints: empty_line_after_doc_comments=allow, dead_code=allow — do NOT override per-module.
- No new dependencies without benchmarking allocation impact first.
- All public functions in lib.rs must have doc comments (/// style).
- Financial calculations: use integer arithmetic or fixed-point — never f64 for money values.
- bench/ outputs go to target/criterion — do NOT commit benchmark HTML reports.

## Landmines
- 2026-04-14: cargo not in PATH by default (macOS zsh). Use full path ~/.cargo/bin/cargo for all commands.
- benchmark.rs has an intentionally unread field — dead_code=allow is required at package level. Do NOT add #[allow(dead_code)] per-module to "fix" this warning.
- Lints are set in [lints.*] in Cargo.toml — changing them affects all modules globally.

## Context Anchors
- Cargo.toml: all dependencies + lint config. Read before adding any crate.
- src/lib.rs: module tree. Read before adding a new module — must declare it here.
- src/main.rs: CLI argument structure. Read before any new subcommand.
- benchmarks/results.md: benchmark baseline. Update after any perf change.
