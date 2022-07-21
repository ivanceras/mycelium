//! Balance transfer, set_balance api
use crate::types::extrinsic_params::PlainTipExtrinsicParamsBuilder;
use crate::{
    error::Error,
    types::{
        extrinsic_params::{PlainTip, PlainTipExtrinsicParams},
        extrinsics::GenericAddress,
    },
    Api,
};
use codec::Compact;
use sp_core::crypto::AccountId32;
use sp_core::crypto::Pair;
use sp_core::H256;
use sp_runtime::generic::Era;
use sp_runtime::MultiSignature;
use sp_runtime::MultiSigner;

impl Api {
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
        MultiSignature: From<P::Signature>,
    {
        let balance_pallet = self.metadata.pallet("Balances")?;
        let balance_transfer_call_index = balance_pallet
            .calls
            .get("transfer")
            .expect("unable to find transfer function");

        let balance_call: ([u8; 2], GenericAddress, Compact<u128>) = (
            [balance_pallet.index, *balance_transfer_call_index],
            GenericAddress::Id(to),
            Compact(amount),
        );

        let tx_params = if let Some(tip) = tip {
            let genesis_hash = self.genesis_hash();

            let tx_params = PlainTipExtrinsicParamsBuilder::new()
                .tip(tip)
                .era(Era::Immortal, genesis_hash);

            Some(tx_params)
        } else {
            None
        };

        self.execute_extrinsic::<P, PlainTipExtrinsicParams, PlainTip,
            ([u8; 2], GenericAddress, Compact<u128>)>(Some(from), balance_call, None, tx_params)
            .await
    }
}
