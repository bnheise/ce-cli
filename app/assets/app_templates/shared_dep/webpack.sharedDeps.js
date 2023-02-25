import { liferayExternals } from "./util/liferayExternals.js";
import path, { dirname } from "path";
import { fileURLToPath } from "url";
import { createRequire } from "module";

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const workspaceConfig = require("./workspace-config.json");

export default {
  entry: { ...workspaceConfig.sharedDependencies },
  mode: "production",
  resolve: {
    extensions: [".js"],
  },
  externals: {
    ...liferayExternals,
  },
  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, "sharedDeps"),
    clean: true,
    module: true,
    libraryTarget: "module",
  },
  optimization: {
    minimize: true,
  },
  experiments: {
    outputModule: true,
  },
};
