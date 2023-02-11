// Do not modify this file;

import { {{ name-camelcase }} } from './customElement';
import "./widget.scss";

export const ELEMENT_ID = '{{ element-name }}';

if (!customElements.get(ELEMENT_ID)) {
	customElements.define(ELEMENT_ID, {{ name-camelcase }});
}
