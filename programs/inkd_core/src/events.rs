use anchor_lang::prelude::*;

#[event]
pub struct IssuerRegistered {
    pub issuer: Pubkey,
    pub authority: Pubkey,
    pub slug: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct AttestationMinted {
    pub attestation: Pubkey,
    pub issuer: Pubkey,
    pub recipient: Pubkey,
    pub leaf: [u8; 32],
    pub index: u64,
    pub timestamp: i64,
}

#[event]
pub struct AttestationVerified {
    pub attestation: Pubkey,
    pub verifier: Pubkey,
    pub valid: bool,
    pub timestamp: i64,
}

#[event]
pub struct AttestationRevoked {
    pub attestation: Pubkey,
    pub issuer: Pubkey,
    pub reason_code: u16,
    pub timestamp: i64,
}
