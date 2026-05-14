use dioxus::prelude::*;

mod components;
mod views;

use views::Index;

use crate::components::{FontFace, MainWrapper};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MainWrapper)]
        #[route("/")]
        Index {},
}

// const FONT_3270: Asset = asset!("/assets/3270-Regular.woff");
// const FONT_3270: Asset = asset!("../assets/3270Condensed-Regular.woff");
const FONT_3270: Asset = asset!("../assets/3270SemiCondensed-Regular.woff");
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!(
    "/assets/tailwind.css",
    AssetOptions::css().with_static_head(true)
);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        FontFace {
            family: "3270",
            style: "normal",
            weight: 400,
            asset: FONT_3270
        }

        div {
            class: "font-['3270']",
            Router::<Route> {}
        }
    }
}
