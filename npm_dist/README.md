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
- Create shared components and easily choose whether to bundle them within the client extension or as a separated
  JavaScript file
- Bundle dependencies shared by multiple components into a single shared dependency to reduce bundle size
  by removing duplicated code
- Supports projects build with React and with Vue
- BETA: import the Objects and Picklists that your extension depends on as .json to keep your repo as the source of truth for your extension
- BETA: import Object data into your repo to keep dummy data ready for use
- BETA: export the Objects and Picklists that your extension depends on from your repo to a Liferay instance. Great for onboarding new developers or even deploying definitions to a production instance. Supports adding new entries and updating existing entries.
- BETA: export Object entries from your repo to a Liferay instance. Useful for deploying dummy data to a local instance (good for onboarding new developers)

## Installation

Run `npm install -g ce-cli` to install. If your operating system or architecture isn't supported, make an issue
and I'll try to add it. I can't guarantee that I'll be able to though, so if that happens, I recommend installing
Rust and compiling the binary yourself with `cargo install ce-cli`.

Note on installing with `yarn`. Currently there is a bug which causes installation to fail with yarn. Install using npm as a workaround.

## Update
Note on installing with `pnpm`. Currently pnpm has a bug where when you try to install a new version of ce-cli, it will
hold binary even if you uninstall and install again. Just running `pnpm store prune` is not enough.
This issue has been filed with pnpm, but for below is the only way to get the new version by pnpm. 

1. Delete your pnpm cache (use `pnpm store path` to get the path, and delete the folder)
2. Run `npm install -g ce-cli` to update it. (Maybe you need the force option `npm install -gf ce-cli`)
3. Check if the version is updated by `ce-cli --version`

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

### Adding a Shared Dependency

A shared dependency is a dependency shared by multiple custom element client extensions, but bundled separately.
This reduces the overall bundle size of each component as it removes the duplicated code from each respective
bundle and instead hosts it separately on Liferay. Like shared components, this can reduce performance on a page
the first time a user visits it since an additional JS file must be loaded, but once the browser caches that file
subsequent loads can see improved performance since the file won't need to be loaded for other components that
aren't in the browser cache yet.

To make a shared dependency, run the following command:

```bash
ce-cli add shared-dependency "{npm-package-name}"
```

Note that you can optionally provide a version for the package as you would when running npm install:

```bash
ce-cli add shared-dependency "lodash-es@~4.17.21"
```

### Importing Data

The command `ce-cli import` allows you to import Object Definitions, Picklists, and Object entries. This is useful
for keeping your repo as the single source of truth for its dependencies. Import you can import the Object Definitions
and Picklists that your client extension depends on and re-export them to other Liferay instances as needed. For
example to import all three, run

```bash
ce-cli import -a
```

You can also specify which you want to import specifically using `-o` (objects), `-p` (picklists), `-d` object
entry data. For example, to import only objects and picklists, run

```bash
ce-cli import -o -p
```

### Exporting Data

The command `ce-cli export` allows you to export Object Definitions, Picklists, and Object that you've imported to your
local repo to another Liferay instance. This is useful when you need to onboard a new developer to the project since they
can easily run a few commands to get the data they need to start developing into their Liferay instance. However, you
can also use this feature to deploy to a remote instance.

Exporting is similar to importing, except that you cannot export Object Definitions, Object data, or Picklists at the same time.
This is because many of the records to be processed are sent in bulk and processed asynchronously, but Objects, Picklists, and
entry data depend on each other. This means it's possible for Liferay to try to create an entity before it's dependencies are
created, leading to an error. This limitation may be lifted in the future. Also note that currently due to limitations in the
picklist batch api, exporting large amounts of picklist data could take some time as parts of the process must be carried out
synchronously due to these limitations.

To export data, run the command `ce-cli export` along with a combination of the flags `-o` (Object defintiions),
`-p` (Picklists), and `-d` (Object entry data). Note that `-o` and `-d` may not be specified at the same time.

For example, to import all objects and picklists, run

```bash
ce-cli export -o -p
```

To export object entry data, run

```bash
ce-cli export -d
```

#### Caveats

##### ES Modules Required

Packages must be bundles using ECMAScript modules to function properly as a shared dependency

##### Packages with Peer Deps Not Supported

Packages with peer dependencies can't be used this way because the peer deps will end up bundled separately,
which will cause the build to break.

There is a workaround for this but in involves making a new entrypoint into the app where you import and rexport
both dependencies. There are some additional steps that need to be made in the webpack config for this to
work. Documentation is pending.

TODO: document shared dependencies with peer dependencies

## Roadmap

- Add additional templates for other kinds of remote apps
  - CSS extension
  - JS extension
  - IFrame extension
  - Theme CSS
  - Theme Favicon
- Add Angular configuration

## Known Issues

Currently, none
