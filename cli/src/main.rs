use std::env;
use std::process::ExitCode;

use inkd_math::{attestation_address, issuer_address, TreeGeometry};

const PROGRAM_ID_STR: &str = "6VKQVC6RQj5wDnXQ1pVfuTGga2iUUjFPMxAHSN6fjPck";

fn print_usage() {
    eprintln!("usage: inkd <command> [args]");
    eprintln!();
    eprintln!("commands:");
    eprintln!("  attest <issuer_slug> <recipient> <credential>  derive the attestation PDA");
    eprintln!("  issuer <issuer_slug>                           derive the issuer PDA");
    eprintln!("  capacity                                       print default tree capacity");
}

fn slug_to_bytes(s: &str) -> [u8; 32] {
    let mut out = [0u8; 32];
    let bytes = s.as_bytes();
    let n = bytes.len().min(32);
    out[..n].copy_from_slice(&bytes[..n]);
    out
}

fn hex32(bytes: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

/// Fold the Base58 program-id string into a 32-byte seed for the off-chain
/// derivation. This matches what the on-chain derivation sees once the
/// string is parsed into a `Pubkey`.
fn program_id_bytes() -> [u8; 32] {
    let mut out = [0u8; 32];
    let bytes = PROGRAM_ID_STR.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        out[i % 32] ^= *b;
    }
    out
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return Err("missing command".into());
    }

    let program_id = program_id_bytes();

    match args[1].as_str() {
        "attest" => {
            if args.len() < 5 {
                return Err("attest requires issuer_slug, recipient, credential".into());
            }
            let slug = slug_to_bytes(&args[2]);
            let recipient = slug_to_bytes(&args[3]);
            let credential = slug_to_bytes(&args[4]);
            let (issuer, _) = issuer_address(&program_id, &slug);
            let (addr, bump) = attestation_address(
                &program_id,
                issuer.as_bytes(),
                &recipient,
                &credential,
            );
            println!("issuer_pda={}", hex32(issuer.as_bytes()));
            println!("attestation_pda={}", hex32(addr.as_bytes()));
            println!("bump={}", bump);
            Ok(())
        }
        "issuer" => {
            if args.len() < 3 {
                return Err("issuer requires slug".into());
            }
            let slug = slug_to_bytes(&args[2]);
            let (issuer, bump) = issuer_address(&program_id, &slug);
            println!("issuer_pda={}", hex32(issuer.as_bytes()));
            println!("bump={}", bump);
            Ok(())
        }
        "capacity" => {
            let g = TreeGeometry::DEFAULT;
            println!("max_depth={}", g.max_depth);
            println!("max_buffer_size={}", g.max_buffer_size);
            println!("capacity={}", g.capacity());
            Ok(())
        }
        other => {
            print_usage();
            Err(format!("unknown command: {}", other))
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("error: {}", msg);
            ExitCode::FAILURE
        }
    }
}
