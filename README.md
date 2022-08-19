# A demo application that shows the use of the DAMS library
First build the binary:
```shell
cargo build
```

Run a key server using the instructions in the README.md of [the DAMS repo.](https://github.com/boltlabs-inc/key-mgmt)

## Register a User
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml" --server "https://127.0.0.1:1113" register --user-id "user_id" --password "SuperSecurePassword"
```

## Open a New Session
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml" --server "https://127.0.0.1:1113" open --user-id "user_id" --password "SuperSecurePassword"
```