pub mod components;
pub mod config;
pub mod constants;
pub mod pages;
pub mod route;
pub mod services;
pub mod theme;

use by_components::effects::HoverEffects;
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use route::Route;
use services::user_service::UserService;
use theme::Theme;

fn main() {
    let conf = config::get();
    dioxus_logger::init(conf.log_level).expect("failed to init logger");
    tracing::debug!("config: {:?}", conf);
    rest_api::set_message(conf.domain.to_string());

    dioxus_aws::launch(app);
}

fn app() -> Element {
    Theme::init();
    UserService::init();
    PopupService::init();

    rsx! {
        document::Title { "국민투표" }
        document::Meta { name: "title", content: "" }
        document::Meta { name: "description", content: "" }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }

        document::Meta {
            name: "keywords",
            content: "SEO, meta tags, search engine optimization",
        }

        document::Link { id: "favicon", rel: "icon", href: asset!("/public/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/public/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css") }
        ImportDevScript {}
        HoverEffects {}

        Router::<Route> {}
    }
}
#[component]
pub fn ImportDevScript() -> Element {
    rsx! {
        div {
            document::Link {
                rel: "stylesheet",
                href: "https://cdn.jsdelivr.net/npm/daisyui@4.12.23/dist/full.min.css",
            }
            document::Script { src: "https://cdn.tailwindcss.com" }
        }
    }
}

#[cfg(feature = "server")]
mod api {
    use dioxus::fullstack::prelude::*;
    use server_fn::codec::{GetUrl, Json};

    #[server(endpoint = "/version", input=GetUrl, output=Json)]
    pub async fn version() -> Result<String, ServerFnError> {
        Ok(match option_env!("VERSION") {
            Some(version) => match option_env!("COMMIT") {
                Some(commit) => format!("{}-{}", version, commit),
                None => format!("{}", version),
            },
            None => match option_env!("DATE") {
                Some(date) => date.to_string(),
                None => "unknown".to_string(),
            },
        }
        .to_string())
    }
}
