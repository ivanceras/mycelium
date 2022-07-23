//#![deny(warnings)]
use mycelium::{
    Api,
    Metadata,
};

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let api = Api::new("http://localhost:9933").await?;
    let metadata: &Metadata = api.metadata();
    println!("metadata: {:#?}", metadata);
    println!("runtime version: {:#?}", api.runtime_version());
    Ok(())
}
