//! An example using an offline extrinsic, using the types of the instantiated chain
#![deny(warnings)]
use mycelium::{
    Api,
};
use node_template_runtime::{
    Header,
};
use sp_keyring::AccountKeyring;
use sp_runtime::{
    generic::Era,
    MultiAddress,
};
use sp_runtime::MultiSignature;
use sp_core::Pair;
use sp_core::crypto::AccountId32;
use codec::Compact;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let to = AccountKeyring::Bob.to_account_id();

    let api = Api::new("http://localhost:9933").await?;

    let head_hash = api
        .chain_get_finalized_head()
        .await?
        .expect("must have a finalized head");
    let header: Header = api
        .chain_get_header(head_hash)
        .await?
        .expect("must have a header");

    let call_index = api.pallet_call_index("Balances", "transfer")?;
    let call:([u8;2], MultiAddress::<AccountId32, ()>, Compact<u128>) = (call_index, MultiAddress::<AccountId32, ()>::Id(to), Compact(69_420));


    let period = 5;
    let era = Era::mortal(period, header.number.into());

    let signer_account = AccountId32::from(from.public());

    let nonce = api.get_nonce_for_account(&signer_account).await?;

    let (payload, extra) = api.compose_opaque_payload_and_extra(nonce, call.clone(), Some(era), Some(head_hash),  Some(10)).await?;

    let signature: sp_core::sr25519::Signature = from.sign(&payload);
    let multi_signature = MultiSignature::from(signature);

    let tx = api.submit_signed_call(call, &signer_account, multi_signature, extra).await?;
    println!("tx: {:?}", tx);
    Ok(())
}
