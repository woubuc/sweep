# Node Cleanup
This little tool (written in Rust for performance and mostly educational
reasons) will recursively browse a directory to find Node.js projects. If the
project hasn't been touched for more than a month, it will remove the
`node_modules` directory. In small to average projects, this can result in a
40-60% reduction in disk space use.

![Screenshot](README.png)

I'm still learning Rust so this project is nowhere near 'complete', or even
'decent'. The entire repository is a mess of test code blocks, attempts at
structuring it, and a lot of trial-and-error learning. Ye be warned.

## Install
- Make sure you have installed Rust
- Clone the repo and run the application with `cargo run -- [your path]`
- Check the (very limited) options with `cargo run -- -help`

## License
The application is published under the MIT license.