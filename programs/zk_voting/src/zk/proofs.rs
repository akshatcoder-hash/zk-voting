use super::elgamal::{ElGamalPubkey, ElGamalCiphertext};
use anchor_lang::prelude::*;
use rand::Rng;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VoteProof {
    pub c: u64,
    pub r: u64,
}

pub fn generate_vote_proof(vote: bool, random: u64, public_key: &ElGamalPubkey) -> VoteProof {
    let mut rng = rand::thread_rng();
    let w = rng.gen_range(1..100);
    let a1 = modpow(public_key.g, w, 101);
    let a2 = modpow(public_key.h, w, 101);
    
    let e = rng.gen_range(1..100);
    let c = e;
    let r = (w - (if vote { 1 } else { 0 }) * random * c) % 100;
    
    VoteProof { c, r }
}

pub fn verify_vote_proof(proof: &VoteProof, ciphertext: &ElGamalCiphertext, public_key: &ElGamalPubkey) -> bool {
    let t1 = (modpow(public_key.g, proof.r, 101) * modpow(ciphertext.c1, proof.c, 101)) % 101;
    let t2 = (modpow(public_key.h, proof.r, 101) * modpow(ciphertext.c2, proof.c, 101)) % 101;
    
    t1 == modpow(public_key.g, proof.r, 101) && (t2 == modpow(public_key.h, proof.r, 101) || t2 == (modpow(public_key.h, proof.r, 101) * public_key.h) % 101)
}

fn modpow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    result
}