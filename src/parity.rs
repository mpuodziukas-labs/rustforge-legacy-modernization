/// Parity testing utilities for verifying equivalence between legacy and Rust implementations
pub mod parity {
    /// Assert floating-point parity within tolerance
    pub fn assert_parity_f64(cobol_result: f64, rust_result: f64, tolerance: f64) -> bool {
        let diff = (cobol_result - rust_result).abs();
        diff <= tolerance
    }

    /// Assert vector parity with element-wise tolerance
    pub fn assert_parity_vec(cobol_vec: &[f64], rust_vec: &[f64], tolerance: f64) -> bool {
        if cobol_vec.len() != rust_vec.len() {
            return false;
        }

        for (cobol_elem, rust_elem) in cobol_vec.iter().zip(rust_vec.iter()) {
            if (cobol_elem - rust_elem).abs() > tolerance {
                return false;
            }
        }
        true
    }

    /// Assert matrix parity with element-wise tolerance
    pub fn assert_parity_matrix(
        cobol_matrix: &[Vec<f64>],
        rust_matrix: &[Vec<f64>],
        tolerance: f64,
    ) -> bool {
        if cobol_matrix.len() != rust_matrix.len() {
            return false;
        }

        for (cobol_row, rust_row) in cobol_matrix.iter().zip(rust_matrix.iter()) {
            if !assert_parity_vec(cobol_row, rust_row, tolerance) {
                return false;
            }
        }
        true
    }

    /// Pretty-print parity results
    pub fn print_parity_result(
        name: &str,
        legacy_val: f64,
        rust_val: f64,
        tolerance: f64,
        passed: bool,
    ) {
        let status = if passed { "PASS" } else { "FAIL" };
        let diff = (legacy_val - rust_val).abs();
        println!(
            "{}: {} | Legacy: {:.10} | Rust: {:.10} | Diff: {:.2e} | Tol: {:.2e}",
            name, status, legacy_val, rust_val, diff, tolerance
        );
    }
}
