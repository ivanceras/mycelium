//! An example using an offline extrinsic, using the types of the instantiated chain
#![deny(warnings)]
use frame_support::BoundedVec;
use mycelium::{
    Api,
};
use node_template_runtime::{
    pallet_forum,
    Call,
    Header,
    Runtime,
};
use pallet_forum::Post;
use sp_keyring::AccountKeyring;
use sp_runtime::generic::Era;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let api = Api::new("http://localhost:9933").await?;


    let head_hash = api
        .chain_get_finalized_head()
        .await?
        .expect("must have a finalized head");
    let header: Header = api
        .chain_get_header(head_hash)
        .await?
        .expect("must have a header");

    let call: Call = Call::ForumModule(pallet_forum::Call::post_content {
        content: BoundedVec::try_from(b"Hello world post using Call!".to_vec())
            .unwrap(),
    });

    let period = 5;
    let era = Era::mortal(period, header.number.into());

    let extrinsic = api
        .sign_extrinsic_with_era(&from, call, Some(era), Some(head_hash), Some(10))
        .await?;

    let current_item: Option<u32> = api
        .fetch_storage_value("ForumModule", "ItemCounter")
        .await?;
    println!("current item: {:?}", current_item);

    let result = api.submit_extrinsic(extrinsic).await?;
    println!("result: {:?}", result);

    std::thread::sleep(std::time::Duration::from_millis(3000));
    let current_item = current_item.unwrap_or(0);

    let inserted_post: Option<Post<Runtime>> = api
        .fetch_storage_map("ForumModule", "AllPosts", current_item)
        .await?;
    println!("inserted-post: {:#?}", inserted_post);
    if let Some(inserted_post) = inserted_post {
        let posted_content = String::from_utf8_lossy(&inserted_post.content);
        println!("posted content: {:?}", posted_content);
    }
    Ok(())
}
