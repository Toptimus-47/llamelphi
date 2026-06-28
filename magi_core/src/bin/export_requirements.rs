//! CLI for exporting project requirements to PDF

use std::path::PathBuf;
use clap::Parser;
use magi_core::infrastructure::doc_generator::export_requirements_to_pdf;

/// Simple command‑line interface to generate a PDF from `Requirements.json`.
#[derive(Parser, Debug)]
#[command(author, version, about = "Export Requirements.json to PDF", long_about = None)]
struct Args {
    /// Path to the Requirements.json file (defaults to project root)
    #[arg(short, long, value_name = "FILE", default_value = "Requirements.json")]
    input: PathBuf,

    /// Output PDF file path (defaults to Requirements.pdf)
    #[arg(short, long, value_name = "FILE", default_value = "Requirements.pdf")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    export_requirements_to_pdf(&args.input, &args.output)?;
    println!("✅ Generated PDF: {}", args.output.display());
    Ok(())
}
