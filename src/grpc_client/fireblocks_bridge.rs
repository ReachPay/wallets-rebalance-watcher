use std::time::Duration;

use my_grpc_extensions::GrpcClientInterceptor;
use my_telemetry::MyTelemetryContext;
use tonic::{codegen::InterceptedService, transport::Channel};

use crate::crypto_wallets_grpc::{
    crypto_wallets_manager_client::CryptoWalletsManagerClient, FireblocksBridgeVaultWallet,
};

pub struct FireblocksBridgeGrpcService {
    channel: Channel,
    timeout: Duration,
}

impl FireblocksBridgeGrpcService {
    pub async fn new(grpc_address: String) -> Self {
        let channel = Channel::from_shared(grpc_address)
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            channel,
            timeout: Duration::from_secs(30),
        }
    }

    fn create_grpc_service(
        &self,
        my_telemetry_context: &MyTelemetryContext,
    ) -> CryptoWalletsManagerClient<InterceptedService<Channel, GrpcClientInterceptor>> {
        return CryptoWalletsManagerClient::with_interceptor(
            self.channel.clone(),
            GrpcClientInterceptor::new(my_telemetry_context.clone()),
        );
    }

    pub async fn get_wallets(
        &self,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Vec<FireblocksBridgeVaultWallet> {
        let mut client = self.create_grpc_service(my_telemetry_context);
        let response = client
            .get_vault_wallets(tonic::Request::new(()))
            .await
            .unwrap();

        return my_grpc_extensions::read_grpc_stream::as_vec(response.into_inner(), self.timeout)
            .await
            .unwrap()
            .unwrap();
    }
}
