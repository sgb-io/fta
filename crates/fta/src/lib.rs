pub mod config;
mod cyclo;
mod halstead;
pub mod output;
pub mod parse;
mod structs;
mod utils;
mod walk;

use ignore::DirEntry;
use ignore::WalkBuilder;
use log::debug;
use log::warn;
use std::env;
use std::fs;
use structs::{FileData, FtaConfig, HalsteadMetrics};
use swc_ecma_ast::Module;
use swc_ecma_parser::error::Error;
use utils::{check_score_cap_breach, get_assessment, is_valid_file, warn_about_language};
use walk::walk_and_analyze_files;

pub fn analyze_file(module: &Module, line_count: usize) -> (usize, HalsteadMetrics, f64) {
    let cyclo = cyclo::cyclomatic_complexity(module);
    let halstead_metrics = halstead::analyze_module(module);

    let line_count_float = line_count as f64;
    let cyclo_float = cyclo as f64;
    let vocab_float = halstead_metrics.vocabulary_size as f64;

    const MINIMUM_CYCLO: f64 = 1.0;

    let factor = if cyclo_float.ln() < MINIMUM_CYCLO {
        MINIMUM_CYCLO
    } else {
        line_count_float / cyclo_float.ln()
    };

    // Normalization formula based on original research
    // Originates from codehawk-cli
    let absolute_fta_score =
        171.0 - 5.2 * vocab_float.ln() - 0.23 * cyclo_float - 16.2 * factor.ln();
    let mut fta_score = 100.0 - ((absolute_fta_score * 100.0) / 171.0);

    if fta_score < 0.0 {
        fta_score = 0.0;
    }

    (cyclo, halstead_metrics, fta_score)
}

fn analyze_parsed_code(file_name: String, module: Module, line_count: usize) -> FileData {
    let (cyclo, halstead, fta_score) = analyze_file(&module, line_count);
    debug!("{} cyclo: {}, halstead: {:?}", file_name, cyclo, halstead);

    FileData {
        file_name,
        cyclo,
        halstead,
        fta_score,
        line_count,
        assessment: get_assessment(fta_score),
    }
}

fn collect_results(
    entry: &DirEntry,
    repo_path: &str,
    module: Module,
    line_count: usize,
    score_cap: std::option::Option<usize>,
) -> FileData {
    // Parse the source code and run the analysis
    let file_name = entry
        .path()
        .strip_prefix(repo_path)
        .unwrap()
        .display()
        .to_string();
    let file_name_cloned = file_name.clone();
    let file_data = analyze_parsed_code(file_name, module, line_count);

    // Keep a record of the fta_score before moving the FileData
    let fta_score = file_data.fta_score;

    // Check if the score cap is breached
    check_score_cap_breach(file_name_cloned.clone(), fta_score, score_cap);

    file_data
}

fn do_analysis(
    entry: &DirEntry,
    repo_path: &str,
    config: &FtaConfig,
    source_code: &str,
    use_tsx: bool,
) -> Result<FileData, Error> {
    let (result, line_count) = parse::parse_module(
        source_code,
        use_tsx,
        config.include_comments.unwrap_or(false),
    );

    match result {
        Ok(module) => Ok(collect_results(
            entry,
            repo_path,
            module,
            line_count,
            config.score_cap,
        )),
        Err(err) => Err(err),
    }
}

fn process_entry(entry: DirEntry, repo_path: &String, config: &FtaConfig) -> Option<Vec<FileData>> {
    let file_name = entry.path().display();
    let source_code = match fs::read_to_string(file_name.to_string()) {
        Ok(code) => code,
        Err(_) => return None,
    };

    let file_extension = entry
        .path()
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default()
        .to_string();
    let use_tsx = file_extension == "tsx" || file_extension == "jsx";

    let mut file_data_result = do_analysis(&entry, repo_path, &config, &source_code, use_tsx);

    if file_data_result.is_err() {
        warn_about_language(&file_name.to_string(), use_tsx);
        file_data_result = do_analysis(&entry, repo_path, &config, &source_code, !use_tsx);
    }

    if file_data_result.is_err() {
        warn!(
            "Failed to analyze {}: {:?}",
            file_name,
            file_data_result.unwrap_err()
        );
        return None;
    }

    let mut file_data_list: Vec<FileData> = Vec::new();

    // Only include files that are equal to or greater than the `exclude_under` option
    let exclude_under_actual = config.exclude_under.unwrap_or(6);
    match file_data_result {
        Ok(data) if data.line_count > exclude_under_actual => file_data_list.push(data),
        _ => {}
    }

    Some(file_data_list)
}

pub fn analyze(repo_path: &String, config: &FtaConfig) -> Vec<FileData> {
    // Initialize the logger
    let mut builder = env_logger::Builder::new();

    // Check if debug mode is enabled using an environment variable
    if env::var("DEBUG").is_ok() {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    let walk = WalkBuilder::new(repo_path)
        .git_ignore(true)
        .git_exclude(true)
        .standard_filters(true)
        .build();

    walk_and_analyze_files(walk, repo_path, config, process_entry, is_valid_file)
}
