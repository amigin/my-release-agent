use std::sync::Arc;

use my_http_server::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut controllers = ControllersMiddleware::new(None, None);

    controllers.register_post_action(Arc::new(
        super::controllers::release_controller::ReleaseAction::new(app.clone()),
    ));

    controllers
}
