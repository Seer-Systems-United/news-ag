use api::Article;
use dioxus::prelude::*;

const ARTICLES_CSS: Asset = asset!("/assets/styling/echo.css");
const FALLBACK_THUMBNAIL_URL: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/0/0c/Associated_Press_logo_2012.svg/1920px-Associated_Press_logo_2012.svg.png";

#[component]
pub fn Articles() -> Element {
    let mut articles = use_signal(Vec::<Article>::new);

    use_effect(move || {
        spawn(async move {
            if let Ok(data) = api::ap_news_get_articles().await {
                articles.set(data);
            }
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: ARTICLES_CSS }
        div {
            style: "padding: 1.5rem;",
            h1 {
                style: "text-align: center;",
                "Example App"
            }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 20px;",
                for article in articles().iter() {
                    div {
                        key: "{article.title}",
                        a {
                            href: "{article.url}",
                            style: "display: block; color: inherit; text-decoration: none;",
                            img {
                                src: article.thumbnail_url.as_deref().unwrap_or(FALLBACK_THUMBNAIL_URL),
                                width: "100%"
                            }
                            p {
                                style: "text-align: center;",
                                i { "{article.title}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
