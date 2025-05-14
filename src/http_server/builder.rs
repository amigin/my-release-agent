use std::sync::Arc;

use service_sdk::HttpServerBuilder;

use crate::app::AppContext;

pub fn build_controllers(app: &Arc<AppContext>, http_server_builder: &mut HttpServerBuilder) {
    http_server_builder.register_post_action(
        super::controllers::release_controller::ReleaseAction::new(app.clone()),
    );
}
