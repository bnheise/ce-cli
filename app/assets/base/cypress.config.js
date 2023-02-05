import { defineConfig } from 'cypress';
import WebpackProdConfig from './webpack.dev.js';
import dotenv from 'dotenv';

dotenv.config();

const CYPRESS_PREFIX = 'CYPRESS_';

export default defineConfig({
	component: {
		devServer: {
			framework: '{{ framework }}',
			bundler: 'webpack',
			webpackConfig: WebpackProdConfig,
		},
	},

	e2e: {
		// eslint-disable-next-line no-undef
		env: Object.keys(process.env)
			.filter(env_var => env_var.startsWith('CYPRESS_'))
			.map(key => key.replace(CYPRESS_PREFIX, ''))
			.reduce((env, key) => {
				// eslint-disable-next-line no-undef
				env[key] = process.env[`${CYPRESS_PREFIX}${key}`];
				return env;
			}, {}),
		// eslint-disable-next-line @typescript-eslint/no-unused-vars
		setupNodeEvents(on, config) {
			// events here
		},
	},
});
