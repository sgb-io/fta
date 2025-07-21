#[cfg(test)]
mod integration_tests {
    use crate::config::read_config;
    use crate::utils::is_excluded_filename;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_exclude_filenames_integration() {
        let config_json = r#"
        {
            "exclude_filenames": [
                ".spec.jsx",
                ".stories.tsx",
                ".types.ts",
                "*.test.ts"
            ]
        }
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", config_json).unwrap();
        let path = temp_file.path().to_str().unwrap();

        let config = read_config(path.to_string(), false).unwrap();

        // Test that suffix patterns work
        assert_eq!(
            is_excluded_filename("Component.spec.jsx", &config.exclude_filenames),
            true
        );
        assert_eq!(
            is_excluded_filename("Button.stories.tsx", &config.exclude_filenames),
            true
        );
        assert_eq!(
            is_excluded_filename("api.types.ts", &config.exclude_filenames),
            true
        );

        // Test that wildcard patterns still work
        assert_eq!(
            is_excluded_filename("utils.test.ts", &config.exclude_filenames),
            true
        );

        // Test that non-matching files are not excluded
        assert_eq!(
            is_excluded_filename("Component.tsx", &config.exclude_filenames),
            false
        );
        assert_eq!(
            is_excluded_filename("Button.jsx", &config.exclude_filenames),
            false
        );
        assert_eq!(
            is_excluded_filename("index.ts", &config.exclude_filenames),
            false
        );

        // Test that default exclusions still work (should include .d.ts from defaults)
        assert_eq!(
            is_excluded_filename("types.d.ts", &config.exclude_filenames),
            true
        );
    }
}
