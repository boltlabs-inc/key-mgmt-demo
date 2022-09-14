#!/bin/bash

echo "[+] Clone and build key-mgmt repo..."
git clone https://github.com/boltlabs-inc/key-mgmt.git
cd key-mgmt
cargo build

echo "[+] Generate a self-signed TLS certificate for key server..."
cd dev/
./generate-certificates
cd -

echo "[+] Retrieve the certificate..." 
cp dev/localhost.crt ../dev/
