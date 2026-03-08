export type Base58 = string;

export interface IssuerRecord {
  authority: Base58;
  slug: string;
  issuedCount: bigint;
  revokedCount: bigint;
  createdAt: number;
  active: boolean;
}

export interface AttestationRecord {
  issuer: Base58;
  recipient: Base58;
  leaf: Uint8Array;
  credential: string;
  issuedAt: number;
  expiresAt: number;
  revokedAt: number;
  index: bigint;
  status: AttestationStatus;
}

export enum AttestationStatus {
  Pending = 0,
  Active = 1,
  Revoked = 2,
  Expired = 3,
}

export interface MintInput {
  issuerSlug: string;
  recipient: Base58;
  credential: string;
  payloadHash: Uint8Array;
  expiresAt?: number;
}

export interface VerifyResult {
  valid: boolean;
  reason?: string;
  checkedAt: number;
}
