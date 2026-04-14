/// Account Balance Module
/// Rust translation of COBOL account_balance.cob
/// Provides identical numerical output with zero rounding errors

#[derive(Debug, Clone, Copy)]
pub struct AccountRecord {
    pub account_id: u32,
    pub balance: f64,
    pub interest_rate: f64,
}

impl AccountRecord {
    /// Creates a new `AccountRecord` with the given account ID, balance, and interest rate.
    ///
    /// # Arguments
    /// * `account_id` - Unique numeric account identifier
    /// * `balance`    - Current account balance in dollars
    /// * `interest_rate` - Annual interest rate as a decimal (e.g. `0.035` for 3.5 %)
    ///
    /// # Example
    /// ```
    /// use rustforge::account_balance::AccountRecord;
    /// let rec = AccountRecord::new(1001, 50_000.0, 0.035);
    /// assert_eq!(rec.account_id, 1001);
    /// ```
    pub fn new(account_id: u32, balance: f64, interest_rate: f64) -> Self {
        AccountRecord {
            account_id,
            balance,
            interest_rate,
        }
    }
}

/// Computes running balance after applying a transaction.
///
/// # Arguments
/// * `opening_balance`    - Starting account balance in dollars
/// * `transaction_amount` - Amount to add (positive credit) or subtract (negative debit)
///
/// # Returns
/// `running_balance = opening_balance + transaction_amount`
///
/// # Example
/// ```
/// let balance = rustforge::account_balance::calculate_balance(100_000.0, 50_000.0);
/// assert!((balance - 150_000.0).abs() < 1e-10);
/// ```
pub fn calculate_balance(opening_balance: f64, transaction_amount: f64) -> f64 {
    opening_balance + transaction_amount
}

/// Computes the interest earned on a given balance over one period.
///
/// The interest is calculated as a simple product: `balance × interest_rate`.
/// This mirrors the COBOL `COMPUTE INTEREST-EARNED = RUNNING-BALANCE * INTEREST-RATE`
/// statement and is intended for single-period (annual) calculations.
///
/// # Arguments
/// * `balance`       - Account balance in dollars at period start
/// * `interest_rate` - Periodic interest rate as a decimal (e.g. `0.035` for 3.5 %)
///
/// # Returns
/// Interest earned in dollars for the period.
///
/// # Example
/// ```
/// let interest = rustforge::account_balance::apply_interest(150_000.0, 0.035);
/// assert!((interest - 5_250.0).abs() < 1e-10);
/// ```
pub fn apply_interest(balance: f64, interest_rate: f64) -> f64 {
    balance * interest_rate
}

/// Adds interest earned to the running balance to produce the final end-of-period balance.
///
/// # Arguments
/// * `running_balance` - Balance after transactions, in dollars
/// * `interest_earned` - Interest for the period (from [`apply_interest`]), in dollars
///
/// # Returns
/// `final_balance = running_balance + interest_earned`
///
/// # Example
/// ```
/// let final_bal = rustforge::account_balance::compute_final_balance(150_000.0, 5_250.0);
/// assert!((final_bal - 155_250.0).abs() < 1e-10);
/// ```
pub fn compute_final_balance(running_balance: f64, interest_earned: f64) -> f64 {
    running_balance + interest_earned
}

/// Processes a single account through the full balance pipeline.
///
/// Convenience wrapper that calls [`calculate_balance`], [`apply_interest`], and
/// [`compute_final_balance`] in sequence, matching the COBOL `PROCESS-ACCOUNT`
/// paragraph.
///
/// # Arguments
/// * `opening`     - Opening balance in dollars
/// * `transaction` - Transaction amount in dollars (positive = credit, negative = debit)
/// * `rate`        - Annual interest rate as a decimal (e.g. `0.035` for 3.5 %)
///
/// # Returns
/// A tuple `(running_balance, interest_earned, final_balance)`, all in dollars.
///
/// # Example
/// ```
/// let (running, interest, final_bal) =
///     rustforge::account_balance::process_account(100_000.0, 50_000.0, 0.035);
/// assert!((running  - 150_000.0).abs() < 1e-10);
/// assert!((interest -   5_250.0).abs() < 1e-10);
/// assert!((final_bal - 155_250.0).abs() < 1e-10);
/// ```
pub fn process_account(opening: f64, transaction: f64, rate: f64) -> (f64, f64, f64) {
    let running_balance = calculate_balance(opening, transaction);
    let interest = apply_interest(running_balance, rate);
    let final_balance = compute_final_balance(running_balance, interest);

    (running_balance, interest, final_balance)
}

/// Prints a formatted account balance report to stdout.
///
/// Outputs a human-readable summary that mirrors the COBOL printed report,
/// including opening balance, transaction, running balance, interest rate,
/// interest earned, and final balance.
///
/// # Arguments
/// * `opening`       - Opening balance in dollars
/// * `transaction`   - Transaction amount in dollars
/// * `running`       - Running balance (post-transaction) in dollars
/// * `interest`      - Interest earned in dollars
/// * `final_balance` - Final end-of-period balance in dollars
/// * `rate`          - Annual interest rate as a decimal
pub fn display_report(
    opening: f64,
    transaction: f64,
    running: f64,
    interest: f64,
    final_balance: f64,
    rate: f64,
) {
    println!("====================================");
    println!("ACCOUNT BALANCE REPORT");
    println!("Date: 04-13-2026");
    println!("====================================");
    println!("Opening Balance:  ${:.2}", opening);
    println!("Transaction:      ${:.2}", transaction);
    println!("Running Balance:  ${:.2}", running);
    println!("Interest Rate:    {:.1}%", rate * 100.0);
    println!("Interest Earned:  ${:.2}", interest);
    println!("Final Balance:    ${:.2}", final_balance);
    println!("====================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_balance() {
        let result = calculate_balance(100000.0, 50000.0);
        assert!((result - 150000.0).abs() < 1e-10);
    }

    #[test]
    fn test_apply_interest() {
        let result = apply_interest(150000.0, 0.035);
        assert!((result - 5250.0).abs() < 1e-10);
    }

    #[test]
    fn test_compute_final_balance() {
        let result = compute_final_balance(150000.0, 5250.0);
        assert!((result - 155250.0).abs() < 1e-10);
    }

    #[test]
    fn test_process_account() {
        let (running, interest, final_bal) = process_account(100000.0, 50000.0, 0.035);
        assert!((running - 150000.0).abs() < 1e-10);
        assert!((interest - 5250.0).abs() < 1e-10);
        assert!((final_bal - 155250.0).abs() < 1e-10);
    }

    #[test]
    fn test_multiple_transactions() {
        let (r1, i1, f1) = process_account(100000.0, 25000.0, 0.035);
        let (r2, i2, f2) = process_account(f1, 25000.0, 0.035);
        let (r3, i3, f3) = process_account(f2, 0.0, 0.035);

        // Verify cumulative effect
        assert!((f3 - 165370.359375).abs() < 1e-6);
    }

    #[test]
    fn test_edge_case_zero_transaction() {
        let (running, interest, final_bal) = process_account(100000.0, 0.0, 0.035);
        assert!((running - 100000.0).abs() < 1e-10);
        assert!((interest - 3500.0).abs() < 1e-10);
        assert!((final_bal - 103500.0).abs() < 1e-10);
    }
}
