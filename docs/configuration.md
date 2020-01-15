# Configuration
Sweep will find a lot of [standard directories](/#discovering-projects) in your projects, but there's a good chance you have different directories that also need to be cleaned.

Create a file in the root directory of your project called `.swpfile` and list all cleanable directories in it. Sweep will detect this file and use the listed directories instead of its defaults.

## Syntax
The syntax of a `.swpfile` is similar to that of a `.gitignore` file, so you can use existing syntax highlighting tools in your IDE of choice. However, the supported syntax is a lot more limited.

- `.swpfile` only supports directories, no individual files
- You cannot use wildcard patterns (`*`), each path must point to a single directory
- All paths should be relative starting from the `.swpfile` file, and should not start with `/`

::: warning
Adding a starting `/` in your path will cause Sweep to start from the root of your filesystem instead of relative from the working directory.
:::

### Example
See the [.swpfile](https://github.com/woubuc/sweep/blob/master/.swpfile) in the Sweep repository.

### Validation
To validate your `.swpfile` file, simply run `swp --all .` and verify that it lists the correct directories.

## Version control
You should commit the `.swpfile` along with your project. That way, everyone working on the project will sweep the same files when running `swp`. This is especially important when you override the default configuration to _prevent_ some default directories from being cleaned.

::: tip NOTE
The default directories may change between versions. New directories may be added at any point, or existing defaults may be removed if it turns out they aren't universal enough.

It is recommended to add a `.swpfile` to all projects with more than 'standard' directories.
:::
