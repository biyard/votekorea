#![allow(unused)]
use by_types::*;
use dioxus::prelude::*;
use dioxus_translate::Language;
use dto::*;

use crate::route::Route;

#[derive(Clone, Copy)]
pub struct Controller {
    pub size: usize,
    pub page: Signal<usize>,
    pub nav: Navigator,
    pub cli: Signal<TopicClient>,
    pub topics: Resource<QueryResponse<TopicSummary>>,
}

impl Controller {
    pub fn new() -> std::result::Result<Self, RenderError> {
        let size = 10;
        let page = Signal::new(1);
        let mut ctrl = Self {
            size,
            page,
            nav: use_navigator(),
            cli: Signal::new(Topic::get_client(&crate::config::get().main_api_endpoint)),
            topics: use_server_future(move || async move {
                let page = page();
                let cli = Topic::get_client(&crate::config::get().main_api_endpoint);
                match cli.query(TopicQuery::new(size).with_page(page)).await {
                    Ok(topics) => {
                        tracing::debug!("Topics: {:?}", topics);
                        topics
                    }
                    Err(err) => {
                        tracing::error!("Error: {:?}", err);
                        QueryResponse::default()
                    }
                }
            })?,
        };

        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn topics(&self) -> Vec<TopicSummary> {
        self.topics
            .with(|topics| topics.clone().unwrap_or_default().items)
    }

    pub fn total_count(&self) -> i64 {
        self.topics
            .with(|topics| topics.clone().unwrap_or_default().total_count)
    }

    pub fn navigate_to_create_topic(&self, lang: &Language) {
        self.nav.push(Route::NewTopicPage { lang: lang.clone() });
    }
}
