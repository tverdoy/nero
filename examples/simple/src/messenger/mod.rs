mod views;

use nero::app::App;
use nero::urlpatterns::UrlPatterns;
use nero::view::View;

struct Messenger {}

impl App for Messenger {
    fn name() -> &'static str {
        "messenger"
    }

    fn url_patterns() -> UrlPatterns {
        let home_view = View::new(Box::new(views::home));

        let mut patterns = UrlPatterns::new();
        patterns.add(vec![
            ("/home", home_view)
        ]);

        patterns
    }
}

fn y() {
    let asd = views::home;
}