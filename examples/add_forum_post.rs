//! Calling function from a custom pallet
#![allow(warnings)]
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use mycelium::sp_core::crypto::AccountId32;
use mycelium::{
    types::extrinsic_params::{PlainTip, PlainTipExtrinsicParams},
    Api,
};
use node_template_runtime::Runtime;
use pallet_forum::Post;
use sp_core::sr25519::Pair;
use sp_keyring::AccountKeyring;
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let api = Api::new("http://localhost:9933").await?;

    let pallet = api.metadata().pallet("ForumModule")?;
    let call_index = pallet
        .calls
        .get("post_content")
        .expect("unable to find function");

    let bounded_content = BoundedVec::try_from(b"Hello world post!".to_vec()).unwrap();

    let call = ([pallet.index, *call_index], bounded_content);
    let result = api.execute_extrinsic::<Pair, PlainTipExtrinsicParams, PlainTip,
            ([u8; 2], BoundedVec<u8, ConstU32<280>>),
            >(Some(from.clone()), call, None, None).await?;
    println!("result: {:?}", result);

    thread::sleep(time::Duration::from_millis(5_000));

    let current_item: Option<u32> = api
        .fetch_storage_value("ForumModule", "ItemCounter")
        .await?;

    println!("current item: {:?}", current_item);
    let current_item = current_item.unwrap();

    let prev_item = current_item.saturating_sub(1);

    let inserted_post: Option<Post<Runtime>> = api
        .fetch_storage_map("ForumModule", "AllPosts", prev_item)
        .await?;

    println!("inserted-post: {:#?}", inserted_post);
    if let Some(inserted_post) = inserted_post {
        let posted_content = String::from_utf8_lossy(&inserted_post.content);
        println!("posted content: {:?}", posted_content);
    }

    thread::sleep(time::Duration::from_millis(5_000));

    add_comment_to(
        &api,
        current_item,
        "This is a comment to Hello world!",
        from.clone(),
    )
    .await?;

    thread::sleep(time::Duration::from_millis(5_000));

    add_comment_to(
        &api,
        current_item,
        "This is a 2nd comment to the Hello world!",
        from,
    )
    .await?;

    thread::sleep(time::Duration::from_millis(5_000));

    let post_comments: Option<BoundedVec<u32, ConstU32<1000>>> = api
        .fetch_storage_map("ForumModule", "Kids", current_item)
        .await?;

    dbg!(post_comments);

    Ok(())
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
        .execute_extrinsic::<Pair, PlainTipExtrinsicParams, PlainTip, ([u8;2], u32, Option<u32>, BoundedVec<u8, ConstU32<280>>)>(
            Some(author),
            call,
            None,
            None,
        )
        .await?;

    println!("comment result: {:?}", result);
    Ok(())
}
