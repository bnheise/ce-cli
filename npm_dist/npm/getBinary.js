const { Binary } = require("binary-install");
const os = require("os");

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") return "x86_64-pc-windows-gnu";
  if (type === "Linux" && arch === "x64") return "aarch64-unknown-linux-gnu";
  if (type === "Darwin" && arch === "x64") return "x86_64-apple-darwin";

  throw new Error(`Unsupported platform: ${type} ${arch}`);
}

function getBinary() {
  const platform = getPlatform();
  const version = require("../package.json").version;
  const url = `https://github.com/bnheise/ce-cli/releases/download/v${version}/ce-cli-${platform}.tar.gz`;
  const name = "ce-cli";
  return new Binary(url, { name });
}

module.exports = getBinary;
