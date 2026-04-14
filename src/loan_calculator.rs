//! Loan Calculator Module — MOD-003
//! Rust translation of COBOL loan_calculator.cob
//! Computes amortizing loan payments with full amortization schedule.
//! Formula parity: matches COBOL COMPUTE statements exactly.

/// Input parameters for a fixed-rate amortising loan.
#[derive(Debug, Clone)]
pub struct LoanParams {
    /// Loan principal in dollars (e.g. 300_000.0)
    pub principal: f64,
    /// Annual interest rate as a percentage (e.g. 6.0 for 6%)
    pub annual_rate_pct: f64,
    /// Loan term in months (e.g. 360 for 30 years)
    pub term_months: u32,
}

/// Full loan calculation result, including amortization schedule.
#[derive(Debug, Clone)]
pub struct LoanResult {
    pub monthly_payment: f64,
    pub total_payment: f64,
    pub total_interest: f64,
    /// Each row: (month, payment, interest_portion, principal_remaining)
    pub amortization: Vec<(u32, f64, f64, f64)>,
}

/// Calculates the monthly payment and full amortization schedule for a fixed-rate loan.
///
/// Uses the standard annuity (present-value) formula, replicating the COBOL
/// `COMPUTE` statements exactly:
///
/// ```text
/// monthly_rate  = annual_rate_pct / 12 / 100
/// payment       = principal × monthly_rate / (1 − (1 + monthly_rate)^(−term_months))
/// ```
///
/// A zero-interest special case is handled separately — payments become equal
/// principal instalments with no interest component.
///
/// # Arguments
/// * `params` - [`LoanParams`] containing principal (dollars), annual rate (percent),
///   and term (months)
///
/// # Returns
/// A [`LoanResult`] with monthly payment, total payment, total interest, and the
/// full month-by-month amortization schedule.
///
/// # Example
/// ```
/// use rustforge::loan_calculator::{calculate_loan, LoanParams};
/// let params = LoanParams { principal: 300_000.0, annual_rate_pct: 6.0, term_months: 360 };
/// let result = calculate_loan(&params);
/// assert!((result.monthly_payment - 1_798.65).abs() < 0.01);
/// assert_eq!(result.amortization.len(), 360);
/// ```
pub fn calculate_loan(params: &LoanParams) -> LoanResult {
    let principal = params.principal;
    let term = params.term_months;

    // Zero-interest special case — equal principal instalments.
    if params.annual_rate_pct == 0.0 {
        let monthly_payment = principal / term as f64;
        let mut amortization = Vec::with_capacity(term as usize);
        let mut balance = principal;

        for month in 1..=term {
            let principal_paid = monthly_payment.min(balance);
            balance -= principal_paid;
            amortization.push((month, monthly_payment, 0.0, balance.max(0.0)));
        }

        return LoanResult {
            monthly_payment,
            total_payment: monthly_payment * term as f64,
            total_interest: 0.0,
            amortization,
        };
    }

    // Monthly rate — matches COBOL: annual_rate / 12 / 100
    let monthly_rate = params.annual_rate_pct / 12.0 / 100.0;

    // Standard annuity payment formula — matches COBOL COMPUTE
    let power_factor = (1.0 + monthly_rate).powi(-(term as i32));
    let denom = 1.0 - power_factor;
    let monthly_payment = principal * monthly_rate / denom;

    // Build amortization schedule
    let mut amortization = Vec::with_capacity(term as usize);
    let mut balance = principal;

    for month in 1..=term {
        let interest_charge = balance * monthly_rate;
        let principal_paid = monthly_payment - interest_charge;
        balance -= principal_paid;
        // Clamp tiny floating-point residuals at final payment
        if balance < 1e-6 {
            balance = 0.0;
        }
        amortization.push((month, monthly_payment, interest_charge, balance));
    }

    let total_payment = monthly_payment * term as f64;
    let total_interest = total_payment - principal;

    LoanResult {
        monthly_payment,
        total_payment,
        total_interest,
        amortization,
    }
}

/// Prints a formatted amortization schedule to stdout.
///
/// Displays the first three and last three rows of the schedule (with an ellipsis
/// for longer loans) along with the summary totals, matching the style of the
/// original COBOL printed report.
///
/// # Arguments
/// * `result` - Completed [`LoanResult`] from [`calculate_loan`]
pub fn display_amortization(result: &LoanResult) {
    println!("====================================");
    println!("LOAN AMORTIZATION SCHEDULE");
    println!("====================================");
    println!(
        "{:>6}  {:>12}  {:>12}  {:>14}",
        "Month", "Payment", "Interest", "Balance"
    );
    println!("{}", "-".repeat(50));

    // Print first 3 and last 3 rows to keep output tidy
    let n = result.amortization.len();
    let preview: Vec<usize> = if n <= 6 {
        (0..n).collect()
    } else {
        let mut v: Vec<usize> = (0..3).collect();
        v.push(usize::MAX); // sentinel for ellipsis
        v.extend((n - 3)..n);
        v
    };

    for &i in &preview {
        if i == usize::MAX {
            println!("  ...");
            continue;
        }
        let (month, payment, interest, balance) = result.amortization[i];
        println!(
            "{:>6}  {:>12.2}  {:>12.2}  {:>14.2}",
            month, payment, interest, balance
        );
    }

    println!("{}", "-".repeat(50));
    println!("Monthly Payment:  ${:.2}", result.monthly_payment);
    println!("Total Payment:    ${:.2}", result.total_payment);
    println!("Total Interest:   ${:.2}", result.total_interest);
    println!("====================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: relative tolerance check for dollar amounts
    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    /// 30-year $300K mortgage at 6% — canonical banking test.
    #[test]
    fn test_basic_mortgage() {
        let params = LoanParams {
            principal: 300_000.0,
            annual_rate_pct: 6.0,
            term_months: 360,
        };
        let result = calculate_loan(&params);
        // Industry-standard payment: $1,798.65
        assert!(
            approx(result.monthly_payment, 1_798.65, 0.01),
            "Expected ~$1798.65, got {:.2}",
            result.monthly_payment
        );
        assert_eq!(result.amortization.len(), 360);
    }

    /// Zero interest: payments are equal principal instalments.
    #[test]
    fn test_zero_interest() {
        let params = LoanParams {
            principal: 12_000.0,
            annual_rate_pct: 0.0,
            term_months: 12,
        };
        let result = calculate_loan(&params);
        assert!(approx(result.monthly_payment, 1_000.0, 1e-10));
        assert!(approx(result.total_interest, 0.0, 1e-10));
        assert!(approx(result.total_payment, 12_000.0, 1e-10));
    }

    /// Single-month loan: payment equals principal + one month's interest.
    #[test]
    fn test_single_month() {
        let params = LoanParams {
            principal: 10_000.0,
            annual_rate_pct: 12.0,
            term_months: 1,
        };
        let result = calculate_loan(&params);
        // monthly_rate = 0.01; payment = 10000 * 0.01 / (1 - (1.01)^-1)
        //   = 100 / (1 - 1/1.01) = 100 / (0.01/1.01) = 100 * 1.01/0.01 = 10_100
        assert!(approx(result.monthly_payment, 10_100.0, 1e-6));
        assert_eq!(result.amortization.len(), 1);
    }

    /// Total payment = monthly_payment * term (within rounding tolerance).
    #[test]
    fn test_total_payments_consistency() {
        let params = LoanParams {
            principal: 200_000.0,
            annual_rate_pct: 4.5,
            term_months: 240,
        };
        let result = calculate_loan(&params);
        let computed_total = result.monthly_payment * 240.0;
        assert!(
            approx(result.total_payment, computed_total, 1e-6),
            "total_payment field inconsistent with monthly_payment * months"
        );
    }

    /// Interest portion of each payment should decrease monotonically
    /// (as balance falls, interest shrinks).
    #[test]
    fn test_interest_decreases_over_time() {
        let params = LoanParams {
            principal: 150_000.0,
            annual_rate_pct: 5.0,
            term_months: 180,
        };
        let result = calculate_loan(&params);
        for i in 1..result.amortization.len() {
            let (_, _, interest_prev, _) = result.amortization[i - 1];
            let (_, _, interest_curr, _) = result.amortization[i];
            assert!(
                interest_curr <= interest_prev + 1e-9,
                "Interest should not increase: month {} interest {:.4} > month {} interest {:.4}",
                i,
                interest_curr,
                i - 1,
                interest_prev
            );
        }
    }

    /// After the final payment, the remaining principal balance must be ~0.
    #[test]
    fn test_principal_reduces_to_zero() {
        let params = LoanParams {
            principal: 250_000.0,
            annual_rate_pct: 3.75,
            term_months: 360,
        };
        let result = calculate_loan(&params);
        let (_, _, _, final_balance) = *result.amortization.last().unwrap();
        assert!(
            final_balance.abs() < 0.02,
            "Final balance should be ~$0, got {:.6}",
            final_balance
        );
    }

    /// Parity with COBOL formula: same inputs, same formula, same result.
    /// COBOL: monthly_rate = annual_rate / 12 / 100
    ///        payment = principal * monthly_rate / (1 - (1 + monthly_rate)^-n)
    #[test]
    fn test_parity_with_cobol_formula() {
        let principal = 300_000.0_f64;
        let annual_rate_pct = 6.0_f64;
        let term_months: u32 = 360;

        // Replicate COBOL COMPUTE statements verbatim
        let monthly_rate = annual_rate_pct / 12.0 / 100.0;
        let power_factor = (1.0 + monthly_rate).powi(-(term_months as i32));
        let denom = 1.0 - power_factor;
        let cobol_payment = principal * monthly_rate / denom;

        let params = LoanParams {
            principal,
            annual_rate_pct,
            term_months,
        };
        let result = calculate_loan(&params);

        assert!(
            approx(result.monthly_payment, cobol_payment, 1e-10),
            "Rust result {:.10} does not match COBOL formula {:.10}",
            result.monthly_payment,
            cobol_payment
        );
    }

    /// High interest edge case: 20% annual rate, 24-month term.
    #[test]
    fn test_high_interest_rate() {
        let params = LoanParams {
            principal: 10_000.0,
            annual_rate_pct: 20.0,
            term_months: 24,
        };
        let result = calculate_loan(&params);
        // Verify formula produces a positive, finite payment
        assert!(result.monthly_payment > 0.0);
        assert!(result.monthly_payment.is_finite());
        // Total interest should be positive and substantial
        assert!(result.total_interest > 0.0);
        // Sanity: total interest less than principal for 2-year term at 20%
        assert!(result.total_interest < params.principal);
        // Final balance should be near zero
        let (_, _, _, final_balance) = *result.amortization.last().unwrap();
        assert!(
            final_balance.abs() < 0.02,
            "High-rate final balance not ~0: {:.6}",
            final_balance
        );
    }

    /// Amortization schedule length matches term_months.
    #[test]
    fn test_amortization_schedule_length() {
        let params = LoanParams {
            principal: 50_000.0,
            annual_rate_pct: 7.5,
            term_months: 60,
        };
        let result = calculate_loan(&params);
        assert_eq!(result.amortization.len(), 60);
    }

    /// Total interest must be positive for any non-zero rate.
    #[test]
    fn test_total_interest_positive_for_nonzero_rate() {
        let params = LoanParams {
            principal: 100_000.0,
            annual_rate_pct: 0.01,
            term_months: 120,
        };
        let result = calculate_loan(&params);
        assert!(result.total_interest > 0.0);
    }
}
