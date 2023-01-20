#![allow(improper_ctypes)]

use ed25519_compact::{KeyPair, PublicKey, Signature};
use ed25519_compact::{Noise, SecretKey};
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;
use std::ops::Deref;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
#[derive(Debug)]
pub struct Ed25519KeyPair {
    pub pk: String,
    pub sk: String,
}

#[marine]
pub fn generate_keypair() -> Ed25519KeyPair {
    let kp = KeyPair::generate();
    let base64_pk = base64::encode(kp.pk.deref());

    let base64_sk = base64::encode(kp.sk.deref());

    Ed25519KeyPair {
        pk: base64_pk,
        sk: base64_sk,
    }
}

#[marine]
pub fn verify(public_key: String, signature: String, message: String) -> bool {
    let p_key_decoded = base64::decode(public_key).unwrap();
    let sign_decoded = base64::decode(signature).unwrap();

    let pk: [u8; 32] = p_key_decoded
        .try_into()
        .expect("Error: public_key with incorrect length");

    let sign: [u8; 64] = sign_decoded
        .try_into()
        .expect("Error: Sign with incorrect length");

    let p_key = PublicKey::new(pk);

    match p_key.verify(message, &Signature::new(sign)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[marine]
pub fn sign(message: String, private_key: String) -> String {
    let pk_key_decoded = base64::decode(private_key).unwrap();

    let pk = pk_key_decoded.try_into().expect("invalid private key");

    let sk = SecretKey::new(pk);

    let signature = sk.sign(message, Some(Noise::default()));

    base64::encode(signature)
}
