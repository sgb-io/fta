use crate::structs::FtaConfigResolved;
use globset::{Glob, GlobSetBuilder};
use ignore::DirEntry;
use log::warn;
use std::path::Path;

mod tests;

pub fn is_excluded_filename(file_name: &str, patterns: &[String]) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        // If pattern starts with a dot but doesn't contain wildcards,
        // treat it as a suffix pattern by prepending *
        let effective_pattern =
            if pattern.starts_with('.') && !pattern.contains('*') && !pattern.contains('?') {
                format!("*{}", pattern)
            } else {
                pattern.clone()
            };

        let glob = Glob::new(&effective_pattern).unwrap();
        builder.add(glob);
    }

    let glob_set = builder.build().unwrap();

    glob_set.is_match(file_name)
}

/// Check if a relative path should be excluded based on directory exclusion patterns.
/// This function properly handles path segments to avoid false positives and supports
/// both absolute patterns (starting with '/') and relative patterns.
///
/// Examples:
/// - Pattern "node_modules" matches "node_modules/lib.js" and "src/node_modules/lib.js"
///   but NOT "my-node_modules/lib.js"
/// - Pattern "/dist" or "dist" matches "dist/file.js" and "packages/dist/file.js"
/// - Pattern "packages/dist" matches "packages/dist/file.js" but NOT "dist/file.js"
pub fn is_excluded_directory_path(relative_path: &str, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return false;
    }

    let path = Path::new(relative_path);
    let path_components: Vec<&str> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();

    for pattern in patterns {
        // Normalize the pattern by removing leading/trailing slashes
        let normalized_pattern = pattern.trim_start_matches('/').trim_end_matches('/');
        if normalized_pattern.is_empty() {
            continue;
        }

        // Split pattern into components
        let pattern_components: Vec<&str> = normalized_pattern.split('/').collect();

        // Check if this pattern matches any subsequence of the path components
        if pattern_components.len() == 1 {
            // Single component pattern - check if it matches any path component exactly
            if path_components.contains(&pattern_components[0]) {
                return true;
            }
        } else {
            // Multi-component pattern - check for consecutive matches
            if path_components.len() >= pattern_components.len() {
                for i in 0..=(path_components.len() - pattern_components.len()) {
                    let path_slice = &path_components[i..i + pattern_components.len()];
                    if path_slice == pattern_components.as_slice() {
                        return true;
                    }
                }
            }
        }
    }

    false
}

pub fn is_valid_file(repo_path: &String, entry: &DirEntry, config: &FtaConfigResolved) -> bool {
    let file_name = entry.path().file_name().unwrap().to_str().unwrap();
    let relative_path = entry
        .path()
        .strip_prefix(repo_path)
        .unwrap()
        .to_str()
        .unwrap();

    let valid_extension = config.extensions.iter().any(|ext| file_name.ends_with(ext));
    let is_excluded_filename = is_excluded_filename(file_name, &config.exclude_filenames);
    let is_excluded_directory =
        is_excluded_directory_path(relative_path, &config.exclude_directories);

    valid_extension && !is_excluded_filename && !is_excluded_directory
}

pub fn warn_about_language(file_name: &str, use_tsx: bool) {
    let tsx_name = if use_tsx { "j/tsx" } else { "non-j/tsx" };
    let opposite_tsx_name = if use_tsx { "non-j/tsx" } else { "j/tsx" };

    warn!(
        "File {} was interpreted as {} but seems to actually be {}. The file extension may be incorrect.",
        file_name,
        tsx_name,
        opposite_tsx_name
    );
}

pub fn check_score_cap_breach(file_name: String, fta_score: f64, score_cap: usize) {
    // Exit 1 if score_cap breached
    if fta_score > score_cap as f64 {
        eprintln!(
            "File {} has a score of {}, which is beyond the score cap of {}, exiting.",
            file_name, fta_score, score_cap
        );
        std::process::exit(1);
    }
}

pub fn get_assessment(score: f64) -> String {
    if score > 60.0 {
        "Needs improvement".to_string()
    } else if score > 50.0 {
        "Could be better".to_string()
    } else {
        "OK".to_string()
    }
}
