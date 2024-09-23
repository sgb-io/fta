// Windows
const exeTargets = [
  "fta-aarch64-pc-windows-msvc",
  "fta-x86_64-pc-windows-msvc",
];

const plainTargets = [
  // macOS
  "fta-x86_64-apple-darwin",
  "fta-aarch64-apple-darwin",
  // Linux
  "fta-x86_64-unknown-linux-musl",
  "fta-aarch64-unknown-linux-musl",
  "fta-arm-unknown-linux-musleabi",
];

module.exports.exeTargets = exeTargets;
module.exports.plainTargets = plainTargets;
