import { merge } from 'webpack-merge';
import common from './webpack.common.js';
import MiniCssExtractPlugin from 'mini-css-extract-plugin';
import { liferayExternals } from './util/liferayExternals.js';
import frameworkSettings from './util/frameworkSettings.js';
import workspaceConfig from './util/workspaceConfig.js';

export default () =>
	merge(common, {
		mode: 'production',
		plugins: [
			new MiniCssExtractPlugin({
				filename: "[name].[contenthash].css"
			}),
			...frameworkSettings.prod.webpackPlugins,
		],
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
								...frameworkSettings.common.tsOptions,
							},
						},
					],
				},
			],
		},
		output: {
			filename: '[name].[contenthash].js',
			module: true,
			libraryTarget: 'module',
		},
		experiments: {
			outputModule: true,
		},
	});
