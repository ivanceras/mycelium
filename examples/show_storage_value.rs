//! This example get the values from the storage items from their respective pallets
#![allow(warnings)]
use mycelium::sp_core::crypto::AccountId32;
use mycelium::Api;
use pallet_balances::AccountData;
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let api = Api::new("http://localhost:9933").await?;
    let something: Result<Option<u32>, _> =
        api.fetch_storage_value("TemplateModule", "Something").await;
    println!("something: {:?}", something);

    let total_issuance: Result<Option<u128>, _> =
        api.fetch_storage_value("Balances", "TotalIssuance").await;
    println!("total issuance: {:?}", total_issuance);

    dbg!(
        api.fetch_opaque_storage_value("Balances", "TotalIssuance")
            .await?
    );

    let account_id: AccountId32 = AccountKeyring::Alice.to_account_id();
    let account_balance: Result<Option<u128>, _> = api
        .fetch_storage_map("Balances", "Account", account_id)
        .await;

    println!("account_balance: {:?}", account_balance);

    let storage_type = api
        .metadata()
        .storage_value_type("TemplateModule", "Something");
    println!(
        "storage type of TemplateModule::Something: {:?}",
        storage_type
    );
    let total_issuance_type = api
        .metadata()
        .storage_value_type("Balances", "TotalIssuance");
    println!(
        "storage type of Balances::TotalIssuance: {:?}",
        total_issuance_type
    );

    let account_balance_type = api.metadata().storage_map_type("Balances", "Account");
    println!(
        "storage type of Balances::Account: {:#?}",
        account_balance_type
    );

    if let Some((key_type, value_type)) =
        api.metadata().storage_map_type("ForumModule", "AllPosts")?
    {
        println!("type of ForumModule AllPosts key: {:#?}", key_type);
    }

    let paged: Result<Option<Vec<Vec<u8>>>, _> = api
        .fetch_opaque_storage_map_paged("Balances", "Reserves", 10, None::<AccountId32>)
        .await;
    println!("paged: {:?}", paged);
    Ok(())
}
