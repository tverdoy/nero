use nero::apps::filestatic::FileStatic;
use nero::apps::not_found::NotFound;
use nero::project::{Project, Settings};
use nero::settings::AuthTokenConf;

pub mod messenger;

#[tokio::main]
async fn main() {
    Settings::set_admin_auth(AuthTokenConf {
        exr: 900,
        secret_key: Vec::from("SECRET_KEY_FOR_ADMIN"),
    });

    let file_static = FileStatic::app("/static/", "./static").unwrap();

    let apps = vec![
        messenger::build_app(),
        file_static,
        nero_admin::build_app().await,
    ];

    Project::new(apps).await.unwrap().run().await.unwrap();
}
