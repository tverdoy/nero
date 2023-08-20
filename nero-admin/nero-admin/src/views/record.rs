use async_trait::async_trait;
use serde::Deserialize;

use nero::http::Status;
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;

use crate::models::admin_user::AdminUser;

pub struct GetRecordView;

#[derive(Deserialize, Debug)]
struct GetRecordParams {
    id: String,
    app: String,
    model: String
}

#[async_trait]
impl View for GetRecordView {
    fn name(&self) -> &'static str {
        "Get record"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        AdminUser::check_auth(request).await?;
        let params: GetRecordParams = request.params_to_obj()?;


        Responder::text(Status::Ok, format!("{params:?}"))
    }
}
