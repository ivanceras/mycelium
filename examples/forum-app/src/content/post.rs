pub use super::comment::*;
use crate::util;
use crate::Msg;
use crate::*;
use codec::{Decode, Encode};
use frame_support::BoundedVec;
use mycelium::sp_core::crypto::AccountId32;
use std::borrow::Cow;

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

    pub fn view(&self) -> Node<Msg> {
        div(
            [class("post-detail")],
            [
                self.view_as_summary(),
                Content::view_submit_comment_form(ParentItem::Post(self.post_id())),
                ul(
                    [class("comment-details")],
                    self.comments
                        .iter()
                        .map(|comment| comment.view_recursively()),
                ),
            ],
        )
    }

    pub fn view_as_summary(&self) -> Node<Msg> {
        let post_id = self.post_id();
        div(
            [class("post-detail-summary")],
            [
                self.post.view(),
                div(
                    [class("post-detail-stats")],
                    [
                        a(
                            [
                                href(self.link()),
                                on_click(move |e| {
                                    e.prevent_default();
                                    Msg::ShowPost(post_id)
                                }),
                            ],
                            [text!("by: {}", self.author())],
                        ),
                        a(
                            [href(self.block_link())],
                            [text!("at: {}", self.block_number())],
                        ),
                        a(
                            [
                                href(self.link()),
                                on_click(move |e| {
                                    e.prevent_default();
                                    Msg::ShowPost(post_id)
                                }),
                            ],
                            [text!("{} ago", self.time_ago())],
                        ),
                        a(
                            [
                                href(self.link()),
                                on_click(move |e| {
                                    e.prevent_default();
                                    Msg::ShowPost(post_id)
                                }),
                            ],
                            [text!("{} comments", self.reply_count)],
                        ),
                    ],
                ),
            ],
        )
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

    pub fn view(&self) -> Node<Msg> {
        li(
            [class("post")],
            [h2([], [pre([class("post-text")], [text(self.content())])])],
        )
    }
}
