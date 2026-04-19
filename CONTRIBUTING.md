# Contributing to RustForge

RustForge is an open-source library for migrating legacy COBOL and Fortran systems to production-grade Rust, with guaranteed numerical parity. We welcome contributions of new migration modules, bug fixes, performance improvements, and documentation.

## Getting Started

### Prerequisites

- **Rust 1.75+** — Install from [rustup.rs](https://rustup.rs/)
- **GnuCOBOL 3.2+** (optional, for testing COBOL translations) — `brew install gnu-cobol` on macOS
- **gfortran** (optional, for testing Fortran translations) — `brew install gcc` on macOS (includes gfortran)
- **Cargo** — Included with Rust

### Setup

```bash
git clone https://github.com/mpuodziukas-labs/rustforge-legacy-modernization.git
cd rustforge
cargo build
cargo test
```

You should see **231 tests passing** on first run. If any fail, open an issue with `cargo test` output.

## Adding a New Migration Module

Follow this step-by-step guide to migrate a new COBOL or Fortran program:

### 1. Add the Legacy Source File

Create a new file in `cobol/` or `fortran/`:

```bash
# For COBOL programs:
touch cobol/your_program.cob

# For Fortran programs:
touch fortran/your_program.f90
```

Include the original program exactly as it exists in the legacy system. Add a header comment:

```cobol
      *> COBOL Program: your_program
      *> Original Author: [name/date]
      *> Purpose: [1-2 sentence description]
      *> Migrated to Rust: rustforge::your_module
```

### 2. Create the Rust Module

Create `src/your_module.rs` following this template:

```rust
/// Your Module
/// Rust translation of COBOL your_program.cob (or Fortran your_program.f90)
/// Guarantees numerical parity within 1e-10 of legacy system output

use crate::error::{MigrationError, validate_positive, verify_parity};

/// Public struct(s) for input/output
#[derive(Debug, Clone)]
pub struct YourParams {
    pub field1: f64,
    pub field2: u32,
}

/// Main calculation function — must return Result<T, MigrationError>
pub fn your_main_function(params: &YourParams) -> Result<YourResult, MigrationError> {
    validate_positive("field1", params.field1)?;

    // Your implementation here
    Ok(YourResult { /* ... */ })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        // At least 8 unit tests required
    }

    #[test]
    fn test_edge_case_zero() {
        // Test zero/boundary values
    }

    // More tests...
}
```

### 3. Declare the Module in lib.rs

Edit `src/lib.rs` and add:

```rust
pub mod your_module;
```

Ensure the module is listed **alphabetically** to maintain consistency.

### 4. Write Unit Tests

Within `src/your_module.rs`, add a `#[cfg(test)] mod tests { ... }` section with **minimum 8 tests**:

- **Basic case:** Normal, expected inputs
- **Edge case (zero):** Behavior with zero values
- **Edge case (boundary):** Maximum/minimum representable values
- **Negative inputs:** If applicable, test with negative amounts
- **Parity test:** Hardcoded legacy output vs. your Rust function
- **Error handling:** Invalid inputs should return `MigrationError`
- **Floating-point precision:** Test within expected tolerance (typically 1e-10 for f64)
- **Documentation example:** Your public function must include a `# Example` in its doc comment — write a test that matches it

Example:

```rust
#[test]
fn test_your_function_basic() {
    let params = YourParams { field1: 100.0, field2: 42 };
    let result = your_main_function(&params).expect("must succeed");
    assert!((result.output - 150.0).abs() < 1e-10);
}

#[test]
fn test_your_function_zero() {
    let params = YourParams { field1: 0.0, field2: 0 };
    let result = your_main_function(&params).expect("must succeed");
    assert_eq!(result.output, 0.0);
}

#[test]
fn test_your_function_parity_cobol() {
    // Run the actual COBOL program and capture output
    // Then verify Rust matches:
    let params = YourParams { field1: 155_250.0, field2: 1001 };
    let result = your_main_function(&params).expect("must succeed");
    let legacy_value = 180_542.50;
    assert!(verify_parity("result", legacy_value, result.output, 1e-10).is_ok());
}

#[test]
fn test_your_function_invalid_negative() {
    let params = YourParams { field1: -100.0, field2: 42 };
    assert!(your_main_function(&params).is_err());
}
```

Run tests with:

```bash
cargo test your_module
```

All tests must pass (green) before submitting a PR.

### 5. Add Integration Tests

Edit `tests/parity_tests.rs` and add tests that verify your module against legacy output:

```rust
#[test]
fn test_your_module_parity_vs_legacy() {
    use rustforge::your_module::{your_main_function, YourParams};
    use rustforge::error::verify_parity;

    // Run your_module with sample data
    let params = YourParams { field1: 100_000.0, field2: 1001 };
    let result = your_main_function(&params).unwrap();

    // Compare to legacy COBOL/Fortran output
    let legacy_output = 105_000.0; // From running original COBOL/Fortran
    assert!(verify_parity("your_output_field", legacy_output, result.output, 1e-10).is_ok());
}
```

### 6. Validate with Analyzer

Run the COBOL analyzer to document your migration:

```bash
cargo build --bin analyze
cargo run --bin analyze -- cobol/your_program.cob
```

Expected output:

```
Program: your_program
Lines of Code: 42
COBOL Constructs Found:
  - COMPUTE statements: 3
  - PERFORM loops: 2
  - FILE operations: 0
  - REDEFINES clauses: 0
```

Document this in your PR description.

### 7. Add Benchmark

Edit `benches/migration_benchmarks.rs` and add a criterion benchmark:

```rust
fn your_module_benchmark(c: &mut Criterion) {
    c.bench_function("your_module_compute", |b| {
        let params = YourParams { field1: 100_000.0, field2: 1001 };
        b.iter(|| your_main_function(&params))
    });
}
```

Run benchmarks:

```bash
cargo bench your_module
```

Results will be saved to `target/criterion/`. Update `benchmarks/results.md` with your results.

### 8. Full Test Suite

Before submitting your PR, ensure everything passes:

```bash
cargo test                                # All tests must pass
cargo clippy -- -D warnings               # No warnings
cargo build --release                     # Compiles in release mode
cargo doc --no-deps --open                # Docs generate correctly
```

## Parity Test Requirements

Every module migration **must** include parity tests proving numerical equivalence to the legacy system:

### Rules

1. **At least 1 parity test per module** — Compare Rust output to COBOL/Fortran within 1e-10 tolerance (or appropriate tolerance for the domain)
2. **Use `verify_parity()` helper** — Located in `crate::error::verify_parity()`:
   ```rust
   use rustforge::error::verify_parity;
   assert!(verify_parity("field_name", legacy_value, rust_value, 1e-10).is_ok());
   ```
3. **Test edge cases:**
   - Zero values
   - Maximum representable values
   - Negative inputs (if applicable)
   - Boundary conditions
4. **Document the legacy value source** — Include a comment explaining where the expected value came from:
   ```rust
   // Legacy output from COBOL run: print-statement at line 427
   let legacy_final_balance = 155_250.00;
   ```

### Example: Account Balance Parity

```rust
#[test]
fn test_account_balance_parity() {
    use rustforge::account_balance::process_account;
    use rustforge::error::verify_parity;

    // Legacy COBOL:
    // MOVE 100000 TO OPENING-BALANCE
    // MOVE 50000 TO TRANSACTION
    // MOVE 0.035 TO INTEREST-RATE
    // COMPUTE RUNNING-BALANCE = OPENING-BALANCE + TRANSACTION  => 150000
    // COMPUTE INTEREST-EARNED = RUNNING-BALANCE * INTEREST-RATE => 5250
    // COMPUTE FINAL-BALANCE = RUNNING-BALANCE + INTEREST-EARNED => 155250

    let (running_bal, interest, final_bal) =
        process_account(100_000.0, 50_000.0, 0.035);

    assert!(verify_parity("running_balance", 150_000.0, running_bal, 1e-10).is_ok());
    assert!(verify_parity("interest_earned", 5_250.0, interest, 1e-10).is_ok());
    assert!(verify_parity("final_balance", 155_250.0, final_bal, 1e-10).is_ok());
}
```

## Code Style

Follow Rust conventions and these RustForge-specific rules:

### Public API

- **No `unwrap()` in public functions** — Always return `Result<T, MigrationError>` for fallible operations
- **Validation functions** — Use `error::validate_positive()`, `error::validate_rate()`, or define your own validators
- **Doc comments** — All public functions/structs must have `///` documentation:
  ```rust
  /// Computes the final balance after applying interest.
  ///
  /// # Arguments
  /// * `opening_balance` - Starting balance in dollars
  /// * `transaction` - Debit (negative) or credit (positive)
  /// * `interest_rate` - Annual interest rate as decimal (e.g. 0.035 for 3.5%)
  ///
  /// # Returns
  /// Tuple: (running_balance, interest_earned, final_balance)
  ///
  /// # Example
  /// ```
  /// use rustforge::your_module::process_account;
  /// let (running, interest, final_bal) = process_account(100_000.0, 50_000.0, 0.035);
  /// assert!((final_bal - 155_250.0).abs() < 1e-10);
  /// ```
  pub fn process_account(opening: f64, transaction: f64, rate: f64) -> (f64, f64, f64) {
      // ...
  }
  ```

### Testing

- **Test module naming:** `#[cfg(test)] mod tests { ... }`
- **Test function naming:** `test_<function>_<scenario>`
- **At least 8 unit tests per module**
- **One documentation example test** that matches your public function's `# Example`

### Linting

```bash
cargo clippy -- -D warnings
```

All clippy warnings must be resolved. No `#[allow(...)]` without justification.

## Pull Request Checklist

Before submitting your PR, verify all of these pass:

- [ ] `cargo test` passes with 0 failures
- [ ] `cargo clippy -- -D warnings` produces no output
- [ ] All public functions have `///` doc comments with at least one `# Example` section
- [ ] Parity test included (at least 1 `verify_parity()` call with legacy baseline)
- [ ] Minimum 8 unit tests in module's `#[cfg(test)]` section
- [ ] Benchmark added to `benches/migration_benchmarks.rs`
- [ ] COBOL/Fortran source file included in `cobol/` or `fortran/` subdirectory
- [ ] Integration test added to `tests/parity_tests.rs`
- [ ] `benchmarks/results.md` updated with new module performance metrics
- [ ] Analyzer output documented in PR description: `cargo run --bin analyze -- cobol/your_program.cob`

## Example PR: Adding account_balance Module

```markdown
## Add account_balance Module

Migrates legacy COBOL account_balance.cob to Rust.

### Benchmarks
- COBOL (original): 2,340 ns/op
- Rust (rustforge): 15 ns/op
- **Speedup: ~156x**

### Analyzer Output
Program: account_balance
Lines of Code: 28
COBOL Constructs:
- COMPUTE statements: 3
- PERFORM loops: 0
- FILE operations: 0
- REDEFINES clauses: 0

### Tests
- 9 unit tests (edge cases, parity, doc examples)
- 1 integration test (vs COBOL legacy output)
- All passing

### Parity
Final balance matches COBOL output within 1e-10 tolerance across 20 test cases.
```

## Questions?

Open an issue or discussion in the GitHub repository. Tag it with `[question]` in the title.

## License

All contributions are under the same license as RustForge (typically MIT or Apache-2.0). By submitting a PR, you agree to license your contribution under the same terms.

---

**Welcome to the RustForge community!** 🦀
