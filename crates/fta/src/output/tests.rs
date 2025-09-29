#[cfg(test)]
mod tests {
    use crate::output::{generate_output_with_colors, truncate_string};
    use crate::structs::{FileData, HalsteadMetrics};

    fn get_test_data() -> Vec<FileData> {
        vec![
            FileData {
                file_name: "test.js".to_string(),
                cyclo: 1,
                halstead: HalsteadMetrics {
                    uniq_operators: 1,
                    uniq_operands: 2,
                    total_operators: 3,
                    total_operands: 4,
                    program_length: 5,
                    vocabulary_size: 6,
                    volume: 7.0,
                    difficulty: 8.0,
                    effort: 9.0,
                    time: 10.0,
                    bugs: 11.0,
                },
                line_count: 1,
                fta_score: 45.00,
                assessment: "OK".to_string(),
            },
            FileData {
                file_name: "foo.tsx".to_string(),
                cyclo: 1,
                halstead: HalsteadMetrics {
                    uniq_operators: 1,
                    uniq_operands: 2,
                    total_operators: 3,
                    total_operands: 4,
                    program_length: 5,
                    vocabulary_size: 6,
                    volume: 7.0,
                    difficulty: 8.0,
                    effort: 9.0,
                    time: 10.0,
                    bugs: 11.0,
                },
                line_count: 25,
                fta_score: 95.00,
                assessment: "OK".to_string(),
            },
            FileData {
                file_name: "bar.jsx".to_string(),
                cyclo: 1,
                halstead: HalsteadMetrics {
                    uniq_operators: 1,
                    uniq_operands: 2,
                    total_operators: 3,
                    total_operands: 4,
                    program_length: 5,
                    vocabulary_size: 6,
                    volume: 7.0,
                    difficulty: 8.0,
                    effort: 9.0,
                    time: 10.0,
                    bugs: 11.0,
                },
                line_count: 50,
                fta_score: 145.00,
                assessment: "OK".to_string(),
            },
        ]
    }

    // Mostly eliminate whitespace from table/csv output to make comparison easier
    fn format_expected_output(expected: &str) -> String {
        let formatted = expected
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("\n");

        formatted
    }

    // Eliminate whitespace from json output to make comparison easier
    fn format_json_output(json: &str) -> String {
        json.chars().filter(|&c| !c.is_whitespace()).collect()
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(
            truncate_string("extremely-long-file-name-that-will-be-hard-to-display", 25),
            "...ill-be-hard-to-display"
        );
        assert_eq!(truncate_string("abcdef", 7), "abcdef");
        assert_eq!(truncate_string("abcdef", 6), "abcdef");
        assert_eq!(truncate_string("abcdef", 5), "...ef");
        assert_eq!(truncate_string("abcdef", 4), "...f");
        assert_eq!(truncate_string("abcdef", 3), "...");
    }

    #[test]
    fn test_output_csv_format() {
        let file_data_list = get_test_data();
        let output_str = format!(
            "\n{}\n",
            generate_output_with_colors(&file_data_list, "csv".to_string(), &0.1_f64, 100, false)
        );
        let expected_output_raw = r##"
            File,Num. lines,FTA Score (Lower is better),Assessment
            test.js,1,45.00,OK
            foo.tsx,25,95.00,OK
            bar.jsx,50,145.00,OK
        "##;
        let expected_output = format_expected_output(expected_output_raw);
        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_output_csv_format_is_not_limited_by_output_limit() {
        let file_data_list = get_test_data();
        let output_limit = 1;
        let output_str = format!(
            "\n{}\n",
            generate_output_with_colors(
                &file_data_list,
                "csv".to_string(),
                &0.1_f64,
                output_limit,
                false
            )
        );
        let expected_output_raw = r##"
            File,Num. lines,FTA Score (Lower is better),Assessment
            test.js,1,45.00,OK
            foo.tsx,25,95.00,OK
            bar.jsx,50,145.00,OK
        "##;
        let expected_output = format_expected_output(expected_output_raw);
        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_output_table_format() {
        let file_data_list = get_test_data();
        let output_str =
            generate_output_with_colors(&file_data_list, "table".to_string(), &0.1_f64, 100, false);
        let expected_output_raw = r##"
            ┌─────────┬────────────┬─────────────────────────────┬────────────┐
            │ File    ┆ Num. lines ┆ FTA Score (Lower is better) ┆ Assessment │
            ╞═════════╪════════════╪═════════════════════════════╪════════════╡
            │ test.js ┆ 1          ┆ 45.00                       ┆ OK         │
            ├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
            │ foo.tsx ┆ 25         ┆ 95.00                       ┆ OK         │
            ├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┤
            │ bar.jsx ┆ 50         ┆ 145.00                      ┆ OK         │
            └─────────┴────────────┴─────────────────────────────┴────────────┘
            3 files analyzed in 0.1s.
        "##;

        let expected_output = format_expected_output(expected_output_raw);
        let expected_output = expected_output
            .trim_start_matches('\n')
            .trim_end_matches('\n');
        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_output_table_can_be_limited() {
        let file_data_list = get_test_data();
        let output_limit = 1;
        let output_str = generate_output_with_colors(
            &file_data_list,
            "table".to_string(),
            &0.1_f64,
            output_limit,
            false,
        );
        let expected_output_raw = r##"
            ┌─────────┬────────────┬─────────────────────────────┬────────────┐
            │ File    ┆ Num. lines ┆ FTA Score (Lower is better) ┆ Assessment │
            ╞═════════╪════════════╪═════════════════════════════╪════════════╡
            │ test.js ┆ 1          ┆ 45.00                       ┆ OK         │
            └─────────┴────────────┴─────────────────────────────┴────────────┘
            3 files analyzed in 0.1s.
        "##;

        let expected_output = format_expected_output(expected_output_raw);
        let expected_output = expected_output
            .trim_start_matches('\n')
            .trim_end_matches('\n');
        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_output_unspecified_format() {
        let file_data_list = get_test_data();
        let output_str = generate_output_with_colors(
            &file_data_list,
            "unspecified".to_string(),
            &0.1_f64,
            100,
            false,
        );
        let expected_output = "No output format specified.";
        assert_eq!(output_str, expected_output);
    }

    #[test]
    fn test_output_json_format() {
        let file_data_list = get_test_data();
        let output_str =
            generate_output_with_colors(&file_data_list, "json".to_string(), &0.1_f64, 100, false);

        let expected_output = r##"[
            {
                "file_name": "test.js",
                "cyclo": 1,
                "halstead":
                {
                    "uniq_operators": 1,
                    "uniq_operands": 2,
                    "total_operators": 3,
                    "total_operands": 4,
                    "program_length": 5,
                    "vocabulary_size": 6,
                    "volume": 7.0,
                    "difficulty": 8.0,
                    "effort": 9.0,
                    "time": 10.0,
                    "bugs": 11.0
                },
                "line_count": 1,
                "fta_score": 45.0,
                "assessment": "OK"
            },
            {
                "file_name": "foo.tsx",
                "cyclo": 1,
                "halstead":
                {
                    "uniq_operators": 1,
                    "uniq_operands": 2,
                    "total_operators": 3,
                    "total_operands": 4,
                    "program_length": 5,
                    "vocabulary_size": 6,
                    "volume": 7.0,
                    "difficulty": 8.0,
                    "effort": 9.0,
                    "time": 10.0,
                    "bugs": 11.0
                },
                "line_count": 25,
                "fta_score": 95.0,
                "assessment": "OK"
            },
            {
                "file_name": "bar.jsx",
                "cyclo": 1,
                "halstead":
                {
                    "uniq_operators": 1,
                    "uniq_operands": 2,
                    "total_operators": 3,
                    "total_operands": 4,
                    "program_length": 5,
                    "vocabulary_size": 6,
                    "volume": 7.0,
                    "difficulty": 8.0,
                    "effort": 9.0,
                    "time": 10.0,
                    "bugs": 11.0
                },
                "line_count": 50,
                "fta_score": 145.0,
                "assessment": "OK"
            }
        ]"##;

        assert_eq!(
            format_json_output(&output_str),
            format_json_output(expected_output)
        );
    }

    #[test]
    fn test_output_colors_are_applied_correctly() {
        let file_data_list = vec![
            FileData {
                file_name: "low_score.js".to_string(),
                cyclo: 1,
                halstead: HalsteadMetrics {
                    uniq_operators: 1,
                    uniq_operands: 2,
                    total_operators: 3,
                    total_operands: 4,
                    program_length: 5,
                    vocabulary_size: 6,
                    volume: 7.0,
                    difficulty: 8.0,
                    effort: 9.0,
                    time: 10.0,
                    bugs: 11.0,
                },
                line_count: 10,
                fta_score: 30.0, // Should be green
                assessment: "OK".to_string(),
            },
            FileData {
                file_name: "medium_score.js".to_string(),
                cyclo: 5,
                halstead: HalsteadMetrics {
                    uniq_operators: 10,
                    uniq_operands: 15,
                    total_operators: 30,
                    total_operands: 40,
                    program_length: 50,
                    vocabulary_size: 25,
                    volume: 100.0,
                    difficulty: 8.0,
                    effort: 800.0,
                    time: 44.0,
                    bugs: 0.03,
                },
                line_count: 50,
                fta_score: 55.0, // Should be yellow
                assessment: "Could be better".to_string(),
            },
            FileData {
                file_name: "high_score.js".to_string(),
                cyclo: 15,
                halstead: HalsteadMetrics {
                    uniq_operators: 20,
                    uniq_operands: 30,
                    total_operators: 100,
                    total_operands: 150,
                    program_length: 250,
                    vocabulary_size: 50,
                    volume: 1000.0,
                    difficulty: 15.0,
                    effort: 15000.0,
                    time: 833.0,
                    bugs: 0.33,
                },
                line_count: 200,
                fta_score: 65.0, // Should be red
                assessment: "Needs improvement".to_string(),
            },
        ];

        let colored_output =
            generate_output_with_colors(&file_data_list, "table".to_string(), &0.1_f64, 100, true);
        let uncolored_output =
            generate_output_with_colors(&file_data_list, "table".to_string(), &0.1_f64, 100, false);

        // Check that colored output contains ANSI codes
        assert!(
            colored_output.contains("\u{1b}["),
            "Colored output should contain ANSI escape codes"
        );

        // Check that uncolored output does not contain ANSI codes
        assert!(
            !uncolored_output.contains("\u{1b}["),
            "Uncolored output should not contain ANSI escape codes"
        );

        // Check for specific colors:
        // White headers
        assert!(
            colored_output.contains("\u{1b}[38;5;15m"),
            "Should contain white color for headers"
        );

        // Dark grey for File and Num. lines columns
        assert!(
            colored_output.contains("\u{1b}[38;5;8m"),
            "Should contain dark grey for File/Num. lines columns"
        );

        // Green for low FTA score (30.0) and "OK"
        assert!(
            colored_output.contains("\u{1b}[38;5;10m"),
            "Should contain green color for low scores/OK"
        );

        // Yellow for medium FTA score (55.0) and "Could be better"
        assert!(
            colored_output.contains("\u{1b}[38;5;11m"),
            "Should contain yellow color for medium scores/Could be better"
        );

        // Red for high FTA score (65.0) and "Needs improvement"
        assert!(
            colored_output.contains("\u{1b}[38;5;9m"),
            "Should contain red color for high scores/Needs improvement"
        );
    }
}
