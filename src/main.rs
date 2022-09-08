use structopt::StructOpt;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;
use crate::client::Client;
use crate::Client::Register;
use dams_client::client::DamsClient;
use dams_client::client::Password;
use dams::user::UserId;
use dams::config::client::Config;
use anyhow::anyhow;
use rand::{prelude::StdRng, SeedableRng};
use std::str::FromStr;

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


    let result = match cli.client {
        Register(_) => {
            DamsClient::register(&UserId::from_str(&cli.user_id).unwrap(), &Password::from_str(&cli.password).unwrap(), &client_config).await
                .map_err(|e| anyhow!(e))
                .map(|sess| {
                    info!("Registered and opened a session: {:?}", sess);
                    sess
                })
        },
        Client::Open(_) => {
            DamsClient::authenticated_client(&UserId::from_str(&cli.user_id).unwrap(), &Password::from_str(&cli.password).unwrap(), &client_config)
            .await
                .map_err(|e| anyhow!(e))
                .map(|sess| {
                    info!("Opened a session: {:?}", sess);
                    sess
                })
        },
        Client::Generate(generate) => {
            let res = DamsClient::authenticated_client(&UserId::from_str(&cli.user_id).unwrap(), &Password::from_str(&cli.password).unwrap(), &client_config)
            .await
                .map_err(|e| anyhow!(e))
                .map(|sess| {
                    info!("Opened a session: {:?}", sess);
                    sess
                });
            let key_id = [0u8; 32];
            info!("Proceed to generate a secret...");
            info!("Key ID: {:?}", key_id);
            // DamsClient::generate().await.map_err(|e| anyhow!(e))
            //     .map(|sess| {
            //         info!("Opened a session: {:?}", sess);
            //         sess
            //     })        
            res
        },
    };
    if let Err(e) = result {
        error!("{}, caused by: {}", e, e.root_cause());
    }
}
