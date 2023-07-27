use nero::apps::filestatic::FileStatic;
use nero::project::{Project, Settings};

mod messenger;

#[tokio::main]
async fn main() {
    let file_static = FileStatic::app("/static/", "./static").unwrap();

    let apps = vec![messenger::build_app(), file_static];
    Project::new(Settings::default(), apps).run().await.unwrap();
}
