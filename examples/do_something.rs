//! This exampel call on an example pallet TemplateModule::do_something function
#![deny(warnings)]
use mycelium::{
    Api,
    Metadata,
};
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), mycelium::Error> {
    let signer: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();
    let api = Api::new("http://localhost:9933").await?;
    let metadata: &Metadata = api.metadata();
    let pallet = metadata.pallet("TemplateModule")?;

    let value: u32 = 1291232313;
    let call_index = pallet
        .calls
        .get("do_something")
        .expect("function name does not exist");
    let call: ([u8; 2], u32) = ([pallet.index, *call_index], value);

    let extrinsic = api.sign_extrinsic(&signer, call, None).await?;

    let result = api.submit_extrinsic(extrinsic).await?;
    println!("result: {:?}", result);

    //wait for a little bit
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let something: Result<Option<u32>, _> =
        api.fetch_storage_value("TemplateModule", "Something").await;
    println!("something: {:?}", something);

    assert_eq!(something.ok().flatten(), Some(value));

    Ok(())
}
