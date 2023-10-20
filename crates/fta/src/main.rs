use clap::Parser;
use fta::analyze;
use fta::config::read_config;
use fta::output::generate_output;
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

    // Parse user config
    let config_path = format!("{}/fta.json", &cli.project);
    let config = read_config(&config_path);

    let mut findings = analyze(&cli.project, &config);

    findings.sort_unstable_by(|a, b| b.fta_score.partial_cmp(&a.fta_score).unwrap());

    let elapsed = start.elapsed().as_secs_f64();

    let output = generate_output(
        &findings,
        if cli.json {
            "json".to_string()
        } else {
            cli.format
        },
        &elapsed,
        config.output_limit.unwrap_or_default(),
    );

    println!("{}", output);
}
