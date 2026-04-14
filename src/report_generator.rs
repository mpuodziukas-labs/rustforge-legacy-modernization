/// Report Generator Module
/// Rust translation of COBOL report_generator.cob
/// Generates formatted transaction reports

use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct TransactionLine {
    pub account_id: u32,
    pub amount: f64,
    pub trans_type: String,
    pub balance: f64,
}

impl TransactionLine {
    pub fn new(account_id: u32, amount: f64, trans_type: &str, balance: f64) -> Self {
        TransactionLine {
            account_id,
            amount,
            trans_type: trans_type.to_string(),
            balance,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{:>10} {:>12.2} {:>8} {:>12.2}",
            self.account_id, self.amount, self.trans_type, self.balance
        )
    }
}

pub struct Report {
    pub title: String,
    pub date: String,
    pub transactions: Vec<TransactionLine>,
    pub grand_total: f64,
}

impl Report {
    pub fn new(title: &str, date: &str) -> Self {
        Report {
            title: title.to_string(),
            date: date.to_string(),
            transactions: Vec::new(),
            grand_total: 0.0,
        }
    }

    pub fn add_transaction(&mut self, transaction: TransactionLine) {
        self.grand_total += transaction.amount;
        self.transactions.push(transaction);
    }

    pub fn format_header(&self) -> String {
        format!("{} - {}", self.title, self.date)
    }

    pub fn format_column_headers(&self) -> String {
        "Account     Amount      Type    Balance".to_string()
    }

    pub fn format_separator(&self) -> String {
        "=".repeat(50)
    }

    pub fn format_footer(&self) -> String {
        format!(
            "Total Transactions: {}    Grand Total: ${:.2}",
            self.transactions.len(),
            self.grand_total
        )
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str(&self.format_header());
        output.push('\n');
        output.push_str(&self.format_separator());
        output.push('\n');
        output.push_str(&self.format_column_headers());
        output.push('\n');
        output.push_str(&self.format_separator());
        output.push('\n');

        for transaction in &self.transactions {
            output.push_str(&transaction.format());
            output.push('\n');
        }

        output.push_str(&self.format_separator());
        output.push('\n');
        output.push_str(&self.format_footer());
        output.push('\n');

        output
    }

    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(self.to_string().as_bytes())?;
        Ok(())
    }

    pub fn display(&self) {
        println!("{}", self.to_string());
    }
}

/// Generate a sample report for testing
pub fn generate_sample_report() -> Report {
    let mut report = Report::new("TRANSACTION REPORT", "04-13-2026");

    report.add_transaction(TransactionLine::new(12345001, 50000.0, "CREDIT", 50000.0));
    report.add_transaction(TransactionLine::new(12345002, 75000.0, "CREDIT", 125000.0));
    report.add_transaction(TransactionLine::new(12345003, 100000.0, "CREDIT", 225000.0));
    report.add_transaction(TransactionLine::new(12345004, 50000.0, "DEBIT", 175000.0));
    report.add_transaction(TransactionLine::new(12345005, 75000.0, "DEBIT", 100000.0));

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction_line() {
        let line = TransactionLine::new(12345001, 50000.0, "CREDIT", 50000.0);
        assert_eq!(line.account_id, 12345001);
        assert!((line.amount - 50000.0).abs() < 1e-10);
        assert_eq!(line.trans_type, "CREDIT");
        assert!((line.balance - 50000.0).abs() < 1e-10);
    }

    #[test]
    fn test_transaction_format() {
        let line = TransactionLine::new(12345001, 50000.0, "CREDIT", 50000.0);
        let formatted = line.format();
        assert!(formatted.contains("12345001"));
        assert!(formatted.contains("50000.00"));
        assert!(formatted.contains("CREDIT"));
    }

    #[test]
    fn test_create_report() {
        let report = Report::new("TEST REPORT", "04-13-2026");
        assert_eq!(report.title, "TEST REPORT");
        assert_eq!(report.date, "04-13-2026");
        assert_eq!(report.transactions.len(), 0);
        assert_eq!(report.grand_total, 0.0);
    }

    #[test]
    fn test_add_transaction_to_report() {
        let mut report = Report::new("TEST REPORT", "04-13-2026");
        let line = TransactionLine::new(12345001, 50000.0, "CREDIT", 50000.0);
        report.add_transaction(line);

        assert_eq!(report.transactions.len(), 1);
        assert!((report.grand_total - 50000.0).abs() < 1e-10);
    }

    #[test]
    fn test_report_with_multiple_transactions() {
        let report = generate_sample_report();
        assert_eq!(report.transactions.len(), 5);
        // 50000+75000+100000+50000+75000 = 350000
        assert!((report.grand_total - 350000.0).abs() < 1e-10);
    }

    #[test]
    fn test_report_header_format() {
        let report = Report::new("TRANSACTION REPORT", "04-13-2026");
        let header = report.format_header();
        assert_eq!(header, "TRANSACTION REPORT - 04-13-2026");
    }

    #[test]
    fn test_report_column_headers() {
        let report = Report::new("TEST REPORT", "04-13-2026");
        let columns = report.format_column_headers();
        assert!(columns.contains("Account"));
        assert!(columns.contains("Amount"));
        assert!(columns.contains("Type"));
        assert!(columns.contains("Balance"));
    }

    #[test]
    fn test_report_string_output() {
        let report = generate_sample_report();
        let output = report.to_string();
        assert!(output.contains("TRANSACTION REPORT"));
        assert!(output.contains("04-13-2026"));
        assert!(output.contains("12345001"));
        assert!(output.contains("Total Transactions: 5"));
    }

    #[test]
    fn test_report_footer_format() {
        let mut report = Report::new("TEST REPORT", "04-13-2026");
        report.add_transaction(TransactionLine::new(12345001, 100000.0, "CREDIT", 100000.0));
        report.add_transaction(TransactionLine::new(12345002, 50000.0, "DEBIT", 50000.0));

        let footer = report.format_footer();
        assert!(footer.contains("Total Transactions: 2"));
        assert!(footer.contains("Grand Total: $150000.00"));
    }
}
