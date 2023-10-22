#[cfg(test)]
mod tests {
    use crate::config::read_config;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_file(content: &str) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", content).unwrap();
        temp_file
    }

    #[test]
    fn test_read_config_with_valid_json() {
        let valid_json = r#"
    {
        "extensions": [".foo.ts"],
        "exclude_filenames": [".bar.ts"],
        "exclude_directories": ["/baz"],
        "exclude_under": 10,
        "output_limit": 2500,
        "score_cap": 500,
        "include_comments": true
    }
    "#;

        let temp_file = create_temp_file(valid_json);
        let path = temp_file.path().to_str().unwrap();
        let config = read_config(path.to_string(), false).unwrap();

        assert_eq!(
            config.extensions,
            vec![
                ".js".to_string(),
                ".jsx".to_string(),
                ".ts".to_string(),
                ".tsx".to_string(),
                ".foo.ts".to_string()
            ]
        );
        assert_eq!(
            config.exclude_filenames,
            vec![
                ".d.ts".to_string(),
                ".min.js".to_string(),
                ".bundle.js".to_string(),
                ".bar.ts".to_string()
            ]
        );
        assert_eq!(
            config.exclude_directories,
            vec![
                "/dist".to_string(),
                "/bin".to_string(),
                "/build".to_string(),
                "/baz".to_string(),
            ]
        );
        assert_eq!(config.output_limit, 2500);
        assert_eq!(config.score_cap, 500);
        assert_eq!(config.include_comments, true);
    }

    #[test]
    fn test_read_config_with_partial_json() {
        let partial_json = r#"
    {
        "extensions": [".foo.ts"],
        "exclude_filenames": [".bar.ts"]
    }
    "#;

        let temp_file = create_temp_file(partial_json);
        let path = temp_file.path().to_str().unwrap();
        let config = read_config(path.to_string(), false).unwrap();

        assert_eq!(
            config.extensions,
            vec![
                ".js".to_string(),
                ".jsx".to_string(),
                ".ts".to_string(),
                ".tsx".to_string(),
                ".foo.ts".to_string()
            ]
        );
        assert_eq!(
            config.exclude_filenames,
            vec![
                ".d.ts".to_string(),
                ".min.js".to_string(),
                ".bundle.js".to_string(),
                ".bar.ts".to_string()
            ]
        );
        assert_eq!(
            config.exclude_directories,
            vec![
                "/dist".to_string(),
                "/bin".to_string(),
                "/build".to_string(),
            ]
        );
        assert_eq!(config.output_limit, 5000);
        assert_eq!(config.score_cap, 1000);
        assert_eq!(config.include_comments, false);
    }

    #[test]
    fn test_read_config_with_nonexistent_file() {
        let nonexistent_path = "nonexistent_file.json";

        let config = read_config(nonexistent_path.to_string(), false).unwrap();

        assert_eq!(
            config.extensions,
            vec![
                ".js".to_string(),
                ".jsx".to_string(),
                ".ts".to_string(),
                ".tsx".to_string(),
            ]
        );
        assert_eq!(
            config.exclude_filenames,
            vec![
                ".d.ts".to_string(),
                ".min.js".to_string(),
                ".bundle.js".to_string(),
            ]
        );
        assert_eq!(
            config.exclude_directories,
            vec![
                "/dist".to_string(),
                "/bin".to_string(),
                "/build".to_string(),
            ]
        );
        assert_eq!(config.output_limit, 5000);
        assert_eq!(config.score_cap, 1000);
        assert_eq!(config.include_comments, false);
    }

    #[test]
    fn test_read_config_with_user_specified_file_path() {
        let valid_json = r#"
    {
        "extensions": [".foo.ts"],
        "exclude_filenames": [".bar.ts"],
        "exclude_directories": ["/baz"],
        "output_limit": 2500,
        "score_cap": 500
    }
    "#;

        let temp_file = create_temp_file(valid_json);
        let path = temp_file.path().to_str().unwrap();

        let config = read_config(path.to_string(), true).unwrap();

        assert_eq!(
            config.extensions,
            vec![
                ".js".to_string(),
                ".jsx".to_string(),
                ".ts".to_string(),
                ".tsx".to_string(),
                ".foo.ts".to_string(),
            ]
        );
        assert_eq!(
            config.exclude_filenames,
            vec![
                ".d.ts".to_string(),
                ".min.js".to_string(),
                ".bundle.js".to_string(),
                ".bar.ts".to_string(),
            ]
        );
        assert_eq!(
            config.exclude_directories,
            vec![
                "/dist".to_string(),
                "/bin".to_string(),
                "/build".to_string(),
                "/baz".to_string(),
            ]
        );
        assert_eq!(config.output_limit, 2500);
        assert_eq!(config.score_cap, 500);
        assert_eq!(config.include_comments, false);
    }

    #[test]
    fn temp_read_config_with_nonexistent_file_and_user_specified_file_path() {
        let config = read_config(String::from("nonexistent_file.json"), true);

        assert!(config.is_err(), "Expected error, got {:?}", config);
    }
}
