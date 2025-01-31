#![allow(non_snake_case)]
use by_components::theme::ColorTheme;
use dioxus::prelude::*;

use crate::{
    components::icons,
    pages::{highlighted_topic_component::HighlightedTopic, i18n::*},
};

use dioxus_translate::{translate, Language};

#[component]
pub fn HomePage(lang: Language) -> Element {
    let mut ctrl = super::controller::Controller::new(lang)?;
    let tr: PagesTranslate = translate(&lang);
    let about: AboutTranslate = translate(&lang);
    let color: ColorTheme = use_context();

    rsx! {
        div { class: "flex flex-col gap-[100px]",
            if let Some(ref topic) = ctrl.topic() {
                HighlightedTopic {
                    id: "topic-{topic.id}",
                    title: "{topic.title}",
                    content: "{topic.content}",
                    period: topic.period(),
                    yes: topic.voters,
                    requirement: topic.requirement,
                    amount: topic.amount,
                    voted: topic.voted,
                    lang,
                    onsubmit: move |_| ctrl.handle_vote(),
                }
            }

            div {
                id: "blockchain-vote-section",
                class: "flex flex-col gap-[10px] items-center justify-center",
                div { class: "flex flex-col gap-[8px] text-[48px] font-bold justify-center items-center",
                    span { "{tr.blockchain_vote_title1}" }
                    span { "{tr.blockchain_vote_title2}" }
                }

                div { class: "grid grid-cols-3 gap-[20px] p-[20px] justify-center items-center",
                    for (title , content) in [
                        (tr.blockchain_vote_tech_title, tr.blockchain_vote_tech_content),
                        (tr.blockchain_vote_org_title, tr.blockchain_vote_org_content),
                        (tr.blockchain_vote_transparency_title, tr.blockchain_vote_transparency_content),
                    ]
                    {
                        div {
                            class: "hover-effect col-span-1 flex flex-col px-[40px] py-[50px] gap-[20px] justify-start items-start rounded-[15px]",
                            background: "{color.card.primary}",
                            span { class: "hover-effect text-[24px] font-bold", "{title}" }
                            span {
                                class: "hover-effect text-[16px] font-normal",
                                color: "{color.text.secondary}",
                                "{content}"
                            }
                        }
                    }
                }
            }

            div {
                id: "about-section",
                class: "flex flex-col gap-[50px] items-center justify-center",
                div { class: "flex flex-col gap-[50px] items-center justify-center",
                    div { class: "flex flex-col gap-[8px] text-[48px] font-bold justify-center items-center",
                        span { "{about.title}" }
                    }
                    div { class: "w-full flex flex-col gap-[10px] items-center justify-center text-[24px] font-light max-w-[1100px] text-center",
                        span { "{about.content1}" }
                        span { "{about.content2}" }
                    }
                }

                div { class: "w-full grid grid-cols-2 grid-rows-2 p-[20px] gap-[20px]",

                    for (icon , title , content) in [
                        (rsx! {
                            icons::TransparencyIcon {}
                        }, about.trans_title, about.trans_content),
                        (rsx! {
                            icons::FairenessIcon {}
                        }, about.fair_title, about.fair_content),
                        (rsx! {
                            icons::SecurityIcon {}
                        }, about.secure_title, about.secure_content),
                        (rsx! {
                            icons::PublicIcon {}
                        }, about.public_title, about.public_content),
                    ]
                    {
                        div {
                            class: "hover-effect col-span-1 flex flex-row px-[20px] py-[30px] justify-start items-start rounded-[15px]",
                            background: "{color.card.primary}",
                            div { class: "hover-effect p-[10px] gap-[10px] flex justify-center items-center w-[230px] h-[180px]",
                                {icon}
                            }
                            div { class: "hover-effect flex flex-col gap-[20px] p-[20px] justify-center items-start",
                                span { class: "hover-effect text-[24px] font-bold", "{title}" }
                                span {
                                    class: "hover-effect text-[16px] font-normal",
                                    color: "{color.text.secondary}",
                                    "{content}"
                                }
                            }
                        }
                    }
                }

            }

        }
    }
}
