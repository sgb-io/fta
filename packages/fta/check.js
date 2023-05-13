#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

const { exeTargets, plainTargets } = require("./targets");

let missingBinaries = [];

function checkBinaryFile(target, path) {
  try {
    fs.readFileSync(path);
  } catch (e) {
    missingBinaries.push(target);
  }
}

for (let i = 0; i < exeTargets.length; i += 1) {
  const bin = path.join(__dirname, "bin", exeTargets[i], "fta.exe");
  checkBinaryFile(exeTargets[i], bin);
}

for (let i = 0; i < plainTargets.length; i += 1) {
  const bin = path.join(__dirname, "bin", plainTargets[i], "fta");
  checkBinaryFile(plainTargets[i], bin);
}

if (missingBinaries.length > 0) {
  console.log("The following binaries are missing: \n");
  missingBinaries.forEach((target) => {
    console.log("- " + target);
  });
  console.log("\n");
  throw new Error("Check failed");
}

console.log("All binaries were located");
