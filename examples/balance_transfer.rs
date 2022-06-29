//! This example transfer some amount from Alice to Charlie
#![allow(warnings)]
use codec::Compact;
use mycelium::{
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
use sp_keyring::AccountKeyring;
use mycelium::sp_core::crypto::AccountId32;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let to: AccountId32= AccountKeyring::Charlie.to_account_id();
    println!("transfering balance from: {:?} to: {}",from.as_ref(), to);

    let api = Api::new("http://localhost:9933").await?;
    let result = api.balance_transfer(from, to, 42_000_000_000_000_u128).await?;
    println!("result: {:?}", result);
    Ok(())
}
