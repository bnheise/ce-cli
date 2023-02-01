pub const ESLINTRC_FILENAME: &str = ".eslintrc.yml";
pub const ESLINTRC: &str = include_str!("files/configs/.eslintrc.yml");

pub const GITIGNORE_FILENAME: &str = ".gitignore";
pub const GITIGNORE: &str = include_str!("files/configs/.gitignore");

pub const PRETTIERRCE_FILENAME: &str = ".prettierrc.yaml";
pub const PRETTIERRC: &str = include_str!("files/configs/.prettierrc.yaml");

pub const PACKAGEJSON_FILENAME: &str = "package.json";
pub const PACKAGEJSON: &str = include_str!("files/configs/package.json");

pub const TSCONFIG_PROD_FILENAME: &str = "tsconfig.prod.json";
pub const TSCONFIG_PROD: &str = include_str!("files/configs/tsconfig.prod.json");

pub const TSCONFIG_BASE_FILENAME: &str = "tsconfig.json";
pub const TSCONFIG_BASE: &str = include_str!("files/configs/tsconfig.json");

pub const TSCONFIG_DEV_FILENAME: &str = "tsconfig.dev.json";
pub const TSCONFIG_DEV: &str = include_str!("files/configs/tsconfig.dev.json");

pub const WEBPACK_CONFIG_COMMON_FILENAME: &str = "webpack.common.js";
pub const WEBPACK_CONFIG_COMMON: &str = include_str!("files/configs/webpack.common.js");

pub const WEBPACK_CONFIG_DEV_FILENAME: &str = "webpack.dev.js";
pub const WEBPACK_CONFIG_DEV: &str = include_str!("files/configs/webpack.dev.js");

pub const WEBPACK_CONFIG_PROD_FILENAME: &str = "webpack.prod.js";
pub const WEBPACK_CONFIG_PROD: &str = include_str!("files/configs/webpack.prod.js");

pub const DOCKERFILE_FILENAME: &str = "Dockerfile";
pub const DOCKERFILE: &str = include_str!("files/configs/Dockerfile");

pub const CLIENT_EXT_YAML_FILENAME: &str = "client-extension.yaml";
pub const CLIENT_EXT_YAML: &str = include_str!("files/configs/client-extension.yaml");

pub const LCP_JSON_FILENAME: &str = "LCP.json";
pub const LCP_JSON: &str = include_str!("files/configs/LCP.json");

pub const LIFERAY_EXTERNALS_FILENAME: &str = "util/liferayExternals.js";
pub const LIFERAY_EXTERNALS: &str = include_str!("files/scripts/liferayExternals.js");

pub const WORKSPACE_CONFIG_FILENAME: &str = "workspace-config.json";

pub const CUSTOM_ELEMENT_CSS: &str = include_str!("files/app_templates/custom_element/app.scss");
pub const CUSTOM_ELEMENT_CSS_FILENAME: &str = "app.scss";

pub const CUSTOM_ELEMENT_INDEX: &str = include_str!("files/app_templates/custom_element/index.ts");
pub const CUSTOM_ELEMENT_INDEX_FILENAME: &str = "index.ts";

pub const CUSTOM_ELEMENT_APP: &str = include_str!("files/app_templates/custom_element/app.tsx");
pub const CUSTOM_ELEMENT_APP_FILENAME: &str = "app.tsx";

pub const CUSTOM_ELEMENT_APP_NAME_CAMEL: &str = "{{app-name-camelcase}}";
pub const CUSTOM_ELEMENT_NAME: &str = "{{custom-element-name}}";
pub const CUSTOM_ELEMENT_APP_NAME: &str = "{{custom-element-app-name}}";

pub const BUILD_DIR: &str = "./build";
// pub const DIST_DIR: &str = "./dist";
pub const CLIENT_EXTENSION: &str = "clientExtension";

pub const CET_CONFIG_FULLY_QUALIFIED_PATH: &str =
    "com.liferay.client.extension.type.configuration.CETConfiguration";

pub const CET_CONFIG_FILENAME_BASE: &str = "client-extension-config.json";

pub const OSGI: &str = "osgi";
pub const CLIENT_EXTENSIONS: &str = "client-extensions";
