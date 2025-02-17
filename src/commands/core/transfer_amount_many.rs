use crate::commands::utils::{setup, IdentityType};
use clap::Args;
use fuels::{
    accounts::{Account, ViewOnlyAccount},
    types::{transaction::TxPolicies, Address, AssetId, ContractId},
};
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Transfers an asset amounts to recipients")]
pub(crate) struct TransferAmountManyCommand {
    /// The b256 id of the account
    #[clap(long)]
    pub(crate) recipient_id: Vec<String>,

    /// The type of account
    #[clap(long)]
    pub(crate) recipient_type: IdentityType,

    /// The asset id
    #[clap(long)]
    pub(crate) asset: String,

    /// The amount to mint
    /// Ex. 10000000
    #[clap(long)]
    pub(crate) amount: Vec<u64>,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl TransferAmountManyCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        if self.asset.len() as u64 != 66 {
            anyhow::bail!("Invalid fuel asset length");
        }

        if self.recipient_id.len() as u64 != self.amount.len() as u64 {
            anyhow::bail!("Recepient and amount sizes are not equal");
        }

        let asset_id = AssetId::from_str(&self.asset).expect("Invalid asset");

        // Transfer asset amount
        match self.recipient_type {
            IdentityType::Address => {
                let mut index = 1;
                for recipient_id in &self.recipient_id {
                    let address = Address::from_str(&recipient_id).expect("Invalid address");
                    let amount = *self.amount.get(index - 1).unwrap();
                    wallet
                        .transfer(&address.into(), amount, asset_id, TxPolicies::default())
                        .await?;
                    println!(
                        "\n{}. An asset {} amount transfered {} to: {}",
                        index, self.asset, amount, recipient_id
                    );
                    index += 1;
                }
            }
            IdentityType::Contract => {
                let mut index = 1;
                for recipient_id in &self.recipient_id {
                    let address = ContractId::from_str(&recipient_id).expect("Invalid contract id");
                    let amount = *self.amount.get(index - 1).unwrap();
                    wallet
                        .force_transfer_to_contract(
                            &address.into(),
                            amount,
                            asset_id,
                            TxPolicies::default(),
                        )
                        .await?;
                    println!(
                        "\n{}. An asset {} amount transfered {} to: {}",
                        index, self.asset, amount, recipient_id
                    );
                    index += 1;
                }
            }
        };

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("Transaction cost: {}", balance - new_balance);
        println!("Sender: 0x{}", wallet.address().hash());

        Ok(())
    }
}
