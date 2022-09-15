use std::path::PathBuf;
use structopt::StructOpt;

/// The keyMgmt client command-line interface.
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Cli {
    /// Path to a configuration file.
    #[structopt(long)]
    pub config: PathBuf,

    /// Local storage
    #[structopt(long)]
    pub storage: String,

    /// Run client commands.
    #[structopt(subcommand)]
    pub client: Client,

    /// User ID
    #[structopt(long)]
    pub account_name: String,

    /// Passphrase
    #[structopt(long)]
    pub password: String,
}

#[derive(Debug, StructOpt)]
pub enum Client {
    Register(Register),
    Generate(Generate),
    Retrieve(Retrieve),
    List(List),
    Delete(Delete),
}

/// Register with the server.
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Register {}

/// Authenticate and Generate a secret
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Generate {}

/// Authenticate and Retrieve a secret given key_id
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Retrieve {
    // key ID
    #[structopt(short, long)]
    pub key_id: String,
}

/// Delete a key
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct Delete {
    // key ID
    #[structopt(short, long)]
    pub key_id: String,
}

/// List the keys stored locally
#[derive(Debug, StructOpt)]
#[non_exhaustive]
pub struct List {
    // add optional key ID
    #[structopt(short, long)]
    pub key_id: Option<String>,
}
