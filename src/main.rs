use std::sync::Arc;

use app::AppContext;

mod app;
mod flows;
mod http_server;
mod settings;

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".release-agent").await;
    let settings_reader = Arc::new(settings_reader);

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;

    let app = Arc::new(AppContext::new(settings_reader, &service_context).await);

    service_context.configure_http_server(|builder| {
        http_server::builder::build_controllers(&app, builder);
    });

    service_context.start_application().await;
}
