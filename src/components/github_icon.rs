use dioxus::prelude::*;

#[component]
pub fn GithubIcon(name: String, class: Option<String>) -> Element {
    let avatar_url = format!("https://github.com/{}.png", name);

    rsx! {
        img {
            class: class,
            src: "{avatar_url}",
            alt: "{name}'s github icon"
        }
    }
}
