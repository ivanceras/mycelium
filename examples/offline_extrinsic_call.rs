//! An example using an offline extrinsic, using the types of the instantiated chain
#![allow(warnings)]
use mycelium::{
    Api,
};
use node_template_runtime::{
    BalancesCall,
    Call,
    Header,
};
use sp_core::H256;
use sp_keyring::AccountKeyring;
use sp_runtime::{
    generic::Era,
    MultiAddress,
};
use sp_runtime::MultiSignature;
use mycelium::types::extrinsics::UncheckedExtrinsicV4;
use sp_core::Pair;
use mycelium::types::extrinsics::GenericAddress;
use sp_core::crypto::AccountId32;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let to = AccountKeyring::Bob.to_account_id();

    let api = Api::new("http://localhost:9933").await?;

    let genesis_hash: H256 = api.genesis_hash();

    let head_hash = api
        .chain_get_finalized_head()
        .await?
        .expect("must have a finalized head");
    let header: Header = api
        .chain_get_header(head_hash)
        .await?
        .expect("must have a header");

    let call: Call = Call::Balances(BalancesCall::transfer {
        dest: MultiAddress::Id(to),
        value: 69_420,
    });


    let period = 5;
    let era = Era::mortal(period, header.number.into());

    let signer_account = AccountId32::from(from.public());

    let (payload, extra) = api.compose_payload_and_extra(&signer_account, call.clone(), Some(era), Some(head_hash),  Some(10)).await?;

    let signature = payload.using_encoded(|payload|from.sign(payload));
    let multi_signature = MultiSignature::from(signature);

    let xt = UncheckedExtrinsicV4::new_signed(call, GenericAddress::from(signer_account), multi_signature, extra);
    let encoded = xt.hex_encode();
    let tx = api.author_submit_extrinsic(encoded).await?;
    println!("tx: {:?}", tx);
    Ok(())
}
