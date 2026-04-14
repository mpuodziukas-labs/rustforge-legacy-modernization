use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustforge-analyze")]
#[command(about = "RustForge COBOL Migration Analyzer — assess legacy code for Rust migration")]
struct Cli {
    /// Path to COBOL source file
    file: PathBuf,

    /// Output format: text (default) or json
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Verbosity level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let cli = Cli::parse();

    let source = fs::read_to_string(&cli.file).unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        std::process::exit(1);
    });

    let filename = cli
        .file
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut report = rustforge::cobol_analyzer::analyze(&source, cli.verbose);
    report.filename = filename;

    match cli.format.as_str() {
        "json" => println!("{}", report.to_json()),
        _ => println!("{}", report.to_text()),
    }

    std::process::exit(report.risk_level as i32);
}
