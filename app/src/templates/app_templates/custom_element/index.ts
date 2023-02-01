// Do not modify this file;

import { {{app-name-camelcase}} } from './customElement';
import "./widget.scss";

export const ELEMENT_ID = '{{custom-element-name}}';

if (!customElements.get(ELEMENT_ID)) {
	customElements.define(ELEMENT_ID, {{app-name-camelcase}});
}
