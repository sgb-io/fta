# Maintenance of FTA

This project currently consists of 4 components:

- The Rust `fta` crate, in `crates/fta`
- The Rust `fta-wasm` crate, in `crates/fta-wasm`
- The NPM `fta-cli` package, in `packages/fta`
- The NPM `fta-wasm` package, an artefact of the `fta-wasm` Rust crate

The NPM `fta-cli` package is a super thin layer that simply calls the relevant `fta` binary. For this to work, the NPM package is designed to contain pre-built binaries.

## Development

Use PRs into `main`. GitHub Actions are set up to:

- Compile the Rust crate & run Rust tests, output test coverage (Ubuntu)
- Build binaries for all targets on windows/macos/linux
- Smoke test all built binaries against a sample file
- Construct the NPM package, i.e. install the compiled binaries into `packages/fta/binaries`
- Publish the NPM package locally using Verdaccio
- Smoke test the verdaccio-published NPM package via a sample file

The NPM CLI package itself is plain JavaScript without any Node.js tests or build step, since those things aren't really warranted.

## Publishing and releasing (`fta` crate, `fta-cli` npm package)

1. Merge changes over time to `main`, with green builds to verify everything is healthy
2. Bump versions and update `CHANGELOG.md`
   1. Set the version in `packages/fta/package.json`
   2. Set the version in `crates/fta/Cargo.toml`, run `cargo update` so that the lockfile updates too. Do this in a PR and merge it to `main`.
3. When you're satisfied everything is ready on `main` (and the build is green), locally tag the repo with a new version e.g. `v1.0.0`. Push this tag to trigger the release.

## WASM npm package

This should be published manually. From the `crates/fta-wasm` directory:

1. Ensure the crate version is in sync. Similar to the `fta-cli` package, it usually makes sense for the core `fta` crate to be published first.
2. If you already have the `crates/fta-wasm/pkg` dir, delete it / clear it out.
3. Run `wasm-pack build --target web`. This'll prep the files in `pkg`.
4. If you want to locally debug before publish, you can paste the contents of `pkg` to override an existing version in `node_modules.`
5. Run `wasm-pack publish pkg`. This directly publishes the output to NPM.

## Code Coverage

Code coverage is reported during the `test` workflow.

To check the coverage locally, install and run `tarpaulin`:

```
cargo install cargo-tarpaulin
cargo tarpaulin
```

Note that `tarpaulin` is not installed as a build dependency, hence should be installed manually to generate coverage.
