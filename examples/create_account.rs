#![allow(warnings)]
use mycelium::{
    sp_core::crypto::AccountId32,
    Api,
};
use sp_core::Pair;
use sp_keyring::AccountKeyring;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let entropy = [0;32];
    let pwd = Some("horse battery staple");
    let (new_account, seed): (sp_core::sr25519::Pair, [u8;32]) = sp_core::sr25519::Pair::from_entropy(&entropy, pwd);
    println!("new account: {:?}", new_account.public());
    println!("seed: {:?}", seed);

    let entropy: [u8;32] = rand::random();
    let pwd= None;
    let (random_account, seed): (sp_core::sr25519::Pair, [u8;32]) = sp_core::sr25519::Pair::from_entropy(&entropy, pwd);
    println!("new account: {:?}", random_account.public());
    println!("seed: {:?}", seed);

    Ok(())
}
