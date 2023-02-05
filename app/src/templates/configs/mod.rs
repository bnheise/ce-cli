pub mod cypress;

pub const ESLINTRC_FILENAME: &str = ".eslintrc.yml";
pub const ESLINTRC: &str = include_str!("./.eslintrc.yml");

pub const GITIGNORE_FILENAME: &str = ".gitignore";
pub const GITIGNORE: &str = include_str!("./.gitignore");

pub const PRETTIERRCE_FILENAME: &str = ".prettierrc.yaml";
pub const PRETTIERRC: &str = include_str!("./.prettierrc.yaml");

pub const PACKAGEJSON_FILENAME: &str = "package.json";
pub const PACKAGEJSON: &str = include_str!("./package.json");

pub const TSCONFIG_PROD_FILENAME: &str = "tsconfig.prod.json";
pub const TSCONFIG_PROD: &str = include_str!("./tsconfig.prod.json");

pub const TSCONFIG_BASE_FILENAME: &str = "tsconfig.json";
pub const TSCONFIG_BASE: &str = include_str!("./tsconfig.json");

pub const TSCONFIG_DEV_FILENAME: &str = "tsconfig.dev.json";
pub const TSCONFIG_DEV: &str = include_str!("./tsconfig.dev.json");

pub const WEBPACK_CONFIG_COMMON_FILENAME: &str = "webpack.common.js";
pub const WEBPACK_CONFIG_COMMON: &str = include_str!("./webpack.common.js");

pub const WEBPACK_CONFIG_DEV_FILENAME: &str = "webpack.dev.js";
pub const WEBPACK_CONFIG_DEV: &str = include_str!("./webpack.dev.js");

pub const WEBPACK_CONFIG_PROD_FILENAME: &str = "webpack.prod.js";
pub const WEBPACK_CONFIG_PROD: &str = include_str!("./webpack.prod.js");

pub const DOCKERFILE_FILENAME: &str = "Dockerfile";
pub const DOCKERFILE: &str = include_str!("./Dockerfile");

pub const CLIENT_EXT_YAML_FILENAME: &str = "client-extension.yaml";
pub const CLIENT_EXT_YAML: &str = include_str!("./client-extension.yaml");

pub const LCP_JSON_FILENAME: &str = "LCP.json";
pub const LCP_JSON: &str = include_str!("./LCP.json");

pub const WORKSPACE_CONFIG_FILENAME: &str = "workspace-config.json";

pub const JEST_CONFIG_JSON_FILENAME: &str = "jest.config.json";
pub const JEST_CONFIG: &str = include_str!("./jest.config.json");

pub const CYPRESS_CONFIG_JSON_FILENAME: &str = "cypress.config.js";
pub const CYPRESS_CONFIG_JSON: &str = include_str!("./cypress.config.js");

pub const ENV_FILENAME: &str = ".env";
pub const ENV_FILE: &str = include_str!("./.env");
