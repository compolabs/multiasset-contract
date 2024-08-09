use crate::commands::core::{asset_new::AssetNewCommand, deploy::DeployCommand, mint::MintCommand};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum CoreCommands {
    /// Create a new asset
    #[clap(short_flag = 'A')]
    AssetNew(AssetNewCommand),

    /// Deploy a new multi asset contract
    #[clap(short_flag = 'D')]
    Deploy(DeployCommand),

    /// Mints an asset amount to recipient
    #[clap(short_flag = 'M')]
    Mint(MintCommand),
}