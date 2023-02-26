import { liferayExternals } from "./util/liferayExternals.js";
import path, { dirname } from "path";
import { fileURLToPath } from "url";
import workspaceConfig from './util/workspaceConfig.js';

const __dirname = dirname(fileURLToPath(import.meta.url));

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
