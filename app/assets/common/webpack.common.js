import ESLintWebpackPlugin from 'eslint-webpack-plugin';
import path, { dirname } from 'path';
import { fileURLToPath } from 'url';
import { resolveAliases } from './util/util.js';
import workspaceConfig from './util/workspaceConfig.js';
import frameworkSettings from './util/frameworkSettings.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const aliasResolved = resolveAliases(workspaceConfig);

export default {
	entry: workspaceConfig.entrypoints,
	plugins: [
		new ESLintWebpackPlugin(),
		...frameworkSettings.common.webpackPlugins,
	],
	module: {
		rules: [
			{
				test: /\.js$/,
				exclude: /node_modules/,
				use: {
					loader: 'babel-loader',
				},
			},
			...frameworkSettings.common.webpackRules,
		],
	},
	resolve: {
		extensions: ['.tsx', '.ts', '.js'],
		alias: { ...aliasResolved },
	},
	experiments: {
		outputModule: true,
	},
	output: {
		path: path.resolve(__dirname, 'dist'),
		clean: true,
		module: true,
	},
};
