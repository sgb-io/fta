mod complexity;
mod config;
mod halstead;
mod parse_module;
mod structs;

#[cfg(test)]
mod complexity_tests;
mod config_tests;
mod parse_module_tests;

use clap::Parser;
use fta::analyze;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    project: String,

    // Output JSON output
    #[arg(long)]
    json: bool,
}

pub fn main() {
    let cli = Cli::parse();

    analyze(&cli.project, cli.json)
}
