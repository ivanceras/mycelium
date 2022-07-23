use crate::content::*;
use crate::Error;
use async_recursion::async_recursion;
use codec::Compact;
use codec::Decode;
use frame_support::traits::Get;
use frame_support::BoundedVec;
use mycelium::sp_core::crypto::AccountId32;
use mycelium::sp_core::H256;
use mycelium::types::extrinsics::GenericAddress;
use mycelium::Api;

const FORUM_MODULE: &str = "ForumModule";
const ALL_POSTS: &str = "AllPosts";
const ALL_COMMENTS: &str = "AllComments";
const KIDS: &str = "Kids";

pub async fn get_post_list(api: &Api) -> Result<Vec<PostDetail>, Error> {
    let mut all_post = Vec::with_capacity(10);
    log::info!("---->Getting all the post_id...");
    let next_to: Option<u32> = None;
    let storage_values: Option<Vec<Vec<u8>>> = api
        .fetch_opaque_storage_map_paged(FORUM_MODULE, ALL_POSTS, 10, next_to)
        .await?;
    log::info!("here..");
    if let Some(storage_values) = storage_values {
        for (i, bytes) in storage_values.into_iter().enumerate() {
            log::info!("At post: {}", i);
            let post: Option<Post> = Post::decode(&mut bytes.as_slice()).ok();
            if let Some(post) = post {
                let reply_count = get_reply_count(api, post.post_id).await?;
                let block_hash = get_block_hash(api, post.block_number)
                    .await?
                    .expect("must have a block hash");
                all_post.push(PostDetail {
                    post,
                    reply_count,
                    comments: vec![],
                    block_hash,
                });
            }
        }
    }
    log::info!("done get_post_list..: {:#?}", all_post);
    all_post.sort_unstable_by_key(|item| item.post.post_id);
    Ok(all_post)
}

pub async fn get_reply_count(api: &Api, post_id: u32) -> Result<usize, Error> {
    let reply_count = get_kids(api, post_id)
        .await?
        .map(|kids| kids.len())
        .unwrap_or(0);
    Ok(reply_count)
}

pub async fn get_post_details(api: &Api, post_id: u32) -> Result<Option<PostDetail>, Error> {
    log::info!("getting the post details of {}", post_id);
    let post = get_post(api, post_id).await?;
    if let Some(post) = post {
        let comment_replies = get_comment_replies(api, post_id).await?;
        let reply_count = get_reply_count(api, post_id).await?;
        let block_hash = get_block_hash(api, post.block_number)
            .await?
            .expect("must have a block hash");
        Ok(Some(PostDetail {
            post,
            comments: comment_replies,
            reply_count,
            block_hash,
        }))
    } else {
        Ok(None)
    }
}

pub async fn get_post(api: &Api, post_id: u32) -> Result<Option<Post>, Error> {
    if let Some(post) = api
        .fetch_opaque_storage_map(FORUM_MODULE, ALL_POSTS, post_id)
        .await?
    {
        let post: Option<Post> = Post::decode(&mut post.as_slice()).ok();
        Ok(post)
    } else {
        Ok(None)
    }
}

async fn get_kids(api: &Api, item_id: u32) -> Result<Option<BoundedVec<u32, MaxComments>>, Error> {
    if let Some(kids) = api
        .fetch_opaque_storage_map(FORUM_MODULE, KIDS, item_id)
        .await?
    {
        let kids: Option<BoundedVec<u32, MaxComments>> = Decode::decode(&mut kids.as_slice()).ok();
        Ok(kids)
    } else {
        Ok(None)
    }
}

#[async_recursion(?Send)]
pub async fn get_comment_replies(api: &Api, item_id: u32) -> Result<Vec<CommentDetail>, Error> {
    let mut comment_details = vec![];
    if let Some(kids) = get_kids(api, item_id).await? {
        log::info!("kids of item_id: {} are: {:?}", item_id, kids);
        for kid in kids {
            log::info!("getting comment: {}", kid);
            if let Some(comment_detail) = get_comment_detail(api, kid).await? {
                comment_details.push(comment_detail);
            }
        }
    }
    comment_details.sort_unstable_by_key(|item| item.comment_id());
    Ok(comment_details)
}

pub async fn get_comment_detail(
    api: &Api,
    comment_id: u32,
) -> Result<Option<CommentDetail>, Error> {
    if let Some(comment) = get_comment(api, comment_id).await? {
        let kid_comments = get_comment_replies(api, comment_id).await?;
        let block_hash = get_block_hash(api, comment.block_number)
            .await?
            .expect("must have a block hash");

        Ok(Some(CommentDetail {
            comment,
            kids: kid_comments,
            block_hash,
        }))
    } else {
        Ok(None)
    }
}

pub async fn get_comment(api: &Api, comment_id: u32) -> Result<Option<Comment>, Error> {
    log::debug!("getting comment_id: {}", comment_id);
    if let Some(comment) = api
        .fetch_opaque_storage_map(FORUM_MODULE, ALL_COMMENTS, comment_id)
        .await?
    {
        let comment: Option<Comment> = Decode::decode(&mut comment.as_slice()).ok();
        log::info!("got comment: {:?}", comment);
        Ok(comment)
    } else {
        Ok(None)
    }
}

pub async fn get_block_hash(api: &Api, block_number: u32) -> Result<Option<String>, Error> {
    let block_hash = api.fetch_block_hash(block_number).await?;
    Ok(block_hash.map(|hash| format!("{:#x}", hash)))
}

pub async fn add_post(api: &Api, post: &str) -> Result<Option<H256>, Error> {
    let bounded_content = BoundedVec::try_from(post.as_bytes().to_vec())
        .or_else(|_e| Err(Error::ContentTooLong(post.len(), MaxContentLength::get())))?;

    let pallet_call = api.pallet_call_index(FORUM_MODULE, "post_content")?;
    let call: ([u8; 2], BoundedVec<u8, MaxContentLength>) = (pallet_call, bounded_content);

    let extrinsic = crate::sign_call(api, call).await?;
    log::info!("added a post..");
    let tx_hash = api.submit_extrinsic(extrinsic).await?;

    Ok(tx_hash)
}

pub async fn add_comment(
    api: &Api,
    parent_item: u32,
    comment: &str,
) -> Result<Option<H256>, Error> {
    let bounded_content = BoundedVec::try_from(comment.as_bytes().to_vec()).or_else(|_e| {
        Err(Error::ContentTooLong(
            comment.len(),
            MaxContentLength::get(),
        ))
    })?;

    let pallet_call = api.pallet_call_index(FORUM_MODULE, "comment_on")?;
    let call: ([u8; 2], u32, BoundedVec<u8, MaxContentLength>) =
        (pallet_call, parent_item, bounded_content);

    let extrinsic = crate::sign_call(api, call).await?;
    log::debug!("Added a comment to parent_item: {}", parent_item);
    let tx_hash = api.submit_extrinsic(extrinsic).await?;

    Ok(tx_hash)
}

/// send some certain amount to this user
#[allow(unused)]
pub async fn send_token(api: &Api, to: AccountId32, amount: u128) -> Result<Option<H256>, Error> {
    let balance_transfer_call_index: [u8; 2] = api.pallet_call_index("Balances", "transfer")?;
    let balance_transfer_call: ([u8; 2], GenericAddress, Compact<u128>) = (
        balance_transfer_call_index,
        GenericAddress::Id(to),
        Compact(amount),
    );

    let extrinsic = crate::sign_call(api, balance_transfer_call).await?;
    let tx_hash = api.submit_extrinsic(extrinsic).await?;
    log::debug!("Sent some coins to with a tx_hash: {:?}", tx_hash);
    Ok(tx_hash)
}
