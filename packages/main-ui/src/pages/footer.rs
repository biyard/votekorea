#![allow(non_snake_case)]
use by_components::theme::ColorTheme;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{components::logo::LogoWrapper, pages::i18n::FooterTranslate};

#[component]
pub fn Footer(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    let tr: FooterTranslate = translate(&lang);
    let color: ColorTheme = use_context();

    rsx! {
        div {
            class: "w-full max-w-[1440px] gap-[45px] pb-[35px] pt-[80px] flex flex-col justify-center items-start",
            color: "{color.text.secondary}",
            div { class: "w-full flex flex-row justify-between items-center",
                div {
                    LogoWrapper {}
                    span { class: "text-[15px]", "{tr.service_desc}" }
                }
                div { class: "flex flex-col gap-[10px] text-[14px] items-start justify-start",
                    span { "{tr.info1}" }
                    span { "{tr.info2}" }
                }
            }

            div {
                class: "w-full h-[1px] opacity-30",
                background: "{color.text.secondary}",
            }

            div { class: "w-full flex flex-row justify-between items-center",
                span { "{tr.copyright}" }
                div { class: "flex flex-row gap-[20px] items-center justify-start",
                    a { href: "/#", "{tr.privacy_policy}" }
                    a { href: "/#", "{tr.terms_of_service}" }

                }
            }
        }
    }
}
