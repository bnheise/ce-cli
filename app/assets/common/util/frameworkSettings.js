import { VueLoaderPlugin } from 'vue-loader';
import webpack from 'webpack';

const frameworkSettings = {
	common: {
		webpackPlugins: [new VueLoaderPlugin()],
		webpackRules: [
			{
				test: /\.vue$/,
				loader: 'vue-loader',
				options: {
					compilerOptions: {
						isCustomElement: tag => tag.includes('-'),
					},
				},
			},
		],
		tsOptions: {
			appendTsSuffixTo: [/\.vue$/],
		},
	},
	dev: {
		devServer: {
			watchFiles: ['src/**/*.vue'],
		},
		webpackPlugins: [
			new webpack.DefinePlugin({
				__VUE_OPTIONS_API__: true,
				__VUE_PROD_DEVTOOLS__: true,
			}),
		],
	},
	prod: {
		webpackPlugins: [
			new webpack.DefinePlugin({
				__VUE_OPTIONS_API__: false,
				__VUE_PROD_DEVTOOLS__: false,
			}),
		],
	},
};

export default frameworkSettings;
