//! An example using an offline extrinsic, using the types of the instantiated chain
#![deny(warnings)]
use mycelium::{
    Api,
};
use node_template_runtime::{
    BalancesCall,
    Call,
    Header,
};
use sp_keyring::AccountKeyring;
use sp_runtime::{
    generic::Era,
    MultiAddress,
};

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
    let period = 5;

    let call: Call = Call::Balances(BalancesCall::transfer {
        dest: MultiAddress::Id(to),
        value: 69_420,
    });

let era = Era::mortal(period, header.number.into());

    let xt = api
        .sign_extrinsic_with_era(
            &from,
            call,
            Some(era),
            Some(head_hash),
            Some(10),
        )
        .await?;

    let result = api.submit_extrinsic(xt).await?;
    println!("result: {:?}", result);
    Ok(())
}
