# Maintenance of FTA

This project currently consists of two primary components:

- The Rust `fta` crate, in `crates/fta`
- The NPM `fta` package, in `packages/fta`

The NPM package is a super thin layer that simply calls the relevant `fta` binary. For this to work, the NPM package is designed to contain pre-built binaries.

## Development

Use PRs into `main`. GitHub actions are set up to test and verify changes to the Rust crate, but nothing is in place for the NPM package.

The NPM package is plain JavaScript with no tests or build step, since those things aren't really warranted.

## Publishing and releasing (`fta` crate, `fta-cli` npm package)

The Rust crate should always be published first, then the NPM package.

The two packages should use the same version number (but the Rust crate is still published / in existence first).

The Rust crate is published **automatically by GitHub actions**, but the NPM package is published manually (locally).

1. Merge work into `main` over time
2. When you want to release, manually bump the version in `crates/fta/Cargo.toml`, run `cargo update` so that the lockfile updates too. Do this in a PR and merge it to main.
3. Manually update `CHANGELOG.md`
4. When you're satisfied everything is ready on `main` (and the build is green), locally tag the repo with a new version e.g. `v1.0.0`. Push this tag to trigger the Rust crate release process.
5. If you want any changes to the NPM package itself as part of this new version, also make those changes. This can be merged to main after the Crate release has happened.
6. Assuming the Rust crate release was successful (step 3), there should be a draft GitHub release, containing the new binaries. Publish the release and download all the binaries.
7. Place the binaries in the relevant directories in `packages/fta/bin` (they should be .gitignore'd). Optionally run `npm run prepublishOnly` from the NPM package directory to verify that all binaries are installed.
8. Bump the NPM package version if you didn't already. This can be pushed to main whenever, but it needs to happen before manually publishing to NPM.
9. Manually publish the NPM package. You can optionally run `npm pack` and/or the `--dry-run` option to double check what will be published. You can also use `npm pack` to test the package out locally.

Note: the NPM package has a `prepublishOnly` script that should automatically run ahead of publishing - this verifies that all binaries are installed.

Although the versions of the two packages are not functionally linked, it's a good idea to keep them in sync to keep things simple. If a mistake is made with the npm package, a patch bump to the Rust crate is a better fix than inconsistent versions or attempting to unpublish.

## WASM npm package

This should be published manually. From the `crates/fta-wasm` directory:

1. Ensure the crate version is in sync. Similar to the `fta-cli` package, it usually makes sense for the core `fta` crate to be published first.
2. If you already have the `crates/fta-wasm/pkg` dir, delete it / clear it out.
3. Run `wasm-pack build --target web`. This'll prep the files in `pkg`.
4. If you want to locally debug before publish, you can paste the contents of `pkg` to override an existing version in `node_modules.`
5. Run `wasm-pack publish pkg`. This directly publishes the output to NPM.

## Why not more automatic?

It's complex: the NPM package relies on the Rust crate binaries, which currently only get built in CI & uploaded to the GitHub release.

A potential improvement on this publishing workflow is to keep hold of the binaries in CI and use them as input to an NPM package publish job. The NPM package version would also need bumping in this scenario.
