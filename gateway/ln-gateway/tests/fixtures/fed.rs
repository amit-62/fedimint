use std::collections::BTreeSet;
use std::sync::Arc;

use fedimint_client_legacy::api::fake::FederationApiFaker;
use fedimint_core::core::ModuleInstanceId;
use fedimint_core::PeerId;
use fedimint_ln_client::LightningGateway;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct MockApi {
    gateway: Option<LightningGateway>,
}

impl MockApi {
    pub async fn make_test_fed(
        module_id: ModuleInstanceId,
        members: BTreeSet<PeerId>,
    ) -> FederationApiFaker<tokio::sync::Mutex<MockApi>> {
        FederationApiFaker::new(Arc::new(Mutex::new(MockApi::default())), members)
            .with(
                format!("/module/{module_id}/register_gateway"),
                |mint: Arc<Mutex<MockApi>>, gateway: LightningGateway| async move {
                    mint.lock().await.gateway = Some(gateway);
                    Ok(())
                },
            )
            .with(
                format!("/module/{module_id}/list_gateways"),
                |mint: Arc<Mutex<MockApi>>, _: ()| async move {
                    Ok(mint
                        .lock()
                        .await
                        .gateway
                        .clone()
                        .into_iter()
                        .collect::<Vec<LightningGateway>>())
                },
            )
    }
}
