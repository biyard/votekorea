#![allow(non_snake_case)]
use by_components::theme::ColorTheme;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::{translate, Language};
use dto::Vote;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::{button::PrimaryButton, checkbox::Checkbox, inputs::LabeledInput},
    pages::i18n::VotingPopupTranslate,
};

#[component]
pub fn VotingPopup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
    topic_id: String,
    topic_title: String,
) -> Element {
    let tr: VotingPopupTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let color: ColorTheme = use_context();

    use_effect(move || {
        popup.with_title(&tr.title);
    });

    let mut checked = use_signal(|| false);
    let mut amount = use_signal(|| 0);
    let mut name = use_signal(|| "".to_string());

    rsx! {
        div { class: "w-full flex flex-col gap-[50px]",
            div { class: "w-full flex flex-col gap-[15px] items-start justify-start",
                LabeledInput {
                    title: "{tr.label_title}",
                    value: "{topic_title}",
                    disabled: true,
                }

                div { class: "w-full flex flex-row items-center justify-start gap-[2px]",
                    div { class: "text-[14px] font-bold", "{tr.agreement}" }
                    Checkbox { onchange: move |e| checked.set(e) }
                }

                if checked() {
                    LabeledInput {
                        title: "{tr.label_amount}",
                        oninput: move |e: String| {
                            amount.set(e.replace(",", "").parse::<i64>().unwrap_or(0));
                        },
                        value: amount().to_formatted_string(&Locale::en),
                        required: true,
                    }

                    LabeledInput {
                        title: "{tr.label_name}",
                        oninput: move |e| {
                            name.set(e);
                        },
                        required: true,
                        placeholder: "{tr.placeholder_name}",
                    }

                    div {
                        class: "flex flex-col gap-[16px] items-start justify-start text-[12px] font-bold",
                        color: color.text.secondary,
                        p { "{tr.notice}" }
                        div { class: "flex flex-col",
                            p { "{tr.account_info}" }
                            p { "{tr.account_owner}" }
                        }

                        p { class: "text-[10px]", "{tr.update}" }
                    }
                }

            }

            PrimaryButton {
                class: "w-full",
                onclick: move |_| {
                    let (amount, name) = match checked() {
                        true => (amount(), name()),
                        false => (0, "".to_string()),
                    };
                    tracing::debug!("name: {}, amount: {}", name, amount);
                    let topic_id = topic_id.clone();
                    async move {
                        match Vote::get_client(&crate::config::get().main_api_endpoint)
                            .support(&topic_id, amount, name)
                            .await
                        {
                            Ok(_) => {
                                popup.close();
                            }
                            Err(e) => {
                                tracing::error!("Error: {:?}", e);
                            }
                        }
                        popup.close();
                    }
                },
                "{tr.support}"
            }
        }
    }
}
