pub mod complexity;
mod config;
mod halstead;
mod parse_module;
mod structs;

use config::read_config;
use ignore::WalkBuilder;
use log::debug;
use log::warn;
use std::cmp::max;
use std::env;
use std::fs;
use std::time::Instant;

use crate::structs::{FileData, FtaConfig, HalsteadMetrics};
use globset::{Glob, GlobSetBuilder};
use ignore::DirEntry;
use swc_ecma_ast::Module;

fn is_excluded_filename(file_name: &str, patterns: &[String]) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let glob = Glob::new(pattern).unwrap();
        builder.add(glob);
    }

    let glob_set = builder.build().unwrap();

    glob_set.is_match(file_name)
}

fn is_valid_file(repo_path: &String, entry: &DirEntry, config: &FtaConfig) -> bool {
    let file_name = entry.path().file_name().unwrap().to_str().unwrap();
    let relative_path = entry
        .path()
        .strip_prefix(repo_path)
        .unwrap()
        .to_str()
        .unwrap();

    let valid_extension = config
        .extensions
        .as_ref()
        .map_or(true, |exts| exts.iter().any(|ext| file_name.ends_with(ext)));

    let is_excluded_filename = config
        .exclude_filenames
        .as_ref()
        .map_or(false, |patterns| is_excluded_filename(file_name, patterns));

    let is_excluded_directory = config.exclude_directories.as_ref().map_or(false, |dirs| {
        dirs.iter().any(|dir| relative_path.starts_with(dir))
    });

    valid_extension && !is_excluded_filename && !is_excluded_directory
}

fn analyze_file(module: &Module, line_count: usize) -> (usize, HalsteadMetrics, f64) {
    let cyclo = complexity::cyclomatic_complexity(module.clone());
    let halstead_metrics = halstead::analyze_module(module);

    let line_count_float = line_count as f64;
    let cyclo_float = cyclo as f64;
    let vocab_float = halstead_metrics.vocabulary_size as f64;

    let factor = if cyclo_float.ln() < 1.0 {
        1.0
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

fn get_assessment(score: f64) -> String {
    if score > 60.0 {
        "(Needs improvement)".to_string()
    } else if score > 50.0 {
        "(Could be better)".to_string()
    } else {
        "OK".to_string()
    }
}

pub fn analyze(repo_path: &String, json: bool) {
    // Initialize the logger
    let mut builder = env_logger::Builder::new();

    // Check if debug mode is enabled using an environment variable
    if env::var("DEBUG").is_ok() {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    // Start tracking execution time
    let start = Instant::now();

    // Parse user config
    let config_path = format!("{}/fta.json", repo_path);
    let config = read_config(&config_path);

    let walk = WalkBuilder::new(repo_path)
        .git_ignore(true)
        .git_exclude(true)
        .standard_filters(true)
        .build();

    let mut files_found = 0;
    let mut file_data_list: Vec<FileData> = Vec::new();

    for entry in walk {
        if let Ok(entry) = entry {
            match entry.file_type() {
                Some(file_type) if file_type.is_file() => {
                    if is_valid_file(repo_path, &entry, &config) {
                        if files_found < config.output_limit.unwrap_or_default() {
                            let file_name = entry.path().display();
                            let source_code = fs::read_to_string(file_name.to_string()).unwrap();

                            match parse_module::parse_module(&source_code) {
                                (Ok(module), line_count) => {
                                    let (cyclo, halstead, fta_score) =
                                        analyze_file(&module, line_count);
                                    debug!(
                                        "{} cyclo: {}, halstead: {:?}",
                                        file_name, cyclo, halstead
                                    );
                                    file_data_list.push(FileData {
                                        file_name: entry
                                            .path()
                                            .strip_prefix(repo_path)
                                            .unwrap()
                                            .display()
                                            .to_string(),
                                        cyclo,
                                        halstead,
                                        fta_score,
                                        line_count,
                                        assessment: get_assessment(fta_score),
                                    });
                                    files_found += 1;

                                    // Exit 1 if score_cap breached
                                    if let Some(score_cap) = config.score_cap {
                                        if fta_score > (score_cap as f64) {
                                            eprintln!("File {} has a score of {}, which is beyond the score cap of {}, exiting.", file_name, fta_score, score_cap);
                                            std::process::exit(1);
                                        }
                                    }
                                }
                                (Err(err), _) => {
                                    warn!("Failed to analyze {}: {:?}", file_name, err);
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
                _ => (),
            }
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    let elapsed_rounded = (elapsed * 10000.0).round() / 10000.0;

    // JSON output - output results as JSON
    if json {
        let json_string = serde_json::to_string(&file_data_list).unwrap();
        println!("{}", json_string);
        std::process::exit(0);
    }

    // Normal output - output results table
    file_data_list.sort_unstable_by(|a, b| b.fta_score.partial_cmp(&a.fta_score).unwrap());

    let mut max_file_name_width = "File".len();
    let mut max_lines_width = "Num. lines".len();
    let mut max_fta_width = "FTA Score (Lower is better)".len();
    let mut max_assessment_width = "Assessment".len();

    for file_data in &file_data_list {
        max_file_name_width = max(max_file_name_width, file_data.file_name.len());
        max_lines_width = max(max_lines_width, file_data.line_count.to_string().len());
        max_fta_width = max(max_fta_width, format!("{:.2}", file_data.fta_score).len());
        max_assessment_width = max(max_assessment_width, file_data.assessment.to_string().len());
    }

    // Add some padding to each column
    max_file_name_width += 2;
    max_lines_width += 2;
    max_fta_width += 2;

    println!(
        "| {} | {} | {} | {} |",
        "-".repeat(max_file_name_width),
        "-".repeat(max_lines_width),
        "-".repeat(max_fta_width),
        "-".repeat(max_assessment_width)
    );
    println!(
        "| {:<f_width$} | {:>c_width$} | {:>h_width$} | {:>a_width$}",
        "File",
        "Num. lines",
        "FTA Score (Lower is better)",
        "Assessment",
        f_width = max_file_name_width,
        c_width = max_lines_width,
        h_width = max_fta_width,
        a_width = max_assessment_width
    );
    println!(
        "| {} | {} | {} | {} |",
        "-".repeat(max_file_name_width),
        "-".repeat(max_lines_width),
        "-".repeat(max_fta_width),
        "-".repeat(max_assessment_width)
    );

    for file_data in file_data_list
        .iter()
        .take(config.output_limit.unwrap_or(100))
    {
        println!(
            "| {:<f_width$} | {:>c_width$} | {:>h_width$.2} | {:>a_width$} |",
            file_data.file_name,
            file_data.line_count,
            file_data.fta_score,
            file_data.assessment,
            f_width = max_file_name_width,
            c_width = max_lines_width,
            h_width = max_fta_width,
            a_width = max_assessment_width
        );
    }
    println!(
        "| {} | {} | {} | {} |",
        "-".repeat(max_file_name_width),
        "-".repeat(max_lines_width),
        "-".repeat(max_fta_width),
        "-".repeat(max_assessment_width)
    );

    println!("{} files analyzed in {}s.", files_found, elapsed_rounded);
}
