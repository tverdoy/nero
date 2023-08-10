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

    let file_static = FileStatic::build_app("/static/", "./static").unwrap();
    let mut apps = vec![messenger::build_app(), file_static];

    let admin_app = AdminPanel::new(&apps).build_app();
    apps.push(admin_app);

    Project::new(apps).await.unwrap().run().await.unwrap();
}
