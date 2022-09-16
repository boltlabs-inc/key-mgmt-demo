#!/bin/bash

# TODO: check if key-mgmt/ dir exists. If not exit
echo "[+] Starting the key server..."
set -x
cd key-mgmt && cargo run --bin key-server-cli -- server --config dev/Server.toml run
set +x
