import { defineConfig } from 'cypress';
import WebpackProdConfig from './webpack.dev.js';
import dotenv from 'dotenv';
import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const workspaceConfig = require('./workspace-config.json');

dotenv.config();

const CYPRESS_PREFIX = 'CYPRESS_';

export default defineConfig({
	component: {
		devServer: {
			framework: workspaceConfig.framework.toLowerCase(),
			bundler: 'webpack',
			webpackConfig: WebpackProdConfig,
		},
	},

	e2e: {
		env: Object.keys(process.env)
			.filter(env_var => env_var.startsWith('CYPRESS_'))
			.map(key => key.replace(CYPRESS_PREFIX, ''))
			.reduce((env, key) => {
				env[key] = process.env[`${CYPRESS_PREFIX}${key}`];
				return env;
			}, {}),

		setupNodeEvents(on, config) {
			// events here
		},
	},
});
