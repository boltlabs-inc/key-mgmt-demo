use dams_client::api::arbitrary_secrets::LocalStorage;
use structopt::StructOpt;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;
use crate::client::Client;
use crate::Client::Register;
use dams_client::client::DamsClient;
use dams_client::client::Password;
use dams::user::AccountName;
use dams::config::client::Config;
use anyhow::anyhow;
use rand::{prelude::StdRng, SeedableRng};
use std::str::FromStr;
use kv::Config as KvConfig;
use kv::*;

pub(crate) mod client;

#[derive(Debug, StructOpt)]
pub enum Cli {
    Client(client::Cli),
}

#[tokio::main]
pub async fn main() {
    let filter = EnvFilter::try_new("info").unwrap();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let cli: client::Cli = client::Cli::from_args();
    let client_config = Config::load(cli.config.unwrap())
        .await
        .expect("Failed to load client config");

    let storage = String::from_str(&cli.storage).unwrap();
    // Configure the local storage
    let cfg = KvConfig::new(storage.as_str());
    // Open the key-value store
    let store = Store::new(cfg).unwrap();

    // Fetch user credentials
    let account_name = AccountName::from_str(&cli.account_name).unwrap();
    info!("Account Name: {:?}", account_name);
    let password = Password::from_str(&cli.password).unwrap();

    let result = match cli.client {
        Register(_) => {
            DamsClient::register(&account_name, &password, &client_config).await
                .map_err(|e| anyhow!(e))
                .map(|sess| {
                    info!("Registered and opened a session: {:?}", sess);
                    sess
                })
        },
        Client::Generate(_) => {
            let dams_client = DamsClient::authenticated_client(&account_name, &password, &client_config).await.unwrap();
            dams_client.generate_and_store().await
                .map_err(|e| anyhow!(e))
                .map(|key_object| {
                    let bucket = store.bucket::<String, String>(Some(&cli.account_name)).unwrap();
                    info!("Generated a key object: {:?}", key_object);
                    let (key_id, local_storage) = key_object;
                    // TODO: Need compact hex representation
                    let mut key_id_vec = Vec::new();
                    let mut it = key_id.into_iter();
                    loop {
                        match it.next() {
                            Some(x) => key_id_vec.push(x),
                            None => break,
                        }
                    }
                    let key_id_hex = hex::encode(&key_id_vec);
                    info!("Key ID: {:?}", key_id_hex);
                    let value = serde_json::to_string(&local_storage).unwrap();
                    bucket.set(&key_id_hex, &value).unwrap();

                })
        },
        Client::List(_list) => {
            // extract contents of local storage
            let dams_client = DamsClient::authenticated_client(&account_name, &password, &client_config).await;
            if dams_client.is_ok() {
                let mut index = 1;
                let bucket = store.bucket::<String, String>(Some(&cli.account_name)).unwrap();
                for item in bucket.iter() {
                    let item = item.unwrap();
                    let key: String = item.key().unwrap();
                    let value = item.value::<String>().unwrap();
                    info!("{}: Key ID: {} => {}", index, key, value);
                    index += 1;
                }
            }
            return
        },
    };
    if let Err(e) = result {
        error!("{}, caused by: {}", e, e.root_cause());
    }
}
