#[cfg(test)]
use crate::complexity::cyclomatic_complexity;
use crate::parse_module::parse_module;

use swc_ecma_ast::Module;

fn parse(src: &str) -> Module {
    match parse_module(src) {
        (Ok(module), _line_count) => module,
        (Err(_err), _) => {
            panic!("failed");
        }
    }
}

#[test]
fn test_empty_module() {
    let ts_code = r#"
            /* Empty TypeScript code */
        "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 1);
}

#[test]
fn test_single_if() {
    let ts_code = r#"
            if (x > 0) {
                console.log("x is positive");
            }
        "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 2);
}

#[test]
fn test_if_else() {
    let ts_code = r#"
            if (x > 0) {
                console.log("x is positive");
            } else {
                console.log("x is not positive");
            }
        "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 2);
}

#[test]
fn test_nested_ifs() {
    let ts_code = r#"
            if (x > 0) {
                if (x < 10) {
                    console.log("x is between 0 and 10");
                }
            } else {
                console.log("x is not positive");
            }
        "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 3);
}

#[test]
fn test_switch_case() {
    let ts_code = r#"
            switch (x) {
                case 0:
                    console.log("x is 0");
                    break;
                case 1:
                    console.log("x is 1");
                    break;
                default:
                    console.log("x is not 0 or 1");
            }
        "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 4);
}

#[test]
fn test_for_loop() {
    let ts_code = r#"
            for (let i = 0; i < 10; i++) {
                console.log(i);
            }
        "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 2);
}

#[test]
fn test_while_loop() {
    let ts_code = r#"
        let i = 0;
        while (i < 10) {
            console.log(i);
            i++;
        }
    "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 2);
}

#[test]
fn test_do_while_loop() {
    let ts_code = r#"
        let i = 0;
        do {
            console.log(i);
            i++;
        } while (i < 10);
    "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 2);
}

#[test]
fn test_for_in_loop() {
    let ts_code = r#"
        let obj = { a: 1, b: 2, c: 3 };
        for (let key in obj) {
            console.log(key, obj[key]);
        }
    "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 2);
}

#[test]
fn test_for_of_loop() {
    let ts_code = r#"
        let arr = [1, 2, 3];
        for (let item of arr) {
            console.log(item);
        }
    "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 2);
}

#[test]
fn test_try_catch() {
    let ts_code = r#"
        try {
            throw new Error("An error occurred");
        } catch (e) {
            console.log(e.message);
        }
    "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 2);
}

#[test]
fn test_conditional_expression() {
    let ts_code = r#"
        let result = x > 0 ? "positive" : "non-positive";
    "#;
    let module = parse(ts_code);
    assert_eq!(cyclomatic_complexity(module), 2);
}
