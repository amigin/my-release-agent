use std::sync::Arc;

use service_sdk::ServiceContext;

use crate::settings::SettingsReader;

pub struct AppContext {
    pub settings_reader: Arc<SettingsReader>,
}

impl AppContext {
    pub async fn new(
        settings_reader: Arc<SettingsReader>,
        _service_context: &ServiceContext,
    ) -> Self {
        let settings_reader = crate::settings::SettingsReader::new(".my-wallet-app").await;
        let settings_reader = Arc::new(settings_reader);

        let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;

        Self { settings_reader }
    }
}
