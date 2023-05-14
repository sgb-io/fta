#[cfg(test)]
mod lib_tests {
    use crate::analyze_file_wasm;
    use serde_json::{from_str, json, Value};

    #[test]
    fn test_analyze_project() {
        let input_code = r#"
            console.log("Hello, World!");
        "#;

        let expected_output = r#"
            {
                "fta_score": 5.583128210518055,
                "line_count": 1,
                "halstead_metrics": {
                    "bugs": 0.005169925001442312,
                    "difficulty": 1.0,
                    "effort": 15.509775004326936,
                    "program_length": 6,
                    "time": 0.861654166907052,
                    "total_operands": 3,
                    "total_operators": 3,
                    "uniq_operands": 3,
                    "uniq_operators": 3,
                    "vocabulary_size": 6,
                    "volume": 15.509775004326936
                }
            }
        "#;

        let result = analyze_file_wasm(input_code);
        let expected_json: Value = from_str(expected_output).unwrap();
        let actual_json: Value = from_str(&result).unwrap();

        assert_eq!(expected_json, actual_json);
    }
}
