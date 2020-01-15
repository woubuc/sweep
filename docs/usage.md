# Usage
Invoke Sweep to crawl all subdirectories of your current directory to discover your projects.

```
swp
```

You can also specify one or more directories to tell Sweep where to start.

```
swp ./work ./personal
```

Depending on how many subdirectories you have, crawling may take a while.

After all directories have been crawled, you will see a list of all discovered directories that will be deleted. Confirm with `y` to delete the listed directories.

![Screenshot of the CLI output](../readme_screenshot.png)


## Flags

### -a, --all
Skip checking the modified date of discovered projects. For when you want to sweep all dependencies, even in projects you recently edited.

### -i, --ignore `<ignore>`
Set a regex pattern for directories to ignore.

If you want to ignore a specific directory every time, you may want to use a [.swpfile](/configuration.md) instead.

::: warning
The ignore pattern will match on the entire path, not just the directory name.

`--ignore test` will ignore `./test/`, `/new-test`, `foo/tests`, etc.
:::

### -f, --force <Badge type="error" text="dangerous" />
Skip the confirmation and immediately sweep the discovered directories.

::: danger
You will not have any opportunity to review which directories will be deleted. It is recommended to only use this on single project directories that use a [.swpfile](./configuration). Use at your own risk.
:::

### -h, --help
View the help info. The same as this, but in your terminal!
