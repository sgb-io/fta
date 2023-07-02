#[cfg(test)]
mod tests {
    use crate::utils::{get_assessment, is_excluded_filename};

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
}
