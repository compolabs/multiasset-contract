use clap::ValueEnum;
use fuels::prelude::{ContractId, Provider, WalletUnlocked};
use std::str::FromStr;

pub(crate) async fn setup(rpc: &str) -> anyhow::Result<WalletUnlocked> {
    let provider = Provider::connect(rpc).await?;
    let mnemonic = std::env::var("MNEMONIC")?;
    let wallet =
        WalletUnlocked::new_from_mnemonic_phrase(&mnemonic, Some(provider.clone())).unwrap();

    Ok(wallet)
}

pub(crate) fn validate_contract_id(contract_id: &str) -> anyhow::Result<ContractId> {
    if contract_id.len() as u64 != 66 {
        anyhow::bail!("Invalid contract id length");
    }

    Ok(ContractId::from_str(contract_id).expect("Invalid contract id"))
}

#[derive(Clone, ValueEnum)]
pub(crate) enum IdentityType {
    /// Externally Owned Account
    Address,
    /// Contract
    Contract,
}
