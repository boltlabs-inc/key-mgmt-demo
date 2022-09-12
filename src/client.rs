use std::path::PathBuf;
use structopt::StructOpt;

/// The keyMgmt client command-line interface.
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Cli {
    /// Path to a configuration file.
    #[structopt(long)]
    pub config: Option<PathBuf>,

    // /// The address for the server.
    // #[structopt(long)]
    // pub server: String,

    /// Local storage
    #[structopt(long)]
    pub storage: String,

    /// Run client commands.
    #[structopt(subcommand)]
    pub client: Client,

    /// User ID
    #[structopt(long)]
    pub user_id: String,

    /// Passphrase
    #[structopt(long)]
    pub password: String,
}

#[derive(Debug, StructOpt)]
pub enum Client {
    Register(Register),
    Open(Open),
    Generate(Generate),
}

/// Register with the server.
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Register {}

/// Authenticate with the server.
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Open {}

/// Generate a secret
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Generate {
    // #[structopt(long)]
    // pub user_id: String,

    // #[structopt(long)]
    // pub password: String,
}
