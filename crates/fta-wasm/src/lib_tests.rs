#[cfg(test)]
mod tests {
    use crate::analyze_file_wasm;
    use serde_json::{from_str, Value};

    #[test]
    fn test_analyze_project() {
        let input_code = r#"
            function add(a: number, b: number): number {
                return a + b;
            }

            const myResult = add(23, 56);
            console.log(myResult); // 79
        "#;

        let expected_output = r#"
            {
                "cyclo": 1,
                "fta_score": 8.159706499414824,
                "line_count": 5,
                "halstead_metrics": {
                    "bugs": 0.027920602761755765,
                    "difficulty": 4.5,
                    "effort": 376.9281372837028,
                    "program_length": 22,
                    "time": 20.940452071316823,
                    "total_operands": 12,
                    "total_operators": 10,
                    "uniq_operands": 8,
                    "uniq_operators": 6,
                    "vocabulary_size": 14,
                    "volume": 83.76180828526729
                }
            }
        "#;

        let result = analyze_file_wasm(input_code, true, false);
        let expected_json: Value = from_str(expected_output).unwrap();
        let actual_json: Value = from_str(&result).unwrap();

        assert_eq!(expected_json, actual_json);
    }
}
