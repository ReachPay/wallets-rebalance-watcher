use std::sync::Arc;

use my_sb_contracts::FireblocksRebalanceCommand;
use rust_extensions::MyTimerTick;

use crate::{get_wallets_to_rebalance, AppContext, crypto_wallets_grpc::FireblocksBridgeVaultWallet};

pub struct RebalanceCheckTimer {
    pub app: Arc<AppContext>,
}

impl RebalanceCheckTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for RebalanceCheckTimer {
    async fn tick(&self) {
        let wallets_to_rebalance = get_wallets_to_rebalance(&self.app).await;
        let data_to_publish: Vec<FireblocksRebalanceCommand> = wallets_to_rebalance
            .iter()
            .map(|x| x.to_owned().into())
            .collect();

        self.app.rebalance_event_publisher.publish_messages(&data_to_publish).await.unwrap();
    }
}


impl Into<FireblocksRebalanceCommand> for FireblocksBridgeVaultWallet {
    fn into(self) -> FireblocksRebalanceCommand {
        FireblocksRebalanceCommand{
            asset_id: self.asset,
            vault_id: self.vault_id,
        }
    }
}