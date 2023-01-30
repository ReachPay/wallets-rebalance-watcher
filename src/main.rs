use std::{sync::Arc, time::Duration};

use rust_extensions::MyTimer;
use wallets_rebalance_watcher::{SettingsReader, AppContext, RebalanceCheckTimer};


#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".reachpay").await;
    let settings_reader = Arc::new(settings_reader);

    let app = Arc::new(AppContext::new(&settings_reader).await);
    let settings_model = settings_reader.get_settings().await;
    let mut rebalance_timer  = MyTimer::new(Duration::from_secs(settings_model.rebalance_check_period_in_seconds));
    rebalance_timer.register_timer("RebalanceTimer", Arc::new(RebalanceCheckTimer::new(app.clone())));

    app.sb_client.start().await;
    app.nosql_connection.start().await;
    rebalance_timer.start(app.app_states.clone(), my_logger::LOGGER.clone());
    app.app_states.wait_until_shutdown().await;
}
