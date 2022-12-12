#!/bin/sh

KL_PWD="$(pwd)"
KL_BASEPATH="${KL_PWD}/servetest"
KL_VANITYPATH="${KL_BASEPATH}/vanitymap.json"
KL_CONFIGFILE="${KL_PWD}/vanitymap.yaml"

cargo run -- -g -o "${KL_BASEPATH}" -m "${KL_VANITYPATH}" -c "${KL_CONFIGFILE}" -n || exit
cd "${KL_BASEPATH}"
python3 -m http.server
cd "${KL_PWD}"
rm -rf "${KL_BASEPATH}"
