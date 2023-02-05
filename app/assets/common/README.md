# Client Extension Workspace

This is generated Client Extension Workspace for building frontend customizations for Liferay DXP 7.4
and Lifery Experience Cloud.

## Features

- Typescript support out of the box
- Does not import packages that are available in Liferay, eg react, react-dom, all of clayui, significantly reducing your bundle size
- Bundles multiple client extensions in a single docker container for efficient use of resources
- Integration with Liferay Workspace
- Testing with Jest and Cypress configured OOTB
- Start the dev server and have your apps available in Liferay immediately -- no need to update client-extension.yaml

## How to Use

### Scripts

- `npm run start`: deploy a dev configuration of your client extensions to Liferay and starts the dev server
- `npm run test:unit`: run all Jest unit tests
- `npm run test:component`: run all Cypress unit tests
- `npm run test:e2e`: Run Cypress End to End tests -- test your client extension in a real Liferay environment (requires a Liferay instance running locally)
- `npm run build`: runs all unit and component tests and then carries out the webpack build
- `npm run deploy`: calls Liferay workspace to run the complete deploy script to your local instance.

### Best Practices

#### Test Driven Development

This workspace enables Test Driven Development -- write tests first and then write code that makes them pass.
To achieve this, start with a Cypress End to End test. Define the workflow that you're targetting programmatically.
Ask yourself, what does my user want to do and what steps will he or she take to do it?

After that, think about your custom component itself -- absent Liferay and all of the headless APIs and Liferay Objects
and whatever else it integrates with, what specifically should it do? Write a component that expects all external input from
Liferay and other sources to be passed to it. This enables component testing, making it possible to send mock data easily.
Write Cypress component tests for this ideal component.

Finally, what supporting logic does your client extension need to function? Identify these key functions and write tests with Jest. Keep in mind -- Jest is for testing pure logic with no inherent visual representation. Do not test IO and do not test
how it should be rendered on the page. IO is for End to End tests and proper rendering is for Component tests.

Now that you have your tests ready, it's time to develop your component. Go out and make the tests pass!
