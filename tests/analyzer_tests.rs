/// Integration tests for the COBOL analyzer against the real cobol/account_balance.cob file.

#[test]
fn test_analyze_real_cobol_file() {
    let source = std::fs::read_to_string("cobol/account_balance.cob")
        .expect("cobol/account_balance.cob must exist");
    let report = rustforge::cobol_analyzer::analyze(&source, 0);

    assert!(report.total_lines > 0, "total_lines must be > 0");
    assert!(report.code_lines > 0, "code_lines must be > 0");
    assert!(
        report.code_lines <= report.total_lines,
        "code_lines must not exceed total_lines"
    );
    assert!(report.pic_clauses > 0, "COBOL file must have PIC clauses");
    assert!(
        report.complexity_score >= 0.0,
        "complexity_score must be >= 0"
    );
    assert!(
        report.complexity_score <= 10.0,
        "complexity_score must be <= 10"
    );
    assert!(
        report.estimated_rust_lines < report.code_lines,
        "Rust is more concise than COBOL"
    );
}

#[test]
fn test_real_file_detects_divisions() {
    let source = std::fs::read_to_string("cobol/account_balance.cob")
        .expect("cobol/account_balance.cob must exist");
    let report = rustforge::cobol_analyzer::analyze(&source, 0);

    // account_balance.cob has all four standard COBOL divisions
    assert!(
        report.divisions.contains(&"IDENTIFICATION".to_string()),
        "must detect IDENTIFICATION DIVISION"
    );
    assert!(
        report.divisions.contains(&"DATA".to_string()),
        "must detect DATA DIVISION"
    );
    assert!(
        report.divisions.contains(&"PROCEDURE".to_string()),
        "must detect PROCEDURE DIVISION"
    );
}

#[test]
fn test_real_file_comp3_vulnerability() {
    let source = std::fs::read_to_string("cobol/account_balance.cob")
        .expect("cobol/account_balance.cob must exist");
    let report = rustforge::cobol_analyzer::analyze(&source, 0);

    // account_balance.cob uses COMP-3 (packed decimal)
    let has_comp3 = report
        .vulnerabilities
        .iter()
        .any(|v| v.contains("COMP-3"));
    assert!(has_comp3, "must detect COMP-3 vulnerability");
}

#[test]
fn test_real_file_report_text_format() {
    let source = std::fs::read_to_string("cobol/account_balance.cob")
        .expect("cobol/account_balance.cob must exist");
    let mut report = rustforge::cobol_analyzer::analyze(&source, 0);
    report.filename = "account_balance.cob".to_string();

    let text = report.to_text();
    assert!(text.contains("RUSTFORGE MIGRATION ANALYSIS REPORT"));
    assert!(text.contains("account_balance.cob"));
    assert!(text.contains("METRICS"));
    assert!(text.contains("RECOMMENDATION"));
}

#[test]
fn test_real_file_report_json_format() {
    let source = std::fs::read_to_string("cobol/account_balance.cob")
        .expect("cobol/account_balance.cob must exist");
    let mut report = rustforge::cobol_analyzer::analyze(&source, 0);
    report.filename = "account_balance.cob".to_string();

    let json = report.to_json();
    assert!(json.contains("\"filename\""));
    assert!(json.contains("account_balance.cob"));
    assert!(json.contains("\"pic_clauses\""));
    assert!(json.starts_with('{'));
    assert!(json.trim_end().ends_with('}'));
}

#[test]
fn test_migration_time_positive() {
    let source = std::fs::read_to_string("cobol/account_balance.cob")
        .expect("cobol/account_balance.cob must exist");
    let report = rustforge::cobol_analyzer::analyze(&source, 0);
    assert!(
        report.migration_time_days >= 0.5,
        "migration_time_days must be at least 0.5"
    );
}
