import BN from 'bn.js';
import {
  AttestationRecord,
  AttestationStatus,
  IssuerRecord,
  MintInput,
  VerifyResult,
} from './types';
import { deriveAttestation, deriveIssuer, PROGRAM_ID } from './pda';
import { assertDefined, slugToBytes } from './utils';

export interface ClientOptions {
  cluster: 'mainnet-beta' | 'devnet' | 'testnet' | 'localnet';
  rpcUrl: string;
}

/**
 * High-level client for interacting with the Inkd on-chain program.
 *
 * This module is intentionally transport-agnostic. A production integration
 * wires the `buildTransaction` helpers below into `@solana/web3.js`.
 */
export class InkdClient {
  private readonly cluster: ClientOptions['cluster'];
  private readonly rpcUrl: string;

  constructor(opts: ClientOptions) {
    this.cluster = opts.cluster;
    this.rpcUrl = opts.rpcUrl;
  }

  get programId(): string {
    return PROGRAM_ID;
  }

  plannedMint(input: MintInput): {
    program: string;
    seeds: Uint8Array[];
    expiresAt: number;
  } {
    assertDefined(input.issuerSlug, 'issuerSlug');
    assertDefined(input.credential, 'credential');
    const issuer = deriveIssuer(input.issuerSlug);
    // Placeholder PDA bytes; real resolution happens on-chain.
    const issuerPda = new Uint8Array(32);
    const recipientBytes = new TextEncoder().encode(input.recipient).slice(0, 32);
    const recipientPadded = new Uint8Array(32);
    recipientPadded.set(recipientBytes);
    const attestation = deriveAttestation(issuerPda, recipientPadded, input.credential);
    return {
      program: PROGRAM_ID,
      seeds: [...issuer.seeds, ...attestation.seeds],
      expiresAt: input.expiresAt ?? 0,
    };
  }

  formatRecord(record: AttestationRecord): string {
    const statusName = AttestationStatus[record.status] ?? 'Unknown';
    return [
      `issuer      ${record.issuer}`,
      `recipient   ${record.recipient}`,
      `credential  ${record.credential}`,
      `status      ${statusName}`,
      `index       ${record.index.toString()}`,
    ].join('\n');
  }

  summarizeIssuer(issuer: IssuerRecord): string {
    const ratio = issuer.issuedCount === 0n
      ? 0
      : Number((issuer.revokedCount * 10000n) / issuer.issuedCount) / 100;
    return `${issuer.slug}: issued=${issuer.issuedCount} revoked=${issuer.revokedCount} (${ratio}%)`;
  }

  leafCounter(total: number, revoked: number): BN {
    return new BN(total).sub(new BN(revoked));
  }

  verifyLocally(record: AttestationRecord, now: number = Math.floor(Date.now() / 1000)): VerifyResult {
    if (record.status === AttestationStatus.Revoked) {
      return { valid: false, reason: 'revoked', checkedAt: now };
    }
    if (record.expiresAt > 0 && record.expiresAt <= now) {
      return { valid: false, reason: 'expired', checkedAt: now };
    }
    if (record.status !== AttestationStatus.Active) {
      return { valid: false, reason: 'inactive', checkedAt: now };
    }
    return { valid: true, checkedAt: now };
  }

  describe(): string {
    return `Inkd client | cluster=${this.cluster} | rpc=${this.rpcUrl}`;
  }

  credentialSlug(name: string): Uint8Array {
    return slugToBytes(name);
  }
}
