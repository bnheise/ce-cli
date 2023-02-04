# Client Extension Workspace

This is generated Client Extension Workspace for building frontend customizations for Liferay DXP 7.4+
and Lifery Experience Cloud.

## Features

- Typescript support out of the box
- Does not import packages that are available in Liferay, eg react, react-dom, all of clayui, significantly reducing your bundle size
- Bundle multiple applications in a single docker container for efficient use of resources
- Integration with Liferay Workspace
- Testing with Jest and Cypress configured OOTB
- Start the dev server and have your apps available in Liferay immediately -- no need to update client-extension.yaml

## How to Use

### Scripts

- `npm run start`: deploy a dev configuration of your client extensions to Liferay and starts the dev server
- `npm run test:unit`: run all Jest unit tests
- `npm run build`: runs all tests and then carries out the webpack build
- `npm run deploy`: calls Liferay workspace to run the complete deploy script to your local instance
