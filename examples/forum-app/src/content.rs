use crate::util;
use crate::Msg;
use codec::{Decode, Encode};
pub use comment::*;
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use mycelium::sp_core::crypto::AccountId32;
use sauron::html::attributes;
use sauron::prelude::*;
use std::borrow::Cow;

mod comment;

pub type MaxComments = ConstU32<1000>;
pub type MaxContentLength = ConstU32<280>;

#[derive(Debug, derive_more::From)]
pub enum Content {
    Posts(Vec<PostDetail>),
    PostDetail(PostDetail),
    Errored(crate::Error),
    SubmitPost,
    CommentDetail(CommentDetail),
}

#[derive(Debug, Clone, Copy)]
pub enum ParentItem {
    Comment(u32),
    Post(u32),
}

impl ParentItem {
    pub fn item_id(&self) -> u32 {
        match self {
            Self::Comment(comment_id) => *comment_id,
            Self::Post(post_id) => *post_id,
        }
    }
}

#[derive(Debug)]
pub struct PostDetail {
    pub post: Post,
    pub comments: Vec<CommentDetail>,
    pub reply_count: usize,
    pub block_hash: String,
}

#[derive(Encode, Decode, Debug)]
pub struct Post {
    pub post_id: u32,
    pub content: BoundedVec<u8, MaxContentLength>,
    pub author: AccountId32,
    pub timestamp: u64,
    pub block_number: u32,
}

impl PostDetail {
    fn link(&self) -> String {
        self.post.link()
    }
    fn post_id(&self) -> u32 {
        self.post.post_id
    }
    fn author(&self) -> String {
        self.post.author()
    }
    fn time_ago(&self) -> String {
        self.post.time_ago()
    }
    fn block_number(&self) -> u32 {
        self.post.block_number
    }

    fn block_link(&self) -> String {
        format!("{}/{}", crate::BLOCK_EXPLORER, self.block_hash)
    }
}

impl Post {
    pub fn content(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.content)
    }

    pub fn link(&self) -> String {
        format!("/item/{}", self.post_id)
    }
    fn author(&self) -> String {
        self.author.to_string()
    }
    fn time_ago(&self) -> String {
        util::timestamp_ago(self.timestamp)
    }
}

impl Content {
    pub fn view(&self) -> Node<Msg> {
        match self {
            Content::Posts(post_details) => self.view_post_detail_list(post_details),
            Content::PostDetail(post_detail) => self.view_post_detail(post_detail),
            Content::Errored(error) => self.view_error(error),
            Content::SubmitPost => self.view_submit_post(),
            Content::CommentDetail(comment_detail) => comment_detail.view(),
        }
    }

    fn view_error(&self, error: &crate::Error) -> Node<Msg> {
        div(
            [class("error")],
            [text!("Something went wrong: {:#?}", error)],
        )
    }

    fn view_post_detail_list(&self, post_details: &[PostDetail]) -> Node<Msg> {
        div(
            [class("post-details-list")],
            [if post_details.is_empty() {
                div([class("empty-posts")], [text("There are no posts yet!")])
            } else {
                ol(
                    [class("post-details")],
                    post_details
                        .into_iter()
                        .rev()
                        .map(|post| self.view_post_detail_summary(post)),
                )
            }],
        )
    }

    fn view_submit_post(&self) -> Node<Msg> {
        form(
            [
                class("post-new"),
                attributes::method("post"),
                action("submit-post"),
            ],
            [div(
                [class("controls")],
                [
                    textarea(
                        [
                            class("post-new-content"),
                            on_change(|e| Msg::ChangePost(e.value)),
                        ],
                        [],
                    ),
                    br([], []),
                    input(
                        [
                            r#type("submit"),
                            value("submit"),
                            on_click(move |e| {
                                e.prevent_default();
                                Msg::SubmitPost
                            }),
                        ],
                        [],
                    ),
                ],
            )],
        )
    }

    fn view_submit_comment_form(&self, parent_item: ParentItem) -> Node<Msg> {
        form(
            [
                class("comment-new"),
                attributes::method("post"),
                action("submit-comment"),
            ],
            [div(
                [class("controls")],
                [
                    textarea(
                        [
                            class("comment-new-content"),
                            on_change(|e| Msg::ChangeComment(e.value)),
                        ],
                        [],
                    ),
                    br([], []),
                    input(
                        [
                            r#type("submit"),
                            value("add comment"),
                            on_click(move |e| {
                                e.prevent_default();
                                Msg::SubmitComment(parent_item)
                            }),
                        ],
                        [],
                    ),
                ],
            )],
        )
    }

    fn view_post_detail(&self, post_detail: &PostDetail) -> Node<Msg> {
        div(
            [class("post-detail")],
            [
                self.view_post_detail_summary(post_detail),
                self.view_submit_comment_form(ParentItem::Post(post_detail.post_id())),
                ul(
                    [class("comment-details")],
                    post_detail
                        .comments
                        .iter()
                        .map(|comment| comment.view_as_list()),
                ),
            ],
        )
    }

    fn view_post_detail_summary(&self, post_detail: &PostDetail) -> Node<Msg> {
        let post_id = post_detail.post_id();
        div(
            [class("post-detail-summary")],
            [
                self.view_post(&post_detail.post),
                div(
                    [class("post-detail-stats")],
                    [
                        a(
                            [
                                href(post_detail.link()),
                                on_click(move |e| {
                                    e.prevent_default();
                                    Msg::ShowPost(post_id)
                                }),
                            ],
                            [text!("by: {}", post_detail.author())],
                        ),
                        a(
                            [href(post_detail.block_link())],
                            [text!("at: {}", post_detail.block_number())],
                        ),
                        a(
                            [
                                href(post_detail.link()),
                                on_click(move |e| {
                                    e.prevent_default();
                                    Msg::ShowPost(post_id)
                                }),
                            ],
                            [text!("{} ago", post_detail.time_ago())],
                        ),
                        a(
                            [
                                href(post_detail.link()),
                                on_click(move |e| {
                                    e.prevent_default();
                                    Msg::ShowPost(post_id)
                                }),
                            ],
                            [text!("{} comments", post_detail.reply_count)],
                        ),
                    ],
                ),
            ],
        )
    }

    fn view_post(&self, post: &Post) -> Node<Msg> {
        li(
            [class("post")],
            [h2([], [pre([class("post-text")], [text(post.content())])])],
        )
    }
}
