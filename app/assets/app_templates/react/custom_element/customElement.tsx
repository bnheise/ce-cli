import { render } from 'react-dom';
import React from 'react';
import Widget from './Widget';

// Reference: https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_custom_elements
export class {{ appNameCamelcase }} extends HTMLElement {
	constructor() {
    super();
  }
	connectedCallback() {
		render(<Widget />, this);
	}

	disconnectedCallback() {}

	adoptedCallback() {}

}
