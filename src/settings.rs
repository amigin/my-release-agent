use serde::{Deserialize, Serialize};

service_sdk::macros::use_settings!();

#[derive(
    my_settings_reader::SettingsModel,
    AutoGenerateSettingsTraits,
    SdkSettingsTraits,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct SettingsModel {
    pub seq_conn_string: String,
    pub my_telemetry: Option<String>,
    pub configurations: Vec<ReleaseConfigurationSettings>,
}

impl SettingsModel {
    pub fn find_configuration(&self, id: &str) -> Option<&ReleaseConfigurationSettings> {
        for itm in self.configurations.iter() {
            if itm.id == id {
                return Some(itm);
            }
        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseConfigurationSettings {
    pub id: String,
    pub path: String,
    pub reset_cache_cloud_flare_api_key: Option<String>,
}
