use async_trait::async_trait;
use nero::db::model::{format_table_name, SurrealDriver};
use nero::error::{Error, ErrorKind};
use serde::Deserialize;
use surrealdb::sql::Value;

use nero::http::Status;
use nero::project::{Project, APPS};
use nero::request::Request;
use nero::responder::Responder;
use nero::view::View;

use crate::models::admin_user::AdminUser;

pub struct GetRecordView;

#[derive(Deserialize, Debug)]
struct GetRecordParams {
    id: String,
    app: String,
    model: String,
}

#[async_trait]
impl View for GetRecordView {
    fn name(&self) -> &'static str {
        "Get record"
    }

    async fn callback(&self, request: &mut Request) -> nero::error::Result<Responder> {
        AdminUser::check_auth(request).await?;
        let params: GetRecordParams = request.params_to_obj()?;

        let apps = Project::apps().await;
        let app = apps
            .iter()
            .find(|app| app.name == params.app)
            .ok_or(Error::new(ErrorKind::ObjectGet, "App not found"))?;
        let model = app
            .models
            .iter()
            .find(|model| model.scheme.name == params.model)
            .ok_or(Error::new(ErrorKind::ObjectGet, "Model not found"))?;

        let res: serde_json::Value =
            SurrealDriver::get((format_table_name(&model.scheme.name), params.id).into()).await?;

        Responder::json(Status::Ok, res)
    }
}
