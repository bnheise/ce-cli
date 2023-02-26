import ESLintWebpackPlugin from "eslint-webpack-plugin";
import path, { dirname } from "path";
import { fileURLToPath } from "url";
import { createRequire } from "module";
{{ framework-imports }}

const require = createRequire(import.meta.url);
const workspaceConfig = require("./workspace-config.json");
const __dirname = dirname(fileURLToPath(import.meta.url));

const aliasResolved = Object.entries(workspaceConfig.alias)
  .map(([key, pathArray]) => [
    key,
    pathArray.map((filepath) => path.resolve(__dirname, filepath)),
  ])
  .reduce((obj, [key, pathArray]) => {
    obj[key] = pathArray;
    return obj;
  }, {});

export default {
  entry: workspaceConfig.entrypoints,
  plugins: [new ESLintWebpackPlugin(), new VueLoaderPlugin()],
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
        },
      },
      {{ framework-rules }}
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
    alias: { ...aliasResolved },
  },
  experiments: {
    outputModule: true,
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    clean: true,
    module: true,
  },
};
