import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const workspaceConfig = require('../workspace-config.json');

export default workspaceConfig;
