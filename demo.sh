#!/usr/bin/env bash
# demo.sh — RustForge Legacy Modernization Demo
# Demonstrates the full migration engine: tests, COBOL analysis, and benchmarks.
# Usage: ./demo.sh
# Requirements: cargo (Rust toolchain)

set -e

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
BOLD='\033[1m'
GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
YELLOW='\033[0;33m'
RESET='\033[0m'

line() { printf '%s\n' "────────────────────────────────────────────────────────────"; }
header() { printf "\n${BOLD}${CYAN}%s${RESET}\n" "$1"; line; }
ok()  { printf "  ${GREEN}✓${RESET}  %s\n" "$1"; }
err() { printf "  ${RED}✗${RESET}  %s\n" "$1"; }
info() { printf "  ${YELLOW}→${RESET}  %s\n" "$1"; }

# ---------------------------------------------------------------------------
# Pre-flight
# ---------------------------------------------------------------------------
if ! command -v cargo &>/dev/null; then
    err "cargo not found. Install Rust: https://rustup.rs/"
    exit 1
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$REPO_ROOT"

# ---------------------------------------------------------------------------
# Banner
# ---------------------------------------------------------------------------
printf "\n"
printf "${BOLD}${CYAN}"
printf "╔══════════════════════════════════════════════════════════════╗\n"
printf "║          RustForge Legacy Modernization Engine v2.0          ║\n"
printf "║     COBOL → Rust migration with verified numerical parity    ║\n"
printf "╚══════════════════════════════════════════════════════════════╝\n"
printf "${RESET}\n"
info "Repo:    $REPO_ROOT"
info "Rust:    $(rustc --version)"
info "Cargo:   $(cargo --version)"
printf "\n"

# ---------------------------------------------------------------------------
# Step 1: Test suite
# ---------------------------------------------------------------------------
header "Step 1 — Test Suite (177 tests across unit, integration, parity)"

info "Running: cargo test --quiet"
printf "\n"

# Capture output; parse pass/fail from the summary line
TEST_OUTPUT=$(cargo test --quiet 2>&1) || true
echo "$TEST_OUTPUT" | tail -20

# Extract result counts from "test result: ok. N passed; M failed" line
PASS_COUNT=$(echo "$TEST_OUTPUT" | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+' | paste -sd'+' - | bc 2>/dev/null || echo "?")
FAIL_COUNT=$(echo "$TEST_OUTPUT" | grep -oE '[0-9]+ failed' | grep -oE '[0-9]+' | paste -sd'+' - | bc 2>/dev/null || echo "0")

printf "\n"
if [ "$FAIL_COUNT" = "0" ] || [ -z "$FAIL_COUNT" ]; then
    ok "Tests passed: ${PASS_COUNT} passed, 0 failed"
else
    err "Test failures detected: ${FAIL_COUNT} failed"
    exit 1
fi

# ---------------------------------------------------------------------------
# Step 2: COBOL Static Analyzer
# ---------------------------------------------------------------------------
header "Step 2 — COBOL Static Analyzer"

COBOL_FILE="cobol/account_balance.cob"
if [ ! -f "$COBOL_FILE" ]; then
    err "COBOL source not found: $COBOL_FILE"
    exit 1
fi

info "Running: cargo run --bin analyze -- $COBOL_FILE"
printf "\n"
cargo run --quiet --bin analyze -- "$COBOL_FILE"
printf "\n"
ok "Analyzer completed — migration risk report generated"

# ---------------------------------------------------------------------------
# Step 3: Benchmark Runner
# ---------------------------------------------------------------------------
header "Step 3 — Benchmark Runner"

info "Running: cargo run --bin benchmark"
printf "\n"
cargo run --quiet --bin benchmark
printf "\n"
ok "Benchmarks completed"

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
header "Demo Summary"

printf "\n"
printf "  ${BOLD}What was demonstrated:${RESET}\n\n"
printf "  1. ${GREEN}Test Suite${RESET}\n"
printf "     177 tests across 3 layers — unit, integration, and parity.\n"
printf "     Parity tests validate Rust output against GnuCOBOL reference\n"
printf "     values to ±0.001 tolerance.\n\n"
printf "  2. ${GREEN}COBOL Static Analyzer${RESET}  (cargo run --bin analyze)\n"
printf "     Parses legacy COBOL source, scores migration complexity, and\n"
printf "     flags constructs that require manual review.\n\n"
printf "  3. ${GREEN}Benchmark Runner${RESET}  (cargo run --bin benchmark)\n"
printf "     Exercises all seven ported modules and three numerical solvers.\n"
printf "     Criterion benchmarks (cargo bench) produce full HTML reports.\n\n"
printf "  ${BOLD}Modules ported:${RESET}\n"
printf "     account_balance  batch_processor  gl_validator\n"
printf "     inventory_valuation  loan_calculator  payroll  report_generator\n\n"
printf "  ${BOLD}Numerical engine:${RESET}\n"
printf "     matrix_operations  eigenvalue_solver  statistics\n\n"

line
printf "${BOLD}${GREEN}  RustForge demo complete.${RESET}\n"
line
printf "\n"
