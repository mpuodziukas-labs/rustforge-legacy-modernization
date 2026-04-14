---
name: Performance Issue
about: Rust implementation slower than documented baseline
labels: performance
---

## Module
(e.g., account_balance, loan_calculator)

## Expected speedup
(From README or benchmarks/results.md)

## Actual speedup measured
```
(paste relevant rows from benchmark output)
```

## How you measured it
```bash
cargo bench --bench <name> -- --verbose
```

## Hardware
- CPU: (e.g., M1 Max, Intel Xeon, AMD EPYC)
- RAM: (e.g., 16GB, 64GB)
- OS: macOS 14.2 / Ubuntu 22.04 / other
- Rust version: (rustc --version)
- Cargo.toml dependencies: (any custom features enabled?)

## Full benchmark output
```
(paste criterion output or cargo bench output)
```

## Profiling results
(Optional: output from `perf`, `flamegraph`, or other profiling tools)

```
(profiling output)
```

## Suspected root cause
(Optional: do you have a hypothesis about what's slow?)

## Related issues or PRs
(Any other related performance issues or PRs?)

## Impact
- [ ] Critical (production deployment blocked)
- [ ] High (significant overhead, needs fix before merge)
- [ ] Medium (nice to have optimization)
- [ ] Low (minor overhead, acceptable tradeoff)
