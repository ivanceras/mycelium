//! Calling function alice a custom pallet
#![allow(warnings)]
use async_recursion::async_recursion;
use frame_support::{
    pallet_prelude::ConstU32,
    BoundedVec,
};
use mycelium::{
    sp_core::crypto::AccountId32,
    types::extrinsic_params::{
        PlainTip,
        PlainTipExtrinsicParams,
    },
    Api,
};
use node_template_runtime::Runtime;
use pallet_forum::{
    Comment,
    Post,
};
use sp_core::{
    crypto::Ss58Codec,
    Pair,
};
use sp_keyring::AccountKeyring;
use std::{
    thread,
    time,
};

type MaxComments = <Runtime as pallet_forum::Config>::MaxComments;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let alice_account_id: AccountId32 = AccountKeyring::Alice.to_account_id();
    dbg!(&alice_account_id);
    println!(
        "alice_account_id.to_ss58check: {}",
        alice_account_id.to_ss58check()
    );

    let alice: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();
    let bytes = alice.to_raw_vec();
    println!("bytes: {:?}", bytes);
    println!("bytes len: {}", bytes.len());

    let hex_string = hex::encode(&bytes);
    println!("hex: {}", hex_string);
    let public = alice.public();
    println!("public: {}", public);
    println!("public.0: {:?}", public.0);
    println!("public to string: {}", public.to_string());

    assert_eq!(public.to_string(), alice_account_id.to_ss58check());

    println!("ss58check: {}", public.to_ss58check());

    let recover = AccountId32::from_ss58check(&public.to_ss58check())
        .expect("must not error");
    assert_eq!(recover, alice_account_id);

    let derived: sp_core::sr25519::Pair =
        Pair::from_seed_slice(&bytes).unwrap();
    assert_eq!(alice.to_raw_vec(), derived.to_raw_vec());
    println!("derived raw: {:?}", derived.to_raw_vec());
    println!("derived public: {:?}", derived.public());
    println!("derived public.0: {:?}", derived.public().0);
    Ok(())
}
