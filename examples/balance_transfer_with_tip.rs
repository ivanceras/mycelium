//! This example transfer some amount from Alice to Charlie
#![deny(warnings)]
use mycelium::{
    sp_core::crypto::AccountId32,
    Api,
};
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let from: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let to: AccountId32 = AccountKeyring::Charlie.to_account_id();
    println!("transfering balance from: {:?} to: {}", from.as_ref(), to);

    let api = Api::new("http://localhost:9933").await?;
    let result = api
        .balance_transfer(
            from.clone(),
            to.clone(),
            41_500_000_000_000_u128,
            Some(500_000_000_000),
        )
        .await?;
    println!("result: {:?}", result);

    std::thread::sleep(std::time::Duration::from_millis(1500));

    let result_no_tip = api
        .balance_transfer(from, to, 40_100_200_300_400_u128, None)
        .await?;
    println!("result_no_tip: {:?}", result_no_tip);
    Ok(())
}
