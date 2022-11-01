#!/bin/sh

KL_PWD="$(pwd)"
KL_BASEPATH="${KL_PWD}/servetest"

cargo build
./target/debug/kurzlink -g -o "${KL_BASEPATH}"
cd "${KL_BASEPATH}"
python -m http.server
cd "${KL_PWD}"
rm -rf "${KL_BASEPATH}"
