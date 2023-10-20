#[cfg(test)]
mod tests {
    use crate::parse::parse_module;

    #[test]
    fn test_parse_module() {
        let ts_code = r#"
            function add(a: number, b: number): number {
                return a + b;
            }

            const myResult = add(23, 56);
            console.log(myResult); // 79
        "#;

        let (parsed_module, line_count) = parse_module(ts_code, true, false);

        assert!(parsed_module.is_ok(), "Failed to parse TypeScript code");
        assert_eq!(line_count, 5, "Incorrect line count");
    }

    #[test]
    fn it_ignores_comments() {
        let ts_code = r#"
            /*
            Block comment with multiple lines.
            */
            function add(a: number, b: number): number {
                return a + b;
            }
            
            // line comment
            const myResult = add(23, 56);
            /* block comment with single line */
            console.log(myResult); // Trailing comments don't count towards the comment count.
        "#;

        let (parsed_module, line_count) = parse_module(ts_code, true, false);

        assert!(parsed_module.is_ok(), "Failed to parse TypeScript code");
        assert_eq!(line_count, 5, "Incorrect line count");
    }

    #[test]
    fn it_can_be_configured_to_include_comments_in_the_line_count() {
        /*
           The below code includes 10 lines of code, but 12 lines in total due to a leading \n and the \n on like 7.
           These are filtered regardless of the include_comments flag.
        */
        let ts_code = r#"
            /*
            Block comment with multiple lines.
            */
            function add(a: number, b: number): number {
                return a + b;
            }

            // line comment
            const myResult = add(23, 56);
            /* block comment with single line */
            console.log(myResult); // Trailing comments don't count towards the comment count.
        "#;

        let (parsed_module, line_count) = parse_module(ts_code, true, true);

        assert!(parsed_module.is_ok(), "Failed to parse TypeScript code");
        assert_eq!(line_count, 10, "Incorrect line count");
    }
}
