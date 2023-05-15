declare namespace Liferay {
	const AUI: AUI;
	const Address: Address;
	const BREAKPOINTS: Breakpoints;
	const Browser: Browser;
	const CollapseProvider: CollapseProvider;
	const CustomDialogs: {
		enabled: boolean;
	}
	const Util: Util;
	const ThemeDisplay: ThemeDisplay;
	const FeatureFlags: FeatureFlags;
	const Icons: Icons;
	const Language: Language;
	const OAuth2: OAuth2;
	const OAuth2Client: OAuth2ClientLoader;
	const PortletKeys: PortletKeys;
	const PropsValues: PropsValues;
	const SPA: {
		app: SpaApp;
	};
	const STATUS_CODE: StatusCode;
	const authToken: string;
}

type Util = {
	fetch: typeof window.fetch;
};

type ThemeDisplay = {
	getLanguageId: () => string;
	getDefaultLanguageId: () => string;
	getBCP47LanguageId: () => string;
	getCDNBaseURL: () => string;
	getCDNDynamicResourcesHost: () => string;
	getCDNHost: () => string;
	getCanonicalURL: () => string;
	getCompanyGroupId: () => string;
	getCompanyId: () => string;
	getDefaultLanguageId: () => string;
	getDoAsUserIdEncoded: () => string;
	getLanguageId: () => string;
	getLayoutId: () => string;
	getLayoutRelativeControlPanelURL: () => string;
	getLayoutRelativeURL: () => string;
	getLayoutURL: () => string;
	getParentGroupId: () => string;
	getParentLayoutId: () => string;
	getPathContext: () => string;
	getPathImage: () => string;
	getPathJavaScript: () => string;
	getPathMain: () => string;
	getPathThemeImages: () => string;
	getPathThemeRoot: () => string;
	getPathThemeSpritemap: () => string;
	getPlid: () => string;
	getPortalURL: () => string;
	getRealUserId: () => string;
	getRemoteAddr: () => string;
	getRemoteHost: () => string;
	getScopeGroupId: () => string;
	getScopeGroupIdOrLiveGroupId: () => string;
	getSessionId: () => string;
	getSiteAdminURL: () => string;
	getSiteGroupId: () => string;
	getURLControlPanel: () => string;
	getURLHome: () => string;
	getUserEmailAddress: () => string;
	getUserId: () => string;
	getUserName: () => string;
	isAddSessionIdToURL: () => boolean;
	isControlPanel: () => boolean;
	isImpersonated: () => boolean;
	isPrivateLayout: () => boolean;
	isSignedIn: () => boolean;
	isStagedPortlet: () => boolean;
	isStateExclusive: () => boolean;
	isStateMaximized: () => boolean;
	isStatePopUp: () => boolean;
	isVirtualLayout: () => boolean;
};

type Breakpoints = {
	PHONE: number;
	TABLET: number;
};

type Browser = {
	acceptsGzip: () => boolean;
	getMajorVersion: () => number;
	getRevision: () => number;
	getVersion: () => number;
	isAir: () => boolean;
	isChrome: () => boolean;
	isEdge: () => boolean;
	isFirefox: () => boolean;
	isGecko: () => boolean;
	isIe: () => boolean;
	isIphone: () => boolean;
	isLinux: () => boolean;
	isMac: () => boolean;
	isMobile: () => boolean;
	isMozilla: () => boolean;
	isOpera: () => boolean;
	isRtf: () => boolean;
	isSafari: () => boolean;
	isSun: () => boolean;
	isWebKit: () => boolean;
	isWindows: () => boolean;
};

type FeatureFlags = {
	'COMMERCE-5898': boolean;
	'COMMERCE-8087': boolean;
	'COMMERCE-8949': boolean;
	'COMMERCE-9410': boolean;
	'LPS-83384': boolean;
	'LPS-114786': boolean;
	'LPS-119382': boolean;
	'LPS-135430': boolean;
	'LPS-142518': boolean;
	'LPS-144527': boolean;
	'LPS-153324': boolean;
	'LPS-154672': boolean;
	'LPS-155284': boolean;
	'LPS-155692': boolean;
	'LPS-156421': boolean;
	'LPS-157670': boolean;
	'LPS-158259': boolean;
	'LPS-158675': boolean;
	'LPS-159643': boolean;
	'LPS-161364': boolean;
	'LPS-161631': boolean;
	'LPS-162765': boolean;
	'LPS-162964': boolean;
	'LPS-162966': boolean;
	'LPS-163118': boolean;
	'LPS-163688': boolean;
	'LPS-164948': boolean;
	'LPS-165476': boolean;
	'LPS-165482': boolean;
	'LPS-165493': boolean;
	'LPS-166036': boolean;
	'LPS-166275': boolean;
	'LPS-166479': boolean;
	'LPS-169923': boolean;
	'LPS-169981': boolean;
	'LPS-170670': boolean;
	'LRAC-10632': boolean;
	'LRAC-10757': boolean;
	'ui.visible[beta]': boolean;
	'ui.visible[dev]': boolean;
	'ui.visible[release]': boolean;
	[key: string]: boolean | undefined;
};

type Icons = {
	spritemap: string;
};

type Language = {
	available: { [key: string]: string };
	direction: { [key: string]: string };
};

type OAuth2 = {
	getAuthorizeURL: () => string;
	getBuiltInRedirectURL: () => string;
	getIntrospectURL: () => string;
	getTokenURL: () => string;
	getUserAgentApplication: (externalReferenceCode) => unknown;
};

type OAuth2ClientLoader = {
	FromParameters: (params: OAuth2Parameters) => OAuth2Parameters;
	FromUserAgentApplication: (userAgent: string) => OAuth2Parameters;
};

type PortletKeys = {
	DOCUMENT_LIBRARY: string;
	DYNAMIC_DATA_MAPPING: string;
	ITEM_SELECTOR: string;
};

type PropsValues = {
	JAVASCRIPT_SINGLE_PAGE_APPLICATION_TIMEOUT: number;
	UPLOAD_SERVLET_REQUEST_IMPL_MAX_SIZE: number;
};

type SpaApp = {
	activeScreen: Screen;
	activePath: string;
	allowPreventNavigate: boolean;
	basePath: string;
	browserPathBeforeNavigate: string;
	captureScrollPositionFromScrollEvent: boolean;
	defaultTitle: string;
	formSelector: string;
	ignoreQueryStringFromRoutePath: boolean;
	linkSelector: string;
	loadingCssClass: string;
	nativeScrollRestorationSupported: boolean;
	navigationStrategy: string;
	isNavigationPending: boolean;
	pendingNavigate: unknown | null;
	popstateScrollLeft: number;
	popstateScrollTop: number;
	redirectPath: string;
	routes: object[];
	scheduledNavigationQueue: unknown[];
	screens: {
		[key: string]: Screen;
	};
	skipLoadPopstate: boolean;
	surfaces: {
		senna_surface1: {
			activeChild: object;
			id: string;
			transitionFn: null | function;
		};
	};
	updateScrollPosition: boolean;
	portletsBlacklist: string[];
	userNotification: {
		message: string;
		title: 'string';
		timeout: number;
	};
	validStatusCodes: number[];
	timeout: number;
	timeoutAlert: unknown | null;
	requestTimer: number;
};

type Screen = {
	cache: string;
	cacheable: boolean;
	id: string;
	metas: object[];
	title: string;
	httpHeaders: {
		[key: string]: string;
	};
	httpMethod: string;
	request: {
		method: string;
		requestBody: object | null;
		requestHeaders: {
			[key: string]: string;
		};
		url: string;
	};
	timeout: number;
	metaTagsSelector: string;
	titleSelector: string;
	response: object;
	cacheLastModified: number;
	virtualDocument: object | null;
	pendingStyles: unknown | null;
};

type StatusCode = {
	BAD_REQUEST: 400;
	INTERNAL_SERVER_ERROR: 500;
	OK: 200;
	SC_DUPLICATE_FILE_EXCEPTION: 490;
	SC_FILE_ANTIVIRUS_EXCEPTION: 494;
	SC_FILE_CUSTOM_EXCEPTION: 499;
	SC_FILE_EXTENSION_EXCEPTION: 491;
	SC_FILE_NAME_EXCEPTION: 492;
	SC_FILE_SIZE_EXCEPTION: 493;
	SC_UPLOAD_REQUEST_SIZE_EXCEPTION: 495;
};

type AUI = {
	getCombine(): boolean;
	getComboPath(): string;
	getDateFormat: string;
	getEditorCKEditorPath(): string;
	getFilter(): string;
	getFilterConfig(): object | null;
	getJavaScriptRootPath(): string;
	getPortletRootPath: string;
	getStaticResourceURLParams(): string;
}

type Address = {
	getCountries(callback: (countries: Country[]) => void);
	getRegions(callback: (regions: Region[]) => void, countryId: string);
}

interface Country {
	a2: string;
	a3: string;
	active: boolean;
	billingAllowed: boolean;
	companyId: string;
	countryId: string;
	createDate: number;
	defaultLanguageId: string;
	groupFilterEnabled: boolean;
	idd: string;
	lastPublishDate: Date | null;
	modifiedDate: number;
	mvccVersion: string;
	name: string;
	nameCurrentValue: string;
	number: string;
	position: number;
	shippingAllowed: boolean;
	subjectToVAT: boolean;
	userId: string;
	userName: string;
	uuid: string;
	zipRequired: boolean;
}

interface Region {
	active: boolean;
	companyId: string;
	countryId: string;
	createDate: number;
	defaultLanguageId: string;
	lastPublishDate: number | null;
	modifiedDate: number;
	mvccVersion: string;
	name: string;
	position: number;
	regionCode: string;
	regionId: string;
	title: string;
	userId: string;
	userName: string;
	uuid: string;
}

type CollapseProvider = {
	EVENT_HIDDEN: string;
	EVENT_HIDE: string;
	EVENT_SHOW: string;
	EVENT_SHOWN: string;
	hide: (input: HideShow) => void;
	show: (input: HideShow) => void;
};

interface HideShow {
	panel: unknown; // TODO: find type
	trigger: unknown; // TODO; find type
}