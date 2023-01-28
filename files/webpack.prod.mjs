import { merge } from 'webpack-merge';
import common from './webpack.common.mjs';
import MiniCssExtractPlugin from 'mini-css-extract-plugin';
import { BundleAnalyzerPlugin } from 'webpack-bundle-analyzer';
import { liferayExteranls } from './util/liferayExternals.mjs';

const plugins = [new MiniCssExtractPlugin()];

export default env =>
	merge(common, {
		mode: 'production',
		plugins: env?.analyze ? [...plugins, new BundleAnalyzerPlugin()] : plugins,
		externals: {
			...liferayExteranls,
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
		},
	});
