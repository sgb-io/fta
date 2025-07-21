#[cfg(test)]
mod tests {
    use crate::utils::{get_assessment, is_excluded_directory_path, is_excluded_filename};

    #[test]
    fn test_get_assessment_ok() {
        let assessment = get_assessment(45.0);
        assert_eq!(assessment, "OK");
    }

    #[test]
    fn test_get_assessment_could_be_better() {
        let assessment = get_assessment(60.0);
        assert_eq!(assessment, "Could be better");
    }

    #[test]
    fn test_get_assessment_needs_improvement() {
        let assessment = get_assessment(75.0);
        assert_eq!(assessment, "Needs improvement");
    }

    #[test]
    fn test_is_excluded_filename_a() {
        let pattern = String::from("*/naughty/*.ts");
        let mut patterns = Vec::new();
        patterns.push(pattern);
        let result = is_excluded_filename("path/to/naughty/file.ts", &patterns);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_excluded_filename_b() {
        let pattern = String::from("*/naughty/*.ts");
        let mut patterns = Vec::new();
        patterns.push(pattern);
        let result = is_excluded_filename("path/to/sensible/file.ts", &patterns);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_excluded_filename_with_suffix_patterns() {
        let patterns = vec![
            ".spec.jsx".to_string(),
            ".stories.tsx".to_string(),
            ".d.ts".to_string()
        ];
        
        // Should match files ending with these patterns
        assert_eq!(is_excluded_filename("Component.spec.jsx", &patterns), true);
        assert_eq!(is_excluded_filename("Button.stories.tsx", &patterns), true);
        assert_eq!(is_excluded_filename("types.d.ts", &patterns), true);
        
        // Should not match files that don't end with these patterns
        assert_eq!(is_excluded_filename("Component.tsx", &patterns), false);
        assert_eq!(is_excluded_filename("Button.jsx", &patterns), false);
        assert_eq!(is_excluded_filename("script.ts", &patterns), false);
        
        // Should still match literal filenames if they exist
        assert_eq!(is_excluded_filename(".spec.jsx", &patterns), true);
        assert_eq!(is_excluded_filename(".stories.tsx", &patterns), true);
    }

    #[test]
    fn test_is_excluded_filename_with_wildcard_patterns() {
        let patterns = vec![
            "*.spec.jsx".to_string(),
            "test_*.tsx".to_string(),
        ];
        
        // Should work with existing wildcard patterns unchanged
        assert_eq!(is_excluded_filename("Component.spec.jsx", &patterns), true);
        assert_eq!(is_excluded_filename("test_utils.tsx", &patterns), true);
        assert_eq!(is_excluded_filename("Component.tsx", &patterns), false);
    }

    // Tests for is_excluded_directory_path function

    #[test]
    fn test_is_excluded_directory_path_single_component_root() {
        let patterns = vec!["node_modules".to_string()];

        // Should match files directly in node_modules
        assert_eq!(
            is_excluded_directory_path("node_modules/lib.js", &patterns),
            true
        );

        // Should match nested node_modules
        assert_eq!(
            is_excluded_directory_path("src/utils/node_modules/lib.js", &patterns),
            true
        );
        assert_eq!(
            is_excluded_directory_path("packages/pkg1/node_modules/lib.js", &patterns),
            true
        );

        // Should NOT match similar-named directories
        assert_eq!(
            is_excluded_directory_path("my-node_modules/lib.js", &patterns),
            false
        );
        assert_eq!(
            is_excluded_directory_path("node_modules_backup/lib.js", &patterns),
            false
        );

        // Should NOT match files with similar names
        assert_eq!(
            is_excluded_directory_path("src/node_modules.js", &patterns),
            false
        );
    }

    #[test]
    fn test_is_excluded_directory_path_single_component_with_slashes() {
        let patterns = vec!["/dist".to_string(), "build/".to_string()];

        // Both "/dist" and "build/" should work like "dist" and "build"
        assert_eq!(is_excluded_directory_path("dist/file.js", &patterns), true);
        assert_eq!(
            is_excluded_directory_path("packages/dist/file.js", &patterns),
            true
        );
        assert_eq!(is_excluded_directory_path("build/file.js", &patterns), true);
        assert_eq!(
            is_excluded_directory_path("src/build/file.js", &patterns),
            true
        );

        // Should NOT match partial matches
        assert_eq!(
            is_excluded_directory_path("my-dist/file.js", &patterns),
            false
        );
        assert_eq!(
            is_excluded_directory_path("build-tools/file.js", &patterns),
            false
        );
    }

    #[test]
    fn test_is_excluded_directory_path_multi_component() {
        let patterns = vec!["packages/dist".to_string(), "src/test".to_string()];

        // Should match exact multi-component paths
        assert_eq!(
            is_excluded_directory_path("packages/dist/file.js", &patterns),
            true
        );
        assert_eq!(
            is_excluded_directory_path("src/test/file.js", &patterns),
            true
        );

        // Should match nested multi-component paths
        assert_eq!(
            is_excluded_directory_path("root/packages/dist/file.js", &patterns),
            true
        );
        assert_eq!(
            is_excluded_directory_path("project/src/test/file.js", &patterns),
            true
        );

        // Should NOT match partial matches
        assert_eq!(
            is_excluded_directory_path("packages/file.js", &patterns),
            false
        );
        assert_eq!(is_excluded_directory_path("dist/file.js", &patterns), false);
        assert_eq!(is_excluded_directory_path("src/file.js", &patterns), false);
        assert_eq!(is_excluded_directory_path("test/file.js", &patterns), false);

        // Should NOT match reversed order
        assert_eq!(
            is_excluded_directory_path("dist/packages/file.js", &patterns),
            false
        );
        assert_eq!(
            is_excluded_directory_path("test/src/file.js", &patterns),
            false
        );
    }

    #[test]
    fn test_is_excluded_directory_path_absolute_patterns() {
        let patterns = vec!["/packages/dist".to_string()];

        // Absolute patterns should work the same as relative ones
        assert_eq!(
            is_excluded_directory_path("packages/dist/file.js", &patterns),
            true
        );
        assert_eq!(
            is_excluded_directory_path("root/packages/dist/file.js", &patterns),
            true
        );

        // Should NOT match when components are separate
        assert_eq!(
            is_excluded_directory_path("packages/other/dist/file.js", &patterns),
            false
        );
    }

    #[test]
    fn test_is_excluded_directory_path_empty_patterns() {
        let patterns: Vec<String> = vec![];
        assert_eq!(
            is_excluded_directory_path("any/path/file.js", &patterns),
            false
        );
    }

    #[test]
    fn test_is_excluded_directory_path_empty_pattern() {
        let patterns = vec!["".to_string(), "/".to_string()];
        // Empty patterns should not match anything
        assert_eq!(
            is_excluded_directory_path("any/path/file.js", &patterns),
            false
        );
    }

    #[test]
    fn test_is_excluded_directory_path_complex_scenario() {
        // Test a realistic monorepo scenario
        let patterns = vec![
            "node_modules".to_string(),
            "/dist".to_string(),
            "packages/legacy".to_string(),
            "build".to_string(),
        ];

        // These should be excluded
        assert_eq!(
            is_excluded_directory_path("node_modules/react/index.js", &patterns),
            true
        );
        assert_eq!(
            is_excluded_directory_path("packages/app/node_modules/lib.js", &patterns),
            true
        );
        assert_eq!(
            is_excluded_directory_path("dist/bundle.js", &patterns),
            true
        );
        assert_eq!(
            is_excluded_directory_path("packages/legacy/old.js", &patterns),
            true
        );
        assert_eq!(
            is_excluded_directory_path("build/output.js", &patterns),
            true
        );
        assert_eq!(
            is_excluded_directory_path("src/build/generated.js", &patterns),
            true
        );

        // These should NOT be excluded
        assert_eq!(
            is_excluded_directory_path("packages/app/src/index.js", &patterns),
            false
        );
        assert_eq!(
            is_excluded_directory_path("packages/new/src/index.js", &patterns),
            false
        );
        assert_eq!(
            is_excluded_directory_path("my-node_modules/custom.js", &patterns),
            false
        );
        assert_eq!(
            is_excluded_directory_path("built-assets/file.js", &patterns),
            false
        );
        assert_eq!(
            is_excluded_directory_path("packages/legacy-utils/file.js", &patterns),
            false
        ); // Not "packages/legacy"
    }
}
