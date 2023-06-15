use clap::Parser;
use fta::analyze;
use fta::output;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(required = true)]
    project: String,

    #[arg(long, short, default_value = "table")]
    format: String,

    #[arg(long)]
    json: bool,
}

pub fn main() {
    // Start tracking execution time
    let start = Instant::now();

    let cli = Cli::parse();

    let findings = analyze(&cli.project);

    let elapsed = start.elapsed().as_secs_f64();

    output::output(&findings, &cli.format, &elapsed);
}
