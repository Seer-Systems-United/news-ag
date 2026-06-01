use dioxus::prelude::*;
use ui::{Articles, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Articles {}
    }
}
