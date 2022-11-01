#!/bin/sh

rm -rf public
cargo build
./target/debug/kurzlink --generate -o public
cd public
python -m http.server
cd ..
