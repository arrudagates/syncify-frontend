use yew::prelude::*;

use anyhow;
use serde::Deserialize;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
    format::{Json, Nothing},
    prelude::*,
};

use yew::utils::window;
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

/// Login page
//pub struct Login;

#[derive(Deserialize, Debug, Clone)]
pub struct Url {
    status: String,
    url: String,
}

#[derive(Debug)]
pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<Url, anyhow::Error>),
}

#[derive(Debug)]
pub struct Login {
    fetch_task: Option<FetchTask>,
    url: Option<Url>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Login {
    fn view_iss_location(&self) -> Html {
        match self.url {
            Some(ref u) => {
                html! {
                    <>
                        <p>{ "URL is:" }</p>
                        <p>{ format!("URL: {}", u.url) }</p>
                    </>
                }
            }
            None => {
                html! {
                     <button onclick=self.link.callback(|_| Msg::GetLocation)>
                         { "Get URL" }
                     </button>
                }
            }
        }
    }
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            url: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            GetLocation => {
                // 1. build the request
                let request = Request::get("http://localhost:8000/get_url")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Url, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        window()
                            .location()
                            .replace(location.clone().url.as_str())
                            .expect("couldnt redirect");
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
        }
    }
    fn view(&self) -> Html {
        match self.url {
            Some(ref u) => {
                html! {
                   <div class="app">
                <header class="app-header">
                        <p>{ "URL is:" }</p>
                        <p>{ format!("URL: {}", u.url) }</p>
                    </header>
                    </div>
                }
            }
            None => {
                html! {
                   <div class="app">
                <header class="app-header">

                     <button onclick=self.link.callback(|_| Msg::GetLocation)>
                         { "Get URL" }
                    </button>
                        </header>
                        </div>
                }
            }
        }
    }
}
