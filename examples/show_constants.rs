//! This example get the values from the storage items from their respective pallets
#![allow(warnings)]
use mycelium::sp_core::crypto::AccountId32;
use mycelium::Api;
use pallet_balances::AccountData;
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let api = Api::new("http://localhost:9933").await?;

    let existential_deposit =
        api.fetch_constant_opaque_value("Balances", "ExistentialDeposit")?;
    let existential_deposit: Result<u128, _> =
        codec::Decode::decode(&mut existential_deposit.as_slice());
    dbg!(existential_deposit);

    dbg!(api.fetch_constant_type("Balances", "ExistentialDeposit")?);

    let max_reserves =
        api.fetch_constant_opaque_value("Balances", "MaxReserves")?;
    let max_reserves: Result<u32, _> =
        codec::Decode::decode(&mut max_reserves.as_slice());
    dbg!(max_reserves);

    let not_found =
        api.fetch_constant_opaque_value("Balances", "ThisDoesNotExists")?;
    let not_found: Result<u32, _> =
        codec::Decode::decode(&mut not_found.as_slice());
    dbg!(not_found);

    let max_comments =
        api.fetch_constant_opaque_value("ForumModule", "MaxComments")?;
    let max_comments: Result<u32, _> =
        codec::Decode::decode(&mut max_comments.as_slice());
    dbg!(max_comments);

    let max_content_length =
        api.fetch_constant_opaque_value("ForumModule", "MaxContentLength")?;
    let max_content_length: Result<u32, _> =
        codec::Decode::decode(&mut max_content_length.as_slice());
    dbg!(max_content_length);
    Ok(())
}
