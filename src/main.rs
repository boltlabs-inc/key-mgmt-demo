//! This is a demonstration of a simple API for generating and retrieving arbitrary secrets from a key server.
//! We show the integrated use of TLS and OPAQUE protocol to securely transfer key material between the DamsClient
//! and key server, where it is securely stored.
//!

use crate::client::Client;
use crate::Client::Register;
use anyhow::anyhow;
use dams::config::client::Config;
use dams::user::AccountName;
use dams_client::client::DamsClient;
use dams_client::client::Password;
use dams::crypto::KeyId;
use dams::RetrieveContext;
use kv::{Config as KvConfig, Store};
use std::str::FromStr;
use structopt::StructOpt;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

pub(crate) mod client;

#[derive(Debug, StructOpt)]
pub enum Cli {
    Client(client::Cli),
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_new("info")?;
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let cli: client::Cli = client::Cli::from_args();
    // Load the client configuration from disk (Client.toml)
    // Needs to be provided with each DamsClient API call
    let client_config = Config::load(cli.config)
        .await
        .expect("Failed to load client config");

    // Get path to local storage
    let storage_path = String::from_str(&cli.storage)?;
    // Configure the local storage
    let cfg = KvConfig::new(storage_path.as_str());
    // Open the key-value store
    let store = Store::new(cfg)?;

    // Fetch user credentials
    let account_name = AccountName::from_str(&cli.account_name)?;
    let password = Password::from_str(&cli.password)?;

    let result = match cli.client {
        // Register once with an account and password to the key server
        Register(_) => DamsClient::register(&account_name, &password, &client_config)
            .await
            .map_err(|e| anyhow!(e))
            .map(|sess| {
                info!("Registered and opened a session: {:?}", sess);
                sess
            }),
        Client::Generate(_) => {
            // Authenticate user to the key server
            let dams_client =
                DamsClient::authenticated_client(&account_name, &password, &client_config).await?;
            // If successful, proceed to generate a secret with the established session
            dams_client
                .generate_and_store()
                .await
                .map_err(|e| anyhow!(e))
                .map(|sec_key| {
                    let bucket = store.bucket::<String, String>(Some(&cli.account_name))?;
                    // Proceed to retrieve the key id and wrapped secret
                    let (key_id, secret) = sec_key;
                    // Prepare the key_id and secret for local storage
                    let key_id_vec: Vec<u8> = key_id.into_iter().collect();
                    let key_id_hex = hex::encode(&key_id_vec);
                    info!("Generated a key with ID: {:?}", key_id_hex);
                    let value = serde_json::to_string(&secret)?;
                    bucket.set(&key_id_hex, &value)?;

                    Ok(())
                })?
        },
        Client::Retrieve(retrieve) => {
            // Authenticate user to the key server
            let dams_client =
                DamsClient::authenticated_client(&account_name, &password, &client_config).await?;
            let key_id_vec = hex::decode(&retrieve.key_id).unwrap();
            info!("Key ID: {:?}", key_id_vec);
            let key_id_str = format!("{:?}", key_id_vec);
            // Convert key ID from string into KeyID struct
            let key_id: KeyId = serde_json::from_str(&key_id_str).unwrap();
            // If successful, proceed to retrieve the secret key from the server with the key ID
            // and can specify a context for your intent with the secret (i.e., for local storage).            
            dams_client
                .retrieve(&key_id, RetrieveContext::LocalOnly)
                .await
                .map_err(|e| anyhow!(e))
                .map(|arbitrary_key| {
                    info!("Retrieved a wrapped key from server: {:?}", arbitrary_key);
                    let bucket = store.bucket::<String, String>(Some(&cli.account_name))?;
                    let value = serde_json::to_string(&arbitrary_key)?;
                    // Override what's in local storage
                    bucket.set(&retrieve.key_id, &value)?;

                    Ok(())
                })?
        },
        Client::List(_list) => {
            let dams_client =
                DamsClient::authenticated_client(&account_name, &password, &client_config).await;
            if dams_client.is_ok() {
                let mut index = 1;
                let bucket = store.bucket::<String, String>(Some(&cli.account_name))?;
                for item in bucket.iter() {
                    let item = item?;
                    let key: String = item.key()?;
                    let value: String = item.value()?;
                    info!("{}: Key ID: {} =>\n{}", index, key, value);
                    index += 1;
                }
            }
            return Ok(());
        },
        Client::Delete(delete) => {
            let dams_client =
                DamsClient::authenticated_client(&account_name, &password, &client_config).await;
            if dams_client.is_ok() { 
                let bucket = store.bucket::<String, String>(Some(&cli.account_name))?;
                let key_id = delete.key_id;
                let value = String::from("None");
                bucket.set(&key_id, &value)?;
                info!("Deleted key with ID: {:?}", key_id);
            }
            return Ok(());

        },
    };
    if let Err(e) = result {
        error!("{}, caused by: {}", e, e.root_cause());
    }

    Ok(())
}
