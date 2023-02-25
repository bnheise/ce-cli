import { merge } from 'webpack-merge';
import common from './webpack.common.js';
import MiniCssExtractPlugin from 'mini-css-extract-plugin';
import { liferayExternals } from './util/liferayExternals.js';
import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const workspaceConfig = require('./workspace-config.json');

export default env =>
	merge(common, {
		mode: 'production',
		plugins: [new MiniCssExtractPlugin()],
		externals: {
			...liferayExternals,
			...workspaceConfig.externals,
		},
		module: {
			rules: [
				{
					test: /\.css$/i,
					exclude: /node_modules/,
					use: [MiniCssExtractPlugin.loader, 'css-loader'],
					sideEffects: true,
				},
				{
					test: /\.s[ac]ss$/i,
					exclude: /node_modules/,
					use: [MiniCssExtractPlugin.loader, 'css-loader', 'sass-loader'],
					sideEffects: true,
				},
				{
					test: /\.tsx?$/,
					exclude: /node_modules/,
					use: [
						{
							loader: 'ts-loader',
							options: {
								configFile: 'tsconfig.prod.json',
							},
						},
					],
				},
			],
		},
		output: {
			filename: '[name].js',
			module: true,
			libraryTarget: 'module',
		},
		experiments: {
			outputModule: true,
		},
	});
