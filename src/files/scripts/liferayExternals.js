const O = 'o';
const FRONTEND_JS_REACT_WEB = 'frontend-js-react-web';
const FRONTEND_TAGLIB_CLAY = 'frontend-taglib-clay';
const __LIFERAY__ = '__liferay__';
const EXPORTS = 'exports';
const CLAYUI = '@clayui';

const reactModules = [
	'classnames',
	'formik',
	'prop-types',
	'react',
	'react-dnd',
	'react-dnd-html5-backend',
	'react-dom',
];

const clayModules = [
	'alert',
	'autocomplete',
	'badge',
	'breadcrumb',
	'button',
	'card',
	'charts',
	'color-picker',
	'data-provider',
	'date-picker',
	'drop-down',
	'empty-state',
	'form',
	'core',
	'icon',
	'label',
	'layout',
	'link',
	'list',
	'loading-indicator',
	'localized-input',
	'management-toolbar',
	'modal',
	'multi-select',
	'multi-step-nav',
	'nav',
	'navigation-bar',
	'pagination',
	'pagination-bar',
	'panel',
	'popover',
	'progress-bar',
	'slider',
	'sticker',
	'table',
	'tabs',
	'time-picker',
	'toolbar',
	'tooltip',
	'upper-toolbar',
	'css',
].map(baseModuleName => `${CLAYUI}/${baseModuleName}`);

const buildFilename = (moduleName, ext) =>
	`${moduleName}.${ext}`.replace('/', '$');
const buildImportPath = (parts, fileName) =>
	`/${[...parts, fileName].join('/')}`;

const toImportPath = parts => (obj, moduleName) => {
	const fileName = buildFilename(moduleName, 'js');
	obj[moduleName] = buildImportPath(parts, fileName);
	return obj;
};

const reactExternals = reactModules.reduce(
	toImportPath([O, FRONTEND_JS_REACT_WEB, __LIFERAY__, EXPORTS]),
	{}
);

const clayExternals = clayModules.reduce(
	toImportPath([O, FRONTEND_TAGLIB_CLAY, __LIFERAY__, EXPORTS]),
	{}
);

export const liferayExteranls = { ...reactExternals, ...clayExternals };
