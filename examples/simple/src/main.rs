use nero::apps::filestatic::FileStatic;
use nero::project::{Project, Settings};
use nero::settings::AuthTokenConf;
use nero::urlpatterns::UrlPatterns;

pub mod messenger;

#[tokio::main]
async fn main() {
    Settings::set_admin_auth(AuthTokenConf {
        exr: 900,
        secret_key: Vec::from("SECRET_KEY_FOR_ADMIN"),
    });

    let file_static = FileStatic::app("/static/", "./static").unwrap();

    let mut apps = vec![messenger::build_app(), file_static];

    let admin_panel = nero_admin::build_app(&apps).await;
    apps.push(admin_panel);

    UrlPatterns::print_all_pattern(&apps);

    Project::new(apps).await.unwrap().run().await.unwrap();
}
