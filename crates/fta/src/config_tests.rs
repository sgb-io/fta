#[cfg(test)]
use crate::config::read_config;
use std::io::Write;
use tempfile::NamedTempFile;

#[allow(dead_code)]
fn create_temp_file(content: &str) -> NamedTempFile {
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "{}", content).unwrap();
    temp_file
}

#[test]
fn test_read_config_with_valid_json() {
    let valid_json = r#"
    {
        "extensions": [".go"],
        "exclude_filenames": [".tmp.go"],
        "exclude_directories": ["/test"],
        "output_limit": 2500,
        "score_cap": 500
    }
    "#;

    let temp_file = create_temp_file(valid_json);
    let path = temp_file.path().to_str().unwrap();

    let config = read_config(path);

    assert_eq!(
        config.extensions,
        Some(vec![
            ".js".to_string(),
            ".jsx".to_string(),
            ".ts".to_string(),
            ".tsx".to_string(),
            ".go".to_string()
        ])
    );
    assert_eq!(
        config.exclude_filenames,
        Some(vec![
            ".d.ts".to_string(),
            ".min.js".to_string(),
            ".bundle.js".to_string(),
            ".tmp.go".to_string()
        ])
    );
    assert_eq!(
        config.exclude_directories,
        Some(vec![
            "/dist".to_string(),
            "/bin".to_string(),
            "/build".to_string(),
            "/test".to_string()
        ])
    );
    assert_eq!(config.output_limit, Some(2500));
    assert_eq!(config.score_cap, Some(500));
}

#[test]
fn test_read_config_with_partial_json() {
    let partial_json = r#"
    {
        "extensions": [".go"],
        "exclude_filenames": [".tmp.go"]
    }
    "#;

    let temp_file = create_temp_file(partial_json);
    let path = temp_file.path().to_str().unwrap();

    let config = read_config(path);

    assert_eq!(
        config.extensions,
        Some(vec![
            ".js".to_string(),
            ".jsx".to_string(),
            ".ts".to_string(),
            ".tsx".to_string(),
            ".go".to_string()
        ])
    );
    assert_eq!(
        config.exclude_filenames,
        Some(vec![
            ".d.ts".to_string(),
            ".min.js".to_string(),
            ".bundle.js".to_string(),
            ".tmp.go".to_string()
        ])
    );
    assert_eq!(
        config.exclude_directories,
        Some(vec![
            "/dist".to_string(),
            "/bin".to_string(),
            "/build".to_string()
        ])
    );
    assert_eq!(config.output_limit, Some(5000));
    assert_eq!(config.score_cap, Some(1000));
}

#[test]
fn test_read_config_with_nonexistent_file() {
    let nonexistent_path = "nonexistent_file.json";

    let config = read_config(nonexistent_path);

    assert_eq!(
        config.extensions,
        Some(vec![
            ".js".to_string(),
            ".jsx".to_string(),
            ".ts".to_string(),
            ".tsx".to_string()
        ])
    );
    assert_eq!(
        config.exclude_filenames,
        Some(vec![
            ".d.ts".to_string(),
            ".min.js".to_string(),
            ".bundle.js".to_string()
        ])
    );
    assert_eq!(
        config.exclude_directories,
        Some(vec![
            "/dist".to_string(),
            "/bin".to_string(),
            "/build".to_string()
        ])
    );
    assert_eq!(config.output_limit, Some(5000));
    assert_eq!(config.score_cap, Some(1000));
}
