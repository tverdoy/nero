pub mod server;

#[tokio::main]
async fn main() {
    server::Server::setup("127.0.0.1:8080")
        .await
        .unwrap()
        .run()
        .await;
}
