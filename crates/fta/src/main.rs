mod complexity;
mod halstead;
mod parse_module;

use halstead::HalsteadMetrics;
use ignore::{DirEntry, WalkBuilder};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Display;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
struct FtaConfig {
    extensions: Option<Vec<String>>,
    exclude_filenames: Option<Vec<String>>,
    exclude_directories: Option<Vec<String>>,
    output_limit: Option<usize>,
}

impl Default for FtaConfig {
    fn default() -> Self {
        FtaConfig {
            extensions: Some(vec![
                ".js".into(),
                ".jsx".into(),
                ".ts".into(),
                ".tsx".into(),
            ]),
            exclude_filenames: Some(vec![".d.ts".into(), ".min.js".into(), ".bundle.js".into()]),
            exclude_directories: Some(vec!["/dist".into(), "/bin".into(), "/build".into()]),
            output_limit: Some(100),
        }
    }
}

fn read_config(config_path: &str) -> FtaConfig {
    if Path::new(config_path).exists() {
        let mut file = File::open(config_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        FtaConfig::default()
    }
}

fn is_valid_file(entry: &DirEntry, config: &FtaConfig) -> bool {
    let file_name = entry.path().file_name().unwrap().to_str().unwrap();
    let relative_path = entry.path().to_str().unwrap();

    let valid_extension = config
        .extensions
        .as_ref()
        .unwrap()
        .iter()
        .any(|ext| file_name.ends_with(ext));
    let is_excluded_filename = config
        .exclude_filenames
        .as_ref()
        .unwrap()
        .iter()
        .any(|ext| file_name.ends_with(ext));
    let is_excluded_directory = config
        .exclude_directories
        .as_ref()
        .unwrap()
        .iter()
        .any(|dir| relative_path.starts_with(dir));

    valid_extension && !is_excluded_filename && !is_excluded_directory
}

fn analyze_file(file_name: &Display) -> (u32, HalsteadMetrics) {
    // Read the file
    let source_code = fs::read_to_string(file_name.to_string()).unwrap();

    let module = parse_module::parse_module(&source_code);
    let cyclo = complexity::cyclomatic_complexity(module.clone());

    let halstead_metrics = halstead::analyze_module(&module);

    (cyclo, halstead_metrics)
}

fn main() {
    let start = Instant::now();
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

    let mut file_count = 0;
    let mut files_found = 0;

    for entry in walk {
        if let Ok(entry) = entry {
            match entry.file_type() {
                Some(file_type) if file_type.is_file() => {
                    if is_valid_file(&entry, &config) {
                        files_found += 1;
                        let file_name = entry.path().display();
                        let (cyclo, halstead) = analyze_file(&file_name);
                        println!("{} cyclo: {}, halstead: {:?}", file_name, cyclo, halstead);
                    }

                    file_count += 1;
                    if file_count >= config.output_limit.unwrap_or_default() {
                        break;
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
