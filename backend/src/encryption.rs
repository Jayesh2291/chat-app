use ring::{aead, rand};
use std::sync::Arc;

pub fn encrypt_message(key: &[u8], message: &[u8]) -> Vec<u8> {
    let rng = rand::SystemRandom::new();
    let nonce = rand::generate(&rng).unwrap().expose_bytes().to_vec();
    let mut in_out = message.to_vec();

    let sealing_key = aead::SealingKey::new(&aead::AES_256_GCM, key).unwrap();
    let _ = aead::seal_in_place(&sealing_key, &nonce, &[], &mut in_out, aead::MAX_TAG_LEN)
        .expect("encryption failed");

    let mut result = nonce;
    result.extend_from_slice(&in_out);
    result
}

pub fn decrypt_message(key: &[u8], encrypted_message: &[u8]) -> Option<Vec<u8>> {
    let (nonce, encrypted_data) = encrypted_message.split_at(12); // AES GCM nonce is 12 bytes
    let mut decrypted_data = encrypted_data.to_vec();

    let opening_key = aead::OpeningKey::new(&aead::AES_256_GCM, key).unwrap();
    aead::open_in_place(&opening_key, nonce, &[], 0, &mut decrypted_data)
        .ok()
        .map(|data| data.to_vec())
}
