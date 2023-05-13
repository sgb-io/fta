#!/usr/bin/env node

const { execSync } = require("node:child_process");
const path = require("node:path");

const { exeTargets, plainTargets } = require("./targets");

const platform = process.platform;
const architecture = process.arch;

function getBinaryPath() {
  const targetDirectory = path.join(__dirname, "bin");

  switch (platform) {
    case "win32":
      if (architecture === "x64") {
        return path.join(
          targetDirectory,
          exeTargets["x86_64-pc-windows-msvc"],
          "fta.exe"
        );
      } else if (architecture === "arm64") {
        return path.join(
          targetDirectory,
          exeTargets["aarch64-pc-windows-msvc"],
          "fta.exe"
        );
      }
    case "darwin":
      if (architecture === "x64") {
        return path.join(
          targetDirectory,
          plainTargets["x86_64-apple-darwin"],
          "fta"
        );
      } else if (architecture === "arm64") {
        return path.join(
          targetDirectory,
          plainTargets["aarch64-apple-darwin"],
          "fta"
        );
      }
    case "linux":
      if (architecture === "x64") {
        return path.join(
          targetDirectory,
          plainTargets["x86_64-unknown-linux-gnu"]
        );
      } else if (architecture === "arm64") {
        return path.join(
          targetDirectory,
          plainTargets["aarch64-unknown-linux-gnu"]
        );
      } else if (architecture === "arm") {
        return path.join(
          targetDirectory,
          plainTargets["armv7-unknown-linux-gnueabihf"]
        );
      }
      break;
    default:
      throw new Error("Unsupported platform: " + platform);
  }

  throw new Error("Binary not found for the current platform");
}

// Run the binary from code
// We build arguments that get sent to the binary
export function runFtaBinary(project, options) {
  const binaryPath = getBinaryPath();
  const binaryArgs = options.json ? "--json" : "";
  const result = execSync(`${binaryPath} ${project} ${binaryArgs}`);
  return result.toString();
}

// Run the binary directly if executed as a standalone script
// Arguments are directly forwarded to the binary
if (require.main === module) {
  const args = process.argv.slice(2); // Exclude the first two arguments (node binary and script file)
  const binaryArgs = args.join(" ");
  const binaryPath = getBinaryPath();
  const result = execSync(`${binaryPath} ${binaryArgs}`);
  console.log(result.toString());
}
