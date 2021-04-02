use crate::services::is_authenticated;
use yew::prelude::*;
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::routes::AppRoute;

/// Home page
pub struct Home {
    link: ComponentLink<Self>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    Ignore,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut created = Self {
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        };

        if !is_authenticated() {
            created
                .router_agent
                .send(ChangeRoute(AppRoute::Login.into()));
        }
        return created;
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="app">
                <header class="app-header">
                    <p>
                        { "Syncify" }
                    </p>
                </header>
            </div>
        }
    }
}
