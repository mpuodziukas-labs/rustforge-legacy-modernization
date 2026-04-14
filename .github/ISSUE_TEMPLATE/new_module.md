---
name: New Migration Module Request
about: Request a new COBOL/Fortran→Rust migration module
labels: enhancement
---

## Legacy language
- [ ] COBOL
- [ ] Fortran
- [ ] Other (please specify):

## Program name or type
(e.g., "VSAM file processor", "FFT calculator", "payroll batch processor")

## Why it matters
(What business problem does this module solve? Why is it a priority?)

## Estimated lines of code
(How many lines is the legacy program?)

## Do you have the COBOL/Fortran source?
- [ ] Yes, I can provide it
- [ ] No, help me locate it
- [ ] Under NDA, restricted access

## Key COBOL/Fortran constructs used
(e.g., REDEFINES, COMP-3, OCCURS, FILE SECTION, GO TO, CALL, nested loops, matrix operations)

- [ ] COBOL-specific
- [ ] Fortran-specific
- [ ] Numeric/math-heavy
- [ ] File I/O intensive
- [ ] String manipulation
- [ ] Other:

## Expected Rust speedup
(Rough estimate based on similar modules, or "unknown")

## Operational constraints
(e.g., must maintain byte-for-byte compatibility, must run in legacy batch window, must support legacy data formats)

## References
Links to documentation, related code repositories, or technical specs (if public).

## Acceptance criteria
What needs to be true for this module to be considered complete?
- [ ] Parity test passes (Rust matches legacy within 1e-10)
- [ ] All edge cases covered (zero, negative, boundary values)
- [ ] 8+ unit tests
- [ ] Integration test in `tests/parity_tests.rs`
- [ ] Benchmark added (with legacy baseline for comparison)
- [ ] Documentation with examples
- [ ] Zero unsafe code (unless explicitly justified)
