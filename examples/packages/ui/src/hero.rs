use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            style: "padding: 3rem 1.5rem; text-align: center;",
            h1 { "News Aggregator" }
            p { "Browse the latest headlines gathered from supported sources." }
        }
    }
}
