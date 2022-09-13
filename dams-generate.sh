#!/bin/bash

mode=debug
storage=local.db

account=$1
if [[ $account == "" ]]; then
    echo "$0: Did not specify an account name."
    exit -1
fi

password=$2
if [[ $password == "" ]]; then
    echo "$0: Did not specify a password."
    exit -1
fi

config="./dev/Client.toml"
echo "Authenticate user '$account'"
./target/$mode/key-mgmt-demo --config "$config" --storage "$storage" --account-name "$account" --password "$password" generate
