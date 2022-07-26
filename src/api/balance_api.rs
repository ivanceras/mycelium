//! Balance transfer, set_balance api
use crate::{
    error::Error,
    types::{
        extrinsics::GenericAddress,
    },
    Api,
};
use codec::Compact;
use sp_core::{
    crypto::{
        AccountId32,
        Pair,
    },
    H256,
};
use sp_runtime::{
    MultiSignature,
    MultiSigner,
};

const BALANCES: &str = "Balances";

impl Api {
    /// transfer an amount using a signer `from` to account `to` with `amount` and `tip`
    pub async fn balance_transfer<P>(
        &self,
        from: P,
        to: AccountId32,
        amount: u128,
        tip: Option<u128>,
    ) -> Result<Option<H256>, Error>
    where
        P: Pair,
        MultiSigner: From<P::Public>,
        AccountId32: From<P::Public>,
        MultiSignature: From<P::Signature>,
    {
        let balance_call_index: [u8; 2] =
            self.pallet_call_index(BALANCES, "transfer")?;

        let balance_call: ([u8; 2], GenericAddress, Compact<u128>) =
            (balance_call_index, GenericAddress::Id(to), Compact(amount));



        let extrinsic = self.sign_extrinsic(&from, balance_call, tip).await?;
        let encoded = extrinsic.hex_encode();
        let tx_hash = self.author_submit_extrinsic(encoded).await?;
        Ok(tx_hash)
    }
}
