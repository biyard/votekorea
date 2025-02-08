#![allow(non_snake_case)]
use super::i18n::SignupPopupTranslate;
use crate::{
    components::icons,
    constants::DEFAULT_PROFILE,
    pages::user_setup_popup::UserSetupPopup,
    services::user_service::{UserEvent, UserService},
};
use by_components::theme::ColorTheme;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::*;
use dto::*;

#[component]
pub fn SignupPopup(
    #[props(default ="signup_popup".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    // onclick: EventHandler<Event<MouseData>>,
    lang: Language,
) -> Element {
    let tr = translate::<SignupPopupTranslate>(&lang);
    let color: ColorTheme = use_context();
    let mut user: UserService = use_context();
    let mut popup: PopupService = use_context();

    rsx! {
        div { id, class,
            button {
                class: "w-full flex flex-row my-[10px] p-[8px] rounded-[8px] justify-start items-center gap-[17px] cursor-pointer bg-[{color.card.secondary}] hover:bg-[{color.card.primary}]",
                onclick: move |_| async move {
                    if let Ok((id_token, res)) = handle_kakao_login().await {
                        match user.login(id_token, res.sub).await {
                            UserEvent::Login => {
                                popup.close();
                            }
                            UserEvent::Signup => {
                                popup.open(rsx! {
                                    UserSetupPopup {
                                        class: "w-[400px]",
                                        nickname: res.nickname.unwrap_or_default(),
                                        profile_url: res.picture.unwrap_or(DEFAULT_PROFILE.to_string()),
                                        email: res.email.unwrap_or_default(),
                                        lang: lang.clone(),
                                    }
                                });
                            }
                            _ => {}
                        };
                    }
                },
                div {
                    class: "rounded-[8px] w-[62px] h-[62px] flex justify-center items-center",
                    background: color.services.kakao,
                    icons::Kakao {}
                }
                div { class: "flex flex-col gap-[3px]",
                    span { class: "text-white text-[16px] leading-[16px] font-extrabold",
                        "{tr.continue_with_kakao}"
                    }
                }
            }
        }
    }
}

pub async fn handle_kakao_login() -> Result<(String, dioxus_oauth::prelude::OpenIdResponse)> {
    use dioxus_oauth::prelude::*;

    let conf = &crate::config::get().kakao;
    let client = OAuthClient::new(
        &conf.client_id,
        &conf.redirect_uri,
        "https://kauth.kakao.com/oauth/authorize",
        "https://kauth.kakao.com/oauth/token",
    )
    .set_openid_url("https://kauth.kakao.com/oauth/tokeninfo");

    let code: String = match client.get_auth_code().await {
        Ok(code) => code,
        Err(e) => {
            tracing::error!("Auth code failed: {:?}", e);
            return Err(ServiceError::Unauthorized);
        }
    };

    let token_response: TokenResponse = match client.get_token(code.as_str()).await {
        Ok(token_response) => token_response,
        Err(e) => {
            tracing::error!("Token response failed: {:?}", e);
            return Err(ServiceError::Unauthorized);
        }
    };
    tracing::debug!("Token response: {:?}", token_response);
    let oid_response: dioxus_oauth::prelude::OpenIdResponse =
        match client.get_openid(&token_response.id_token).await {
            Ok(oid_response) => oid_response,
            Err(e) => {
                tracing::error!("Token response failed: {:?}", e);
                return Err(ServiceError::Unauthorized);
            }
        };

    tracing::debug!("OID response: {:?}", oid_response);
    Ok((token_response.id_token, oid_response))
}
