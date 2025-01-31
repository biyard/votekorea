#![allow(non_snake_case)]
use by_components::theme::ColorTheme;
use dioxus::prelude::*;
use dioxus_popup::PopupZone;
use dioxus_translate::*;

use super::header::Header;
use crate::{pages::footer::Footer, route::Route};

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let color: ColorTheme = use_context();
    let route = use_route::<Route>();

    use_effect(move || {
        tracing::debug!("use_effect route: {:?}", route);
    });

    rsx! {
        div {
            class: "flex flex-col items-center justify-start w-full min-h-[100vh] max-[1440px]:px-[10px]",
            background: color.background,
            color: color.text.primary,
            div { class: "max-w-[1440px] w-full ",
                Header { lang }
            }
            div { class: "w-full max-w-[1440px]", Outlet::<Route> {} }
            Footer { lang }
        }
        PopupZone {}
    }
}
