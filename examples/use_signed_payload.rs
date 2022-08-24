#![allow(warnings)]
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
use sp_runtime::generic::SignedPayload;
use sp_runtime::generic::UncheckedExtrinsic;
use codec::Encode;
use codec::Decode;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let extra = (); //(Era::immortal(), Compact(0), Compact(0));

    let payload = SignedPayload::new(call.clone(), extra)?;
    let payload_encoded = payload.encode();
    let signature = from.sign(&payload_encoded);

    let multi_signature = MultiSignature::from(signature.clone());
    let signed_xt = UncheckedExtrinsic::<AccountId32, ([u8;2], MultiAddress::<AccountId32, ()>, Compact<u128>), MultiSignature, ()>
        ::new_signed(call.clone(), signer_account.clone(), multi_signature, extra);

    println!("signed_xt encoded : {:?}", signed_xt.encode());
    println!("encoded again     : {:?}", signed_xt.encode().encode());
    println!("encoded again2    : {:?}", signed_xt.encode().encode());
    let recovered = UncheckedExtrinsic::<AccountId32, ([u8;2], MultiAddress::<AccountId32, ()>, Compact<u128>), MultiSignature, ()>::decode(&mut signed_xt.encode().encode().as_slice())?;
    println!("recovered: {:?}", recovered);

    let signed_xt_hex = hex::encode(signed_xt.encode());
    let tx_hash = api.author_submit_extrinsic(signed_xt_hex).await;
    dbg!(&tx_hash);

    Ok(())
}
