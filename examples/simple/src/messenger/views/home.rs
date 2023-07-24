use async_trait::async_trait;
use nero::request::Request;
use nero::view::View;

pub struct HomeView;

#[async_trait]
impl View for HomeView {
    async fn callback(&self, request: &mut Request) -> nero::error::Result<()> {
        println!("Hello: {:?}", request.http_head().method);

        request.send_text("How are u?").await;

        Ok(())
    }
}
