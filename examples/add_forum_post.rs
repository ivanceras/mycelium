//! Calling function from a custom pallet
#![allow(warnings)]
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use mycelium::sp_core::crypto::AccountId32;
use mycelium::{
    types::extrinsic_params::{PlainTip, PlainTipExtrinsicParams},
    Api,
};
use sp_core::sr25519::Pair;
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let api = Api::new("http://localhost:9933").await?;

    let pallet = api.metadata().pallet("ForumModule")?;
    let call_index = pallet
        .calls
        .get("post_content")
        .expect("unable to find transfer function");

    let bounded_content = BoundedVec::try_from(b"Hello world post!".to_vec()).unwrap();

    let balance_call = ([pallet.index, *call_index], bounded_content);
    let xt = api.compose_extrinsics::<Pair, PlainTipExtrinsicParams, PlainTip,
            ([u8; 2], BoundedVec<u8, ConstU32<280>>),
            >(Some(from), balance_call, None, None).await?;
    let encoded = xt.hex_encode();
    let result = api.author_submit_extrinsic(&encoded).await;
    println!("result: {:?}", result);

    let current_item: Option<u32> = api
        .fetch_storage_value("ForumModule", "ItemCounter")
        .await?;
    println!("current item: {:?}", current_item);

    let prev_item = current_item.unwrap().saturating_sub(1);

    let inserted_post: Option<(BoundedVec<u8, ConstU32<280>>, AccountId32)> = api
        .fetch_storage_map("ForumModule", "Post", prev_item)
        .await?;
    println!("inserted-post: {:#?}", inserted_post);
    if let Some(inserted_post) = inserted_post {
        let posted_content = String::from_utf8_lossy(&inserted_post.0);
        println!("posted content: {:?}", posted_content);
    }
    Ok(())
}
