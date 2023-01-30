

import { render } from 'react-dom';
import React from 'react';
import './app.scss';

export class {{app-name-camelcase}} extends HTMLElement {
	connectedCallback() {
		render(<h1>Hello {{custom-element-app-name}}</h1>, this);
	}
}
