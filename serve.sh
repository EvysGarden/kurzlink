#!/bin/sh

KL_PWD="$(pwd)"
KL_BASEPATH="${KL_PWD}/servetest"
KL_VANITYPATH="${KL_BASEPATH}/vanitymap.json"

cargo build
./target/debug/kurzlink -g -o "${KL_BASEPATH}" -m "${KL_VANITYPATH}"
cd "${KL_BASEPATH}"
python3 -m http.server
cd "${KL_PWD}"
rm -rf "${KL_BASEPATH}"
