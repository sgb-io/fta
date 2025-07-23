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

        // Expected output updated for SWC 14.0 - see halstead/tests.rs for detailed explanation
        // Key change: console.log() treated as single operand instead of console + log
        // Old: uniq_operands: 8, total_operands: 12 | New: uniq_operands: 7, total_operands: 11
        let expected_output = r#"
            {
                "cyclo": 1,
                "fta_score": 7.93434892327484,
                "line_count": 5,
                "halstead_metrics": {
                    "bugs": 0.025903078026987644,
                    "difficulty": 4.714285714285714,
                    "effort": 366.3435320959681,
                    "program_length": 21,
                    "time": 20.352418449776007,
                    "total_operands": 11,
                    "total_operators": 10,
                    "uniq_operands": 7,
                    "uniq_operators": 6,
                    "vocabulary_size": 13,
                    "volume": 77.70923408096293
                }
            }
        "#;

        let result = analyze_file_wasm(input_code, true, false);
        let expected_json: Value = from_str(expected_output).unwrap();
        let actual_json: Value = from_str(&result).unwrap();

        assert_eq!(expected_json, actual_json);
    }
}
