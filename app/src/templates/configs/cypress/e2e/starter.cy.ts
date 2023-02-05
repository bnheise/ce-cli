/// <reference types="cypress" />

import { login } from './util';

// Welcome to End to End testing!
//
// Here you can test your client extension as it runs when deployed to your Liferay instance.
// Although it tests only in your local dev instance, which isn't as good as doing it in
// a proper UAT environment, this nonetheless will help you and your team have confidence
// that your app integrated properly with Liferay.
//
// These tests are power by Cypress. To learn more about how Cypress works
// please read their getting started guide:
// https://on.cypress.io/introduction-to-cypress

describe('liferay login flow', () => {
	beforeEach(() => {
		// Cypress starts out with a blank slate for each test
		// so we must tell it to visit our website with the `cy.visit()` command.
		// Since we want to visit the same URL at the start of all our tests,
		// we include it in our beforeEach function so that it runs before each test

		// Note that we have hardcoded localhost:8080 since that is the default Liferay url
		// your locally running intance. Feel free to change this.
		cy.visit('http://localhost:8080');
	});

	it('logging in works', () => {
		// Here we call a utility function that logs us in as a specific user.
		// We get the username and password from the .env file with the keys CYPRESS_LIFERAY_ADMIN_USER
		// and CYPRESS_LIFERAY_ADMIN_PASS. We need the Cypress keyword to prevenr exposing environment
		// variables that we don't want in the browser.
		login(Cypress.env('LIFERAY_ADMIN_USER'), Cypress.env('LIFERAY_ADMIN_PASS'));
	});
});
