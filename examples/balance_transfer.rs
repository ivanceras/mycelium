//! This example transfer some amount from Alice to Charlie
#![allow(warnings)]
use codec::Compact;
use mycelium::{
    sp_core::crypto::AccountId32,
    types::{
        extrinsic_params::{
            PlainTip,
            PlainTipExtrinsicParams,
        },
        extrinsics::GenericAddress,
    },
    Api,
    Metadata,
};
use sp_core::Pair;
use sp_keyring::AccountKeyring;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();
    let alice_account_id = AccountKeyring::Alice.to_account_id();
    println!("raw vec: {:?}", from.to_raw_vec());

    let to: AccountId32 = AccountKeyring::Charlie.to_account_id();

    let alt_to = AccountId32::from_str(
        "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
    )
    .expect("must not error");

    assert_eq!(to, alt_to);

    println!("transfering balance from: {:?} to: {}", from.as_ref(), to);

    let api = Api::new("http://localhost:9933").await?;

    let info = api.get_account_info(&alice_account_id).await?;
    println!("account info: {:#?}", info);

    let result = api
        .balance_transfer(from, to, 42_000_000_000_000_u128, None)
        .await?;
    println!("result: {:?}", result);
    Ok(())
}
