# A demo application that shows the use of the DAMS library
First build the binary:
```shell
cargo build
```

Run a key server using the instructions in the README.md of [the DAMS repo.](https://github.com/boltlabs-inc/key-mgmt)
Copy `localhost.crt` from `key-mgmt/dev` to the local `dev` directory.


# Running operations with scripts
Scripts are provided for each operation to allow for easier testing.

## Register a User
```shell
sh register.sh user_id SuperSecurePassword
```

## Authenticate and Generate a secret
```shell
sh generate.sh user_id SuperSecurePassword
```

## List keys stored locally
```shell
sh list_keys.sh user_id SuperSecurePassword
```

# Running operations manually

## Register a User
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml" --storage "local.db" --account-name "account1" --password "SuperSecurePassword" register
```

## Authenticate and Generate a secret
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml"  --storage "local.db" --account-name "account1" --password "SuperSecurePassword" generate
```

## List keys stored locally
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml"  --storage "local.db" --account-name "account1" --password "SuperSecurePassword" list
```
