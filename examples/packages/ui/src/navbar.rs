use dioxus::prelude::*;

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        nav {
            style: "display: flex; gap: 1rem; padding: 1rem 1.5rem; align-items: center;",
            {children}
        }
    }
}
