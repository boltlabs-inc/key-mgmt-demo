use std::path::PathBuf;
use structopt::StructOpt;

/// The keyMgmt client command-line interface.
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Cli {
    /// Path to a configuration file.
    #[structopt(long)]
    pub config: Option<PathBuf>,

    /// Run client commands.
    #[structopt(subcommand)]
    pub client: Client,
}

#[derive(Debug, StructOpt)]
pub enum Client {
    Register(Register),
    Open(Open),
}

/// Register with the server.
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Register {
    /// The address for the server.
    #[structopt(long)]
    pub server: String,

    #[structopt(long)]
    pub user_id: String,

    #[structopt(long)]
    pub password: String,
}

/// Authenticate with the server.
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Open {
    /// The address for the server.
    #[structopt(long)]
    pub server: String,

    #[structopt(long)]
    pub user_id: String,

    #[structopt(long)]
    pub password: String,
}
