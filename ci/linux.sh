#!/usr/bin/env bash

apt-get -y install zip

mkdir -p dist

echo "Linux 64-bit"

#cargo clean
#rustup install stable-x86_64-unknown-linux-gnu
#cargo +stable-x86_64-pc-windows-msvc build --release

cd target/release
zip -r ../../dist/project-cleanup-linux-64bit.zip project-cleanup
cd ../../

echo "Linux 32-bit"

#cargo clean
#rustup install stable-i686-unknown-linux-gnu
#cargo +stable-i686-pc-windows-msvc build --release

cd target/release
zip -r ../../dist/project-cleanup-linux-32bit.zip project-cleanup
cd ../../
