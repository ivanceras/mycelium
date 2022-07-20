#![deny(warnings)]
use mycelium::Api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = Api::new("http://localhost:9933").await?;
    let genesis_hash = api.fetch_genesis_hash().await?;
    println!("genesis block hash {:#?}", genesis_hash);
    assert_eq!(genesis_hash, api.fetch_block_hash(0).await?);
    if let Some(hash) = api.fetch_block_hash(1).await? {
        println!("hash 1: {:#x}", hash);
        println!("hash 1: {:#X}", hash);
        println!("hex encode: {}", hex::encode(hash.0));
    }
    Ok(())
}
