use std::{collections::HashMap, sync::Arc};

use crate::{crypto_wallets_grpc::FireblocksBridgeVaultWallet, AppContext};

pub async fn get_wallets_to_rebalance(app: &Arc<AppContext>) -> Vec<FireblocksBridgeVaultWallet> {
    let telemetry = my_telemetry::MyTelemetryContext::new();

    let fireblock_vaults = app
        .fireblocks_bridge_grpc_client
        .get_wallets(&telemetry)
        .await;

    let assets_trashholdears = get_rebalance_trashhold(&app).await;

    let wallets_to_rebalance = fireblock_vaults
        .iter()
        .filter(|x| {
            if app.settings_model.ignore_vault_ids.contains(&x.vault_id) {
                return false;
            }

            let asset_transhold = assets_trashholdears.get(&x.asset);

            if let None = asset_transhold {
                return false;
            }

            if x.available_balance >= asset_transhold.unwrap().to_owned() {
                return true;
            }

            return false;
        })
        .cloned()
        .collect();

    return wallets_to_rebalance;
}

async fn get_rebalance_trashhold(app: &Arc<AppContext>) -> HashMap<String, f64> {
    return app
        .crypto_deposit_settings_reader
        .get_table_snapshot()
        .await
        .unwrap()
        .values()
        .flat_map(|x| {
            x.values()
                .map(|x| (x.fireblocks_id.clone(), x.rebalance_trashold))
        })
        .collect();
}
