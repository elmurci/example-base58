use anstream::println;
use anyhow::Result;
use xrpl_vl_tool::util::{base58_to_hex, Version};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version)]
#[clap(propagate_version = false)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Base58ToHex { key: String, key_type: String },
}

// {
//    "key_type" : "ed25519",
//    "public_key" : "nHUTxEuGtZYDwwvV7HfrAqBG5ttPZyAA2NiMnMRjz4bmgEhidayM",
//    "revoked" : false,
//    "secret_key" : "paPEQYWSi25pAakHs2yRAj7fSJovqoVMiN562MMNoe6U6HthUvM",
//    "token_sequence" : 0
// }

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Base58ToHex { key, key_type } => {
            let version = match key_type.as_str() {
                "public" => Version::NodePublic,
                "private" => Version::NodePrivate,
                _ => {
                    println!("Invalid key type. Use one of: public, private");
                    return Ok(());
                }
            };
            println!(
                "Base58: {}",
                base58_to_hex(key, version,).unwrap().to_uppercase()
            );
        }
    }

    Ok(())
}
