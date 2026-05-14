use dioxus::prelude::*;

#[component]
pub fn FontFace(family: &'static str, style: &'static str, weight: usize, asset: Asset) -> Element {
    rsx! {
        document::Style {{
            format!("
                @font-face {{
                    font-display: swap;
                    font-family: '{}';
                    font-style: {};
                    font-weight: {};
                    src: url('{}') format('woff');
                }}
                ", family, style, weight, asset
            )
        }}
    }
}
