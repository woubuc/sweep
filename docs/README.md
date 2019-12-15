![Project Cleanup logo](../readme_logo.png)

Project Cleanup finds old projects that haven't been changed in more
than a month. It will clean up and remove unnecessary directories
containing libraries, dependencies, builds, etc. These files can easily
be re-generated at any time by running install or build commands, and
if you haven't worked on the project in a while you probably don't need
them taking up space right now.

![Screenshot](../readme_screenshot.png)

## Getting started
Install Project Cleanup with npm

```
npm install --global project-cleanup
```

See [installation](/installation.md) for alternative ways to install.

## Basic usage
Simply run Project Cleanup and it will start searching the current directory and all subdirectories (recursively) for code projects it recognises.

```
project-cleanup
```

You can also pass a specific path to search

```
project-cleanup ./dev
```

See [usage](/usage.md) for more options

## Discovering projects
At this time, the list of built-in discovery rules is relatively short. It will recognise basic Node.js, Rust and Java projects, but nothing beyond that. If you have more sofisticated needs, you can use a [.cleanuprc file](./configuration.md).

### Built-in rules
- Node.js (will remove the `node_modules` and `.cache` directories)
- Rust (will remove the `target` directory)
- Java (will remove the `.gradle` and `build` directories)

These languages are based on my own experience and use patterns.

### Adding more rules
Contributions are welcome! Add the necessary checks in
[detect_cleanable_project.rs](https://github.com/woubuc/project-cleanup/blob/master/src/discover_projects/detect_cleanable_project.rs)
and submit a pull request. See [contributing](./contributing.md) for more details on how to contribute to the project.

## License
Project Cleanup is published under the MIT license. See
the [license file](https://github.com/woubuc/project-cleanup/blob/master/LICENSE) for details.
