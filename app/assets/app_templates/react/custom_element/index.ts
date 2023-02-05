// Do not modify this file;

import { {{ appNameCamelcase }} } from './customElement';
import "./widget.scss";

export const ELEMENT_ID = '{{ customElementName }}';

if (!customElements.get(ELEMENT_ID)) {
	customElements.define(ELEMENT_ID, {{ appNameCamelcase }});
}
