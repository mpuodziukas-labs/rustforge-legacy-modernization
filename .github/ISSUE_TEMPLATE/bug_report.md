---
name: Bug Report
about: Parity violation or incorrect migration output
labels: bug
---

## Module affected
(e.g., account_balance, eigenvalue_solver, loan_calculator)

## Legacy value
What did the original COBOL/Fortran program produce?

## Rust value
What did rustforge produce instead?

## Difference
`legacy - rust = ?` (what's the actual discrepancy?)

## Input that triggered it
```
(paste the input values)
```

## Steps to reproduce
1. 
2. 
3. 

## Expected behavior
Rust output should match legacy within 1e-10 tolerance (or appropriate domain tolerance).

## Actual behavior
(What actually happened?)

## cargo test output
```
(paste the full failing test output, including module name)
```

## Environment
- Rust version: `rustc --version`
- RustForge version or commit hash: 
- OS: macOS / Linux / Windows

## Additional context
Add any other context about the problem here.
