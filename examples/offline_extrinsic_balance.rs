//! An example using an offline extrinsic, using the types of the instantiated chain
#![allow(warnings)]
use codec::Compact;
use mycelium::{
    types::{
        extrinsic_params::{PlainTip, PlainTipExtrinsicParams, PlainTipExtrinsicParamsBuilder},
        extrinsics::GenericAddress,
    },
    Api, Metadata,
};
use node_template_runtime::{BalancesCall, Call, Header};
use sp_core::H256;
use sp_keyring::AccountKeyring;
use sp_runtime::{generic::Era, MultiAddress};

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let to = AccountKeyring::Bob.to_account_id();

    let api = Api::new("http://localhost:9933").await?;
    let metadata: &Metadata = api.metadata();

    let genesis_hash: H256 = api.genesis_hash();

    let head_hash = api
        .chain_get_finalized_head()
        .await?
        .expect("must have a finalized head");
    let header: Header = api
        .chain_get_header(head_hash)
        .await?
        .expect("must have a header");
    let period = 5;
    let tx_params = PlainTipExtrinsicParamsBuilder::new()
        .era(Era::mortal(period, header.number.into()), genesis_hash)
        .tip(10);

    let call: Call = Call::Balances(BalancesCall::transfer {
        dest: MultiAddress::Id(to),
        value: 69_420,
    });

    let xt = api
        .compose_extrinsics::<sp_core::sr25519::Pair, PlainTipExtrinsicParams, PlainTip, Call>(
            Some(from),
            call,
            Some(head_hash),
            Some(tx_params),
        )
        .await?;

    let encoded = xt.hex_encode();
    println!("encoded: {}", encoded);
    let result = api.author_submit_extrinsic(&encoded).await?;
    println!("result: {:?}", result);
    Ok(())
}
