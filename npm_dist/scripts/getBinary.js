#!/usr/bin/env node

import { Binary } from "binary-install";
import * as os from "os";
import { createRequire } from "module";

const require = createRequire(import.meta.url);
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

const getBinary = () => {
  const { RUST_TARGET, BINARY_NAME } = getPlatformMetadata();
  const url = `${repository.url}/releases/download/${version}/${name}-${RUST_TARGET}.tar.gz`;
  return new Binary(BINARY_NAME, url);
};

export default getBinary;
