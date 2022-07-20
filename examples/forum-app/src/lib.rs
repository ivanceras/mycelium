use content::*;
use mycelium::Api;
use sauron::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

const URL: &str = "http://localhost:9933";
const BLOCK_EXPLORER: &str = "https://polkadot.js.org/apps/#/explorer/query";

mod content;
mod fetch;
mod util;

enum Msg {
    FetchPosts,
    ShowPost(u32),
    PostsReceived(Vec<PostDetail>),
    PostDetailsReceived(PostDetail),
    Errored(Error),
    InitApi(Api),
    UrlChanged(String),
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Http Request Error: {0}")]
    RequestError(String),
    #[error("Initialization of substrate API failed: {0}")]
    ApiInitializationError(String),
    #[error("Item can not be found on the server: {0}")]
    Error404(u32),
    #[error("mycelium Api Error: {0}")]
    MyCeliumError(#[from] mycelium::Error),
}

struct App {
    content: Option<Content>,
    api: Option<Api>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            content: None,
            api: None,
        }
    }
}

impl App {
    fn init_api(&self) -> Cmd<Self, Msg> {
        log::info!("initializing api..");
        Cmd::new(move |program| {
            let async_fetch = |program: Program<Self, Msg>| async move {
                match Api::new(URL).await {
                    Ok(api) => {
                        log::info!("got some api..");
                        program.dispatch(Msg::InitApi(api));
                    }
                    Err(e) => {
                        program
                            .dispatch(Msg::Errored(Error::ApiInitializationError(e.to_string())));
                    }
                }
            };
            spawn_local(async_fetch(program))
        })
    }

    fn fetch_posts(&self) -> Cmd<Self, Msg> {
        log::warn!("fetching posts..");
        let api = self.api.clone();
        Cmd::new(move |program| {
            let async_fetch = |program: Program<Self, Msg>| async move {
                let api = api.unwrap();
                match fetch::get_post_list(&api).await {
                    Ok(posts) => {
                        log::info!("Go some posts..: {:?}", posts);
                        program.dispatch(Msg::PostsReceived(posts));
                    }
                    Err(e) => {
                        log::error!("Something is wrong when fetching: {}", e.to_string());
                        program.dispatch(Msg::Errored(Error::RequestError(e.to_string())));
                    }
                }
            };
            spawn_local(async_fetch(program))
        })
    }

    fn fetch_post_details(&self, post_id: u32) -> Cmd<Self, Msg> {
        log::warn!("fetching posts..");
        let api = self.api.clone();
        Cmd::new(move |program| {
            let async_fetch = |program: Program<Self, Msg>| async move {
                let api = api.unwrap();
                match fetch::get_post_details(&api, post_id).await {
                    Ok(post_detail) => {
                        if let Some(post_detail) = post_detail {
                            program.dispatch(Msg::PostDetailsReceived(post_detail));
                        } else {
                            program.dispatch(Msg::Errored(Error::Error404(post_id)))
                        }
                    }
                    Err(e) => {
                        log::error!("Something is wrong when fetching: {}", e.to_string());
                        program.dispatch(Msg::Errored(Error::RequestError(e.to_string())));
                    }
                }
            };
            spawn_local(async_fetch(program))
        })
    }

    fn view_content(&self) -> Node<Msg> {
        match &self.content {
            Some(content) => content.view(),
            None => p([], [text("Waiting around...")]),
        }
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        let mut cmd = Window::add_event_listeners(vec![on_popstate(|_e| {
            log::trace!("pop_state is triggered in sauron add event listener");
            let url = sauron::window()
                .location()
                .pathname()
                .expect("must have get a pathname");
            Msg::UrlChanged(url)
        })]);

        log::info!("Initializing app...");

        if self.api.is_none() {
            cmd.push(self.init_api());
        }
        cmd
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::InitApi(api) => {
                self.api = Some(api);
                self.fetch_posts()
            }
            Msg::FetchPosts => self.fetch_posts(),
            Msg::PostsReceived(posts) => {
                log::debug!("posts: {:#?}", posts);
                self.content = Some(Content::from(posts));
                Cmd::none()
            }
            Msg::ShowPost(post_id) => self.fetch_post_details(post_id),
            Msg::PostDetailsReceived(post_detail) => {
                self.content = Some(Content::from(post_detail));
                Cmd::none()
            }
            Msg::UrlChanged(url) => Cmd::none(),
            Msg::Errored(error) => {
                self.content = Some(Content::from(error));
                Cmd::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        main(
            [],
            [
                header(
                    [],
                    [a(
                        [on_click(|e| {
                            e.prevent_default();
                            Msg::FetchPosts
                        })],
                        [div([class("logo")], [text("Y")])],
                    )],
                ),
                self.view_content(),
            ],
        )
    }
}

#[wasm_bindgen(start)]
pub async fn startup() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();
    log::info!("Starting");
    Program::mount_to_body(App::default());
}
