/// Cross-module integration tests
///
/// Tests that chain multiple RustForge modules together,
/// verifying they work as a complete migration pipeline.
/// These tests prove the system works end-to-end, not just module-by-module.

#[cfg(test)]
mod integration_chain_tests {
    use rustforge::account_balance;
    use rustforge::batch_processor::{BatchSummary, Transaction};
    use rustforge::loan_calculator::{calculate_loan, LoanParams};
    use rustforge::statistics::compute_stats;
    use rustforge::error::verify_parity;

    // ========== CHAIN 1: Account balance feeds into batch processor ==========

    /// Test that account balances can be fed into the batch processor as transactions
    #[test]
    fn test_account_to_batch_chain() {
        // Calculate final balances for 3 accounts
        let (_, _, bal1) = account_balance::process_account(100_000.0, 50_000.0, 0.035);
        let (_, _, bal2) = account_balance::process_account(200_000.0, 75_000.0, 0.035);
        let (_, _, bal3) = account_balance::process_account(50_000.0, 25_000.0, 0.035);

        // Feed into batch processor as credit transactions
        let mut batch = BatchSummary::new();
        batch.add_transaction(&Transaction::new(1001, bal1, 'C'));
        batch.add_transaction(&Transaction::new(1002, bal2, 'C'));
        batch.add_transaction(&Transaction::new(1003, bal3, 'C'));

        let expected_total = bal1 + bal2 + bal3;
        assert!(verify_parity("batch_credits", expected_total, batch.total_credits, 1e-10).is_ok());
        assert_eq!(batch.transaction_count, 3);
        assert_eq!(batch.total_debits, 0.0);
    }

    // ========== CHAIN 2: Loan payments feed into batch processor ==========

    /// Test that loan payment schedule can be processed as batch transactions
    #[test]
    fn test_loan_payments_to_batch() {
        let params = LoanParams {
            principal: 300_000.0,
            annual_rate_pct: 6.5,
            term_months: 360,
        };
        let loan = calculate_loan(&params);

        // Feed 12 monthly payments into batch processor as debit transactions
        let mut batch = BatchSummary::new();
        for month in 0..12 {
            batch.add_transaction(&Transaction::new(
                month as u32 + 1,
                loan.monthly_payment,
                'D',
            ));
        }

        let expected_annual = loan.monthly_payment * 12.0;
        assert!(verify_parity("annual_payments", expected_annual, batch.total_debits, 1e-6).is_ok());
        assert_eq!(batch.transaction_count, 12);
        assert_eq!(batch.total_credits, 0.0);
    }

    // ========== CHAIN 3: Batch totals feed into statistics engine ==========

    /// Test that batch processor output can feed into statistical analysis
    #[test]
    fn test_batch_amounts_to_statistics() {
        let amounts = vec![
            100_000.0, 75_000.0, 50_000.0, 125_000.0, 200_000.0,
            80_000.0, 90_000.0, 110_000.0, 95_000.0, 105_000.0,
        ];

        let stats = compute_stats(&amounts).expect("stats must compute");

        // Verify statistical properties
        let manual_mean: f64 = amounts.iter().sum::<f64>() / amounts.len() as f64;
        assert!(verify_parity("mean", manual_mean, stats.mean, 1e-10).is_ok());
        assert!(stats.min < stats.max);
        assert!(stats.std_dev > 0.0);
        assert_eq!(stats.count, 10);
    }

    // ========== CHAIN 4: Account balance cascade with statistics ==========

    /// Test that repeated account balance calculations can be analyzed statistically
    #[test]
    fn test_account_cascade_with_statistics() {
        let mut final_balances = Vec::new();
        let mut balance = 100_000.0;

        // Simulate 10 periods of compounding account growth
        for _ in 0..10 {
            let (_, _, final_bal) = account_balance::process_account(balance, 5_000.0, 0.035);
            final_balances.push(final_bal);
            balance = final_bal;
        }

        let stats = compute_stats(&final_balances).expect("stats must compute");

        // Verify balance grows monotonically and statistics are correct
        assert!(stats.max > stats.min);
        assert!(stats.mean > 100_000.0);
        assert_eq!(stats.count, 10);
        assert!(stats.std_dev >= 0.0);

        // Final balance should match the maximum in the series
        let last_balance = *final_balances.last().unwrap();
        assert!(verify_parity("final_cascade", last_balance, stats.max, 1e-10).is_ok());
    }

    // ========== CHAIN 5: Loan amortization analysis ==========

    /// Test that full amortization schedule can be analyzed statistically
    #[test]
    fn test_loan_amortization_statistics() {
        let params = LoanParams {
            principal: 500_000.0,
            annual_rate_pct: 7.0,
            term_months: 360,
        };
        let loan = calculate_loan(&params);

        // Extract interest payments from amortization schedule
        let interest_payments: Vec<f64> = loan.amortization
            .iter()
            .map(|(_, _, interest, _)| *interest)
            .collect();

        let stats = compute_stats(&interest_payments).expect("stats must compute");

        // Interest should be highest at the start (first payments) and lowest at end
        assert!(stats.max > stats.min);
        assert_eq!(stats.count, 360);

        // Total interest paid should match loan calculation
        let computed_total_interest: f64 = interest_payments.iter().sum();
        assert!(verify_parity("total_interest", loan.total_interest, computed_total_interest, 1.0).is_ok());
    }

    // ========== CHAIN 6: Complex scenario - Account statements with loan payments ==========

    /// Test a realistic scenario: Multiple accounts with loan payment transactions
    #[test]
    fn test_complex_financial_scenario() {
        // Step 1: Calculate account balances
        let (_, _, checking_balance) = account_balance::process_account(50_000.0, 10_000.0, 0.01);
        let (_, _, savings_balance) = account_balance::process_account(100_000.0, 5_000.0, 0.025);

        // Step 2: Create loan for property
        let loan_params = LoanParams {
            principal: 400_000.0,
            annual_rate_pct: 5.5,
            term_months: 360,
        };
        let loan = calculate_loan(&loan_params);

        // Step 3: Build batch with account balances and first 12 loan payments
        let mut batch = BatchSummary::new();

        // Add account balances as credits
        batch.add_transaction(&Transaction::new(1001, checking_balance, 'C'));
        batch.add_transaction(&Transaction::new(1002, savings_balance, 'C'));

        // Add 12 monthly loan payments as debits
        for month in 0..12 {
            batch.add_transaction(&Transaction::new(
                2000 + month as u32,
                loan.monthly_payment,
                'D',
            ));
        }

        // Step 4: Verify batch totals
        let total_credits = checking_balance + savings_balance;
        let total_debits = loan.monthly_payment * 12.0;

        assert!(verify_parity("credits", total_credits, batch.total_credits, 1e-10).is_ok());
        assert!(verify_parity("debits", total_debits, batch.total_debits, 1e-6).is_ok());
        assert_eq!(batch.transaction_count, 14);
        assert!(verify_parity("net_change",
            total_credits - total_debits,
            batch.net_change,
            1e-6).is_ok());

        // Step 5: Analyze all transaction amounts statistically
        let mut all_amounts = vec![checking_balance, savings_balance];
        for _ in 0..12 {
            all_amounts.push(loan.monthly_payment);
        }

        let stats = compute_stats(&all_amounts).expect("stats must compute");
        assert_eq!(stats.count, 14);
        assert!(stats.mean > 0.0);
        assert!(stats.std_dev > 0.0);
    }

    // ========== CHAIN 7: Interest accrual across multiple accounts ==========

    /// Test interest calculation consistency across multiple accounts feeding into summary
    #[test]
    fn test_multi_account_interest_consistency() {
        let accounts = vec![
            (100_000.0, 50_000.0, 0.035),
            (200_000.0, 75_000.0, 0.035),
            (150_000.0, 30_000.0, 0.035),
            (75_000.0, 25_000.0, 0.035),
            (300_000.0, 100_000.0, 0.035),
        ];

        let mut total_final_balances = Vec::new();
        let mut all_interest_amounts = Vec::new();

        // Process each account
        for (opening, transaction, rate) in accounts {
            let (running, interest, final_bal) = account_balance::process_account(opening, transaction, rate);

            // Verify each account's parity
            let expected_running = opening + transaction;
            assert!(verify_parity("running", expected_running, running, 1e-10).is_ok());

            let expected_interest = running * rate;
            assert!(verify_parity("interest", expected_interest, interest, 1e-10).is_ok());

            let expected_final = running + interest;
            assert!(verify_parity("final", expected_final, final_bal, 1e-10).is_ok());

            total_final_balances.push(final_bal);
            all_interest_amounts.push(interest);
        }

        // Analyze all final balances
        let balance_stats = compute_stats(&total_final_balances).expect("balance stats");
        assert_eq!(balance_stats.count, 5);
        assert!(balance_stats.mean > 0.0);

        // Analyze all interest amounts
        let interest_stats = compute_stats(&all_interest_amounts).expect("interest stats");
        assert_eq!(interest_stats.count, 5);
        assert!(interest_stats.mean > 0.0);
        assert!(interest_stats.min > 0.0); // All accounts earned interest
    }

    // ========== CHAIN 8: Zero-interest loan with batch processing ==========

    /// Test that zero-interest loans work correctly in batch pipeline
    #[test]
    fn test_zero_interest_loan_in_batch() {
        let params = LoanParams {
            principal: 120_000.0,
            annual_rate_pct: 0.0,
            term_months: 60,
        };
        let loan = calculate_loan(&params);

        // Each payment should be equal to principal / term
        let expected_payment = 120_000.0 / 60.0;
        assert!(verify_parity("payment", expected_payment, loan.monthly_payment, 1e-10).is_ok());
        assert_eq!(loan.total_interest, 0.0);

        // Add payments to batch as debits
        let mut batch = BatchSummary::new();
        for month in 0..60 {
            batch.add_transaction(&Transaction::new(
                month as u32 + 1,
                loan.monthly_payment,
                'D',
            ));
        }

        assert_eq!(batch.total_debits, 120_000.0);
        assert_eq!(batch.transaction_count, 60);
    }
}
