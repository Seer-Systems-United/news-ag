use api::Article;
use dioxus::prelude::*;

const ARTICLES_CSS: Asset = asset!("/assets/styling/echo.css");
const FALLBACK_THUMBNAIL_URL: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/0/0c/Associated_Press_logo_2012.svg/1920px-Associated_Press_logo_2012.svg.png";

#[component]
pub fn Articles() -> Element {
    let mut articles = use_signal(Vec::<Article>::new);

    use_effect(move || {
        spawn(async move {
            if let Ok(mut data) = api::ap_news_get_articles().await {
                data.sort_by(|a, b| b.published_at.cmp(&a.published_at));
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
                        style: "display: flex; flex-direction: column; height: 100%;",
                        a {
                            href: "{article.url}",
                            style: "display: flex; flex-direction: column; height: 100%; color: inherit; text-decoration: none;",
                            img {
                                src: article.thumbnail_url.as_deref().unwrap_or(FALLBACK_THUMBNAIL_URL),
                                width: "100%"
                            }
                            p {
                                style: "text-align: center;",
                                i { "{article.title}" }
                            }
                            p {
                                style: "margin-top: auto; text-align: center; font-size: 0.9rem; color: #666;",
                                "{article.published_at.map(|date| date.format(\"%Y-%m-%d\").to_string()).unwrap_or_else(|| \"Unknown date\".to_string())}"
                            }
                        }
                    }
                }
            }
        }
    }
}
