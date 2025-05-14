use my_settings_reader::SettingsModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, SettingsModel)]
pub struct SettingsModel {
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
