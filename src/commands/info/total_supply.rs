use crate::commands::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::types::AssetId;
use multiasset_sdk::MultiAssetContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Query an asset total supply")]
pub(crate) struct TotalSupplyCommand {
    /// The asset id
    #[clap(long)]
    pub(crate) asset: String,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl TotalSupplyCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        if self.asset.len() as u64 != 66 {
            anyhow::bail!("Invalid fuel asset length");
        }

        let asset_id = AssetId::from_str(&self.asset).expect("Invalid asset");

        // Connect to the deployed contract via the rpc
        let contract = MultiAssetContract::new(contract_id, wallet.clone()).await;

        // Create a new asset
        let total_supply = contract.total_supply(&asset_id).await?.value.unwrap();

        println!(
            "\nAn asset 0x{} total supply is: {}",
            self.asset, total_supply
        );

        Ok(())
    }
}
