//! Do a sudoer function Balance::set_balance
//! This is using Alice as the sudo user then set the balance amount to Bob
#![deny(warnings)]
use codec::Compact;
use mycelium::{
    types::{
        extrinsics::GenericAddress,
    },
    Api,
    Metadata,
};
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let sudoer: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();

    let to = AccountKeyring::Bob.to_account_id();

    let api = Api::new("http://localhost:9933").await?;
    let metadata: &Metadata = api.metadata();

    let balance_pallet = metadata.pallet("Balances")?;
    let set_balance_call_index = balance_pallet
        .calls
        .get("set_balance")
        .expect("function name does not exist");

    //u128::MAX = 340_282_366_920_938_463_463_374_607_431_768_211_455u128
    // 1Yunit = 1_000_000_000_000_000_000_000_000_000_000_000_000_u128
    // 1Munit = 1_000_000_000_000_000_000_u128
    let balance_call = (
        [balance_pallet.index, *set_balance_call_index],
        GenericAddress::Id(to),
        Compact(42_000_000_000_000_000_000_u128), //new free
        Compact(42_000_000_000_000_000_000_u128), //new reserved
    );

    let sudo_pallet = metadata.pallet("Sudo")?;
    let sudo_call_index = sudo_pallet
        .calls
        .get("sudo")
        .expect("function name does not exist");
    let sudo_call: (
        [u8; 2],
        ([u8; 2], GenericAddress, Compact<u128>, Compact<u128>),
    ) = ([sudo_pallet.index, *sudo_call_index as u8], balance_call);

    let xt = api.sign_extrinsic(&sudoer, sudo_call, None).await?;

    let result = api.submit_extrinsic(xt).await?;
    println!("result: {:?}", result);
    Ok(())
}
