// Windows
const exeTargets = ["aarch64-pc-windows-msvc", "x86_64-pc-windows-msvc"];

const plainTargets = [
  // macOS
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
  // Linux
  "x86_64-unknown-linux-musl",
  "aarch64-unknown-linux-musl",
  "arm-unknown-linux-musleabi",
];

module.exports.exeTargets = exeTargets;
module.exports.plainTargets = plainTargets;
