use structopt::StructOpt;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;
use crate::client::Client;
use crate::Client::Register;
use dams_local_client::api::Session;
use dams_local_client::api::SessionConfig;
use dams_local_client::api::Password;
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

    let mut rng = StdRng::from_entropy();

    let cli: client::Cli = client::Cli::from_args();
    let client_config = Config::load(cli.config.unwrap())
        .await
        .expect("Failed to load client config");

    let mut client =
        dams_local_client::api::connect(cli.server)
            .await
            .expect("Could not return a client");


    let result = match cli.client {
        Register(register) => {
            let client_config = SessionConfig::new(client_config);

            Session::register(&mut client, &mut rng, &UserId::from_str(&register.user_id).unwrap(), &Password::from_str(&register.password).unwrap(), &client_config)
                .await
                .map_err(|e| anyhow!(e))
                .map(|sess| {
                    info!("Registered and opened a session: {:?}", sess);
                    sess
                })
        },
        Client::Open(open) => {
            let client_config = SessionConfig::new(client_config);

            Session::open(&mut client, &mut rng, &UserId::from_str(&open.user_id).unwrap(), &Password::from_str(&open.password).unwrap(), &client_config)
                .await
                .map_err(|e| anyhow!(e))
                .map(|sess| {
                    info!("Opened a session: {:?}", sess);
                    sess
                })
        }
    };
    if let Err(e) = result {
        error!("{}, caused by: {}", e, e.root_cause());
    }
}
