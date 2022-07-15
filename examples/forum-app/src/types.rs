use async_recursion::async_recursion;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use mycelium::sp_core::crypto::AccountId32;
use mycelium::sp_core::sr25519::Pair;
use mycelium::{
    types::extrinsic_params::{PlainTip, PlainTipExtrinsicParams},
    Api,
};
use std::borrow::Cow;
use wasm_bindgen::prelude::*;

pub type MaxComments = ConstU32<1000>;
pub type MaxContentLength = ConstU32<280>;

#[derive(Encode, Decode, Debug)]
pub struct Post {
    pub post_id: u32,
    pub content: BoundedVec<u8, MaxContentLength>,
    pub author: AccountId32,
}

impl Post {
    pub fn content(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.content)
    }
}

#[derive(Encode, Decode, Debug)]
pub struct Comment {
    pub comment_id: u32,
    pub content: BoundedVec<u8, MaxContentLength>,
    pub author: AccountId32,
    pub parent_item: Option<u32>,
}
impl Comment {
    pub fn content(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.content)
    }
}

#[derive(Debug)]
pub struct CommentDetails {
    pub comment: Comment,
    pub kids: Vec<CommentDetails>,
}

impl CommentDetails {
    pub fn content(&self) -> Cow<'_, str> {
        self.comment.content()
    }
}

#[derive(Debug)]
pub struct PostDetails {
    pub post: Post,
    pub comments: Vec<CommentDetails>,
}
