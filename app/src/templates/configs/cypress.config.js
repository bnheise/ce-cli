import { defineConfig } from 'cypress';
import WebpackProdConfig from './webpack.dev.js';

export default defineConfig({
	component: {
		devServer: {
			framework: 'react',
			bundler: 'webpack',
			webpackConfig: WebpackProdConfig,
		},
	},
});
