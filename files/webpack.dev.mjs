import { merge } from 'webpack-merge';
import common from './webpack.common.mjs';
import Dotenv from 'dotenv-webpack';

const PORT = 3000;

export default merge(common, {
	mode: 'development',
	devtool: 'inline-source-map',
	devServer: {
		static: './public',
		port: PORT,
		hot: true,
		headers: {
			'Access-Control-Allow-Origin': '*',
			'Access-Control-Allow-Methods': '*',
			'Access-Control-Allow-Headers':
				'X-Requested-With, content-type, Authorization',
		},
		liveReload: true,
		watchFiles: ['src/**/*.tsx', 'src/**/*.ts', 'src/**/*.scss'],
		open: false,
	},
	plugins: [new Dotenv()],
	module: {
		rules: [
			{
				test: /\.css$/,
				exclude: /node_modules/,
				use: ['style-loader', 'css-loader'],
				sideEffects: true,
			},
			{
				test: /\.s[ac]ss$/i,
				exclude: /node_modules/,
				use: ['style-loader', 'css-loader', 'sass-loader'],
				sideEffects: true,
			},
			{
				test: /\.tsx?$/,
				exclude: /node_modules/,
				use: {
					loader: 'ts-loader',
					options: {
						configFile: 'tsconfig.dev.json',
					},
				},
			},
		],
	},
	output: {
		filename: '[name].js',
		publicPath: `http://localhost:${PORT}/`,
	},
});
