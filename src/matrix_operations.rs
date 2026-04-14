/// Matrix Operations Module
/// Rust translation of Fortran matrix_operations.f90
/// Implements matrix multiplication, LU decomposition, and Gaussian elimination

pub struct MatrixOps;

impl MatrixOps {
    /// Matrix multiplication: C = A * B
    pub fn matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = a.len();
        let m = a[0].len();
        let p = b[0].len();

        let mut c = vec![vec![0.0; p]; n];

        for i in 0..n {
            for j in 0..p {
                for k in 0..m {
                    c[i][j] += a[i][k] * b[k][j];
                }
            }
        }

        c
    }

    /// LU Decomposition: A = L * U
    pub fn lu_decomposition(a: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let n = a.len();
        let mut l = vec![vec![0.0; n]; n];
        let mut u = vec![vec![0.0; n]; n];

        for i in 0..n {
            // Compute U row
            for j in i..n {
                let mut sum = 0.0;
                for k in 0..i {
                    sum += l[i][k] * u[k][j];
                }
                u[i][j] = a[i][j] - sum;
            }

            // Compute L column
            for j in i + 1..n {
                let mut sum = 0.0;
                for k in 0..i {
                    sum += l[j][k] * u[k][i];
                }

                if u[i][i].abs() > 1e-15 {
                    l[j][i] = (a[j][i] - sum) / u[i][i];
                } else {
                    l[j][i] = 0.0;
                }
            }
        }

        // Set diagonal of L to 1
        for i in 0..n {
            l[i][i] = 1.0;
        }

        (l, u)
    }

    /// Gaussian Elimination with partial pivoting
    pub fn gaussian_elimination(a_input: &[Vec<f64>], b_input: &[f64]) -> Vec<f64> {
        let n = a_input.len();
        let mut a = a_input.to_vec();
        let mut b = b_input.to_vec();

        // Forward elimination
        for k in 0..n - 1 {
            // Partial pivoting
            let mut pivot_row = k;
            for i in k + 1..n {
                if a[i][k].abs() > a[pivot_row][k].abs() {
                    pivot_row = i;
                }
            }

            // Swap rows if needed
            if pivot_row != k {
                for j in k..n {
                    let temp = a[k][j];
                    a[k][j] = a[pivot_row][j];
                    a[pivot_row][j] = temp;
                }
                let temp = b[k];
                b[k] = b[pivot_row];
                b[pivot_row] = temp;
            }

            // Eliminate below pivot
            for i in k + 1..n {
                if a[k][k].abs() > 1e-15 {
                    let factor = a[i][k] / a[k][k];
                    for j in k..n {
                        a[i][j] -= factor * a[k][j];
                    }
                    b[i] -= factor * b[k];
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            x[i] = b[i];
            for j in i + 1..n {
                x[i] -= a[i][j] * x[j];
            }
            if a[i][i].abs() > 1e-15 {
                x[i] /= a[i][i];
            }
        }

        x
    }

    pub fn print_matrix(a: &[Vec<f64>], name: &str) {
        println!("{}", name);
        for row in a {
            for val in row {
                print!("{:12.6} ", val);
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![vec![5.0, 3.0], vec![2.0, 4.0]];
        let c = MatrixOps::matrix_multiply(&a, &b);

        assert!((c[0][0] - 5.0).abs() < 1e-10);
        assert!((c[0][1] - 3.0).abs() < 1e-10);
        assert!((c[1][0] - 2.0).abs() < 1e-10);
        assert!((c[1][1] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_matrix_multiply_general() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let c = MatrixOps::matrix_multiply(&a, &b);

        // [1*5+2*7, 1*6+2*8] = [19, 22]
        // [3*5+4*7, 3*6+4*8] = [43, 50]
        assert!((c[0][0] - 19.0).abs() < 1e-10);
        assert!((c[0][1] - 22.0).abs() < 1e-10);
        assert!((c[1][0] - 43.0).abs() < 1e-10);
        assert!((c[1][1] - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_lu_decomposition() {
        let a = vec![
            vec![4.0, 3.0, 1.0],
            vec![3.0, 5.0, 2.0],
            vec![1.0, 2.0, 3.0],
        ];

        let (l, u) = MatrixOps::lu_decomposition(&a);

        // Verify L has 1s on diagonal
        for i in 0..3 {
            assert!((l[i][i] - 1.0).abs() < 1e-10);
        }

        // Verify L * U = A
        let lu = MatrixOps::matrix_multiply(&l, &u);
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (lu[i][j] - a[i][j]).abs() < 1e-8,
                    "LU[{},{}] = {} != A[{},{}] = {}",
                    i,
                    j,
                    lu[i][j],
                    i,
                    j,
                    a[i][j]
                );
            }
        }
    }

    #[test]
    fn test_gaussian_elimination_2x2() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![8.0, 13.0];
        let x = MatrixOps::gaussian_elimination(&a, &b);

        // Solution: 2x+y=8, x+3y=13 → x=2.2, y=3.6
        assert!((x[0] - 2.2).abs() < 1e-8);
        assert!((x[1] - 3.6).abs() < 1e-8);
    }

    #[test]
    fn test_gaussian_elimination_3x3() {
        let a = vec![
            vec![3.0, 2.0, -1.0],
            vec![2.0, -2.0, 4.0],
            vec![-1.0, 0.5, -1.0],
        ];
        let b = vec![1.0, -2.0, 0.0];
        let x = MatrixOps::gaussian_elimination(&a, &b);

        // Verify solution: A*x = b
        for i in 0..3 {
            let mut sum = 0.0;
            for j in 0..3 {
                sum += a[i][j] * x[j];
            }
            assert!(
                (sum - b[i]).abs() < 1e-8,
                "A*x[{}] = {} != b[{}] = {}",
                i,
                sum,
                i,
                b[i]
            );
        }
    }

    #[test]
    fn test_lu_preserves_determinant() {
        let a = vec![
            vec![4.0, 7.0],
            vec![2.0, 6.0],
        ];

        let (l, u) = MatrixOps::lu_decomposition(&a);

        // det(A) = det(L) * det(U)
        // det(L) = 1 (unit lower triangular)
        // det(U) = product of diagonal
        let det_u = u[0][0] * u[1][1];
        let det_a = a[0][0] * a[1][1] - a[0][1] * a[1][0];

        assert!((det_a - det_u).abs() < 1e-10);
    }

    #[test]
    fn test_gaussian_elimination_singular_avoidance() {
        // Test with potential pivot issues
        let a = vec![
            vec![0.001, 1.0],
            vec![1.0, 1.0],
        ];
        let b = vec![1.0, 2.0];

        let x = MatrixOps::gaussian_elimination(&a, &b);
        assert_eq!(x.len(), 2);
        // Should complete without panic
    }

    #[test]
    fn test_matrix_multiply_rectangular() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let b = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];
        let c = MatrixOps::matrix_multiply(&a, &b);

        // c[0][0] = 1*7 + 2*9 + 3*11 = 58
        // c[1][1] = 4*8 + 5*10 + 6*12 = 154
        assert!((c[0][0] - 58.0).abs() < 1e-10);
        assert!((c[1][1] - 154.0).abs() < 1e-10);
    }
}
