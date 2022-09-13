use crate::client::Client;
use crate::Client::Register;
use anyhow::anyhow;
use dams::config::client::Config;
use dams::user::AccountName;
use dams_client::client::DamsClient;
use dams_client::client::Password;
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
    let client_config = Config::load(cli.config)
        .await
        .expect("Failed to load client config");

    let storage = String::from_str(&cli.storage)?;
    // Configure the local storage
    let cfg = KvConfig::new(storage.as_str());
    // Open the key-value store
    let store = Store::new(cfg)?;

    // Fetch user credentials
    let account_name = AccountName::from_str(&cli.account_name)?;
    info!("Account Name: {:?}", account_name);
    let password = Password::from_str(&cli.password)?;

    let result = match cli.client {
        Register(_) => DamsClient::register(&account_name, &password, &client_config)
            .await
            .map_err(|e| anyhow!(e))
            .map(|sess| {
                info!("Registered and opened a session: {:?}", sess);
                sess
            }),
        Client::Generate(_) => {
            let dams_client =
                DamsClient::authenticated_client(&account_name, &password, &client_config).await?;
            dams_client
                .generate_and_store()
                .await
                .map_err(|e| anyhow!(e))
                .map(|key_object| {
                    let bucket = store.bucket::<String, String>(Some(&cli.account_name))?;
                    info!("Generated a key object: {:?}", key_object);
                    let (key_id, local_storage) = key_object;
                    // TODO: Need compact hex representation
                    let key_id_vec: Vec<u8> = key_id.into_iter().collect();
                    let key_id_hex = hex::encode(&key_id_vec);
                    info!("Key ID: {:?}", key_id_hex);
                    let value = serde_json::to_string(&local_storage)?;
                    bucket.set(&key_id_hex, &value)?;

                    Ok(())
                })?
        }
        Client::List(_list) => {
            // extract contents of local storage
            let dams_client =
                DamsClient::authenticated_client(&account_name, &password, &client_config).await;
            if dams_client.is_ok() {
                let mut index = 1;
                let bucket = store.bucket::<String, String>(Some(&cli.account_name))?;
                for item in bucket.iter() {
                    let item = item?;
                    let key: String = item.key()?;
                    let value: String = item.value()?;
                    info!("{}: Key ID: {} => {}", index, key, value);
                    index += 1;
                }
            }
            return Ok(());
        }
    };
    if let Err(e) = result {
        error!("{}, caused by: {}", e, e.root_cause());
    }

    Ok(())
}
