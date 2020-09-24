---
title: Installation
---

## Install via npm
Install the package globally with npm or yarn.

```
npm install --global swp
```

This will download the JavaScript installer from npm, which will download the relevant pre-compiled binary from the corresponding release on GitHub.

You can also install the npm package locally, if you want to keep Sweep in your project's `devDependencies` and invoke it from a script.

[`swp` on npm](https://www.npmjs.com/package/swp)

## Download manually
Download the binary for your platform from the [latest release<GithubLatestVersion />](https://github.com/woubuc/sweep/releases/latest) on GitHub.

## Install via cargo
The Rust Way&#8482;. Install using Cargo.

```
cargo install swp
```

Building Sweep does not require unstable Rust features and should work on all recent versions of Rust.

## Other package managers
If you have a favourite package manager and you think Sweep should be published on it as well, please [create an issue](https://github.com/woubuc/sweep/issues) and we can talk about how to get it published.
