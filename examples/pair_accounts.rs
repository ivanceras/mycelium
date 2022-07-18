//! Calling function from a custom pallet
#![allow(warnings)]
use async_recursion::async_recursion;
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use mycelium::sp_core::crypto::AccountId32;
use mycelium::{
    types::extrinsic_params::{PlainTip, PlainTipExtrinsicParams},
    Api,
};
use node_template_runtime::Runtime;
use pallet_forum::Comment;
use pallet_forum::Post;
use sp_core::crypto::Ss58Codec;
use sp_core::Pair;
use sp_keyring::AccountKeyring;
use std::{thread, time};

type MaxComments = <Runtime as pallet_forum::Config>::MaxComments;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();
    let bytes = from.to_raw_vec();
    println!("seed: {:?}", bytes);
    println!("seed len: {}", bytes.len());

    let hex_string = hex::encode(&bytes);
    println!("hex: {}", hex_string);
    let public = from.public();
    println!("public: {}", public);
    println!("public.0: {:?}", public.0);
    println!("public to string: {}", public.to_string());
    println!("ss58check: {}", public.to_ss58check());

    let derived: sp_core::sr25519::Pair = Pair::from_seed_slice(&bytes).unwrap();
    assert_eq!(from.to_raw_vec(), derived.to_raw_vec());
    println!("derived raw: {:?}", derived.to_raw_vec());
    println!("derived public: {:?}", derived.public());
    println!("derived public.0: {:?}", derived.public().0);
    Ok(())
}
