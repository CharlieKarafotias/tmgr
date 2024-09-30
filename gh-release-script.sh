#!/bin/sh

git push
version=$(./target/release/tmgr -V | cut -d ' ' -f 2)
gh release create v"$version" ./target/release/tmgr --latest --generate-notes
