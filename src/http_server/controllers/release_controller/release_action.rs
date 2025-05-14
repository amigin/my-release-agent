use my_http_server::{macros::*, *};

use std::sync::Arc;

use crate::app::AppContext;

#[my_http_server::macros::http_route(
    method: "POST",
    route: "/api/release/v1",
    summary: "Release Service",
    description: "Release Service",
    controller: "Release",
    input_data: ReleaseHttpInputModel,
    result:[
        {status_code: 200, description: "Ok response"},
    ]
)]
pub struct ReleaseAction {
    app: Arc<AppContext>,
}

impl ReleaseAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ReleaseAction,
    input_data: ReleaseHttpInputModel,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    match crate::flows::release(&action.app, &input_data.id).await {
        Ok(ok) => HttpOutput::as_text(ok).into_ok_result(true).into(),
        Err(err) => HttpOutput::as_text(err).into_fail_result(500, false),
    }
}

#[derive(Debug, MyHttpInput)]
pub struct ReleaseHttpInputModel {
    #[http_query(description: "id of release unit" )]
    pub id: String,
}
