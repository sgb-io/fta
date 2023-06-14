use clap::Parser;
use fta::analyze;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    project: String,

    // Output JSON output
    #[arg(long, short, default_value = "table")]
    format: String,
}

pub fn main() {
    let cli = Cli::parse();

    analyze(&cli.project, cli.format)
}
