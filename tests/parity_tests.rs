/// Parity Tests
/// Integration tests verifying output equivalence between legacy and Rust implementations

#[cfg(test)]
mod parity_tests {
    use rustforge::account_balance::{
        apply_interest, calculate_balance, compute_final_balance, process_account,
    };
    use rustforge::batch_processor::{BatchSummary, Transaction};
    use rustforge::eigenvalue_solver::PowerIterationSolver;
    use rustforge::matrix_operations::MatrixOps;
    use rustforge::parity::{assert_parity_f64, assert_parity_vec};

    // MOD-001: Account Balance Parity Tests

    #[test]
    fn test_account_balance_parity_single_transaction() {
        // COBOL: Opening balance 100,000 + transaction 50,000 = 150,000
        let opening = 100000.0;
        let transaction = 50000.0;
        let running = calculate_balance(opening, transaction);

        let cobol_running = 150000.0;
        assert!(assert_parity_f64(cobol_running, running, 1e-10));
    }

    #[test]
    fn test_account_balance_parity_with_interest() {
        // COBOL: Interest = 150,000 * 0.035 = 5,250
        let running = 150000.0;
        let rate = 0.035;
        let interest = apply_interest(running, rate);

        let cobol_interest = 5250.0;
        assert!(assert_parity_f64(cobol_interest, interest, 1e-10));
    }

    #[test]
    fn test_account_balance_parity_final_balance() {
        // COBOL: Final = 150,000 + 5,250 = 155,250
        let running = 150000.0;
        let interest = 5250.0;
        let final_balance = compute_final_balance(running, interest);

        let cobol_final = 155250.0;
        assert!(assert_parity_f64(cobol_final, final_balance, 1e-10));
    }

    #[test]
    fn test_account_balance_parity_complete_workflow() {
        // COBOL process_account: opening=100K, trans=50K, rate=3.5%
        let (running, interest, final_balance) = process_account(100000.0, 50000.0, 0.035);

        let cobol_running = 150000.0;
        let cobol_interest = 5250.0;
        let cobol_final = 155250.0;

        assert!(assert_parity_f64(cobol_running, running, 1e-10));
        assert!(assert_parity_f64(cobol_interest, interest, 1e-10));
        assert!(assert_parity_f64(cobol_final, final_balance, 1e-10));
    }

    #[test]
    fn test_account_balance_parity_multiple_cascading() {
        // COBOL: Three sequential transactions
        let (r1, i1, f1) = process_account(100000.0, 25000.0, 0.035);
        let (r2, i2, f2) = process_account(f1, 25000.0, 0.035);
        let (r3, i3, f3) = process_account(f2, 0.0, 0.035);

        // Verify cascade through interest compounding
        assert!(f3 > f1); // Each step adds interest
        assert!(f2 > f1);

        // Expected final: ((125000 * 1.035) + 25000) * 1.035 * 1.035 = 165370.359375
        let expected = 165370.359375;
        assert!(assert_parity_f64(expected, f3, 1e-6));
    }

    // MOD-002: Batch Processor Parity Tests

    #[test]
    fn test_batch_processor_parity_single_credit() {
        let mut summary = BatchSummary::new();
        let trans = Transaction::new(12345001, 50000.0, 'C');
        summary.add_transaction(&trans);

        let cobol_credits = 50000.0;
        assert!(assert_parity_f64(cobol_credits, summary.total_credits, 1e-10));
        assert!(assert_parity_f64(0.0, summary.total_debits, 1e-10));
    }

    #[test]
    fn test_batch_processor_parity_single_debit() {
        let mut summary = BatchSummary::new();
        let trans = Transaction::new(12345002, 25000.0, 'D');
        summary.add_transaction(&trans);

        let cobol_debits = 25000.0;
        assert!(assert_parity_f64(cobol_debits, summary.total_debits, 1e-10));
        assert!(assert_parity_f64(0.0, summary.total_credits, 1e-10));
    }

    #[test]
    fn test_batch_processor_parity_five_transactions() {
        // COBOL batch: 5 transactions (3 credits, 2 debits)
        let mut summary = BatchSummary::new();
        let transactions = vec![
            Transaction::new(12345001, 100000.0, 'C'),
            Transaction::new(12345002, 75000.0, 'C'),
            Transaction::new(12345003, 100000.0, 'C'),
            Transaction::new(12345004, 50000.0, 'D'),
            Transaction::new(12345005, 75000.0, 'D'),
        ];

        summary.process_transactions(&transactions);

        // COBOL totals
        let cobol_credits = 275000.0;
        let cobol_debits = 125000.0;
        let cobol_net = 150000.0;

        assert!(assert_parity_f64(cobol_credits, summary.total_credits, 1e-10));
        assert!(assert_parity_f64(cobol_debits, summary.total_debits, 1e-10));
        assert!(assert_parity_f64(cobol_net, summary.net_change, 1e-10));
    }

    #[test]
    fn test_batch_processor_parity_net_calculation() {
        let mut summary = BatchSummary::new();

        // Add multiple credits
        for _ in 0..3 {
            summary.add_transaction(&Transaction::new(10001, 50000.0, 'C'));
        }

        // Add multiple debits
        for _ in 0..2 {
            summary.add_transaction(&Transaction::new(10002, 25000.0, 'D'));
        }

        // Total credits = 150,000, debits = 50,000, net = 100,000
        let cobol_net = 100000.0;
        assert!(assert_parity_f64(cobol_net, summary.net_change, 1e-10));
    }

    #[test]
    fn test_batch_processor_parity_transaction_count() {
        let mut summary = BatchSummary::new();
        let transactions = vec![
            Transaction::new(1, 1000.0, 'C'),
            Transaction::new(2, 2000.0, 'C'),
            Transaction::new(3, 3000.0, 'D'),
            Transaction::new(4, 4000.0, 'D'),
            Transaction::new(5, 5000.0, 'C'),
        ];

        summary.process_transactions(&transactions);

        // Credits: 1000+2000+5000=8000, Debits: 3000+4000=7000
        assert_eq!(summary.transaction_count, 5);
        assert!(assert_parity_f64(8000.0, summary.total_credits, 1e-10));
        assert!(assert_parity_f64(7000.0, summary.total_debits, 1e-10));
    }

    // MOD-004: Eigenvalue Solver Parity Tests

    #[test]
    fn test_eigenvalue_solver_parity_convergence() {
        let solver = PowerIterationSolver::new(100, 1e-10);
        let matrix = PowerIterationSolver::initialize_matrix(5);
        let (eigenvalue, _eigenvector) = solver.solve(&matrix);

        // Fortran eigenvalue should be approximately 5.6 for this matrix
        assert!(eigenvalue > 5.0);
        assert!(eigenvalue < 6.0);
    }

    #[test]
    fn test_eigenvalue_solver_parity_eigenvector_property() {
        // A*v = λ*v must hold
        let solver = PowerIterationSolver::new(100, 1e-10);
        let matrix = PowerIterationSolver::initialize_matrix(5);
        let (eigenvalue, eigenvector) = solver.solve(&matrix);

        let av = solver.matvec(&matrix, &eigenvector);

        // Check each component: av[i] ≈ λ * v[i]
        for (i, (avi, vi)) in av.iter().zip(eigenvector.iter()).enumerate() {
            let expected = eigenvalue * vi;
            assert!(
                (avi - expected).abs() < 1e-4,
                "Parity failure at index {}: {} vs {}",
                i,
                avi,
                expected
            );
        }
    }

    #[test]
    fn test_eigenvalue_solver_parity_normalization() {
        let solver = PowerIterationSolver::new(100, 1e-10);
        let matrix = PowerIterationSolver::initialize_matrix(5);
        let (_eigenvalue, eigenvector) = solver.solve(&matrix);

        // Eigenvector must be unit length
        let norm = solver.norm(&eigenvector);
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_eigenvalue_solver_parity_tolerance_agreement() {
        // Two solvers with different tolerances should agree on eigenvalue
        let solver_tight = PowerIterationSolver::new(1000, 1e-12);
        let solver_loose = PowerIterationSolver::new(1000, 1e-6);

        let matrix = PowerIterationSolver::initialize_matrix(5);

        let (eig_tight, _) = solver_tight.solve(&matrix);
        let (eig_loose, _) = solver_loose.solve(&matrix);

        // Should converge to same value
        assert!((eig_tight - eig_loose).abs() < 1e-4);
    }

    // MOD-005: Matrix Operations Parity Tests

    #[test]
    fn test_matrix_multiply_parity_identity() {
        let identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let a = vec![vec![5.0, 3.0], vec![2.0, 4.0]];
        let result = MatrixOps::matrix_multiply(&identity, &a);

        // I*A = A
        for i in 0..2 {
            for j in 0..2 {
                assert!(assert_parity_f64(a[i][j], result[i][j], 1e-10));
            }
        }
    }

    #[test]
    fn test_matrix_multiply_parity_general() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let c = MatrixOps::matrix_multiply(&a, &b);

        // Manual computation: [19, 22; 43, 50]
        assert!(assert_parity_f64(19.0, c[0][0], 1e-10));
        assert!(assert_parity_f64(22.0, c[0][1], 1e-10));
        assert!(assert_parity_f64(43.0, c[1][0], 1e-10));
        assert!(assert_parity_f64(50.0, c[1][1], 1e-10));
    }

    #[test]
    fn test_lu_decomposition_parity_reconstruction() {
        let a = vec![
            vec![4.0, 3.0, 1.0],
            vec![3.0, 5.0, 2.0],
            vec![1.0, 2.0, 3.0],
        ];

        let (l, u) = MatrixOps::lu_decomposition(&a);
        let lu = MatrixOps::matrix_multiply(&l, &u);

        // L*U should reconstruct A
        for i in 0..3 {
            for j in 0..3 {
                assert!(assert_parity_f64(a[i][j], lu[i][j], 1e-8));
            }
        }
    }

    #[test]
    fn test_lu_decomposition_parity_unit_lower() {
        let a = vec![
            vec![4.0, 3.0, 1.0],
            vec![3.0, 5.0, 2.0],
            vec![1.0, 2.0, 3.0],
        ];

        let (l, _u) = MatrixOps::lu_decomposition(&a);

        // L must have 1s on diagonal
        for i in 0..3 {
            assert!(assert_parity_f64(1.0, l[i][i], 1e-10));
        }
    }

    #[test]
    fn test_gaussian_elimination_parity_2x2() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![8.0, 13.0];
        let x = MatrixOps::gaussian_elimination(&a, &b);

        // Solution of 2x+y=8, x+3y=13: x=2.2, y=3.6
        assert!(assert_parity_f64(2.2, x[0], 1e-8));
        assert!(assert_parity_f64(3.6, x[1], 1e-8));
    }

    #[test]
    fn test_gaussian_elimination_parity_3x3_residual() {
        let a = vec![
            vec![3.0, 2.0, -1.0],
            vec![2.0, -2.0, 4.0],
            vec![-1.0, 0.5, -1.0],
        ];
        let b = vec![1.0, -2.0, 0.0];
        let x = MatrixOps::gaussian_elimination(&a, &b);

        // Verify A*x = b (residual should be near 0)
        for i in 0..3 {
            let mut sum = 0.0;
            for j in 0..3 {
                sum += a[i][j] * x[j];
            }
            assert!(assert_parity_f64(b[i], sum, 1e-8));
        }
    }

    #[test]
    fn test_gaussian_elimination_parity_solution_vector() {
        let a = vec![
            vec![2.0, 1.0],
            vec![1.0, 3.0],
        ];
        let b = vec![5.0, 6.0];
        let x = MatrixOps::gaussian_elimination(&a, &b);

        // Verify each equation
        // 2x + y = 5
        let eq1 = 2.0 * x[0] + x[1];
        assert!(assert_parity_f64(5.0, eq1, 1e-10));

        // x + 3y = 6
        let eq2 = x[0] + 3.0 * x[1];
        assert!(assert_parity_f64(6.0, eq2, 1e-10));
    }

    // Cross-module integration test

    #[test]
    fn test_cross_module_account_batch_integration() {
        // Use account balance results in batch processor
        let (running, interest, final_balance) = process_account(100000.0, 50000.0, 0.035);

        let mut batch = BatchSummary::new();
        batch.add_transaction(&Transaction::new(1, final_balance, 'C'));
        batch.add_transaction(&Transaction::new(2, 50000.0, 'D'));

        // Net should be final_balance - 50000
        let expected_net = final_balance - 50000.0;
        assert!(assert_parity_f64(expected_net, batch.net_change, 1e-10));
    }
}
