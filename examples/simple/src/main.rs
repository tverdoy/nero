#[macro_use]
extern crate nero;

use nero::apps::filestatic::FileStatic;
use nero::project::Project;
use nero::settings::{AuthTokenConf, Settings};
use nero_admin::AdminPanel;

pub mod messenger;

#[tokio::main]
async fn main() {
    Settings::set_admin_auth(AuthTokenConf {
        exr: 900,
        secret_key: Vec::from("SECRET_KEY_FOR_ADMIN"),
    });

    FileStatic::register("/static/", "./static").await.unwrap();
    messenger::register().await;
    AdminPanel::register().await;

    Project::connect_to_db().await.unwrap();
    Project::run().await.unwrap();
}
