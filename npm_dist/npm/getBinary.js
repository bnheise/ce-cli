#!/usr/bin/env node

const { Binary } = require("binary-install");
const os = require("os");
const { version, name, repository } = require("./package.json");

const error = (msg) => {
  console.error(msg);
  process.exit(1);
};

const supportedPlatforms = [
  {
    TYPE: "Windows_NT",
    ARCHITECTURE: "x64",
    RUST_TARGET: "x86_64-pc-windows-msvc",
    BINARY_NAME: "ce-cli.exe",
  },
  {
    TYPE: "Linux",
    ARCHITECTURE: "x64",
    RUST_TARGET: "x86_64-unknown-linux-musl",
    BINARY_NAME: "ce-cli",
  },
  {
    TYPE: "Darwin",
    ARCHITECTURE: "x64",
    RUST_TARGET: "x86_64-apple-darwin",
    BINARY_NAME: "ce-cli",
  },
];

const getPlatformMetadata = () => {
  const type = os.type();
  const architecture = os.arch();

  for (let supportedPlatform of supportedPlatforms) {
    if (
      type === supportedPlatform.TYPE &&
      architecture === supportedPlatform.ARCHITECTURE
    ) {
      return supportedPlatform;
    }
  }

  error(
    `Platform with type "${type}" and architecture "${architecture}" is not supported by ${name}.\nYour system must be one of the following:\n\n${cTable.getTable(
      supportedPlatforms
    )}`
  );
};

function getBinary() {
  const platformMetadata = getPlatformMetadata();

  const url = `${repository.url}/releases/download/v${version}/${name}-${platformMetadata.RUST_TARGET}.tar.gz`;
  const name = "ce-cli";
  return new Binary(name, url);
}

module.exports = getBinary;
