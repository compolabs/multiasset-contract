use fuels::prelude::{
    launch_custom_provider_and_get_wallets, WalletUnlocked, WalletsConfig, DEFAULT_COIN_AMOUNT,
};
use multiasset_sdk::MultiAssetContract;

pub(crate) struct User {
    pub(crate) wallet: WalletUnlocked,
}

pub(crate) async fn setup() -> anyhow::Result<(MultiAssetContract, User, User)> {
    let config = WalletsConfig::new(Some(2), Some(1), Some(DEFAULT_COIN_AMOUNT));
    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await?;
    let deployer = wallets.pop().unwrap();
    let user = wallets.pop().unwrap();

    let contract = MultiAssetContract::deploy(&deployer).await?;

    let deployer = User { wallet: deployer };
    let user = User { wallet: user };

    Ok((contract, deployer, user))
}
