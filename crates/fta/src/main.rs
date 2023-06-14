use clap::Parser;
use fta::analyze;
use fta::output;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    project: String,

    // Output JSON output
    #[arg(long, short, default_value = "table")]
    format: String,
}

pub fn main() {
    // Start tracking execution time
    let start = Instant::now();

    let cli = Cli::parse();

    let findings = analyze(&cli.project);

    // - Move to a metrics/perf module.
    let elapsed = start.elapsed().as_secs_f64();

    output::output(&findings, &cli.format, &elapsed);
}
