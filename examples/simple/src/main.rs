use nero::project::Project;

mod messenger;

#[tokio::main]
async fn main() {
    let mut project = Project::new();

    project.add_apps(vec![messenger::build_app()]);

    project.run().await.unwrap();
}
