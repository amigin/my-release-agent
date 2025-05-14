use std::sync::Arc;

use app::AppContext;

mod app;
mod flows;
mod http_server;
mod settings;

#[tokio::main]
async fn main() {
    let settings_file = rust_extensions::file_utils::format_path("~/.release-agent");
    let settings_reader = crate::settings::SettingsReader::new(settings_file.as_str()).await;
    let settings_reader = Arc::new(settings_reader);
    let app = AppContext::new(settings_reader).await;
    let app = Arc::new(app);

    crate::http_server::setup_server(&app);
    app.app_states.wait_until_shutdown().await;
}
