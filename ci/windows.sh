#!/usr/bin/env bash

mkdir -p dist

echo "Windows 64-bit"

cargo clean
rustup install stable-x86_64-pc-windows-msvc
cargo +stable-x86_64-pc-windows-msvc build --release

cd target/release
7z a ../../dist/project-cleanup-windows-64bit.zip project-cleanup.exe
cd ../../

echo "Windows 32-bit"

cargo clean
rustup install stable-i686-pc-windows-msvc
cargo +stable-i686-pc-windows-msvc build --release

cd target/release
7z a ../../dist/project-cleanup-windows-32bit.zip project-cleanup.exe
cd ../../
