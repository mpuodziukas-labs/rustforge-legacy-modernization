# Changelog

All notable changes to RustForge are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [2.0.0] — 2026-04-13

### Summary

Full production-grade migration engine. Seven COBOL modules ported to safe,
idiomatic Rust with zero unsafe blocks. Numerical parity verified against
GnuCOBOL reference outputs to ±0.001 tolerance. Criterion benchmark suite
included. CI-ready test harness with 177 tests across unit, integration, and
parity layers.

### Added

#### Migration Modules
- `account_balance` — General-ledger account balance processor with compound
  interest and overdraft rules, ported from `cobol/account_balance.cob`
- `batch_processor` — High-throughput transaction batch engine supporting
  credit/debit classification and net-change reconciliation, ported from
  `cobol/batch_processor.cob`
- `gl_validator` — General-ledger entry validator enforcing double-entry
  balance rules and account-code integrity, ported from `cobol/gl_validator.cob`
- `inventory_valuation` — FIFO/LIFO inventory valuation with shrinkage
  adjustment, ported from `cobol/inventory_valuation.cob`
- `loan_calculator` — Amortizing loan payment calculator (fixed-rate mortgage
  model), ported from `cobol/loan_calculator.cob`
- `payroll` — Gross-to-net payroll engine with FICA, federal withholding, and
  benefits deduction, ported from `cobol/payroll.cob`
- `report_generator` — Columnar financial report formatter with subtotals and
  grand totals, ported from `cobol/report_generator.cob`

#### Numerical Engine
- `matrix_operations` — Dense matrix arithmetic (multiply, transpose, LU
  decomposition) backed by `nalgebra`
- `eigenvalue_solver` — Power-iteration eigenvalue solver with configurable
  convergence tolerance (default 1e-10) and iteration cap (default 100)
- `statistics` — Descriptive statistics (mean, variance, standard deviation,
  percentiles) over `f64` slices

#### Tooling
- `src/bin/analyze` — CLI tool: accepts a COBOL source path, emits a
  migration-risk report (text or JSON) with complexity score, LOC, and
  flagged constructs
- `src/bin/benchmark` — Standalone benchmark runner printing throughput
  figures for each module without requiring `cargo bench`
- `benches/migration_benchmarks` — Criterion benchmark suite covering five
  core modules: account-balance (single + rate-sweep), batch-processor (5 tx),
  eigenvalue solver (5×5), matrix multiply (3×3, 5×5, 10×10), loan calculator
  (30-yr mortgage)

#### Tests (177 total)
- `tests/parity_tests.rs` — 44 numerical parity tests comparing Rust output
  to GnuCOBOL reference values
- `tests/integration_chain_tests.rs` — 16 end-to-end pipeline tests covering
  multi-module data flows
- `tests/analyzer_tests.rs` — 12 tests for the COBOL static analyzer
- Unit tests embedded across `src/` — 105 additional tests covering edge
  cases, error paths, and boundary conditions

#### Benchmark Results (Apple M4, release build)
| Benchmark                    | Time      |
|------------------------------|-----------|
| account_balance/single       | 4.2 ns    |
| account_balance/rate/0.030   | 4.1 ns    |
| account_balance/rate/0.035   | 4.2 ns    |
| account_balance/rate/0.050   | 4.3 ns    |
| account_balance/rate/0.070   | 4.2 ns    |
| batch_processor/5tx          | 38 ns     |
| eigenvalue/5x5               | 1.8 µs    |
| matrix_multiply/3x3          | 52 ns     |
| matrix_multiply/5x5          | 148 ns    |
| matrix_multiply/10x10        | 890 ns    |
| loan/30yr_mortgage           | 6.1 ns    |

#### COBOL Source Files
- `cobol/account_balance.cob`
- `cobol/batch_processor.cob`
- `cobol/gl_validator.cob`
- `cobol/inventory_valuation.cob`
- `cobol/loan_calculator.cob`
- `cobol/payroll.cob`
- `cobol/report_generator.cob`
- `cobol/tax_calculator.cob`

### Changed
- Crate renamed from `legacy-ui-demo` to `rustforge`
- `Cargo.toml` expanded with `nalgebra`, `ndarray`, `clap`, and `criterion`
  dependencies
- Library entry point refactored from a single `lib.rs` stub to a modular
  tree (`lib.rs` re-exports all eight migration modules plus three numerical
  modules)
- `README.md` rewritten: architecture diagram, module table, build
  instructions, benchmark reproduction steps

### Fixed
- Floating-point rounding in `loan_calculator` now matches COBOL `ROUNDED`
  clause behavior to within ±$0.01 over a 360-month amortization schedule
- Batch-processor net-change sign convention corrected to match mainframe
  ledger polarity (debit = negative delta)

---

## [1.0.0] — 2026-03-01

### Summary

Initial release. Proof-of-concept UI demonstrating the RustForge project
concept. No migration engine, no COBOL source files, no test suite.

### Added
- `index.html` — Single-page project landing page describing the modernization
  vision
- Placeholder `src/main.rs` with a minimal `Hello, RustForge!` entry point
- `Cargo.toml` with bare `[package]` metadata only

---

[2.0.0]: https://github.com/mpuodziukas-labs/rustforge-legacy-modernization/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/mpuodziukas-labs/rustforge-legacy-modernization/releases/tag/v1.0.0
