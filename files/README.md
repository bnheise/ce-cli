# Client Extension Workspace

This repo is a starter for building Liferay Client Extensions.
It provides a fast and easy way to get started.

## Features

- Typescript support out of the box
- Does not import packages that are available in Liferay, eg react, react-dom, all of clayui, significantly reducing your bundle size.
- Bundle multiple applications in a single docker container for efficient use of resources
- Integration with Liferay Workspace

## How to Use

1. Clone this repository into your Liferay Workspace
2. Update `client-extension.yaml` with the details for your Client Extensions;
3. Make a folder in `./src` for each individual Client Extension. Two started are already available. Feel free to copy them.
4. Make sure the `ELEMENT_ID` variable in each of the above folders matches what you specified in `client-extension.yaml`
5. In `webpack.common.mjs`, add each new Client Extension to the entrypoints parameter.
6. Run the deploy script to deploy to your local Liferay instance. Note that you will need to be in a Liferay Workspace and have the Blade cli tool installed for this to work.

Tip: If you want to be able to hotreload while developing, make sure you make your source urls in
`client-extension.yaml` start with the absolute url `http://localhost:3000/${your-widget-name}.js`;
Then run the start script from `package.json` to serve your app locally.
