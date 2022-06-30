//#![deny(warnings)]
use mycelium::{Api, Metadata};

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let api = Api::new("http://localhost:9933").await?;
    let metadata: &Metadata = api.metadata();
    println!("metadata: {:#?}", metadata);
    let storage_type = metadata.storage_value_type("TemplateModule", "Something");
    println!(
        "storage type of TemplateModule::Something: {:?}",
        storage_type
    );
    let total_issuance = metadata.storage_value_type("Balances", "TotalIssuance");
    println!(
        "storage type of Balances::TotalIssuance: {:?}",
        total_issuance
    );

    let account_balance = metadata.storage_map_type("Balances", "Account");
    println!("storage type of Balances::Account: {:#?}", account_balance);
    Ok(())
}
