use dioxus::prelude::*;
use ui::Articles;

#[component]
pub fn Home() -> Element {
    rsx! {
        Articles {}
    }
}
