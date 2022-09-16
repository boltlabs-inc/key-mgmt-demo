# Demo of DAMS client and key server

### Table of Contents
1. [Install](#install)
2. [Testing `DamsClient` via scripts](#testing-damsclient-via-scripts)
3. [Running operations via CLI](#running-operations-via-cli)
4. [`DamsClient` API walkthrough](#damsclient-api-walkthrough)

## Install

You can build the key server automatically in this demo directory using the following script:
```shell
sh ./build-key-server.sh
```

We recommend reviewing the instructions in the README.md of [the DAMS repo](https://github.com/boltlabs-inc/key-mgmt) to ensure the MongoDB is installed correctly for your OS.

If successful, then you can build the demo here as follows:
```shell
cargo build
```

At this point, we recommend starting a new terminal to run the key server as follows:
```shell
sh ./run-key-server.sh
```

Now you're ready to generate and retrieve secrets locally and remotely using the CLI of the sample demo app.

## Testing `DamsClient` via scripts

We have provided some bash scripts to simplify `DamsClient` API testing. It allows the following basic operations:
* **Register a user.** Should only be executed once per user.
* **Authenticate and generate a secret.** Can be called repeatedly to generate as many keys per account as possible. The app shows how to backup the secret in a local storage DB (but backup functionality is outside the scope of the `DamsClient`).
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

### Delete key stored locally
```shell
./target/debug/key-mgmt-demo --config "./dev/Client.toml"  --storage "local.db" --account-name "account1" --password "SuperSecurePassword" delete --key-id <key-id-hex>
```

## `DamsClient` API walkthrough

To build your own application, we provide a walkthrough of the `DamsClient` API which consists of four API calls. 
* ``DamsClient::register()`` - takes the account name, password and client configuration and registers the user with the server.
* ``DamsClient::authenticated_client()`` - takes the account name, password and client configuration and opens a secure session with the server if the specified credentials are correct.
* ``DamsClient::generate_and_store()`` - generates a secret and stores it on the key server. Outputs a tuple that consists of a key ID and wrapped secret for local storage.
* ``DamsClient::retrieve()`` - retrieve a secret from the server given a key ID and a retrieve context (local storage or export only). Outputs a wrapped secret with the corresponding context.

### Register a User

Given a account name and password, the first step is to register a user with the key server as follows:
```
// Load the account name and password
let account_name = AccountName::from_str("account1")?;
let password = Password::from_str("SuperSecurePassword")?;

// Load the client config toml file above (i.e, Client.toml)
// This tells the DamsClient how to reach the key server and 
// loads the TLS certificate
let client_config = Config::load(cli.config)
    .await
    .expect("Failed to load client config");

// Now you can register the user
let result = DamsClient::register(&account_name, &password, &client_config).await
                        .map_err(|e| anyhow!(e))
                        .map(|sess| {
                            info!("Successfully registered and here's the session info: {:?}", sess);
                            sess
                        });

// If an error occurs, extract the DamsClientError type/message
if let Err(e) = result {
    error!("{}, caused by: {}", e, e.root_cause());
}
```

### Generate and store a secret

Once the user has successfully been registered, you can authenticate and generate secrets with that user's credentials as follows:
```
// Load the acount name, password and client config as before
...
// Authenticate to the key server
let dams_client = DamsClient::authenticated_client(&account_name, &password, &client_config).await?;

// If successful, proceed to generate a secret with the established session
let result = dams_client.generate_and_store().await
                        .map_err(|e| anyhow!(e))
                        .map(|sec_key| {
                            // Proceed to retrieve the key id and wrapped secret
                            let (key_id, secret) = sec_key;
                            ....
                            // the key id serves as a reference to the generated secret
                            // in your local storage. Proceed to store 
                            // or use the secret in your application.
                        });
...
```

### Retrieve a secret

Once a key has been generated and stored remotely, you can authenticate to the key server and retrieve the secret if you persist the key ID in your local storage as follows:
```
// Load the acount name, password and client config as before
...
// Authenticate to the key server as before
...
// Convert key ID from string into KeyID struct
let key_id_str = ...
let key_id: KeyId = serde_json::from_str(&key_id_str)?;

// If successful, proceed to retrieve the secret key from the server with the key ID
// and can specify a context for your intent with the secret (i.e., for local storage).
let result = dams_client.retrieve(&key_id, RetrieveContext::LocalOnly)
                        .await
                        .map_err(|e| anyhow!(e))
                        .map(|arbitrary_key| {
                            info!("Retrieved a wrapped key from server: {:?}", arbitrary_key);
                            // Proceed to use or store the arbitrary_key in your local storage DB.
                        });
...
```

## What's Next

In the next development phases to come, we will demonstrate the following functionality in the `DamsClient`:

* A remote client and signing key support in addition to arbitrary keys
* Expanded operations over the generated