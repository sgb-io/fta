<p align="center">
  <img src="fta-logo.png" alt="FTA" width="120" />
</p>

<h2 align="center">
  Fast TypeScript Analyzer
</h2>

FTA (Fast TypeScript Analyzer) is a super-fast TypeScript static analysis tool written in Rust. It captures static information about TypeScript code and generates easy-to-understand analytics that tell you about complexity and maintainability issues that you may want to address.

FTA uses [swc](https://github.com/swc-project/swc) to parse your code then runs various analytical routines against it to understand how complex and maintainable it is likely to be. JavaScript code is also supported.

## Quickstart

There are several ways to use `fta`. The simplest is to use `fta-cli`:

```
npx fta-cli path/to/project
```

Example output against the Redux project:

```
| ----------------------------------------- | ------------ | ----------------------------- | ------------------- |
| File                                      |   Num. lines |   FTA Score (Lower is better) |          Assessment |
| ----------------------------------------- | ------------ | ----------------------------- | ------------------- |
| src\createStore.ts                        |          490 |                         70.25 | (Needs improvement) |
| website\src\pages\index.js                |          218 |                         64.94 | (Needs improvement) |
| src\combineReducers.ts                    |          202 |                         61.61 | (Needs improvement) |
| src\compose.ts                            |           62 |                         52.68 |   (Could be better) |
| src\bindActionCreators.ts                 |           84 |                         51.89 |   (Could be better) |
| src\utils\kindOf.ts                       |           71 |                         48.80 |                  OK |
| src\utils\warning.ts                      |           19 |                         35.00 |                  OK |
| src\utils\isPlainObject.ts                |           15 |                         34.32 |                  OK |
| src\utils\symbol-observable.ts            |           11 |                         31.89 |                  OK |
| website\docusaurus.config.js              |          197 |                         18.04 |                  OK |
| website\sidebars.js                       |          149 |                         15.82 |                  OK |
| rollup.config.js                          |           80 |                         15.79 |                  OK |
| tsup.config.ts                            |           73 |                         15.59 |                  OK |
| src\applyMiddleware.ts                    |           78 |                         15.45 |                  OK |
| website\src\pages\errors.js               |           63 |                         15.09 |                  OK |
| website\src\js\monokaiTheme.js            |           63 |                         14.32 |                  OK |
| src\utils\actionTypes.ts                  |           18 |                         11.91 |                  OK |
| src\index.ts                              |           47 |                         11.84 |                  OK |
| vitest.config.ts                          |           18 |                          9.92 |                  OK |
| docs\components\DetailedExplanation.jsx   |           16 |                          9.67 |                  OK |
| src\utils\formatProdErrorMessage.ts       |           14 |                          8.57 |                  OK |
| ----------------------------------------- | ------------ | ----------------------------- | ------------------- |
21 files analyzed in 0.1079s.
```

## Call FTA from a script

1. To call FTA from a script, install `fta-cli` as a dependency and call it:

<!-- - As a script that prints out information about your project (_Recommended_)
  - You can optionally set `score_cap` to require a minimum quality level in your CI (See [Configuration](https://ftaproject.dev/docs/configuration))
- From code so that you can programmatically interact with the output -->

```bash
yarn add fta-cli
# or
npm install fta-cli
# or
pnpm install fta-cli
```

2. Call `fta` from a `package.json` script:

```
"scripts": {
  "fta": "fta src"
}
```

## Call FTA from code

You can also call `fta-cli` from code:

```javascript
import { runFta } from "fta-cli";

// Print the standard ascii table output
const standardOutput = runFta("path/to/project");

// Alternatively, get the full output as JSON so that you can interact with it
const output = runFta("path/to/project", { json: true });
```

## Output

By default, `fta` outputs a table of output that summarizes the result. You can optionally supply the `json` argument to get the full output as JSON.

You can also get the JSON output in a scripting context:

```
fta /path/to/project --json
```

For more information on using FTA, be sure to check out the [docs](https://ftaproject.dev).

## Configuring FTA

Various configuration options are available, including the ability to cause CI to fail if a certain score threshold is breached. See the full Configuration options on the [docs](https://ftaproject.dev/docs/configuration).

## Docs

Read the full documentation on the [docs](https://ftaproject.dev).

## License

[MIT](LICENSE.md)
