use anchor_lang::prelude::*;

use crate::contexts::RegisterIssuer;
use crate::errors::InkdError;
use crate::events::IssuerRegistered;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RegisterIssuerParams {
    pub slug: [u8; 32],
}

pub fn handler(ctx: Context<RegisterIssuer>, params: RegisterIssuerParams) -> Result<()> {
    require!(!ctx.accounts.config.paused, InkdError::ProtocolPaused);

    let now = Clock::get()?.unix_timestamp;
    let issuer = &mut ctx.accounts.issuer;
    issuer.authority = ctx.accounts.authority.key();
    issuer.slug = params.slug;
    issuer.issued_count = 0;
    issuer.revoked_count = 0;
    issuer.created_at = now;
    issuer.bump = ctx.bumps.issuer;
    issuer.active = true;

    let config = &mut ctx.accounts.config;
    config.issuer_count = config
        .issuer_count
        .checked_add(1)
        .ok_or(InkdError::MathOverflow)?;

    emit!(IssuerRegistered {
        issuer: issuer.key(),
        authority: issuer.authority,
        slug: issuer.slug,
        timestamp: now,
    });

    Ok(())
}
