#[cfg(test)]
mod tests {
    use crate::halstead::analyze_module;
    use crate::parse::parse_module;
    use crate::structs::HalsteadMetrics;
    use swc_ecma_ast::Module;

    fn parse(ts_code: &str) -> Module {
        let (parsed_module, line_count) = parse_module(ts_code, true);

        if let Ok(parsed_module) = parsed_module {
            parsed_module
        } else {
            panic!("failed");
        }
    }

    fn analyze(module: &Module) -> HalsteadMetrics {
        let metrics = analyze_module(module);
        metrics
    }

    #[test]
    fn test_empty_module() {
        let ts_code = r#"
            /* Empty TypeScript code */
        "#;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 0,
            uniq_operands: 0,
            total_operators: 0,
            total_operands: 0,
            program_length: 0,
            vocabulary_size: 0,
            volume: 0.0,
            difficulty: 0.0,
            effort: 0.0,
            time: 0.0,
            bugs: 0.0,
        };
        assert_eq!(analyze(&module), expected);
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
        let expected = HalsteadMetrics {
            uniq_operators: 3,
            uniq_operands: 8,
            total_operators: 9,
            total_operands: 12,
            program_length: 11,
            vocabulary_size: 21,
            volume: 48.315491650566365,
            difficulty: 2.6666666666666665,
            effort: 128.84131106817696,
            time: 7.15785061489872,
            bugs: 0.016105163883522122,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_component_a() {
        let ts_code = r##"
        import { React, useState } from 'react';
        import { asyncOperation } from './asyncOperation';
        
        let staticFoo = true;
        
        function displayThing(thing: string) {
          return `thing: ${thing}`;
        }
        
        export default function DummyComponent() {
          const [thing, setThing] = useState(null);
        
          const thingForDisplay = displayThing(thing) as string;
        
          const interact = async () => {
            const result = await asyncOperation();
            setThing(result);
            staticFoo = false;
            
            if (typeof thing === 'object' && thing?.foo?.bar) {
              console.log('This should not happen');
            }
          }
        
          const baz = staticFoo ? 32 : 42;
        
          return (
            <>
              <div>
                <h1>Hello World</h1>
              </div>
              <div>
                <h2>This is a test. {thingForDisplay} {baz}</h2>
                <button onClick={interact}>Click me</button>
              </div>
            </>
          )
        }
      "##;
        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 21,
            uniq_operands: 26,
            total_operators: 43,
            total_operands: 47,
            program_length: 47,
            vocabulary_size: 90,
            volume: 305.1170955274947,
            difficulty: 11.617021276595745,
            effort: 3544.5517905960023,
            time: 196.91954392200012,
            bugs: 0.1017056985091649,
        };
        assert_eq!(analyze(&module), expected);
    }

    #[test]
    fn test_complex_component_c() {
        let ts_code = r##"
        let a, b, c = 3;
        a = 1;
        b = 2;
        let myArray = [a, b, c];
        
        myArray = [...myArray, ...myArray, 8, 9, 10];
        
        const myObject = {
          foo: 'bar'
        }
        
        const myOtherObject = {
          ...myObject,
          bar: 'baz'
        }
        
        class Foo {
          constructor() {
            this.foo = 'some value';
          }
        
          getFoo() {
            return this.foo!;
          }
        
          isFooCool() {
            const myRegex = /cool/;
            return myRegex.test(this.foo);
          }
        }
        
        const myFoo = new Foo();
        
        export { myFoo, myOtherObject };
      "##;

        let module = parse(ts_code);
        let expected = HalsteadMetrics {
            uniq_operators: 10,
            uniq_operands: 25,
            total_operators: 31,
            total_operands: 44,
            program_length: 35,
            vocabulary_size: 75,
            volume: 218.00865416735581,
            difficulty: 8.522727272727273,
            effort: 1858.0283025626918,
            time: 103.22379458681621,
            bugs: 0.07266955138911861,
        };
        assert_eq!(analyze(&module), expected);
    }
}
