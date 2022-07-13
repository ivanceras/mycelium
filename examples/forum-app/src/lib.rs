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
use wasm_bindgen::prelude::*;

type MaxComments = ConstU32<1000>;
type MaxContentLength = ConstU32<280>;

#[derive(Encode, Decode, Debug)]
struct Post {
    post_id: u32,
    content: BoundedVec<u8, MaxContentLength>,
    author: AccountId32,
}

#[derive(Encode, Decode, Debug)]
pub struct Comment {
    comment_id: u32,
    content: BoundedVec<u8, MaxContentLength>,
    author: AccountId32,
    parent_item: Option<u32>,
}

#[derive(Debug)]
struct CommentDetails {
    comment: Comment,
    kids: Vec<CommentDetails>,
}

#[derive(Debug)]
struct PostDetails {
    post: Post,
    comments: Vec<CommentDetails>,
}

async fn add_comment_to(
    api: &Api,
    post_id: u32,
    comment: &str,
    author: Pair,
) -> Result<(), mycelium::Error> {
    let pallet = api.metadata().pallet("ForumModule")?;
    let call_index = pallet.calls.get("comment_on").unwrap();
    let bounded_comment = BoundedVec::try_from(comment.as_bytes().to_vec()).unwrap();
    let call = ([pallet.index, *call_index], post_id, None, bounded_comment);
    let result = api
        .execute_extrinsic::<Pair, PlainTipExtrinsicParams, PlainTip, ([u8;2], u32, Option<u32>, BoundedVec<u8, MaxContentLength>)>(
            Some(author),
            call,
            None,
            None,
        )
        .await?;

    println!("comment result: {:?}", result);
    Ok(())
}

async fn get_post_details(api: &Api, post_id: u32) -> Result<Option<PostDetails>, mycelium::Error> {
    println!("getting the post details of {}", post_id);
    let post = get_post(api, post_id).await?;
    if let Some(post) = post {
        let comment_replies = get_comment_replies(api, post_id).await?;
        Ok(Some(PostDetails {
            post,
            comments: comment_replies,
        }))
    } else {
        Ok(None)
    }
}

async fn get_post(api: &Api, post_id: u32) -> Result<Option<Post>, mycelium::Error> {
    if let Some(post) = api
        .fetch_opaque_storage_map("ForumModule", "AllPosts", post_id)
        .await?
    {
        let post: Option<Post> = Post::decode(&mut post.as_slice()).ok();
        Ok(post)
    } else {
        Ok(None)
    }
}

async fn get_kids(
    api: &Api,
    item_id: u32,
) -> Result<Option<BoundedVec<u32, MaxComments>>, mycelium::Error> {
    if let Some(kids) = api
        .fetch_opaque_storage_map("ForumModule", "Kids", item_id)
        .await?
    {
        let kids: Option<BoundedVec<u32, MaxComments>> = Decode::decode(&mut kids.as_slice()).ok();
        Ok(kids)
    } else {
        Ok(None)
    }
}

#[async_recursion(?Send)]
async fn get_comment_replies(
    api: &Api,
    item_id: u32,
) -> Result<Vec<CommentDetails>, mycelium::Error> {
    println!("getting comment replies for: {}", item_id);
    let mut comment_details = vec![];
    if let Some(kids) = get_kids(api, item_id).await? {
        for kid in kids {
            let comment = get_comment(api, kid)
                .await?
                .expect("must have a comment entry");

            let kid_comments = get_comment_replies(api, kid).await?;
            comment_details.push(CommentDetails {
                comment,
                kids: kid_comments,
            });
        }
    }
    Ok(comment_details)
}

async fn get_comment(api: &Api, comment_id: u32) -> Result<Option<Comment>, mycelium::Error> {
    println!("getting comment {}", comment_id);
    if let Some(comment) = api
        .fetch_opaque_storage_map("ForumModule", "AllComments", comment_id)
        .await?
    {
        let comment: Option<Comment> = Decode::decode(&mut comment.as_slice()).ok();
        Ok(comment)
    } else {
        Ok(None)
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).ok();
    console_error_panic_hook::set_once();
    log::info!("Starting");
}
