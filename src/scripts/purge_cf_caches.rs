use my_settings_reader::flurl::FlUrl;
use serde::*;

pub async fn purge_cf_caches(api_key: &str, zone_id: &str) -> Result<String, String> {
    let model = PurgeCacheContent {
        purge_everything: true,
    };

    let auth_header = format!("Bearer {}", api_key);

    println!("Purge cache in zone: {}", zone_id);

    let mut response = FlUrl::new("https://api.cloudflare.com")
        .append_path_segment("client")
        .append_path_segment("v4")
        .append_path_segment("zones")
        .append_path_segment(zone_id)
        .append_path_segment("purge_cache")
        .with_header("Authorization", auth_header)
        .post_json(&model)
        .await
        .unwrap();

    let status_code = response.get_status_code();

    let body_as_str = response.body_as_str().await;
    if status_code == 200 {
        return Ok(format!(
            "Cloud flare cache purged Ok. StatusCode:200. Message:{:?}",
            body_as_str
        ));
    }

    Err(format!(
        "Error purging Cloudflare cache. StatusCode: {}. Err: {:?}",
        status_code, body_as_str
    ))
}

#[derive(Serialize, Deserialize)]
pub struct PurgeCacheContent {
    pub purge_everything: bool,
}
