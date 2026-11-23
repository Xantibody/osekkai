use std::path::{Path, PathBuf};
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
        /// Report a document that declares no reader state
        #[arg(long)]
        require_state: bool,
    },
    /// The reader state: its vocabulary, and what a declared state changes
    State {
        #[command(subcommand)]
        command: StateCommand,
    },
    /// The density thresholds the lint enforces, to plan the grouping before drafting
    Limits,
    /// The fixed order of steps; with --check, verify a skill text keeps it
    Steps {
        /// A SKILL.md whose <!-- osekkai-step: ... --> anchors must follow the order
        #[arg(long)]
        check: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum StateCommand {
    /// Every dimension and its values
    List,
    /// Guidance and lint rules for a declared state, e.g. `unknown=鈴木,APM purpose=inform`
    Show { declaration: Vec<String> },
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Text,
    Json,
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Lint {
            files,
            format,
            require_state,
        } => {
            let options = osekkai::Options { require_state };
            report(&files, format, |_, src| osekkai::lint_with(src, &options))
        }
        Command::State {
            command: StateCommand::List,
        } => {
            print!("{}", osekkai::state::list());
            ExitCode::SUCCESS
        }
        Command::State {
            command: StateCommand::Show { declaration },
        } => match osekkai::State::parse(&declaration.join(" ")) {
            Ok(state) => {
                print!("{}", state.describe());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::from(2)
            }
        },
        Command::Limits => {
            print!("{}", osekkai::rules::limits());
            ExitCode::SUCCESS
        }
        Command::Steps { check: None } => {
            print!("{}", osekkai::procedure::describe());
            ExitCode::SUCCESS
        }
        Command::Steps { check: Some(path) } => match std::fs::read_to_string(&path) {
            Ok(text) => match osekkai::procedure::check_anchors(&text) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("{}: {e}", path.display());
                    ExitCode::from(1)
                }
            },
            Err(e) => {
                eprintln!("{}: {e}", path.display());
                ExitCode::from(2)
            }
        },
    }
}

#[derive(serde::Serialize)]
struct FileResult<'a> {
    #[serde(rename = "filePath")]
    file_path: &'a str,
    messages: Vec<osekkai::Finding>,
}

/// Run `check` on every file and print its findings; exit 1 when any.
fn report(
    files: &[PathBuf],
    format: Format,
    check: impl Fn(&Path, &str) -> Vec<osekkai::Finding>,
) -> ExitCode {
    let mut results = Vec::new();
    for path in files {
        let src = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}: {e}", path.display());
                return ExitCode::from(2);
            }
        };
        results.push((path.display().to_string(), check(path, &src)));
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
