# CE CLI -- Client Extension CLI

The ce-cli, or Client Extension Command Line Interface, is a tool to help
set up local development projects for Liferay Client extensions.

## Features

- Automatically initialize a Client Extension workspace
- eslint and prettier configs OOTB for consistent code style
- TypeScript configured OOTB
- Production build strips out packages that Liferay makes available
- Dev build leaves packages in so the dev server doesn't break
- Integration with Liferay Workspace and Blade CLI -- use `blade gw deploy` to deploy your apps
- Scaffold new Client Extensions automatically

## Installation

Currently, you'll need to download a binary and add it to your path manually.
Eventually I'll add it to npm.

Get the binary [here](https://github.com/bnheise/ce-cli/releases/tag/v0.1.0).
Add it to the PATH in your machine or just copy it into your workspace folder if you want it quick and dirty.

As the project is built in Rust, you can also install it using `cargo`:

```bash
cargo install ce-cli
```

## Usage

This CLI expects its workspace to be hosted in a Liferay Workspace.
To setup a Liferay Workspace, first download the [Blade CLI](https://help.liferay.com/hc/en-us/articles/360017885232-Installing-Blade-CLI-).

After that, run `blade init` to set up your Liferay workspace.

Next run `blade server init` to download a Liferay bundle.

Next, anywhere in the workspace folder, create a folder to host your Client Extension Workspace and cd into it.
For example:

```bash
mkdir my-ce-workspace && cd my-ce-workspace
```

Next, initialize the workspace:

```bash
ce-cli init
```

Next, add your first custom element remote app:

```bash
ce-cli add custom-element "My New App"
```

Now your app is ready to develop.

To deploy to your local Liferay server and see what you've got, just use Blade: `blade gw deploy`.
