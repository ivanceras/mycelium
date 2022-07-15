use content::Content;
use mycelium::Api;
use sauron::prelude::*;
use types::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod content;
mod fetch;
mod types;

#[derive(Debug, Clone)]
enum FetchStatus<T> {
    Idle,
    Complete(T),
}

enum Msg {
    FetchPosts,
    ShowPost(u32),
    PostsReceived(Vec<Post>),
    PostDetailsReceived(PostDetails),
    Errored(Error),
    InitApi(Api),
}

enum Error {
    RequestError(String),
    ApiError(String),
    Error404(u32),
}

struct App {
    content: FetchStatus<Content>,
    api: Option<Api>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            content: FetchStatus::Idle,
            api: None,
        }
    }
}

impl App {
    fn init_api(&self) -> Cmd<Self, Msg> {
        log::info!("initializing api..");
        Cmd::new(move |program| {
            let async_fetch = |program: Program<Self, Msg>| async move {
                match Api::new("http://localhost:9933").await {
                    Ok(api) => {
                        log::info!("got some api..");
                        program.dispatch(Msg::InitApi(api));
                    }
                    Err(e) => {
                        program.dispatch(Msg::Errored(Error::ApiError(e.to_string())));
                    }
                }
            };
            spawn_local(async_fetch(program))
        })
    }

    /// run to display the content
    fn fetch_content(&self) -> Cmd<Self, Msg> {
        if let FetchStatus::Idle = self.content {
            self.fetch_posts()
        } else {
            Cmd::none()
        }
    }

    fn fetch_posts(&self) -> Cmd<Self, Msg> {
        log::warn!("fetching posts..");
        let api = self.api.clone();
        Cmd::new(move |program| {
            let async_fetch = |program: Program<Self, Msg>| async move {
                let api = api.unwrap();
                match fetch::get_all_posts(&api).await {
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
            FetchStatus::Idle => p([], [text("Waiting around...")]),
            FetchStatus::Complete(content) => content.view(),
        }
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        log::info!("Initializing app...");
        if self.api.is_none() {
            self.init_api()
        } else {
            Cmd::none()
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::InitApi(api) => {
                self.api = Some(api);
                self.fetch_content()
            }
            Msg::PostsReceived(posts) => {
                log::debug!("posts: {:#?}", posts);
                self.content = FetchStatus::Complete(Content::from(posts));
                Cmd::none()
            }
            Msg::ShowPost(post_id) => self.fetch_post_details(post_id),
            Msg::PostDetailsReceived(post_detail) => {
                self.content = FetchStatus::Complete(Content::from(post_detail));
                Cmd::none()
            }
            _ => Cmd::none(),
        }
    }

    fn view(&self) -> Node<Msg> {
        div([], [text("hello polywrap"), self.view_content()])
    }
}

async fn start() -> anyhow::Result<()> {
    let api = Api::new("http://localhost:9933").await?;
    log::info!("Getting all the posts..");
    let posts = fetch::get_all_posts(&api).await?;
    log::info!("posts: {:#?}", posts);
    for (n, post) in posts.iter().enumerate() {
        log::info!("post[{}]:{}", n, post.content());
        let replies = fetch::get_comment_replies(&api, post.post_id).await?;
        for (c, comment) in replies.iter().enumerate() {
            log::info!("\tcomment[{}]: {}", c, comment.content());
        }
    }
    log::info!("Done..");
    Ok(())
}
#[wasm_bindgen(start)]
pub async fn main() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();
    log::info!("Starting");
    Program::mount_to_body(App::default());
}
