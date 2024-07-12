use anchor_lang::prelude::*;
use rand::Rng;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ElGamalPubkey {
    pub g: u64,
    pub h: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ElGamalCiphertext {
    pub c1: u64,
    pub c2: u64,
}

pub fn generate_keypair() -> (ElGamalPubkey, u64) {
    let mut rng = rand::thread_rng();
    let g: u64 = 5; // A primitive root modulo p
    let x: u64 = rng.gen_range(1..100); // Private key
    let h = modpow(g, x, 101); // Public key, h = g^x mod p
    (ElGamalPubkey { g, h }, x)
}

pub fn encrypt(message: u64, public_key: &ElGamalPubkey) -> ElGamalCiphertext {
    let mut rng = rand::thread_rng();
    let y: u64 = rng.gen_range(1..100);
    let c1 = modpow(public_key.g, y, 101);
    let s = modpow(public_key.h, y, 101);
    let c2 = (message * s) % 101;
    ElGamalCiphertext { c1, c2 }
}

pub fn decrypt(ciphertext: &ElGamalCiphertext, private_key: u64) -> u64 {
    let s = modpow(ciphertext.c1, private_key, 101);
    let s_inv = modinv(s, 101);
    (ciphertext.c2 * s_inv) % 101
}

// Helper function for modular exponentiation
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

// Helper function for modular inverse
fn modinv(a: u64, m: u64) -> u64 {
    let mut a = a as i64;
    let mut m = m as i64;
    let mut y = 0;
    let mut x = 1;
    while a > 1 {
        let q = a / m;
        let mut t = m;
        m = a % m;
        a = t;
        t = y;
        y = x - q * y;
        x = t;
    }
    if x < 0 {
        x += m as i64;
    }
    x as u64
}