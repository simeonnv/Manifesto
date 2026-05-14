use crate::{
    components::{Footer, Header},
    Route,
};
use dioxus::prelude::*;

#[component]
pub fn MainWrapper() -> Element {
    rsx! {
        div {
            class: "min-h-screen w-full flex flex-col justify-between items-center",

            Header {}

            div {
                class: "w-full flex-1",
                Outlet::<Route> {}
            }

            Footer {}
        }
    }
}
