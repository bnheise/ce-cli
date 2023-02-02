#!/usr/bin/env node

const { Binary } = require("binary-install");
const os = require("os");
const { version, name, repository } = require("../package.json");
const supportedPlatforms = require("../platforms.json");

const error = (msg) => {
  console.error(msg);
  process.exit(1);
};

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
  const { RUST_TARGET, BINARY_NAME } = getPlatformMetadata();

  const url = `${repository.url}/releases/download/${version}/${name}-${RUST_TARGET}.tar.gz`;

  return new Binary(BINARY_NAME, url);
}
getBinary();
module.exports = getBinary;
