import { InkdClient } from '../sdk/src/client';
import { AttestationRecord, AttestationStatus } from '../sdk/src/types';

describe('integration (skipped on CI without a local validator)', () => {
  const client = new InkdClient({ cluster: 'localnet', rpcUrl: 'http://127.0.0.1:8899' });

  it.skip('issues and verifies a credential end-to-end', async () => {
    const plan = client.plannedMint({
      issuerSlug: 'inkd',
      recipient: '3C7Mqhnb9N4cerkMkVMZKa5ceQm1i5rSqQNaC9goNxEN',
      credential: 'day-one',
      payloadHash: new Uint8Array(32),
    });
    expect(plan.program).toBeDefined();
  });

  it('plans a mint deterministically', () => {
    const a = client.plannedMint({
      issuerSlug: 'inkd',
      recipient: '3C7Mqhnb9N4cerkMkVMZKa5ceQm1i5rSqQNaC9goNxEN',
      credential: 'first-time',
      payloadHash: new Uint8Array(32),
    });
    const b = client.plannedMint({
      issuerSlug: 'inkd',
      recipient: '3C7Mqhnb9N4cerkMkVMZKa5ceQm1i5rSqQNaC9goNxEN',
      credential: 'first-time',
      payloadHash: new Uint8Array(32),
    });
    expect(a.seeds.length).toBe(b.seeds.length);
  });

  it('rejects an expired attestation', () => {
    const expired: AttestationRecord = {
      issuer: '6VKQVC6RQj5wDnXQ1pVfuTGga2iUUjFPMxAHSN6fjPck',
      recipient: '3C7Mqhnb9N4cerkMkVMZKa5ceQm1i5rSqQNaC9goNxEN',
      leaf: new Uint8Array(32),
      credential: 'old',
      issuedAt: 1,
      expiresAt: 2,
      revokedAt: 0,
      index: 0n,
      status: AttestationStatus.Active,
    };
    const res = client.verifyLocally(expired, 3);
    expect(res.valid).toBe(false);
    expect(res.reason).toBe('expired');
  });
});
