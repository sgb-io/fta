#[cfg(test)]
mod tests {
    use crate::parse_module::parse_module;

    #[test]
    fn test_parse_module() {
        let ts_code = r#"
            function add(a: number, b: number): number {
                return a + b;
            }

            const myResult = add(23, 56);
            console.log(myResult); // 79
        "#;

        let (parsed_module, line_count) = parse_module(ts_code);

        assert!(parsed_module.is_ok(), "Failed to parse TypeScript code");
        assert_eq!(line_count, 8, "Incorrect line count");
    }
}
