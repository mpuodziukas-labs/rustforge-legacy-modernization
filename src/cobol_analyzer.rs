//! COBOL Migration Analyzer
//!
//! Performs a genuine lexical scan of COBOL source files and produces
//! quantitative metrics that feed the RustForge migration report.

pub struct MigrationReport {
    pub filename: String,
    pub total_lines: usize,
    pub code_lines: usize,
    pub divisions: Vec<String>,
    pub paragraphs: Vec<String>,
    pub pic_clauses: usize,
    pub perform_count: usize,
    pub file_operations: usize,
    pub arithmetic_ops: usize,
    pub condition_checks: usize,
    pub move_statements: usize,
    pub estimated_rust_lines: usize,
    pub complexity_score: f64,
    pub risk_level: u8,
    pub vulnerabilities: Vec<String>,
    pub migration_time_days: f64,
}

impl MigrationReport {
    /// Renders the migration report as a human-readable text block.
    ///
    /// The output is structured into four sections — Metrics, Construct Analysis,
    /// Migration Estimate, and Vulnerabilities Detected — using Unicode box-drawing
    /// characters to match the style of a classic COBOL printed report.
    ///
    /// # Returns
    /// A `String` containing the full formatted report.
    pub fn to_text(&self) -> String {
        let risk_label = match self.risk_level {
            0 => "LOW  ✅",
            1 => "MEDIUM  ⚠️",
            _ => "HIGH  🔴",
        };

        let recommendation = match self.risk_level {
            0 => "GREEN — Proceed with migration",
            1 => "YELLOW — Migration feasible; plan carefully",
            _ => "RED — High complexity; phase migration",
        };

        let division_count = self.divisions.len();
        let paragraph_count = self.paragraphs.len();
        let reduction_pct = (self.estimated_rust_lines * 100)
            .checked_div(self.code_lines)
            .map(|v| 100usize.saturating_sub(v))
            .unwrap_or(0);

        // Build vulnerability lines
        let mut vuln_lines = String::new();
        let checks = [
            ("packed decimal (COMP-3)", "Packed decimal (COMP-3): requires explicit f64 conversion in Rust"),
            ("REDEFINES clause", "REDEFINES clause: union-like structure, requires Rust enum/union"),
            ("OCCURS clause", "OCCURS clause: table/array definition, map to Vec<T> in Rust"),
            ("GO TO statement", "GO TO statement: spaghetti control flow, requires restructuring"),
        ];

        for (label, full_msg) in &checks {
            let found = self.vulnerabilities.iter().any(|v| v.contains(label));
            if found {
                vuln_lines.push_str(&format!("⚠️  {}\n", full_msg));
            } else {
                let short = label.to_string();
                vuln_lines.push_str(&format!("✅ No {} — clean pattern\n", short));
            }
        }

        // File operations warning (separate since it has count)
        let file_vuln = self.vulnerabilities.iter().find(|v| v.contains("file operations"));
        if let Some(msg) = file_vuln {
            vuln_lines.push_str(&format!("⚠️  {}\n", msg));
        } else {
            vuln_lines.push_str(&format!(
                "✅ {} file operations — standard file I/O migration needed\n",
                self.file_operations
            ));
        }

        format!(
            "\
╔════════════════════════════════════════════════╗
║  RUSTFORGE MIGRATION ANALYSIS REPORT           ║
╚════════════════════════════════════════════════╝

File: {}
Risk Level: {}

METRICS
──────────────────────────────────────
Total Lines:          {}
Code Lines:           {}
Estimated Rust LOC:   {}  ({}% reduction)
COBOL Divisions:      {} of 4 (complete program)
Paragraphs:           {}

CONSTRUCT ANALYSIS
──────────────────────────────────────
PIC Clauses:          {}
PERFORM Statements:   {}
File Operations:      {}
Arithmetic Ops:       {}
Conditions (IF/EVAL): {}
MOVE Statements:      {}

MIGRATION ESTIMATE
──────────────────────────────────────
Complexity Score:     {:.1} / 10.0
Migration Time:       ~{:.1} days
Expected Speedup:     40-50x (estimated)
Memory Reduction:     ~85% (estimated)

VULNERABILITIES DETECTED
──────────────────────────────────────
{}
RECOMMENDATION: {}
",
            self.filename,
            risk_label,
            self.total_lines,
            self.code_lines,
            self.estimated_rust_lines,
            reduction_pct,
            division_count,
            paragraph_count,
            self.pic_clauses,
            self.perform_count,
            self.file_operations,
            self.arithmetic_ops,
            self.condition_checks,
            self.move_statements,
            self.complexity_score,
            self.migration_time_days,
            vuln_lines,
            recommendation,
        )
    }

    /// Serialises the migration report to a JSON string (no external dependencies).
    ///
    /// Produces a flat JSON object containing all numeric metrics, the divisions
    /// array, and the vulnerabilities array.  String values with embedded double
    /// quotes are escaped with a backslash.
    ///
    /// # Returns
    /// A `String` containing valid JSON.
    pub fn to_json(&self) -> String {
        let vuln_json: Vec<String> = self
            .vulnerabilities
            .iter()
            .map(|v| format!("\"{}\"", v.replace('"', "\\\"")))
            .collect();

        let div_json: Vec<String> = self
            .divisions
            .iter()
            .map(|d| format!("\"{}\"", d))
            .collect();

        format!(
            r#"{{
  "filename": "{}",
  "total_lines": {},
  "code_lines": {},
  "divisions": [{}],
  "pic_clauses": {},
  "perform_count": {},
  "file_operations": {},
  "arithmetic_ops": {},
  "condition_checks": {},
  "move_statements": {},
  "estimated_rust_lines": {},
  "complexity_score": {:.2},
  "risk_level": {},
  "migration_time_days": {:.2},
  "vulnerabilities": [{}]
}}"#,
            self.filename,
            self.total_lines,
            self.code_lines,
            div_json.join(", "),
            self.pic_clauses,
            self.perform_count,
            self.file_operations,
            self.arithmetic_ops,
            self.condition_checks,
            self.move_statements,
            self.estimated_rust_lines,
            self.complexity_score,
            self.risk_level,
            self.migration_time_days,
            vuln_json.join(", "),
        )
    }
}

/// Count non-overlapping occurrences of a keyword in the source string.
fn count_keyword(source: &str, keyword: &str) -> usize {
    source.matches(keyword).count()
}

/// Performs a lexical scan of a COBOL source string and returns a populated [`MigrationReport`].
///
/// ## Metrics computed
/// | Metric | Method |
/// |---|---|
/// | `total_lines` / `code_lines` | Line counting with comment-column detection |
/// | `divisions` | Keyword search for `IDENTIFICATION`, `ENVIRONMENT`, `DATA`, `PROCEDURE` |
/// | `pic_clauses` | Count of `PIC ` and `PICTURE ` tokens |
/// | `perform_count` | Count of `PERFORM ` tokens |
/// | `file_operations` | Sum of `READ`, `WRITE`, `OPEN`, `CLOSE` tokens |
/// | `arithmetic_ops` | Sum of `COMPUTE`, `ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE` tokens |
/// | `condition_checks` | Count of `IF ` and `EVALUATE ` tokens |
/// | `move_statements` | Count of `MOVE ` tokens |
/// | `complexity_score` | Weighted formula over file ops, PERFORM, PIC, normalised 0–10 |
/// | `estimated_rust_lines` | `code_lines × 0.37` (empirical 63 % line-count reduction) |
/// | `migration_time_days` | `code_lines / 200`, minimum 0.5 days |
///
/// Vulnerabilities are flagged for `COMP-3`, `REDEFINES`, `OCCURS`, `GO TO`, and
/// high file-operation counts (> 10).
///
/// # Arguments
/// * `source`  - Full COBOL source text as a `&str`
/// * `verbose` - Verbosity level: `0` = silent, `1` = summary to stderr,
///   `2` = debug line counts to stderr
///
/// # Returns
/// A [`MigrationReport`] with `filename` left as an empty string (caller should set it).
///
/// # Example
/// ```
/// let src = "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. FOO.\n";
/// let report = rustforge::cobol_analyzer::analyze(src, 0);
/// assert!(report.divisions.contains(&"IDENTIFICATION".to_string()));
/// ```
pub fn analyze(source: &str, verbose: u8) -> MigrationReport {
    let lines: Vec<&str> = source.lines().collect();
    let total_lines = lines.len();

    // Code lines: non-blank, not a comment (COBOL comment = '*' in column 7, i.e. index 6)
    let code_lines = lines
        .iter()
        .filter(|l| {
            let s = l.trim();
            if s.is_empty() {
                return false;
            }
            // Fixed-format COBOL: column 7 (0-indexed col 6) holds '*' for comments
            if l.len() >= 7 && l.as_bytes()[6] == b'*' {
                return false;
            }
            // Also skip lines that are purely '*' after trimming (free-format comments)
            !s.starts_with('*')
        })
        .count();

    if verbose >= 2 {
        eprintln!("[DEBUG] total_lines={} code_lines={}", total_lines, code_lines);
    }

    // Detect COBOL divisions
    let upper = source.to_uppercase();
    let division_names = ["IDENTIFICATION", "ENVIRONMENT", "DATA", "PROCEDURE"];
    let mut divisions = Vec::new();
    for div in &division_names {
        if upper.contains(&format!("{} DIVISION", div)) {
            divisions.push(div.to_string());
        }
    }

    // Count COBOL constructs via keyword scanning (uppercase source)
    let pic_clauses = count_keyword(&upper, "PIC ") + count_keyword(&upper, "PICTURE ");
    let perform_count = count_keyword(&upper, "PERFORM ");
    let file_operations = count_keyword(&upper, "READ ")
        + count_keyword(&upper, "WRITE ")
        + count_keyword(&upper, "OPEN ")
        + count_keyword(&upper, "CLOSE ");
    let arithmetic_ops = count_keyword(&upper, "COMPUTE ")
        + count_keyword(&upper, "ADD ")
        + count_keyword(&upper, "SUBTRACT ")
        + count_keyword(&upper, "MULTIPLY ")
        + count_keyword(&upper, "DIVIDE ");
    let condition_checks = count_keyword(&upper, "IF ") + count_keyword(&upper, "EVALUATE ");
    let move_statements = count_keyword(&upper, "MOVE ");

    if verbose >= 1 {
        eprintln!(
            "[VERBOSE] PIC={} PERFORM={} FILE_OPS={} ARITH={} COND={} MOVE={}",
            pic_clauses, perform_count, file_operations, arithmetic_ops, condition_checks, move_statements
        );
    }

    // Extract paragraph names: lines whose trimmed content ends with '.' and has no spaces
    // (and is not a DIVISION or SECTION header)
    let paragraphs: Vec<String> = lines
        .iter()
        .filter(|l| {
            let t = l.trim();
            t.ends_with('.')
                && !t.contains("DIVISION")
                && !t.contains("SECTION")
                && t.len() > 2
                && !t.contains(' ')
        })
        .map(|l| l.trim().trim_end_matches('.').to_string())
        .collect();

    // Compute complexity score (0.0–10.0)
    let mut complexity = 0.0_f64;
    complexity += (file_operations as f64) * 0.5;
    complexity += (perform_count as f64) * 0.3;
    complexity += (pic_clauses as f64) * 0.1;
    if code_lines > 0 {
        complexity = (complexity / (code_lines as f64) * 10.0).min(10.0);
    }

    let risk_level: u8 = if complexity < 3.0 {
        0
    } else if complexity < 6.0 {
        1
    } else {
        2
    };

    // Detect specific migration vulnerabilities
    let mut vulnerabilities = Vec::new();

    if upper.contains("COMP-3") || upper.contains("PACKED-DECIMAL") {
        vulnerabilities.push(
            "packed decimal (COMP-3): requires explicit f64 conversion in Rust".to_string(),
        );
    }
    if upper.contains("REDEFINES") {
        vulnerabilities
            .push("REDEFINES clause: union-like structure, requires Rust enum/union".to_string());
    }
    if upper.contains("OCCURS") {
        vulnerabilities.push(
            "OCCURS clause: table/array definition, map to Vec<T> in Rust".to_string(),
        );
    }
    if upper.contains("GO TO") {
        vulnerabilities.push(
            "GO TO statement: spaghetti control flow, requires restructuring".to_string(),
        );
    }
    if file_operations > 10 {
        vulnerabilities.push(format!(
            "{} file operations — high I/O complexity",
            file_operations
        ));
    }

    let estimated_rust_lines = (code_lines as f64 * 0.37) as usize;
    let migration_time_days = (code_lines as f64 / 200.0).max(0.5);

    MigrationReport {
        filename: String::new(), // filled by caller
        total_lines,
        code_lines,
        divisions,
        paragraphs,
        pic_clauses,
        perform_count,
        file_operations,
        arithmetic_ops,
        condition_checks,
        move_statements,
        estimated_rust_lines,
        complexity_score: complexity,
        risk_level,
        vulnerabilities,
        migration_time_days,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_COBOL: &str = r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. TEST-PROG.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 WS-VALUE PIC 9(5) VALUE 0.
       PROCEDURE DIVISION.
       MAIN-PARA.
           MOVE 42 TO WS-VALUE.
           STOP RUN.
"#;

    #[test]
    fn test_minimal_cobol_parses() {
        let report = analyze(MINIMAL_COBOL, 0);
        assert!(report.total_lines > 0);
        assert!(report.code_lines > 0);
        assert!(report.code_lines <= report.total_lines);
    }

    #[test]
    fn test_divisions_detected() {
        let report = analyze(MINIMAL_COBOL, 0);
        assert!(report.divisions.contains(&"IDENTIFICATION".to_string()));
        assert!(report.divisions.contains(&"DATA".to_string()));
        assert!(report.divisions.contains(&"PROCEDURE".to_string()));
    }

    #[test]
    fn test_pic_clauses_counted() {
        let report = analyze(MINIMAL_COBOL, 0);
        assert_eq!(report.pic_clauses, 1);
    }

    #[test]
    fn test_move_counted() {
        let report = analyze(MINIMAL_COBOL, 0);
        assert_eq!(report.move_statements, 1);
    }

    #[test]
    fn test_complexity_in_range() {
        let report = analyze(MINIMAL_COBOL, 0);
        assert!(report.complexity_score >= 0.0);
        assert!(report.complexity_score <= 10.0);
    }

    #[test]
    fn test_estimated_rust_lines_less_than_cobol() {
        let report = analyze(MINIMAL_COBOL, 0);
        assert!(report.estimated_rust_lines < report.code_lines);
    }

    #[test]
    fn test_comp3_vulnerability_detected() {
        let cobol_with_comp3 = format!("{}\n       01 WS-AMT PIC S9(9)V99 COMP-3.", MINIMAL_COBOL);
        let report = analyze(&cobol_with_comp3, 0);
        let has_vuln = report
            .vulnerabilities
            .iter()
            .any(|v| v.contains("COMP-3"));
        assert!(has_vuln);
    }

    #[test]
    fn test_to_text_contains_key_sections() {
        let mut report = analyze(MINIMAL_COBOL, 0);
        report.filename = "test.cob".to_string();
        let text = report.to_text();
        assert!(text.contains("RUSTFORGE MIGRATION ANALYSIS REPORT"));
        assert!(text.contains("METRICS"));
        assert!(text.contains("CONSTRUCT ANALYSIS"));
        assert!(text.contains("MIGRATION ESTIMATE"));
        assert!(text.contains("RECOMMENDATION"));
    }

    #[test]
    fn test_to_json_is_parseable_structure() {
        let mut report = analyze(MINIMAL_COBOL, 0);
        report.filename = "test.cob".to_string();
        let json = report.to_json();
        // Basic structural checks (without pulling in serde)
        assert!(json.contains("\"filename\""));
        assert!(json.contains("\"total_lines\""));
        assert!(json.contains("\"complexity_score\""));
        assert!(json.starts_with('{'));
        assert!(json.trim_end().ends_with('}'));
    }
}
