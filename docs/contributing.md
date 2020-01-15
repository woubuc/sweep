# Contributing
This project welcomes contributions of any kind, whether you want to add new features, improve the documentation or just want to give some feedback.


## Better defaults
The main thing still missing from Sweep is more and better [default detection rules](/#discovering-projects). Pull requests to add more rules (or refine the existing ones) are always welcome.

### How to add rules
1. Open the file [src/discover_projects/detect_cleanable_project.rs](https://github.com/woubuc/sweep/blob/master/src/discover_projects/detect_cleanable_project.rs)
2. Add your checks to the main `detect_cleanable_project` function
3. Add the necessary tests to the test at the bottom of the file

You'll need some basic knowledge of [Rust](https://www.rust-lang.org/), but you definitely don't need to be a master.


## How to contribute
This is a relatively standard process for most open source projects. I'll repeat it here for clarity's sake, and to help new contributors get started. In case of questions, create an issue or [reach out on Twitter](https://twitter.com/woubuc).

### 1. Create an issue
It all starts by [creating an issue](https://github.com/woubuc/sweep/issues) in the Sweep repository. Use this to describe and discuss the changes you intend to make.

You can also use issues for questions or general feedback, although this may be changed as we get more issues.

::: tip NOTE
You can skip creating an issue for small fixes, maintenance, etc. Use your best judgment.
:::

### 2. Submit a PR
Fork the repository, make the changes you want to make, and then create a pull request to merge them back into the main project.

Keep in mind there will probably be some feedback and you may need to make some changes before your PR can be merged into the project.
