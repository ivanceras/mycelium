use crate::util;
use crate::Msg;
use codec::{Decode, Encode};
pub use comment::*;
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use mycelium::sp_core::crypto::AccountId32;
pub use post::*;
use sauron::html::attributes;
use sauron::prelude::*;
use std::borrow::Cow;

pub mod comment;
pub mod post;

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

impl Content {
    pub fn view(&self) -> Node<Msg> {
        match self {
            Content::Posts(post_details) => self.view_post_detail_list(post_details),
            Content::PostDetail(post_detail) => post_detail.view(),
            Content::Errored(error) => self.view_error(error),
            Content::SubmitPost => Self::view_submit_post(),
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
                        .map(|post| post.view_as_summary()),
                )
            }],
        )
    }

    fn view_submit_post() -> Node<Msg> {
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

    pub fn view_submit_comment_form(parent_item: ParentItem) -> Node<Msg> {
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
}
