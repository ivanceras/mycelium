//! Calling function from a custom pallet and using types that are redefinition to disconnect from
//! encoding
#![deny(warnings)]
use async_recursion::async_recursion;
use codec::{
    Decode,
    Encode,
};
use frame_support::{
    pallet_prelude::ConstU32,
    BoundedVec,
};
use mycelium::{
    sp_core::crypto::AccountId32,
    Api,
};
use sp_core::sr25519::Pair;
use sp_keyring::AccountKeyring;
use std::{
    thread,
    time,
};

type MaxComments = ConstU32<1000>;
type MaxContentLength = ConstU32<280>;

/// Note: Make sure that the fields are in the same order as in the original type stored in the
/// database, otherwise it will be unable to decode back to this user defined type
/// The name of the field doesn't matter, only the arragement of the field and the type to be the
/// same
#[derive(Encode, Decode, Debug)]
struct Post {
    post_id: u32,
    content: BoundedVec<u8, MaxContentLength>,
    author: AccountId32,
    timestamp: u64,
    block_number: u32,
}

#[derive(Encode, Decode, Debug)]
pub struct Comment {
    comment_id: u32,
    content: BoundedVec<u8, MaxContentLength>,
    author: AccountId32,
    parent_item: u32,
    timestamp: u64,
    block_number: u32,
}

#[derive(Encode, Decode, Debug)]
struct CommentDetails {
    comment: Comment,
    kids: Vec<CommentDetails>,
}

#[derive(Encode, Decode, Debug)]
struct PostDetails {
    post: Post,
    comments: Vec<CommentDetails>,
}

const DELAY: u64 = 1500; // in ms

fn sleep(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let alice: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();
    let bob: sp_core::sr25519::Pair = AccountKeyring::Bob.pair();

    let api = Api::new("http://localhost:9933").await?;

    let last_post_id = add_post(&api, "Hello world!1111", &alice).await?;

    sleep(DELAY);

    let inserted_post: Option<Vec<u8>> = api
        .fetch_opaque_storage_map("ForumModule", "AllPosts", last_post_id)
        .await?;

    println!("inserted-post: {:#?}", inserted_post);
    if let Some(inserted_post) = inserted_post {
        let inserted_post = Post::decode(&mut inserted_post.as_slice())?;
        let posted_content = String::from_utf8_lossy(&inserted_post.content);
        println!("posted content: {:?}", posted_content);
    }

    sleep(DELAY);

    let _item1 = add_comment_to(
        &api,
        last_post_id,
        "This is a comment to Hello world!",
        &alice,
    )
    .await?;

    sleep(DELAY);

    let _item2 = add_comment_to(
        &api,
        last_post_id,
        "This is a 2nd comment to the Hello world!",
        &bob,
    )
    .await?;
    println!("item2: {}", _item2);

    sleep(DELAY);

    let _item3 = add_comment_to(
        &api,
        _item2,
        "This is a comment to the 2nd comment to the Hello world post!",
        &alice,
    )
    .await?;

    sleep(DELAY);

    if let Some(post_comments) = api
        .fetch_opaque_storage_map("ForumModule", "Kids", last_post_id)
        .await?
    {
        let post_comments: BoundedVec<u32, MaxComments> =
            Decode::decode(&mut post_comments.as_slice())?;

        dbg!(post_comments);
    }

    sleep(DELAY);

    let post_details = get_post_details(&api, last_post_id).await?;
    dbg!(post_details);

    let all_posts = get_all_posts(&api).await?;
    dbg!(all_posts);

    let kids: Option<Vec<Vec<u8>>> = api
        .fetch_opaque_storage_map_paged("ForumModule", "Kids", 10, None::<u32>)
        .await?;
    dbg!(&kids);
    if let Some(kids) = kids {
        for (i, kid) in kids.into_iter().enumerate() {
            let kid: Result<BoundedVec<u32, MaxComments>, _> =
                Decode::decode(&mut kid.as_slice());
            println!("kid[{}]: {:?}", i, kid);
        }
    }

    Ok(())
}

async fn add_post(
    api: &Api,
    post: &str,
    author: &Pair,
) -> Result<u32, mycelium::Error> {
    let pallet = api.metadata().pallet("ForumModule")?;
    let call_index = pallet
        .calls
        .get("post_content")
        .expect("unable to find function");

    let bounded_content =
        BoundedVec::try_from(post.as_bytes().to_vec()).unwrap();
    let call: ([u8; 2], BoundedVec<u8, MaxContentLength>) =
        ([pallet.index, *call_index], bounded_content);

    let current_item = get_current_item(api).await?;

    let extrinsic = api.sign_extrinsic(author, call, None).await?;
    let result = api.submit_extrinsic(extrinsic).await?;
    println!("result: {:?}", result);
    Ok(current_item)
}

/// Warning this is an approximation value, since
/// there could another extrinsic call to the forum module to increment it while
/// this is executing in between the function calls in the following intended extrinsics
async fn get_current_item(api: &Api) -> Result<u32, mycelium::Error> {
    let current_item: Option<Vec<u8>> = api
        .fetch_opaque_storage_value("ForumModule", "ItemCounter")
        .await?;

    if let Some(current_item) = current_item {
        let current_item = Decode::decode(&mut current_item.as_slice())?;
        Ok(current_item)
    } else {
        println!("There is no current item yet..");
        eprintln!("There is no current item");
        Ok(0)
    }
}

async fn add_comment_to(
    api: &Api,
    parent_item: u32,
    comment: &str,
    author: &Pair,
) -> Result<u32, mycelium::Error> {
    let pallet = api.metadata().pallet("ForumModule")?;
    let call_index = pallet.calls.get("comment_on").unwrap();
    let bounded_comment =
        BoundedVec::try_from(comment.as_bytes().to_vec()).unwrap();
    let call: ([u8; 2], u32, BoundedVec<u8, MaxContentLength>) =
        ([pallet.index, *call_index], parent_item, bounded_comment);

    let current_item = get_current_item(api).await?;

    let extrinsic = api.sign_extrinsic(author, call, None).await?;
    let result = api.submit_extrinsic(extrinsic).await?;
    println!("comment result: {:?}", result);
    Ok(current_item)
}

async fn get_post_details(
    api: &Api,
    post_id: u32,
) -> Result<Option<PostDetails>, mycelium::Error> {
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

async fn get_all_posts(api: &Api) -> Result<Vec<Post>, mycelium::Error> {
    let mut all_post = Vec::with_capacity(10);
    println!("---->Getting all the post_id...");
    let next_to: Option<u32> = None;
    let storage_values: Option<Vec<Vec<u8>>> = api
        .fetch_opaque_storage_map_paged("ForumModule", "AllPosts", 10, next_to)
        .await?;
    if let Some(storage_values) = storage_values {
        for bytes in storage_values.into_iter() {
            let post: Post = Post::decode(&mut bytes.as_slice())?;
            all_post.push(post);
        }
    }
    Ok(all_post)
}

async fn get_post(
    api: &Api,
    post_id: u32,
) -> Result<Option<Post>, mycelium::Error> {
    if let Some(post) = api
        .fetch_opaque_storage_map("ForumModule", "AllPosts", post_id)
        .await?
    {
        let post: Post = Post::decode(&mut post.as_slice())?;
        Ok(Some(post))
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
        let kids: BoundedVec<u32, MaxComments> =
            Decode::decode(&mut kids.as_slice())?;
        println!("kids of item: {} are: {:?}", item_id, kids);
        Ok(Some(kids))
    } else {
        println!("There is no kid for {}", item_id);
        Ok(None)
    }
}

#[async_recursion]
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

async fn get_comment(
    api: &Api,
    comment_id: u32,
) -> Result<Option<Comment>, mycelium::Error> {
    println!("getting comment {}", comment_id);
    if let Some(comment) = api
        .fetch_opaque_storage_map("ForumModule", "AllComments", comment_id)
        .await?
    {
        let comment: Comment = Decode::decode(&mut comment.as_slice())?;
        Ok(Some(comment))
    } else {
        Ok(None)
    }
}
