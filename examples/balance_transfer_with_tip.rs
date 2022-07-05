//! This example transfer some amount from Alice to Charlie
#![allow(warnings)]
use codec::Compact;
use mycelium::sp_core::crypto::AccountId32;
use mycelium::{
    types::{
        extrinsic_params::{PlainTip, PlainTipExtrinsicParams},
        extrinsics::GenericAddress,
    },
    Api, Metadata,
};
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let to: AccountId32 = AccountKeyring::Charlie.to_account_id();
    println!("transfering balance from: {:?} to: {}", from.as_ref(), to);

    let api = Api::new("http://localhost:9933").await?;
    let result = api
        .balance_transfer(from, to, 41_500_000_000_000_u128, Some(500_000_000_000))
        .await?;
    println!("result: {:?}", result);
    Ok(())
}
