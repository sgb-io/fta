mod complexity;
mod halstead;
mod parse_module;

use halstead::HalsteadMetrics;
use ignore::{DirEntry, WalkBuilder};
use log::warn;
use serde::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Instant;
use swc_ecma_ast::Module;

#[derive(Debug, Deserialize, Default)]
struct FtaConfig {
    extensions: Option<Vec<String>>,
    exclude_filenames: Option<Vec<String>>,
    exclude_directories: Option<Vec<String>>,
    output_limit: Option<usize>,
}

fn read_config(config_path: &str) -> FtaConfig {
    let default_config = FtaConfig {
        extensions: Some(vec![
            ".js".to_string(),
            ".jsx".to_string(),
            ".ts".to_string(),
            ".tsx".to_string(),
        ]),
        exclude_filenames: Some(vec![
            ".d.ts".to_string(),
            ".min.js".to_string(),
            ".bundle.js".to_string(),
        ]),
        exclude_directories: Some(vec![
            "/dist".to_string(),
            "/bin".to_string(),
            "/build".to_string(),
        ]),
        output_limit: Some(100),
    };

    if Path::new(config_path).exists() {
        let mut file = File::open(config_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let provided_config: FtaConfig = serde_json::from_str(&content).unwrap_or_default();

        FtaConfig {
            extensions: {
                let mut extensions = default_config.extensions.unwrap();
                if let Some(mut provided) = provided_config.extensions {
                    extensions.append(&mut provided);
                }
                Some(extensions)
            },
            exclude_filenames: {
                let mut exclude_filenames = default_config.exclude_filenames.unwrap();
                if let Some(mut provided) = provided_config.exclude_filenames {
                    exclude_filenames.append(&mut provided);
                }
                Some(exclude_filenames)
            },
            exclude_directories: provided_config
                .exclude_directories
                .or(default_config.exclude_directories),
            output_limit: provided_config.output_limit.or(default_config.output_limit),
        }
    } else {
        default_config
    }
}

fn is_valid_file(entry: &DirEntry, config: &FtaConfig) -> bool {
    let file_name = entry.path().file_name().unwrap().to_str().unwrap();
    let relative_path = entry.path().to_str().unwrap();

    let valid_extension = config
        .extensions
        .as_ref()
        .map_or(true, |exts| exts.iter().any(|ext| file_name.ends_with(ext)));
    let is_excluded_filename = config.exclude_filenames.as_ref().map_or(false, |exts| {
        exts.iter().any(|ext| file_name.ends_with(ext))
    });
    let is_excluded_directory = config.exclude_directories.as_ref().map_or(false, |dirs| {
        dirs.iter().any(|dir| relative_path.starts_with(dir))
    });

    valid_extension && !is_excluded_filename && !is_excluded_directory
}

fn analyze_file(module: &Module) -> (u32, HalsteadMetrics) {
    let cyclo = complexity::cyclomatic_complexity(module.clone());
    let halstead_metrics = halstead::analyze_module(&module);

    (cyclo, halstead_metrics)
}

fn main() {
    let start = Instant::now();

    // Initialize the logger
    let mut builder = env_logger::Builder::new();

    // Check if debug mode is enabled using an environment variable
    if env::var("DEBUG").is_ok() {
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a project path");
        return;
    }

    let repo_path = &args[1];
    let config_path = format!("{}/fta.json", repo_path);
    let config = read_config(&config_path);

    let walk = WalkBuilder::new(repo_path)
        .git_ignore(true)
        .git_exclude(true)
        .standard_filters(true)
        .build();

    let mut files_found = 0;

    for entry in walk {
        if let Ok(entry) = entry {
            match entry.file_type() {
                Some(file_type) if file_type.is_file() => {
                    if is_valid_file(&entry, &config) {
                        if files_found < config.output_limit.unwrap_or_default() {
                            let file_name = entry.path().display();
                            let source_code = fs::read_to_string(file_name.to_string()).unwrap();

                            match parse_module::parse_module(&source_code) {
                                Ok((module)) => {
                                    let (cyclo, halstead) = analyze_file(&module);
                                    println!(
                                        "{} cyclo: {}, halstead: {:?}",
                                        file_name, cyclo, halstead
                                    );
                                    files_found += 1;
                                }
                                Err(e) => {
                                    warn!("Failed to analyze {}: {:?}", file_name, e);
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

    println!("{} files analyzed in {}s.", files_found, elapsed_rounded);
}
