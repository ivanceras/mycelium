//! This example transfer all of Alice balance units to Charlie
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

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let to = AccountKeyring::Charlie.to_account_id();

    let api = Api::new("http://localhost:9933").await?;
    let metadata: &Metadata = api.metadata();

    let balance_pallet = metadata.pallet("Balances")?;
    let balance_transfer_call_index = balance_pallet
        .calls
        .get("transfer_all")
        .expect("function name does not exist");

    //u128::MAX = 340_282_366_920_938_463_463_374_607_431_768_211_455u128
    // 1Yunit = 1_000_000_000_000_000_000_000_000_000_000_000_000_u128
    // 1Munit = 1_000_000_000_000_000_000_u128
    let balance_call = (
        [balance_pallet.index, *balance_transfer_call_index],
        GenericAddress::Id(to),
        true,
    );

    let xt = api.compose_extrinsics::<
        sp_core::sr25519::Pair,
        PlainTipExtrinsicParams,
        PlainTip,
        ([u8; 2], GenericAddress, bool),
    >(Some(from), balance_call, None, None)
    .await?;

    let encoded = xt.hex_encode();
    println!("encoded: {}", encoded);
    let result = api.author_submit_extrinsic(&encoded).await?;
    println!("result: {:?}", result);
    Ok(())
}
