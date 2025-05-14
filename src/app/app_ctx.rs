use std::sync::Arc;

use rust_extensions::AppStates;

use crate::settings::SettingsReader;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    pub settings_reader: Arc<SettingsReader>,
    pub app_states: Arc<AppStates>,
}

impl AppContext {
    pub async fn new(settings_reader: Arc<SettingsReader>) -> Self {
        Self {
            app_states: Arc::new(AppStates::create_initialized()),
            settings_reader,
        }
    }
}
