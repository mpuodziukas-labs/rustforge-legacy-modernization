mod account_balance;
mod batch_processor;
mod cobol_analyzer;
mod eigenvalue_solver;
mod inventory_valuation;
mod loan_calculator;
mod matrix_operations;
mod parity;
mod report_generator;
mod statistics;

use account_balance::{display_report, process_account};
use batch_processor::{BatchSummary, Transaction};
use eigenvalue_solver::PowerIterationSolver;
use inventory_valuation::{display_report as inv_display_report, InventoryLedger};
use loan_calculator::{calculate_loan, display_amortization, LoanParams};
use matrix_operations::MatrixOps;
use parity::parity::{assert_parity_f64, print_parity_result};
use report_generator::generate_sample_report;
use statistics::{compute_stats, display_report as stats_display_report};

fn main() {
    println!("================================================");
    println!("RUSTFORGE LEGACY MODERNIZATION SUITE");
    println!("================================================\n");

    demo_account_balance();
    println!();
    demo_batch_processor();
    println!();
    demo_report_generator();
    println!();
    demo_eigenvalue_solver();
    println!();
    demo_matrix_operations();
    println!();
    demo_loan_calculator();
    println!();
    demo_inventory_valuation();
    println!();
    demo_statistics();
}

fn demo_account_balance() {
    println!("--- MODULE 1: ACCOUNT BALANCE ---\n");

    let opening = 100000.0;
    let transaction = 50000.0;
    let rate = 0.035;

    let (running, interest, final_balance) = process_account(opening, transaction, rate);

    display_report(opening, transaction, running, interest, final_balance, rate);

    // Parity check with known COBOL values
    let cobol_final = 155250.0;
    let passed = assert_parity_f64(cobol_final, final_balance, 1e-10);
    print_parity_result(
        "Final Balance Parity",
        cobol_final,
        final_balance,
        1e-10,
        passed,
    );
}

fn demo_batch_processor() {
    println!("--- MODULE 2: BATCH PROCESSOR ---\n");

    let mut summary = BatchSummary::new();

    let transactions = vec![
        Transaction::new(12345001, 100000.0, 'C'),
        Transaction::new(12345002, 75000.0, 'C'),
        Transaction::new(12345003, 100000.0, 'C'),
        Transaction::new(12345004, 50000.0, 'D'),
        Transaction::new(12345005, 75000.0, 'D'),
    ];

    summary.process_transactions(&transactions);

    println!("====================================");
    println!("BATCH PROCESSOR SUMMARY REPORT");
    println!("====================================");
    println!("Transactions:        {}", summary.transaction_count);
    println!("Total Credits:       ${:.2}", summary.total_credits);
    println!("Total Debits:        ${:.2}", summary.total_debits);
    println!("Net Change:          ${:.2}", summary.net_change);
    println!("====================================");

    // Parity check
    let cobol_total_credits = 275000.0;
    let passed = assert_parity_f64(cobol_total_credits, summary.total_credits, 1e-10);
    print_parity_result(
        "Credits Parity",
        cobol_total_credits,
        summary.total_credits,
        1e-10,
        passed,
    );
}

fn demo_report_generator() {
    println!("--- MODULE 3: REPORT GENERATOR ---\n");

    let report = generate_sample_report();
    report.display();

    // Verify report was generated correctly
    println!("Report Status: {} transactions captured", report.transactions.len());
}

fn demo_eigenvalue_solver() {
    println!("--- MODULE 4: EIGENVALUE SOLVER ---\n");

    let solver = PowerIterationSolver::new(100, 1e-10);
    let matrix = PowerIterationSolver::initialize_matrix(5);
    let (eigenvalue, eigenvector) = solver.solve(&matrix);

    solver.print_results(eigenvalue, &eigenvector);

    // Verify A*v ≈ λ*v
    let av = solver.matvec(&matrix, &eigenvector);
    let mut max_error = 0.0;
    for (avi, vi) in av.iter().zip(eigenvector.iter()) {
        let error = (avi - eigenvalue * vi).abs() as f64;
        if error > max_error {
            max_error = error;
        }
    }

    println!("Eigenvalue Verification: Max error = {:.2e}", max_error);
    println!("(Should be < 1e-10 for convergence)");

    print_parity_result(
        "Eigenvalue Convergence",
        0.0,
        max_error,
        1e-8,
        max_error < 1e-8,
    );
}

fn demo_matrix_operations() {
    println!("--- MODULE 5: MATRIX OPERATIONS ---\n");

    // Matrix multiplication
    println!("Matrix Multiplication:");
    let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    let c = MatrixOps::matrix_multiply(&a, &b);

    println!("A =");
    for row in &a {
        println!("  {:?}", row);
    }
    println!("B =");
    for row in &b {
        println!("  {:?}", row);
    }
    println!("C = A*B =");
    for row in &c {
        println!("  {:?}", row);
    }

    // LU decomposition
    println!("\nLU Decomposition:");
    let matrix = vec![
        vec![4.0, 3.0, 1.0],
        vec![3.0, 5.0, 2.0],
        vec![1.0, 2.0, 3.0],
    ];

    let (l, u) = MatrixOps::lu_decomposition(&matrix);
    println!("L =");
    for row in &l {
        println!("  {:?}", row);
    }
    println!("U =");
    for row in &u {
        println!("  {:?}", row);
    }

    // Verify L*U = original matrix
    let lu = MatrixOps::matrix_multiply(&l, &u);
    let mut decomp_error: f64 = 0.0;
    for i in 0..3 {
        for j in 0..3 {
            decomp_error = decomp_error.max((lu[i][j] - matrix[i][j]).abs());
        }
    }

    println!(
        "LU Reconstruction Error: {:.2e} (should be < 1e-8)",
        decomp_error
    );

    // Gaussian elimination
    println!("\nGaussian Elimination:");
    let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
    let b = vec![8.0, 13.0];
    let x = MatrixOps::gaussian_elimination(&a, &b);

    println!("Solving 2x + y = 8, x + 3y = 13");
    println!("Solution: x = {:.4}, y = {:.4}", x[0], x[1]);
    println!("Expected: x = 3.0000, y = 2.0000");

    // Verify solution
    let residual = (a[0][0] * x[0] + a[0][1] * x[1] - b[0]).abs()
        + (a[1][0] * x[0] + a[1][1] * x[1] - b[1]).abs();
    println!("Residual: {:.2e}", residual);

    println!("\n================================================");
    println!("ALL MODULES COMPLETE");
    println!("================================================");
}

fn demo_loan_calculator() {
    println!("--- MODULE 3: LOAN CALCULATOR (MOD-003) ---\n");

    let params = LoanParams {
        principal: 300_000.0,
        annual_rate_pct: 6.0,
        term_months: 360,
    };
    let result = calculate_loan(&params);
    display_amortization(&result);

    // Parity check: Rust payment matches COBOL formula
    let monthly_rate = 6.0_f64 / 12.0 / 100.0;
    let cobol_payment = 300_000.0 * monthly_rate
        / (1.0 - (1.0 + monthly_rate).powi(-360));

    let passed = assert_parity_f64(cobol_payment, result.monthly_payment, 1e-10);
    print_parity_result(
        "Monthly Payment Parity (COBOL formula)",
        cobol_payment,
        result.monthly_payment,
        1e-10,
        passed,
    );
}

fn demo_inventory_valuation() {
    println!("--- MODULE 4: INVENTORY VALUATION / FIFO (MOD-004) ---\n");

    let mut ledger = InventoryLedger::new("WIDGET-A");
    ledger.purchase(100, 10.00);
    ledger.purchase(200, 12.50);
    ledger.purchase(150, 15.00);

    let cogs = ledger.sell(250);
    println!("Sale of 250 units — COGS: ${:.2}", cogs);

    inv_display_report(&ledger);

    // Parity: COBOL FIFO-SELL result
    let cobol_cogs = 100.0 * 10.00 + 150.0 * 12.50;
    let passed = assert_parity_f64(cobol_cogs, cogs, 1e-10);
    print_parity_result("FIFO COGS Parity (COBOL formula)", cobol_cogs, cogs, 1e-10, passed);
}

fn demo_statistics() {
    println!("--- MODULE 5: STATISTICS REPORT GENERATOR (MOD-005) ---\n");

    // Fortran canonical test vector
    let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0];
    let stats = compute_stats(&data).unwrap();
    stats_display_report(&stats);

    // Parity: replicate Fortran formula
    let n = data.len();
    let fortran_mean: f64 = data.iter().sum::<f64>() / n as f64;
    let sum_sq: f64 = data.iter().map(|&x| (x - fortran_mean).powi(2)).sum();
    let fortran_var = sum_sq / (n - 1) as f64;

    let passed = assert_parity_f64(fortran_var, stats.variance, 1e-10);
    print_parity_result(
        "Variance Parity (Fortran formula)",
        fortran_var,
        stats.variance,
        1e-10,
        passed,
    );
}
