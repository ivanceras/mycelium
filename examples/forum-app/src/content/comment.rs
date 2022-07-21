use crate::util;
use crate::Msg;
use crate::*;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use mycelium::sp_core::crypto::AccountId32;
use sauron::html::attributes;
use sauron::prelude::*;
use std::borrow::Cow;

#[derive(Debug)]
pub struct CommentDetail {
    pub comment: Comment,
    pub kids: Vec<CommentDetail>,
    pub block_hash: String,
}

#[derive(Encode, Decode, Debug)]
pub struct Comment {
    pub comment_id: u32,
    pub content: BoundedVec<u8, MaxContentLength>,
    pub author: AccountId32,
    pub parent_item: u32,
    pub timestamp: u64,
    pub block_number: u32,
}

impl CommentDetail {
    pub fn content(&self) -> Cow<'_, str> {
        self.comment.content()
    }
    pub fn comment_id(&self) -> u32 {
        self.comment.comment_id
    }

    pub fn block_link(&self) -> String {
        format!("{}/{}", crate::BLOCK_EXPLORER, self.block_hash)
    }
    pub fn author(&self) -> String {
        self.comment.author()
    }
    pub fn time_ago(&self) -> String {
        self.comment.time_ago()
    }
    pub fn block_number(&self) -> u32 {
        self.comment.block_number
    }

    pub fn view(&self) -> Node<Msg> {
        li(
            [class("comment-detail")],
            [
                self.view_as_summary(),
                Content::view_submit_comment_form(ParentItem::Comment(self.comment_id())),
                ul(
                    [],
                    self.kids.iter().map(|comment| comment.view_recursively()),
                ),
            ],
        )
    }

    pub fn view_recursively(&self) -> Node<Msg> {
        li(
            [class("comment-detail-list")],
            [
                self.view_as_summary(),
                ul(
                    [],
                    self.kids.iter().map(|comment| comment.view_recursively()),
                ),
            ],
        )
    }

    pub fn view_as_summary(&self) -> Node<Msg> {
        let comment_id = self.comment_id();
        div(
            [class("comment-detail-summary")],
            [
                self.comment.view(),
                div(
                    [class("comment-stats")],
                    [
                        a([], [text!("by: {}", self.author())]),
                        a(
                            [href(self.block_link())],
                            [text!("at: {}", self.block_number())],
                        ),
                        a([], [text!("{} ago", self.time_ago())]),
                    ],
                ),
                a(
                    [
                        class("comment-reply-btn"),
                        on_click(move |e| {
                            e.prevent_default();
                            Msg::ShowReplyToCommentForm(comment_id)
                        }),
                    ],
                    [text("reply")],
                ),
            ],
        )
    }
}

impl Comment {
    pub fn content(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.content)
    }
    pub fn author(&self) -> String {
        self.author.to_string()
    }
    pub fn time_ago(&self) -> String {
        util::timestamp_ago(self.timestamp)
    }

    fn view(&self) -> Node<Msg> {
        li(
            [class("comment")],
            [pre([class("comment-text")], [text(self.content())])],
        )
    }
}
