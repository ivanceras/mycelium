use crate::{
    api::Api,
    error::Error,
    types::{
        account_info::AccountInfo,
        extrinsic_params::{
            BaseExtrinsicParams, BaseExtrinsicParamsBuilder, ExtrinsicParams, GenericExtra,
            SignedPayload,
        },
        extrinsics::{GenericAddress, UncheckedExtrinsicV4},
    },
};
use codec::Encode;
use sp_core::{crypto::Pair, H256};
use sp_runtime::{traits::IdentifyAccount, AccountId32, MultiSignature, MultiSigner};
use std::fmt;

impl Api {
    pub fn signer_account<P>(&self, signer: &P) -> AccountId32
    where
        P: Pair,
        MultiSigner: From<P::Public>,
    {
        let multi_signer = MultiSigner::from(signer.public());
        multi_signer.into_account()
    }

    pub async fn get_nonce<P>(&self, signer: &P) -> Result<u32, Error>
    where
        P: Pair,
        MultiSigner: From<P::Public>,
    {
        let signer_account = self.signer_account(signer);
        let account_info = self.get_account_info(signer_account).await?;
        match account_info {
            None => Ok(0),
            Some(account_info) => Ok(account_info.nonce),
        }
    }

    pub async fn get_account_info(
        &self,
        account_id: AccountId32,
    ) -> Result<Option<AccountInfo>, Error> {
        self.fetch_storage_map("System", "Account", account_id)
            .await
    }

    pub fn unsigned_extrinsic<Call>(&self, call: Call) -> UncheckedExtrinsicV4<Call>
    where
        Call: Encode,
    {
        UncheckedExtrinsicV4::new_unsigned(call)
    }

    pub async fn sign_extrinsic<P, Call>(
        &self,
        signer: P,
        call: Call,
    ) -> Result<UncheckedExtrinsicV4<Call>, Error>
    where
        P: Pair,
        MultiSigner: From<P::Public>,
        MultiSignature: From<P::Signature>,
        Call: Encode + Clone + fmt::Debug,
    {
        let nonce = self.get_nonce(&signer).await?;
        let extra: GenericExtra = GenericExtra::immortal_with_nonce_and_tip(nonce, 0);
        let xt = self.sign_extrinsic_with_extra(signer, call, extra).await?;
        Ok(xt)
    }

    pub async fn sign_extrinsic_with_extra<P, Call>(
        &self,
        signer: P,
        call: Call,
        extra: GenericExtra,
    ) -> Result<UncheckedExtrinsicV4<Call>, Error>
    where
        P: Pair,
        MultiSigner: From<P::Public>,
        MultiSignature: From<P::Signature>,
        Call: Encode + Clone + fmt::Debug,
    {
        let raw_payload: SignedPayload<Call> = SignedPayload::from_raw(
            call.clone(),
            extra.clone(),
            (
                self.runtime_version.spec_version,
                self.runtime_version.transaction_version,
                self.genesis_hash,
                self.genesis_hash,
                (),
                (),
                (),
            ),
        );
        let signature = self.sign_raw_payload(&signer, raw_payload);

        let multi_signer = MultiSigner::from(signer.public());
        let multi_signature = MultiSignature::from(signature);
        Ok(UncheckedExtrinsicV4::new_signed(
            call,
            GenericAddress::from(multi_signer.into_account()),
            multi_signature,
            extra,
        ))
    }

    /// create an UncheckedExtrisic<Call>
    /// This is a simplified version of compose_extrinsics_with_params
    /// but has less generics to deal with
    pub async fn compose_extrinsics<P, Call>(
        &self,
        signer: Option<P>,
        call: Call,
    ) -> Result<UncheckedExtrinsicV4<Call>, Error>
    where
        P: Pair,
        MultiSigner: From<P::Public>,
        MultiSignature: From<P::Signature>,
        Call: Encode + Clone + fmt::Debug,
    {
        match signer {
            None => Ok(self.unsigned_extrinsic(call)),
            Some(signer) => Ok(self.sign_extrinsic(signer, call).await?),
        }
    }

    /// create an UncheckedExtrinsic from call with an optional signer
    pub async fn compose_extrinsics_with_params<P, Params, Tip, Call>(
        &self,
        signer: Option<P>,
        call: Call,
        extrinsic_params: Option<Params::OtherParams>,
    ) -> Result<UncheckedExtrinsicV4<Call>, Error>
    where
        P: Pair,
        MultiSigner: From<P::Public>,
        MultiSignature: From<P::Signature>,
        Params: ExtrinsicParams<OtherParams = BaseExtrinsicParamsBuilder<Tip>>,
        u128: From<Tip>,
        Tip: Encode + Default,
        Call: Encode + Clone + fmt::Debug,
    {
        match signer {
            None => Ok(self.unsigned_extrinsic(call)),
            Some(signer) => {
                let nonce = self.get_nonce(&signer).await?;

                let other_params: BaseExtrinsicParamsBuilder<Tip> =
                    extrinsic_params.unwrap_or_default();
                let params: BaseExtrinsicParams<Tip> =
                    BaseExtrinsicParams::new(nonce, other_params);
                let extra: GenericExtra = GenericExtra::from(params);
                let xt = self.sign_extrinsic_with_extra(signer, call, extra).await?;
                Ok(xt)
            }
        }
    }

    /// sign a payload
    pub fn sign_raw_payload<P, Call>(
        &self,
        signer: &P,
        raw_payload: SignedPayload<Call>,
    ) -> P::Signature
    where
        P: Pair,
        Call: Encode + Clone + fmt::Debug,
    {
        raw_payload.using_encoded(|payload| self.sign(signer, payload))
    }

    /// sign a bytes with the specified signer
    /// TODO: This should call an external API for the runtime
    /// otherwise, this api acts as if it is a wallet
    pub fn sign<P>(&self, signer: &P, payload: &[u8]) -> P::Signature
    where
        P: Pair,
    {
        signer.sign(payload)
    }

    /// A simplified version of sign_and_submit_extrisic_with_params
    /// with less Generic parameters to deal with
    pub async fn sign_and_submit_extrinsic<P, Call>(
        &self,
        signer: Option<P>,
        call: Call,
    ) -> Result<Option<H256>, Error>
    where
        P: sp_core::crypto::Pair,
        MultiSigner: From<P::Public>,
        MultiSignature: From<P::Signature>,
        Call: Clone + fmt::Debug + Encode,
    {
        let xt = self.compose_extrinsics::<P, Call>(signer, call).await?;

        let encoded = xt.hex_encode();
        self.author_submit_extrinsic(&encoded).await
    }

    pub async fn sign_and_submit_extrinsic_with_params<P, Params, Tip, Call>(
        &self,
        signer: Option<P>,
        call: Call,
        extrinsic_params: Option<Params::OtherParams>,
    ) -> Result<Option<H256>, Error>
    where
        P: sp_core::crypto::Pair,
        MultiSigner: From<P::Public>,
        MultiSignature: From<P::Signature>,
        Params: ExtrinsicParams<OtherParams = BaseExtrinsicParamsBuilder<Tip>>,
        u128: From<Tip>,
        Tip: Encode + Default,
        Call: Clone + fmt::Debug + Encode,
    {
        let xt = self
            .compose_extrinsics_with_params::<P, Params, Tip, Call>(signer, call, extrinsic_params)
            .await?;

        let encoded = xt.hex_encode();
        self.author_submit_extrinsic(&encoded).await
    }
}
