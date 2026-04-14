//! Payroll Calculator — MOD-008
//!
//! Rust translation of `cobol/payroll.cob`.
//! Computes gross pay (with 1.5× overtime over 40 hrs), federal income-tax
//! withholding (bracket-based, Single/MFJ/MFS filers), FICA (Social Security
//! + Medicare), and net pay.

/// Filing status for federal income-tax bracket selection.
#[derive(Debug, Clone, PartialEq)]
pub enum FilingStatus {
    /// Single filer (or Married Filing Separately for bracket purposes)
    Single,
    /// Married Filing Jointly
    MarriedJoint,
    /// Married Filing Separately
    MarriedSeparate,
}

/// Inputs required to compute a payroll period.
#[derive(Debug, Clone)]
pub struct PayrollInput {
    /// Unique numeric employee identifier
    pub employee_id: u32,
    /// Total hours worked in the pay period (must be ≥ 0)
    pub hours_worked: f64,
    /// Regular hourly pay rate in dollars
    pub hourly_rate: f64,
    /// Federal income-tax filing status
    pub filing_status: FilingStatus,
    /// W-4 withholding allowances (currently unused in bracket calc; reserved for future use)
    pub allowances: u32,
}

/// Full payroll calculation result for one pay period.
#[derive(Debug, Clone)]
pub struct PayrollResult {
    /// Total gross pay in dollars
    pub gross_pay: f64,
    /// Regular-time portion of gross pay (hours ≤ 40 × rate)
    pub regular_pay: f64,
    /// Overtime portion of gross pay (hours > 40 × rate × 1.5)
    pub overtime_pay: f64,
    /// Federal income-tax withheld in dollars
    pub federal_tax: f64,
    /// Employee Social Security contribution (6.2 % up to wage base) in dollars
    pub social_security: f64,
    /// Employee Medicare contribution (1.45 %) in dollars
    pub medicare: f64,
    /// Take-home pay after all deductions in dollars
    pub net_pay: f64,
}

// 2024 Social Security wage base
const SS_WAGE_BASE: f64 = 168_600.0;
const SS_RATE: f64 = 0.062;
const MEDICARE_RATE: f64 = 0.0145;

/// Computes the complete payroll for one employee in one pay period.
///
/// Steps mirror the COBOL `CALC-GROSS-PAY`, `CALC-FEDERAL-TAX`, `CALC-FICA`,
/// and `CALC-NET-PAY` paragraphs:
/// 1. Regular pay = min(hours, 40) × rate
/// 2. Overtime pay = max(hours − 40, 0) × rate × 1.5
/// 3. Gross = regular + overtime
/// 4. Federal tax via [`compute_federal_tax`]
/// 5. FICA via [`compute_fica`]
/// 6. Net = gross − federal − SS − Medicare
///
/// # Arguments
/// * `input` - [`PayrollInput`] describing the employee and hours
///
/// # Returns
/// A [`PayrollResult`] with all dollar amounts.
///
/// # Edge cases
/// * Negative `hours_worked` — gross (and all deductions) will be 0 because
///   regular hours are clamped via `min(hours, 40).max(0.0)`.
/// * Zero `hourly_rate` — results in zero gross and zero deductions.
///
/// # Example
/// ```
/// use rustforge::payroll::{PayrollInput, FilingStatus, calculate_payroll};
/// let input = PayrollInput {
///     employee_id: 1,
///     hours_worked: 45.0,
///     hourly_rate: 20.0,
///     filing_status: FilingStatus::Single,
///     allowances: 1,
/// };
/// let result = calculate_payroll(&input);
/// // Regular: 40 × $20 = $800; Overtime: 5 × $20 × 1.5 = $150; Gross = $950
/// assert!((result.gross_pay - 950.0).abs() < 1e-9);
/// ```
pub fn calculate_payroll(input: &PayrollInput) -> PayrollResult {
    let hours = input.hours_worked.max(0.0);
    let regular_hours = hours.min(40.0);
    let overtime_hours = (hours - 40.0).max(0.0);

    let regular_pay = regular_hours * input.hourly_rate;
    let overtime_pay = overtime_hours * input.hourly_rate * 1.5;
    let gross_pay = regular_pay + overtime_pay;

    let federal_tax = compute_federal_tax(gross_pay, &input.filing_status);
    let (social_security, medicare) = compute_fica(gross_pay);
    let net_pay = (gross_pay - federal_tax - social_security - medicare).max(0.0);

    PayrollResult {
        gross_pay,
        regular_pay,
        overtime_pay,
        federal_tax,
        social_security,
        medicare,
        net_pay,
    }
}

/// Computes federal income-tax withholding using 2024 marginal tax brackets.
///
/// Bracket thresholds and rates (annual, per-period gross treated as annual here):
///
/// | Bracket | Up to (Single / MFS) | Up to (MFJ) | Rate |
/// |---------|---------------------|-------------|------|
/// | 1 | $11,600 | $23,200 | 10 % |
/// | 2 | $47,150 | $94,300 | 12 % |
/// | 3 | $100,525 | $201,050 | 22 % |
/// | 4 | $191,950 | $383,900 | 24 % |
/// | 5+ | above | above | 32 % |
///
/// # Arguments
/// * `gross`  - Gross pay for the period in dollars (treated as annual income for brackets)
/// * `status` - [`FilingStatus`] determining which bracket table to apply
///
/// # Returns
/// Federal tax withheld in dollars (≥ 0).
pub fn compute_federal_tax(gross: f64, status: &FilingStatus) -> f64 {
    if gross <= 0.0 {
        return 0.0;
    }

    // Bracket thresholds vary by filing status; MFS uses same thresholds as Single.
    let (b1, b2, b3, b4) = match status {
        FilingStatus::Single | FilingStatus::MarriedSeparate => {
            (11_600.0, 47_150.0, 100_525.0, 191_950.0)
        }
        FilingStatus::MarriedJoint => (23_200.0, 94_300.0, 201_050.0, 383_900.0),
    };

    // Bracket base taxes (cumulative tax at the bottom of each bracket)
    let (t1, t2, t3, t4) = match status {
        FilingStatus::Single | FilingStatus::MarriedSeparate => {
            (1_160.0, 5_426.0, 17_168.50, 40_426.50)
        }
        FilingStatus::MarriedJoint => (2_320.0, 10_852.0, 34_337.0, 80_853.0),
    };

    if gross <= b1 {
        gross * 0.10
    } else if gross <= b2 {
        t1 + (gross - b1) * 0.12
    } else if gross <= b3 {
        t2 + (gross - b2) * 0.22
    } else if gross <= b4 {
        t3 + (gross - b3) * 0.24
    } else {
        t4 + (gross - b4) * 0.32
    }
}

/// Computes employee FICA contributions: Social Security and Medicare.
///
/// Rates (2024):
/// * Social Security: 6.2 % on earnings up to the $168,600 wage base
/// * Medicare: 1.45 % on all earnings (no cap)
///
/// # Arguments
/// * `gross` - Gross pay for the period in dollars
///
/// # Returns
/// `(social_security, medicare)` — both values in dollars, both ≥ 0.
///
/// # Example
/// ```
/// let (ss, med) = rustforge::payroll::compute_fica(1_000.0);
/// assert!((ss  - 62.0).abs()  < 1e-9);
/// assert!((med - 14.5).abs()  < 1e-9);
/// ```
pub fn compute_fica(gross: f64) -> (f64, f64) {
    if gross <= 0.0 {
        return (0.0, 0.0);
    }
    let ss_taxable = gross.min(SS_WAGE_BASE);
    let social_security = ss_taxable * SS_RATE;
    let medicare = gross * MEDICARE_RATE;
    (social_security, medicare)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_input(hours: f64, rate: f64) -> PayrollInput {
        PayrollInput {
            employee_id: 1,
            hours_worked: hours,
            hourly_rate: rate,
            filing_status: FilingStatus::Single,
            allowances: 0,
        }
    }

    // ---- Gross pay / overtime boundary tests ----

    #[test]
    fn test_no_overtime_exactly_40_hours() {
        let result = calculate_payroll(&make_input(40.0, 20.0));
        assert!((result.regular_pay - 800.0).abs() < 1e-9);
        assert!((result.overtime_pay - 0.0).abs() < 1e-9);
        assert!((result.gross_pay - 800.0).abs() < 1e-9);
    }

    #[test]
    fn test_overtime_at_45_hours() {
        let result = calculate_payroll(&make_input(45.0, 20.0));
        // regular: 40 * 20 = 800; overtime: 5 * 20 * 1.5 = 150
        assert!((result.regular_pay  - 800.0).abs() < 1e-9);
        assert!((result.overtime_pay - 150.0).abs() < 1e-9);
        assert!((result.gross_pay    - 950.0).abs() < 1e-9);
    }

    #[test]
    fn test_no_overtime_under_40_hours() {
        let result = calculate_payroll(&make_input(35.0, 15.0));
        assert!((result.regular_pay  - 525.0).abs() < 1e-9);
        assert!((result.overtime_pay - 0.0).abs()   < 1e-9);
    }

    #[test]
    fn test_negative_hours_yields_zero_gross() {
        let result = calculate_payroll(&make_input(-5.0, 20.0));
        assert!((result.gross_pay - 0.0).abs() < 1e-9);
        assert!((result.net_pay   - 0.0).abs() < 1e-9);
    }

    // ---- Federal tax bracket tests ----

    #[test]
    fn test_tax_bracket_1_single() {
        // $5,000 gross → 10 % bracket
        let tax = compute_federal_tax(5_000.0, &FilingStatus::Single);
        assert!((tax - 500.0).abs() < 1e-6);
    }

    #[test]
    fn test_tax_bracket_2_single() {
        // $20,000 → 10 % on first $11,600, then 12 % on remainder
        let expected = 1_160.0 + (20_000.0 - 11_600.0) * 0.12;
        let tax = compute_federal_tax(20_000.0, &FilingStatus::Single);
        assert!((tax - expected).abs() < 1e-6);
    }

    #[test]
    fn test_tax_bracket_3_single() {
        let expected = 5_426.0 + (60_000.0 - 47_150.0) * 0.22;
        let tax = compute_federal_tax(60_000.0, &FilingStatus::Single);
        assert!((tax - expected).abs() < 1e-6);
    }

    #[test]
    fn test_zero_gross_zero_tax() {
        assert!((compute_federal_tax(0.0, &FilingStatus::Single)).abs() < 1e-9);
    }

    // ---- FICA tests ----

    #[test]
    fn test_fica_standard() {
        let (ss, med) = compute_fica(1_000.0);
        assert!((ss  - 62.0).abs()  < 1e-9);
        assert!((med - 14.5).abs()  < 1e-9);
    }

    #[test]
    fn test_fica_ss_wage_base_cap() {
        // At exactly the SS wage base, SS = 168600 * 0.062
        let (ss, _med) = compute_fica(SS_WAGE_BASE);
        assert!((ss - SS_WAGE_BASE * SS_RATE).abs() < 1e-6);

        // Above wage base, SS is capped
        let (ss_above, _) = compute_fica(SS_WAGE_BASE + 10_000.0);
        assert!((ss_above - SS_WAGE_BASE * SS_RATE).abs() < 1e-6);
    }

    // ---- Net-pay parity test (mirrors COBOL formula) ----

    #[test]
    fn test_net_pay_parity() {
        let input = make_input(40.0, 25.0);
        let result = calculate_payroll(&input);

        // Re-derive independently
        let gross = 40.0 * 25.0;
        let federal = compute_federal_tax(gross, &FilingStatus::Single);
        let (ss, med) = compute_fica(gross);
        let expected_net = gross - federal - ss - med;

        assert!((result.gross_pay  - gross).abs()        < 1e-9);
        assert!((result.federal_tax - federal).abs()     < 1e-9);
        assert!((result.social_security - ss).abs()      < 1e-9);
        assert!((result.medicare    - med).abs()         < 1e-9);
        assert!((result.net_pay - expected_net).abs()    < 1e-9);
    }
}
