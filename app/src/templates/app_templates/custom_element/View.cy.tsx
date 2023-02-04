import React from 'react';
import View from './View';

describe('<View />', () => {
	it('renders', () => {
		// see: https://on.cypress.io/mounting-react
		cy.mount(<View />);
	});
});
