#!/bin/sh

rm -rf public
mkdir -p public
cargo build
./target/debug/kurzlink --generate
cd public
python -m http.server
cd ..
