#![allow(non_snake_case)]
use super::i18n::*;
use crate::{
    components::{button::RoundedYesButton, icon_text::IconText, icons},
    services::user_service::UserService,
    theme::Theme,
};
use by_components::theme::ColorTheme;
use dioxus::prelude::*;
use dioxus_translate::*;
use dto::UserRole;
use num_format::{Locale, ToFormattedString};

#[component]
pub fn HighlightedTopic(
    id: String,
    title: String,
    content: String,

    period: String,
    yes: i64,
    requirement: i64,
    amount: i64,
    lang: Language,
    voted: bool,

    onsubmit: EventHandler<MouseEvent>,
) -> Element {
    let color: ColorTheme = use_context();
    let tr: HighlightedTopicTranslate = translate(&lang);
    let amount = amount.to_formatted_string(&Locale::en);
    let user_service: UserService = use_context();
    let remaining_people = requirement - yes;

    rsx! {
        div {
            id,
            class: "w-full flex flex-col gap-[10px] bg-[{color.card.primary}] rounded-[8px]  py-[50px]",
            div {
                class: "transition-all flex flex-col justify-start items-start py-[20px] gap-[30px] px-[80px]",
                border_bottom: if user_service.role() != UserRole::Guest { format!("1px solid {}", color.button.secondary) } else { "".to_string() },
                div { class: "flex flex-col gap-[20px] items-start justify-start",
                    h1 {
                        class: "text-[42px] font-extrabold tracking-normal line-clamp-1",
                        color: "{color.text.primary}",
                        "{title}"
                    }
                    p {
                        class: "text-[16px] max-w-[674px] font-regular leading-[24px] tracking-[0.5px] line-clamp-4",
                        color: "{color.text.secondary}",
                        "{content}"
                    }

                    div { class: "flex flex-row gap-[8px]",
                        div {
                            class: "flex flex-row gap-[4px] text-[14px] font-bold px-[14px] py-[8px] rounded-[8px] tooltip cursor-help",
                            "data-tip": "{tr.period_tooltip}",
                            background: "{color.button.secondary}",
                            icons::Clock {}
                            "{period}"
                        }

                        div {
                            class: "flex flex-row gap-[4px] text-[14px] font-bold px-[14px] py-[8px] rounded-[8px] tooltip cursor-help",
                            "data-tip": "{tr.requirement_tooltip}",
                            background: "{color.button.secondary}",
                            icons::RequirementIcon { width: "19", height: "20" }
                            "{requirement} {tr.unit}"
                        }

                        div {
                            class: "flex flex-row gap-[4px] text-[14px] font-bold px-[14px] py-[8px] rounded-[8px] tooltip cursor-help",
                            "data-tip": "{tr.amount_tooltip}",
                            icons::Money {}
                            "{tr.amount_title} {amount} {tr.currency}"
                            icons::Tooltip {}
                        }
                    }
                }

                div { class: "w-full flex flex-row gap-[10px] items-center justify-start",
                    VoteResultHorizontalBars {
                        class: "grow",
                        yes,
                        requirement,
                        lang,
                    }
                    div {
                        class: "flex flex-row gap-[4px] text-[14px] font-bold px-[14px] py-[8px] rounded-[8px] tooltip cursor-help",
                        "data-tip": "{tr.remaining_tooltip}",
                        icons::OutlinedHandshakeIcon { size: 20.0 }
                        "{remaining_people} {tr.unit}"
                    }
                }
            }

            if user_service.role() != UserRole::Guest {
                if voted {
                    div { class: "w-full text-[16px] font-bold flex flex-row items-center justify-center gap-[10px] px-[10px] mt-[30px]",
                        "{tr.voted}"
                    }
                } else {
                    RoundedYesButton {
                        class: "transition-all w-full flex flex-row justify-center items-center px-[10px] mt-[30px]",
                        disabled: voted,
                        onclick: if !voted { onsubmit },
                        "{tr.btn_support}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn DonationSelector(
    #[props(default ="donation_selector".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    onselect: EventHandler<u64>,
) -> Element {
    let theme: Theme = use_context();
    let theme = theme.get_data();
    let mut value = use_signal(|| 0);

    rsx! {
        div { id, class,
            div {
                class: "w-full flex flex-row items-center justify-start gap-[40px] px-[32px] py-[9px] rounded-[8px] bg-[{theme.background}] h-full opacity-80 hover:opacity-100 cursor-pointer",
                onclick: move |_| {
                    if value() == 0 {
                        value.set(1000);
                        onselect(1000);
                    } else {
                        value.set(0);
                        onselect(0);
                    }
                },
                div {
                    class: "relative w-full h-[8px] rounded-full bg-[#1F202E]",
                    style: "position: relative;",
                    div { class: "absolute top-0 left-0 h-[8px] w-[calc({value/10}%)] bg-gradient-to-r from-[#5A68FF] to-[{theme.active}] rounded-full" }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "1000",
                        disabled: true,
                        value: "{value().to_formatted_string(&Locale::en)}",
                        class: "absolute w-full h-4 opacity-0 cursor-pointer",
                                        // oninput: move |evt| {
                    //     value.set(evt.value().parse::<i32>().unwrap_or(0));
                    // }
                    }
                    div {
                        class: "absolute flex items-center top-[-8px] justify-center shadow-md",
                        style: "left: calc({value/10}% - 12px);",
                        icons::SlideBall {}
                    }
                }

                div { class: "flex flex-row items-center gap-[8px] max-[600px]:hidden overflow-hidden",
                    input {
                        r#type: "number",
                        value: "{value}",
                        disabled: true,
                        class: "w-[90px] h-[39px] text-[18px] font-bold text-white bg-[#1F202E] rounded-[6px] text-right py-[8px] px-[20px] cursor-pointer",
                        {format!("{}", value().to_formatted_string(&Locale::en))}
                    }
                    span { class: "text-[16px] font-bold text-[{theme.primary03}]", "원" }
                }
            }
        }
    }
}

#[component]
pub fn DescriptionWrapper(title: String, content: String) -> Element {
    let theme: Theme = use_context();
    let theme_data = theme.get_data();
    rsx! {
        div { class: "flex flex-col gap-[22px] items-start justify-start",
            h1 { class: "text-[28px] font-extrabold tracking-normal line-clamp-1",
                "{title}"
            }
            p {
                class: "text-[16px] max-w-[674px] font-regular leading-[24px] tracking-[0.5px] line-clamp-4",
                style: "color: {theme_data.primary00};",
                "{content}"
            }
        }
    }
}

#[component]
pub fn ContentWrapper(
    title: String,
    description: String,
    period: String,
    donations: u64,
    replies: u64,
    lang: Language,
) -> Element {
    let theme: Theme = use_context();
    let theme_data = theme.get_data();
    let tr = translate::<ContentWrapperTranslate>(&lang);
    rsx! {
        div { class: "flex flex-col gap-[22px] items-start justify-start h-[209px]",
            h1 { class: "text-[42px] font-extrabold tracking-normal line-clamp-1",
                "{title}"
            }
            p {
                class: "text-[16px] max-w-[674px] font-regular leading-[24px] tracking-[0.5px] line-clamp-2",
                style: "color: {theme_data.primary00};",
                "{description}"
            }
            div { class: "flex flex-row gap-[8px] justify-start items-center",
                IconText { text: "{period}", background: "{theme_data.primary06}", icons::Clock {} }
                IconText {
                    text: format!(
                        "{} {}₩",
                        tr.cumulative_donations,
                        donations.to_formatted_string(&Locale::en),
                    ),
                    icons::Money {}
                }
                IconText { class: "", text: "{replies}", icons::ChatBubble {} }
            }
        }
    }
}

#[component]
pub fn VoteResultHorizontalBars(
    yes: i64,
    requirement: i64,
    lang: Language,
    #[props(default = "w-[580px]".to_string())] class: String,
) -> Element {
    let yes_percent = (yes as f32) / (requirement as f32) * 100.0;
    tracing::debug!("yes =  {} requirement = {}", yes_percent, requirement);
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();
    let tr: VoteResultHorizontalBarsTranslate = translate(&lang);
    let yes = yes.to_formatted_string(&Locale::en);

    rsx! {
        div { class: "flex flex-row justify-start items-center gap-[4px] {class}",
            div { class: "w-[{yes_percent}%] flex flex-row justify-start items-center gap-[5px]",
                div {
                    class: "relative animate-grow flex flex-row justify-end items-center px-[20px] text-[15px] font-bold w-[calc(50%-6px)] h-[28px] rounded-[6px]",
                    style: "background: linear-gradient(90deg, {theme.primary05} 0%, rgba(104, 211, 108, 0.5) 100%);",
                    div { class: "absolute z-[20] h-[22px] w-[22px] right-[2.46px] top-[3px] rounded-[6px] bg-[{theme.active}] opacity-50" }
                    span { class: "absolute top-[6px] right-[5px] z-[30] text-[12px]",
                        "{yes}{tr.people_unit}/{yes_percent}%"
                    }
                }

            }

        }
    }
}
