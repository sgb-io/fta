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

    #[test]
    fn it_parses_decorators() {
        let ts_code = r#"
            import { IsNumber, IsOptional } from 'class-validator';

            export class RulesReward {
                @IsNumber()
                @IsOptional()
                points?: number;
            }
        "#;

        let (parsed_module, _line_count) = parse_module(ts_code, true, false);
        assert!(parsed_module.is_ok(), "Failed to parse decorators");
    }

    #[test]
    fn it_parses_import_attributes() {
        let ts_code = r#"
            import data from './data.json' with { type: 'json' };
            import styles from './styles.css' with { type: 'css' };

            console.log(data);
        "#;

        let (parsed_module, line_count) = parse_module(ts_code, true, false);

        assert!(
            parsed_module.is_ok(),
            "Failed to parse import attributes (import with syntax): {:?}",
            parsed_module.err()
        );
        assert_eq!(line_count, 3, "Incorrect line count");
    }

    #[test]
    fn it_parses_various_import_attribute_forms() {
        let ts_code = r#"
            import data from './data.json' with { type: 'json' };
            import config from './config.json' with { type: 'json', assert: true };
            import { default as content } from './content.json' with { type: 'json' };
            import * as allData from './all-data.json' with { type: 'json' };
            const result = { data, config, content, allData };
        "#;

        let (parsed_module, line_count) = parse_module(ts_code, true, false);

        assert!(
            parsed_module.is_ok(),
            "Failed to parse various import attribute forms: {:?}",
            parsed_module.err()
        );
        assert_eq!(line_count, 5, "Incorrect line count");
    }
}
