/// Batch Processor Module
/// Rust translation of COBOL batch_processor.cob
/// Processes transaction records and produces summary totals

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub account_id: u32,
    pub amount: f64,
    pub trans_type: char, // 'D' for debit, 'C' for credit
}

impl Transaction {
    pub fn new(account_id: u32, amount: f64, trans_type: char) -> Self {
        Transaction {
            account_id,
            amount,
            trans_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BatchSummary {
    pub total_debits: f64,
    pub total_credits: f64,
    pub transaction_count: usize,
    pub net_change: f64,
}

impl BatchSummary {
    pub fn new() -> Self {
        BatchSummary {
            total_debits: 0.0,
            total_credits: 0.0,
            transaction_count: 0,
            net_change: 0.0,
        }
    }

    pub fn add_transaction(&mut self, transaction: &Transaction) {
        self.transaction_count += 1;

        match transaction.trans_type {
            'D' | 'd' => {
                self.total_debits += transaction.amount;
            }
            'C' | 'c' => {
                self.total_credits += transaction.amount;
            }
            _ => {} // Ignore invalid types
        }

        self.net_change = self.total_credits - self.total_debits;
    }

    pub fn process_transactions(&mut self, transactions: &[Transaction]) {
        for transaction in transactions {
            self.add_transaction(transaction);
        }
    }
}

/// Process transactions from a file
pub fn process_from_file(filename: &str) -> std::io::Result<BatchSummary> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut summary = BatchSummary::new();

    for line in reader.lines() {
        let line = line?;
        if let Ok(transaction) = parse_transaction_line(&line) {
            summary.add_transaction(&transaction);
        }
    }

    Ok(summary)
}

/// Parse transaction line: "ACCOUNT_ID AMOUNT TYPE"
pub fn parse_transaction_line(line: &str) -> Result<Transaction, String> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();

    if parts.len() < 3 {
        return Err("Invalid transaction format".to_string());
    }

    let account_id = parts[0]
        .parse::<u32>()
        .map_err(|_| "Invalid account ID".to_string())?;
    let amount = parts[1]
        .parse::<f64>()
        .map_err(|_| "Invalid amount".to_string())?;
    let trans_type = parts[2]
        .chars()
        .next()
        .ok_or("Missing transaction type".to_string())?;

    Ok(Transaction::new(account_id, amount, trans_type))
}

/// Display batch summary report
pub fn display_summary(summary: &BatchSummary) {
    println!("====================================");
    println!("BATCH PROCESSOR SUMMARY REPORT");
    println!("====================================");
    println!("Transactions:        {}", summary.transaction_count);
    println!("Total Credits:       ${:.2}", summary.total_credits);
    println!("Total Debits:        ${:.2}", summary.total_debits);
    println!("Net Change:          ${:.2}", summary.net_change);
    println!("====================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction() {
        let t = Transaction::new(12345001, 50000.0, 'C');
        assert_eq!(t.account_id, 12345001);
        assert!((t.amount - 50000.0).abs() < 1e-10);
        assert_eq!(t.trans_type, 'C');
    }

    #[test]
    fn test_batch_summary_initialization() {
        let summary = BatchSummary::new();
        assert_eq!(summary.total_debits, 0.0);
        assert_eq!(summary.total_credits, 0.0);
        assert_eq!(summary.transaction_count, 0);
    }

    #[test]
    fn test_add_credit_transaction() {
        let mut summary = BatchSummary::new();
        let transaction = Transaction::new(12345001, 50000.0, 'C');
        summary.add_transaction(&transaction);

        assert!((summary.total_credits - 50000.0).abs() < 1e-10);
        assert_eq!(summary.total_debits, 0.0);
        assert_eq!(summary.transaction_count, 1);
        assert!((summary.net_change - 50000.0).abs() < 1e-10);
    }

    #[test]
    fn test_add_debit_transaction() {
        let mut summary = BatchSummary::new();
        let transaction = Transaction::new(12345002, 25000.0, 'D');
        summary.add_transaction(&transaction);

        assert_eq!(summary.total_credits, 0.0);
        assert!((summary.total_debits - 25000.0).abs() < 1e-10);
        assert_eq!(summary.transaction_count, 1);
        assert!((summary.net_change - (-25000.0)).abs() < 1e-10);
    }

    #[test]
    fn test_multiple_transactions() {
        let mut summary = BatchSummary::new();
        let transactions = vec![
            Transaction::new(12345001, 100000.0, 'C'),
            Transaction::new(12345002, 75000.0, 'C'),
            Transaction::new(12345003, 100000.0, 'C'),
            Transaction::new(12345004, 50000.0, 'D'),
            Transaction::new(12345005, 75000.0, 'D'),
        ];

        summary.process_transactions(&transactions);

        assert!((summary.total_credits - 275000.0).abs() < 1e-10);
        assert!((summary.total_debits - 125000.0).abs() < 1e-10);
        assert_eq!(summary.transaction_count, 5);
        assert!((summary.net_change - 150000.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_transaction_line() {
        let line = "12345001 50000.00 C";
        let transaction = parse_transaction_line(line).unwrap();

        assert_eq!(transaction.account_id, 12345001);
        assert!((transaction.amount - 50000.0).abs() < 1e-10);
        assert_eq!(transaction.trans_type, 'C');
    }

    #[test]
    fn test_parse_debit_transaction_line() {
        let line = "12345002 25000.50 D";
        let transaction = parse_transaction_line(line).unwrap();

        assert_eq!(transaction.account_id, 12345002);
        assert!((transaction.amount - 25000.50).abs() < 1e-10);
        assert_eq!(transaction.trans_type, 'D');
    }

    #[test]
    fn test_invalid_transaction_line() {
        let line = "invalid data";
        let result = parse_transaction_line(line);
        assert!(result.is_err());
    }
}
