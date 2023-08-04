<div align="center">
    <h1>Nero</h1>
    <p>
        <strong>Nero is a web framework with which you can think about the logic of the task rather than the way it is implemented</strong>
    </p>
</div>

---

See directory `examples`

## How to use

1. Project creation, customization and connection of applications

```rust
#[tokio::main]
async fn main() {
    let file_static = FileStatic::app("/static/", "./static").unwrap();

    let apps = vec![messenger::build_app(), file_static];
    Project::new(Settings::default(), apps).run().await.unwrap();
}
```

2. Create app

```rust
pub fn build_app() -> AppCard {
    let mut patterns = UrlPatterns::default();
    patterns.add(vec![("/home", Box::new(views::home::HomeView))]);

    AppCard::new("Messenger", patterns)
}
```

3. Create view

```rust
#[async_trait]
impl View for HomeView {
    fn name(&self) -> &'static str {
        "Home"
    }

    async fn callback(&self, _request: &mut Request) -> nero::error::Result<Responder> {
        Responder::file(Status::Ok, "./src/messenger/templates/home.html").await
    }
}
```