use clap::Parser;
use fta::analyze;
use fta::output;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(required = true, help = "Path to the project to analyze.")]
    project: String,

    #[arg(
        long,
        short,
        default_value = "table",
        value_parser(["table", "csv", "json"]),
        help = "Output format.",
        conflicts_with = "json"
    )]
    format: String,

    #[arg(long, help = "Output as JSON.", conflicts_with = "format")]
    json: bool,
}

pub fn main() {
    // Start tracking execution time
    let start = Instant::now();

    let cli = Cli::parse();

    let mut findings = analyze(&cli.project);

    findings.sort_unstable_by(|a, b| b.fta_score.partial_cmp(&a.fta_score).unwrap());

    let elapsed = start.elapsed().as_secs_f64();

    output::output(
        &findings,
        if cli.json {
            "json".to_string()
        } else {
            cli.format
        },
        &elapsed,
    );
}
