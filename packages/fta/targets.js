const exeTargets = ["aarch64-pc-windows-msvc", "x86_64-pc-windows-msvc"];

const plainTargets = [
  "aarch64-unknown-linux-gnu",
  // "aarch64-unknown-linux-musl", // Not usually needed
  "armv7-unknown-linux-gnueabihf",
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
  "x86_64-unknown-linux-gnu",
];

module.exports.exeTargets = exeTargets;
module.exports.plainTargets = plainTargets;
