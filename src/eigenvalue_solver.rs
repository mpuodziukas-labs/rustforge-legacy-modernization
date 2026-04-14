//! Eigenvalue Solver Module
//! Rust translation of Fortran eigenvalue_solver.f90
//! Implements power iteration method for computing dominant eigenvalue

pub struct PowerIterationSolver {
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl PowerIterationSolver {
    /// Creates a new solver with the specified iteration limit and convergence tolerance.
    ///
    /// # Arguments
    /// * `max_iterations` - Maximum number of power-iteration steps before giving up
    /// * `tolerance`      - Convergence threshold: iteration stops when the change in
    ///   the Rayleigh-quotient eigenvalue estimate falls below this value
    ///
    /// # Example
    /// ```
    /// use rustforge::eigenvalue_solver::PowerIterationSolver;
    /// let solver = PowerIterationSolver::new(100, 1e-10);
    /// ```
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        PowerIterationSolver {
            max_iterations,
            tolerance,
        }
    }

    /// Returns the canonical 5×5 symmetric positive-definite test matrix used
    /// throughout the benchmark and test suite.
    ///
    /// The matrix has a known dominant eigenvalue of approximately 5.6, which makes
    /// it a convenient, reproducible fixture for validating convergence.  For `n < 5`
    /// the function returns an `n×n` zero matrix.
    ///
    /// # Arguments
    /// * `n` - Desired matrix dimension; the hardcoded values are only applied when `n >= 5`
    ///
    /// # Returns
    /// An `n×n` matrix represented as `Vec<Vec<f64>>`.
    pub fn initialize_matrix(n: usize) -> Vec<Vec<f64>> {
        let mut matrix = vec![vec![0.0; n]; n];

        // Create test matrix:
        // [5.0, 1.0, 0.5, 0.2, 0.1]
        // [1.0, 4.0, 0.8, 0.3, 0.2]
        // [0.5, 0.8, 3.0, 0.6, 0.4]
        // [0.2, 0.3, 0.6, 2.0, 0.5]
        // [0.1, 0.2, 0.4, 0.5, 1.0]

        if n >= 5 {
            matrix[0] = vec![5.0, 1.0, 0.5, 0.2, 0.1];
            matrix[1] = vec![1.0, 4.0, 0.8, 0.3, 0.2];
            matrix[2] = vec![0.5, 0.8, 3.0, 0.6, 0.4];
            matrix[3] = vec![0.2, 0.3, 0.6, 2.0, 0.5];
            matrix[4] = vec![0.1, 0.2, 0.4, 0.5, 1.0];
        }

        matrix
    }

    /// Computes the matrix-vector product `y = A × x`.
    ///
    /// # Arguments
    /// * `matrix` - An `n×n` matrix represented as `Vec<Vec<f64>>`
    /// * `x`      - Input vector of length `n`
    ///
    /// # Returns
    /// Output vector `y` of length `n` where `y[i] = Σ_j matrix[i][j] * x[j]`.
    pub fn matvec(&self, matrix: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
        let n = matrix.len();
        let mut y = vec![0.0; n];

        for i in 0..n {
            for j in 0..n {
                y[i] += matrix[i][j] * x[j];
            }
        }

        y
    }

    /// Computes the Euclidean (L2) norm of a vector: `‖v‖ = √(Σ vᵢ²)`.
    ///
    /// # Arguments
    /// * `v` - Input vector of any length
    ///
    /// # Returns
    /// Non-negative scalar ≥ 0.
    ///
    /// # Example
    /// ```
    /// use rustforge::eigenvalue_solver::PowerIterationSolver;
    /// let solver = PowerIterationSolver::new(1, 1e-10);
    /// assert!((solver.norm(&[3.0, 4.0]) - 5.0).abs() < 1e-10);
    /// ```
    pub fn norm(&self, v: &[f64]) -> f64 {
        v.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// Rayleigh quotient for eigenvalue estimation
    fn rayleigh_quotient(&self, v: &[f64], av: &[f64]) -> f64 {
        v.iter()
            .zip(av.iter())
            .map(|(vi, avi)| vi * avi)
            .sum()
    }

    /// Finds the dominant eigenvalue and its eigenvector using the power iteration method.
    ///
    /// ## Algorithm
    /// 1. Start with a uniform unit vector `v₀`.
    /// 2. Repeatedly compute `v_{k+1} = A × vₖ` and normalise to unit length.
    /// 3. Estimate the eigenvalue via the Rayleigh quotient: `λ = vᵀ A v`.
    /// 4. Stop when `|λₖ − λₖ₋₁| < tolerance` or `max_iterations` is reached.
    ///
    /// Power iteration converges to the *dominant* (largest-magnitude) eigenvalue.
    /// The convergence rate depends on the ratio `|λ₁/λ₂|`; a larger ratio means
    /// faster convergence.
    ///
    /// # Arguments
    /// * `matrix` - Square `n×n` matrix as `&[Vec<f64>]`
    ///
    /// # Returns
    /// `(eigenvalue, eigenvector)` — the dominant eigenvalue and its corresponding
    /// unit eigenvector.
    pub fn solve(&self, matrix: &[Vec<f64>]) -> (f64, Vec<f64>) {
        let n = matrix.len();

        // Initialize eigenvector
        let mut v = vec![1.0 / (n as f64).sqrt(); n];
        let mut lambda_old = 0.0;
        let mut lambda = 0.0;

        for _iter in 0..self.max_iterations {
            // Matrix-vector multiplication: v_new = A * v
            let v_new_unnormalized = self.matvec(matrix, &v);

            // Compute Euclidean norm
            let norm_val = self.norm(&v_new_unnormalized);

            // Normalize
            let v_new: Vec<f64> = v_new_unnormalized.iter().map(|x| x / norm_val).collect();

            // Rayleigh quotient for eigenvalue
            let av = self.matvec(matrix, &v_new);
            lambda = self.rayleigh_quotient(&v_new, &av);

            // Check convergence
            let error = (lambda - lambda_old).abs();
            if error < self.tolerance {
                return (lambda, v_new);
            }

            lambda_old = lambda;
            v = v_new;
        }

        (lambda, v)
    }

    /// Prints the dominant eigenvalue and eigenvector components to stdout.
    ///
    /// # Arguments
    /// * `eigenvalue`  - The dominant eigenvalue returned by [`solve`](Self::solve)
    /// * `eigenvector` - The corresponding unit eigenvector
    pub fn print_results(&self, eigenvalue: f64, eigenvector: &[f64]) {
        println!("=====================================");
        println!("EIGENVALUE SOLVER RESULTS");
        println!("=====================================");
        println!("Dominant Eigenvalue: {:.10}", eigenvalue);
        println!("Eigenvector:");
        for (i, val) in eigenvector.iter().enumerate() {
            println!("{}: {:.10}", i + 1, val);
        }
        println!("=====================================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_matrix() {
        let matrix = PowerIterationSolver::initialize_matrix(5);
        assert_eq!(matrix.len(), 5);
        assert_eq!(matrix[0].len(), 5);
        assert!((matrix[0][0] - 5.0).abs() < 1e-10);
        assert!((matrix[1][1] - 4.0).abs() < 1e-10);
        assert!((matrix[2][2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_matvec_multiplication() {
        let solver = PowerIterationSolver::new(100, 1e-10);
        let matrix = PowerIterationSolver::initialize_matrix(5);
        let x = vec![1.0; 5];
        let result = solver.matvec(&matrix, &x);

        assert_eq!(result.len(), 5);
        // First element: 5*1 + 1*1 + 0.5*1 + 0.2*1 + 0.1*1 = 6.8
        assert!((result[0] - 6.8).abs() < 1e-10);
    }

    #[test]
    fn test_norm_calculation() {
        let solver = PowerIterationSolver::new(100, 1e-10);
        let v = vec![3.0, 4.0];
        let norm = solver.norm(&v);
        assert!((norm - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_eigenvalue_solver_convergence() {
        let solver = PowerIterationSolver::new(100, 1e-10);
        let matrix = PowerIterationSolver::initialize_matrix(5);
        let (eigenvalue, eigenvector) = solver.solve(&matrix);

        // Dominant eigenvalue should be approximately 5.6 for this matrix
        assert!(eigenvalue > 5.0);
        assert!(eigenvalue < 6.0);
        assert_eq!(eigenvector.len(), 5);
    }

    #[test]
    fn test_eigenvalue_eigenvector_relationship() {
        let solver = PowerIterationSolver::new(100, 1e-10);
        let matrix = PowerIterationSolver::initialize_matrix(5);
        let (eigenvalue, eigenvector) = solver.solve(&matrix);

        // Verify A*v ≈ λ*v
        let av = solver.matvec(&matrix, &eigenvector);
        for (i, (avi, vi)) in av.iter().zip(eigenvector.iter()).enumerate() {
            let expected = eigenvalue * vi;
            assert!(
                (avi - expected).abs() < 1e-4,
                "Mismatch at index {}: {} vs {}",
                i,
                avi,
                expected
            );
        }
    }

    #[test]
    fn test_eigenvector_normalization() {
        let solver = PowerIterationSolver::new(100, 1e-10);
        let matrix = PowerIterationSolver::initialize_matrix(5);
        let (_eigenvalue, eigenvector) = solver.solve(&matrix);

        // Eigenvector should be approximately unit length
        let norm = solver.norm(&eigenvector);
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_convergence_tolerance() {
        let solver_strict = PowerIterationSolver::new(1000, 1e-12);
        let solver_loose = PowerIterationSolver::new(1000, 1e-6);

        let matrix = PowerIterationSolver::initialize_matrix(5);

        let (eig_strict, _vec_strict) = solver_strict.solve(&matrix);
        let (eig_loose, _vec_loose) = solver_loose.solve(&matrix);

        // Both should converge to approximately the same value
        assert!((eig_strict - eig_loose).abs() < 1e-4);
    }
}
