use yew::prelude::*;

use anyhow;
use serde::{Deserialize, Serialize};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
    format::{Json, Nothing},
    prelude::*,
};

use crate::routes::AppRoute;
use crate::services::set_token;
use serde_json::json;
use yew::utils::window;
use yew_router::{
    agent::RouteRequest::ChangeRoute, prelude::*, route::Route, service::RouteService,
};

/// Redirect Page
pub struct Redirect {
    fetch_task: Option<FetchTask>,
    token: Option<String>,
    link: ComponentLink<Self>,
    error: Option<String>,
    route: RouteService,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

#[derive(Deserialize, Clone)]
pub struct Code {
    code: String,
}

impl From<Result<String, anyhow::Error>> for Code {
    fn from(result: Result<String, anyhow::Error>) -> Code {
        return Code {
            code: result.expect("failed to unwwrap"),
        };
    }
}

#[derive(Deserialize, Clone)]
pub struct Token {
    status: String,
    token: String,
}

impl From<Result<String, anyhow::Error>> for Token {
    fn from(result: Result<String, anyhow::Error>) -> Token {
        return Token {
            token: result.expect("failed to unwwrap"),
            status: String::from("ok"),
        };
    }
}

pub enum Msg {
    Ignore,
    ReceiveResponse(Token),
}

impl Component for Redirect {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut created = Self {
            fetch_task: None,
            token: None,
            error: None,
            route: RouteService::new(),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        };

        let query = RouteService::get_query(&created.route);
        let mut query_code: Vec<&str> = query.split('=').collect();
        query_code = query_code[1].split('&').collect();
        let code = query_code[0];

        let request = Request::post("http://localhost:8000/auth")
            .header("Content-Type", "application/json")
            .body(Ok(String::from(code)))
            .expect("Failed to build request.");

        let callback = created.link.callback(|response: Response<Token>| {
            let data = response.into_body();
            Msg::ReceiveResponse(data)
        });

        let task = FetchService::fetch(request, callback).unwrap();

        created.fetch_task = Some(task);

        return created;
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            ReceiveResponse(response) => {
                let Token { token, status } = response;
                set_token(Some(token));
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
                self.fetch_task = None;
            }
            Ignore => {}
        }
        true
    }
    fn view(&self) -> Html {
        match self.token {
            Some(ref t) => {
                html! {
                    <>
                        <p>{ "URL is:" }</p>
                        <p>{ format!("URL: {}", t) }</p>
                    </>
                }
            }
            None => {
                html! {
                   <div class="app">
                <header class="app-header">

                        <p>{ "Redirecting..." }</p>
                        </header>
                        </div>
                }
            }
        }
    }
}
