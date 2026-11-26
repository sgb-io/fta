use clap::Parser;
use fta::analyze_with_count;
use fta::config::read_config;

#[cfg(feature = "use_output")]
use fta::output::generate_output_with_count;
#[cfg(feature = "use_output")]
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(required = true, help = "Path to the project to analyze")]
    project: String,

    #[arg(long, short, help = "Path to config file")]
    config_path: Option<String>,

    #[arg(
        long,
        short,
        default_value = "table",
        value_parser(["table", "csv", "json"]),
        help = "Output format (default: table)",
        conflicts_with = "json"
    )]
    format: String,

    #[arg(long, help = "Output as JSON.", conflicts_with = "format")]
    json: bool,

    #[arg(
        long,
        short,
        help = "Maximum number of files to include in the table output (only applies when using table output) (default: 5000)"
    )]
    output_limit: Option<usize>,

    #[arg(
        long,
        short,
        help = "Maximum FTA score which will cause FTA to throw (default: 1000)"
    )]
    score_cap: Option<usize>,

    #[arg(
        long,
        short,
        help = "Whether to include code comments when analysing (default: false)"
    )]
    include_comments: Option<bool>,

    #[arg(
        long,
        short,
        help = "Minimum number of lines of code for files to be included in output (default: 6)"
    )]
    exclude_under: Option<usize>,
}

pub fn main() {
    // Start tracking execution time
    #[cfg(feature = "use_output")]
    let start = Instant::now();

    let cli = Cli::parse();

    // Resolve the fta.json path, which can optionally be used-supplied
    let (config_path, path_specified_by_user) = match cli.config_path {
        Some(config_path_arg) => (config_path_arg, true),
        None => (format!("{}/fta.json", cli.project), false),
    };

    // Resolve the input config. Optionally adds fta.json values to the default config.
    let mut config = match read_config(config_path, path_specified_by_user) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    // Override config with CLI args where allowed + values are provided
    if let Some(value) = cli.output_limit {
        config.output_limit = value;
    }
    if let Some(value) = cli.score_cap {
        config.score_cap = value;
    }
    if let Some(value) = cli.include_comments {
        config.include_comments = value;
    }
    if let Some(value) = cli.exclude_under {
        config.exclude_under = value;
    }

    let analysis = analyze_with_count(&cli.project, &config);

    #[cfg(feature = "use_output")]
    {
        let (mut findings, analyzed_count) = analysis;
        findings.sort_unstable_by(|a, b| b.fta_score.partial_cmp(&a.fta_score).unwrap());

        let output = generate_output_with_count(
            &findings,
            analyzed_count,
            if cli.json { "json" } else { &cli.format },
            start.elapsed().as_secs_f64(),
            config.output_limit,
        );

        println!("{}", output);
    }

    #[cfg(not(feature = "use_output"))]
    drop(analysis);
}
