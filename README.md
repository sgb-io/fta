# TypeWire

TypeWire is a super-fast TypeScript static analysis tool written in Rust. It captures static information about TypeScript code and generates easy-to-understand analytics that tell you about complexity and maintainability issues that you may want to address.

TypeWire uses [swc](https://github.com/swc-project/swc) to parse your code then runs various analytical routines against it to understand how complex and maintainable it is likely to be.

# Features

Under the hood, various metrics are calculated:

- The Halstead Metrics: uses the unique and total number of operators and operands in the code to calculate several complexity measures such as size, vocabulary, difficulty, time to program and "delivered bugs".
- Cyclomatic Complexity: the effective number of distinct logical paths through the code
- _More likely to follow_

_More to follow_

# Documentation

TypeWire is not yet published or distributed outside of the GitHub repo. To use it, clone the project and run it via `cargo`, e.g:

```bash
cargo run path/to/typescript/code.ts
```

_More to follow_
