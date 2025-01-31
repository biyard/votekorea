use dioxus::CapturedError;
use dioxus_aws::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::Language;
use dto::*;

use crate::{config, pages::voting_popup::VotingPopup, services::user_service::UserService};

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub topic: Resource<Topic>,
    pub popup: PopupService,
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let conf = config::get();
        let user_service: UserService = use_context();

        let ctrl = Self {
            lang,
            popup: use_context(),
            topic: use_server_future(move || {
                let role = (user_service.role)();

                async move {
                    tracing::debug!("Role: {:?}", role);

                    let cli = Topic::get_client(&conf.main_api_endpoint);
                    match cli.get_topic().await {
                        Ok(topic) => {
                            tracing::debug!("Topic: {:?}", topic);
                            topic
                        }
                        Err(err) => {
                            tracing::error!("Error: {:?}", err);
                            Topic::default()
                        }
                    }
                }
            })?,
        };
        use_context_provider(|| ctrl);
        tracing::debug!("Controller initialized");

        Ok(ctrl)
    }

    pub fn topic(&self) -> Option<Topic> {
        self.topic.with(|topic| topic.clone())
    }

    pub fn handle_vote(&mut self) {
        let topic = match self.topic() {
            Some(topic) => topic,
            None => {
                tracing::error!("No topic found");
                return;
            }
        };

        self.popup.open(rsx! {
            VotingPopup {
                lang: self.lang,
                topic_id: topic.id.as_str(),
                topic_title: topic.title.as_str(),
            }
        });
    }
}
