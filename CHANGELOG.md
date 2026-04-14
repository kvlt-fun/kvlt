# Changelog

All notable changes to this project will be documented in this file.
The format is based on Keep a Changelog, and this project adheres to
Semantic Versioning.

## [0.5.2] - 2026-04-16

### Added
- Revocation instruction with issuer-scoped authority check.
- Merkle leaf helper in `libs/inkd_math` with fixed-size hashing.
- TypeScript PDA helpers for issuer and attestation accounts.

### Changed
- Switched internal time representation to `i64` unix seconds.
- Tightened account space calculations with explicit padding.

### Fixed
- Off-by-one in attestation index when the tree buffer wraps.
- CLI now returns a non-zero exit code on verification failure.

## [0.4.0] - 2026-03-22

### Added
- Issuer registration flow with per-issuer nonce.
- Event emission for `Minted`, `Revoked`, and `Verified`.

### Changed
- Reorganized instructions into submodules.

## [0.3.0] - 2026-02-20

### Added
- First pass of the TypeScript SDK client wrapper.
- End-to-end test harness against local validator.

## [0.2.0] - 2026-01-31

### Added
- Core account layouts: `Attestation`, `Issuer`.
- Mint attestation instruction skeleton.

## [0.1.0] - 2026-01-11

### Added
- Initial workspace scaffold and CI pipeline.
