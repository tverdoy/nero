use nero::apps::filestatic::FileStatic;
use nero::project::Project;

mod messenger;

#[tokio::main]
async fn main() {
    let mut project = Project::new();
    let file_static = FileStatic::app("/static/", "./static").unwrap();

    project.add_apps(vec![messenger::build_app(), file_static]);

    project.run().await.unwrap();
}
