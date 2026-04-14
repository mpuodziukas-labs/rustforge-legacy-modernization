//! GL Journal Entry Validator — MOD-009
//!
//! Rust translation of `cobol/gl_validator.cob`.
//! Validates double-entry bookkeeping journal entries: the sum of debits must
//! equal the sum of credits, amounts must be positive, and account codes must
//! be non-empty.

/// A single double-entry journal line item.
#[derive(Debug, Clone)]
pub struct JournalEntry {
    /// Unique identifier for this journal entry line
    pub entry_id: u32,
    /// Account code being debited (must be non-empty)
    pub debit_account: String,
    /// Account code being credited (must be non-empty)
    pub credit_account: String,
    /// Transaction amount in dollars (must be > 0)
    pub amount: f64,
    /// Human-readable description of the transaction
    pub description: String,
}

/// Aggregated result of validating a set of journal entries.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// `true` if total debits == total credits within floating-point tolerance
    pub is_balanced: bool,
    /// Sum of all debit amounts in dollars
    pub total_debits: f64,
    /// Sum of all credit amounts in dollars
    pub total_credits: f64,
    /// Total number of validation errors found
    pub error_count: usize,
    /// Human-readable error messages, one per violation
    pub errors: Vec<String>,
}

/// Validates a slice of [`JournalEntry`] records for double-entry balance.
///
/// For each entry [`validate_single_entry`] is called; all errors are collected.
/// After processing all entries the total debits are compared against total
/// credits.  Because each entry carries equal debit and credit amounts (it is
/// a single line with one debit account and one credit account), the journal
/// is balanced when every individual entry is valid and no individual entry
/// has been rejected for a zero or negative amount.
///
/// # Arguments
/// * `entries` - Slice of [`JournalEntry`] to validate
///
/// # Returns
/// A [`ValidationResult`] summarising balances and any errors found.
///
/// # Example
/// ```
/// use rustforge::gl_validator::{JournalEntry, validate_journal};
/// let entries = vec![
///     JournalEntry {
///         entry_id: 1,
///         debit_account:  "1200".to_string(),
///         credit_account: "4000".to_string(),
///         amount: 500.0,
///         description: "Cash sale".to_string(),
///     },
/// ];
/// let result = validate_journal(&entries);
/// assert!(result.is_balanced);
/// assert_eq!(result.error_count, 0);
/// ```
pub fn validate_journal(entries: &[JournalEntry]) -> ValidationResult {
    let mut total_debits = 0.0_f64;
    let mut total_credits = 0.0_f64;
    let mut all_errors: Vec<String> = Vec::new();

    for entry in entries {
        let entry_errors = validate_single_entry(entry);
        if entry_errors.is_empty() {
            // Only accumulate amounts for valid entries
            total_debits += entry.amount;
            total_credits += entry.amount;
        } else {
            all_errors.extend(entry_errors);
        }
    }

    // Check overall balance (tolerant of floating-point rounding)
    let balanced = (total_debits - total_credits).abs() < 1e-9;
    if !balanced {
        all_errors.push(format!(
            "Journal out of balance: debits {:.2} != credits {:.2}",
            total_debits, total_credits
        ));
    }

    let error_count = all_errors.len();
    ValidationResult {
        is_balanced: balanced,
        total_debits,
        total_credits,
        error_count,
        errors: all_errors,
    }
}

/// Validates a single [`JournalEntry`] and returns a list of error messages.
///
/// Checks performed (mirrors COBOL `VALIDATE-SINGLE-ENTRY` paragraph):
/// 1. Amount must be > 0 (negative or zero amounts are rejected)
/// 2. `debit_account` must be non-empty
/// 3. `credit_account` must be non-empty
/// 4. `debit_account` and `credit_account` must not be the same string
///
/// # Arguments
/// * `entry` - The journal entry to inspect
///
/// # Returns
/// An empty `Vec` if the entry is valid, or one or more error strings describing
/// each violation found.
///
/// # Example
/// ```
/// use rustforge::gl_validator::{JournalEntry, validate_single_entry};
/// let bad = JournalEntry {
///     entry_id: 99,
///     debit_account:  "".to_string(),
///     credit_account: "4000".to_string(),
///     amount: -50.0,
///     description: "bad entry".to_string(),
/// };
/// let errors = validate_single_entry(&bad);
/// assert!(errors.len() >= 2); // amount + missing debit
/// ```
pub fn validate_single_entry(entry: &JournalEntry) -> Vec<String> {
    let mut errors = Vec::new();

    if entry.amount <= 0.0 {
        errors.push(format!(
            "Entry {}: amount {:.2} must be positive",
            entry.entry_id, entry.amount
        ));
    }
    if entry.debit_account.trim().is_empty() {
        errors.push(format!(
            "Entry {}: debit_account is missing",
            entry.entry_id
        ));
    }
    if entry.credit_account.trim().is_empty() {
        errors.push(format!(
            "Entry {}: credit_account is missing",
            entry.entry_id
        ));
    }
    if !entry.debit_account.trim().is_empty()
        && !entry.credit_account.trim().is_empty()
        && entry.debit_account.trim() == entry.credit_account.trim()
    {
        errors.push(format!(
            "Entry {}: debit_account and credit_account are the same ({})",
            entry.entry_id,
            entry.debit_account.trim()
        ));
    }

    errors
}

// ---- helpers ----------------------------------------------------------------

#[allow(dead_code)]
fn make_entry(id: u32, debit: &str, credit: &str, amount: f64) -> JournalEntry {
    JournalEntry {
        entry_id: id,
        debit_account: debit.to_string(),
        credit_account: credit.to_string(),
        amount,
        description: format!("Test entry {}", id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced_single_entry() {
        let entries = vec![make_entry(1, "1200", "4000", 500.0)];
        let result = validate_journal(&entries);
        assert!(result.is_balanced);
        assert_eq!(result.error_count, 0);
        assert!((result.total_debits  - 500.0).abs() < 1e-9);
        assert!((result.total_credits - 500.0).abs() < 1e-9);
    }

    #[test]
    fn test_balanced_multiple_entries() {
        let entries = vec![
            make_entry(1, "1200", "4000", 1_000.0),
            make_entry(2, "5100", "2000",   250.0),
            make_entry(3, "1100", "1200",   750.0),
        ];
        let result = validate_journal(&entries);
        assert!(result.is_balanced);
        assert_eq!(result.error_count, 0);
    }

    #[test]
    fn test_negative_amount_rejected() {
        let entries = vec![make_entry(1, "1200", "4000", -100.0)];
        let result = validate_journal(&entries);
        assert!(!result.errors.is_empty());
        // Invalid entry not accumulated, so totals stay 0 — still "balanced" at 0/0
        // but error count is non-zero
        assert!(result.error_count > 0);
    }

    #[test]
    fn test_zero_amount_rejected() {
        let errors = validate_single_entry(&make_entry(5, "1200", "4000", 0.0));
        assert!(!errors.is_empty());
        assert!(errors[0].contains("must be positive"));
    }

    #[test]
    fn test_missing_debit_account() {
        let errors = validate_single_entry(&make_entry(2, "", "4000", 100.0));
        assert!(errors.iter().any(|e| e.contains("debit_account is missing")));
    }

    #[test]
    fn test_missing_credit_account() {
        let errors = validate_single_entry(&make_entry(3, "1200", "", 100.0));
        assert!(errors.iter().any(|e| e.contains("credit_account is missing")));
    }

    #[test]
    fn test_same_debit_and_credit_account() {
        let errors = validate_single_entry(&make_entry(4, "1200", "1200", 100.0));
        assert!(errors.iter().any(|e| e.contains("same")));
    }

    #[test]
    fn test_empty_journal() {
        let result = validate_journal(&[]);
        assert!(result.is_balanced);
        assert_eq!(result.error_count, 0);
        assert_eq!(result.total_debits, 0.0);
        assert_eq!(result.total_credits, 0.0);
    }
}
