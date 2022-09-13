# A demo application that shows the use of the DAMS library
First build the binary:
```shell
cargo build
```

Run a key server using the instructions in the README.md of [the DAMS repo.](https://github.com/boltlabs-inc/key-mgmt)

## Register a User
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml" --storage "local.db" --account-name "account1" --password "SuperSecurePassword" register
```

## Authenticate and Generate a secret
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml"  --storage "local.db" --account-name "account1" --password "SuperSecurePassword" generate
```
