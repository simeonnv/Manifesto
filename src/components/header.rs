use dioxus::prelude::*;

use crate::components::github_icon::GithubIcon;

static DISCORD: Asset = asset!("/assets/Discord-Logo-Fill--Streamline-Phosphor-Fill.svg");
static INSTAGRAM: Asset = asset!("/assets/Logo-Social-Media-Old-Instagram--Streamline-Pixel.svg");

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-4 p-2 w-full border-b border-primary items-center",

            div {
                class: "avatar w-16 h-16 border border-primary",

                GithubIcon {
                    name: "simeonnv",
                }
            }

            div {
                class: "flex flex-col gap-1 items-center justify-center font-bold",

                "SimeonNV"
                p {
                    class: "badge badge-outline badge-primary rounded-none",
                    "Available"
                }
            }

            div { class: "grow" }

            nav {
                class: "hidden md:flex flex-row gap-6 mr-4 font-medium",
                a { href: "#projects", class: "hover:text-primary transition", "Projects" }
                a { href: "#stack", class: "hover:text-primary transition", "Stack" }
                a { href: "#contact", class: "hover:text-primary transition", "Contact" }
            }

            div {
                class: "flex flex-row gap-6 pr-4",

                img {
                    class: "w-8 h-8 opacity-70 hover:opacity-100 invert brightness-0",
                    src: DISCORD
                }
                img {
                    class: "w-8 h-8 opacity-70 hover:opacity-100 invert brightness-0",
                    src: INSTAGRAM
                }
            }
        }
    }
}
