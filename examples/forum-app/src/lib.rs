#![allow(warnings)]
use content::*;
use mycelium::Api;
use sauron::prelude::*;
use wasm_bindgen_futures::spawn_local;

const URL: &str = "http://localhost:9933";
const BLOCK_EXPLORER: &str = "https://polkadot.js.org/apps/#/explorer/query";

mod content;
mod fetch;
mod util;

pub enum Msg {
    /// Ask the program to fetch a list of post summary
    FetchPosts,
    /// Ask the program to Show post with `post_id`
    ShowPost(u32),
    /// The program received a list of PostDetail summary
    PostsReceived(Vec<PostDetail>),
    /// The program receives the requested post detail
    PostDetailsReceived(PostDetail),
    /// The program received the requested comment detail
    CommentDetailReceived(CommentDetail),
    /// Where there is an error encountered in the program
    Errored(Error),
    /// Initiating the Api
    InitApi(Api),
    UrlChanged(String),
    /// The user clicks on the `submit` button, the program will then show the form for submitting
    /// a new post
    ShowSubmitForm,
    ShowReplyToCommentForm(u32),
    /// The comment content is changed, triggered when the user starts typing on the comment box
    ChangeComment(String),
    /// The post content is changed, triggered when the user starts typing on the post content box
    ChangePost(String),
    /// The user clicks on the `add comment` button.
    /// The program will use `ParentItem` and the `new_comment` field of App and use it as input for the comment
    /// reply
    SubmitComment(ParentItem),
    /// The user clicks on the `submit post` button.
    /// The program will use the `new_post` field of App and use it as input for new post
    /// submission
    SubmitPost,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
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
    new_comment: Option<String>,
    new_post: Option<String>,
    api: Option<Api>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            content: None,
            new_comment: None,
            new_post: None,
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

    fn fetch_comment_details(&self, comment_id: u32) -> Cmd<Self, Msg> {
        log::warn!("fetching comment details for comment_id: {}", comment_id);
        let api = self.api.clone();
        Cmd::new(move |program| {
            let async_fetch = |program: Program<Self, Msg>| async move {
                let api = api.unwrap();
                match fetch::get_comment_detail(&api, comment_id).await {
                    Ok(comment_detail) => {
                        if let Some(comment_detail) = comment_detail {
                            program.dispatch(Msg::CommentDetailReceived(comment_detail));
                        } else {
                            program.dispatch(Msg::Errored(Error::Error404(comment_id)))
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
            Some(content) => div([class("content")], [content.view()]),
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
            Msg::ShowSubmitForm => {
                self.content = Some(Content::SubmitPost);
                Cmd::none()
            }
            Msg::ShowReplyToCommentForm(parent_item) => self.fetch_comment_details(parent_item),
            Msg::PostDetailsReceived(post_detail) => {
                self.content = Some(Content::from(post_detail));
                Cmd::none()
            }
            Msg::CommentDetailReceived(comment_detail) => {
                self.content = Some(Content::CommentDetail(comment_detail));
                Cmd::none()
            }
            Msg::UrlChanged(_url) => Cmd::none(),
            Msg::Errored(error) => {
                self.content = Some(Content::from(error));
                Cmd::none()
            }
            Msg::ChangeComment(comment) => {
                log::info!("got a new comment: {}", comment);
                self.new_comment = Some(comment);
                Cmd::none()
            }
            Msg::ChangePost(post) => {
                self.new_post = Some(post);
                Cmd::none()
            }
            Msg::SubmitComment(parent) => {
                let parent_item = parent.item_id();
                let new_comment: &str = &self.new_comment.as_ref().expect("must have a comment");
                log::info!("comment to :{} with:\n{}", parent_item, new_comment);
                Cmd::none()
            }
            Msg::SubmitPost => {
                let new_post: &str = self.new_post.as_ref().expect("must have a new post");
                log::info!("A new post submission: \n{}", new_post);
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
                    [
                        a(
                            [on_click(|e| {
                                e.prevent_default();
                                Msg::FetchPosts
                            })],
                            [div([class("logo")], [text("Y")])],
                        ),
                        a(
                            [on_click(|e| {
                                e.prevent_default();
                                Msg::ShowSubmitForm
                            })],
                            [text("submit")],
                        ),
                    ],
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
