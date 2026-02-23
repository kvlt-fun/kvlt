use anchor_lang::prelude::*;

use crate::contexts::Verify;
use crate::errors::InkdError;
use crate::events::AttestationVerified;
use inkd_math::leaf::compute_leaf;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct VerifyParams {
    pub payload_hash: [u8; 32],
}

pub fn handler(ctx: Context<Verify>, params: VerifyParams) -> Result<()> {
    let attestation = &ctx.accounts.attestation;
    let issuer = &ctx.accounts.issuer;

    let recomputed = compute_leaf(
        &issuer.key().to_bytes(),
        &attestation.recipient.to_bytes(),
        &attestation.credential,
        &params.payload_hash,
        attestation.index,
    );

    require!(recomputed == attestation.leaf, InkdError::LeafMismatch);

    let now = Clock::get()?.unix_timestamp;
    let valid = attestation.is_active(now);

    emit!(AttestationVerified {
        attestation: attestation.key(),
        verifier: ctx.accounts.verifier.key(),
        valid,
        timestamp: now,
    });

    if !valid {
        return err!(InkdError::Expired);
    }

    Ok(())
}
