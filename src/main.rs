use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "osekkai", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Lint Markdown files and print findings to stdout
    Lint {
        files: Vec<PathBuf>,
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Text,
    Json,
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Lint { files, format } => lint(&files, format),
    }
}

#[derive(serde::Serialize)]
struct FileResult<'a> {
    #[serde(rename = "filePath")]
    file_path: &'a str,
    messages: Vec<osekkai::Finding>,
}

fn lint(files: &[PathBuf], format: Format) -> ExitCode {
    let mut results = Vec::new();
    for path in files {
        let src = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}: {e}", path.display());
                return ExitCode::from(2);
            }
        };
        results.push((path.display().to_string(), osekkai::lint(&src)));
    }
    let found = results.iter().any(|(_, f)| !f.is_empty());
    match format {
        Format::Text => {
            for (path, findings) in &results {
                for f in findings {
                    println!("{path}:{}:{}: {}: {}", f.line, f.column, f.rule, f.message);
                }
            }
        }
        Format::Json => {
            let out: Vec<FileResult> = results
                .iter()
                .map(|(p, f)| FileResult {
                    file_path: p,
                    messages: f.clone(),
                })
                .collect();
            println!(
                "{}",
                serde_json::to_string(&out).expect("findings serialize")
            );
        }
    }
    if found {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
