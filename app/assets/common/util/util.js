import { fileURLToPath } from 'url';
import path, { dirname } from 'path';

const __dirname = dirname(fileURLToPath(import.meta.url));

export const resolveAliases = workspaceConfig =>
	Object.entries(workspaceConfig.alias)
		.map(([key, pathArray]) => [
			key,
			pathArray.map(filepath => path.resolve(__dirname, filepath)),
		])
		.reduce((obj, [key, pathArray]) => {
			obj[key] = pathArray;
			return obj;
		}, {});
