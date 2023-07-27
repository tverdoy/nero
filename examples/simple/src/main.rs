use nero::apps::filestatic::FileStatic;
use nero::project::{Project, Settings};

mod messenger;

#[tokio::main]
async fn main() {
    let settings = Settings {
        db_addr: "127.0.0.1:8000".to_string(),
        db_user: "root".to_string(),
        db_password: "root".to_string(),
        ..Default::default()
    };

    let file_static = FileStatic::app("/static/", "./static").unwrap();

    let apps = vec![messenger::build_app(), file_static];
    Project::new(settings, apps)
        .await
        .unwrap()
        .run()
        .await
        .unwrap();
}
