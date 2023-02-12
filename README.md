# CE CLI -- Client Extension CLI

The ce-cli, or Client Extension Command Line Interface, is a tool to help
set up local development projects for Liferay Client extensions.

## Features

- Automatically initialize a Client Extension workspace
- eslint and prettier configs OOTB for consistent code style
- TypeScript configured OOTB
- Unit testing with Jest configured OOTB
- Component Testing and Integration testing with Cypress configured OOTB
- Production build strips out packages available on the Liferay, reducing your bundle sizes!
- Dev build leaves packages in so the dev server doesn't break
- Integration with Liferay Workspace and Blade CLI -- use `blade gw deploy` to deploy your apps
- Scaffold new Client Extensions automatically
- Create shared components and easily choose whether to bundle them within the client extension or as a separated JavaScript file

## Installation

Run `npm install -g ce-cli` to install. If your operating system or architecture isn't supported, make an issue
and I'll try to add it. I can't guarantee that I'll be able to though, so if that happens, I recommend installing
Rust and compiling the binary yourself with `cargo install ce-cli`.

Note on installing with `yarn`. Currently there is a bug which causes installation to fail with yarn. Install using npm as a workaround.

Note on installing with `pnpm`. Currently pnpm has a bug where when you try to install a new version of ce-cli, it will
hold binary even if you uninstall and install again. Youll have to delete your pnpm cache to clear it. Just running
`pnpm cache prune` is not enough. An issue has been filed with pnpm, but for now this is the only way to get the new version
by pnpm.

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

The above command will generate various configuration files, like Webpack, TypeScript, eslint, prettier, Jest, Cypress, etc.

Next, add your first custom element remote app:

```bash
ce-cli add custom-element "My New App"
```

This will generate a scaffold for a custom element remote app and add a default configuration to the client-extension.yaml file.

To get started developing, just run `npm run start` to start the dev server. This will automatically deploy the configuration
for your remote apps to your local instance with the js and css urls directed at your dev server. No need to update the values
using in client-extension.yaml.

Note that if you add any additional apps using the cli after the dev server is running, you'll have to restart it so it can
pick up the new files.

### Making a Shared Component

A Shared Component is a UI component that is used between two or more of your Custom Element Client Extensions.
Making a shared component allows you to easily specify whether a Shared Component should be bundled inside of
a Custom Element or separated out as it's own bundle. If you're Shared Component is of trivial size, it's best
to simply bundle it inside of your Custom Elements. However, if it's significantly large, it may be preferable
to bundle it separately. Bundling separately may increase load time on initial load since it causes an additional
file to be loaded, but once it's loaded and cached by the browser, when the user first loads another component
that uses the same Shared Component, the load time will be reduced since the browser already has the Shared
Component in the cache and only has to load the other component.

To make a shared comonent, run the following command:

```bash
ce-cli add shared-component "{Component Name}"
```

After that, develop the component as you would any other. However, be sure not to make any network calls or
access global variables -- instead, pass them in as props from the parent component. This ensures that your
component is easier to test and more independent of its environment.

We indicate whether a shared component should be bundled within it's parent Custom Element or bundled separately
by it's import syntax. If you use a relative path ('../../path/to/component') it will be bundled with the
Custom Element. If, however, you use simply the name of the folder that the element was generated into,
which should look something like 'my-element-name', Webpack will know to bundle it separately and convert your
import to load it externally.

## Roadmap

- Add additional templates for other kinds of remote apps
  - CSS extension
  - JS extension
  - IFrame extension
  - Theme CSS
  - Theme Favicon
- Add Object definition deployment configuration
- Add Vue configuration
- Add Angular configuration
- Add shared dependency configuration (make a dependency in package.json deployed as it's own bundle rather than bundling everywhere its used)
- Deploy to virtual instances

## Known Issues

Currently, none!
