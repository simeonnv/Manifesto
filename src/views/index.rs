use dioxus::prelude::*;

#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            class: "w-full flex items-center justify-center",

            div {
                class: "bg-red-300 w-full md:w-3/4" ,
                "hallo"
            }
            // p {
            //     class: "text-3xl",
            //     "Engineering the invisible infrastructure that scales the modern web."
            // }
        }
    }
}
