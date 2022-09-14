# Demo of DAMS client and key server

You can build the key server automatically in this demo directory using the following script:
```shell
sh ./build-key-server.sh
```

We recommend reviewing the instructions in the README.md of [the DAMS repo.](https://github.com/boltlabs-inc/key-mgmt) to ensure the MongoDB is installed correctly for your OS.

If successful, then you can build the demo here as follows:
```shell
cargo build
```

At this point, we recommend starting a new terminal to run the key server as follows:
```shell
sh ./run-key-server.sh
```

Now you're ready to generate and retrieve secrets locally and remotely using the CLI of the sample demo app.

## Quick Testing DamsClient via scripts

We have provided some bash scripts to simpplify `DamsClient` API testing. It allows the following basic operations:
* **Register a user.** Should only be executed once per user.
* **Authenticate and generate a secret.** Can be called repeatedly to generate as many keys per account as possible. The app stores a backup of the secret in a local storage DB.
* **Retrieve a secret based on a key ID.** Allows the user to securely retrieve any secrets from the key server.
* **List locally stored secrets.** Shows list of stored key IDs and secrets for a given account.

### Register a User
```shell
sh register.sh account1 SuperSecurePassword
```

### Authenticate and Generate a secret
```shell
sh generate.sh account1 SuperSecurePassword
```

### Authenticate and Retrieve a secret
```shell
sh retrieve.sh account1 SuperSecurePassword
```

### List secrets stored locally
```shell
sh list_secrets.sh account1 SuperSecurePassword
```

## Running operations via CLI

### Register a User
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml" --storage "local.db" --account-name "account1" --password "SuperSecurePassword" register
```

### Authenticate and Generate a secret
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml"  --storage "local.db" --account-name "account1" --password "SuperSecurePassword" generate
```

### Authenticate and Retrieve a secret
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml"  --storage "local.db" --account-name "account1" --password "SuperSecurePassword" retrieve --key-id <key-id-hex>
```

### List keys stored locally
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml"  --storage "local.db" --account-name "account1" --password "SuperSecurePassword" list
```

## Walkthrough of the DAMS client API

To build your own application, we provide a walkthrough of the `DamsClient` API in this section.
```
TODO
```
