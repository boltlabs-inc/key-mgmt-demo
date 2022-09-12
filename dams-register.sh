#!/bin/bash

mode=release
storage=local.db

userid=$1
if [[ $userid == "" ]]; then
    echo "$0: Did not specify a user id."
    exit -1
fi

password=$2
if [[ $password == "" ]]; then
    echo "$0: Did not specify a password."
    exit -1
fi

config="./dev/Client.toml"
echo "registering a user '$userid'"
./target/$mode/key-mgmt-demo --config "$config" --storage "$storage" --user-id "$user_id" --password "$password" register
