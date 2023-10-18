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
                "fta_score": 9.534164185651022,
                "line_count": 8,
                "halstead_metrics": {
                    "bugs": 0.020810680886974055,
                    "difficulty": 3.3333333333333335,
                    "effort": 208.10680886974055,
                    "program_length": 14,
                    "time": 11.561489381652253,
                    "total_operands": 12,
                    "total_operators": 10,
                    "uniq_operands": 8,
                    "uniq_operators": 6,
                    "vocabulary_size": 22,
                    "volume": 62.43204266092216
                }
            }
        "#;

        let result = analyze_file_wasm(input_code, true, false);
        let expected_json: Value = from_str(expected_output).unwrap();
        let actual_json: Value = from_str(&result).unwrap();

        assert_eq!(expected_json, actual_json);
    }
}
