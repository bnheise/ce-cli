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

Run `npm install -g ce-cli` to install. If your operating system or architecture isn't supported, make an issue
and I'll try to add it. I can't guarantee that I'll be able to though, so if that happens, I recommend installing
Rust and compiling the binary yourself with `cargo install -ce-cli`.

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

This will generate a scaffold for a custom element remote app and add a default configuration to the client-extension.yaml file.
To get started developing, just run `npm run start` to start the dev server. This will automatically deploy the configuration
for your remote apps to your local instance with the js and css urls directed at your dev server. No need to update the values
using in client-extension.yaml!

Note that if you add any additional apps using the cli after the dev server is running, you'll have to restart it so it can
pick up the new files.

## Roadmap

- Add OOTB unit testing configuration with Jasmine
- Add OOTB component testing configuration with Cypress
- Add additional templates for other kinds of remote apps
  - CSS extension
  - JS extension
  - IFrame extension
  - Theme CSS
  - Theme Favicon
- Add Object definition deployment configuration
- Add Vue configuration
- Add Angular configuration
- Add shared component configuration (component used by other components but not registered as a remote app)
- Add shared dependency configuration (make a dependency in package.json deployed as it's own bundle rather than bundling everywhere its used)
- Deploy to virtual instances

## Known Issues

Currently, none!
