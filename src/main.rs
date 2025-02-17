mod commands;

use clap::Parser;
use commands::{
    cli::{Cli, Command},
    core::cli::CoreCommands,
    info::cli::InfoCommands,
};

use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Command::Core(args) => match args.commands {
            CoreCommands::AssetNew(args) => args.run().await,
            CoreCommands::Deploy(args) => args.run().await,
            CoreCommands::Mint(args) => args.run().await,
            CoreCommands::MintMany(args) => args.run().await,
            CoreCommands::TransferMany(args) => args.run().await,
            CoreCommands::TransferAmountMany(args) => args.run().await,
        },
        Command::Info(args) => match args.commands {
            InfoCommands::Bech32Conv(args) => args.run().await,
            InfoCommands::Decimals(args) => args.run().await,
            InfoCommands::Name(args) => args.run().await,
            InfoCommands::RestrictedMint(args) => args.run().await,
            InfoCommands::Symbol(args) => args.run().await,
            InfoCommands::TotalAssets(args) => args.run().await,
            InfoCommands::TotalSupply(args) => args.run().await,
        },
    }
}
