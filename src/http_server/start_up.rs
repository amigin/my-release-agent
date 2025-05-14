use std::{net::SocketAddr, sync::Arc};

use my_http_server::*;

use my_http_server::controllers::swagger::SwaggerMiddleware;

use crate::app::AppContext;

pub fn setup_server(app: &Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 5000)));

    let controllers = Arc::new(super::builder::build_controllers(app));

    let swagger_middleware = SwaggerMiddleware::new(
        controllers.clone(),
        crate::app::APP_NAME,
        crate::app::APP_VERSION,
    );

    http_server.add_middleware(Arc::new(swagger_middleware));

    http_server.add_middleware(controllers);

    http_server.add_middleware(Arc::new(StaticFilesMiddleware::new(None, None)));
    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
